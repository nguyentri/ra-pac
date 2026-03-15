#!/usr/bin/env python3

import os
import sys
import shutil
import subprocess
import glob
import tempfile
import re
import xml.etree.ElementTree as ET
from pathlib import Path
from collections import defaultdict, Counter
import argparse

# Global constants should be in uppercase
RUST_BACKTRACE = "1"
RUST_FULLTRACE = "full"
RUST_LOG = "info"

# SVD preprocessing options
FIX_ACCESS_TYPES_NESTED = True
FIX_ACCESS_TYPES = False
FIX_DERIVED_FROM = False
FIX_ENUMERATED_VALUES = False
FIX_ENUMERATED_VALUE_RANGES = True
FIX_WRITE_CONSTRAINTS = True
FIX_EMPTY_NAME_ENUMERATED_VALUES = True
FIX_MISSING_PLACEHOLDERS = True
FIX_RESET_VALUES = True
FIX_MISSING_ACCESS_TAGS = True
FIX_N_TO_NAME_TAGS = True

# Cargo version
CARGO_VERSION = "0.3.0"

# Define manifest template
manifest_template = '''
[package]
name = "@package_name@"
version = "@version@"
edition = "2024"
description = "Peripheral Access Crate (PAC) for @DEVICE_NAME@."
authors = ["Tri Nguyen <trongtribk06@gmail.com>"]
keywords = ["@device_name@", "@family_name@", "arm", "cortex-m", "renesas"]
repository = "https://github.com/nguyentri/@family_name@-pac"
documentation = "https://docs.rs/crate/@device_name@-pac/latest"
categories = ["embedded", "hardware-support", "no-std"]
include = ["src/*", "Cargo.toml", "README.md", "build.rs", "device.x"]
readme = "README.md"
license = "MIT OR Apache-2.0"

'''

DEFAULT_ONLY_ENUM_BLOCK_RE = re.compile(
    r'<enumeratedValue>.*?<isDefault>true</isDefault>(?:(?!<value>).)*?</enumeratedValue>',
    re.DOTALL,
)
EMPTY_ENUM_VALUES_RE = re.compile(
    r'\s*<enumeratedValues>\s*</enumeratedValues>',
    re.DOTALL,
)

def run_cargo_fmt(pac_dir):
    """Run cargo fmt --all in the specified PAC directory."""
    original_cwd = os.getcwd()
    try:
        print(f"Running cargo fmt for {pac_dir}...")

        # Change to the PAC directory
        os.chdir(pac_dir)

        # Run cargo fmt --all
        result = subprocess.run(
            ["cargo", "fmt", "--all"],
            capture_output=True,
            text=True,
            timeout=60  # 60 second timeout
        )

        # Change back to original directory
        os.chdir(original_cwd)

        if result.returncode == 0:
            print(f"✓ Successfully formatted {pac_dir}")
            return True
        else:
            print(f"✗ Failed to format {pac_dir}")
            if result.stderr:
                print(f"  Error: {result.stderr.strip()}")
            return False

    except subprocess.TimeoutExpired:
        print(f"✗ Timeout formatting {pac_dir}")
        os.chdir(original_cwd)
        return False
    except Exception as e:
        print(f"✗ Error formatting {pac_dir}: {str(e)}")
        os.chdir(original_cwd)
        return False

def fix_rust_file(file_path):
    """Fix common Rust formatting and duplicate issues in any Rust file."""
    backup_path = None
    try:
        # Read the contents of the Rust file with UTF-8 encoding and error handling
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as file:
            content = file.read()

        # Make a backup of the original file
        backup_path = f"{file_path}.bak"
        shutil.copy2(file_path, backup_path)

        # Fix "extern blocks must be unsafe"
        content = re.sub(r'extern\s+"C"\s+\{', 'unsafe extern "C" {', content)

        # Fix "unsafe attribute used without unsafe" for link_section
        content = re.sub(r'#\[link_section\s*=\s*"(.+?)"\]', r'#[unsafe(link_section = "\1")]', content)

        # Fix "unsafe attribute used without unsafe" for no_mangle
        content = re.sub(r'#\[no_mangle\]', r'#[unsafe(no_mangle)]', content)

        # Fix duplicate "unsafe unsafe" keywords
        content = re.sub(r'unsafe\s+unsafe\s+extern\s+"C"', 'unsafe extern "C"', content)
        content = re.sub(r'unsafe\s+unsafe\s+fn', 'unsafe fn', content)

        # Fix duplicate #[allow(dead_code)] attributes pattern
        # Pattern: #[allow(dead_code)] followed by any attribute then #[allow(dead_code)] again
        duplicate_pattern = r'#\[allow\(dead_code\)\]\s*\n\s*(#\[.*?\])\s*\n\s*#\[allow\(dead_code\)\]'
        content = re.sub(duplicate_pattern, r'#[allow(dead_code)]\n    \1', content, flags=re.MULTILINE)

        # Fix the specific problematic pattern with wrong indentation
        # Pattern from the build error: duplicate #[allow(dead_code)] with #[inline(always)] in between and wrong indentation
        specific_pattern = (
            r'\s*#\[allow\(dead_code\)\]\s*\n'
            r'\s*#\[inline\(always\)\]\s*\n'
            r'\s*#\[allow\(dead_code\)\]\s*\n'
            r'\s{6}(pub\(crate\)\s+const\s+unsafe\s+fn\s+from_ptr\(ptr:\s+\*mut\s+u8\)\s+->\s+&\'static\s+Self\s+\{)'
        )

        specific_replacement = (
            '    #[allow(dead_code)]\n'
            '    #[inline(always)]\n'
            '    \\1'
        )

        content = re.sub(specific_pattern, specific_replacement, content, flags=re.MULTILINE)

        # Fix for from_ptr functions without #[allow(dead_code)] - only if not already fixed
        pattern1 = r'^(\s*)(pub\(crate\)\s+const\s+fn\s+from_ptr\(ptr:\s+\*mut\s+u8\)\s+->.*?\{)'
        def replace1(match):
            indent = match.group(1)
            func = match.group(2)
            if '#[allow(dead_code)]' not in match.string[max(0, match.start()-100):match.start()]:
                return f'{indent}#[allow(dead_code)]\n{indent}{func}'
            return match.group(0)
        content = re.sub(pattern1, replace1, content, flags=re.MULTILINE)

        # Fix for unsafe from_ptr functions without #[allow(dead_code)] - only if not already fixed
        pattern2 = r'^(\s*)(pub\(crate\)\s+const\s+unsafe\s+fn\s+from_ptr\(ptr:\s+\*mut\s+u8\)\s+->.*?\{)'
        def replace2(match):
            indent = match.group(1)
            func = match.group(2)
            if '#[allow(dead_code)]' not in match.string[max(0, match.start()-100):match.start()]:
                return f'{indent}#[allow(dead_code)]\n{indent}{func}'
            return match.group(0)
        content = re.sub(pattern2, replace2, content, flags=re.MULTILINE)

        # Fix incorrectly converted function return arrows (-&gt; back to ->)
        content = re.sub(r'-&gt;', '->', content)

        # Fix unclosed HTML tags in doc comments (e.g., "BUSERRSTAT<Master Name>" -> "BUSERRSTAT&lt;Master Name&gt;")
        # Pattern to match ONLY doc comments with angle brackets that could be interpreted as HTML tags
        # This pattern is very specific to avoid affecting function signatures or other code
        doc_comment_pattern = r'(#\[doc\s*=\s*"[^"]*?)<([^<>"]*?)>([^"]*?")'

        def fix_doc_comment(match):
            prefix = match.group(1)
            tag_content = match.group(2)
            suffix = match.group(3)
            # Only apply if the tag_content looks like a proper name/identifier (not code)
            # Avoid fixing things like generic types or function signatures
            if not re.search(r'[-=:()]', tag_content):
                return f'{prefix}&lt;{tag_content}&gt;{suffix}'
            return match.group(0)  # Return unchanged if it looks like code

        content = re.sub(doc_comment_pattern, fix_doc_comment, content)

        # Write the modified content back to the file with UTF-8 encoding
        with open(file_path, 'w', encoding='utf-8') as file:
            file.write(content)

        print(f"Successfully fixed Rust formatting issues in {file_path}")

        # Delete backup if successful
        if os.path.exists(backup_path):
            os.remove(backup_path)
            print(f"Deleted backup file {backup_path}")

        return True

    except UnicodeDecodeError as e:
        print(f"Unicode encoding error in {file_path}: {str(e)}")
        print(f"Skipping file {file_path} due to encoding issues")
        return True  # Return True to continue processing other files
    except Exception as e:
        print(f"Error processing {file_path}: {str(e)}")
        # Restore backup if exists
        if backup_path and os.path.exists(backup_path):
            shutil.copy2(backup_path, file_path)
            print(f"Restored backup of {file_path}")
        return False

