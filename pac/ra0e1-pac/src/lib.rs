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
// Generated from SVD 1.10.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:59 +0000
#![cfg_attr(not(feature = "tracing"), no_std)]
#![allow(non_camel_case_types)]
#![doc = "Arm Cortex-M23 based Microcontroller RA0E1 device"]
pub mod common;
pub use common::*;

#[cfg(feature = "tracing")]
pub mod reg_name;
#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "adc_d")]
pub mod adc_d;
#[cfg(feature = "bus")]
pub mod bus;
#[cfg(feature = "crc")]
pub mod crc;
#[cfg(feature = "dbg")]
pub mod dbg;
#[cfg(feature = "dtc")]
pub mod dtc;
#[cfg(feature = "elc")]
pub mod elc;
#[cfg(feature = "flcn")]
pub mod flcn;
#[cfg(feature = "icu")]
pub mod icu;
#[cfg(feature = "iica")]
pub mod iica;
#[cfg(feature = "iwdt")]
pub mod iwdt;
#[cfg(feature = "mstp")]
pub mod mstp;
#[cfg(feature = "pclbuz")]
pub mod pclbuz;
#[cfg(feature = "pfs_a")]
pub mod pfs_a;
#[cfg(feature = "porga")]
pub mod porga;
#[cfg(feature = "port0")]
pub mod port0;
#[cfg(feature = "port1")]
pub mod port1;
#[cfg(feature = "port2")]
pub mod port2;
#[cfg(feature = "port3")]
pub mod port3;
#[cfg(feature = "port4")]
pub mod port4;
#[cfg(feature = "port9")]
pub mod port9;
#[cfg(feature = "rtc_c")]
pub mod rtc_c;
#[cfg(feature = "sau0")]
pub mod sau0;
#[cfg(feature = "sau1")]
pub mod sau1;
#[cfg(feature = "sram")]
pub mod sram;
#[cfg(feature = "sysc")]
pub mod sysc;
#[cfg(feature = "tau")]
pub mod tau;
#[cfg(feature = "tml32")]
pub mod tml32;
#[cfg(feature = "trng")]
pub mod trng;
#[cfg(feature = "uarta")]
pub mod uarta;

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
#[cfg(feature = "dbg")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbg {
    ptr: *mut u8,
}
#[cfg(feature = "dbg")]
pub const DBG: self::Dbg = self::Dbg {
    ptr: 0x4001b000u32 as _,
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
#[cfg(feature = "mstp")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstp {
    ptr: *mut u8,
}
#[cfg(feature = "mstp")]
pub const MSTP: self::Mstp = self::Mstp {
    ptr: 0x40047000u32 as _,
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
#[cfg(feature = "port0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port0 {
    ptr: *mut u8,
}
#[cfg(feature = "port0")]
pub const PORT0: self::Port0 = self::Port0 {
    ptr: 0x400a0000u32 as _,
};
#[cfg(feature = "port1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port1 {
    ptr: *mut u8,
}
#[cfg(feature = "port1")]
pub const PORT1: self::Port1 = self::Port1 {
    ptr: 0x400a0020u32 as _,
};
#[cfg(feature = "port2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port2 {
    ptr: *mut u8,
}
#[cfg(feature = "port2")]
pub const PORT2: self::Port2 = self::Port2 {
    ptr: 0x400a0040u32 as _,
};
#[cfg(feature = "port3")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port3 {
    ptr: *mut u8,
}
#[cfg(feature = "port3")]
pub const PORT3: self::Port3 = self::Port3 {
    ptr: 0x400a0060u32 as _,
};
#[cfg(feature = "port4")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port4 {
    ptr: *mut u8,
}
#[cfg(feature = "port4")]
pub const PORT4: self::Port4 = self::Port4 {
    ptr: 0x400a0080u32 as _,
};
#[cfg(feature = "port9")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port9 {
    ptr: *mut u8,
}
#[cfg(feature = "port9")]
pub const PORT9: self::Port9 = self::Port9 {
    ptr: 0x400a0120u32 as _,
};
#[cfg(feature = "pfs_a")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PfsA {
    ptr: *mut u8,
}
#[cfg(feature = "pfs_a")]
pub const PFS_A: self::PfsA = self::PfsA {
    ptr: 0x400a0200u32 as _,
};
#[cfg(feature = "porga")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Porga {
    ptr: *mut u8,
}
#[cfg(feature = "porga")]
pub const PORGA: self::Porga = self::Porga {
    ptr: 0x400a1000u32 as _,
};
#[cfg(feature = "adc_d")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcD {
    ptr: *mut u8,
}
#[cfg(feature = "adc_d")]
pub const ADC_D: self::AdcD = self::AdcD {
    ptr: 0x400a1800u32 as _,
};
#[cfg(feature = "sau0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sau0 {
    ptr: *mut u8,
}
#[cfg(feature = "sau0")]
pub const SAU0: self::Sau0 = self::Sau0 {
    ptr: 0x400a2000u32 as _,
};
#[cfg(feature = "sau1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sau1 {
    ptr: *mut u8,
}
#[cfg(feature = "sau1")]
pub const SAU1: self::Sau1 = self::Sau1 {
    ptr: 0x400a2200u32 as _,
};
#[cfg(feature = "tau")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tau {
    ptr: *mut u8,
}
#[cfg(feature = "tau")]
pub const TAU: self::Tau = self::Tau {
    ptr: 0x400a2600u32 as _,
};
#[cfg(feature = "rtc_c")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcC {
    ptr: *mut u8,
}
#[cfg(feature = "rtc_c")]
pub const RTC_C: self::RtcC = self::RtcC {
    ptr: 0x400a2c00u32 as _,
};
#[cfg(feature = "iica")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iica {
    ptr: *mut u8,
}
#[cfg(feature = "iica")]
pub const IICA: self::Iica = self::Iica {
    ptr: 0x400a3000u32 as _,
};
#[cfg(feature = "uarta")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uarta {
    ptr: *mut u8,
}
#[cfg(feature = "uarta")]
pub const UARTA: self::Uarta = self::Uarta {
    ptr: 0x400a3400u32 as _,
};
#[cfg(feature = "tml32")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tml32 {
    ptr: *mut u8,
}
#[cfg(feature = "tml32")]
pub const TML32: self::Tml32 = self::Tml32 {
    ptr: 0x400a3800u32 as _,
};
#[cfg(feature = "pclbuz")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pclbuz {
    ptr: *mut u8,
}
#[cfg(feature = "pclbuz")]
pub const PCLBUZ: self::Pclbuz = self::Pclbuz {
    ptr: 0x400a3b00u32 as _,
};
#[cfg(feature = "trng")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trng {
    ptr: *mut u8,
}
#[cfg(feature = "trng")]
pub const TRNG: self::Trng = self::Trng {
    ptr: 0x400d1000u32 as _,
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

pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, NVIC, SCB, SYST, TPIU};
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
    }
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 64] = [
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
    #[cfg(feature = "sram")]
    pub SRAM: self::Sram,
    #[cfg(feature = "bus")]
    pub BUS: self::Bus,
    #[cfg(feature = "dtc")]
    pub DTC: self::Dtc,
    #[cfg(feature = "icu")]
    pub ICU: self::Icu,
    #[cfg(feature = "dbg")]
    pub DBG: self::Dbg,
    #[cfg(feature = "sysc")]
    pub SYSC: self::Sysc,
    #[cfg(feature = "elc")]
    pub ELC: self::Elc,
    #[cfg(feature = "iwdt")]
    pub IWDT: self::Iwdt,
    #[cfg(feature = "mstp")]
    pub MSTP: self::Mstp,
    #[cfg(feature = "crc")]
    pub CRC: self::Crc,
    #[cfg(feature = "port0")]
    pub PORT0: self::Port0,
    #[cfg(feature = "port1")]
    pub PORT1: self::Port1,
    #[cfg(feature = "port2")]
    pub PORT2: self::Port2,
    #[cfg(feature = "port3")]
    pub PORT3: self::Port3,
    #[cfg(feature = "port4")]
    pub PORT4: self::Port4,
    #[cfg(feature = "port9")]
    pub PORT9: self::Port9,
    #[cfg(feature = "pfs_a")]
    pub PFS_A: self::PfsA,
    #[cfg(feature = "porga")]
    pub PORGA: self::Porga,
    #[cfg(feature = "adc_d")]
    pub ADC_D: self::AdcD,
    #[cfg(feature = "sau0")]
    pub SAU0: self::Sau0,
    #[cfg(feature = "sau1")]
    pub SAU1: self::Sau1,
    #[cfg(feature = "tau")]
    pub TAU: self::Tau,
    #[cfg(feature = "rtc_c")]
    pub RTC_C: self::RtcC,
    #[cfg(feature = "iica")]
    pub IICA: self::Iica,
    #[cfg(feature = "uarta")]
    pub UARTA: self::Uarta,
    #[cfg(feature = "tml32")]
    pub TML32: self::Tml32,
    #[cfg(feature = "pclbuz")]
    pub PCLBUZ: self::Pclbuz,
    #[cfg(feature = "trng")]
    pub TRNG: self::Trng,
    #[cfg(feature = "flcn")]
    pub FLCN: self::Flcn,
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
            #[cfg(feature = "sram")]
            SRAM: crate::SRAM,
            #[cfg(feature = "bus")]
            BUS: crate::BUS,
            #[cfg(feature = "dtc")]
            DTC: crate::DTC,
            #[cfg(feature = "icu")]
            ICU: crate::ICU,
            #[cfg(feature = "dbg")]
            DBG: crate::DBG,
            #[cfg(feature = "sysc")]
            SYSC: crate::SYSC,
            #[cfg(feature = "elc")]
            ELC: crate::ELC,
            #[cfg(feature = "iwdt")]
            IWDT: crate::IWDT,
            #[cfg(feature = "mstp")]
            MSTP: crate::MSTP,
            #[cfg(feature = "crc")]
            CRC: crate::CRC,
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
            #[cfg(feature = "port9")]
            PORT9: crate::PORT9,
            #[cfg(feature = "pfs_a")]
            PFS_A: crate::PFS_A,
            #[cfg(feature = "porga")]
            PORGA: crate::PORGA,
            #[cfg(feature = "adc_d")]
            ADC_D: crate::ADC_D,
            #[cfg(feature = "sau0")]
            SAU0: crate::SAU0,
            #[cfg(feature = "sau1")]
            SAU1: crate::SAU1,
            #[cfg(feature = "tau")]
            TAU: crate::TAU,
            #[cfg(feature = "rtc_c")]
            RTC_C: crate::RTC_C,
            #[cfg(feature = "iica")]
            IICA: crate::IICA,
            #[cfg(feature = "uarta")]
            UARTA: crate::UARTA,
            #[cfg(feature = "tml32")]
            TML32: crate::TML32,
            #[cfg(feature = "pclbuz")]
            PCLBUZ: crate::PCLBUZ,
            #[cfg(feature = "trng")]
            TRNG: crate::TRNG,
            #[cfg(feature = "flcn")]
            FLCN: crate::FLCN,
        }
    }
}
