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
// Generated from SVD 1.20.01, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:48:57 +0000
#![cfg_attr(not(feature = "tracing"), no_std)]
#![allow(non_camel_case_types)]
#![doc = "Arm Cortex-M33 based Microcontroller RA4L1 group"]
pub mod common;
pub use common::*;

#[cfg(feature = "tracing")]
pub mod reg_name;
#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "acmplp")]
pub mod acmplp;
#[cfg(feature = "adc120")]
pub mod adc120;
#[cfg(feature = "agtw0")]
pub mod agtw0;
#[cfg(feature = "bus")]
pub mod bus;
#[cfg(feature = "cac")]
pub mod cac;
#[cfg(feature = "cache")]
pub mod cache;
#[cfg(feature = "canfd")]
pub mod canfd;
#[cfg(feature = "cibc")]
pub mod cibc;
#[cfg(feature = "cpscu")]
pub mod cpscu;
#[cfg(feature = "cpu_ocd")]
pub mod cpu_ocd;
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
#[cfg(feature = "doc_b")]
pub mod doc_b;
#[cfg(feature = "dtc")]
pub mod dtc;
#[cfg(feature = "eccmb")]
pub mod eccmb;
#[cfg(feature = "elc")]
pub mod elc;
#[cfg(feature = "faci")]
pub mod faci;
#[cfg(feature = "fcache")]
pub mod fcache;
#[cfg(feature = "flcn")]
pub mod flcn;
#[cfg(feature = "gpt162")]
pub mod gpt162;
#[cfg(feature = "gpt320")]
pub mod gpt320;
#[cfg(feature = "gpt_ops")]
pub mod gpt_ops;
#[cfg(feature = "i3c")]
pub mod i3c;
#[cfg(feature = "icu")]
pub mod icu;
#[cfg(feature = "iic0")]
pub mod iic0;
#[cfg(feature = "iic0wu")]
pub mod iic0wu;
#[cfg(feature = "iwdt")]
pub mod iwdt;
#[cfg(feature = "mstp")]
pub mod mstp;
#[cfg(feature = "pfs")]
pub mod pfs;
#[cfg(feature = "poeg")]
pub mod poeg;
#[cfg(feature = "port0")]
pub mod port0;
#[cfg(feature = "port1")]
pub mod port1;
#[cfg(feature = "pscu")]
pub mod pscu;
#[cfg(feature = "qspi")]
pub mod qspi;
#[cfg(feature = "rmpu")]
pub mod rmpu;
#[cfg(feature = "rtc")]
pub mod rtc;
#[cfg(feature = "sci0")]
pub mod sci0;
#[cfg(feature = "sci5")]
pub mod sci0;
#[cfg(feature = "sci1")]
pub mod sci1;
#[cfg(feature = "sci3")]
pub mod sci3;
#[cfg(feature = "slcdc")]
pub mod slcdc;
#[cfg(feature = "spi0")]
pub mod spi0;
#[cfg(feature = "sram")]
pub mod sram;
#[cfg(feature = "ssie0")]
pub mod ssie0;
#[cfg(feature = "sysc")]
pub mod sysc;
#[cfg(feature = "tzf")]
pub mod tzf;
#[cfg(feature = "uarta")]
pub mod uarta;
#[cfg(feature = "usbfs")]
pub mod usbfs;
#[cfg(feature = "wdt")]
pub mod wdt;

