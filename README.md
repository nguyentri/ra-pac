
# RA-PAC

This repository contains the Peripheral Access Crate (PAC) for Renesas RA Microcontrollers.

The crate is generated from the SVD files in [packs](https://www.keil.arm.com/packs) using [svd2pac](https://github.com/Infineon/svd2pac).

It serves as the foundation for the Hardware Abstraction Layer (HAL) in Rust for Renesas RA Series Microcontrollers.

## Supported Devices

- RA0E1
- RA2A1
- RA2A2
- RA2E1
- RA2E2
- RA2E3
- RA2L1
- RA4E1
- RA4E2
- RA4L1
- RA4M1
- RA4M2
- RA4M3
- RA4T1
- RA4W1
- RA6E1
- RA6E2
- RA6M1
- RA6M2
- RA6M3
- RA6M4
- RA6M5
- RA6T1
- RA6T2
- RA6T3
- RA8D1
- RA8E1
- RA8E2
- RA8M1
- RA8P1

## Target Architectures

Each device corresponds to a specific architecture target:

- **RA0, RA2 Series**: `thumbv8m.base-none-eabi` (Cortex-M23)

- **RA4, RA6 Series**:
  - Cortex-M4 devices: `thumbv7em-none-eabihf`
    - `ra4m1` (Cortex-M4)
    - `ra4m2` (Cortex-M4)
    - `ra4m3` (Cortex-M4)
    - `ra4w1` (Cortex-M4)
    - `ra6m1` (Cortex-M4)
    - `ra6m2` (Cortex-M4)
    - `ra6m3` (Cortex-M4)
    - `ra6t1` (Cortex-M4)
  - Cortex-M33 devices: `thumbv8m.main-none-eabihf`
    - The other RA4 and RA6 devices

- **RA8 Series**: `thumbv8m.main-none-eabihf` (Cortex-M85)

## License

This crate is licensed under either the MIT License or the Apache License, Version 2.0.
