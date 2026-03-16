#!/usr/bin/env python3

import argparse
import pathlib
import re
import sys


ENUM_BLOCK_RE = re.compile(r"<enumeratedValue>.*?</enumeratedValue>", re.DOTALL)
EMPTY_ENUM_VALUES_RE = re.compile(
    r"\s*<enumeratedValues>\s*</enumeratedValues>", re.DOTALL
)


def strip_default_only_enum_blocks(text: str) -> tuple[str, int]:
    removed = 0

    def repl(match: re.Match[str]) -> str:
        nonlocal removed
        block = match.group(0)
        if "<isDefault>true</isDefault>" in block and "<value>" not in block:
            removed += 1
            return ""
        return block

    return ENUM_BLOCK_RE.sub(repl, text), removed


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Preprocess a Renesas SVD into a form stock svd2pac can ingest."
    )
    parser.add_argument("input", type=pathlib.Path)
    parser.add_argument("output", type=pathlib.Path)
    args = parser.parse_args()

    text = args.input.read_text(encoding="utf-8")
    processed, removed = strip_default_only_enum_blocks(text)
    processed, empty_removed = EMPTY_ENUM_VALUES_RE.subn("", processed)

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(processed, encoding="utf-8")
    print(f"removed_default_only_enum_blocks={removed}", file=sys.stderr)
    print(f"removed_empty_enumerated_values={empty_removed}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