def fix_pac_device(device_name):
    """Fix PAC issues for a single device."""
    print(f"Fixing PAC issues for {device_name}...")

    # Convert device name to lowercase for consistency
    device_name = device_name.lower()

    # Define the PAC directory
    pac_dir = os.path.join("pac", f"{device_name}-pac")

    # Check if the PAC directory exists
    if not os.path.exists(pac_dir):
        print(f"Error: PAC directory for {device_name} not found at {pac_dir}")
        return False

    success = True

    # Fix all Rust files in the src directory
    src_dir = os.path.join(pac_dir, "src")
    if os.path.exists(src_dir):
        # Get all .rs files in the src directory
        for root, dirs, files in os.walk(src_dir):
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    if not fix_rust_file(file_path):
                        success = False
                        print(f"Failed to fix {file_path}")
    else:
        print(f"Warning: src directory not found at {src_dir}")
        success = False

    return success

def fix_html_tags_device(device_name):
    """Fix HTML tag warnings for a single device PAC."""
    print(f"Fixing HTML tag warnings for {device_name}...")

    # Convert device name to lowercase for consistency
    device_name = device_name.lower()

    # Define the PAC directory
    pac_dir = os.path.join("pac", f"{device_name}-pac")

    # Check if the PAC directory exists
    if not os.path.exists(pac_dir):
        print(f"Error: PAC directory for {device_name} not found at {pac_dir}")
        return False

    success = True

    # Fix all Rust files in the src directory
    src_dir = os.path.join(pac_dir, "src")
    if os.path.exists(src_dir):
        # Get all .rs files in the src directory
        for root, dirs, files in os.walk(src_dir):
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    if not fix_html_tags_in_file(file_path):
                        success = False
                        print(f"Failed to fix HTML tags in {file_path}")
    else:
        print(f"Warning: src directory not found at {src_dir}")
        success = False

    return success

def fix_html_tags_in_file(file_path):
    """Fix unclosed HTML tag warnings in a single Rust file."""
    backup_path = None
    try:
        # Read the contents of the Rust file with UTF-8 encoding and error handling
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as file:
            content = file.read()

        # Make a backup of the original file
        backup_path = f"{file_path}.bak"
        shutil.copy2(file_path, backup_path)

        # Fix incorrectly converted function return arrows (-&gt; back to ->)
        content = re.sub(r'-&gt;', '->', content)

        # Fix unclosed HTML tags in doc comments (e.g., "BUSERRSTAT<Master Name>" -> "BUSERRSTAT&lt;Master Name&gt;")
        # Pattern to match ONLY doc comments with angle brackets that could be interpreted as HTML tags
        # This pattern is very specific to avoid affecting function signatures or other code
        doc_comment_pattern = r'(#\[doc\s*=\s*"[^"]*?)<([^<>"]*?)>([^"]*?")'

        def fix_doc_comment(match):
            prefix = match.group(1)
            tag_content = match.group(2)
            suffix = match.group(3)
            # Only apply if the tag_content looks like a proper name/identifier (not code)
            # Avoid fixing things like generic types or function signatures
            if not re.search(r'[-=:()]', tag_content):
                return f'{prefix}&lt;{tag_content}&gt;{suffix}'
            return match.group(0)  # Return unchanged if it looks like code

        # Apply the fix repeatedly until no more matches are found
        # (in case there are multiple angle bracket pairs in a single doc comment)
        prev_content = ""
        while prev_content != content:
            prev_content = content
            content = re.sub(doc_comment_pattern, fix_doc_comment, content)

        # Write the modified content back to the file with UTF-8 encoding
        with open(file_path, 'w', encoding='utf-8') as file:
            file.write(content)

        print(f"Successfully fixed HTML tag warnings in {file_path}")

        # Delete backup if successful
        if os.path.exists(backup_path):
            os.remove(backup_path)

        return True

    except UnicodeDecodeError as e:
        print(f"Unicode encoding error in {file_path}: {str(e)}")
        print(f"Skipping file {file_path} due to encoding issues")
        return True  # Return True to continue processing other files
    except Exception as e:
        print(f"Error processing {file_path}: {str(e)}")
        # Restore backup if exists
        if backup_path and os.path.exists(backup_path):
            shutil.copy2(backup_path, file_path)
            print(f"Restored backup of {file_path}")
        return False

def fix_all_html_tags_devices():
    """Fix HTML tag warnings for all PAC devices."""
    print("Fixing HTML tag warnings for all PAC devices...")

    # Get list of PAC directories
    pac_dirs = [d for d in os.listdir("pac") if os.path.isdir(os.path.join("pac", d)) and d.endswith("-pac")]
    if not pac_dirs:
        print("No PAC directories found.")
        return False

    successes = 0
    failures = 0

    for pac_dir in sorted(pac_dirs):
        # Extract device name from directory name
        device_name = pac_dir.replace("-pac", "")
        if fix_html_tags_device(device_name):
            successes += 1
        else:
            failures += 1

    print(f"\nHTML tag fixing complete:")
    print(f"  ✓ Successful: {successes}")
    print(f"  ✗ Failed: {failures}")

    return failures == 0

def fix_fmt_device(device_name):
    """Fix formatting for a single device PAC."""
    # Convert device name to lowercase for consistency
    device_name = device_name.lower()

    # Define the PAC directory
    pac_dir = os.path.join("pac", f"{device_name}-pac")

    # Check if the PAC directory exists
    if not os.path.exists(pac_dir):
        print(f"Error: PAC directory for {device_name} not found at {pac_dir}")
        return False

    return run_cargo_fmt(pac_dir)

def fix_all_pac_devices():
    """Fix PAC issues for all devices."""
    print("Fixing PAC issues for all devices...")

    # Get list of PAC directories
    pac_dirs = [d for d in os.listdir("pac") if os.path.isdir(os.path.join("pac", d)) and d.endswith("-pac")]
    if not pac_dirs:
        print("No PAC directories found.")
        return False

    successes = 0
    failures = 0

    for pac_dir in sorted(pac_dirs):
        # Extract device name from directory name
        device_name = pac_dir.replace("-pac", "")
        if fix_pac_device(device_name):
            successes += 1
        else:
            failures += 1

    print(f"\nPAC fixing complete:")
    print(f"  ✓ Successful: {successes}")
    print(f"  ✗ Failed: {failures}")

    return failures == 0

def fix_all_fmt_devices():
    """Fix formatting for all PAC devices."""
    print("Fixing formatting for all PAC devices...")

    # Get list of PAC directories
    pac_base_dir = "pac"
    if not os.path.exists(pac_base_dir):
        print(f"Error: PAC base directory '{pac_base_dir}' not found")
        return False

    pac_dirs = [
        os.path.join(pac_base_dir, d)
        for d in os.listdir(pac_base_dir)
        if os.path.isdir(os.path.join(pac_base_dir, d)) and d.endswith("-pac")
    ]

    if not pac_dirs:
        print("No PAC directories found.")
        return False

    print(f"Found {len(pac_dirs)} PAC directories")

    successes = 0
    failures = 0

    for pac_dir in sorted(pac_dirs):
        if run_cargo_fmt(pac_dir):
            successes += 1
        else:
            failures += 1

    print(f"\nFormatting complete:")
    print(f"  ✓ Successful: {successes}")
    print(f"  ✗ Failed: {failures}")

    return failures == 0

def format_cargo_toml(content):
    """
    Format the Cargo.toml content:
    - Add space between word and '='
    - Remove trailing spaces at the end of each line
    - Remove continuous blank lines
    - Ensure only one blank line between sections
    - Add a blank line before `tracing`, `tracing_dummy`, and `rt` sections
    - Add a space after commas in lists
    """
    # Add space between word and '='
    content = re.sub(r'(\w+)=', r'\1 =', content)

    # Add a space after commas in lists (e.g., features = ["all","rt"] -> features = ["all", "rt"])
    content = re.sub(r',(\S)', r', \1', content)

    # Remove trailing spaces at the end of each line
    content = re.sub(r'[ \t]+$', '', content, flags=re.MULTILINE)

    # Replace multiple blank lines with a single blank line
    content = re.sub(r'\n\s*\n+', '\n\n', content)

    # Add a blank line before tracing, tracing_dummy, and rt sections
    content = re.sub(r'(?<=\n)(tracing|tracing_dummy|rt)\s*=.*\n', r'\n\g<0>', content)

    return content

def update_cargo_toml(pac_dir, package_name):
    """
    Update the Cargo.toml file by replacing the [package] section with the manifest template.

    Args:
        pac_dir: Path to the PAC directory
        package_name: Name of the package (e.g., "ra0e1-pac")

    Returns:
        bool: True if successful, False otherwise
    """
    cargo_toml_path = pac_dir / "Cargo.toml"
    if not cargo_toml_path.exists():
        print(f"Error: Cargo.toml not found at {cargo_toml_path}")
        return False

    try:
        # Read the current Cargo.toml
        with open(cargo_toml_path, 'r') as f:
            cargo_content = f.read()

        # Get device name and family name from package_name
        # Example: "ra0e1-pac" -> device_name="ra0e1", family_name="ra"
        device_name = package_name.split('-')[0]
        family_name = device_name[:2] if len(device_name) >= 2 else device_name

        # For Renesas devices, device_name is usually the full device name in uppercase
        if package_name.startswith("ra"):
            DEVICE_NAME = "R7FA" + device_name[1:].upper()
        else:
            DEVICE_NAME = device_name.upper()

        # Replace all placeholders in the template
        updated_manifest = manifest_template.replace("@package_name@", package_name)
        updated_manifest = updated_manifest.replace("@version@", CARGO_VERSION)
        updated_manifest = updated_manifest.replace("@DEVICE_NAME@", DEVICE_NAME)
        updated_manifest = updated_manifest.replace("@device_name@", device_name)
        updated_manifest = updated_manifest.replace("@family_name@", family_name)

        # Replace the [package] section
        # First, find where the [package] section starts and ends
        package_section_pattern = r'^\[package\].*?(?=^\[)'
        updated_content = re.sub(
            package_section_pattern,
            updated_manifest,
            cargo_content,
            flags=re.MULTILINE | re.DOTALL
        )

        # Handle the case where [package] is the last section
        if updated_content == cargo_content:
            package_section_pattern = r'^\[package\].*'
            updated_content = re.sub(
                package_section_pattern,
                updated_manifest,
                cargo_content,
                flags=re.MULTILINE | re.DOTALL
            )

        # Format the content
        updated_content = format_cargo_toml(updated_content)

        # Write back to the file
        with open(cargo_toml_path, 'w') as f:
            f.write(updated_content)

        print(f"Successfully updated Cargo.toml for {package_name}")
        return True

    except Exception as e:
        print(f"Error updating Cargo.toml for {package_name}: {str(e)}")
        return False

