/*
DISCLAIMER
This software is supplied by Renesas Electronics Corporation and is only intended for use with Renesas products.
No other uses are authorized. This software is owned by Renesas Electronics Corporation and is protected under all
applicable laws, including copyright laws.
THIS SOFTWARE IS PROVIDED "AS IS" AND RENESAS MAKES NO WARRANTIES REGARDING THIS SOFTWARE, WHETHER EXPRESS, IMPLIED
OR STATUTORY, INCLUDING BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NON-INFRINGEMENT.  ALL SUCH WARRANTIES ARE EXPRESSLY DISCLAIMED.TO THE MAXIMUM EXTENT PERMITTED NOT PROHIBITED BY
LAW, NEITHER RENESAS ELECTRONICS CORPORATION NOR ANY OF ITS AFFILIATED COMPANIES SHALL BE LIABLE FOR ANY DIRECT,
INDIRECT, SPECIAL, INCIDENTAL OR CONSEQUENTIAL DAMAGES FOR ANY REASON RELATED TO THIS SOFTWARE, EVEN IF RENESAS OR
ITS AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.
Renesas reserves the right, without notice, to make changes to this software and to discontinue the availability
of this software. By using this software, you agree to the additional terms and conditions found by accessing the
following link:
http://www.renesas.com/disclaimer

*/
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:11:03 +0000
#![cfg_attr(not(feature = "tracing"), no_std)]
#![allow(non_camel_case_types)]
#![doc = "Arm 32-bit Cortex-M4F Microcontroller based device, CPU clock up to 120MHz, etc."]
pub mod common;
pub use common::*;

#[cfg(feature = "tracing")]
pub mod reg_name;
#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "acmphs0")]
pub mod acmphs0;
#[cfg(feature = "acmphs1")]
pub mod acmphs1;
#[cfg(feature = "adc120")]
pub mod adc120;
#[cfg(feature = "adc121")]
pub mod adc121;
#[cfg(feature = "agt0")]
pub mod agt0;
#[cfg(feature = "bus")]
pub mod bus;
#[cfg(feature = "cac")]
pub mod cac;
#[cfg(feature = "can0")]
pub mod can0;
#[cfg(feature = "crc")]
pub mod crc;
#[cfg(feature = "ctsu")]
pub mod ctsu;
#[cfg(feature = "dac12")]
pub mod dac12;
#[cfg(feature = "dbg")]
pub mod dbg;
#[cfg(feature = "dma")]
pub mod dma;
#[cfg(feature = "dmac0")]
pub mod dmac0;
#[cfg(feature = "doc")]
pub mod doc;
#[cfg(feature = "dtc")]
pub mod dtc;
#[cfg(feature = "elc")]
pub mod elc;
#[cfg(feature = "fcache")]
pub mod fcache;
#[cfg(feature = "gpt328")]
pub mod gpt328;
#[cfg(feature = "gpt32eh0")]
pub mod gpt32eh0;
#[cfg(feature = "gpt_odc")]
pub mod gpt_odc;
#[cfg(feature = "gpt_ops")]
pub mod gpt_ops;
#[cfg(feature = "icu")]
pub mod icu;
#[cfg(feature = "iic0")]
pub mod iic0;
#[cfg(feature = "iic1")]
pub mod iic1;
#[cfg(feature = "irda")]
pub mod irda;
#[cfg(feature = "iwdt")]
pub mod iwdt;
#[cfg(feature = "kint")]
pub mod kint;
#[cfg(feature = "mmf")]
pub mod mmf;
#[cfg(feature = "mmpu")]
pub mod mmpu;
#[cfg(feature = "mstp")]
pub mod mstp;
#[cfg(feature = "pfs")]
pub mod pfs;
#[cfg(feature = "pmisc")]
pub mod pmisc;
#[cfg(feature = "poeg")]
pub mod poeg;
#[cfg(feature = "port0")]
pub mod port0;
#[cfg(feature = "port1")]
pub mod port1;
#[cfg(feature = "qspi")]
pub mod qspi;
#[cfg(feature = "rtc")]
pub mod rtc;
#[cfg(feature = "sci0")]
pub mod sci0;
#[cfg(feature = "sdhi0")]
pub mod sdhi0;
#[cfg(feature = "smpu")]
pub mod smpu;
#[cfg(feature = "spi0")]
pub mod spi0;
#[cfg(feature = "spmon")]
pub mod spmon;
#[cfg(feature = "sram")]
pub mod sram;
#[cfg(feature = "src")]
pub mod src;
#[cfg(feature = "srcram")]
pub mod srcram;
#[cfg(feature = "ssie0")]
pub mod ssie0;
#[cfg(feature = "system")]
pub mod system;
#[cfg(feature = "tsd")]
pub mod tsd;
#[cfg(feature = "tsn")]
pub mod tsn;
#[cfg(feature = "usbfs")]
pub mod usbfs;
#[cfg(feature = "wdt")]
pub mod wdt;

