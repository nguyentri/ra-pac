# Changelog

## 0.3.1

All crates now build with every feature enabled (`--features all,rt`).

### Breaking changes

- `ra6m5-pac`: CANFD data-field register accessors were renamed to the scheme used by
  the other RA PACs, because the old names collided and the `canfd` feature did not
  compile. No working code could depend on the old names.
  - Register arrays: `cfdrfdf1()` → `cfdrfdf_1()`, `cfdcfdf1_0()` → `cfdcfdf_1_0()`,
    `cfdrmdf0__0()` → `cfdrmdf_0_0()`, `cfdtmdf0__0()` → `cfdtmdf_0_0()`
  - Single elements: `<array><index>()`, e.g. FIFO 3 of data field 1 is `cfdrfdf3_1()`
- `ra8p1-pac`: MFWD `fwcttc*` accessors renamed for the same reason, e.g. `fwcttc0()` → `fwcttc_0()`,
  `fwcttc20()` → `fwcttc_20()` / `fwcttc2_0()`.
- `ra4l1-pac`: `SCI5` now has its own `Sci5` type and `sci5` module (it was a second, conflicting
  `Sci0`); the IrDA register `ircr()` moved from `sci0` to `sci5`.

### Fixes

- `ra6t2`, `ra6t3`, `ra8d1`, `ra8e1`, `ra8e2`: the `rt` feature failed to compile on edition 2024
  (`unsafe extern` / `unsafe(no_mangle)` / `unsafe(link_section)`, see #1).
- `ra8p1-pac`: duplicated `_10` enum variants in DRW `WRITEALPHA` and USBHS `PID` fixed to `_11 = 3`.
- `ra6t1-pac`: published manifest metadata (version, description, license) fixed.
- Crate descriptions now name the right part number (e.g. `R7FA0E1`, not `R7FAA0E1`).
- Removed stale, unused source files from the packaged crates.
- CI builds every crate with all features before publishing, uses the correct target for
  RA4M2/RA4M3/RA6T2/RA6T3 (Cortex-M33), and fails when a crate does not build.