def show_usage():
    """Display script usage information."""
    print("Usage: gen_pac.py [options] device_name")
    print("")
    print("Options:")
    print("  -h, --help         Show this help message and exit")
    print("  -s, --svd          Generate patched SVD files only")
    print("  -p, --pac          Generate PACs from patched SVD files only")
    print("  -a, --all          Generate full process (SVD and PAC) - default if no option specified")
    print("  --fix-pac          Fix PAC issues (duplicate attributes, unsafe keywords, etc.)")
    print("  --fix-fmt          Fix formatting using cargo fmt")
    print("  --fix-html         Fix unclosed HTML tag warnings in doc comments")
    print("  --fix-all          Fix both PAC issues and formatting")
    print("")
    print("Arguments:")
    print("  device_name        Mandatory. Process only the specified device.")
    print("                     Use 'ALL' to process all SVD files in the svd directory.")
    print("")
    print("Examples:")
    print("  gen_pac.py ALL               # Process all SVD files (full process)")
    print("  gen_pac.py R7FA4M1AB         # Process only R7FA4M1AB.svd (full process)")
    print("  gen_pac.py -s ALL            # Generate patched SVD files only for all devices")
    print("  gen_pac.py -p R7FA4M1AB      # Generate PAC only for R7FA4M1AB")
    print("  gen_pac.py --fix-pac ALL     # Fix PAC issues for all devices")
    print("  gen_pac.py --fix-fmt ra8m1   # Fix formatting for ra8m1-pac")
    print("  gen_pac.py --fix-html ALL    # Fix HTML tag warnings for all devices")
    print("  gen_pac.py --fix-all ALL     # Fix both PAC issues and formatting for all devices")
    print("")

def check_command_exists(command):
    """Check if a command exists in the system path."""
    if shutil.which(command) is None:
        print(f"{command} could not be found. Install it with the following command:")
        print("")
        if command == "svd2pac":
            print("    cargo install --git https://github.com/Infineon/svd2pac --tag 0.6.1 --locked")
        print("")
        sys.exit(1)

def run_command(command, check=True):
    """Run a shell command and check for errors."""
    try:
        # Let the command output directly to the console instead of capturing it.
        result = subprocess.run(command, check=check, shell=True, text=True)
        print("Command executed successfully.")
        return result
    except subprocess.CalledProcessError as e:
        print(f"Command failed: {e.cmd}")
        if check:
            sys.exit(1)
        return e

def fix_n_to_name_tags(root):
    """
    Fix SVD file by converting non-standard <n> tags to standard <name> tags.
    This is required for CMSIS-SVD compliance as Renesas SVD files use <n> instead of <name>.
    """
    print("Converting <n> tags to <name> tags for CMSIS-SVD compliance...")

    fixed_count = 0

    # Find all <n> elements in the XML tree using iter() which is more reliable
    n_elements = []
    for elem in root.iter():
        if elem.tag == 'n':
            n_elements.append(elem)

    for n_elem in n_elements:
        # Create a new <name> element with the same text content and attributes
        name_elem = ET.Element("name")
        name_elem.text = n_elem.text
        name_elem.tail = n_elem.tail
        # Copy any attributes (though <n> tags shouldn't have any)
        name_elem.attrib = n_elem.attrib.copy()

        # Find the parent element
        parent = None
        for potential_parent in root.iter():
            if n_elem in list(potential_parent):
                parent = potential_parent
                break

        if parent is not None:
            # Find the index of the <n> element
            index = list(parent).index(n_elem)

            # Remove the old <n> element and insert the new <name> element at the same position
            parent.remove(n_elem)
            parent.insert(index, name_elem)

            fixed_count += 1

            # Log the conversion based on the parent element type
            parent_tag = parent.tag
            if parent_tag == "peripheral":
                print(f"  Converted peripheral <n> to <name>: {name_elem.text}")
            elif parent_tag == "register":
                print(f"  Converted register <n> to <name>: {name_elem.text}")
            elif parent_tag == "field":
                print(f"  Converted field <n> to <name>: {name_elem.text}")
            elif parent_tag == "enumeratedValue":
                print(f"  Converted enumeratedValue <n> to <name>: {name_elem.text}")
            else:
                print(f"  Converted {parent_tag} <n> to <name>: {name_elem.text}")

    print(f"Converted {fixed_count} <n> tags to <name> tags")
    return root

# Updated function to target the <access> tag within <register> and <field> elements
def fix_access_types_nested(root):
    """
    Fix access types in SVD XML tree.
    Converts all 'read-writeonce' to 'read-write' within <register> and <field> elements.
    """
    print("Fixing access types...")

    fixed_count = 0
    for register in root.findall(".//register"):
        for field in register.findall(".//field"):
            access_elem = field.find("access")
            if access_elem is not None and access_elem.text == "read-writeonce":
                access_elem.text = "read-write"
                print(f"  Replaced 'read-writeonce' with 'read-write' in field: {field.find('name').text}")
                fixed_count += 1
            if access_elem is not None and access_elem.text == "writeonce":
                access_elem.text = "read-write"
                print(f"  Replaced 'writeonce' with 'read-write' in field: {field.find('name').text}")
                fixed_count += 1

    print(f"Total fixed: {fixed_count}")
    return root

def add_missing_access_tags(root):
    """
    Adds missing 'access' tags with 'read-write' as the default for all fields in the SVD XML.
    """
    print("Checking for missing access tags...")

    # Iterate through all 'field' elements in the SVD XML
    for field in root.findall(".//field"):
        access_elem = field.find("access")

        # If the 'access' tag is missing, add it with 'read-write' as default
        if access_elem is None:
            print(f"Adding 'access' tag to field: {field.find('name').text}")
            access_tag = ET.SubElement(field, "access")
            access_tag.text = "read-write"

    print("Finished adding missing 'access' tags.")
    return root


def fix_access_types(root):
    """Fix access types in SVD file (read,write -> read-write)"""
    print("Fixing access types...")
    for access_elem in root.findall(".//*[@access]"):
        if access_elem.get("access") == "read,write":
            access_elem.set("access", "read-write")

def fix_derived_from(root):
    """Remove derived_from attributes from registers"""
    print("Removing derived_from attributes...")
    for derived in root.findall(".//register[@derivedFrom]"):
        derived.attrib.pop('derivedFrom')

def strip_default_only_enum_blocks(text):
    """Strip enumerated values that contain only isDefault=true and no explicit value."""
    removed = 0

    def repl(match):
        nonlocal removed
        removed += 1
        return ""

    return DEFAULT_ONLY_ENUM_BLOCK_RE.sub(repl, text), removed

def sanitize_enum_variant_name(name):
    """Approximate svd2pac's enum variant naming so we can remove duplicates safely."""
    normalized = re.sub(r'\W+', '_', (name or '').strip())
    if not normalized:
        return ""
    if normalized[0].isdigit():
        normalized = f"_{normalized}"
    return normalized

def get_child_text(element, child_name):
    """Return stripped child text when present."""
    child = element.find(child_name)
    if child is None or child.text is None:
        return None
    return child.text.strip()

def parse_int(text):
    """Parse decimal or hexadecimal integer strings."""
    if text is None:
        return None
    return int(text, 0)

def parse_dim_index(dim_index_text):
    """Expand CMSIS-SVD dimIndex formats like 0-12 or 14,15 into integers."""
    indices = []
    if not dim_index_text:
        return indices

    for part in dim_index_text.split(','):
        part = part.strip()
        if not part:
            continue
        if '-' in part:
            start_text, end_text = part.split('-', 1)
            start = int(start_text)
            end = int(end_text)
            indices.extend(range(start, end + 1))
            continue
        indices.append(int(part))

    return indices

