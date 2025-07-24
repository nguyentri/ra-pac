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
// Generated from SVD 1.00.01, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:53:56 +0000
#![cfg_attr(not(feature = "tracing"), no_std)]
#![allow(non_camel_case_types)]
#![doc = "Arm Cortex-M85 based Microcontroller RA8E1 group"]
pub mod common;
pub use common::*;

#[cfg(feature = "tracing")]
pub mod reg_name;
#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "acmphs0")]
pub mod acmphs0;
#[cfg(feature = "acmphs0_ns")]
pub mod acmphs0_ns;
#[cfg(feature = "adc120")]
pub mod adc120;
#[cfg(feature = "adc120_ns")]
pub mod adc120_ns;
#[cfg(feature = "adc121")]
pub mod adc121;
#[cfg(feature = "adc121_ns")]
pub mod adc121_ns;
#[cfg(feature = "agt0")]
pub mod agt0;
#[cfg(feature = "agt0_ns")]
pub mod agt0_ns;
#[cfg(feature = "bus")]
pub mod bus;
#[cfg(feature = "bus_ns")]
pub mod bus_ns;
#[cfg(feature = "cac")]
pub mod cac;
#[cfg(feature = "cac_ns")]
pub mod cac_ns;
#[cfg(feature = "canfd0")]
pub mod canfd0;
#[cfg(feature = "canfd0_ns")]
pub mod canfd0_ns;
#[cfg(feature = "ceu")]
pub mod ceu;
#[cfg(feature = "ceu_ns")]
pub mod ceu_ns;
#[cfg(feature = "cpscu")]
pub mod cpscu;
#[cfg(feature = "cpscu_ns")]
pub mod cpscu_ns;
#[cfg(feature = "cpu_ctrl")]
pub mod cpu_ctrl;
#[cfg(feature = "cpu_ctrl_ns")]
pub mod cpu_ctrl_ns;
#[cfg(feature = "cpu_dbg")]
pub mod cpu_dbg;
#[cfg(feature = "cpu_dbg_ns")]
pub mod cpu_dbg_ns;
#[cfg(feature = "crc")]
pub mod crc;
#[cfg(feature = "crc_ns")]
pub mod crc_ns;
#[cfg(feature = "dac12")]
pub mod dac12;
#[cfg(feature = "dac12_ns")]
pub mod dac12_ns;
#[cfg(feature = "dma0")]
pub mod dma0;
#[cfg(feature = "dma0_ns")]
pub mod dma0_ns;
#[cfg(feature = "dmac00")]
pub mod dmac00;
#[cfg(feature = "dmac00_ns")]
pub mod dmac00_ns;
#[cfg(feature = "doc_b")]
pub mod doc_b;
#[cfg(feature = "doc_b_ns")]
pub mod doc_b_ns;
#[cfg(feature = "dtc0")]
pub mod dtc0;
#[cfg(feature = "dtc0_ns")]
pub mod dtc0_ns;
#[cfg(feature = "eccmb0")]
pub mod eccmb0;
#[cfg(feature = "eccmb0_ns")]
pub mod eccmb0_ns;
#[cfg(feature = "edmac0")]
pub mod edmac0;
#[cfg(feature = "edmac0_ns")]
pub mod edmac0_ns;
#[cfg(feature = "elc")]
pub mod elc;
#[cfg(feature = "elc_ns")]
pub mod elc_ns;
#[cfg(feature = "etherc0")]
pub mod etherc0;
#[cfg(feature = "etherc0_ns")]
pub mod etherc0_ns;
#[cfg(feature = "faci")]
pub mod faci;
#[cfg(feature = "faci_ns")]
pub mod faci_ns;
#[cfg(feature = "fcache")]
pub mod fcache;
#[cfg(feature = "fcache_ns")]
pub mod fcache_ns;
#[cfg(feature = "flad")]
pub mod flad;
#[cfg(feature = "flad_ns")]
pub mod flad_ns;
#[cfg(feature = "gpt1610")]
pub mod gpt1610;
#[cfg(feature = "gpt1610_ns")]
pub mod gpt1610_ns;
#[cfg(feature = "gpt168")]
pub mod gpt168;
#[cfg(feature = "gpt168_ns")]
pub mod gpt168_ns;
#[cfg(feature = "gpt320")]
pub mod gpt320;
#[cfg(feature = "gpt320_ns")]
pub mod gpt320_ns;
#[cfg(feature = "icu")]
pub mod icu;
#[cfg(feature = "icu_common")]
pub mod icu_common;
#[cfg(feature = "icu_common_ns")]
pub mod icu_common_ns;
#[cfg(feature = "icu_ns")]
pub mod icu_ns;
#[cfg(feature = "iic0")]
pub mod iic0;
#[cfg(feature = "iic0_ns")]
pub mod iic0_ns;
#[cfg(feature = "iic0wu")]
pub mod iic0wu;
#[cfg(feature = "iic0wu_ns")]
pub mod iic0wu_ns;
#[cfg(feature = "iwdt")]
pub mod iwdt;
#[cfg(feature = "iwdt_ns")]
pub mod iwdt_ns;
#[cfg(feature = "mstp")]
pub mod mstp;
#[cfg(feature = "mstp_ns")]
pub mod mstp_ns;
#[cfg(feature = "ocd_cpu")]
pub mod ocd_cpu;
#[cfg(feature = "ocd_cpu_ns")]
pub mod ocd_cpu_ns;
#[cfg(feature = "ospi0_b")]
pub mod ospi0_b;
#[cfg(feature = "ospi0_b_ns")]
pub mod ospi0_b_ns;
#[cfg(feature = "pfs")]
pub mod pfs;
#[cfg(feature = "pfs_ns")]
pub mod pfs_ns;
#[cfg(feature = "poeg")]
pub mod poeg;
#[cfg(feature = "poeg_ns")]
pub mod poeg_ns;
#[cfg(feature = "port0")]
pub mod port0;
#[cfg(feature = "port0_ns")]
pub mod port0_ns;
#[cfg(feature = "port1")]
pub mod port1;
#[cfg(feature = "port1_ns")]
pub mod port1_ns;
#[cfg(feature = "porta")]
pub mod porta;
#[cfg(feature = "porta_ns")]
pub mod porta_ns;
#[cfg(feature = "portb")]
pub mod portb;
#[cfg(feature = "portb_ns")]
pub mod portb_ns;
#[cfg(feature = "pscu")]
pub mod pscu;
#[cfg(feature = "pscu_ns")]
pub mod pscu_ns;
#[cfg(feature = "rmpu")]
pub mod rmpu;
#[cfg(feature = "rmpu_ns")]
pub mod rmpu_ns;
#[cfg(feature = "rtc")]
pub mod rtc;
#[cfg(feature = "rtc_ns")]
pub mod rtc_ns;
#[cfg(feature = "sci0_b")]
pub mod sci0_b;
#[cfg(feature = "sci0_b_ns")]
pub mod sci0_b_ns;
#[cfg(feature = "spi0_b")]
pub mod spi0_b;
#[cfg(feature = "spi0_b_ns")]
pub mod spi0_b_ns;
#[cfg(feature = "sram")]
pub mod sram;
#[cfg(feature = "sram_ns")]
pub mod sram_ns;
#[cfg(feature = "ssie0")]
pub mod ssie0;
#[cfg(feature = "ssie0_ns")]
pub mod ssie0_ns;
#[cfg(feature = "sysc")]
pub mod sysc;
#[cfg(feature = "sysc_ns")]
pub mod sysc_ns;
#[cfg(feature = "tsd")]
pub mod tsd;
#[cfg(feature = "tsd_ns")]
pub mod tsd_ns;
#[cfg(feature = "tsn")]
pub mod tsn;
#[cfg(feature = "tsn_ns")]
pub mod tsn_ns;
#[cfg(feature = "ulpt0")]
pub mod ulpt0;
#[cfg(feature = "ulpt0_ns")]
pub mod ulpt0_ns;
#[cfg(feature = "usbfs")]
pub mod usbfs;
#[cfg(feature = "usbfs_ns")]
pub mod usbfs_ns;
#[cfg(feature = "wdt0")]
pub mod wdt0;
#[cfg(feature = "wdt0_ns")]
pub mod wdt0_ns;

