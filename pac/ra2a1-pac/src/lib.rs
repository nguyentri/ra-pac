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
// Generated from SVD 1.1, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:53 +0000
#![cfg_attr(not(feature = "tracing"), no_std)]
#![allow(non_camel_case_types)]
#![doc = "ARM 32-bit Cortex-M23 Microcontroller based device, CPU clock up to 48MHz, etc."]
pub mod common;
pub use common::*;

#[cfg(feature = "tracing")]
pub mod reg_name;
#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "acmphs0")]
pub mod acmphs0;
#[cfg(feature = "acmplp")]
pub mod acmplp;
#[cfg(feature = "adc160")]
pub mod adc160;
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
#[cfg(feature = "dac8")]
pub mod dac8;
#[cfg(feature = "dbg")]
pub mod dbg;
#[cfg(feature = "doc")]
pub mod doc;
#[cfg(feature = "dtc")]
pub mod dtc;
#[cfg(feature = "elc")]
pub mod elc;
#[cfg(feature = "fcache")]
pub mod fcache;
#[cfg(feature = "gpt161")]
pub mod gpt161;
#[cfg(feature = "gpt320")]
pub mod gpt320;
#[cfg(feature = "gpt_ops")]
pub mod gpt_ops;
#[cfg(feature = "icu")]
pub mod icu;
#[cfg(feature = "iic0")]
pub mod iic0;
#[cfg(feature = "iic1")]
pub mod iic1;
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
#[cfg(feature = "opamp")]
pub mod opamp;
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
#[cfg(feature = "rtc")]
pub mod rtc;
#[cfg(feature = "sci0")]
pub mod sci0;
#[cfg(feature = "sci1")]
pub mod sci1;
#[cfg(feature = "sdadc24")]
pub mod sdadc24;
#[cfg(feature = "smpu")]
pub mod smpu;
#[cfg(feature = "spi0")]
pub mod spi0;
#[cfg(feature = "spmon")]
pub mod spmon;
#[cfg(feature = "sram")]
pub mod sram;
#[cfg(feature = "system")]
pub mod system;
#[cfg(feature = "tsn")]
pub mod tsn;
#[cfg(feature = "usbfs")]
pub mod usbfs;
#[cfg(feature = "wdt")]
pub mod wdt;