#[cfg(feature = "rmpu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmpu {
    ptr: *mut u8,
}
#[cfg(feature = "rmpu")]
pub const RMPU: self::Rmpu = self::Rmpu {
    ptr: 0x40000000u32 as _,
};
#[cfg(feature = "tzf")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tzf {
    ptr: *mut u8,
}
#[cfg(feature = "tzf")]
pub const TZF: self::Tzf = self::Tzf {
    ptr: 0x40000e00u32 as _,
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
#[cfg(feature = "bus")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bus {
    ptr: *mut u8,
}
#[cfg(feature = "bus")]
pub const BUS: self::Bus = self::Bus {
    ptr: 0x40003000u32 as _,
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
#[cfg(feature = "icu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icu {
    ptr: *mut u8,
}
#[cfg(feature = "icu")]
pub const ICU: self::Icu = self::Icu {
    ptr: 0x40006000u32 as _,
};
#[cfg(feature = "cache")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cache {
    ptr: *mut u8,
}
#[cfg(feature = "cache")]
pub const CACHE: self::Cache = self::Cache {
    ptr: 0x40007000u32 as _,
};
#[cfg(feature = "cpscu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpscu {
    ptr: *mut u8,
}
#[cfg(feature = "cpscu")]
pub const CPSCU: self::Cpscu = self::Cpscu {
    ptr: 0x40008000u32 as _,
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
#[cfg(feature = "fcache")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcache {
    ptr: *mut u8,
}
#[cfg(feature = "fcache")]
pub const FCACHE: self::Fcache = self::Fcache {
    ptr: 0x4001c100u32 as _,
};
#[cfg(feature = "sysc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysc {
    ptr: *mut u8,
}
#[cfg(feature = "sysc")]
pub const SYSC: self::Sysc = self::Sysc {
    ptr: 0x4001e000u32 as _,
};
#[cfg(feature = "port0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port0 {
    ptr: *mut u8,
}
#[cfg(feature = "port0")]
pub const PORT0: self::Port0 = self::Port0 {
    ptr: 0x4001f000u32 as _,
};
#[cfg(feature = "port1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port1 {
    ptr: *mut u8,
}
#[cfg(feature = "port1")]
pub const PORT1: self::Port1 = self::Port1 {
    ptr: 0x4001f020u32 as _,
};
#[cfg(feature = "port2")]
pub const PORT2: self::Port1 = self::Port1 {
    ptr: 0x4001f040u32 as _,
};
#[cfg(feature = "port3")]
pub const PORT3: self::Port1 = self::Port1 {
    ptr: 0x4001f060u32 as _,
};
#[cfg(feature = "port4")]
pub const PORT4: self::Port1 = self::Port1 {
    ptr: 0x4001f080u32 as _,
};
#[cfg(feature = "port5")]
pub const PORT5: self::Port0 = self::Port0 {
    ptr: 0x4001f0a0u32 as _,
};
#[cfg(feature = "port8")]
pub const PORT8: self::Port0 = self::Port0 {
    ptr: 0x4001f100u32 as _,
};
#[cfg(feature = "pfs")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfs {
    ptr: *mut u8,
}
#[cfg(feature = "pfs")]
pub const PFS: self::Pfs = self::Pfs {
    ptr: 0x4001f800u32 as _,
};
#[cfg(feature = "elc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elc {
    ptr: *mut u8,
}
#[cfg(feature = "elc")]
pub const ELC: self::Elc = self::Elc {
    ptr: 0x40082000u32 as _,
};
#[cfg(feature = "rtc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
#[cfg(feature = "rtc")]
pub const RTC: self::Rtc = self::Rtc {
    ptr: 0x40083000u32 as _,
};
#[cfg(feature = "iwdt")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdt {
    ptr: *mut u8,
}
#[cfg(feature = "iwdt")]
pub const IWDT: self::Iwdt = self::Iwdt {
    ptr: 0x40083200u32 as _,
};
#[cfg(feature = "wdt")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt {
    ptr: *mut u8,
}
#[cfg(feature = "wdt")]
pub const WDT: self::Wdt = self::Wdt {
    ptr: 0x40083400u32 as _,
};
#[cfg(feature = "cac")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cac {
    ptr: *mut u8,
}
#[cfg(feature = "cac")]
pub const CAC: self::Cac = self::Cac {
    ptr: 0x40083600u32 as _,
};
#[cfg(feature = "mstp")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstp {
    ptr: *mut u8,
}
#[cfg(feature = "mstp")]
pub const MSTP: self::Mstp = self::Mstp {
    ptr: 0x40084000u32 as _,
};
#[cfg(feature = "poeg")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Poeg {
    ptr: *mut u8,
}
#[cfg(feature = "poeg")]
pub const POEG: self::Poeg = self::Poeg {
    ptr: 0x4008a000u32 as _,
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
#[cfg(feature = "uarta")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uarta {
    ptr: *mut u8,
}
#[cfg(feature = "uarta")]
pub const UARTA: self::Uarta = self::Uarta {
    ptr: 0x40097000u32 as _,
};
#[cfg(feature = "ssie0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssie0 {
    ptr: *mut u8,
}
#[cfg(feature = "ssie0")]
pub const SSIE0: self::Ssie0 = self::Ssie0 {
    ptr: 0x4009d000u32 as _,
};
#[cfg(feature = "iic0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iic0 {
    ptr: *mut u8,
}
#[cfg(feature = "iic0")]
pub const IIC0: self::Iic0 = self::Iic0 {
    ptr: 0x4009f000u32 as _,
};
#[cfg(feature = "iic0wu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iic0Wu {
    ptr: *mut u8,
}
#[cfg(feature = "iic0wu")]
pub const IIC0WU: self::Iic0Wu = self::Iic0Wu {
    ptr: 0x4009f014u32 as _,
};
#[cfg(feature = "canfd")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Canfd {
    ptr: *mut u8,
}
#[cfg(feature = "canfd")]
pub const CANFD: self::Canfd = self::Canfd {
    ptr: 0x400b0000u32 as _,
};
#[cfg(feature = "ctsu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsu {
    ptr: *mut u8,
}
#[cfg(feature = "ctsu")]
pub const CTSU: self::Ctsu = self::Ctsu {
    ptr: 0x400d0000u32 as _,
};
#[cfg(feature = "slcdc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slcdc {
    ptr: *mut u8,
}
#[cfg(feature = "slcdc")]
pub const SLCDC: self::Slcdc = self::Slcdc {
    ptr: 0x400d4000u32 as _,
};
#[cfg(feature = "pscu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscu {
    ptr: *mut u8,
}
#[cfg(feature = "pscu")]
pub const PSCU: self::Pscu = self::Pscu {
    ptr: 0x400e0000u32 as _,
};
#[cfg(feature = "agtw0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtw0 {
    ptr: *mut u8,
}
#[cfg(feature = "agtw0")]
pub const AGTW0: self::Agtw0 = self::Agtw0 {
    ptr: 0x400e8000u32 as _,
};
#[cfg(feature = "agtw1")]
pub const AGTW1: self::Agtw0 = self::Agtw0 {
    ptr: 0x400e8100u32 as _,
};
#[cfg(feature = "acmplp")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acmplp {
    ptr: *mut u8,
}
#[cfg(feature = "acmplp")]
pub const ACMPLP: self::Acmplp = self::Acmplp {
    ptr: 0x400f4000u32 as _,
};
#[cfg(feature = "crc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crc {
    ptr: *mut u8,
}
#[cfg(feature = "crc")]
pub const CRC: self::Crc = self::Crc {
    ptr: 0x40108000u32 as _,
};
#[cfg(feature = "doc_b")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DocB {
    ptr: *mut u8,
}
#[cfg(feature = "doc_b")]
pub const DOC_B: self::DocB = self::DocB {
    ptr: 0x40109000u32 as _,
};
#[cfg(feature = "sci0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sci0 {
    ptr: *mut u8,
}
#[cfg(feature = "sci0")]
pub const SCI0: self::Sci0 = self::Sci0 {
    ptr: 0x40118000u32 as _,
};
#[cfg(feature = "sci1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sci1 {
    ptr: *mut u8,
}
#[cfg(feature = "sci1")]
pub const SCI1: self::Sci1 = self::Sci1 {
    ptr: 0x40118100u32 as _,
};
#[cfg(feature = "sci3")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sci3 {
    ptr: *mut u8,
}
#[cfg(feature = "sci3")]
pub const SCI3: self::Sci3 = self::Sci3 {
    ptr: 0x40118300u32 as _,
};
#[cfg(feature = "sci4")]
pub const SCI4: self::Sci3 = self::Sci3 {
    ptr: 0x40118400u32 as _,
};
#[cfg(feature = "sci5")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sci0 {
    ptr: *mut u8,
}
#[cfg(feature = "sci5")]
pub const SCI5: self::Sci0 = self::Sci0 {
    ptr: 0x40118500u32 as _,
};
#[cfg(feature = "sci9")]
pub const SCI9: self::Sci0 = self::Sci0 {
    ptr: 0x40118900u32 as _,
};
#[cfg(feature = "spi0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi0 {
    ptr: *mut u8,
}
#[cfg(feature = "spi0")]
pub const SPI0: self::Spi0 = self::Spi0 {
    ptr: 0x4011a000u32 as _,
};
#[cfg(feature = "i3c")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C {
    ptr: *mut u8,
}
#[cfg(feature = "i3c")]
pub const I3C: self::I3C = self::I3C {
    ptr: 0x4011f000u32 as _,
};
#[cfg(feature = "eccmb")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccmb {
    ptr: *mut u8,
}
#[cfg(feature = "eccmb")]
pub const ECCMB: self::Eccmb = self::Eccmb {
    ptr: 0x4012f000u32 as _,
};
#[cfg(feature = "gpt320")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt320 {
    ptr: *mut u8,
}
#[cfg(feature = "gpt320")]
pub const GPT320: self::Gpt320 = self::Gpt320 {
    ptr: 0x40169000u32 as _,
};
#[cfg(feature = "gpt321")]
pub const GPT321: self::Gpt320 = self::Gpt320 {
    ptr: 0x40169100u32 as _,
};
#[cfg(feature = "gpt162")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt162 {
    ptr: *mut u8,
}
#[cfg(feature = "gpt162")]
pub const GPT162: self::Gpt162 = self::Gpt162 {
    ptr: 0x40169200u32 as _,
};
#[cfg(feature = "gpt163")]
pub const GPT163: self::Gpt162 = self::Gpt162 {
    ptr: 0x40169300u32 as _,
};
#[cfg(feature = "gpt164")]
pub const GPT164: self::Gpt162 = self::Gpt162 {
    ptr: 0x40169400u32 as _,
};
#[cfg(feature = "gpt165")]
pub const GPT165: self::Gpt162 = self::Gpt162 {
    ptr: 0x40169500u32 as _,
};
#[cfg(feature = "gpt_ops")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GptOps {
    ptr: *mut u8,
}
#[cfg(feature = "gpt_ops")]
pub const GPT_OPS: self::GptOps = self::GptOps {
    ptr: 0x40169a00u32 as _,
};
#[cfg(feature = "adc120")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc120 {
    ptr: *mut u8,
}
#[cfg(feature = "adc120")]
pub const ADC120: self::Adc120 = self::Adc120 {
    ptr: 0x40170000u32 as _,
};
#[cfg(feature = "dac12")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac12 {
    ptr: *mut u8,
}
#[cfg(feature = "dac12")]
pub const DAC12: self::Dac12 = self::Dac12 {
    ptr: 0x40171000u32 as _,
};
#[cfg(feature = "flcn")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flcn {
    ptr: *mut u8,
}
#[cfg(feature = "flcn")]
pub const FLCN: self::Flcn = self::Flcn {
    ptr: 0x407ec000u32 as _,
};
#[cfg(feature = "cibc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cibc {
    ptr: *mut u8,
}
#[cfg(feature = "cibc")]
pub const CIBC: self::Cibc = self::Cibc {
    ptr: 0x407ec200u32 as _,
};
#[cfg(feature = "faci")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Faci {
    ptr: *mut u8,
}
#[cfg(feature = "faci")]
pub const FACI: self::Faci = self::Faci {
    ptr: 0x407fe000u32 as _,
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
#[cfg(feature = "cpu_ocd")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpuOcd {
    ptr: *mut u8,
}
#[cfg(feature = "cpu_ocd")]
pub const CPU_OCD: self::CpuOcd = self::CpuOcd {
    ptr: 0x80000000u32 as _,
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
    #[cfg(feature = "rmpu")]
    pub RMPU: self::Rmpu,
    #[cfg(feature = "tzf")]
    pub TZF: self::Tzf,
    #[cfg(feature = "sram")]
    pub SRAM: self::Sram,
    #[cfg(feature = "bus")]
    pub BUS: self::Bus,
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
    #[cfg(feature = "icu")]
    pub ICU: self::Icu,
    #[cfg(feature = "cache")]
    pub CACHE: self::Cache,
    #[cfg(feature = "cpscu")]
    pub CPSCU: self::Cpscu,
    #[cfg(feature = "dbg")]
    pub DBG: self::Dbg,
    #[cfg(feature = "fcache")]
    pub FCACHE: self::Fcache,
    #[cfg(feature = "sysc")]
    pub SYSC: self::Sysc,
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
    #[cfg(feature = "port8")]
    pub PORT8: self::Port0,
    #[cfg(feature = "pfs")]
    pub PFS: self::Pfs,
    #[cfg(feature = "elc")]
    pub ELC: self::Elc,
    #[cfg(feature = "rtc")]
    pub RTC: self::Rtc,
    #[cfg(feature = "iwdt")]
    pub IWDT: self::Iwdt,
    #[cfg(feature = "wdt")]
    pub WDT: self::Wdt,
    #[cfg(feature = "cac")]
    pub CAC: self::Cac,
    #[cfg(feature = "mstp")]
    pub MSTP: self::Mstp,
    #[cfg(feature = "poeg")]
    pub POEG: self::Poeg,
    #[cfg(feature = "usbfs")]
    pub USBFS: self::Usbfs,
    #[cfg(feature = "uarta")]
    pub UARTA: self::Uarta,
    #[cfg(feature = "ssie0")]
    pub SSIE0: self::Ssie0,
    #[cfg(feature = "iic0")]
    pub IIC0: self::Iic0,
    #[cfg(feature = "iic0wu")]
    pub IIC0WU: self::Iic0Wu,
    #[cfg(feature = "canfd")]
    pub CANFD: self::Canfd,
    #[cfg(feature = "ctsu")]
    pub CTSU: self::Ctsu,
    #[cfg(feature = "slcdc")]
    pub SLCDC: self::Slcdc,
    #[cfg(feature = "pscu")]
    pub PSCU: self::Pscu,
    #[cfg(feature = "agtw0")]
    pub AGTW0: self::Agtw0,
    #[cfg(feature = "agtw1")]
    pub AGTW1: self::Agtw0,
    #[cfg(feature = "acmplp")]
    pub ACMPLP: self::Acmplp,
    #[cfg(feature = "crc")]
    pub CRC: self::Crc,
    #[cfg(feature = "doc_b")]
    pub DOC_B: self::DocB,
    #[cfg(feature = "sci0")]
    pub SCI0: self::Sci0,
    #[cfg(feature = "sci1")]
    pub SCI1: self::Sci1,
    #[cfg(feature = "sci3")]
    pub SCI3: self::Sci3,
    #[cfg(feature = "sci4")]
    pub SCI4: self::Sci3,
    #[cfg(feature = "sci5")]
    pub SCI5: self::Sci0,
    #[cfg(feature = "sci9")]
    pub SCI9: self::Sci0,
    #[cfg(feature = "spi0")]
    pub SPI0: self::Spi0,
    #[cfg(feature = "i3c")]
    pub I3C: self::I3C,
    #[cfg(feature = "eccmb")]
    pub ECCMB: self::Eccmb,
    #[cfg(feature = "gpt320")]
    pub GPT320: self::Gpt320,
    #[cfg(feature = "gpt321")]
    pub GPT321: self::Gpt320,
    #[cfg(feature = "gpt162")]
    pub GPT162: self::Gpt162,
    #[cfg(feature = "gpt163")]
    pub GPT163: self::Gpt162,
    #[cfg(feature = "gpt164")]
    pub GPT164: self::Gpt162,
    #[cfg(feature = "gpt165")]
    pub GPT165: self::Gpt162,
    #[cfg(feature = "gpt_ops")]
    pub GPT_OPS: self::GptOps,
    #[cfg(feature = "adc120")]
    pub ADC120: self::Adc120,
    #[cfg(feature = "dac12")]
    pub DAC12: self::Dac12,
    #[cfg(feature = "flcn")]
    pub FLCN: self::Flcn,
    #[cfg(feature = "cibc")]
    pub CIBC: self::Cibc,
    #[cfg(feature = "faci")]
    pub FACI: self::Faci,
    #[cfg(feature = "qspi")]
    pub QSPI: self::Qspi,
    #[cfg(feature = "cpu_ocd")]
    pub CPU_OCD: self::CpuOcd,
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
            #[cfg(feature = "rmpu")]
            RMPU: crate::RMPU,
            #[cfg(feature = "tzf")]
            TZF: crate::TZF,
            #[cfg(feature = "sram")]
            SRAM: crate::SRAM,
            #[cfg(feature = "bus")]
            BUS: crate::BUS,
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
            #[cfg(feature = "icu")]
            ICU: crate::ICU,
            #[cfg(feature = "cache")]
            CACHE: crate::CACHE,
            #[cfg(feature = "cpscu")]
            CPSCU: crate::CPSCU,
            #[cfg(feature = "dbg")]
            DBG: crate::DBG,
            #[cfg(feature = "fcache")]
            FCACHE: crate::FCACHE,
            #[cfg(feature = "sysc")]
            SYSC: crate::SYSC,
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
            #[cfg(feature = "port8")]
            PORT8: crate::PORT8,
            #[cfg(feature = "pfs")]
            PFS: crate::PFS,
            #[cfg(feature = "elc")]
            ELC: crate::ELC,
            #[cfg(feature = "rtc")]
            RTC: crate::RTC,
            #[cfg(feature = "iwdt")]
            IWDT: crate::IWDT,
            #[cfg(feature = "wdt")]
            WDT: crate::WDT,
            #[cfg(feature = "cac")]
            CAC: crate::CAC,
            #[cfg(feature = "mstp")]
            MSTP: crate::MSTP,
            #[cfg(feature = "poeg")]
            POEG: crate::POEG,
            #[cfg(feature = "usbfs")]
            USBFS: crate::USBFS,
            #[cfg(feature = "uarta")]
            UARTA: crate::UARTA,
            #[cfg(feature = "ssie0")]
            SSIE0: crate::SSIE0,
            #[cfg(feature = "iic0")]
            IIC0: crate::IIC0,
            #[cfg(feature = "iic0wu")]
            IIC0WU: crate::IIC0WU,
            #[cfg(feature = "canfd")]
            CANFD: crate::CANFD,
            #[cfg(feature = "ctsu")]
            CTSU: crate::CTSU,
            #[cfg(feature = "slcdc")]
            SLCDC: crate::SLCDC,
            #[cfg(feature = "pscu")]
            PSCU: crate::PSCU,
            #[cfg(feature = "agtw0")]
            AGTW0: crate::AGTW0,
            #[cfg(feature = "agtw1")]
            AGTW1: crate::AGTW1,
            #[cfg(feature = "acmplp")]
            ACMPLP: crate::ACMPLP,
            #[cfg(feature = "crc")]
            CRC: crate::CRC,
            #[cfg(feature = "doc_b")]
            DOC_B: crate::DOC_B,
            #[cfg(feature = "sci0")]
            SCI0: crate::SCI0,
            #[cfg(feature = "sci1")]
            SCI1: crate::SCI1,
            #[cfg(feature = "sci3")]
            SCI3: crate::SCI3,
            #[cfg(feature = "sci4")]
            SCI4: crate::SCI4,
            #[cfg(feature = "sci5")]
            SCI5: crate::SCI5,
            #[cfg(feature = "sci9")]
            SCI9: crate::SCI9,
            #[cfg(feature = "spi0")]
            SPI0: crate::SPI0,
            #[cfg(feature = "i3c")]
            I3C: crate::I3C,
            #[cfg(feature = "eccmb")]
            ECCMB: crate::ECCMB,
            #[cfg(feature = "gpt320")]
            GPT320: crate::GPT320,
            #[cfg(feature = "gpt321")]
            GPT321: crate::GPT321,
            #[cfg(feature = "gpt162")]
            GPT162: crate::GPT162,
            #[cfg(feature = "gpt163")]
            GPT163: crate::GPT163,
            #[cfg(feature = "gpt164")]
            GPT164: crate::GPT164,
            #[cfg(feature = "gpt165")]
            GPT165: crate::GPT165,
            #[cfg(feature = "gpt_ops")]
            GPT_OPS: crate::GPT_OPS,
            #[cfg(feature = "adc120")]
            ADC120: crate::ADC120,
            #[cfg(feature = "dac12")]
            DAC12: crate::DAC12,
            #[cfg(feature = "flcn")]
            FLCN: crate::FLCN,
            #[cfg(feature = "cibc")]
            CIBC: crate::CIBC,
            #[cfg(feature = "faci")]
            FACI: crate::FACI,
            #[cfg(feature = "qspi")]
            QSPI: crate::QSPI,
            #[cfg(feature = "cpu_ocd")]
            CPU_OCD: crate::CPU_OCD,
        }
    }
}