def remove_duplicate_enumerated_values(root):
    """Remove duplicate enum variants within a single enumeratedValues block."""
    print("Removing duplicate enumerated values...")

    removed_count = 0

    for field in root.findall(".//field"):
        field_name_elem = field.find("name")
        field_name = field_name_elem.text if field_name_elem is not None else "unnamed"

        for enum_values in field.findall("./enumeratedValues"):
            seen_names = set()
            duplicates = []

            for enum_value in enum_values.findall("./enumeratedValue"):
                name_elem = enum_value.find("name")
                normalized_name = sanitize_enum_variant_name(
                    name_elem.text if name_elem is not None else ""
                )
                if not normalized_name:
                    continue
                if normalized_name in seen_names:
                    duplicates.append(enum_value)
                    continue
                seen_names.add(normalized_name)

            for duplicate in duplicates:
                name_elem = duplicate.find("name")
                desc_elem = duplicate.find("description")
                duplicate_name = name_elem.text if name_elem is not None else ""
                duplicate_desc = desc_elem.text if desc_elem is not None else ""
                enum_values.remove(duplicate)
                removed_count += 1
                print(
                    f"  Removed duplicate enum variant '{duplicate_name}' from field '{field_name}'"
                    f" ({duplicate_desc})"
                )

    print(f"Removed {removed_count} duplicate enumerated values")

def fix_icu_irqcr_layouts(root, svd_file_path):
    """Normalize split ICU IRQCR register chunks into a single contiguous array."""
    if not os.path.basename(svd_file_path).upper().startswith("R7FA"):
        return root

    for peripheral in root.findall(".//peripheral"):
        if get_child_text(peripheral, "name") != "ICU":
            continue

        registers = peripheral.find("registers")
        if registers is None:
            continue

        irqcr_registers = [
            register
            for register in registers.findall("./register")
            if get_child_text(register, "name") == "IRQCR%s"
        ]

        if len(irqcr_registers) < 2:
            continue

        register_chunks = []
        for register in irqcr_registers:
            offset = parse_int(get_child_text(register, "addressOffset"))
            dim_indices = parse_dim_index(get_child_text(register, "dimIndex"))
            if offset is None or not dim_indices:
                continue
            register_chunks.append((offset, dim_indices, register))

        if len(register_chunks) < 2:
            continue

        register_chunks.sort(key=lambda chunk: chunk[0])
        primary_offset, primary_indices, primary = register_chunks[0]
        all_indices = sorted({index for _, indices, _ in register_chunks for index in indices})

        if primary_offset != 0 or not all_indices:
            continue

        expected_last_index = max(all_indices)
        expected_dim = expected_last_index + 1
        if len(register_chunks) == 1 and len(all_indices) == expected_dim:
            continue

        print(
            f"Fixing ICU IRQCR register layout in {os.path.basename(svd_file_path)}..."
        )

        primary.find("dim").text = str(expected_dim)
        primary.find("dimIncrement").text = "0x1"
        primary.find("dimIndex").text = f"0-{expected_last_index}"
        primary.find("addressOffset").text = "0x000"

        for _, _, register in register_chunks[1:]:
            registers.remove(register)

        address_blocks = peripheral.findall("./addressBlock")
        expected_end = expected_dim
        for address_block in address_blocks:
            offset_text = get_child_text(address_block, "offset")
            size_text = get_child_text(address_block, "size")
            offset = parse_int(offset_text)
            size = parse_int(size_text)
            if offset is None or size is None:
                continue
            if offset == 0:
                address_block.find("size").text = str(expected_dim)
                continue
            if 0 < offset < expected_end:
                peripheral.remove(address_block)

        print(
            f"  Replaced split IRQCR chunks with a contiguous 0-{expected_last_index} definition at offset 0x000"
        )
        break

    return root

def fix_enumerated_values(root):
    """Fix SVD file by adding explicit values to enumerated values with isDefault attribute."""
    print("Fixing enumerated values with isDefault attribute...")

    # Track how many changes we make
    changed_count = 0

    # Find all fields containing enumeratedValues
    for field in root.findall(".//field"):
        # Look for enumeratedValues within this field
        enum_values_list = field.findall("./enumeratedValues")

        for enum_values in enum_values_list:
            # Check if any enumeratedValue in this section has isDefault=true
            default_values = enum_values.findall("./enumeratedValue[isDefault='true']")

            if default_values:
                # Get lsb and msb from the field
                lsb_elem = field.find("lsb")
                msb_elem = field.find("msb")

                if lsb_elem is not None and msb_elem is not None:
                    lsb = int(lsb_elem.text)
                    msb = int(msb_elem.text)
                    bit_width = msb - lsb + 1

                    # Default value should be all zeros with the proper width
                    default_value = "#" + "0" * bit_width

                    # Check if other enum values use a different format
                    bin_format = False
                    dec_format = False
                    max_width = bit_width

                    for enum_value in enum_values.findall("./enumeratedValue/value"):
                        if enum_value.text:
                            if (enum_value.text.startswith('#') and len(enum_value.text) > 1):
                                bin_format = True
                                max_width = max(max_width, len(enum_value.text[1:]))
                            else:
                                try:
                                    int(enum_value.text)
                                    dec_format = True
                                except ValueError:
                                    pass

                    # Add explicit value to the default value(s)
                    for default_value_elem in default_values:
                        # If it already has a value, don't modify
                        if default_value_elem.find("value") is not None:
                            continue

                        # Create the value element with proper formatting
                        value_elem = ET.Element("value")

                        # Use the appropriate format for the value
                        if bin_format:
                            value_elem.text = f"#{'0' * max_width}"
                        elif dec_format:
                            value_elem.text = "0"
                        else:
                            value_elem.text = f"#{'0' * bit_width}"

                        # Simply append the value element to default_value_elem
                        # We'll handle formatting during XML writing
                        default_value_elem.append(value_elem)

                        name_elem = default_value_elem.find("name")
                        name_text = name_elem.text if name_elem is not None else "default"
                        print(f"  Added value {value_elem.text} to default enumerated value '{name_text}'")
                        changed_count += 1
                else:
                    # If we can't find lsb/msb, use a default value of "#0"
                    for default_value_elem in default_values:
                        if default_value_elem.find("value") is not None:
                            continue

                        value_elem = ET.Element("value")
                        value_elem.text = "#0"

                        # Simply append the value element
                        default_value_elem.append(value_elem)

                        name_elem = default_value_elem.find("name")
                        name_text = name_elem.text if name_elem is not None else "default"
                        print(f"  Added value {value_elem.text} to default enumerated value '{name_text}' (no lsb/msb info found)")
                        changed_count += 1

    # Handle any enumeratedValues that are not directly under a field (if any exist)
    for enum_values in root.findall(".//enumeratedValues"):
        # Skip those that we've already processed (those under fields)
        if enum_values.find("..") is not None and enum_values.find("..").tag == "field":
            continue

        # Check if any enumeratedValue in this section has isDefault=true
        default_values = enum_values.findall("./enumeratedValue[isDefault='true']")

        if default_values:
            # For these cases, we don't have field context, so use a default value of "#0"
            for default_value_elem in default_values:
                if default_value_elem.find("value") is not None:
                    continue

                value_elem = ET.Element("value")
                value_elem.text = "#0"

                # Simply append the value element
                default_value_elem.append(value_elem)

                name_elem = default_value_elem.find("name")
                name_text = name_elem.text if name_elem is not None else "default"
                print(f"  Added value {value_elem.text} to default enumerated value '{name_text}' (no field context)")
                changed_count += 1

    print(f"Fixed {changed_count} enumerated values with isDefault attribute")

