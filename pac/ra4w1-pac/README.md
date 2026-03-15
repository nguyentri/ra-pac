
# ra4w1-pac

This is the Peripheral Access Crate (PAC) for the RA4W1 device series.

The crate is generated from the device SVD file in the [packs](https://www.keil.arm.com/packs) using [sdv2pac](https://github.com/Infineon/svd2pac).

## Overview

The `ra4w1-pac` crate provides low-level access to device registers, enabling developers to interact with the hardware safely and efficiently.

## Usage

Include this crate in your `Cargo.toml`:

```toml
[dependencies]
 ra4w1-pac = "0.3.0"
```

## License

This crate is licensed under either the MIT License or the Apache License, Version 2.0.