#[cfg(feature = "acmphs0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acmphs0 {
    ptr: *mut u8,
}
#[cfg(feature = "acmphs0")]
pub const ACMPHS0: self::Acmphs0 = self::Acmphs0 {
    ptr: 0x40085000u32 as _,
};
#[cfg(feature = "acmplp")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acmplp {
    ptr: *mut u8,
}
#[cfg(feature = "acmplp")]
pub const ACMPLP: self::Acmplp = self::Acmplp {
    ptr: 0x40085e00u32 as _,
};
#[cfg(feature = "adc160")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc160 {
    ptr: *mut u8,
}
#[cfg(feature = "adc160")]
pub const ADC160: self::Adc160 = self::Adc160 {
    ptr: 0x4005c000u32 as _,
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
#[cfg(feature = "opamp")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp {
    ptr: *mut u8,
}
#[cfg(feature = "opamp")]
pub const OPAMP: self::Opamp = self::Opamp {
    ptr: 0x40086800u32 as _,
};
#[cfg(feature = "sdadc24")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadc24 {
    ptr: *mut u8,
}
#[cfg(feature = "sdadc24")]
pub const SDADC24: self::Sdadc24 = self::Sdadc24 {
    ptr: 0x4009c000u32 as _,
};
#[cfg(feature = "tsn")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsn {
    ptr: *mut u8,
}
#[cfg(feature = "tsn")]
pub const TSN: self::Tsn = self::Tsn {
    ptr: 0x407ec000u32 as _,
};
#[cfg(feature = "dac12")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac12 {
    ptr: *mut u8,
}
#[cfg(feature = "dac12")]
pub const DAC12: self::Dac12 = self::Dac12 {
    ptr: 0x4005e000u32 as _,
};
#[cfg(feature = "dac8")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac8 {
    ptr: *mut u8,
}
#[cfg(feature = "dac8")]
pub const DAC8: self::Dac8 = self::Dac8 {
    ptr: 0x4009e000u32 as _,
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
#[cfg(feature = "usbfs")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbfs {
    ptr: *mut u8,
}
#[cfg(feature = "usbfs")]
pub const USBFS: self::Usbfs = self::Usbfs {
    ptr: 0x40090000u32 as _,
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
#[cfg(feature = "cac")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cac {
    ptr: *mut u8,
}
#[cfg(feature = "cac")]
pub const CAC: self::Cac = self::Cac {
    ptr: 0x40044600u32 as _,
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
#[cfg(feature = "doc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doc {
    ptr: *mut u8,
}
#[cfg(feature = "doc")]
pub const DOC: self::Doc = self::Doc {
    ptr: 0x40054100u32 as _,
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sci1 {
    ptr: *mut u8,
}
#[cfg(feature = "sci1")]
pub const SCI1: self::Sci1 = self::Sci1 {
    ptr: 0x40070020u32 as _,
};
#[cfg(feature = "sci9")]
pub const SCI9: self::Sci1 = self::Sci1 {
    ptr: 0x40070120u32 as _,
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
#[cfg(feature = "can0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can0 {
    ptr: *mut u8,
}
#[cfg(feature = "can0")]
pub const CAN0: self::Can0 = self::Can0 {
    ptr: 0x40050000u32 as _,
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
#[cfg(feature = "mmf")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmf {
    ptr: *mut u8,
}
#[cfg(feature = "mmf")]
pub const MMF: self::Mmf = self::Mmf {
    ptr: 0x40001000u32 as _,
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
#[cfg(feature = "dbg")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbg {
    ptr: *mut u8,
}
#[cfg(feature = "dbg")]
pub const DBG: self::Dbg = self::Dbg {
    ptr: 0x4001b000u32 as _,
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
#[cfg(feature = "gpt320")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt320 {
    ptr: *mut u8,
}
#[cfg(feature = "gpt320")]
pub const GPT320: self::Gpt320 = self::Gpt320 {
    ptr: 0x40078000u32 as _,
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
#[cfg(feature = "gpt161")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt161 {
    ptr: *mut u8,
}
#[cfg(feature = "gpt161")]
pub const GPT161: self::Gpt161 = self::Gpt161 {
    ptr: 0x40078100u32 as _,
};
#[cfg(feature = "gpt162")]
pub const GPT162: self::Gpt161 = self::Gpt161 {
    ptr: 0x40078200u32 as _,
};
#[cfg(feature = "gpt163")]
pub const GPT163: self::Gpt161 = self::Gpt161 {
    ptr: 0x40078300u32 as _,
};
#[cfg(feature = "gpt164")]
pub const GPT164: self::Gpt161 = self::Gpt161 {
    ptr: 0x40078400u32 as _,
};
#[cfg(feature = "gpt165")]
pub const GPT165: self::Gpt161 = self::Gpt161 {
    ptr: 0x40078500u32 as _,
};
#[cfg(feature = "gpt166")]
pub const GPT166: self::Gpt161 = self::Gpt161 {
    ptr: 0x40078600u32 as _,
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
#[cfg(feature = "rtc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
#[cfg(feature = "rtc")]
pub const RTC: self::Rtc = self::Rtc {
    ptr: 0x40044000u32 as _,
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
pub const PORT3: self::Port0 = self::Port0 {
    ptr: 0x40040060u32 as _,
};
#[cfg(feature = "port4")]
pub const PORT4: self::Port0 = self::Port0 {
    ptr: 0x40040080u32 as _,
};
#[cfg(feature = "port5")]
pub const PORT5: self::Port0 = self::Port0 {
    ptr: 0x400400a0u32 as _,
};
#[cfg(feature = "port9")]
pub const PORT9: self::Port0 = self::Port0 {
    ptr: 0x40040120u32 as _,
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

pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
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
extern "C" {
    fn IEL0();
    fn IEL1();
    fn IEL2();
    fn IEL3();
    fn IEL4();
    fn IEL5();
    fn IEL6();
    fn IEL7();
    fn IEL8();
    fn IEL9();
    fn IEL10();
    fn IEL11();
    fn IEL12();
    fn IEL13();
    fn IEL14();
    fn IEL15();
    fn IEL16();
    fn IEL17();
    fn IEL18();
    fn IEL19();
    fn IEL20();
    fn IEL21();
    fn IEL22();
    fn IEL23();
    fn IEL24();
    fn IEL25();
    fn IEL26();
    fn IEL27();
    fn IEL28();
    fn IEL29();
    fn IEL30();
    fn IEL31();
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: IEL0 },
    Vector { _handler: IEL1 },
    Vector { _handler: IEL2 },
    Vector { _handler: IEL3 },
    Vector { _handler: IEL4 },
    Vector { _handler: IEL5 },
    Vector { _handler: IEL6 },
    Vector { _handler: IEL7 },
    Vector { _handler: IEL8 },
    Vector { _handler: IEL9 },
    Vector { _handler: IEL10 },
    Vector { _handler: IEL11 },
    Vector { _handler: IEL12 },
    Vector { _handler: IEL13 },
    Vector { _handler: IEL14 },
    Vector { _handler: IEL15 },
    Vector { _handler: IEL16 },
    Vector { _handler: IEL17 },
    Vector { _handler: IEL18 },
    Vector { _handler: IEL19 },
    Vector { _handler: IEL20 },
    Vector { _handler: IEL21 },
    Vector { _handler: IEL22 },
    Vector { _handler: IEL23 },
    Vector { _handler: IEL24 },
    Vector { _handler: IEL25 },
    Vector { _handler: IEL26 },
    Vector { _handler: IEL27 },
    Vector { _handler: IEL28 },
    Vector { _handler: IEL29 },
    Vector { _handler: IEL30 },
    Vector { _handler: IEL31 },
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
    #[cfg(feature = "acmphs0")]
    pub ACMPHS0: self::Acmphs0,
    #[cfg(feature = "acmplp")]
    pub ACMPLP: self::Acmplp,
    #[cfg(feature = "adc160")]
    pub ADC160: self::Adc160,
    #[cfg(feature = "ctsu")]
    pub CTSU: self::Ctsu,
    #[cfg(feature = "opamp")]
    pub OPAMP: self::Opamp,
    #[cfg(feature = "sdadc24")]
    pub SDADC24: self::Sdadc24,
    #[cfg(feature = "tsn")]
    pub TSN: self::Tsn,
    #[cfg(feature = "dac12")]
    pub DAC12: self::Dac12,
    #[cfg(feature = "dac8")]
    pub DAC8: self::Dac8,
    #[cfg(feature = "elc")]
    pub ELC: self::Elc,
    #[cfg(feature = "iwdt")]
    pub IWDT: self::Iwdt,
    #[cfg(feature = "kint")]
    pub KINT: self::Kint,
    #[cfg(feature = "usbfs")]
    pub USBFS: self::Usbfs,
    #[cfg(feature = "wdt")]
    pub WDT: self::Wdt,
    #[cfg(feature = "cac")]
    pub CAC: self::Cac,
    #[cfg(feature = "crc")]
    pub CRC: self::Crc,
    #[cfg(feature = "doc")]
    pub DOC: self::Doc,
    #[cfg(feature = "sci0")]
    pub SCI0: self::Sci0,
    #[cfg(feature = "sci1")]
    pub SCI1: self::Sci1,
    #[cfg(feature = "sci9")]
    pub SCI9: self::Sci1,
    #[cfg(feature = "spi0")]
    pub SPI0: self::Spi0,
    #[cfg(feature = "spi1")]
    pub SPI1: self::Spi0,
    #[cfg(feature = "can0")]
    pub CAN0: self::Can0,
    #[cfg(feature = "iic0")]
    pub IIC0: self::Iic0,
    #[cfg(feature = "iic1")]
    pub IIC1: self::Iic1,
    #[cfg(feature = "mmf")]
    pub MMF: self::Mmf,
    #[cfg(feature = "mmpu")]
    pub MMPU: self::Mmpu,
    #[cfg(feature = "smpu")]
    pub SMPU: self::Smpu,
    #[cfg(feature = "spmon")]
    pub SPMON: self::Spmon,
    #[cfg(feature = "sram")]
    pub SRAM: self::Sram,
    #[cfg(feature = "bus")]
    pub BUS: self::Bus,
    #[cfg(feature = "dbg")]
    pub DBG: self::Dbg,
    #[cfg(feature = "dtc")]
    pub DTC: self::Dtc,
    #[cfg(feature = "icu")]
    pub ICU: self::Icu,
    #[cfg(feature = "system")]
    pub SYSTEM: self::System,
    #[cfg(feature = "mstp")]
    pub MSTP: self::Mstp,
    #[cfg(feature = "agt0")]
    pub AGT0: self::Agt0,
    #[cfg(feature = "agt1")]
    pub AGT1: self::Agt0,
    #[cfg(feature = "gpt320")]
    pub GPT320: self::Gpt320,
    #[cfg(feature = "gpt_ops")]
    pub GPT_OPS: self::GptOps,
    #[cfg(feature = "gpt161")]
    pub GPT161: self::Gpt161,
    #[cfg(feature = "gpt162")]
    pub GPT162: self::Gpt161,
    #[cfg(feature = "gpt163")]
    pub GPT163: self::Gpt161,
    #[cfg(feature = "gpt164")]
    pub GPT164: self::Gpt161,
    #[cfg(feature = "gpt165")]
    pub GPT165: self::Gpt161,
    #[cfg(feature = "gpt166")]
    pub GPT166: self::Gpt161,
    #[cfg(feature = "poeg")]
    pub POEG: self::Poeg,
    #[cfg(feature = "rtc")]
    pub RTC: self::Rtc,
    #[cfg(feature = "fcache")]
    pub FCACHE: self::Fcache,
    #[cfg(feature = "port0")]
    pub PORT0: self::Port0,
    #[cfg(feature = "port1")]
    pub PORT1: self::Port1,
    #[cfg(feature = "port2")]
    pub PORT2: self::Port1,
    #[cfg(feature = "port3")]
    pub PORT3: self::Port0,
    #[cfg(feature = "port4")]
    pub PORT4: self::Port0,
    #[cfg(feature = "port5")]
    pub PORT5: self::Port0,
    #[cfg(feature = "port9")]
    pub PORT9: self::Port0,
    #[cfg(feature = "pfs")]
    pub PFS: self::Pfs,
    #[cfg(feature = "pmisc")]
    pub PMISC: self::Pmisc,
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
            #[cfg(feature = "acmphs0")]
            ACMPHS0: crate::ACMPHS0,
            #[cfg(feature = "acmplp")]
            ACMPLP: crate::ACMPLP,
            #[cfg(feature = "adc160")]
            ADC160: crate::ADC160,
            #[cfg(feature = "ctsu")]
            CTSU: crate::CTSU,
            #[cfg(feature = "opamp")]
            OPAMP: crate::OPAMP,
            #[cfg(feature = "sdadc24")]
            SDADC24: crate::SDADC24,
            #[cfg(feature = "tsn")]
            TSN: crate::TSN,
            #[cfg(feature = "dac12")]
            DAC12: crate::DAC12,
            #[cfg(feature = "dac8")]
            DAC8: crate::DAC8,
            #[cfg(feature = "elc")]
            ELC: crate::ELC,
            #[cfg(feature = "iwdt")]
            IWDT: crate::IWDT,
            #[cfg(feature = "kint")]
            KINT: crate::KINT,
            #[cfg(feature = "usbfs")]
            USBFS: crate::USBFS,
            #[cfg(feature = "wdt")]
            WDT: crate::WDT,
            #[cfg(feature = "cac")]
            CAC: crate::CAC,
            #[cfg(feature = "crc")]
            CRC: crate::CRC,
            #[cfg(feature = "doc")]
            DOC: crate::DOC,
            #[cfg(feature = "sci0")]
            SCI0: crate::SCI0,
            #[cfg(feature = "sci1")]
            SCI1: crate::SCI1,
            #[cfg(feature = "sci9")]
            SCI9: crate::SCI9,
            #[cfg(feature = "spi0")]
            SPI0: crate::SPI0,
            #[cfg(feature = "spi1")]
            SPI1: crate::SPI1,
            #[cfg(feature = "can0")]
            CAN0: crate::CAN0,
            #[cfg(feature = "iic0")]
            IIC0: crate::IIC0,
            #[cfg(feature = "iic1")]
            IIC1: crate::IIC1,
            #[cfg(feature = "mmf")]
            MMF: crate::MMF,
            #[cfg(feature = "mmpu")]
            MMPU: crate::MMPU,
            #[cfg(feature = "smpu")]
            SMPU: crate::SMPU,
            #[cfg(feature = "spmon")]
            SPMON: crate::SPMON,
            #[cfg(feature = "sram")]
            SRAM: crate::SRAM,
            #[cfg(feature = "bus")]
            BUS: crate::BUS,
            #[cfg(feature = "dbg")]
            DBG: crate::DBG,
            #[cfg(feature = "dtc")]
            DTC: crate::DTC,
            #[cfg(feature = "icu")]
            ICU: crate::ICU,
            #[cfg(feature = "system")]
            SYSTEM: crate::SYSTEM,
            #[cfg(feature = "mstp")]
            MSTP: crate::MSTP,
            #[cfg(feature = "agt0")]
            AGT0: crate::AGT0,
            #[cfg(feature = "agt1")]
            AGT1: crate::AGT1,
            #[cfg(feature = "gpt320")]
            GPT320: crate::GPT320,
            #[cfg(feature = "gpt_ops")]
            GPT_OPS: crate::GPT_OPS,
            #[cfg(feature = "gpt161")]
            GPT161: crate::GPT161,
            #[cfg(feature = "gpt162")]
            GPT162: crate::GPT162,
            #[cfg(feature = "gpt163")]
            GPT163: crate::GPT163,
            #[cfg(feature = "gpt164")]
            GPT164: crate::GPT164,
            #[cfg(feature = "gpt165")]
            GPT165: crate::GPT165,
            #[cfg(feature = "gpt166")]
            GPT166: crate::GPT166,
            #[cfg(feature = "poeg")]
            POEG: crate::POEG,
            #[cfg(feature = "rtc")]
            RTC: crate::RTC,
            #[cfg(feature = "fcache")]
            FCACHE: crate::FCACHE,
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
            #[cfg(feature = "port9")]
            PORT9: crate::PORT9,
            #[cfg(feature = "pfs")]
            PFS: crate::PFS,
            #[cfg(feature = "pmisc")]
            PMISC: crate::PMISC,
        }
    }
}