def fix_enumerated_value_ranges(root):
    """Fix SVD file by ensuring enumerated values are within valid range based on field bit width."""
    print("Fixing enumerated values with out-of-range values...")

    # Track how many changes we make
    fixed_count = 0

    # First pass: Handle standard fields with enumeratedValues
    for field in root.findall(".//field"):
        # Get field name for logging
        field_name_elem = field.find("name")
        field_name = field_name_elem.text if field_name_elem is not None else "unnamed"

        # Get bit width from lsb and msb
        lsb_elem = field.find("lsb")
        msb_elem = field.find("msb")

        # If lsb and msb are available, calculate bit width
        if lsb_elem is not None and msb_elem is not None:
            lsb = int(lsb_elem.text)
            msb = int(msb_elem.text)
            bit_width = msb - lsb + 1
            max_value = (1 << bit_width) - 1  # Calculate maximum value: 2^bit_width - 1
        # Try alternative: check for bitWidth element
        else:
            bit_width_elem = field.find("bitWidth")
            if bit_width_elem is not None:
                bit_width = int(bit_width_elem.text)
                max_value = (1 << bit_width) - 1
            # Check for bitOffset element with bitWidth
            elif field.find("bitOffset") is not None and field.find("bitWidth") is not None:
                bit_width = int(field.find("bitWidth").text)
                max_value = (1 << bit_width) - 1
            # Fallback for fields with just bitField elements
            elif field.find("bitField") is not None:
                # Get all bitFields and determine max bit position
                bit_fields = field.findall("bitField")
                max_bit = 0
                for bit_field in bit_fields:
                    pos_elem = bit_field.find("position")
                    if pos_elem is not None and pos_elem.text:
                        max_bit = max(max_bit, int(pos_elem.text))
                bit_width = max_bit + 1
                max_value = (1 << bit_width) - 1
            else:
                # If we can't determine bit width, assume 1-bit (conservative)
                bit_width = 1
                max_value = 1
                print(f"  Warning: Couldn't determine bit width for field '{field_name}', assuming 1-bit")

        # Look for enumeratedValues within this field
        enum_values_list = field.findall("./enumeratedValues")

        for enum_values in enum_values_list:
            # Check all enumeratedValue items
            for enum_value in enum_values.findall("./enumeratedValue"):
                value_elem = enum_value.find("value")
                name_elem = enum_value.find("name")

                if value_elem is not None and value_elem.text:
                    value_text = value_elem.text
                    name = name_elem.text if name_elem is not None else "unnamed"

                    try:
                        # Handle binary format (#10101)
                        if value_text.startswith('#'):
                            # Convert binary string to integer
                            int_value = int(value_text[1:], 2)
                        # Handle hexadecimal format (0x...)
                        elif value_text.lower().startswith('0x'):
                            int_value = int(value_text, 0)  # Base 0 auto-detects hex/octal/decimal
                        else:
                            # Regular decimal value
                            int_value = int(value_text)

                        # Check if the value exceeds the maximum allowed
                        if int_value > max_value:
                            print(f"  Field '{field_name}': Value {int_value} for '{name}' exceeds max {max_value} (bit width: {bit_width})")

                            # Truncate the value to fit within the bit width
                            new_value = int_value & max_value

                            # Update the value in the appropriate format
                            if value_text.startswith('#'):
                                # Keep binary format with same length padding
                                binary_width = len(value_text) - 1
                                binary_str = bin(new_value)[2:]  # Remove '0b' prefix
                                value_elem.text = f"#{'0' * (binary_width - len(binary_str))}{binary_str}"
                            elif value_text.lower().startswith('0x'):
                                # Keep hex format with same prefix
                                value_elem.text = hex(new_value)
                            else:
                                value_elem.text = str(new_value)

                            print(f"    Fixed: Changed value to {value_elem.text}")
                            fixed_count += 1

                    except ValueError:
                        # Skip values that can't be parsed (invalid format)
                        print(f"  Warning: Could not parse value '{value_text}' for '{name}' in field '{field_name}'")
                        continue

    # Second pass: Handle standalone enumeratedValues (not under a field)
    for enum_values in root.findall(".//enumeratedValues"):
        # Skip those under fields (already processed)
        parent = enum_values.getparent() if hasattr(enum_values, 'getparent') else enum_values.find("..")
        if parent is not None and parent.tag == "field":
            continue

        # Find all enumeratedValue elements
        for enum_value in enum_values.findall("./enumeratedValue"):
            value_elem = enum_value.find("value")
            name_elem = enum_value.find("name")

            if value_elem is not None and value_elem.text:
                value_text = value_elem.text
                name = name_elem.text if name_elem is not None else "unnamed"

                # For standalone enum values, we need to make an educated guess about bit width
                # Look for patterns in other values in the same enumeratedValues group
                bit_width = None
                max_val = 0

                # Find max value to determine required bit width
                for ev in enum_values.findall("./enumeratedValue/value"):
                    if ev is not None and ev.text:
                        try:
                            val = int(ev.text, 0) if ev.text.lower().startswith('0x') else \
                                  int(ev.text[1:], 2) if ev.text.startswith('#') else int(ev.text)
                            max_val = max(max_val, val)
                        except ValueError:
                            continue

                # Calculate bit width based on max value
                if max_val > 0:
                    bit_width = max_val.bit_length()
                    max_value = (1 << bit_width) - 1
                else:
                    # Default to 32-bit if we can't determine
                    bit_width = 32
                    max_value = 0xFFFFFFFF

                try:
                    # Parse the value
                    if value_text.startswith('#'):
                        int_value = int(value_text[1:], 2)
                    elif value_text.lower().startswith('0x'):
                        int_value = int(value_text, 0)
                    else:
                        int_value = int(value_text)

                    # Check if value is valid (simple check for standalone enums)
                    if int_value > max_value:
                        print(f"  Standalone enumeratedValue: Value {int_value} for '{name}' exceeds max {max_value}")

                        # Truncate the value
                        new_value = int_value & max_value

                        # Update in the appropriate format
                        if value_text.startswith('#'):
                            binary_width = len(value_text) - 1
                            binary_str = bin(new_value)[2:]  # Remove '0b' prefix
                            value_elem.text = f"#{'0' * (binary_width - len(binary_str))}{binary_str}"
                        elif value_text.lower().startswith('0x'):
                            value_elem.text = hex(new_value)
                        else:
                            value_elem.text = str(new_value)

                        print(f"    Fixed: Changed value to {value_elem.text}")
                        fixed_count += 1

                except ValueError:
                    print(f"  Warning: Could not parse standalone value '{value_text}' for '{name}'")
                    continue

    print(f"Fixed {fixed_count} out-of-range enumerated values")

def fix_write_constraints(root):
    """Fix SVD file by ensuring writeConstraint ranges are within valid bit width limits for their fields."""
    print("Fixing write constraints with out-of-range values...")

    # Track how many changes we make
    fixed_count = 0

    # Find all fields containing writeConstraint elements
    for field in root.findall(".//field"):
        # Get field name for logging
        field_name_elem = field.find("name")
        field_name = field_name_elem.text if field_name_elem is not None else "unnamed"

        # Find writeConstraint element
        write_constraint = field.find("./writeConstraint")
        if write_constraint is None:
            continue

        # Check for range constraints
        range_elem = write_constraint.find("./range")
        if range_elem is None:
            continue

        # Get minimum and maximum values
        minimum_elem = range_elem.find("./minimum")
        maximum_elem = range_elem.find("./maximum")

        if minimum_elem is None or maximum_elem is None:
            continue

        # Get bit width from lsb and msb
        lsb_elem = field.find("lsb")
        msb_elem = field.find("msb")

        # Calculate bit width
        if lsb_elem is not None and msb_elem is not None:
            lsb = int(lsb_elem.text)
            msb = int(msb_elem.text)
            bit_width = msb - lsb + 1
            max_allowed = (1 << bit_width) - 1  # Calculate maximum value: 2^bit_width - 1
        else:
            # Try alternative: check for bitWidth element
            bit_width_elem = field.find("bitWidth")
            if bit_width_elem is not None:
                bit_width = int(bit_width_elem.text)
                max_allowed = (1 << bit_width) - 1
            # Check for bitOffset element with bitWidth
            elif field.find("bitOffset") is not None and field.find("bitWidth") is not None:
                bit_width = int(field.find("bitWidth").text)
                max_allowed = (1 << bit_width) - 1
            else:
                # If we can't determine bit width, log a warning and skip
                print(f"  Warning: Couldn't determine bit width for field '{field_name}' with writeConstraint, skipping")
                continue

        # Parse minimum and maximum values
        try:
            min_text = minimum_elem.text
            max_text = maximum_elem.text

            # Handle different number formats (hex, decimal)
            if min_text.lower().startswith('0x'):
                min_val = int(min_text, 0)  # Base 0 for auto-detection
            else:
                min_val = int(min_text)

            if max_text.lower().startswith('0x'):
                max_val = int(max_text, 0)
            else:
                max_val = int(max_text)

            # Check if values exceed the allowed range
            need_fix = False

            if min_val > max_allowed:
                print(f"  Field '{field_name}': Minimum value {min_val} exceeds max allowed {max_allowed} (bit width: {bit_width})")
                min_val = min_val & max_allowed
                need_fix = True

            if max_val > max_allowed:
                print(f"  Field '{field_name}': Maximum value {max_val} exceeds max allowed {max_allowed} (bit width: {bit_width})")
                max_val = max_val & max_allowed
                need_fix = True

            # Update values if needed while preserving format
            if need_fix:
                if min_text.lower().startswith('0x'):
                    minimum_elem.text = hex(min_val)
                else:
                    minimum_elem.text = str(min_val)

                if max_text.lower().startswith('0x'):
                    maximum_elem.text = hex(max_val)
                else:
                    maximum_elem.text = str(max_val)

                print(f"    Fixed: Changed range to [{minimum_elem.text} - {maximum_elem.text}]")
                fixed_count += 1

        except ValueError:
            print(f"  Warning: Could not parse writeConstraint values for field '{field_name}', skipping")
            continue

    print(f"Fixed {fixed_count} out-of-range write constraints")