#[cfg(feature = "dac12")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac12 {
    ptr: *mut u8,
}
#[cfg(feature = "dac12")]
pub const DAC12: self::Dac12 = self::Dac12 {
    ptr: 0x4005e000u32 as _,
};
#[cfg(feature = "acmphs0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acmphs0 {
    ptr: *mut u8,
}
#[cfg(feature = "acmphs0")]
pub const ACMPHS0: self::Acmphs0 = self::Acmphs0 {
    ptr: 0x40085000u32 as _,
};
#[cfg(feature = "acmphs1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acmphs1 {
    ptr: *mut u8,
}
#[cfg(feature = "acmphs1")]
pub const ACMPHS1: self::Acmphs1 = self::Acmphs1 {
    ptr: 0x40085100u32 as _,
};
#[cfg(feature = "acmphs2")]
pub const ACMPHS2: self::Acmphs1 = self::Acmphs1 {
    ptr: 0x40085200u32 as _,
};
#[cfg(feature = "acmphs3")]
pub const ACMPHS3: self::Acmphs1 = self::Acmphs1 {
    ptr: 0x40085300u32 as _,
};
#[cfg(feature = "acmphs4")]
pub const ACMPHS4: self::Acmphs1 = self::Acmphs1 {
    ptr: 0x40085400u32 as _,
};
#[cfg(feature = "acmphs5")]
pub const ACMPHS5: self::Acmphs1 = self::Acmphs1 {
    ptr: 0x40085500u32 as _,
};
#[cfg(feature = "tsn")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsn {
    ptr: *mut u8,
}
#[cfg(feature = "tsn")]
pub const TSN: self::Tsn = self::Tsn {
    ptr: 0x4005d000u32 as _,
};
#[cfg(feature = "crc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crc {
    ptr: *mut u8,
}
#[cfg(feature = "crc")]
pub const CRC: self::Crc = self::Crc {
    ptr: 0x40074000u32 as _,
};
#[cfg(feature = "ssie0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssie0 {
    ptr: *mut u8,
}
#[cfg(feature = "ssie0")]
pub const SSIE0: self::Ssie0 = self::Ssie0 {
    ptr: 0x4004e000u32 as _,
};
#[cfg(feature = "elc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elc {
    ptr: *mut u8,
}
#[cfg(feature = "elc")]
pub const ELC: self::Elc = self::Elc {
    ptr: 0x40041000u32 as _,
};
#[cfg(feature = "cac")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cac {
    ptr: *mut u8,
}
#[cfg(feature = "cac")]
pub const CAC: self::Cac = self::Cac {
    ptr: 0x40044600u32 as _,
};
#[cfg(feature = "doc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doc {
    ptr: *mut u8,
}
#[cfg(feature = "doc")]
pub const DOC: self::Doc = self::Doc {
    ptr: 0x40054100u32 as _,
};
#[cfg(feature = "iwdt")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdt {
    ptr: *mut u8,
}
#[cfg(feature = "iwdt")]
pub const IWDT: self::Iwdt = self::Iwdt {
    ptr: 0x40044400u32 as _,
};
#[cfg(feature = "kint")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kint {
    ptr: *mut u8,
}
#[cfg(feature = "kint")]
pub const KINT: self::Kint = self::Kint {
    ptr: 0x40080000u32 as _,
};
#[cfg(feature = "wdt")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt {
    ptr: *mut u8,
}
#[cfg(feature = "wdt")]
pub const WDT: self::Wdt = self::Wdt {
    ptr: 0x40044200u32 as _,
};
#[cfg(feature = "usbfs")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbfs {
    ptr: *mut u8,
}
#[cfg(feature = "usbfs")]
pub const USBFS: self::Usbfs = self::Usbfs {
    ptr: 0x40090000u32 as _,
};
#[cfg(feature = "can0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can0 {
    ptr: *mut u8,
}
#[cfg(feature = "can0")]
pub const CAN0: self::Can0 = self::Can0 {
    ptr: 0x40050000u32 as _,
};
#[cfg(feature = "can1")]
pub const CAN1: self::Can0 = self::Can0 {
    ptr: 0x40051000u32 as _,
};
#[cfg(feature = "irda")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irda {
    ptr: *mut u8,
}
#[cfg(feature = "irda")]
pub const IRDA: self::Irda = self::Irda {
    ptr: 0x40070f00u32 as _,
};
#[cfg(feature = "qspi")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspi {
    ptr: *mut u8,
}
#[cfg(feature = "qspi")]
pub const QSPI: self::Qspi = self::Qspi {
    ptr: 0x64000000u32 as _,
};
#[cfg(feature = "spi0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi0 {
    ptr: *mut u8,
}
#[cfg(feature = "spi0")]
pub const SPI0: self::Spi0 = self::Spi0 {
    ptr: 0x40072000u32 as _,
};
#[cfg(feature = "spi1")]
pub const SPI1: self::Spi0 = self::Spi0 {
    ptr: 0x40072100u32 as _,
};
#[cfg(feature = "srcram")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srcram {
    ptr: *mut u8,
}
#[cfg(feature = "srcram")]
pub const SRCRAM: self::Srcram = self::Srcram {
    ptr: 0x40048000u32 as _,
};
#[cfg(feature = "src")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src {
    ptr: *mut u8,
}
#[cfg(feature = "src")]
pub const SRC: self::Src = self::Src {
    ptr: 0x4004dff0u32 as _,
};
#[cfg(feature = "dbg")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbg {
    ptr: *mut u8,
}
#[cfg(feature = "dbg")]
pub const DBG: self::Dbg = self::Dbg {
    ptr: 0x4001b000u32 as _,
};
#[cfg(feature = "dmac0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac0 {
    ptr: *mut u8,
}
#[cfg(feature = "dmac0")]
pub const DMAC0: self::Dmac0 = self::Dmac0 {
    ptr: 0x40005000u32 as _,
};
#[cfg(feature = "dmac1")]
pub const DMAC1: self::Dmac0 = self::Dmac0 {
    ptr: 0x40005040u32 as _,
};
#[cfg(feature = "dmac2")]
pub const DMAC2: self::Dmac0 = self::Dmac0 {
    ptr: 0x40005080u32 as _,
};
#[cfg(feature = "dmac3")]
pub const DMAC3: self::Dmac0 = self::Dmac0 {
    ptr: 0x400050c0u32 as _,
};
#[cfg(feature = "dmac4")]
pub const DMAC4: self::Dmac0 = self::Dmac0 {
    ptr: 0x40005100u32 as _,
};
#[cfg(feature = "dmac5")]
pub const DMAC5: self::Dmac0 = self::Dmac0 {
    ptr: 0x40005140u32 as _,
};
#[cfg(feature = "dmac6")]
pub const DMAC6: self::Dmac0 = self::Dmac0 {
    ptr: 0x40005180u32 as _,
};
#[cfg(feature = "dmac7")]
pub const DMAC7: self::Dmac0 = self::Dmac0 {
    ptr: 0x400051c0u32 as _,
};
#[cfg(feature = "dma")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
#[cfg(feature = "dma")]
pub const DMA: self::Dma = self::Dma {
    ptr: 0x40005200u32 as _,
};
#[cfg(feature = "dtc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtc {
    ptr: *mut u8,
}
#[cfg(feature = "dtc")]
pub const DTC: self::Dtc = self::Dtc {
    ptr: 0x40005400u32 as _,
};
#[cfg(feature = "mmf")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmf {
    ptr: *mut u8,
}
#[cfg(feature = "mmf")]
pub const MMF: self::Mmf = self::Mmf {
    ptr: 0x40001000u32 as _,
};
#[cfg(feature = "sram")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sram {
    ptr: *mut u8,
}
#[cfg(feature = "sram")]
pub const SRAM: self::Sram = self::Sram {
    ptr: 0x40002000u32 as _,
};
#[cfg(feature = "fcache")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcache {
    ptr: *mut u8,
}
#[cfg(feature = "fcache")]
pub const FCACHE: self::Fcache = self::Fcache {
    ptr: 0x4001c000u32 as _,
};
#[cfg(feature = "system")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct System {
    ptr: *mut u8,
}
#[cfg(feature = "system")]
pub const SYSTEM: self::System = self::System {
    ptr: 0x4001e000u32 as _,
};
#[cfg(feature = "mstp")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstp {
    ptr: *mut u8,
}
#[cfg(feature = "mstp")]
pub const MSTP: self::Mstp = self::Mstp {
    ptr: 0x40047000u32 as _,
};
#[cfg(feature = "rtc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
#[cfg(feature = "rtc")]
pub const RTC: self::Rtc = self::Rtc {
    ptr: 0x40044000u32 as _,
};
#[cfg(feature = "agt0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agt0 {
    ptr: *mut u8,
}
#[cfg(feature = "agt0")]
pub const AGT0: self::Agt0 = self::Agt0 {
    ptr: 0x40084000u32 as _,
};
#[cfg(feature = "agt1")]
pub const AGT1: self::Agt0 = self::Agt0 {
    ptr: 0x40084100u32 as _,
};
#[cfg(feature = "gpt_ops")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GptOps {
    ptr: *mut u8,
}
#[cfg(feature = "gpt_ops")]
pub const GPT_OPS: self::GptOps = self::GptOps {
    ptr: 0x40078ff0u32 as _,
};
#[cfg(feature = "gpt32eh0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt32Eh0 {
    ptr: *mut u8,
}
#[cfg(feature = "gpt32eh0")]
pub const GPT32EH0: self::Gpt32Eh0 = self::Gpt32Eh0 {
    ptr: 0x40078000u32 as _,
};
#[cfg(feature = "gpt32eh1")]
pub const GPT32EH1: self::Gpt32Eh0 = self::Gpt32Eh0 {
    ptr: 0x40078100u32 as _,
};
#[cfg(feature = "gpt32eh2")]
pub const GPT32EH2: self::Gpt32Eh0 = self::Gpt32Eh0 {
    ptr: 0x40078200u32 as _,
};
#[cfg(feature = "gpt32eh3")]
pub const GPT32EH3: self::Gpt32Eh0 = self::Gpt32Eh0 {
    ptr: 0x40078300u32 as _,
};
#[cfg(feature = "gpt32e4")]
pub const GPT32E4: self::Gpt32Eh0 = self::Gpt32Eh0 {
    ptr: 0x40078400u32 as _,
};
#[cfg(feature = "gpt32e5")]
pub const GPT32E5: self::Gpt32Eh0 = self::Gpt32Eh0 {
    ptr: 0x40078500u32 as _,
};
#[cfg(feature = "gpt32e6")]
pub const GPT32E6: self::Gpt32Eh0 = self::Gpt32Eh0 {
    ptr: 0x40078600u32 as _,
};
#[cfg(feature = "gpt32e7")]
pub const GPT32E7: self::Gpt32Eh0 = self::Gpt32Eh0 {
    ptr: 0x40078700u32 as _,
};
#[cfg(feature = "poeg")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Poeg {
    ptr: *mut u8,
}
#[cfg(feature = "poeg")]
pub const POEG: self::Poeg = self::Poeg {
    ptr: 0x40042000u32 as _,
};
#[cfg(feature = "gpt_odc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GptOdc {
    ptr: *mut u8,
}
#[cfg(feature = "gpt_odc")]
pub const GPT_ODC: self::GptOdc = self::GptOdc {
    ptr: 0x4007b000u32 as _,
};
#[cfg(feature = "bus")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bus {
    ptr: *mut u8,
}
#[cfg(feature = "bus")]
pub const BUS: self::Bus = self::Bus {
    ptr: 0x40003000u32 as _,
};
#[cfg(feature = "icu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icu {
    ptr: *mut u8,
}
#[cfg(feature = "icu")]
pub const ICU: self::Icu = self::Icu {
    ptr: 0x40006000u32 as _,
};
#[cfg(feature = "port0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port0 {
    ptr: *mut u8,
}
#[cfg(feature = "port0")]
pub const PORT0: self::Port0 = self::Port0 {
    ptr: 0x40040000u32 as _,
};
#[cfg(feature = "port1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port1 {
    ptr: *mut u8,
}
#[cfg(feature = "port1")]
pub const PORT1: self::Port1 = self::Port1 {
    ptr: 0x40040020u32 as _,
};
#[cfg(feature = "port2")]
pub const PORT2: self::Port1 = self::Port1 {
    ptr: 0x40040040u32 as _,
};
#[cfg(feature = "port3")]
pub const PORT3: self::Port1 = self::Port1 {
    ptr: 0x40040060u32 as _,
};
#[cfg(feature = "port4")]
pub const PORT4: self::Port1 = self::Port1 {
    ptr: 0x40040080u32 as _,
};
#[cfg(feature = "port5")]
pub const PORT5: self::Port0 = self::Port0 {
    ptr: 0x400400a0u32 as _,
};
#[cfg(feature = "port6")]
pub const PORT6: self::Port0 = self::Port0 {
    ptr: 0x400400c0u32 as _,
};
#[cfg(feature = "port7")]
pub const PORT7: self::Port0 = self::Port0 {
    ptr: 0x400400e0u32 as _,
};
#[cfg(feature = "pfs")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfs {
    ptr: *mut u8,
}
#[cfg(feature = "pfs")]
pub const PFS: self::Pfs = self::Pfs {
    ptr: 0x40040800u32 as _,
};
#[cfg(feature = "pmisc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmisc {
    ptr: *mut u8,
}
#[cfg(feature = "pmisc")]
pub const PMISC: self::Pmisc = self::Pmisc {
    ptr: 0x40040d00u32 as _,
};
#[cfg(feature = "iic0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iic0 {
    ptr: *mut u8,
}
#[cfg(feature = "iic0")]
pub const IIC0: self::Iic0 = self::Iic0 {
    ptr: 0x40053000u32 as _,
};
#[cfg(feature = "iic1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iic1 {
    ptr: *mut u8,
}
#[cfg(feature = "iic1")]
pub const IIC1: self::Iic1 = self::Iic1 {
    ptr: 0x40053100u32 as _,
};
#[cfg(feature = "sci0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sci0 {
    ptr: *mut u8,
}
#[cfg(feature = "sci0")]
pub const SCI0: self::Sci0 = self::Sci0 {
    ptr: 0x40070000u32 as _,
};
#[cfg(feature = "sci1")]
pub const SCI1: self::Sci0 = self::Sci0 {
    ptr: 0x40070020u32 as _,
};
#[cfg(feature = "sci2")]
pub const SCI2: self::Sci0 = self::Sci0 {
    ptr: 0x40070040u32 as _,
};
#[cfg(feature = "sci3")]
pub const SCI3: self::Sci0 = self::Sci0 {
    ptr: 0x40070060u32 as _,
};
#[cfg(feature = "sci4")]
pub const SCI4: self::Sci0 = self::Sci0 {
    ptr: 0x40070080u32 as _,
};
#[cfg(feature = "sci8")]
pub const SCI8: self::Sci0 = self::Sci0 {
    ptr: 0x40070100u32 as _,
};
#[cfg(feature = "sci9")]
pub const SCI9: self::Sci0 = self::Sci0 {
    ptr: 0x40070120u32 as _,
};
#[cfg(feature = "sdhi0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdhi0 {
    ptr: *mut u8,
}
#[cfg(feature = "sdhi0")]
pub const SDHI0: self::Sdhi0 = self::Sdhi0 {
    ptr: 0x40062000u32 as _,
};
#[cfg(feature = "sdhi1")]
pub const SDHI1: self::Sdhi0 = self::Sdhi0 {
    ptr: 0x40062400u32 as _,
};
#[cfg(feature = "gpt328")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt328 {
    ptr: *mut u8,
}
#[cfg(feature = "gpt328")]
pub const GPT328: self::Gpt328 = self::Gpt328 {
    ptr: 0x40078800u32 as _,
};
#[cfg(feature = "gpt329")]
pub const GPT329: self::Gpt328 = self::Gpt328 {
    ptr: 0x40078900u32 as _,
};
#[cfg(feature = "gpt3210")]
pub const GPT3210: self::Gpt328 = self::Gpt328 {
    ptr: 0x40078a00u32 as _,
};
#[cfg(feature = "gpt3211")]
pub const GPT3211: self::Gpt328 = self::Gpt328 {
    ptr: 0x40078b00u32 as _,
};
#[cfg(feature = "gpt3212")]
pub const GPT3212: self::Gpt328 = self::Gpt328 {
    ptr: 0x40078c00u32 as _,
};
#[cfg(feature = "adc120")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc120 {
    ptr: *mut u8,
}
#[cfg(feature = "adc120")]
pub const ADC120: self::Adc120 = self::Adc120 {
    ptr: 0x4005c000u32 as _,
};
#[cfg(feature = "adc121")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc121 {
    ptr: *mut u8,
}
#[cfg(feature = "adc121")]
pub const ADC121: self::Adc121 = self::Adc121 {
    ptr: 0x4005c200u32 as _,
};
#[cfg(feature = "ctsu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsu {
    ptr: *mut u8,
}
#[cfg(feature = "ctsu")]
pub const CTSU: self::Ctsu = self::Ctsu {
    ptr: 0x40081000u32 as _,
};
#[cfg(feature = "mmpu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpu {
    ptr: *mut u8,
}
#[cfg(feature = "mmpu")]
pub const MMPU: self::Mmpu = self::Mmpu {
    ptr: 0x40000000u32 as _,
};
#[cfg(feature = "smpu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpu {
    ptr: *mut u8,
}
#[cfg(feature = "smpu")]
pub const SMPU: self::Smpu = self::Smpu {
    ptr: 0x40000c00u32 as _,
};
#[cfg(feature = "spmon")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spmon {
    ptr: *mut u8,
}
#[cfg(feature = "spmon")]
pub const SPMON: self::Spmon = self::Spmon {
    ptr: 0x40000d00u32 as _,
};
#[cfg(feature = "tsd")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsd {
    ptr: *mut u8,
}
#[cfg(feature = "tsd")]
pub const TSD: self::Tsd = self::Tsd {
    ptr: 0x407fb17cu32 as _,
};

pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub mod interrupt_handlers {
    unsafe extern "C" {
        pub fn IEL0();
        pub fn IEL1();
        pub fn IEL2();
        pub fn IEL3();
        pub fn IEL4();
        pub fn IEL5();
        pub fn IEL6();
        pub fn IEL7();
        pub fn IEL8();
        pub fn IEL9();
        pub fn IEL10();
        pub fn IEL11();
        pub fn IEL12();
        pub fn IEL13();
        pub fn IEL14();
        pub fn IEL15();
        pub fn IEL16();
        pub fn IEL17();
        pub fn IEL18();
        pub fn IEL19();
        pub fn IEL20();
        pub fn IEL21();
        pub fn IEL22();
        pub fn IEL23();
        pub fn IEL24();
        pub fn IEL25();
        pub fn IEL26();
        pub fn IEL27();
        pub fn IEL28();
        pub fn IEL29();
        pub fn IEL30();
        pub fn IEL31();
        pub fn IEL32();
        pub fn IEL33();
        pub fn IEL34();
        pub fn IEL35();
        pub fn IEL36();
        pub fn IEL37();
        pub fn IEL38();
        pub fn IEL39();
        pub fn IEL40();
        pub fn IEL41();
        pub fn IEL42();
        pub fn IEL43();
        pub fn IEL44();
        pub fn IEL45();
        pub fn IEL46();
        pub fn IEL47();
        pub fn IEL48();
        pub fn IEL49();
        pub fn IEL50();
        pub fn IEL51();
        pub fn IEL52();
        pub fn IEL53();
        pub fn IEL54();
        pub fn IEL55();
        pub fn IEL56();
        pub fn IEL57();
        pub fn IEL58();
        pub fn IEL59();
        pub fn IEL60();
        pub fn IEL61();
        pub fn IEL62();
        pub fn IEL63();
        pub fn IEL64();
        pub fn IEL65();
        pub fn IEL66();
        pub fn IEL67();
        pub fn IEL68();
        pub fn IEL69();
        pub fn IEL70();
        pub fn IEL71();
        pub fn IEL72();
        pub fn IEL73();
        pub fn IEL74();
        pub fn IEL75();
        pub fn IEL76();
        pub fn IEL77();
        pub fn IEL78();
        pub fn IEL79();
        pub fn IEL80();
        pub fn IEL81();
        pub fn IEL82();
        pub fn IEL83();
        pub fn IEL84();
        pub fn IEL85();
        pub fn IEL86();
        pub fn IEL87();
        pub fn IEL88();
        pub fn IEL89();
        pub fn IEL90();
        pub fn IEL91();
        pub fn IEL92();
        pub fn IEL93();
        pub fn IEL94();
        pub fn IEL95();
    }
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 96] = [
    Vector {
        _handler: interrupt_handlers::IEL0,
    },
    Vector {
        _handler: interrupt_handlers::IEL1,
    },
    Vector {
        _handler: interrupt_handlers::IEL2,
    },
    Vector {
        _handler: interrupt_handlers::IEL3,
    },
    Vector {
        _handler: interrupt_handlers::IEL4,
    },
    Vector {
        _handler: interrupt_handlers::IEL5,
    },
    Vector {
        _handler: interrupt_handlers::IEL6,
    },
    Vector {
        _handler: interrupt_handlers::IEL7,
    },
    Vector {
        _handler: interrupt_handlers::IEL8,
    },
    Vector {
        _handler: interrupt_handlers::IEL9,
    },
    Vector {
        _handler: interrupt_handlers::IEL10,
    },
    Vector {
        _handler: interrupt_handlers::IEL11,
    },
    Vector {
        _handler: interrupt_handlers::IEL12,
    },
    Vector {
        _handler: interrupt_handlers::IEL13,
    },
    Vector {
        _handler: interrupt_handlers::IEL14,
    },
    Vector {
        _handler: interrupt_handlers::IEL15,
    },
    Vector {
        _handler: interrupt_handlers::IEL16,
    },
    Vector {
        _handler: interrupt_handlers::IEL17,
    },
    Vector {
        _handler: interrupt_handlers::IEL18,
    },
    Vector {
        _handler: interrupt_handlers::IEL19,
    },
    Vector {
        _handler: interrupt_handlers::IEL20,
    },
    Vector {
        _handler: interrupt_handlers::IEL21,
    },
    Vector {
        _handler: interrupt_handlers::IEL22,
    },
    Vector {
        _handler: interrupt_handlers::IEL23,
    },
    Vector {
        _handler: interrupt_handlers::IEL24,
    },
    Vector {
        _handler: interrupt_handlers::IEL25,
    },
    Vector {
        _handler: interrupt_handlers::IEL26,
    },
    Vector {
        _handler: interrupt_handlers::IEL27,
    },
    Vector {
        _handler: interrupt_handlers::IEL28,
    },
    Vector {
        _handler: interrupt_handlers::IEL29,
    },
    Vector {
        _handler: interrupt_handlers::IEL30,
    },
    Vector {
        _handler: interrupt_handlers::IEL31,
    },
    Vector {
        _handler: interrupt_handlers::IEL32,
    },
    Vector {
        _handler: interrupt_handlers::IEL33,
    },
    Vector {
        _handler: interrupt_handlers::IEL34,
    },
    Vector {
        _handler: interrupt_handlers::IEL35,
    },
    Vector {
        _handler: interrupt_handlers::IEL36,
    },
    Vector {
        _handler: interrupt_handlers::IEL37,
    },
    Vector {
        _handler: interrupt_handlers::IEL38,
    },
    Vector {
        _handler: interrupt_handlers::IEL39,
    },
    Vector {
        _handler: interrupt_handlers::IEL40,
    },
    Vector {
        _handler: interrupt_handlers::IEL41,
    },
    Vector {
        _handler: interrupt_handlers::IEL42,
    },
    Vector {
        _handler: interrupt_handlers::IEL43,
    },
    Vector {
        _handler: interrupt_handlers::IEL44,
    },
    Vector {
        _handler: interrupt_handlers::IEL45,
    },
    Vector {
        _handler: interrupt_handlers::IEL46,
    },
    Vector {
        _handler: interrupt_handlers::IEL47,
    },
    Vector {
        _handler: interrupt_handlers::IEL48,
    },
    Vector {
        _handler: interrupt_handlers::IEL49,
    },
    Vector {
        _handler: interrupt_handlers::IEL50,
    },
    Vector {
        _handler: interrupt_handlers::IEL51,
    },
    Vector {
        _handler: interrupt_handlers::IEL52,
    },
    Vector {
        _handler: interrupt_handlers::IEL53,
    },
    Vector {
        _handler: interrupt_handlers::IEL54,
    },
    Vector {
        _handler: interrupt_handlers::IEL55,
    },
    Vector {
        _handler: interrupt_handlers::IEL56,
    },
    Vector {
        _handler: interrupt_handlers::IEL57,
    },
    Vector {
        _handler: interrupt_handlers::IEL58,
    },
    Vector {
        _handler: interrupt_handlers::IEL59,
    },
    Vector {
        _handler: interrupt_handlers::IEL60,
    },
    Vector {
        _handler: interrupt_handlers::IEL61,
    },
    Vector {
        _handler: interrupt_handlers::IEL62,
    },
    Vector {
        _handler: interrupt_handlers::IEL63,
    },
    Vector {
        _handler: interrupt_handlers::IEL64,
    },
    Vector {
        _handler: interrupt_handlers::IEL65,
    },
    Vector {
        _handler: interrupt_handlers::IEL66,
    },
    Vector {
        _handler: interrupt_handlers::IEL67,
    },
    Vector {
        _handler: interrupt_handlers::IEL68,
    },
    Vector {
        _handler: interrupt_handlers::IEL69,
    },
    Vector {
        _handler: interrupt_handlers::IEL70,
    },
    Vector {
        _handler: interrupt_handlers::IEL71,
    },
    Vector {
        _handler: interrupt_handlers::IEL72,
    },
    Vector {
        _handler: interrupt_handlers::IEL73,
    },
    Vector {
        _handler: interrupt_handlers::IEL74,
    },
    Vector {
        _handler: interrupt_handlers::IEL75,
    },
    Vector {
        _handler: interrupt_handlers::IEL76,
    },
    Vector {
        _handler: interrupt_handlers::IEL77,
    },
    Vector {
        _handler: interrupt_handlers::IEL78,
    },
    Vector {
        _handler: interrupt_handlers::IEL79,
    },
    Vector {
        _handler: interrupt_handlers::IEL80,
    },
    Vector {
        _handler: interrupt_handlers::IEL81,
    },
    Vector {
        _handler: interrupt_handlers::IEL82,
    },
    Vector {
        _handler: interrupt_handlers::IEL83,
    },
    Vector {
        _handler: interrupt_handlers::IEL84,
    },
    Vector {
        _handler: interrupt_handlers::IEL85,
    },
    Vector {
        _handler: interrupt_handlers::IEL86,
    },
    Vector {
        _handler: interrupt_handlers::IEL87,
    },
    Vector {
        _handler: interrupt_handlers::IEL88,
    },
    Vector {
        _handler: interrupt_handlers::IEL89,
    },
    Vector {
        _handler: interrupt_handlers::IEL90,
    },
    Vector {
        _handler: interrupt_handlers::IEL91,
    },
    Vector {
        _handler: interrupt_handlers::IEL92,
    },
    Vector {
        _handler: interrupt_handlers::IEL93,
    },
    Vector {
        _handler: interrupt_handlers::IEL94,
    },
    Vector {
        _handler: interrupt_handlers::IEL95,
    },
];
#[doc = "Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "ICU Interrupt 0"]
    IEL0 = 0,

    #[doc = "ICU Interrupt 1"]
    IEL1 = 1,

    #[doc = "ICU Interrupt 2"]
    IEL2 = 2,

    #[doc = "ICU Interrupt 3"]
    IEL3 = 3,

    #[doc = "ICU Interrupt 4"]
    IEL4 = 4,

    #[doc = "ICU Interrupt 5"]
    IEL5 = 5,

    #[doc = "ICU Interrupt 6"]
    IEL6 = 6,

    #[doc = "ICU Interrupt 7"]
    IEL7 = 7,

    #[doc = "ICU Interrupt 8"]
    IEL8 = 8,

    #[doc = "ICU Interrupt 9"]
    IEL9 = 9,

    #[doc = "ICU Interrupt 10"]
    IEL10 = 10,

    #[doc = "ICU Interrupt 11"]
    IEL11 = 11,

    #[doc = "ICU Interrupt 12"]
    IEL12 = 12,

    #[doc = "ICU Interrupt 13"]
    IEL13 = 13,

    #[doc = "ICU Interrupt 14"]
    IEL14 = 14,

    #[doc = "ICU Interrupt 15"]
    IEL15 = 15,

    #[doc = "ICU Interrupt 16"]
    IEL16 = 16,

    #[doc = "ICU Interrupt 17"]
    IEL17 = 17,

    #[doc = "ICU Interrupt 18"]
    IEL18 = 18,

    #[doc = "ICU Interrupt 19"]
    IEL19 = 19,

    #[doc = "ICU Interrupt 20"]
    IEL20 = 20,

    #[doc = "ICU Interrupt 21"]
    IEL21 = 21,

    #[doc = "ICU Interrupt 22"]
    IEL22 = 22,

    #[doc = "ICU Interrupt 23"]
    IEL23 = 23,

    #[doc = "ICU Interrupt 24"]
    IEL24 = 24,

    #[doc = "ICU Interrupt 25"]
    IEL25 = 25,

    #[doc = "ICU Interrupt 26"]
    IEL26 = 26,

    #[doc = "ICU Interrupt 27"]
    IEL27 = 27,

    #[doc = "ICU Interrupt 28"]
    IEL28 = 28,

    #[doc = "ICU Interrupt 29"]
    IEL29 = 29,

    #[doc = "ICU Interrupt 30"]
    IEL30 = 30,

    #[doc = "ICU Interrupt 31"]
    IEL31 = 31,

    #[doc = "ICU Interrupt 32"]
    IEL32 = 32,

    #[doc = "ICU Interrupt 33"]
    IEL33 = 33,

    #[doc = "ICU Interrupt 34"]
    IEL34 = 34,

    #[doc = "ICU Interrupt 35"]
    IEL35 = 35,

    #[doc = "ICU Interrupt 36"]
    IEL36 = 36,

    #[doc = "ICU Interrupt 37"]
    IEL37 = 37,

    #[doc = "ICU Interrupt 38"]
    IEL38 = 38,

    #[doc = "ICU Interrupt 39"]
    IEL39 = 39,

    #[doc = "ICU Interrupt 40"]
    IEL40 = 40,

    #[doc = "ICU Interrupt 41"]
    IEL41 = 41,

    #[doc = "ICU Interrupt 42"]
    IEL42 = 42,

    #[doc = "ICU Interrupt 43"]
    IEL43 = 43,

    #[doc = "ICU Interrupt 44"]
    IEL44 = 44,

    #[doc = "ICU Interrupt 45"]
    IEL45 = 45,

    #[doc = "ICU Interrupt 46"]
    IEL46 = 46,

    #[doc = "ICU Interrupt 47"]
    IEL47 = 47,

    #[doc = "ICU Interrupt 48"]
    IEL48 = 48,

    #[doc = "ICU Interrupt 49"]
    IEL49 = 49,

    #[doc = "ICU Interrupt 50"]
    IEL50 = 50,

    #[doc = "ICU Interrupt 51"]
    IEL51 = 51,

    #[doc = "ICU Interrupt 52"]
    IEL52 = 52,

    #[doc = "ICU Interrupt 53"]
    IEL53 = 53,

    #[doc = "ICU Interrupt 54"]
    IEL54 = 54,

    #[doc = "ICU Interrupt 55"]
    IEL55 = 55,

    #[doc = "ICU Interrupt 56"]
    IEL56 = 56,

    #[doc = "ICU Interrupt 57"]
    IEL57 = 57,

    #[doc = "ICU Interrupt 58"]
    IEL58 = 58,

    #[doc = "ICU Interrupt 59"]
    IEL59 = 59,

    #[doc = "ICU Interrupt 60"]
    IEL60 = 60,

    #[doc = "ICU Interrupt 61"]
    IEL61 = 61,

    #[doc = "ICU Interrupt 62"]
    IEL62 = 62,

    #[doc = "ICU Interrupt 63"]
    IEL63 = 63,

    #[doc = "ICU Interrupt 64"]
    IEL64 = 64,

    #[doc = "ICU Interrupt 65"]
    IEL65 = 65,

    #[doc = "ICU Interrupt 66"]
    IEL66 = 66,

    #[doc = "ICU Interrupt 67"]
    IEL67 = 67,

    #[doc = "ICU Interrupt 68"]
    IEL68 = 68,

    #[doc = "ICU Interrupt 69"]
    IEL69 = 69,

    #[doc = "ICU Interrupt 70"]
    IEL70 = 70,

    #[doc = "ICU Interrupt 71"]
    IEL71 = 71,

    #[doc = "ICU Interrupt 72"]
    IEL72 = 72,

    #[doc = "ICU Interrupt 73"]
    IEL73 = 73,

    #[doc = "ICU Interrupt 74"]
    IEL74 = 74,

    #[doc = "ICU Interrupt 75"]
    IEL75 = 75,

    #[doc = "ICU Interrupt 76"]
    IEL76 = 76,

    #[doc = "ICU Interrupt 77"]
    IEL77 = 77,

    #[doc = "ICU Interrupt 78"]
    IEL78 = 78,

    #[doc = "ICU Interrupt 79"]
    IEL79 = 79,

    #[doc = "ICU Interrupt 80"]
    IEL80 = 80,

    #[doc = "ICU Interrupt 81"]
    IEL81 = 81,

    #[doc = "ICU Interrupt 82"]
    IEL82 = 82,

    #[doc = "ICU Interrupt 83"]
    IEL83 = 83,

    #[doc = "ICU Interrupt 84"]
    IEL84 = 84,

    #[doc = "ICU Interrupt 85"]
    IEL85 = 85,

    #[doc = "ICU Interrupt 86"]
    IEL86 = 86,

    #[doc = "ICU Interrupt 87"]
    IEL87 = 87,

    #[doc = "ICU Interrupt 88"]
    IEL88 = 88,

    #[doc = "ICU Interrupt 89"]
    IEL89 = 89,

    #[doc = "ICU Interrupt 90"]
    IEL90 = 90,

    #[doc = "ICU Interrupt 91"]
    IEL91 = 91,

    #[doc = "ICU Interrupt 92"]
    IEL92 = 92,

    #[doc = "ICU Interrupt 93"]
    IEL93 = 93,

    #[doc = "ICU Interrupt 94"]
    IEL94 = 94,

    #[doc = "ICU Interrupt 95"]
    IEL95 = 95,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[allow(non_snake_case)]