#[cfg(feature = "rmpu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmpu {
    ptr: *mut u8,
}
#[cfg(feature = "rmpu")]
pub const RMPU: self::Rmpu = self::Rmpu {
    ptr: 0x40000000u32 as _,
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
#[cfg(feature = "icu_common")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IcuCommon {
    ptr: *mut u8,
}
#[cfg(feature = "icu_common")]
pub const ICU_COMMON: self::IcuCommon = self::IcuCommon {
    ptr: 0x40006000u32 as _,
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
#[cfg(feature = "dmac00")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac00 {
    ptr: *mut u8,
}
#[cfg(feature = "dmac00")]
pub const DMAC00: self::Dmac00 = self::Dmac00 {
    ptr: 0x4000a000u32 as _,
};
#[cfg(feature = "dmac01")]
pub const DMAC01: self::Dmac00 = self::Dmac00 {
    ptr: 0x4000a040u32 as _,
};
#[cfg(feature = "dmac02")]
pub const DMAC02: self::Dmac00 = self::Dmac00 {
    ptr: 0x4000a080u32 as _,
};
#[cfg(feature = "dmac03")]
pub const DMAC03: self::Dmac00 = self::Dmac00 {
    ptr: 0x4000a0c0u32 as _,
};
#[cfg(feature = "dmac04")]
pub const DMAC04: self::Dmac00 = self::Dmac00 {
    ptr: 0x4000a100u32 as _,
};
#[cfg(feature = "dmac05")]
pub const DMAC05: self::Dmac00 = self::Dmac00 {
    ptr: 0x4000a140u32 as _,
};
#[cfg(feature = "dmac06")]
pub const DMAC06: self::Dmac00 = self::Dmac00 {
    ptr: 0x4000a180u32 as _,
};
#[cfg(feature = "dmac07")]
pub const DMAC07: self::Dmac00 = self::Dmac00 {
    ptr: 0x4000a1c0u32 as _,
};
#[cfg(feature = "dma0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0 {
    ptr: *mut u8,
}
#[cfg(feature = "dma0")]
pub const DMA0: self::Dma0 = self::Dma0 {
    ptr: 0x4000a800u32 as _,
};
#[cfg(feature = "dtc0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtc0 {
    ptr: *mut u8,
}
#[cfg(feature = "dtc0")]
pub const DTC0: self::Dtc0 = self::Dtc0 {
    ptr: 0x4000ac00u32 as _,
};
#[cfg(feature = "icu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icu {
    ptr: *mut u8,
}
#[cfg(feature = "icu")]
pub const ICU: self::Icu = self::Icu {
    ptr: 0x4000c000u32 as _,
};
#[cfg(feature = "cpu_ctrl")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpuCtrl {
    ptr: *mut u8,
}
#[cfg(feature = "cpu_ctrl")]
pub const CPU_CTRL: self::CpuCtrl = self::CpuCtrl {
    ptr: 0x4000f000u32 as _,
};
#[cfg(feature = "ocd_cpu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OcdCpu {
    ptr: *mut u8,
}
#[cfg(feature = "ocd_cpu")]
pub const OCD_CPU: self::OcdCpu = self::OcdCpu {
    ptr: 0x40011000u32 as _,
};
#[cfg(feature = "cpu_dbg")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpuDbg {
    ptr: *mut u8,
}
#[cfg(feature = "cpu_dbg")]
pub const CPU_DBG: self::CpuDbg = self::CpuDbg {
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
#[cfg(feature = "tsd")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsd {
    ptr: *mut u8,
}
#[cfg(feature = "tsd")]
pub const TSD: self::Tsd = self::Tsd {
    ptr: 0x4011b000u32 as _,
};
#[cfg(feature = "flad")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flad {
    ptr: *mut u8,
}
#[cfg(feature = "flad")]
pub const FLAD: self::Flad = self::Flad {
    ptr: 0x4011c000u32 as _,
};
#[cfg(feature = "faci")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Faci {
    ptr: *mut u8,
}
#[cfg(feature = "faci")]
pub const FACI: self::Faci = self::Faci {
    ptr: 0x4011e000u32 as _,
};
#[cfg(feature = "elc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elc {
    ptr: *mut u8,
}
#[cfg(feature = "elc")]
pub const ELC: self::Elc = self::Elc {
    ptr: 0x40201000u32 as _,
};
#[cfg(feature = "rtc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
#[cfg(feature = "rtc")]
pub const RTC: self::Rtc = self::Rtc {
    ptr: 0x40202000u32 as _,
};
#[cfg(feature = "iwdt")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdt {
    ptr: *mut u8,
}
#[cfg(feature = "iwdt")]
pub const IWDT: self::Iwdt = self::Iwdt {
    ptr: 0x40202200u32 as _,
};
#[cfg(feature = "cac")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cac {
    ptr: *mut u8,
}
#[cfg(feature = "cac")]
pub const CAC: self::Cac = self::Cac {
    ptr: 0x40202400u32 as _,
};
#[cfg(feature = "wdt0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt0 {
    ptr: *mut u8,
}
#[cfg(feature = "wdt0")]
pub const WDT0: self::Wdt0 = self::Wdt0 {
    ptr: 0x40202600u32 as _,
};
#[cfg(feature = "mstp")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstp {
    ptr: *mut u8,
}
#[cfg(feature = "mstp")]
pub const MSTP: self::Mstp = self::Mstp {
    ptr: 0x40203000u32 as _,
};
#[cfg(feature = "pscu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscu {
    ptr: *mut u8,
}
#[cfg(feature = "pscu")]
pub const PSCU: self::Pscu = self::Pscu {
    ptr: 0x40204000u32 as _,
};
#[cfg(feature = "poeg")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Poeg {
    ptr: *mut u8,
}
#[cfg(feature = "poeg")]
pub const POEG: self::Poeg = self::Poeg {
    ptr: 0x40212000u32 as _,
};
#[cfg(feature = "ulpt0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulpt0 {
    ptr: *mut u8,
}
#[cfg(feature = "ulpt0")]
pub const ULPT0: self::Ulpt0 = self::Ulpt0 {
    ptr: 0x40220000u32 as _,
};
#[cfg(feature = "ulpt1")]
pub const ULPT1: self::Ulpt0 = self::Ulpt0 {
    ptr: 0x40220100u32 as _,
};
#[cfg(feature = "agt0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agt0 {
    ptr: *mut u8,
}
#[cfg(feature = "agt0")]
pub const AGT0: self::Agt0 = self::Agt0 {
    ptr: 0x40221000u32 as _,
};
#[cfg(feature = "agt1")]
pub const AGT1: self::Agt0 = self::Agt0 {
    ptr: 0x40221100u32 as _,
};
#[cfg(feature = "tsn")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsn {
    ptr: *mut u8,
}
#[cfg(feature = "tsn")]
pub const TSN: self::Tsn = self::Tsn {
    ptr: 0x40235000u32 as _,
};
#[cfg(feature = "acmphs0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acmphs0 {
    ptr: *mut u8,
}
#[cfg(feature = "acmphs0")]
pub const ACMPHS0: self::Acmphs0 = self::Acmphs0 {
    ptr: 0x40236000u32 as _,
};
#[cfg(feature = "acmphs1")]
pub const ACMPHS1: self::Acmphs0 = self::Acmphs0 {
    ptr: 0x40236100u32 as _,
};
#[cfg(feature = "usbfs")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbfs {
    ptr: *mut u8,
}
#[cfg(feature = "usbfs")]
pub const USBFS: self::Usbfs = self::Usbfs {
    ptr: 0x40250000u32 as _,
};
#[cfg(feature = "ssie0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssie0 {
    ptr: *mut u8,
}
#[cfg(feature = "ssie0")]
pub const SSIE0: self::Ssie0 = self::Ssie0 {
    ptr: 0x4025d000u32 as _,
};
#[cfg(feature = "ssie1")]
pub const SSIE1: self::Ssie0 = self::Ssie0 {
    ptr: 0x4025d100u32 as _,
};
#[cfg(feature = "iic0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iic0 {
    ptr: *mut u8,
}
#[cfg(feature = "iic0")]
pub const IIC0: self::Iic0 = self::Iic0 {
    ptr: 0x4025e000u32 as _,
};
#[cfg(feature = "iic0wu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iic0Wu {
    ptr: *mut u8,
}
#[cfg(feature = "iic0wu")]
pub const IIC0WU: self::Iic0Wu = self::Iic0Wu {
    ptr: 0x4025e014u32 as _,
};
#[cfg(feature = "iic1")]
pub const IIC1: self::Iic0 = self::Iic0 {
    ptr: 0x4025e100u32 as _,
};
#[cfg(feature = "ospi0_b")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ospi0B {
    ptr: *mut u8,
}
#[cfg(feature = "ospi0_b")]
pub const OSPI0_B: self::Ospi0B = self::Ospi0B {
    ptr: 0x40268000u32 as _,
};
#[cfg(feature = "crc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crc {
    ptr: *mut u8,
}
#[cfg(feature = "crc")]
pub const CRC: self::Crc = self::Crc {
    ptr: 0x40310000u32 as _,
};
#[cfg(feature = "doc_b")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DocB {
    ptr: *mut u8,
}
#[cfg(feature = "doc_b")]
pub const DOC_B: self::DocB = self::DocB {
    ptr: 0x40311000u32 as _,
};
#[cfg(feature = "gpt320")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt320 {
    ptr: *mut u8,
}
#[cfg(feature = "gpt320")]
pub const GPT320: self::Gpt320 = self::Gpt320 {
    ptr: 0x40322000u32 as _,
};
#[cfg(feature = "gpt321")]
pub const GPT321: self::Gpt320 = self::Gpt320 {
    ptr: 0x40322100u32 as _,
};
#[cfg(feature = "gpt322")]
pub const GPT322: self::Gpt320 = self::Gpt320 {
    ptr: 0x40322200u32 as _,
};
#[cfg(feature = "gpt323")]
pub const GPT323: self::Gpt320 = self::Gpt320 {
    ptr: 0x40322300u32 as _,
};
#[cfg(feature = "gpt324")]
pub const GPT324: self::Gpt320 = self::Gpt320 {
    ptr: 0x40322400u32 as _,
};
#[cfg(feature = "gpt325")]
pub const GPT325: self::Gpt320 = self::Gpt320 {
    ptr: 0x40322500u32 as _,
};
#[cfg(feature = "gpt326")]
pub const GPT326: self::Gpt320 = self::Gpt320 {
    ptr: 0x40322600u32 as _,
};
#[cfg(feature = "gpt327")]
pub const GPT327: self::Gpt320 = self::Gpt320 {
    ptr: 0x40322700u32 as _,
};
#[cfg(feature = "gpt168")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt168 {
    ptr: *mut u8,
}
#[cfg(feature = "gpt168")]
pub const GPT168: self::Gpt168 = self::Gpt168 {
    ptr: 0x40322800u32 as _,
};
#[cfg(feature = "gpt169")]
pub const GPT169: self::Gpt168 = self::Gpt168 {
    ptr: 0x40322900u32 as _,
};
#[cfg(feature = "gpt1610")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt1610 {
    ptr: *mut u8,
}
#[cfg(feature = "gpt1610")]
pub const GPT1610: self::Gpt1610 = self::Gpt1610 {
    ptr: 0x40322a00u32 as _,
};
#[cfg(feature = "gpt1611")]
pub const GPT1611: self::Gpt1610 = self::Gpt1610 {
    ptr: 0x40322b00u32 as _,
};
#[cfg(feature = "gpt1612")]
pub const GPT1612: self::Gpt1610 = self::Gpt1610 {
    ptr: 0x40322c00u32 as _,
};
#[cfg(feature = "gpt1613")]
pub const GPT1613: self::Gpt1610 = self::Gpt1610 {
    ptr: 0x40322d00u32 as _,
};
#[cfg(feature = "adc120")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc120 {
    ptr: *mut u8,
}
#[cfg(feature = "adc120")]
pub const ADC120: self::Adc120 = self::Adc120 {
    ptr: 0x40332000u32 as _,
};
#[cfg(feature = "adc121")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc121 {
    ptr: *mut u8,
}
#[cfg(feature = "adc121")]
pub const ADC121: self::Adc121 = self::Adc121 {
    ptr: 0x40332200u32 as _,
};
#[cfg(feature = "dac12")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac12 {
    ptr: *mut u8,
}
#[cfg(feature = "dac12")]
pub const DAC12: self::Dac12 = self::Dac12 {
    ptr: 0x40333000u32 as _,
};
#[cfg(feature = "ceu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ceu {
    ptr: *mut u8,
}
#[cfg(feature = "ceu")]
pub const CEU: self::Ceu = self::Ceu {
    ptr: 0x40348000u32 as _,
};
#[cfg(feature = "edmac0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edmac0 {
    ptr: *mut u8,
}
#[cfg(feature = "edmac0")]
pub const EDMAC0: self::Edmac0 = self::Edmac0 {
    ptr: 0x40354000u32 as _,
};
#[cfg(feature = "etherc0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etherc0 {
    ptr: *mut u8,
}
#[cfg(feature = "etherc0")]
pub const ETHERC0: self::Etherc0 = self::Etherc0 {
    ptr: 0x40354100u32 as _,
};
#[cfg(feature = "sci0_b")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sci0B {
    ptr: *mut u8,
}
#[cfg(feature = "sci0_b")]
pub const SCI0_B: self::Sci0B = self::Sci0B {
    ptr: 0x40358000u32 as _,
};
#[cfg(feature = "sci1_b")]
pub const SCI1_B: self::Sci0B = self::Sci0B {
    ptr: 0x40358100u32 as _,
};
#[cfg(feature = "sci2_b")]
pub const SCI2_B: self::Sci0B = self::Sci0B {
    ptr: 0x40358200u32 as _,
};
#[cfg(feature = "sci3_b")]
pub const SCI3_B: self::Sci0B = self::Sci0B {
    ptr: 0x40358300u32 as _,
};
#[cfg(feature = "sci4_b")]
pub const SCI4_B: self::Sci0B = self::Sci0B {
    ptr: 0x40358400u32 as _,
};
#[cfg(feature = "sci9_b")]
pub const SCI9_B: self::Sci0B = self::Sci0B {
    ptr: 0x40358900u32 as _,
};
#[cfg(feature = "spi0_b")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi0B {
    ptr: *mut u8,
}
#[cfg(feature = "spi0_b")]
pub const SPI0_B: self::Spi0B = self::Spi0B {
    ptr: 0x4035c000u32 as _,
};
#[cfg(feature = "spi1_b")]
pub const SPI1_B: self::Spi0B = self::Spi0B {
    ptr: 0x4035c100u32 as _,
};
#[cfg(feature = "eccmb0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccmb0 {
    ptr: *mut u8,
}
#[cfg(feature = "eccmb0")]
pub const ECCMB0: self::Eccmb0 = self::Eccmb0 {
    ptr: 0x4036f200u32 as _,
};
#[cfg(feature = "eccmb1")]
pub const ECCMB1: self::Eccmb0 = self::Eccmb0 {
    ptr: 0x4036f300u32 as _,
};
#[cfg(feature = "canfd0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Canfd0 {
    ptr: *mut u8,
}
#[cfg(feature = "canfd0")]
pub const CANFD0: self::Canfd0 = self::Canfd0 {
    ptr: 0x40380000u32 as _,
};
#[cfg(feature = "canfd1")]
pub const CANFD1: self::Canfd0 = self::Canfd0 {
    ptr: 0x40382000u32 as _,
};
#[cfg(feature = "port0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port0 {
    ptr: *mut u8,
}
#[cfg(feature = "port0")]
pub const PORT0: self::Port0 = self::Port0 {
    ptr: 0x40400000u32 as _,
};
#[cfg(feature = "port1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port1 {
    ptr: *mut u8,
}
#[cfg(feature = "port1")]
pub const PORT1: self::Port1 = self::Port1 {
    ptr: 0x40400020u32 as _,
};
#[cfg(feature = "port2")]
pub const PORT2: self::Port1 = self::Port1 {
    ptr: 0x40400040u32 as _,
};
#[cfg(feature = "port3")]
pub const PORT3: self::Port1 = self::Port1 {
    ptr: 0x40400060u32 as _,
};
#[cfg(feature = "port4")]
pub const PORT4: self::Port1 = self::Port1 {
    ptr: 0x40400080u32 as _,
};
#[cfg(feature = "port5")]
pub const PORT5: self::Port0 = self::Port0 {
    ptr: 0x404000a0u32 as _,
};
#[cfg(feature = "port6")]
pub const PORT6: self::Port0 = self::Port0 {
    ptr: 0x404000c0u32 as _,
};
#[cfg(feature = "port7")]
pub const PORT7: self::Port0 = self::Port0 {
    ptr: 0x404000e0u32 as _,
};
#[cfg(feature = "port8")]
pub const PORT8: self::Port0 = self::Port0 {
    ptr: 0x40400100u32 as _,
};
#[cfg(feature = "port9")]
pub const PORT9: self::Port0 = self::Port0 {
    ptr: 0x40400120u32 as _,
};
#[cfg(feature = "porta")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Porta {
    ptr: *mut u8,
}
#[cfg(feature = "porta")]
pub const PORTA: self::Porta = self::Porta {
    ptr: 0x40400140u32 as _,
};
#[cfg(feature = "portb")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Portb {
    ptr: *mut u8,
}
#[cfg(feature = "portb")]
pub const PORTB: self::Portb = self::Portb {
    ptr: 0x40400160u32 as _,
};
#[cfg(feature = "pfs")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfs {
    ptr: *mut u8,
}
#[cfg(feature = "pfs")]
pub const PFS: self::Pfs = self::Pfs {
    ptr: 0x40400800u32 as _,
};
#[cfg(feature = "rmpu_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmpuNs {
    ptr: *mut u8,
}
#[cfg(feature = "rmpu_ns")]
pub const RMPU_NS: self::RmpuNs = self::RmpuNs {
    ptr: 0x50000000u32 as _,
};
#[cfg(feature = "sram_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramNs {
    ptr: *mut u8,
}
#[cfg(feature = "sram_ns")]
pub const SRAM_NS: self::SramNs = self::SramNs {
    ptr: 0x50002000u32 as _,
};
#[cfg(feature = "bus_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BusNs {
    ptr: *mut u8,
}
#[cfg(feature = "bus_ns")]
pub const BUS_NS: self::BusNs = self::BusNs {
    ptr: 0x50003000u32 as _,
};
#[cfg(feature = "icu_common_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IcuCommonNs {
    ptr: *mut u8,
}
#[cfg(feature = "icu_common_ns")]
pub const ICU_COMMON_NS: self::IcuCommonNs = self::IcuCommonNs {
    ptr: 0x50006000u32 as _,
};
#[cfg(feature = "cpscu_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpscuNs {
    ptr: *mut u8,
}
#[cfg(feature = "cpscu_ns")]
pub const CPSCU_NS: self::CpscuNs = self::CpscuNs {
    ptr: 0x50008000u32 as _,
};
#[cfg(feature = "dmac00_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac00Ns {
    ptr: *mut u8,
}
#[cfg(feature = "dmac00_ns")]
pub const DMAC00_NS: self::Dmac00Ns = self::Dmac00Ns {
    ptr: 0x5000a000u32 as _,
};
#[cfg(feature = "dmac01_ns")]
pub const DMAC01_NS: self::Dmac00Ns = self::Dmac00Ns {
    ptr: 0x5000a040u32 as _,
};
#[cfg(feature = "dmac02_ns")]
pub const DMAC02_NS: self::Dmac00Ns = self::Dmac00Ns {
    ptr: 0x5000a080u32 as _,
};
#[cfg(feature = "dmac03_ns")]
pub const DMAC03_NS: self::Dmac00Ns = self::Dmac00Ns {
    ptr: 0x5000a0c0u32 as _,
};
#[cfg(feature = "dmac04_ns")]
pub const DMAC04_NS: self::Dmac00Ns = self::Dmac00Ns {
    ptr: 0x5000a100u32 as _,
};
#[cfg(feature = "dmac05_ns")]
pub const DMAC05_NS: self::Dmac00Ns = self::Dmac00Ns {
    ptr: 0x5000a140u32 as _,
};
#[cfg(feature = "dmac06_ns")]
pub const DMAC06_NS: self::Dmac00Ns = self::Dmac00Ns {
    ptr: 0x5000a180u32 as _,
};
#[cfg(feature = "dmac07_ns")]
pub const DMAC07_NS: self::Dmac00Ns = self::Dmac00Ns {
    ptr: 0x5000a1c0u32 as _,
};
#[cfg(feature = "dma0_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0Ns {
    ptr: *mut u8,
}
#[cfg(feature = "dma0_ns")]
pub const DMA0_NS: self::Dma0Ns = self::Dma0Ns {
    ptr: 0x5000a800u32 as _,
};
#[cfg(feature = "dtc0_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtc0Ns {
    ptr: *mut u8,
}
#[cfg(feature = "dtc0_ns")]
pub const DTC0_NS: self::Dtc0Ns = self::Dtc0Ns {
    ptr: 0x5000ac00u32 as _,
};
#[cfg(feature = "icu_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IcuNs {
    ptr: *mut u8,
}
#[cfg(feature = "icu_ns")]
pub const ICU_NS: self::IcuNs = self::IcuNs {
    ptr: 0x5000c000u32 as _,
};
#[cfg(feature = "cpu_ctrl_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpuCtrlNs {
    ptr: *mut u8,
}
#[cfg(feature = "cpu_ctrl_ns")]
pub const CPU_CTRL_NS: self::CpuCtrlNs = self::CpuCtrlNs {
    ptr: 0x5000f000u32 as _,
};
#[cfg(feature = "ocd_cpu_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OcdCpuNs {
    ptr: *mut u8,
}
#[cfg(feature = "ocd_cpu_ns")]
pub const OCD_CPU_NS: self::OcdCpuNs = self::OcdCpuNs {
    ptr: 0x50011000u32 as _,
};
#[cfg(feature = "cpu_dbg_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpuDbgNs {
    ptr: *mut u8,
}
#[cfg(feature = "cpu_dbg_ns")]
pub const CPU_DBG_NS: self::CpuDbgNs = self::CpuDbgNs {
    ptr: 0x5001b000u32 as _,
};
#[cfg(feature = "fcache_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FcacheNs {
    ptr: *mut u8,
}
#[cfg(feature = "fcache_ns")]
pub const FCACHE_NS: self::FcacheNs = self::FcacheNs {
    ptr: 0x5001c100u32 as _,
};
#[cfg(feature = "sysc_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscNs {
    ptr: *mut u8,
}
#[cfg(feature = "sysc_ns")]
pub const SYSC_NS: self::SyscNs = self::SyscNs {
    ptr: 0x5001e000u32 as _,
};
#[cfg(feature = "tsd_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TsdNs {
    ptr: *mut u8,
}
#[cfg(feature = "tsd_ns")]
pub const TSD_NS: self::TsdNs = self::TsdNs {
    ptr: 0x5011b000u32 as _,
};
#[cfg(feature = "flad_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FladNs {
    ptr: *mut u8,
}
#[cfg(feature = "flad_ns")]
pub const FLAD_NS: self::FladNs = self::FladNs {
    ptr: 0x5011c000u32 as _,
};
#[cfg(feature = "faci_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FaciNs {
    ptr: *mut u8,
}
#[cfg(feature = "faci_ns")]
pub const FACI_NS: self::FaciNs = self::FaciNs {
    ptr: 0x5011e000u32 as _,
};
#[cfg(feature = "elc_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElcNs {
    ptr: *mut u8,
}
#[cfg(feature = "elc_ns")]
pub const ELC_NS: self::ElcNs = self::ElcNs {
    ptr: 0x50201000u32 as _,
};
#[cfg(feature = "rtc_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcNs {
    ptr: *mut u8,
}
#[cfg(feature = "rtc_ns")]
pub const RTC_NS: self::RtcNs = self::RtcNs {
    ptr: 0x50202000u32 as _,
};
#[cfg(feature = "iwdt_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IwdtNs {
    ptr: *mut u8,
}
#[cfg(feature = "iwdt_ns")]
pub const IWDT_NS: self::IwdtNs = self::IwdtNs {
    ptr: 0x50202200u32 as _,
};
#[cfg(feature = "cac_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacNs {
    ptr: *mut u8,
}
#[cfg(feature = "cac_ns")]
pub const CAC_NS: self::CacNs = self::CacNs {
    ptr: 0x50202400u32 as _,
};
#[cfg(feature = "wdt0_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt0Ns {
    ptr: *mut u8,
}
#[cfg(feature = "wdt0_ns")]
pub const WDT0_NS: self::Wdt0Ns = self::Wdt0Ns {
    ptr: 0x50202600u32 as _,
};
#[cfg(feature = "mstp_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MstpNs {
    ptr: *mut u8,
}
#[cfg(feature = "mstp_ns")]
pub const MSTP_NS: self::MstpNs = self::MstpNs {
    ptr: 0x50203000u32 as _,
};
#[cfg(feature = "pscu_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PscuNs {
    ptr: *mut u8,
}
#[cfg(feature = "pscu_ns")]
pub const PSCU_NS: self::PscuNs = self::PscuNs {
    ptr: 0x50204000u32 as _,
};
#[cfg(feature = "poeg_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PoegNs {
    ptr: *mut u8,
}
#[cfg(feature = "poeg_ns")]
pub const POEG_NS: self::PoegNs = self::PoegNs {
    ptr: 0x50212000u32 as _,
};
#[cfg(feature = "ulpt0_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulpt0Ns {
    ptr: *mut u8,
}
#[cfg(feature = "ulpt0_ns")]
pub const ULPT0_NS: self::Ulpt0Ns = self::Ulpt0Ns {
    ptr: 0x50220000u32 as _,
};
#[cfg(feature = "ulpt1_ns")]
pub const ULPT1_NS: self::Ulpt0Ns = self::Ulpt0Ns {
    ptr: 0x50220100u32 as _,
};
#[cfg(feature = "agt0_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agt0Ns {
    ptr: *mut u8,
}
#[cfg(feature = "agt0_ns")]
pub const AGT0_NS: self::Agt0Ns = self::Agt0Ns {
    ptr: 0x50221000u32 as _,
};
#[cfg(feature = "agt1_ns")]
pub const AGT1_NS: self::Agt0Ns = self::Agt0Ns {
    ptr: 0x50221100u32 as _,
};
#[cfg(feature = "tsn_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TsnNs {
    ptr: *mut u8,
}
#[cfg(feature = "tsn_ns")]
pub const TSN_NS: self::TsnNs = self::TsnNs {
    ptr: 0x50235000u32 as _,
};
#[cfg(feature = "acmphs0_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acmphs0Ns {
    ptr: *mut u8,
}
#[cfg(feature = "acmphs0_ns")]
pub const ACMPHS0_NS: self::Acmphs0Ns = self::Acmphs0Ns {
    ptr: 0x50236000u32 as _,
};
#[cfg(feature = "acmphs1_ns")]
pub const ACMPHS1_NS: self::Acmphs0Ns = self::Acmphs0Ns {
    ptr: 0x50236100u32 as _,
};
#[cfg(feature = "usbfs_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbfsNs {
    ptr: *mut u8,
}
#[cfg(feature = "usbfs_ns")]
pub const USBFS_NS: self::UsbfsNs = self::UsbfsNs {
    ptr: 0x50250000u32 as _,
};
#[cfg(feature = "ssie0_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssie0Ns {
    ptr: *mut u8,
}
#[cfg(feature = "ssie0_ns")]
pub const SSIE0_NS: self::Ssie0Ns = self::Ssie0Ns {
    ptr: 0x5025d000u32 as _,
};
#[cfg(feature = "ssie1_ns")]
pub const SSIE1_NS: self::Ssie0Ns = self::Ssie0Ns {
    ptr: 0x5025d100u32 as _,
};
#[cfg(feature = "iic0_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iic0Ns {
    ptr: *mut u8,
}
#[cfg(feature = "iic0_ns")]
pub const IIC0_NS: self::Iic0Ns = self::Iic0Ns {
    ptr: 0x5025e000u32 as _,
};
#[cfg(feature = "iic0wu_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iic0WuNs {
    ptr: *mut u8,
}
#[cfg(feature = "iic0wu_ns")]
pub const IIC0WU_NS: self::Iic0WuNs = self::Iic0WuNs {
    ptr: 0x5025e014u32 as _,
};
#[cfg(feature = "iic1_ns")]
pub const IIC1_NS: self::Iic0Ns = self::Iic0Ns {
    ptr: 0x5025e100u32 as _,
};
#[cfg(feature = "ospi0_b_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ospi0BNs {
    ptr: *mut u8,
}
#[cfg(feature = "ospi0_b_ns")]
pub const OSPI0_B_NS: self::Ospi0BNs = self::Ospi0BNs {
    ptr: 0x50268000u32 as _,
};
#[cfg(feature = "crc_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrcNs {
    ptr: *mut u8,
}
#[cfg(feature = "crc_ns")]
pub const CRC_NS: self::CrcNs = self::CrcNs {
    ptr: 0x50310000u32 as _,
};
#[cfg(feature = "doc_b_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DocBNs {
    ptr: *mut u8,
}
#[cfg(feature = "doc_b_ns")]
pub const DOC_B_NS: self::DocBNs = self::DocBNs {
    ptr: 0x50311000u32 as _,
};
#[cfg(feature = "gpt320_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt320Ns {
    ptr: *mut u8,
}
#[cfg(feature = "gpt320_ns")]
pub const GPT320_NS: self::Gpt320Ns = self::Gpt320Ns {
    ptr: 0x50322000u32 as _,
};
#[cfg(feature = "gpt321_ns")]
pub const GPT321_NS: self::Gpt320Ns = self::Gpt320Ns {
    ptr: 0x50322100u32 as _,
};
#[cfg(feature = "gpt322_ns")]
pub const GPT322_NS: self::Gpt320Ns = self::Gpt320Ns {
    ptr: 0x50322200u32 as _,
};
#[cfg(feature = "gpt323_ns")]
pub const GPT323_NS: self::Gpt320Ns = self::Gpt320Ns {
    ptr: 0x50322300u32 as _,
};
#[cfg(feature = "gpt324_ns")]
pub const GPT324_NS: self::Gpt320Ns = self::Gpt320Ns {
    ptr: 0x50322400u32 as _,
};
#[cfg(feature = "gpt325_ns")]
pub const GPT325_NS: self::Gpt320Ns = self::Gpt320Ns {
    ptr: 0x50322500u32 as _,
};
#[cfg(feature = "gpt326_ns")]
pub const GPT326_NS: self::Gpt320Ns = self::Gpt320Ns {
    ptr: 0x50322600u32 as _,
};
#[cfg(feature = "gpt327_ns")]
pub const GPT327_NS: self::Gpt320Ns = self::Gpt320Ns {
    ptr: 0x50322700u32 as _,
};
#[cfg(feature = "gpt168_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt168Ns {
    ptr: *mut u8,
}
#[cfg(feature = "gpt168_ns")]
pub const GPT168_NS: self::Gpt168Ns = self::Gpt168Ns {
    ptr: 0x50322800u32 as _,
};
#[cfg(feature = "gpt169_ns")]
pub const GPT169_NS: self::Gpt168Ns = self::Gpt168Ns {
    ptr: 0x50322900u32 as _,
};
#[cfg(feature = "gpt1610_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt1610Ns {
    ptr: *mut u8,
}
#[cfg(feature = "gpt1610_ns")]
pub const GPT1610_NS: self::Gpt1610Ns = self::Gpt1610Ns {
    ptr: 0x50322a00u32 as _,
};
#[cfg(feature = "gpt1611_ns")]
pub const GPT1611_NS: self::Gpt1610Ns = self::Gpt1610Ns {
    ptr: 0x50322b00u32 as _,
};
#[cfg(feature = "gpt1612_ns")]
pub const GPT1612_NS: self::Gpt1610Ns = self::Gpt1610Ns {
    ptr: 0x50322c00u32 as _,
};
#[cfg(feature = "gpt1613_ns")]
pub const GPT1613_NS: self::Gpt1610Ns = self::Gpt1610Ns {
    ptr: 0x50322d00u32 as _,
};
#[cfg(feature = "adc120_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc120Ns {
    ptr: *mut u8,
}
#[cfg(feature = "adc120_ns")]
pub const ADC120_NS: self::Adc120Ns = self::Adc120Ns {
    ptr: 0x50332000u32 as _,
};
#[cfg(feature = "adc121_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc121Ns {
    ptr: *mut u8,
}
#[cfg(feature = "adc121_ns")]
pub const ADC121_NS: self::Adc121Ns = self::Adc121Ns {
    ptr: 0x50332200u32 as _,
};
#[cfg(feature = "dac12_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac12Ns {
    ptr: *mut u8,
}
#[cfg(feature = "dac12_ns")]
pub const DAC12_NS: self::Dac12Ns = self::Dac12Ns {
    ptr: 0x50333000u32 as _,
};
#[cfg(feature = "ceu_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CeuNs {
    ptr: *mut u8,
}
#[cfg(feature = "ceu_ns")]
pub const CEU_NS: self::CeuNs = self::CeuNs {
    ptr: 0x50348000u32 as _,
};
#[cfg(feature = "edmac0_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edmac0Ns {
    ptr: *mut u8,
}
#[cfg(feature = "edmac0_ns")]
pub const EDMAC0_NS: self::Edmac0Ns = self::Edmac0Ns {
    ptr: 0x50354000u32 as _,
};
#[cfg(feature = "etherc0_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etherc0Ns {
    ptr: *mut u8,
}
#[cfg(feature = "etherc0_ns")]
pub const ETHERC0_NS: self::Etherc0Ns = self::Etherc0Ns {
    ptr: 0x50354100u32 as _,
};
#[cfg(feature = "sci0_b_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sci0BNs {
    ptr: *mut u8,
}
#[cfg(feature = "sci0_b_ns")]
pub const SCI0_B_NS: self::Sci0BNs = self::Sci0BNs {
    ptr: 0x50358000u32 as _,
};
#[cfg(feature = "sci1_b_ns")]
pub const SCI1_B_NS: self::Sci0BNs = self::Sci0BNs {
    ptr: 0x50358100u32 as _,
};
#[cfg(feature = "sci2_b_ns")]
pub const SCI2_B_NS: self::Sci0BNs = self::Sci0BNs {
    ptr: 0x50358200u32 as _,
};
#[cfg(feature = "sci3_b_ns")]
pub const SCI3_B_NS: self::Sci0BNs = self::Sci0BNs {
    ptr: 0x50358300u32 as _,
};
#[cfg(feature = "sci4_b_ns")]
pub const SCI4_B_NS: self::Sci0BNs = self::Sci0BNs {
    ptr: 0x50358400u32 as _,
};
#[cfg(feature = "sci9_b_ns")]
pub const SCI9_B_NS: self::Sci0BNs = self::Sci0BNs {
    ptr: 0x50358900u32 as _,
};
#[cfg(feature = "spi0_b_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi0BNs {
    ptr: *mut u8,
}
#[cfg(feature = "spi0_b_ns")]
pub const SPI0_B_NS: self::Spi0BNs = self::Spi0BNs {
    ptr: 0x5035c000u32 as _,
};
#[cfg(feature = "spi1_b_ns")]
pub const SPI1_B_NS: self::Spi0BNs = self::Spi0BNs {
    ptr: 0x5035c100u32 as _,
};
#[cfg(feature = "eccmb0_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccmb0Ns {
    ptr: *mut u8,
}
#[cfg(feature = "eccmb0_ns")]
pub const ECCMB0_NS: self::Eccmb0Ns = self::Eccmb0Ns {
    ptr: 0x5036f200u32 as _,
};
#[cfg(feature = "eccmb1_ns")]
pub const ECCMB1_NS: self::Eccmb0Ns = self::Eccmb0Ns {
    ptr: 0x5036f300u32 as _,
};
#[cfg(feature = "canfd0_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Canfd0Ns {
    ptr: *mut u8,
}
#[cfg(feature = "canfd0_ns")]
pub const CANFD0_NS: self::Canfd0Ns = self::Canfd0Ns {
    ptr: 0x50380000u32 as _,
};
#[cfg(feature = "canfd1_ns")]
pub const CANFD1_NS: self::Canfd0Ns = self::Canfd0Ns {
    ptr: 0x50382000u32 as _,
};
#[cfg(feature = "port0_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port0Ns {
    ptr: *mut u8,
}
#[cfg(feature = "port0_ns")]
pub const PORT0_NS: self::Port0Ns = self::Port0Ns {
    ptr: 0x50400000u32 as _,
};
#[cfg(feature = "port1_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port1Ns {
    ptr: *mut u8,
}
#[cfg(feature = "port1_ns")]
pub const PORT1_NS: self::Port1Ns = self::Port1Ns {
    ptr: 0x50400020u32 as _,
};
#[cfg(feature = "port2_ns")]
pub const PORT2_NS: self::Port1Ns = self::Port1Ns {
    ptr: 0x50400040u32 as _,
};
#[cfg(feature = "port3_ns")]
pub const PORT3_NS: self::Port1Ns = self::Port1Ns {
    ptr: 0x50400060u32 as _,
};
#[cfg(feature = "port4_ns")]
pub const PORT4_NS: self::Port1Ns = self::Port1Ns {
    ptr: 0x50400080u32 as _,
};
#[cfg(feature = "port5_ns")]
pub const PORT5_NS: self::Port0Ns = self::Port0Ns {
    ptr: 0x504000a0u32 as _,
};
#[cfg(feature = "port6_ns")]
pub const PORT6_NS: self::Port0Ns = self::Port0Ns {
    ptr: 0x504000c0u32 as _,
};
#[cfg(feature = "port7_ns")]
pub const PORT7_NS: self::Port0Ns = self::Port0Ns {
    ptr: 0x504000e0u32 as _,
};
#[cfg(feature = "port8_ns")]
pub const PORT8_NS: self::Port0Ns = self::Port0Ns {
    ptr: 0x50400100u32 as _,
};
#[cfg(feature = "port9_ns")]
pub const PORT9_NS: self::Port0Ns = self::Port0Ns {
    ptr: 0x50400120u32 as _,
};
#[cfg(feature = "porta_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PortaNs {
    ptr: *mut u8,
}
#[cfg(feature = "porta_ns")]
pub const PORTA_NS: self::PortaNs = self::PortaNs {
    ptr: 0x50400140u32 as _,
};
#[cfg(feature = "portb_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PortbNs {
    ptr: *mut u8,
}
#[cfg(feature = "portb_ns")]
pub const PORTB_NS: self::PortbNs = self::PortbNs {
    ptr: 0x50400160u32 as _,
};
#[cfg(feature = "pfs_ns")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PfsNs {
    ptr: *mut u8,
}
#[cfg(feature = "pfs_ns")]
pub const PFS_NS: self::PfsNs = self::PfsNs {
    ptr: 0x50400800u32 as _,
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
    #[cfg(feature = "sram")]
    pub SRAM: self::Sram,
    #[cfg(feature = "bus")]
    pub BUS: self::Bus,
    #[cfg(feature = "icu_common")]
    pub ICU_COMMON: self::IcuCommon,
    #[cfg(feature = "cpscu")]
    pub CPSCU: self::Cpscu,
    #[cfg(feature = "dmac00")]
    pub DMAC00: self::Dmac00,
    #[cfg(feature = "dmac01")]
    pub DMAC01: self::Dmac00,
    #[cfg(feature = "dmac02")]
    pub DMAC02: self::Dmac00,
    #[cfg(feature = "dmac03")]
    pub DMAC03: self::Dmac00,
    #[cfg(feature = "dmac04")]
    pub DMAC04: self::Dmac00,
    #[cfg(feature = "dmac05")]
    pub DMAC05: self::Dmac00,
    #[cfg(feature = "dmac06")]
    pub DMAC06: self::Dmac00,
    #[cfg(feature = "dmac07")]
    pub DMAC07: self::Dmac00,
    #[cfg(feature = "dma0")]
    pub DMA0: self::Dma0,
    #[cfg(feature = "dtc0")]
    pub DTC0: self::Dtc0,
    #[cfg(feature = "icu")]
    pub ICU: self::Icu,
    #[cfg(feature = "cpu_ctrl")]
    pub CPU_CTRL: self::CpuCtrl,
    #[cfg(feature = "ocd_cpu")]
    pub OCD_CPU: self::OcdCpu,
    #[cfg(feature = "cpu_dbg")]
    pub CPU_DBG: self::CpuDbg,
    #[cfg(feature = "fcache")]
    pub FCACHE: self::Fcache,
    #[cfg(feature = "sysc")]
    pub SYSC: self::Sysc,
    #[cfg(feature = "tsd")]
    pub TSD: self::Tsd,
    #[cfg(feature = "flad")]
    pub FLAD: self::Flad,
    #[cfg(feature = "faci")]
    pub FACI: self::Faci,
    #[cfg(feature = "elc")]
    pub ELC: self::Elc,
    #[cfg(feature = "rtc")]
    pub RTC: self::Rtc,
    #[cfg(feature = "iwdt")]
    pub IWDT: self::Iwdt,
    #[cfg(feature = "cac")]
    pub CAC: self::Cac,
    #[cfg(feature = "wdt0")]
    pub WDT0: self::Wdt0,
    #[cfg(feature = "mstp")]
    pub MSTP: self::Mstp,
    #[cfg(feature = "pscu")]
    pub PSCU: self::Pscu,
    #[cfg(feature = "poeg")]
    pub POEG: self::Poeg,
    #[cfg(feature = "ulpt0")]
    pub ULPT0: self::Ulpt0,
    #[cfg(feature = "ulpt1")]
    pub ULPT1: self::Ulpt0,
    #[cfg(feature = "agt0")]
    pub AGT0: self::Agt0,
    #[cfg(feature = "agt1")]
    pub AGT1: self::Agt0,
    #[cfg(feature = "tsn")]
    pub TSN: self::Tsn,
    #[cfg(feature = "acmphs0")]
    pub ACMPHS0: self::Acmphs0,
    #[cfg(feature = "acmphs1")]
    pub ACMPHS1: self::Acmphs0,
    #[cfg(feature = "usbfs")]
    pub USBFS: self::Usbfs,
    #[cfg(feature = "ssie0")]
    pub SSIE0: self::Ssie0,
    #[cfg(feature = "ssie1")]
    pub SSIE1: self::Ssie0,
    #[cfg(feature = "iic0")]
    pub IIC0: self::Iic0,
    #[cfg(feature = "iic0wu")]
    pub IIC0WU: self::Iic0Wu,
    #[cfg(feature = "iic1")]
    pub IIC1: self::Iic0,
    #[cfg(feature = "ospi0_b")]
    pub OSPI0_B: self::Ospi0B,
    #[cfg(feature = "crc")]
    pub CRC: self::Crc,
    #[cfg(feature = "doc_b")]
    pub DOC_B: self::DocB,
    #[cfg(feature = "gpt320")]
    pub GPT320: self::Gpt320,
    #[cfg(feature = "gpt321")]
    pub GPT321: self::Gpt320,
    #[cfg(feature = "gpt322")]
    pub GPT322: self::Gpt320,
    #[cfg(feature = "gpt323")]
    pub GPT323: self::Gpt320,
    #[cfg(feature = "gpt324")]
    pub GPT324: self::Gpt320,
    #[cfg(feature = "gpt325")]
    pub GPT325: self::Gpt320,
    #[cfg(feature = "gpt326")]
    pub GPT326: self::Gpt320,
    #[cfg(feature = "gpt327")]
    pub GPT327: self::Gpt320,
    #[cfg(feature = "gpt168")]
    pub GPT168: self::Gpt168,
    #[cfg(feature = "gpt169")]
    pub GPT169: self::Gpt168,
    #[cfg(feature = "gpt1610")]
    pub GPT1610: self::Gpt1610,
    #[cfg(feature = "gpt1611")]
    pub GPT1611: self::Gpt1610,
    #[cfg(feature = "gpt1612")]
    pub GPT1612: self::Gpt1610,
    #[cfg(feature = "gpt1613")]
    pub GPT1613: self::Gpt1610,
    #[cfg(feature = "adc120")]
    pub ADC120: self::Adc120,
    #[cfg(feature = "adc121")]
    pub ADC121: self::Adc121,
    #[cfg(feature = "dac12")]
    pub DAC12: self::Dac12,
    #[cfg(feature = "ceu")]
    pub CEU: self::Ceu,
    #[cfg(feature = "edmac0")]
    pub EDMAC0: self::Edmac0,
    #[cfg(feature = "etherc0")]
    pub ETHERC0: self::Etherc0,
    #[cfg(feature = "sci0_b")]
    pub SCI0_B: self::Sci0B,
    #[cfg(feature = "sci1_b")]
    pub SCI1_B: self::Sci0B,
    #[cfg(feature = "sci2_b")]
    pub SCI2_B: self::Sci0B,
    #[cfg(feature = "sci3_b")]
    pub SCI3_B: self::Sci0B,
    #[cfg(feature = "sci4_b")]
    pub SCI4_B: self::Sci0B,
    #[cfg(feature = "sci9_b")]
    pub SCI9_B: self::Sci0B,
    #[cfg(feature = "spi0_b")]
    pub SPI0_B: self::Spi0B,
    #[cfg(feature = "spi1_b")]
    pub SPI1_B: self::Spi0B,
    #[cfg(feature = "eccmb0")]
    pub ECCMB0: self::Eccmb0,
    #[cfg(feature = "eccmb1")]
    pub ECCMB1: self::Eccmb0,
    #[cfg(feature = "canfd0")]
    pub CANFD0: self::Canfd0,
    #[cfg(feature = "canfd1")]
    pub CANFD1: self::Canfd0,
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
    #[cfg(feature = "port8")]
    pub PORT8: self::Port0,
    #[cfg(feature = "port9")]
    pub PORT9: self::Port0,
    #[cfg(feature = "porta")]
    pub PORTA: self::Porta,
    #[cfg(feature = "portb")]
    pub PORTB: self::Portb,
    #[cfg(feature = "pfs")]
    pub PFS: self::Pfs,
    #[cfg(feature = "rmpu_ns")]
    pub RMPU_NS: self::RmpuNs,
    #[cfg(feature = "sram_ns")]
    pub SRAM_NS: self::SramNs,
    #[cfg(feature = "bus_ns")]
    pub BUS_NS: self::BusNs,
    #[cfg(feature = "icu_common_ns")]
    pub ICU_COMMON_NS: self::IcuCommonNs,
    #[cfg(feature = "cpscu_ns")]
    pub CPSCU_NS: self::CpscuNs,
    #[cfg(feature = "dmac00_ns")]
    pub DMAC00_NS: self::Dmac00Ns,
    #[cfg(feature = "dmac01_ns")]
    pub DMAC01_NS: self::Dmac00Ns,
    #[cfg(feature = "dmac02_ns")]
    pub DMAC02_NS: self::Dmac00Ns,
    #[cfg(feature = "dmac03_ns")]
    pub DMAC03_NS: self::Dmac00Ns,
    #[cfg(feature = "dmac04_ns")]
    pub DMAC04_NS: self::Dmac00Ns,
    #[cfg(feature = "dmac05_ns")]
    pub DMAC05_NS: self::Dmac00Ns,
    #[cfg(feature = "dmac06_ns")]
    pub DMAC06_NS: self::Dmac00Ns,
    #[cfg(feature = "dmac07_ns")]
    pub DMAC07_NS: self::Dmac00Ns,
    #[cfg(feature = "dma0_ns")]
    pub DMA0_NS: self::Dma0Ns,
    #[cfg(feature = "dtc0_ns")]
    pub DTC0_NS: self::Dtc0Ns,
    #[cfg(feature = "icu_ns")]
    pub ICU_NS: self::IcuNs,
    #[cfg(feature = "cpu_ctrl_ns")]
    pub CPU_CTRL_NS: self::CpuCtrlNs,
    #[cfg(feature = "ocd_cpu_ns")]
    pub OCD_CPU_NS: self::OcdCpuNs,
    #[cfg(feature = "cpu_dbg_ns")]
    pub CPU_DBG_NS: self::CpuDbgNs,
    #[cfg(feature = "fcache_ns")]
    pub FCACHE_NS: self::FcacheNs,
    #[cfg(feature = "sysc_ns")]
    pub SYSC_NS: self::SyscNs,
    #[cfg(feature = "tsd_ns")]
    pub TSD_NS: self::TsdNs,
    #[cfg(feature = "flad_ns")]
    pub FLAD_NS: self::FladNs,
    #[cfg(feature = "faci_ns")]
    pub FACI_NS: self::FaciNs,
    #[cfg(feature = "elc_ns")]
    pub ELC_NS: self::ElcNs,
    #[cfg(feature = "rtc_ns")]
    pub RTC_NS: self::RtcNs,
    #[cfg(feature = "iwdt_ns")]
    pub IWDT_NS: self::IwdtNs,
    #[cfg(feature = "cac_ns")]
    pub CAC_NS: self::CacNs,
    #[cfg(feature = "wdt0_ns")]
    pub WDT0_NS: self::Wdt0Ns,
    #[cfg(feature = "mstp_ns")]
    pub MSTP_NS: self::MstpNs,
    #[cfg(feature = "pscu_ns")]
    pub PSCU_NS: self::PscuNs,
    #[cfg(feature = "poeg_ns")]
    pub POEG_NS: self::PoegNs,
    #[cfg(feature = "ulpt0_ns")]
    pub ULPT0_NS: self::Ulpt0Ns,
    #[cfg(feature = "ulpt1_ns")]
    pub ULPT1_NS: self::Ulpt0Ns,
    #[cfg(feature = "agt0_ns")]
    pub AGT0_NS: self::Agt0Ns,
    #[cfg(feature = "agt1_ns")]
    pub AGT1_NS: self::Agt0Ns,
    #[cfg(feature = "tsn_ns")]
    pub TSN_NS: self::TsnNs,
    #[cfg(feature = "acmphs0_ns")]
    pub ACMPHS0_NS: self::Acmphs0Ns,
    #[cfg(feature = "acmphs1_ns")]
    pub ACMPHS1_NS: self::Acmphs0Ns,
    #[cfg(feature = "usbfs_ns")]
    pub USBFS_NS: self::UsbfsNs,
    #[cfg(feature = "ssie0_ns")]
    pub SSIE0_NS: self::Ssie0Ns,
    #[cfg(feature = "ssie1_ns")]
    pub SSIE1_NS: self::Ssie0Ns,
    #[cfg(feature = "iic0_ns")]
    pub IIC0_NS: self::Iic0Ns,
    #[cfg(feature = "iic0wu_ns")]
    pub IIC0WU_NS: self::Iic0WuNs,
    #[cfg(feature = "iic1_ns")]
    pub IIC1_NS: self::Iic0Ns,
    #[cfg(feature = "ospi0_b_ns")]
    pub OSPI0_B_NS: self::Ospi0BNs,
    #[cfg(feature = "crc_ns")]
    pub CRC_NS: self::CrcNs,
    #[cfg(feature = "doc_b_ns")]
    pub DOC_B_NS: self::DocBNs,
    #[cfg(feature = "gpt320_ns")]
    pub GPT320_NS: self::Gpt320Ns,
    #[cfg(feature = "gpt321_ns")]
    pub GPT321_NS: self::Gpt320Ns,
    #[cfg(feature = "gpt322_ns")]
    pub GPT322_NS: self::Gpt320Ns,
    #[cfg(feature = "gpt323_ns")]
    pub GPT323_NS: self::Gpt320Ns,
    #[cfg(feature = "gpt324_ns")]
    pub GPT324_NS: self::Gpt320Ns,
    #[cfg(feature = "gpt325_ns")]
    pub GPT325_NS: self::Gpt320Ns,
    #[cfg(feature = "gpt326_ns")]
    pub GPT326_NS: self::Gpt320Ns,
    #[cfg(feature = "gpt327_ns")]
    pub GPT327_NS: self::Gpt320Ns,
    #[cfg(feature = "gpt168_ns")]
    pub GPT168_NS: self::Gpt168Ns,
    #[cfg(feature = "gpt169_ns")]
    pub GPT169_NS: self::Gpt168Ns,
    #[cfg(feature = "gpt1610_ns")]
    pub GPT1610_NS: self::Gpt1610Ns,
    #[cfg(feature = "gpt1611_ns")]
    pub GPT1611_NS: self::Gpt1610Ns,
    #[cfg(feature = "gpt1612_ns")]
    pub GPT1612_NS: self::Gpt1610Ns,
    #[cfg(feature = "gpt1613_ns")]
    pub GPT1613_NS: self::Gpt1610Ns,
    #[cfg(feature = "adc120_ns")]
    pub ADC120_NS: self::Adc120Ns,
    #[cfg(feature = "adc121_ns")]
    pub ADC121_NS: self::Adc121Ns,
    #[cfg(feature = "dac12_ns")]
    pub DAC12_NS: self::Dac12Ns,
    #[cfg(feature = "ceu_ns")]
    pub CEU_NS: self::CeuNs,
    #[cfg(feature = "edmac0_ns")]
    pub EDMAC0_NS: self::Edmac0Ns,
    #[cfg(feature = "etherc0_ns")]
    pub ETHERC0_NS: self::Etherc0Ns,
    #[cfg(feature = "sci0_b_ns")]
    pub SCI0_B_NS: self::Sci0BNs,
    #[cfg(feature = "sci1_b_ns")]
    pub SCI1_B_NS: self::Sci0BNs,
    #[cfg(feature = "sci2_b_ns")]
    pub SCI2_B_NS: self::Sci0BNs,
    #[cfg(feature = "sci3_b_ns")]
    pub SCI3_B_NS: self::Sci0BNs,
    #[cfg(feature = "sci4_b_ns")]
    pub SCI4_B_NS: self::Sci0BNs,
    #[cfg(feature = "sci9_b_ns")]
    pub SCI9_B_NS: self::Sci0BNs,
    #[cfg(feature = "spi0_b_ns")]
    pub SPI0_B_NS: self::Spi0BNs,
    #[cfg(feature = "spi1_b_ns")]
    pub SPI1_B_NS: self::Spi0BNs,
    #[cfg(feature = "eccmb0_ns")]
    pub ECCMB0_NS: self::Eccmb0Ns,
    #[cfg(feature = "eccmb1_ns")]
    pub ECCMB1_NS: self::Eccmb0Ns,
    #[cfg(feature = "canfd0_ns")]
    pub CANFD0_NS: self::Canfd0Ns,
    #[cfg(feature = "canfd1_ns")]
    pub CANFD1_NS: self::Canfd0Ns,
    #[cfg(feature = "port0_ns")]
    pub PORT0_NS: self::Port0Ns,
    #[cfg(feature = "port1_ns")]
    pub PORT1_NS: self::Port1Ns,
    #[cfg(feature = "port2_ns")]
    pub PORT2_NS: self::Port1Ns,
    #[cfg(feature = "port3_ns")]
    pub PORT3_NS: self::Port1Ns,
    #[cfg(feature = "port4_ns")]
    pub PORT4_NS: self::Port1Ns,
    #[cfg(feature = "port5_ns")]
    pub PORT5_NS: self::Port0Ns,
    #[cfg(feature = "port6_ns")]
    pub PORT6_NS: self::Port0Ns,
    #[cfg(feature = "port7_ns")]
    pub PORT7_NS: self::Port0Ns,
    #[cfg(feature = "port8_ns")]
    pub PORT8_NS: self::Port0Ns,
    #[cfg(feature = "port9_ns")]
    pub PORT9_NS: self::Port0Ns,
    #[cfg(feature = "porta_ns")]
    pub PORTA_NS: self::PortaNs,
    #[cfg(feature = "portb_ns")]
    pub PORTB_NS: self::PortbNs,
    #[cfg(feature = "pfs_ns")]
    pub PFS_NS: self::PfsNs,
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
            #[cfg(feature = "sram")]
            SRAM: crate::SRAM,
            #[cfg(feature = "bus")]
            BUS: crate::BUS,
            #[cfg(feature = "icu_common")]
            ICU_COMMON: crate::ICU_COMMON,
            #[cfg(feature = "cpscu")]
            CPSCU: crate::CPSCU,
            #[cfg(feature = "dmac00")]
            DMAC00: crate::DMAC00,
            #[cfg(feature = "dmac01")]
            DMAC01: crate::DMAC01,
            #[cfg(feature = "dmac02")]
            DMAC02: crate::DMAC02,
            #[cfg(feature = "dmac03")]
            DMAC03: crate::DMAC03,
            #[cfg(feature = "dmac04")]
            DMAC04: crate::DMAC04,
            #[cfg(feature = "dmac05")]
            DMAC05: crate::DMAC05,
            #[cfg(feature = "dmac06")]
            DMAC06: crate::DMAC06,
            #[cfg(feature = "dmac07")]
            DMAC07: crate::DMAC07,
            #[cfg(feature = "dma0")]
            DMA0: crate::DMA0,
            #[cfg(feature = "dtc0")]
            DTC0: crate::DTC0,
            #[cfg(feature = "icu")]
            ICU: crate::ICU,
            #[cfg(feature = "cpu_ctrl")]
            CPU_CTRL: crate::CPU_CTRL,
            #[cfg(feature = "ocd_cpu")]
            OCD_CPU: crate::OCD_CPU,
            #[cfg(feature = "cpu_dbg")]
            CPU_DBG: crate::CPU_DBG,
            #[cfg(feature = "fcache")]
            FCACHE: crate::FCACHE,
            #[cfg(feature = "sysc")]
            SYSC: crate::SYSC,
            #[cfg(feature = "tsd")]
            TSD: crate::TSD,
            #[cfg(feature = "flad")]
            FLAD: crate::FLAD,
            #[cfg(feature = "faci")]
            FACI: crate::FACI,
            #[cfg(feature = "elc")]
            ELC: crate::ELC,
            #[cfg(feature = "rtc")]
            RTC: crate::RTC,
            #[cfg(feature = "iwdt")]
            IWDT: crate::IWDT,
            #[cfg(feature = "cac")]
            CAC: crate::CAC,
            #[cfg(feature = "wdt0")]
            WDT0: crate::WDT0,
            #[cfg(feature = "mstp")]
            MSTP: crate::MSTP,
            #[cfg(feature = "pscu")]
            PSCU: crate::PSCU,
            #[cfg(feature = "poeg")]
            POEG: crate::POEG,
            #[cfg(feature = "ulpt0")]
            ULPT0: crate::ULPT0,
            #[cfg(feature = "ulpt1")]
            ULPT1: crate::ULPT1,
            #[cfg(feature = "agt0")]
            AGT0: crate::AGT0,
            #[cfg(feature = "agt1")]
            AGT1: crate::AGT1,
            #[cfg(feature = "tsn")]
            TSN: crate::TSN,
            #[cfg(feature = "acmphs0")]
            ACMPHS0: crate::ACMPHS0,
            #[cfg(feature = "acmphs1")]
            ACMPHS1: crate::ACMPHS1,
            #[cfg(feature = "usbfs")]
            USBFS: crate::USBFS,
            #[cfg(feature = "ssie0")]
            SSIE0: crate::SSIE0,
            #[cfg(feature = "ssie1")]
            SSIE1: crate::SSIE1,
            #[cfg(feature = "iic0")]
            IIC0: crate::IIC0,
            #[cfg(feature = "iic0wu")]
            IIC0WU: crate::IIC0WU,
            #[cfg(feature = "iic1")]
            IIC1: crate::IIC1,
            #[cfg(feature = "ospi0_b")]
            OSPI0_B: crate::OSPI0_B,
            #[cfg(feature = "crc")]
            CRC: crate::CRC,
            #[cfg(feature = "doc_b")]
            DOC_B: crate::DOC_B,
            #[cfg(feature = "gpt320")]
            GPT320: crate::GPT320,
            #[cfg(feature = "gpt321")]
            GPT321: crate::GPT321,
            #[cfg(feature = "gpt322")]
            GPT322: crate::GPT322,
            #[cfg(feature = "gpt323")]
            GPT323: crate::GPT323,
            #[cfg(feature = "gpt324")]
            GPT324: crate::GPT324,
            #[cfg(feature = "gpt325")]
            GPT325: crate::GPT325,
            #[cfg(feature = "gpt326")]
            GPT326: crate::GPT326,
            #[cfg(feature = "gpt327")]
            GPT327: crate::GPT327,
            #[cfg(feature = "gpt168")]
            GPT168: crate::GPT168,
            #[cfg(feature = "gpt169")]
            GPT169: crate::GPT169,
            #[cfg(feature = "gpt1610")]
            GPT1610: crate::GPT1610,
            #[cfg(feature = "gpt1611")]
            GPT1611: crate::GPT1611,
            #[cfg(feature = "gpt1612")]
            GPT1612: crate::GPT1612,
            #[cfg(feature = "gpt1613")]
            GPT1613: crate::GPT1613,
            #[cfg(feature = "adc120")]
            ADC120: crate::ADC120,
            #[cfg(feature = "adc121")]
            ADC121: crate::ADC121,
            #[cfg(feature = "dac12")]
            DAC12: crate::DAC12,
            #[cfg(feature = "ceu")]
            CEU: crate::CEU,
            #[cfg(feature = "edmac0")]
            EDMAC0: crate::EDMAC0,
            #[cfg(feature = "etherc0")]
            ETHERC0: crate::ETHERC0,
            #[cfg(feature = "sci0_b")]
            SCI0_B: crate::SCI0_B,
            #[cfg(feature = "sci1_b")]
            SCI1_B: crate::SCI1_B,
            #[cfg(feature = "sci2_b")]
            SCI2_B: crate::SCI2_B,
            #[cfg(feature = "sci3_b")]
            SCI3_B: crate::SCI3_B,
            #[cfg(feature = "sci4_b")]
            SCI4_B: crate::SCI4_B,
            #[cfg(feature = "sci9_b")]
            SCI9_B: crate::SCI9_B,
            #[cfg(feature = "spi0_b")]
            SPI0_B: crate::SPI0_B,
            #[cfg(feature = "spi1_b")]
            SPI1_B: crate::SPI1_B,
            #[cfg(feature = "eccmb0")]
            ECCMB0: crate::ECCMB0,
            #[cfg(feature = "eccmb1")]
            ECCMB1: crate::ECCMB1,
            #[cfg(feature = "canfd0")]
            CANFD0: crate::CANFD0,
            #[cfg(feature = "canfd1")]
            CANFD1: crate::CANFD1,
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
            #[cfg(feature = "port8")]
            PORT8: crate::PORT8,
            #[cfg(feature = "port9")]
            PORT9: crate::PORT9,
            #[cfg(feature = "porta")]
            PORTA: crate::PORTA,
            #[cfg(feature = "portb")]
            PORTB: crate::PORTB,
            #[cfg(feature = "pfs")]
            PFS: crate::PFS,
            #[cfg(feature = "rmpu_ns")]
            RMPU_NS: crate::RMPU_NS,
            #[cfg(feature = "sram_ns")]
            SRAM_NS: crate::SRAM_NS,
            #[cfg(feature = "bus_ns")]
            BUS_NS: crate::BUS_NS,
            #[cfg(feature = "icu_common_ns")]
            ICU_COMMON_NS: crate::ICU_COMMON_NS,
            #[cfg(feature = "cpscu_ns")]
            CPSCU_NS: crate::CPSCU_NS,
            #[cfg(feature = "dmac00_ns")]
            DMAC00_NS: crate::DMAC00_NS,
            #[cfg(feature = "dmac01_ns")]
            DMAC01_NS: crate::DMAC01_NS,
            #[cfg(feature = "dmac02_ns")]
            DMAC02_NS: crate::DMAC02_NS,
            #[cfg(feature = "dmac03_ns")]
            DMAC03_NS: crate::DMAC03_NS,
            #[cfg(feature = "dmac04_ns")]
            DMAC04_NS: crate::DMAC04_NS,
            #[cfg(feature = "dmac05_ns")]
            DMAC05_NS: crate::DMAC05_NS,
            #[cfg(feature = "dmac06_ns")]
            DMAC06_NS: crate::DMAC06_NS,
            #[cfg(feature = "dmac07_ns")]
            DMAC07_NS: crate::DMAC07_NS,
            #[cfg(feature = "dma0_ns")]
            DMA0_NS: crate::DMA0_NS,
            #[cfg(feature = "dtc0_ns")]
            DTC0_NS: crate::DTC0_NS,
            #[cfg(feature = "icu_ns")]
            ICU_NS: crate::ICU_NS,
            #[cfg(feature = "cpu_ctrl_ns")]
            CPU_CTRL_NS: crate::CPU_CTRL_NS,
            #[cfg(feature = "ocd_cpu_ns")]
            OCD_CPU_NS: crate::OCD_CPU_NS,
            #[cfg(feature = "cpu_dbg_ns")]
            CPU_DBG_NS: crate::CPU_DBG_NS,
            #[cfg(feature = "fcache_ns")]
            FCACHE_NS: crate::FCACHE_NS,
            #[cfg(feature = "sysc_ns")]
            SYSC_NS: crate::SYSC_NS,
            #[cfg(feature = "tsd_ns")]
            TSD_NS: crate::TSD_NS,
            #[cfg(feature = "flad_ns")]
            FLAD_NS: crate::FLAD_NS,
            #[cfg(feature = "faci_ns")]
            FACI_NS: crate::FACI_NS,
            #[cfg(feature = "elc_ns")]
            ELC_NS: crate::ELC_NS,
            #[cfg(feature = "rtc_ns")]
            RTC_NS: crate::RTC_NS,
            #[cfg(feature = "iwdt_ns")]
            IWDT_NS: crate::IWDT_NS,
            #[cfg(feature = "cac_ns")]
            CAC_NS: crate::CAC_NS,
            #[cfg(feature = "wdt0_ns")]
            WDT0_NS: crate::WDT0_NS,
            #[cfg(feature = "mstp_ns")]
            MSTP_NS: crate::MSTP_NS,
            #[cfg(feature = "pscu_ns")]
            PSCU_NS: crate::PSCU_NS,
            #[cfg(feature = "poeg_ns")]
            POEG_NS: crate::POEG_NS,
            #[cfg(feature = "ulpt0_ns")]
            ULPT0_NS: crate::ULPT0_NS,
            #[cfg(feature = "ulpt1_ns")]
            ULPT1_NS: crate::ULPT1_NS,
            #[cfg(feature = "agt0_ns")]
            AGT0_NS: crate::AGT0_NS,
            #[cfg(feature = "agt1_ns")]
            AGT1_NS: crate::AGT1_NS,
            #[cfg(feature = "tsn_ns")]
            TSN_NS: crate::TSN_NS,
            #[cfg(feature = "acmphs0_ns")]
            ACMPHS0_NS: crate::ACMPHS0_NS,
            #[cfg(feature = "acmphs1_ns")]
            ACMPHS1_NS: crate::ACMPHS1_NS,
            #[cfg(feature = "usbfs_ns")]
            USBFS_NS: crate::USBFS_NS,
            #[cfg(feature = "ssie0_ns")]
            SSIE0_NS: crate::SSIE0_NS,
            #[cfg(feature = "ssie1_ns")]
            SSIE1_NS: crate::SSIE1_NS,
            #[cfg(feature = "iic0_ns")]
            IIC0_NS: crate::IIC0_NS,
            #[cfg(feature = "iic0wu_ns")]
            IIC0WU_NS: crate::IIC0WU_NS,
            #[cfg(feature = "iic1_ns")]
            IIC1_NS: crate::IIC1_NS,
            #[cfg(feature = "ospi0_b_ns")]
            OSPI0_B_NS: crate::OSPI0_B_NS,
            #[cfg(feature = "crc_ns")]
            CRC_NS: crate::CRC_NS,
            #[cfg(feature = "doc_b_ns")]
            DOC_B_NS: crate::DOC_B_NS,
            #[cfg(feature = "gpt320_ns")]
            GPT320_NS: crate::GPT320_NS,
            #[cfg(feature = "gpt321_ns")]
            GPT321_NS: crate::GPT321_NS,
            #[cfg(feature = "gpt322_ns")]
            GPT322_NS: crate::GPT322_NS,
            #[cfg(feature = "gpt323_ns")]
            GPT323_NS: crate::GPT323_NS,
            #[cfg(feature = "gpt324_ns")]
            GPT324_NS: crate::GPT324_NS,
            #[cfg(feature = "gpt325_ns")]
            GPT325_NS: crate::GPT325_NS,
            #[cfg(feature = "gpt326_ns")]
            GPT326_NS: crate::GPT326_NS,
            #[cfg(feature = "gpt327_ns")]
            GPT327_NS: crate::GPT327_NS,
            #[cfg(feature = "gpt168_ns")]
            GPT168_NS: crate::GPT168_NS,
            #[cfg(feature = "gpt169_ns")]
            GPT169_NS: crate::GPT169_NS,
            #[cfg(feature = "gpt1610_ns")]
            GPT1610_NS: crate::GPT1610_NS,
            #[cfg(feature = "gpt1611_ns")]
            GPT1611_NS: crate::GPT1611_NS,
            #[cfg(feature = "gpt1612_ns")]
            GPT1612_NS: crate::GPT1612_NS,
            #[cfg(feature = "gpt1613_ns")]
            GPT1613_NS: crate::GPT1613_NS,
            #[cfg(feature = "adc120_ns")]
            ADC120_NS: crate::ADC120_NS,
            #[cfg(feature = "adc121_ns")]
            ADC121_NS: crate::ADC121_NS,
            #[cfg(feature = "dac12_ns")]
            DAC12_NS: crate::DAC12_NS,
            #[cfg(feature = "ceu_ns")]
            CEU_NS: crate::CEU_NS,
            #[cfg(feature = "edmac0_ns")]
            EDMAC0_NS: crate::EDMAC0_NS,
            #[cfg(feature = "etherc0_ns")]
            ETHERC0_NS: crate::ETHERC0_NS,
            #[cfg(feature = "sci0_b_ns")]
            SCI0_B_NS: crate::SCI0_B_NS,
            #[cfg(feature = "sci1_b_ns")]
            SCI1_B_NS: crate::SCI1_B_NS,
            #[cfg(feature = "sci2_b_ns")]
            SCI2_B_NS: crate::SCI2_B_NS,
            #[cfg(feature = "sci3_b_ns")]
            SCI3_B_NS: crate::SCI3_B_NS,
            #[cfg(feature = "sci4_b_ns")]
            SCI4_B_NS: crate::SCI4_B_NS,
            #[cfg(feature = "sci9_b_ns")]
            SCI9_B_NS: crate::SCI9_B_NS,
            #[cfg(feature = "spi0_b_ns")]
            SPI0_B_NS: crate::SPI0_B_NS,
            #[cfg(feature = "spi1_b_ns")]
            SPI1_B_NS: crate::SPI1_B_NS,
            #[cfg(feature = "eccmb0_ns")]
            ECCMB0_NS: crate::ECCMB0_NS,
            #[cfg(feature = "eccmb1_ns")]
            ECCMB1_NS: crate::ECCMB1_NS,
            #[cfg(feature = "canfd0_ns")]
            CANFD0_NS: crate::CANFD0_NS,
            #[cfg(feature = "canfd1_ns")]
            CANFD1_NS: crate::CANFD1_NS,
            #[cfg(feature = "port0_ns")]
            PORT0_NS: crate::PORT0_NS,
            #[cfg(feature = "port1_ns")]
            PORT1_NS: crate::PORT1_NS,
            #[cfg(feature = "port2_ns")]
            PORT2_NS: crate::PORT2_NS,
            #[cfg(feature = "port3_ns")]
            PORT3_NS: crate::PORT3_NS,
            #[cfg(feature = "port4_ns")]
            PORT4_NS: crate::PORT4_NS,
            #[cfg(feature = "port5_ns")]
            PORT5_NS: crate::PORT5_NS,
            #[cfg(feature = "port6_ns")]
            PORT6_NS: crate::PORT6_NS,
            #[cfg(feature = "port7_ns")]
            PORT7_NS: crate::PORT7_NS,
            #[cfg(feature = "port8_ns")]
            PORT8_NS: crate::PORT8_NS,
            #[cfg(feature = "port9_ns")]
            PORT9_NS: crate::PORT9_NS,
            #[cfg(feature = "porta_ns")]
            PORTA_NS: crate::PORTA_NS,
            #[cfg(feature = "portb_ns")]
            PORTB_NS: crate::PORTB_NS,
            #[cfg(feature = "pfs_ns")]
            PFS_NS: crate::PFS_NS,
        }
    }
}