def remove_empty_name_enumerated_values(root):
    """Remove enumerated values that have empty name tags."""
    print("Removing enumerated values with empty name tags...")

    # Track how many elements we remove
    removed_count = 0
    removed_parent_count = 0

    # Find all enumeratedValues elements first
    for enum_values in root.findall(".//enumeratedValues"):
        # We'll collect the indices of children to remove
        to_remove = []

        # Check each enumeratedValue child
        for i, enum_value in enumerate(enum_values.findall("./enumeratedValue")):
            # Try to find the name element
            name_elem = enum_value.find("name")

            # Empty name tag can be either:
            # 1. <name /> (self-closing tag)
            # 2. <name></name> (empty content)
            # 3. <name>   </name> (whitespace only)
            if name_elem is not None and (name_elem.text is None or name_elem.text.strip() == ""):
                # Get description and value for logging
                desc_elem = enum_value.find("description")
                desc_text = desc_elem.text if desc_elem is not None and desc_elem.text else "No description"

                value_elem = enum_value.find("value")
                value_text = value_elem.text if value_elem is not None and value_elem.text else "No value"

                print(f"  Found enumerated value with empty name: description='{desc_text}', value='{value_text}'")
                to_remove.append(i)

        # Remove the elements from last to first to avoid index shifting
        for idx in sorted(to_remove, reverse=True):
            # Use direct index access since we're working with a parent element
            try:
                # enumeratedValues elements typically contain multiple enumeratedValue children
                children = list(enum_values)
                if idx < len(children):
                    enum_values.remove(children[idx])
                    removed_count += 1
                    print(f"  Removed enumerated value at index {idx}")
            except Exception as e:
                print(f"  Warning: Could not remove enumerated value: {str(e)}")

        # Check if the enumeratedValues element is now empty
        # If it contains no more enumeratedValue children, remove it too
        if len(enum_values.findall("./enumeratedValue")) == 0:
            # Find the parent of enumeratedValues
            parent = None
            for potential_parent in root.findall(".//*"):
                if enum_values in potential_parent:
                    parent = potential_parent
                    break

            if parent is not None:
                print(f"  Removing empty enumeratedValues element")
                parent.remove(enum_values)
                removed_parent_count += 1
            else:
                print(f"  Warning: Could not find parent for empty enumeratedValues element")

    print(f"Removed {removed_count} enumerated values with empty name tags and {removed_parent_count} empty enumeratedValues elements")

    # If no elements were removed but we found some, it might be due to a different XML structure
    # Let's try an alternative approach for more complex XML structures
    if removed_count == 0:
        print("Trying alternative approach to remove enumerated values with empty names...")

        # This more direct approach modifies the XML tree by replacing enumeratedValues elements
        for parent in root.findall(".//*[enumeratedValues]"):
            for enum_values in parent.findall("./enumeratedValues"):
                # Create a new enumeratedValues element
                new_enum_values = ET.Element("enumeratedValues")

                # Copy only valid enumeratedValue elements
                valid_count = 0
                total_count = 0
                for enum_value in enum_values.findall("./enumeratedValue"):
                    total_count += 1
                    name_elem = enum_value.find("name")
                    if name_elem is None or (name_elem.text is not None and name_elem.text.strip() != ""):
                        # This is a valid element, copy it
                        new_enum_values.append(enum_value)
                        valid_count += 1
                    else:
                        # Log the skipped element
                        desc_elem = enum_value.find("description")
                        desc_text = desc_elem.text if desc_elem is not None and desc_elem.text else "No description"

                        value_elem = enum_value.find("value")
                        value_text = value_elem.text if value_elem is not None and value_elem.text else "No value"

                        print(f"  Skipping enumerated value with empty name: description='{desc_text}', value='{value_text}'")
                        removed_count += 1

                # If all enumeratedValue elements were removed, remove the entire enumeratedValues element
                if valid_count == 0:
                    parent.remove(enum_values)
                    removed_parent_count += 1
                    print(f"  Removed empty enumeratedValues element")
                # Otherwise, replace the old enumeratedValues with the new one if we removed any elements
                elif valid_count < total_count:
                    idx = list(parent).index(enum_values)
                    parent.remove(enum_values)
                    parent.insert(idx, new_enum_values)
                    print(f"  Replaced enumeratedValues element with cleaned version")

        print(f"Alternative approach: Removed {removed_count} enumerated values with empty name tags and {removed_parent_count} empty enumeratedValues elements")

def fix_missing_placeholders(root):
    """Fix SVD file by correcting register names with dimension attributes but missing %s placeholder."""
    print("Fixing register names with missing %s placeholders...")

    # Track how many names we fix
    fixed_count = 0

    # Find all registers with dimension attributes
    for register in root.findall(".//register"):
        dim_elem = register.find("dim")
        if dim_elem is None or not dim_elem.text:
            continue

        name_elem = register.find("name")
        if name_elem is None or not name_elem.text:
            continue

        register_name = name_elem.text

        # Check if the register name has a %s placeholder
        if "%s" not in register_name:
            print(f"  Found register with dimension attributes but no %s placeholder: {register_name}")

            # We'll try to identify a character that should be replaced with %s
            # Let's check the alternateRegister attribute first
            alt_reg = register.find("alternateRegister")
            if alt_reg is not None and alt_reg.text and "%s" in alt_reg.text:
                # Extract the pattern from alternateRegister
                alt_pattern = alt_reg.text

                # Find where the %s is in the alternate register name
                alt_parts = alt_pattern.split("%s")

                # Try to locate the corresponding section in the actual register name
                if len(alt_parts) == 2:
                    prefix = alt_parts[0]
                    suffix = alt_parts[1]

                    # If the register name starts with the prefix and ends with the suffix,
                    # the character(s) in between should be replaced with %s
                    if register_name.startswith(prefix) and register_name.endswith(suffix):
                        middle = register_name[len(prefix):-len(suffix) if len(suffix) > 0 else None]
                        if middle:
                            # Replace the middle part with %s
                            new_name = f"{prefix}%s{suffix}"
                            print(f"    Fixing name: {register_name} → {new_name}")
                            name_elem.text = new_name
                            fixed_count += 1
                            continue

            # If we couldn't fix using alternateRegister, try looking for a pattern in sibling registers
            parent = register.getparent() if hasattr(register, 'getparent') else None
            if parent is not None:
                # Find sibling registers with similar names
                siblings = []
                for sibling in parent.findall("./register"):
                    # Skip the current register
                    if sibling == register:
                        continue

                    sibling_name = sibling.find("name")
                    if sibling_name is not None and sibling_name.text:
                        siblings.append(sibling_name.text)

                # Look for registers with similar names but with %s
                similar_name_found = False
                for sibling_name in siblings:
                    if "%s" in sibling_name:
                        # Check if the sibling name has a similar pattern
                        # by replacing %s with a character and comparing
                        for char in "0123456789abcdefghijklmnopqrstuvwxyz":
                            test_name = sibling_name.replace("%s", char)

                            # Calculate similarity based on character substitution
                            if len(register_name) == len(test_name):
                                differing_positions = [(i, register_name[i], test_name[i])
                                                      for i in range(len(register_name))
                                                      if register_name[i] != test_name[i]]

                                # If there's only one difference and it matches our test character,
                                # we've found the character to replace with %s
                                if len(differing_positions) == 1:
                                    pos, reg_char, test_char = differing_positions[0]
                                    if test_char == char:
                                        # Replace the character at this position with %s
                                        new_name = register_name[:pos] + "%s" + register_name[pos+1:]
                                        print(f"    Fixing name by similarity: {register_name} → {new_name}")
                                        name_elem.text = new_name
                                        fixed_count += 1
                                        similar_name_found = True
                                        break

                        if similar_name_found:
                            break

            # If we still couldn't fix the name, check for specific patterns we know about
            if register_name == "SQCH1DSCmDR_H":
                new_name = "SQCH1DSC%sDR_H"
                print(f"    Fixing name by known pattern: {register_name} → {new_name}")
                name_elem.text = new_name
                fixed_count += 1

    print(f"Fixed {fixed_count} register names with missing %s placeholders")

def fix_reset_values(root):
    """Fix SVD file by ensuring resetValue is within valid range according to resetMask."""
    print("Fixing out-of-range resetValues...")

    # Track how many values we fix
    fixed_count = 0

    # Find all registers with resetValue and resetMask
    for register in root.findall(".//register"):
        reset_value_elem = register.find("resetValue")
        reset_mask_elem = register.find("resetMask")

        # Skip if either element is missing
        if reset_value_elem is None or reset_mask_elem is None:
            continue

        # Get register name for logging
        name_elem = register.find("name")
        register_name = name_elem.text if name_elem is not None and name_elem.text else "unnamed"

        try:
            # Parse values (support decimal, hex, binary)
            if reset_value_elem.text.lower().startswith("0x"):
                reset_value = int(reset_value_elem.text, 16)
            elif reset_value_elem.text.startswith("#"):
                reset_value = int(reset_value_elem.text[1:], 2)
            else:
                reset_value = int(reset_value_elem.text)

            if reset_mask_elem.text.lower().startswith("0x"):
                reset_mask = int(reset_mask_elem.text, 16)
            elif reset_mask_elem.text.startswith("#"):
                reset_mask = int(reset_mask_elem.text[1:], 2)
            else:
                reset_mask = int(reset_mask_elem.text)

            # Check if the resetValue has bits set outside the resetMask
            if (reset_value & ~reset_mask) != 0:
                # Fix by masking the resetValue with resetMask
                original_value = reset_value
                fixed_value = reset_value & reset_mask

                # Preserve the original format (hex, decimal) when updating
                if reset_value_elem.text.lower().startswith("0x"):
                    reset_value_elem.text = f"0x{fixed_value:X}"
                elif reset_value_elem.text.startswith("#"):
                    # Convert to binary and preserve width
                    binary_width = len(reset_value_elem.text) - 1
                    binary_str = bin(fixed_value)[2:]  # Remove '0b' prefix
                    reset_value_elem.text = f"#{'0' * (binary_width - len(binary_str))}{binary_str}"
                else:
                    reset_value_elem.text = str(fixed_value)

                print(f"  Register '{register_name}': Fixed resetValue from {hex(original_value)} to {hex(fixed_value)} (mask: {hex(reset_mask)})")
                fixed_count += 1

        except ValueError:
            print(f"  Warning: Could not parse resetValue or resetMask for register '{register_name}'")
            continue

    print(f"Fixed {fixed_count} out-of-range resetValues")