/// Required for compatibility with RTIC and other frameworks
pub struct Peripherals {
    #[cfg(feature = "dac12")]
    pub DAC12: self::Dac12,
    #[cfg(feature = "acmphs0")]
    pub ACMPHS0: self::Acmphs0,
    #[cfg(feature = "acmphs1")]
    pub ACMPHS1: self::Acmphs1,
    #[cfg(feature = "acmphs2")]
    pub ACMPHS2: self::Acmphs1,
    #[cfg(feature = "acmphs3")]
    pub ACMPHS3: self::Acmphs1,
    #[cfg(feature = "acmphs4")]
    pub ACMPHS4: self::Acmphs1,
    #[cfg(feature = "acmphs5")]
    pub ACMPHS5: self::Acmphs1,
    #[cfg(feature = "tsn")]
    pub TSN: self::Tsn,
    #[cfg(feature = "crc")]
    pub CRC: self::Crc,
    #[cfg(feature = "ssie0")]
    pub SSIE0: self::Ssie0,
    #[cfg(feature = "elc")]
    pub ELC: self::Elc,
    #[cfg(feature = "cac")]
    pub CAC: self::Cac,
    #[cfg(feature = "doc")]
    pub DOC: self::Doc,
    #[cfg(feature = "iwdt")]
    pub IWDT: self::Iwdt,
    #[cfg(feature = "kint")]
    pub KINT: self::Kint,
    #[cfg(feature = "wdt")]
    pub WDT: self::Wdt,
    #[cfg(feature = "usbfs")]
    pub USBFS: self::Usbfs,
    #[cfg(feature = "can0")]
    pub CAN0: self::Can0,
    #[cfg(feature = "can1")]
    pub CAN1: self::Can0,
    #[cfg(feature = "irda")]
    pub IRDA: self::Irda,
    #[cfg(feature = "qspi")]
    pub QSPI: self::Qspi,
    #[cfg(feature = "spi0")]
    pub SPI0: self::Spi0,
    #[cfg(feature = "spi1")]
    pub SPI1: self::Spi0,
    #[cfg(feature = "srcram")]
    pub SRCRAM: self::Srcram,
    #[cfg(feature = "src")]
    pub SRC: self::Src,
    #[cfg(feature = "dbg")]
    pub DBG: self::Dbg,
    #[cfg(feature = "dmac0")]
    pub DMAC0: self::Dmac0,
    #[cfg(feature = "dmac1")]
    pub DMAC1: self::Dmac0,
    #[cfg(feature = "dmac2")]
    pub DMAC2: self::Dmac0,
    #[cfg(feature = "dmac3")]
    pub DMAC3: self::Dmac0,
    #[cfg(feature = "dmac4")]
    pub DMAC4: self::Dmac0,
    #[cfg(feature = "dmac5")]
    pub DMAC5: self::Dmac0,
    #[cfg(feature = "dmac6")]
    pub DMAC6: self::Dmac0,
    #[cfg(feature = "dmac7")]
    pub DMAC7: self::Dmac0,
    #[cfg(feature = "dma")]
    pub DMA: self::Dma,
    #[cfg(feature = "dtc")]
    pub DTC: self::Dtc,
    #[cfg(feature = "mmf")]
    pub MMF: self::Mmf,
    #[cfg(feature = "sram")]
    pub SRAM: self::Sram,
    #[cfg(feature = "fcache")]
    pub FCACHE: self::Fcache,
    #[cfg(feature = "system")]
    pub SYSTEM: self::System,
    #[cfg(feature = "mstp")]
    pub MSTP: self::Mstp,
    #[cfg(feature = "rtc")]
    pub RTC: self::Rtc,
    #[cfg(feature = "agt0")]
    pub AGT0: self::Agt0,
    #[cfg(feature = "agt1")]
    pub AGT1: self::Agt0,
    #[cfg(feature = "gpt_ops")]
    pub GPT_OPS: self::GptOps,
    #[cfg(feature = "gpt32eh0")]
    pub GPT32EH0: self::Gpt32Eh0,
    #[cfg(feature = "gpt32eh1")]
    pub GPT32EH1: self::Gpt32Eh0,
    #[cfg(feature = "gpt32eh2")]
    pub GPT32EH2: self::Gpt32Eh0,
    #[cfg(feature = "gpt32eh3")]
    pub GPT32EH3: self::Gpt32Eh0,
    #[cfg(feature = "gpt32e4")]
    pub GPT32E4: self::Gpt32Eh0,
    #[cfg(feature = "gpt32e5")]
    pub GPT32E5: self::Gpt32Eh0,
    #[cfg(feature = "gpt32e6")]
    pub GPT32E6: self::Gpt32Eh0,
    #[cfg(feature = "gpt32e7")]
    pub GPT32E7: self::Gpt32Eh0,
    #[cfg(feature = "poeg")]
    pub POEG: self::Poeg,
    #[cfg(feature = "gpt_odc")]
    pub GPT_ODC: self::GptOdc,
    #[cfg(feature = "bus")]
    pub BUS: self::Bus,
    #[cfg(feature = "icu")]
    pub ICU: self::Icu,
    #[cfg(feature = "port0")]
    pub PORT0: self::Port0,
    #[cfg(feature = "port1")]
    pub PORT1: self::Port1,
    #[cfg(feature = "port2")]
    pub PORT2: self::Port1,
    #[cfg(feature = "port3")]
    pub PORT3: self::Port1,
    #[cfg(feature = "port4")]
    pub PORT4: self::Port1,
    #[cfg(feature = "port5")]
    pub PORT5: self::Port0,
    #[cfg(feature = "port6")]
    pub PORT6: self::Port0,
    #[cfg(feature = "port7")]
    pub PORT7: self::Port0,
    #[cfg(feature = "pfs")]
    pub PFS: self::Pfs,
    #[cfg(feature = "pmisc")]
    pub PMISC: self::Pmisc,
    #[cfg(feature = "iic0")]
    pub IIC0: self::Iic0,
    #[cfg(feature = "iic1")]
    pub IIC1: self::Iic1,
    #[cfg(feature = "sci0")]
    pub SCI0: self::Sci0,
    #[cfg(feature = "sci1")]
    pub SCI1: self::Sci0,
    #[cfg(feature = "sci2")]
    pub SCI2: self::Sci0,
    #[cfg(feature = "sci3")]
    pub SCI3: self::Sci0,
    #[cfg(feature = "sci4")]
    pub SCI4: self::Sci0,
    #[cfg(feature = "sci8")]
    pub SCI8: self::Sci0,
    #[cfg(feature = "sci9")]
    pub SCI9: self::Sci0,
    #[cfg(feature = "sdhi0")]
    pub SDHI0: self::Sdhi0,
    #[cfg(feature = "sdhi1")]
    pub SDHI1: self::Sdhi0,
    #[cfg(feature = "gpt328")]
    pub GPT328: self::Gpt328,
    #[cfg(feature = "gpt329")]
    pub GPT329: self::Gpt328,
    #[cfg(feature = "gpt3210")]
    pub GPT3210: self::Gpt328,
    #[cfg(feature = "gpt3211")]
    pub GPT3211: self::Gpt328,
    #[cfg(feature = "gpt3212")]
    pub GPT3212: self::Gpt328,
    #[cfg(feature = "adc120")]
    pub ADC120: self::Adc120,
    #[cfg(feature = "adc121")]
    pub ADC121: self::Adc121,
    #[cfg(feature = "ctsu")]
    pub CTSU: self::Ctsu,
    #[cfg(feature = "mmpu")]
    pub MMPU: self::Mmpu,
    #[cfg(feature = "smpu")]
    pub SMPU: self::Smpu,
    #[cfg(feature = "spmon")]
    pub SPMON: self::Spmon,
    #[cfg(feature = "tsd")]
    pub TSD: self::Tsd,
}

