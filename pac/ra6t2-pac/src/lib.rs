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
// Generated from SVD 1.40.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:37:20 +0000
#![cfg_attr(not(feature = "tracing"), no_std)]
#![allow(non_camel_case_types)]
#![doc = "Arm Cortex-M33 based Microcontroller RA6T2 group"]
pub mod common;
pub use common::*;

#[cfg(feature = "tracing")]
pub mod reg_name;
#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "acmphs0")]
pub mod acmphs0;
#[cfg(feature = "adc_b")]
pub mod adc_b;
#[cfg(feature = "agtw_b0")]
pub mod agtw_b0;
#[cfg(feature = "bus")]
pub mod bus;
#[cfg(feature = "cac")]
pub mod cac;
#[cfg(feature = "cache")]
pub mod cache;
#[cfg(feature = "canfd")]
pub mod canfd;
#[cfg(feature = "cpscu")]
pub mod cpscu;
#[cfg(feature = "crc")]
pub mod crc;
#[cfg(feature = "dac120")]
pub mod dac120;
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
#[cfg(feature = "elc_b")]
pub mod elc_b;
#[cfg(feature = "faci")]
pub mod faci;
#[cfg(feature = "fcache")]
pub mod fcache;
#[cfg(feature = "flad")]
pub mod flad;
#[cfg(feature = "gpt320")]
pub mod gpt320;
#[cfg(feature = "gpt324")]
pub mod gpt324;
#[cfg(feature = "gpt_gtclk")]
pub mod gpt_gtclk;
#[cfg(feature = "gpt_ops")]
pub mod gpt_ops;
#[cfg(feature = "icu")]
pub mod icu;
#[cfg(feature = "iic0wu_b")]
pub mod iic0wu_b;
#[cfg(feature = "iic_b0")]
pub mod iic_b0;
#[cfg(feature = "iirfa")]
pub mod iirfa;
#[cfg(feature = "iwdt")]
pub mod iwdt;
#[cfg(feature = "kint")]
pub mod kint;
#[cfg(feature = "mstp")]
pub mod mstp;
#[cfg(feature = "pdg")]
pub mod pdg;
#[cfg(feature = "pfs_b")]
pub mod pfs_b;
#[cfg(feature = "poeg")]
pub mod poeg;
#[cfg(feature = "port0")]
pub mod port0;
#[cfg(feature = "port2")]
pub mod port2;
#[cfg(feature = "porta")]
pub mod porta;
#[cfg(feature = "portb")]
pub mod portb;
#[cfg(feature = "portc")]
pub mod portc;
#[cfg(feature = "portd")]
pub mod portd;
#[cfg(feature = "porte")]
pub mod porte;
#[cfg(feature = "pscu")]
pub mod pscu;
#[cfg(feature = "rmpu")]
pub mod rmpu;
#[cfg(feature = "sci_b0")]
pub mod sci_b0;
#[cfg(feature = "spi_b0")]
pub mod spi_b0;
#[cfg(feature = "sram")]
pub mod sram;
#[cfg(feature = "sysc")]
pub mod sysc;
#[cfg(feature = "tfu")]
pub mod tfu;
#[cfg(feature = "tsd")]
pub mod tsd;
#[cfg(feature = "tsn")]
pub mod tsn;
#[cfg(feature = "tzf")]
pub mod tzf;
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
#[cfg(feature = "port2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port2 {
    ptr: *mut u8,
}
#[cfg(feature = "port2")]
pub const PORT2: self::Port2 = self::Port2 {
    ptr: 0x4001f040u32 as _,
};
#[cfg(feature = "porta")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Porta {
    ptr: *mut u8,
}
#[cfg(feature = "porta")]
pub const PORTA: self::Porta = self::Porta {
    ptr: 0x4001f140u32 as _,
};
#[cfg(feature = "portb")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Portb {
    ptr: *mut u8,
}
#[cfg(feature = "portb")]
pub const PORTB: self::Portb = self::Portb {
    ptr: 0x4001f160u32 as _,
};
#[cfg(feature = "portc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Portc {
    ptr: *mut u8,
}
#[cfg(feature = "portc")]
pub const PORTC: self::Portc = self::Portc {
    ptr: 0x4001f180u32 as _,
};
#[cfg(feature = "portd")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Portd {
    ptr: *mut u8,
}
#[cfg(feature = "portd")]
pub const PORTD: self::Portd = self::Portd {
    ptr: 0x4001f1a0u32 as _,
};
#[cfg(feature = "porte")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Porte {
    ptr: *mut u8,
}
#[cfg(feature = "porte")]
pub const PORTE: self::Porte = self::Porte {
    ptr: 0x4001f1c0u32 as _,
};
#[cfg(feature = "pfs_b")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PfsB {
    ptr: *mut u8,
}
#[cfg(feature = "pfs_b")]
pub const PFS_B: self::PfsB = self::PfsB {
    ptr: 0x4001f800u32 as _,
};
#[cfg(feature = "iirfa")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirfa {
    ptr: *mut u8,
}
#[cfg(feature = "iirfa")]
pub const IIRFA: self::Iirfa = self::Iirfa {
    ptr: 0x40020000u32 as _,
};
#[cfg(feature = "tfu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfu {
    ptr: *mut u8,
}
#[cfg(feature = "tfu")]
pub const TFU: self::Tfu = self::Tfu {
    ptr: 0x40021000u32 as _,
};
#[cfg(feature = "elc_b")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElcB {
    ptr: *mut u8,
}
#[cfg(feature = "elc_b")]
pub const ELC_B: self::ElcB = self::ElcB {
    ptr: 0x40082000u32 as _,
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
#[cfg(feature = "kint")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kint {
    ptr: *mut u8,
}
#[cfg(feature = "kint")]
pub const KINT: self::Kint = self::Kint {
    ptr: 0x40085000u32 as _,
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
#[cfg(feature = "canfd")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Canfd {
    ptr: *mut u8,
}
#[cfg(feature = "canfd")]
pub const CANFD: self::Canfd = self::Canfd {
    ptr: 0x400b0000u32 as _,
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
#[cfg(feature = "agtw_b0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgtwB0 {
    ptr: *mut u8,
}
#[cfg(feature = "agtw_b0")]
pub const AGTW_B0: self::AgtwB0 = self::AgtwB0 {
    ptr: 0x400e8000u32 as _,
};
#[cfg(feature = "agtw_b1")]
pub const AGTW_B1: self::AgtwB0 = self::AgtwB0 {
    ptr: 0x400e8100u32 as _,
};
#[cfg(feature = "tsn")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsn {
    ptr: *mut u8,
}
#[cfg(feature = "tsn")]
pub const TSN: self::Tsn = self::Tsn {
    ptr: 0x400f3000u32 as _,
};
#[cfg(feature = "acmphs0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acmphs0 {
    ptr: *mut u8,
}
#[cfg(feature = "acmphs0")]
pub const ACMPHS0: self::Acmphs0 = self::Acmphs0 {
    ptr: 0x400f4000u32 as _,
};
#[cfg(feature = "acmphs1")]
pub const ACMPHS1: self::Acmphs0 = self::Acmphs0 {
    ptr: 0x400f4100u32 as _,
};
#[cfg(feature = "acmphs2")]
pub const ACMPHS2: self::Acmphs0 = self::Acmphs0 {
    ptr: 0x400f4200u32 as _,
};
#[cfg(feature = "acmphs3")]
pub const ACMPHS3: self::Acmphs0 = self::Acmphs0 {
    ptr: 0x400f4300u32 as _,
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
#[cfg(feature = "sci_b0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SciB0 {
    ptr: *mut u8,
}
#[cfg(feature = "sci_b0")]
pub const SCI_B0: self::SciB0 = self::SciB0 {
    ptr: 0x40118000u32 as _,
};
#[cfg(feature = "sci_b1")]
pub const SCI_B1: self::SciB0 = self::SciB0 {
    ptr: 0x40118100u32 as _,
};
#[cfg(feature = "sci_b2")]
pub const SCI_B2: self::SciB0 = self::SciB0 {
    ptr: 0x40118200u32 as _,
};
#[cfg(feature = "sci_b3")]
pub const SCI_B3: self::SciB0 = self::SciB0 {
    ptr: 0x40118300u32 as _,
};
#[cfg(feature = "sci_b4")]
pub const SCI_B4: self::SciB0 = self::SciB0 {
    ptr: 0x40118400u32 as _,
};
#[cfg(feature = "sci_b9")]
pub const SCI_B9: self::SciB0 = self::SciB0 {
    ptr: 0x40118900u32 as _,
};
#[cfg(feature = "spi_b0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiB0 {
    ptr: *mut u8,
}
#[cfg(feature = "spi_b0")]
pub const SPI_B0: self::SpiB0 = self::SpiB0 {
    ptr: 0x4011a000u32 as _,
};
#[cfg(feature = "spi_b1")]
pub const SPI_B1: self::SpiB0 = self::SpiB0 {
    ptr: 0x4011a100u32 as _,
};
#[cfg(feature = "iic_b0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IicB0 {
    ptr: *mut u8,
}
#[cfg(feature = "iic_b0")]
pub const IIC_B0: self::IicB0 = self::IicB0 {
    ptr: 0x4011f000u32 as _,
};
#[cfg(feature = "iic0wu_b")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iic0WuB {
    ptr: *mut u8,
}
#[cfg(feature = "iic0wu_b")]
pub const IIC0WU_B: self::Iic0WuB = self::Iic0WuB {
    ptr: 0x4011f098u32 as _,
};
#[cfg(feature = "iic_b1")]
pub const IIC_B1: self::IicB0 = self::IicB0 {
    ptr: 0x4011f400u32 as _,
};
#[cfg(feature = "eccmb")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccmb {
    ptr: *mut u8,
}
#[cfg(feature = "eccmb")]
pub const ECCMB: self::Eccmb = self::Eccmb {
    ptr: 0x4012f200u32 as _,
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
#[cfg(feature = "gpt322")]
pub const GPT322: self::Gpt320 = self::Gpt320 {
    ptr: 0x40169200u32 as _,
};
#[cfg(feature = "gpt323")]
pub const GPT323: self::Gpt320 = self::Gpt320 {
    ptr: 0x40169300u32 as _,
};
#[cfg(feature = "gpt324")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt324 {
    ptr: *mut u8,
}
#[cfg(feature = "gpt324")]
pub const GPT324: self::Gpt324 = self::Gpt324 {
    ptr: 0x40169400u32 as _,
};
#[cfg(feature = "gpt325")]
pub const GPT325: self::Gpt324 = self::Gpt324 {
    ptr: 0x40169500u32 as _,
};
#[cfg(feature = "gpt326")]
pub const GPT326: self::Gpt324 = self::Gpt324 {
    ptr: 0x40169600u32 as _,
};
#[cfg(feature = "gpt327")]
pub const GPT327: self::Gpt324 = self::Gpt324 {
    ptr: 0x40169700u32 as _,
};
#[cfg(feature = "gpt328")]
pub const GPT328: self::Gpt324 = self::Gpt324 {
    ptr: 0x40169800u32 as _,
};
#[cfg(feature = "gpt329")]
pub const GPT329: self::Gpt324 = self::Gpt324 {
    ptr: 0x40169900u32 as _,
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
#[cfg(feature = "gpt_gtclk")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GptGtclk {
    ptr: *mut u8,
}
#[cfg(feature = "gpt_gtclk")]
pub const GPT_GTCLK: self::GptGtclk = self::GptGtclk {
    ptr: 0x40169b00u32 as _,
};
#[cfg(feature = "pdg")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdg {
    ptr: *mut u8,
}
#[cfg(feature = "pdg")]
pub const PDG: self::Pdg = self::Pdg {
    ptr: 0x4016a000u32 as _,
};
#[cfg(feature = "adc_b")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcB {
    ptr: *mut u8,
}
#[cfg(feature = "adc_b")]
pub const ADC_B: self::AdcB = self::AdcB {
    ptr: 0x40170000u32 as _,
};
#[cfg(feature = "dac120")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac120 {
    ptr: *mut u8,
}
#[cfg(feature = "dac120")]
pub const DAC120: self::Dac120 = self::Dac120 {
    ptr: 0x40172000u32 as _,
};
#[cfg(feature = "dac121")]
pub const DAC121: self::Dac120 = self::Dac120 {
    ptr: 0x40172100u32 as _,
};
#[cfg(feature = "tsd")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsd {
    ptr: *mut u8,
}
#[cfg(feature = "tsd")]
pub const TSD: self::Tsd = self::Tsd {
    ptr: 0x407fb000u32 as _,
};
#[cfg(feature = "flad")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flad {
    ptr: *mut u8,
}
#[cfg(feature = "flad")]
pub const FLAD: self::Flad = self::Flad {
    ptr: 0x407fc000u32 as _,
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
    extern "C" {
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
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
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
    #[cfg(feature = "port2")]
    pub PORT2: self::Port2,
    #[cfg(feature = "porta")]
    pub PORTA: self::Porta,
    #[cfg(feature = "portb")]
    pub PORTB: self::Portb,
    #[cfg(feature = "portc")]
    pub PORTC: self::Portc,
    #[cfg(feature = "portd")]
    pub PORTD: self::Portd,
    #[cfg(feature = "porte")]
    pub PORTE: self::Porte,
    #[cfg(feature = "pfs_b")]
    pub PFS_B: self::PfsB,
    #[cfg(feature = "iirfa")]
    pub IIRFA: self::Iirfa,
    #[cfg(feature = "tfu")]
    pub TFU: self::Tfu,
    #[cfg(feature = "elc_b")]
    pub ELC_B: self::ElcB,
    #[cfg(feature = "iwdt")]
    pub IWDT: self::Iwdt,
    #[cfg(feature = "wdt")]
    pub WDT: self::Wdt,
    #[cfg(feature = "cac")]
    pub CAC: self::Cac,
    #[cfg(feature = "mstp")]
    pub MSTP: self::Mstp,
    #[cfg(feature = "kint")]
    pub KINT: self::Kint,
    #[cfg(feature = "poeg")]
    pub POEG: self::Poeg,
    #[cfg(feature = "canfd")]
    pub CANFD: self::Canfd,
    #[cfg(feature = "pscu")]
    pub PSCU: self::Pscu,
    #[cfg(feature = "agtw_b0")]
    pub AGTW_B0: self::AgtwB0,
    #[cfg(feature = "agtw_b1")]
    pub AGTW_B1: self::AgtwB0,
    #[cfg(feature = "tsn")]
    pub TSN: self::Tsn,
    #[cfg(feature = "acmphs0")]
    pub ACMPHS0: self::Acmphs0,
    #[cfg(feature = "acmphs1")]
    pub ACMPHS1: self::Acmphs0,
    #[cfg(feature = "acmphs2")]
    pub ACMPHS2: self::Acmphs0,
    #[cfg(feature = "acmphs3")]
    pub ACMPHS3: self::Acmphs0,
    #[cfg(feature = "crc")]
    pub CRC: self::Crc,
    #[cfg(feature = "doc_b")]
    pub DOC_B: self::DocB,
    #[cfg(feature = "sci_b0")]
    pub SCI_B0: self::SciB0,
    #[cfg(feature = "sci_b1")]
    pub SCI_B1: self::SciB0,
    #[cfg(feature = "sci_b2")]
    pub SCI_B2: self::SciB0,
    #[cfg(feature = "sci_b3")]
    pub SCI_B3: self::SciB0,
    #[cfg(feature = "sci_b4")]
    pub SCI_B4: self::SciB0,
    #[cfg(feature = "sci_b9")]
    pub SCI_B9: self::SciB0,
    #[cfg(feature = "spi_b0")]
    pub SPI_B0: self::SpiB0,
    #[cfg(feature = "spi_b1")]
    pub SPI_B1: self::SpiB0,
    #[cfg(feature = "iic_b0")]
    pub IIC_B0: self::IicB0,
    #[cfg(feature = "iic0wu_b")]
    pub IIC0WU_B: self::Iic0WuB,
    #[cfg(feature = "iic_b1")]
    pub IIC_B1: self::IicB0,
    #[cfg(feature = "eccmb")]
    pub ECCMB: self::Eccmb,
    #[cfg(feature = "gpt320")]
    pub GPT320: self::Gpt320,
    #[cfg(feature = "gpt321")]
    pub GPT321: self::Gpt320,
    #[cfg(feature = "gpt322")]
    pub GPT322: self::Gpt320,
    #[cfg(feature = "gpt323")]
    pub GPT323: self::Gpt320,
    #[cfg(feature = "gpt324")]
    pub GPT324: self::Gpt324,
    #[cfg(feature = "gpt325")]
    pub GPT325: self::Gpt324,
    #[cfg(feature = "gpt326")]
    pub GPT326: self::Gpt324,
    #[cfg(feature = "gpt327")]
    pub GPT327: self::Gpt324,
    #[cfg(feature = "gpt328")]
    pub GPT328: self::Gpt324,
    #[cfg(feature = "gpt329")]
    pub GPT329: self::Gpt324,
    #[cfg(feature = "gpt_ops")]
    pub GPT_OPS: self::GptOps,
    #[cfg(feature = "gpt_gtclk")]
    pub GPT_GTCLK: self::GptGtclk,
    #[cfg(feature = "pdg")]
    pub PDG: self::Pdg,
    #[cfg(feature = "adc_b")]
    pub ADC_B: self::AdcB,
    #[cfg(feature = "dac120")]
    pub DAC120: self::Dac120,
    #[cfg(feature = "dac121")]
    pub DAC121: self::Dac120,
    #[cfg(feature = "tsd")]
    pub TSD: self::Tsd,
    #[cfg(feature = "flad")]
    pub FLAD: self::Flad,
    #[cfg(feature = "faci")]
    pub FACI: self::Faci,
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
            #[cfg(feature = "port2")]
            PORT2: crate::PORT2,
            #[cfg(feature = "porta")]
            PORTA: crate::PORTA,
            #[cfg(feature = "portb")]
            PORTB: crate::PORTB,
            #[cfg(feature = "portc")]
            PORTC: crate::PORTC,
            #[cfg(feature = "portd")]
            PORTD: crate::PORTD,
            #[cfg(feature = "porte")]
            PORTE: crate::PORTE,
            #[cfg(feature = "pfs_b")]
            PFS_B: crate::PFS_B,
            #[cfg(feature = "iirfa")]
            IIRFA: crate::IIRFA,
            #[cfg(feature = "tfu")]
            TFU: crate::TFU,
            #[cfg(feature = "elc_b")]
            ELC_B: crate::ELC_B,
            #[cfg(feature = "iwdt")]
            IWDT: crate::IWDT,
            #[cfg(feature = "wdt")]
            WDT: crate::WDT,
            #[cfg(feature = "cac")]
            CAC: crate::CAC,
            #[cfg(feature = "mstp")]
            MSTP: crate::MSTP,
            #[cfg(feature = "kint")]
            KINT: crate::KINT,
            #[cfg(feature = "poeg")]
            POEG: crate::POEG,
            #[cfg(feature = "canfd")]
            CANFD: crate::CANFD,
            #[cfg(feature = "pscu")]
            PSCU: crate::PSCU,
            #[cfg(feature = "agtw_b0")]
            AGTW_B0: crate::AGTW_B0,
            #[cfg(feature = "agtw_b1")]
            AGTW_B1: crate::AGTW_B1,
            #[cfg(feature = "tsn")]
            TSN: crate::TSN,
            #[cfg(feature = "acmphs0")]
            ACMPHS0: crate::ACMPHS0,
            #[cfg(feature = "acmphs1")]
            ACMPHS1: crate::ACMPHS1,
            #[cfg(feature = "acmphs2")]
            ACMPHS2: crate::ACMPHS2,
            #[cfg(feature = "acmphs3")]
            ACMPHS3: crate::ACMPHS3,
            #[cfg(feature = "crc")]
            CRC: crate::CRC,
            #[cfg(feature = "doc_b")]
            DOC_B: crate::DOC_B,
            #[cfg(feature = "sci_b0")]
            SCI_B0: crate::SCI_B0,
            #[cfg(feature = "sci_b1")]
            SCI_B1: crate::SCI_B1,
            #[cfg(feature = "sci_b2")]
            SCI_B2: crate::SCI_B2,
            #[cfg(feature = "sci_b3")]
            SCI_B3: crate::SCI_B3,
            #[cfg(feature = "sci_b4")]
            SCI_B4: crate::SCI_B4,
            #[cfg(feature = "sci_b9")]
            SCI_B9: crate::SCI_B9,
            #[cfg(feature = "spi_b0")]
            SPI_B0: crate::SPI_B0,
            #[cfg(feature = "spi_b1")]
            SPI_B1: crate::SPI_B1,
            #[cfg(feature = "iic_b0")]
            IIC_B0: crate::IIC_B0,
            #[cfg(feature = "iic0wu_b")]
            IIC0WU_B: crate::IIC0WU_B,
            #[cfg(feature = "iic_b1")]
            IIC_B1: crate::IIC_B1,
            #[cfg(feature = "eccmb")]
            ECCMB: crate::ECCMB,
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
            #[cfg(feature = "gpt328")]
            GPT328: crate::GPT328,
            #[cfg(feature = "gpt329")]
            GPT329: crate::GPT329,
            #[cfg(feature = "gpt_ops")]
            GPT_OPS: crate::GPT_OPS,
            #[cfg(feature = "gpt_gtclk")]
            GPT_GTCLK: crate::GPT_GTCLK,
            #[cfg(feature = "pdg")]
            PDG: crate::PDG,
            #[cfg(feature = "adc_b")]
            ADC_B: crate::ADC_B,
            #[cfg(feature = "dac120")]
            DAC120: crate::DAC120,
            #[cfg(feature = "dac121")]
            DAC121: crate::DAC121,
            #[cfg(feature = "tsd")]
            TSD: crate::TSD,
            #[cfg(feature = "flad")]
            FLAD: crate::FLAD,
            #[cfg(feature = "faci")]
            FACI: crate::FACI,
        }
    }
}