def fix_missing_base_peripherals(root):
    """Fix SVD file by adding missing base peripherals that are referenced by derived peripherals."""
    print("Checking for missing base peripheral references...")

    # Track how many peripherals we add
    added_count = 0

    # Find all derived peripherals and their references
    derived_peripherals = []
    for peripheral in root.findall(".//peripheral[@derivedFrom]"):
        derived_from = peripheral.get("derivedFrom")
        if derived_from:
            derived_peripherals.append(derived_from)

    # Find all existing peripheral names
    existing_peripherals = set()
    for peripheral in root.findall(".//peripheral"):
        name_elem = peripheral.find("name") or peripheral.find("n")
        if name_elem is not None and name_elem.text:
            existing_peripherals.add(name_elem.text)

    # Find missing base peripherals
    missing_peripherals = set(derived_peripherals) - existing_peripherals

    if missing_peripherals:
        print(f"Found missing base peripherals: {', '.join(missing_peripherals)}")

        # Add SSIE0 peripheral if missing (specific fix for RA8 devices)
        if "SSIE0" in missing_peripherals:
            print("Adding missing SSIE0 peripheral definition...")
            ssie0_xml = '''
        <peripheral>
            <name>SSIE0</name>
            <description>Serial Sound Interface Enhanced (SSIE)</description>
            <baseAddress>0x4025D000</baseAddress>
            <addressBlock>
                <offset>0x00</offset>
                <size>8</size>
                <usage>registers</usage>
            </addressBlock>
            <addressBlock>
                <offset>0x10</offset>
                <size>24</size>
                <usage>registers</usage>
            </addressBlock>
            <registers>
                <register>
                    <name>SSICR</name>
                    <description>Control Register</description>
                    <addressOffset>0x00</addressOffset>
                    <size>32</size>
                    <access>read-write</access>
                    <resetValue>0x00000000</resetValue>
                    <resetMask>0xFFFFFFFF</resetMask>
                    <fields>
                        <field>
                            <name>REN</name>
                            <description>Receive Enable</description>
                            <lsb>0</lsb>
                            <msb>0</msb>
                            <access>read-write</access>
                        </field>
                        <field>
                            <name>TEN</name>
                            <description>Transmit Enable</description>
                            <lsb>1</lsb>
                            <msb>1</msb>
                            <access>read-write</access>
                        </field>
                    </fields>
                </register>
            </registers>
        </peripheral>'''

            # Parse the XML and add it to the peripherals section
            try:
                # Find the peripherals element
                peripherals_elem = root.find("peripherals")
                if peripherals_elem is not None:
                    # Parse the SSIE0 peripheral XML
                    ssie0_elem = ET.fromstring(ssie0_xml.strip())
                    peripherals_elem.append(ssie0_elem)
                    added_count += 1
                    print("Successfully added SSIE0 peripheral")
                else:
                    print("Warning: Could not find peripherals element")
            except ET.ParseError as e:
                print(f"Warning: Failed to parse SSIE0 peripheral XML: {e}")

    print(f"Added {added_count} missing base peripherals")
    return root

def preprocess_svd_file(svd_file_path):
    """
    Preprocess SVD file to fix issues that might cause problems with svd2pac.
    Returns the path to the preprocessed file.
    """
    # Create a temporary directory for preprocessed files
    temp_dir = tempfile.mkdtemp()

    try:
        raw_text = Path(svd_file_path).read_text(encoding='utf-8')
        processed_text, removed_default_only = strip_default_only_enum_blocks(raw_text)
        processed_text, removed_empty_enums = EMPTY_ENUM_VALUES_RE.subn("", processed_text)

        if removed_default_only:
            print(f"Removed {removed_default_only} default-only enumeratedValue blocks")
        if removed_empty_enums:
            print(f"Removed {removed_empty_enums} empty enumeratedValues blocks")

        root = ET.fromstring(processed_text)

        # Apply fixes based on configuration
        # IMPORTANT: Fix <n> to <name> tags FIRST before any other processing
        if FIX_N_TO_NAME_TAGS:
            fix_n_to_name_tags(root)

        fix_icu_irqcr_layouts(root, svd_file_path)

        if FIX_DERIVED_FROM:
            fix_derived_from(root)

        if FIX_ENUMERATED_VALUES:
            fix_enumerated_values(root)

        if FIX_ENUMERATED_VALUE_RANGES:
            fix_enumerated_value_ranges(root)

        if FIX_WRITE_CONSTRAINTS:
            fix_write_constraints(root)

        if FIX_EMPTY_NAME_ENUMERATED_VALUES:
            remove_empty_name_enumerated_values(root)

        remove_duplicate_enumerated_values(root)

        if FIX_MISSING_PLACEHOLDERS:
            fix_missing_placeholders(root)

        if FIX_RESET_VALUES:
            fix_reset_values(root)

        if FIX_ACCESS_TYPES:
            fix_access_types(root)

        if FIX_ACCESS_TYPES_NESTED:
            fix_access_types_nested(root)

        if FIX_MISSING_ACCESS_TAGS:
            add_missing_access_tags(root)

        # Get the filename
        filename = os.path.basename(svd_file_path)

        # Write the modified XML to a temporary file with proper formatting
        temp_file_path = os.path.join(temp_dir, filename)

        # Use our custom XML writing function that preserves formatting
        write_formatted_xml(root, temp_file_path)

        return temp_file_path, temp_dir

    except Exception as e:
        print(f"Error preprocessing XML in {svd_file_path}: {str(e)}")
        sys.exit(1)

def write_formatted_xml(root, file_path):
    """
    Write XML with proper formatting for value elements.
    This specifically ensures proper formatting for value elements added to enumeratedValue elements.
    """
    import re

    # First convert the XML to a string
    xml_string = ET.tostring(root, encoding='utf-8').decode('utf-8')

    # Use regex to find the pattern where a value tag follows an isDefault tag
    # This pattern looks for <isDefault>true</isDefault> followed by <value> tag
    pattern = r'(<isDefault>true</isDefault>)(<value>[^<]+</value>)'

    # Replace with properly formatted version (add a line break and indentation)
    def format_value_tag(match):
        is_default_tag = match.group(1)
        value_tag = match.group(2)

        # Find the indentation of the isDefault tag by looking at the characters before it
        start_idx = match.start(1)
        line_start = xml_string.rfind('\n', 0, start_idx)
        if line_start == -1:
            line_start = 0

        # Calculate the indentation (spaces before isDefault tag)
        indent = xml_string[line_start:start_idx]

        # Return the formatted string with line break and indentation
        return f"{is_default_tag}\n{indent}{value_tag}"

    # Apply the formatting
    formatted_xml = re.sub(pattern, format_value_tag, xml_string)

    # Write with XML declaration
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write('<?xml version="1.0" encoding="utf-8"?>\n')
        f.write(formatted_xml)