impl Peripherals {
    /// Returns Peripheral struct multiple times
    /// Required for compatibility with RTIC and other frameworks
    #[inline]
    pub fn take() -> Option<Self> {
        Some(Self::steal())
    }

    /// Returns Peripheral struct multiple times
    /// Required for compatibility with RTIC and other frameworks
    #[inline]
    pub fn steal() -> Self {
        Peripherals {
            #[cfg(feature = "dac12")]
            DAC12: crate::DAC12,
            #[cfg(feature = "acmphs0")]
            ACMPHS0: crate::ACMPHS0,
            #[cfg(feature = "acmphs1")]
            ACMPHS1: crate::ACMPHS1,
            #[cfg(feature = "acmphs2")]
            ACMPHS2: crate::ACMPHS2,
            #[cfg(feature = "acmphs3")]
            ACMPHS3: crate::ACMPHS3,
            #[cfg(feature = "acmphs4")]
            ACMPHS4: crate::ACMPHS4,
            #[cfg(feature = "acmphs5")]
            ACMPHS5: crate::ACMPHS5,
            #[cfg(feature = "tsn")]
            TSN: crate::TSN,
            #[cfg(feature = "crc")]
            CRC: crate::CRC,
            #[cfg(feature = "ssie0")]
            SSIE0: crate::SSIE0,
            #[cfg(feature = "elc")]
            ELC: crate::ELC,
            #[cfg(feature = "cac")]
            CAC: crate::CAC,
            #[cfg(feature = "doc")]
            DOC: crate::DOC,
            #[cfg(feature = "iwdt")]
            IWDT: crate::IWDT,
            #[cfg(feature = "kint")]
            KINT: crate::KINT,
            #[cfg(feature = "wdt")]
            WDT: crate::WDT,
            #[cfg(feature = "usbfs")]
            USBFS: crate::USBFS,
            #[cfg(feature = "can0")]
            CAN0: crate::CAN0,
            #[cfg(feature = "can1")]
            CAN1: crate::CAN1,
            #[cfg(feature = "irda")]
            IRDA: crate::IRDA,
            #[cfg(feature = "qspi")]
            QSPI: crate::QSPI,
            #[cfg(feature = "spi0")]
            SPI0: crate::SPI0,
            #[cfg(feature = "spi1")]
            SPI1: crate::SPI1,
            #[cfg(feature = "srcram")]
            SRCRAM: crate::SRCRAM,
            #[cfg(feature = "src")]
            SRC: crate::SRC,
            #[cfg(feature = "dbg")]
            DBG: crate::DBG,
            #[cfg(feature = "dmac0")]
            DMAC0: crate::DMAC0,
            #[cfg(feature = "dmac1")]
            DMAC1: crate::DMAC1,
            #[cfg(feature = "dmac2")]
            DMAC2: crate::DMAC2,
            #[cfg(feature = "dmac3")]
            DMAC3: crate::DMAC3,
            #[cfg(feature = "dmac4")]
            DMAC4: crate::DMAC4,
            #[cfg(feature = "dmac5")]
            DMAC5: crate::DMAC5,
            #[cfg(feature = "dmac6")]
            DMAC6: crate::DMAC6,
            #[cfg(feature = "dmac7")]
            DMAC7: crate::DMAC7,
            #[cfg(feature = "dma")]
            DMA: crate::DMA,
            #[cfg(feature = "dtc")]
            DTC: crate::DTC,
            #[cfg(feature = "mmf")]
            MMF: crate::MMF,
            #[cfg(feature = "sram")]
            SRAM: crate::SRAM,
            #[cfg(feature = "fcache")]
            FCACHE: crate::FCACHE,
            #[cfg(feature = "system")]
            SYSTEM: crate::SYSTEM,
            #[cfg(feature = "mstp")]
            MSTP: crate::MSTP,
            #[cfg(feature = "rtc")]
            RTC: crate::RTC,
            #[cfg(feature = "agt0")]
            AGT0: crate::AGT0,
            #[cfg(feature = "agt1")]
            AGT1: crate::AGT1,
            #[cfg(feature = "gpt_ops")]
            GPT_OPS: crate::GPT_OPS,
            #[cfg(feature = "gpt32eh0")]
            GPT32EH0: crate::GPT32EH0,
            #[cfg(feature = "gpt32eh1")]
            GPT32EH1: crate::GPT32EH1,
            #[cfg(feature = "gpt32eh2")]
            GPT32EH2: crate::GPT32EH2,
            #[cfg(feature = "gpt32eh3")]
            GPT32EH3: crate::GPT32EH3,
            #[cfg(feature = "gpt32e4")]
            GPT32E4: crate::GPT32E4,
            #[cfg(feature = "gpt32e5")]
            GPT32E5: crate::GPT32E5,
            #[cfg(feature = "gpt32e6")]
            GPT32E6: crate::GPT32E6,
            #[cfg(feature = "gpt32e7")]
            GPT32E7: crate::GPT32E7,
            #[cfg(feature = "poeg")]
            POEG: crate::POEG,
            #[cfg(feature = "gpt_odc")]
            GPT_ODC: crate::GPT_ODC,
            #[cfg(feature = "bus")]
            BUS: crate::BUS,
            #[cfg(feature = "icu")]
            ICU: crate::ICU,
            #[cfg(feature = "port0")]
            PORT0: crate::PORT0,
            #[cfg(feature = "port1")]
            PORT1: crate::PORT1,
            #[cfg(feature = "port2")]
            PORT2: crate::PORT2,
            #[cfg(feature = "port3")]
            PORT3: crate::PORT3,
            #[cfg(feature = "port4")]
            PORT4: crate::PORT4,
            #[cfg(feature = "port5")]
            PORT5: crate::PORT5,
            #[cfg(feature = "port6")]
            PORT6: crate::PORT6,
            #[cfg(feature = "port7")]
            PORT7: crate::PORT7,
            #[cfg(feature = "pfs")]
            PFS: crate::PFS,
            #[cfg(feature = "pmisc")]
            PMISC: crate::PMISC,
            #[cfg(feature = "iic0")]
            IIC0: crate::IIC0,
            #[cfg(feature = "iic1")]
            IIC1: crate::IIC1,
            #[cfg(feature = "sci0")]
            SCI0: crate::SCI0,
            #[cfg(feature = "sci1")]
            SCI1: crate::SCI1,
            #[cfg(feature = "sci2")]
            SCI2: crate::SCI2,
            #[cfg(feature = "sci3")]
            SCI3: crate::SCI3,
            #[cfg(feature = "sci4")]
            SCI4: crate::SCI4,
            #[cfg(feature = "sci8")]
            SCI8: crate::SCI8,
            #[cfg(feature = "sci9")]
            SCI9: crate::SCI9,
            #[cfg(feature = "sdhi0")]
            SDHI0: crate::SDHI0,
            #[cfg(feature = "sdhi1")]
            SDHI1: crate::SDHI1,
            #[cfg(feature = "gpt328")]
            GPT328: crate::GPT328,
            #[cfg(feature = "gpt329")]
            GPT329: crate::GPT329,
            #[cfg(feature = "gpt3210")]
            GPT3210: crate::GPT3210,
            #[cfg(feature = "gpt3211")]
            GPT3211: crate::GPT3211,
            #[cfg(feature = "gpt3212")]
            GPT3212: crate::GPT3212,
            #[cfg(feature = "adc120")]
            ADC120: crate::ADC120,
            #[cfg(feature = "adc121")]
            ADC121: crate::ADC121,
            #[cfg(feature = "ctsu")]
            CTSU: crate::CTSU,
            #[cfg(feature = "mmpu")]
            MMPU: crate::MMPU,
            #[cfg(feature = "smpu")]
            SMPU: crate::SMPU,
            #[cfg(feature = "spmon")]
            SPMON: crate::SPMON,
            #[cfg(feature = "tsd")]
            TSD: crate::TSD,
        }
    }
}
