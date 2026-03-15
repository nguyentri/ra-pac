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
// Generated from SVD 1.00.01, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:40:06 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Debug Function"]
unsafe impl ::core::marker::Send for super::CpuDbg {}
unsafe impl ::core::marker::Sync for super::CpuDbg {}
impl super::CpuDbg {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Debug Status Register"]
    #[inline(always)]
    pub const fn dbgstr(&self) -> &'static crate::common::Reg<self::Dbgstr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dbgstr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Debug Stop Control Register"]
    #[inline(always)]
    pub const fn dbgstopcr(
        &self,
    ) -> &'static crate::common::Reg<self::Dbgstopcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dbgstopcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Debug Authentication Control Register 0"]
    #[inline(always)]
    pub const fn dbgauth0(
        &self,
    ) -> &'static crate::common::Reg<self::Dbgauth0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dbgauth0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Debug Authentication Control Register 1"]
    #[inline(always)]
    pub const fn dbgauth1(
        &self,
    ) -> &'static crate::common::Reg<self::Dbgauth1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dbgauth1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Trace Port Control Register"]
    #[inline(always)]
    pub const fn trportcr(
        &self,
    ) -> &'static crate::common::Reg<self::Trportcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Trportcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Cache Debug Control Register"]
    #[inline(always)]
    pub const fn cachedbgcr(
        &self,
    ) -> &'static crate::common::Reg<self::Cachedbgcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cachedbgcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "MOCO Enable Request Register for Debug"]
    #[inline(always)]
    pub const fn dbgmocoen(
        &self,
    ) -> &'static crate::common::Reg<self::Dbgmocoen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dbgmocoen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[doc = "Flash Sequencer Clock Select Register for Debug"]
    #[inline(always)]
    pub const fn dbgfclksel(
        &self,
    ) -> &'static crate::common::Reg<self::Dbgfclksel_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dbgfclksel_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(784usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgstr_SPEC;
impl crate::sealed::RegSpec for Dbgstr_SPEC {
    type DataType = u32;
}

#[doc = "Debug Status Register"]
pub type Dbgstr = crate::RegValueT<Dbgstr_SPEC>;

impl Dbgstr {
    #[doc = "Debug power-up request"]
    #[inline(always)]
    pub fn cdbgpwrupreq(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        dbgstr::Cdbgpwrupreq,
        dbgstr::Cdbgpwrupreq,
        Dbgstr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            dbgstr::Cdbgpwrupreq,
            dbgstr::Cdbgpwrupreq,
            Dbgstr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Debug power-up acknowledge"]
    #[inline(always)]
    pub fn cdbgpwrupack(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        dbgstr::Cdbgpwrupack,
        dbgstr::Cdbgpwrupack,
        Dbgstr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            dbgstr::Cdbgpwrupack,
            dbgstr::Cdbgpwrupack,
            Dbgstr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dbgstr {
    #[inline(always)]
    fn default() -> Dbgstr {
        <crate::RegValueT<Dbgstr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dbgstr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdbgpwrupreq_SPEC;
    pub type Cdbgpwrupreq = crate::EnumBitfieldStruct<u8, Cdbgpwrupreq_SPEC>;
    impl Cdbgpwrupreq {
        #[doc = "OCD is not requesting debug power-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "OCD is requesting debug power-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdbgpwrupack_SPEC;
    pub type Cdbgpwrupack = crate::EnumBitfieldStruct<u8, Cdbgpwrupack_SPEC>;
    impl Cdbgpwrupack {
        #[doc = "Debug power-up request is not acknowledged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Debug power-up request is acknowledged"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgstopcr_SPEC;
impl crate::sealed::RegSpec for Dbgstopcr_SPEC {
    type DataType = u32;
}

#[doc = "Debug Stop Control Register"]
pub type Dbgstopcr = crate::RegValueT<Dbgstopcr_SPEC>;

impl Dbgstopcr {
    #[doc = "Mask bit for IWDT reset/interrupt in the OCD run mode"]
    #[inline(always)]
    pub fn dbgstop_iwdt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dbgstopcr::DbgstopIwdt,
        dbgstopcr::DbgstopIwdt,
        Dbgstopcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dbgstopcr::DbgstopIwdt,
            dbgstopcr::DbgstopIwdt,
            Dbgstopcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mask bit for WDT reset/interrupt in the OCD run mode"]
    #[inline(always)]
    pub fn dbgstop_wdt0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dbgstopcr::DbgstopWdt0,
        dbgstopcr::DbgstopWdt0,
        Dbgstopcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dbgstopcr::DbgstopWdt0,
            dbgstopcr::DbgstopWdt0,
            Dbgstopcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mask bit for PVDn (n = 1, 2) reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_pvd(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        dbgstopcr::DbgstopPvd,
        dbgstopcr::DbgstopPvd,
        Dbgstopcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            dbgstopcr::DbgstopPvd,
            dbgstopcr::DbgstopPvd,
            Dbgstopcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dbgstop_rer(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        dbgstopcr::DbgstopRer,
        dbgstopcr::DbgstopRer,
        Dbgstopcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            dbgstopcr::DbgstopRer,
            dbgstopcr::DbgstopRer,
            Dbgstopcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dbgstopcr {
    #[inline(always)]
    fn default() -> Dbgstopcr {
        <crate::RegValueT<Dbgstopcr_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod dbgstopcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgstopIwdt_SPEC;
    pub type DbgstopIwdt = crate::EnumBitfieldStruct<u8, DbgstopIwdt_SPEC>;
    impl DbgstopIwdt {
        #[doc = "Enable IWDT reset/interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mask IWDT reset/interrupt and stop IWDT counter"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgstopWdt0_SPEC;
    pub type DbgstopWdt0 = crate::EnumBitfieldStruct<u8, DbgstopWdt0_SPEC>;
    impl DbgstopWdt0 {
        #[doc = "Enable WDT reset/interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mask WDT reset/interrupt and stop WDT counter"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgstopPvd_SPEC;
    pub type DbgstopPvd = crate::EnumBitfieldStruct<u8, DbgstopPvd_SPEC>;
    impl DbgstopPvd {
        #[doc = "Enable PVDn (n = 1, 2) reset/interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mask PVDn (n = 1, 2) reset/interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgstopRer_SPEC;
    pub type DbgstopRer = crate::EnumBitfieldStruct<u8, DbgstopRer_SPEC>;
    impl DbgstopRer {
        #[doc = "Enable SRAM parity error reset/interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mask SRAM parity error reset/interrupt"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgauth0_SPEC;
impl crate::sealed::RegSpec for Dbgauth0_SPEC {
    type DataType = u32;
}

#[doc = "Debug Authentication Control Register 0"]
pub type Dbgauth0 = crate::RegValueT<Dbgauth0_SPEC>;

impl Dbgauth0 {
    #[doc = "CPU invasive debug enable"]
    #[inline(always)]
    pub fn dbgen0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dbgauth0::Dbgen0,
        dbgauth0::Dbgen0,
        Dbgauth0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dbgauth0::Dbgen0,
            dbgauth0::Dbgen0,
            Dbgauth0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU non-invasive debug enable"]
    #[inline(always)]
    pub fn niden0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dbgauth0::Niden0,
        dbgauth0::Niden0,
        Dbgauth0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dbgauth0::Niden0,
            dbgauth0::Niden0,
            Dbgauth0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU AHB-AP (AP0) debug enable"]
    #[inline(always)]
    pub fn dbgenap(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dbgauth0::Dbgenap,
        dbgauth0::Dbgenap,
        Dbgauth0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dbgauth0::Dbgenap,
            dbgauth0::Dbgenap,
            Dbgauth0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "APB-AP (AP1) authentication"]
    #[inline(always)]
    pub fn deviceen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dbgauth0::Deviceen,
        dbgauth0::Deviceen,
        Dbgauth0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dbgauth0::Deviceen,
            dbgauth0::Deviceen,
            Dbgauth0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software control of debug function"]
    #[inline(always)]
    pub fn swdbg(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        dbgauth0::Swdbg,
        dbgauth0::Swdbg,
        Dbgauth0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            dbgauth0::Swdbg,
            dbgauth0::Swdbg,
            Dbgauth0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dbgauth0 {
    #[inline(always)]
    fn default() -> Dbgauth0 {
        <crate::RegValueT<Dbgauth0_SPEC> as RegisterValue<_>>::new(65536)
    }
}
pub mod dbgauth0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbgen0_SPEC;
    pub type Dbgen0 = crate::EnumBitfieldStruct<u8, Dbgen0_SPEC>;
    impl Dbgen0 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Niden0_SPEC;
    pub type Niden0 = crate::EnumBitfieldStruct<u8, Niden0_SPEC>;
    impl Niden0 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbgenap_SPEC;
    pub type Dbgenap = crate::EnumBitfieldStruct<u8, Dbgenap_SPEC>;
    impl Dbgenap {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Deviceen_SPEC;
    pub type Deviceen = crate::EnumBitfieldStruct<u8, Deviceen_SPEC>;
    impl Deviceen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swdbg_SPEC;
    pub type Swdbg = crate::EnumBitfieldStruct<u8, Swdbg_SPEC>;
    impl Swdbg {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgauth1_SPEC;
impl crate::sealed::RegSpec for Dbgauth1_SPEC {
    type DataType = u32;
}

#[doc = "Debug Authentication Control Register 1"]
pub type Dbgauth1 = crate::RegValueT<Dbgauth1_SPEC>;

impl Dbgauth1 {
    #[doc = "CPU AHB-AP (AP0) debug enable"]
    #[inline(always)]
    pub fn spidenap(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dbgauth1::Spidenap,
        dbgauth1::Spidenap,
        Dbgauth1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dbgauth1::Spidenap,
            dbgauth1::Spidenap,
            Dbgauth1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dbgauth1 {
    #[inline(always)]
    fn default() -> Dbgauth1 {
        <crate::RegValueT<Dbgauth1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dbgauth1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spidenap_SPEC;
    pub type Spidenap = crate::EnumBitfieldStruct<u8, Spidenap_SPEC>;
    impl Spidenap {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trportcr_SPEC;
impl crate::sealed::RegSpec for Trportcr_SPEC {
    type DataType = u32;
}

#[doc = "Trace Port Control Register"]
pub type Trportcr = crate::RegValueT<Trportcr_SPEC>;

impl Trportcr {
    #[doc = "Data Out Enable bit indicates whether Trace Clock, Trace Data, and SWO outputs are enabled."]
    #[inline(always)]
    pub fn oe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        trportcr::Oe,
        trportcr::Oe,
        Trportcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            trportcr::Oe,
            trportcr::Oe,
            Trportcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Drive Capability Control indicates trace port buffer speed:"]
    #[inline(always)]
    pub fn drv(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        trportcr::Drv,
        trportcr::Drv,
        Trportcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            trportcr::Drv,
            trportcr::Drv,
            Trportcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Trportcr {
    #[inline(always)]
    fn default() -> Trportcr {
        <crate::RegValueT<Trportcr_SPEC> as RegisterValue<_>>::new(4)
    }
}
pub mod trportcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oe_SPEC;
    pub type Oe = crate::EnumBitfieldStruct<u8, Oe_SPEC>;
    impl Oe {
        #[doc = "Output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drv_SPEC;
    pub type Drv = crate::EnumBitfieldStruct<u8, Drv_SPEC>;
    impl Drv {
        #[doc = "Low-drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle-drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High-drive"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cachedbgcr_SPEC;
impl crate::sealed::RegSpec for Cachedbgcr_SPEC {
    type DataType = u32;
}

#[doc = "Cache Debug Control Register"]
pub type Cachedbgcr = crate::RegValueT<Cachedbgcr_SPEC>;

impl Cachedbgcr {
    #[doc = "Disable L1 cache automatic invalidation"]
    #[inline(always)]
    pub fn l1rstdis(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cachedbgcr::L1Rstdis,
        cachedbgcr::L1Rstdis,
        Cachedbgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cachedbgcr::L1Rstdis,
            cachedbgcr::L1Rstdis,
            Cachedbgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cachedbgcr {
    #[inline(always)]
    fn default() -> Cachedbgcr {
        <crate::RegValueT<Cachedbgcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cachedbgcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Rstdis_SPEC;
    pub type L1Rstdis = crate::EnumBitfieldStruct<u8, L1Rstdis_SPEC>;
    impl L1Rstdis {
        #[doc = "Enable automatic invalidation of the L1 cache"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable automatic invalidation of the L1 cache"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgmocoen_SPEC;
impl crate::sealed::RegSpec for Dbgmocoen_SPEC {
    type DataType = u32;
}

#[doc = "MOCO Enable Request Register for Debug"]
pub type Dbgmocoen = crate::RegValueT<Dbgmocoen_SPEC>;

impl Dbgmocoen {
    #[doc = "MOCO enable request"]
    #[inline(always)]
    pub fn mocoen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dbgmocoen::Mocoen,
        dbgmocoen::Mocoen,
        Dbgmocoen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dbgmocoen::Mocoen,
            dbgmocoen::Mocoen,
            Dbgmocoen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dbgmocoen {
    #[inline(always)]
    fn default() -> Dbgmocoen {
        <crate::RegValueT<Dbgmocoen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dbgmocoen {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mocoen_SPEC;
    pub type Mocoen = crate::EnumBitfieldStruct<u8, Mocoen_SPEC>;
    impl Mocoen {
        #[doc = "No request MOCO enable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request MOCO enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgfclksel_SPEC;
impl crate::sealed::RegSpec for Dbgfclksel_SPEC {
    type DataType = u32;
}

#[doc = "Flash Sequencer Clock Select Register for Debug"]
pub type Dbgfclksel = crate::RegValueT<Dbgfclksel_SPEC>;

impl Dbgfclksel {
    #[doc = "Flash sequencer clock select"]
    #[inline(always)]
    pub fn fclksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dbgfclksel::Fclksel,
        dbgfclksel::Fclksel,
        Dbgfclksel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dbgfclksel::Fclksel,
            dbgfclksel::Fclksel,
            Dbgfclksel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dbgfclksel {
    #[inline(always)]
    fn default() -> Dbgfclksel {
        <crate::RegValueT<Dbgfclksel_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dbgfclksel {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fclksel_SPEC;
    pub type Fclksel = crate::EnumBitfieldStruct<u8, Fclksel_SPEC>;
    impl Fclksel {
        #[doc = "FCLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "MOCO"]
        pub const _1: Self = Self::new(1);
    }
}