def process_device(device_name, patch_only=False, pac_only=False):
    """
    Process a single device SVD file.

    Args:
        device_name: Name of the device (without .svd extension)
        patch_only: If True, only generate patched SVD files, skip PAC generation
        pac_only: If True, skip SVD preprocessing and use existing patched files

    Returns:
        bool: True if processing was successful
    """
    svd_file = f"svd/{device_name}.svd"
    patched_svd_file = f"patched_svd/{device_name}.svd"

    # Check if the SVD file exists (unless we're only generating PACs from existing files)
    if not pac_only and not os.path.exists(svd_file):
        print(f"SVD file {svd_file} not found.")
        return False

    print(f"\n===== Processing device: {device_name} =====")

    # Skip SVD preprocessing if we're only generating PACs from existing patched files
    temp_dir = None
    preprocessed_svd = None
    if not pac_only:
        # Preprocess the SVD file
        print(f"Preprocessing SVD file {device_name}...")
        preprocessed_svd, temp_dir = preprocess_svd_file(svd_file)

        # Ensure patched_svd directory exists
        os.makedirs("patched_svd", exist_ok=True)

        # Copy the preprocessed SVD file to the patched_svd directory
        dst = patched_svd_file
        try:
            shutil.copyfile(preprocessed_svd, dst)
            print(f"Copied {preprocessed_svd} to {dst}")
        except Exception as e:
            print(f"Error copying preprocessed SVD file: {str(e)}")
            return False
    else:
        print(f"PAC-only mode: Using existing patched SVD file for {device_name}")
        if not os.path.exists(patched_svd_file):
            print(f"Error: Patched SVD file {patched_svd_file} not found.")
            return False

    # If patch_only mode, we're done
    if patch_only:
        print(f"SVD-only mode: Skipping PAC generation for {device_name}")
        # Clean up temporary directory before exiting
        if temp_dir and os.path.exists(temp_dir):
            shutil.rmtree(temp_dir)
        return True

    try:
        # Create directories (package name)
        if (device_name.startswith("R7FA")):
            package_name = device_name.lower()[0] + device_name.lower()[3:7] + "-pac"
        elif (device_name.startswith("R7KA")):
            package_name = device_name.lower()[0] + device_name.lower()[3:7] + "-pac"
        elif (device_name.startswith("DA")):
            package_name = device_name.lower() + "-pac"
        elif (device_name.startswith("U5")):
            package_name = device_name.lower() + "-pac"
        else:
            print(f"Unknown device name format: {device_name}")
            return False

        pac_dir = Path(f"pac/{package_name}")
        pac_dir.mkdir(parents=True, exist_ok=True)

        # If there's a patched SVD file available, try using that
        patched_svd = os.path.join("patched_svd", f"{device_name.upper()}.svd")
        if os.path.exists(patched_svd):
            print(f"Found patched SVD file: {patched_svd}")
            print(f"Attempting to run svd2pac with the patched file {patched_svd} to create {package_name}...")
            # Use quotes for the file paths to correctly handle backslashes on Windows
            patched_cmd = f'svd2pac "{patched_svd}" "{pac_dir}" --target cortex-m --tracing --license-file LICENSE.txt --package-name {package_name}'
            print(f"Running command: {patched_cmd}")
            result = run_command(patched_cmd, check=False)

            if result.returncode != 0:
                print(f"Warning: svd2pac failed for {device_name}")
                print(f"Output: {result.stdout}")
                print(f"Error: {result.stderr}")
                if not pac_only:
                    # Save the problematic SVD file for debugging
                    debug_dir = Path("debug_svd")
                    debug_dir.mkdir(exist_ok=True)
                    debug_file = debug_dir / f"{device_name}_debug.svd"
                    if preprocessed_svd is not None:
                        shutil.copyfile(preprocessed_svd, debug_file)
                        print(f"Saved problematic SVD file to {debug_file} for debugging")
                return False
            else:
                print(f"Successfully generated files using patched SVD file")
        else:
            print(f"No patched SVD file found for {device_name}")
            print(f"You may need to manually create a patched SVD file at: {patched_svd}")
            return False

        # Format files in src directory
        src_dir_path = pac_dir / "src"
        if src_dir_path.exists():
            print(f"Formatting all Rust files in {src_dir_path}...")
            rs_files = list(src_dir_path.glob("**/*.rs"))
            if rs_files:
                formatted_files = [str(file) for file in rs_files]
                print(f"Found {len(formatted_files)} Rust files to format")
                run_command(f"rustfmt {' '.join(formatted_files)}", check=False)
            else:
                print(f"No Rust files found in {src_dir_path}")
        else:
            print(f"Warning: {src_dir_path} not found for formatting")

        # Check and generate docs for the device PAC
        if pac_dir.exists():
            # Update Cargo.toml with the manifest template
            update_cargo_toml(pac_dir, package_name)
            # Change to the pac directory
            os.chdir(pac_dir)
            try:
                run_command("cargo fix --all --allow-dirty", check=False)
                run_command("cargo fmt --all", check=False)
                # Remove old documentation if it exists
                doc_path = pac_dir / "target" / "doc"
                if doc_path.exists():
                    shutil.rmtree(doc_path)
                run_command("cargo doc --no-deps", check=False)
                # Print path to documentation
                DOC_PATH = doc_path / f"{package_name.replace('-', '_')}" / "index.html"
                if DOC_PATH.exists():
                    print(f"Documentation generated at: {DOC_PATH}")
            finally:
                os.chdir("../..")  # Go back to the original directory

        return True

    finally:
        # Clean up temporary directory
        if temp_dir and os.path.exists(temp_dir):
            shutil.rmtree(temp_dir)

def main():
    # Parse command-line arguments
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("-h", "--help", action="store_true", help="Show usage information and exit")
    parser.add_argument("-s", "--svd", action="store_true", help="Generate patched SVD files only")
    parser.add_argument("-p", "--pac", action="store_true", help="Generate PACs from patched SVD files only")
    parser.add_argument("-a", "--all", action="store_true", help="Generate full process (SVD and PAC)")
    parser.add_argument("--fix-pac", action="store_true", help="Fix PAC issues (duplicate attributes, unsafe keywords, etc.)")
    parser.add_argument("--fix-fmt", action="store_true", help="Fix formatting using cargo fmt")
    parser.add_argument("--fix-html", action="store_true", help="Fix unclosed HTML tag warnings in doc comments")
    parser.add_argument("--fix-all", action="store_true", help="Fix both PAC issues and formatting")
    parser.add_argument("device_name", help="Process the specified device or 'ALL' for all devices")

    args = parser.parse_args()

    # Show usage information if requested
    if args.help:
        show_usage()
        sys.exit(0)

    # Handle fix modes
    if args.fix_pac or args.fix_fmt or args.fix_html or args.fix_all:
        # Check if cargo is available for formatting
        if args.fix_fmt or args.fix_all:
            try:
                subprocess.run(["cargo", "--version"], capture_output=True, check=True)
            except (subprocess.CalledProcessError, FileNotFoundError):
                print("Error: cargo not found. Please install Rust and Cargo")
                sys.exit(1)

        # Check if we're in the right directory (should contain 'pac' folder)
        if not os.path.exists("pac"):
            print("Error: This script should be run from the root of the ren-pac repository")
            print("Current directory should contain a 'pac' folder")
            sys.exit(1)

        # Process all devices if ALL is specified for fix modes
        if args.device_name.upper() == "ALL":
            success = True

            if args.fix_all:
                print("Fixing both PAC issues and formatting for all devices...")
                if not fix_all_pac_devices():
                    success = False
                if not fix_all_fmt_devices():
                    success = False
            elif args.fix_pac:
                success = fix_all_pac_devices()
            elif args.fix_fmt:
                success = fix_all_fmt_devices()
            elif args.fix_html:
                success = fix_all_html_tags_devices()

        else:
            # Process single device for fix modes
            device_name = args.device_name
            success = True

            if args.fix_all:
                print(f"Fixing both PAC issues and formatting for {device_name}...")
                if not fix_pac_device(device_name):
                    success = False
                if not fix_fmt_device(device_name):
                    success = False
            elif args.fix_pac:
                success = fix_pac_device(device_name)
            elif args.fix_fmt:
                success = fix_fmt_device(device_name)
            elif args.fix_html:
                success = fix_html_tags_device(device_name)

        if success:
            print("\n🎉 All fixing operations completed successfully!")
            sys.exit(0)
        else:
            print("\n❌ Some fixing operations failed!")
            sys.exit(1)

    # Original SVD/PAC generation logic
    # Check if required commands are installed
    check_command_exists("svd2pac")

    # Create directories if they don't exist
    os.makedirs("pac", exist_ok=True)
    os.makedirs("patched_svd", exist_ok=True)

    # Set environment variables
    os.environ["RUST_BACKTRACE"] = RUST_BACKTRACE
    os.environ["RUST_LOG"] = RUST_LOG
    # Disable Windows Schannel certificate revocation checks (CRYPT_E_NO_REVOCATION_CHECK)
    # which fail on networks where CRL/OCSP endpoints are blocked (e.g. corporate proxies).
    os.environ.setdefault("CARGO_HTTP_CHECK_REVOKE", "false")

    # Determine processing mode
    svd_only = args.svd
    pac_only = args.pac

    # If both are specified, do full process
    if svd_only and pac_only:
        svd_only = False
        pac_only = False
    # If --all is specified, do full process
    elif args.all:
        svd_only = False
        pac_only = False
    # If no mode is specified, default to full process
    elif not svd_only and not pac_only:
        svd_only = False
        pac_only = False

    # Process all devices if ALL is specified
    if args.device_name.upper() == "ALL":
        print("Processing all SVD files...")
        if svd_only:
            print("Mode: Generate patched SVD files only")
        elif pac_only:
            print("Mode: Generate PACs from patched SVD files only")
        else:
            print("Mode: Full process (generate patched SVD files and PACs)")

        # For PAC-only mode with ALL, we need to get the list of available patched SVD files
        if pac_only:
            svd_files = glob.glob("patched_svd/*.svd")
            if not svd_files:
                print("No patched SVD files found in patched_svd directory.")
                sys.exit(1)
        else:
            svd_files = glob.glob("svd/*.svd")
            if not svd_files:
                print("No SVD files found in svd directory.")
                sys.exit(1)

        successes = 0
        failures = 0

        for svd_file in svd_files:
            # Extract device name from filename
            device_name = os.path.basename(svd_file).split('.')[0]
            if process_device(device_name, patch_only=svd_only, pac_only=pac_only):
                successes += 1
            else:
                failures += 1

        print(f"\nProcessed all SVD files. Successes: {successes}, Failures: {failures}")

        if failures > 0:
            print(f"Warning: {failures} device(s) failed to process properly.")
    else:
        # Process a single device
        device_name = args.device_name

        # Determine which steps to perform
        if svd_only:
            print(f"Mode: Generate patched SVD file only for {device_name}")
            success = process_device(device_name, patch_only=True)
        elif pac_only:
            print(f"Mode: Generate PAC only for {device_name}")
            # We need the patched SVD file first, but we won't regenerate it if it exists
            patched_svd = os.path.join("patched_svd", f"{device_name}.svd")
            if not os.path.exists(patched_svd):
                print(f"Patched SVD file {patched_svd} not found. Generating it first...")
                process_device(device_name, patch_only=True)
            success = process_device(device_name, patch_only=False, pac_only=True)
        else:
            print(f"Mode: Full process for {device_name}")
            success = process_device(device_name, patch_only=False)

        if not success:
            print(f"Failed to process device {device_name}")
            sys.exit(1)

    print("Done.")

if __name__ == "__main__":
    main()
