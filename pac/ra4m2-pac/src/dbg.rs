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
// Generated from SVD 1.30.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:07:12 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Debug Function"]
unsafe impl ::core::marker::Send for super::Dbg {}
unsafe impl ::core::marker::Sync for super::Dbg {}
impl super::Dbg {
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
        #[doc = "OCD is not requesting debug power up"]
        pub const _0: Self = Self::new(0);

        #[doc = "OCD is requesting debug power up"]
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
    pub fn dbgstop_wdt(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dbgstopcr::DbgstopWdt,
        dbgstopcr::DbgstopWdt,
        Dbgstopcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dbgstopcr::DbgstopWdt,
            dbgstopcr::DbgstopWdt,
            Dbgstopcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mask bit for LVD0 reset"]
    #[inline(always)]
    pub fn dbgstop_lvd0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dbgstopcr::DbgstopLvd0,
        dbgstopcr::DbgstopLvd0,
        Dbgstopcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dbgstopcr::DbgstopLvd0,
            dbgstopcr::DbgstopLvd0,
            Dbgstopcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mask bit for LVD1 reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_lvd1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        dbgstopcr::DbgstopLvd1,
        dbgstopcr::DbgstopLvd1,
        Dbgstopcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            dbgstopcr::DbgstopLvd1,
            dbgstopcr::DbgstopLvd1,
            Dbgstopcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mask bit for LVD2 reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_lvd2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        dbgstopcr::DbgstopLvd2,
        dbgstopcr::DbgstopLvd2,
        Dbgstopcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            dbgstopcr::DbgstopLvd2,
            dbgstopcr::DbgstopLvd2,
            Dbgstopcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mask bit for SRAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_rper(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        dbgstopcr::DbgstopRper,
        dbgstopcr::DbgstopRper,
        Dbgstopcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            dbgstopcr::DbgstopRper,
            dbgstopcr::DbgstopRper,
            Dbgstopcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mask bit for SRAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_reccr(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        dbgstopcr::DbgstopReccr,
        dbgstopcr::DbgstopReccr,
        Dbgstopcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            dbgstopcr::DbgstopReccr,
            dbgstopcr::DbgstopReccr,
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
    pub struct DbgstopWdt_SPEC;
    pub type DbgstopWdt = crate::EnumBitfieldStruct<u8, DbgstopWdt_SPEC>;
    impl DbgstopWdt {
        #[doc = "Enable WDT reset/interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mask WDT reset/interrupt and stop WDT counter"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgstopLvd0_SPEC;
    pub type DbgstopLvd0 = crate::EnumBitfieldStruct<u8, DbgstopLvd0_SPEC>;
    impl DbgstopLvd0 {
        #[doc = "Enable LVD0 reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mask LVD0 reset"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgstopLvd1_SPEC;
    pub type DbgstopLvd1 = crate::EnumBitfieldStruct<u8, DbgstopLvd1_SPEC>;
    impl DbgstopLvd1 {
        #[doc = "Enable LVD1 reset/interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mask LVD1 reset/interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgstopLvd2_SPEC;
    pub type DbgstopLvd2 = crate::EnumBitfieldStruct<u8, DbgstopLvd2_SPEC>;
    impl DbgstopLvd2 {
        #[doc = "Enable LVD2 reset/interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mask LVD2 reset/interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgstopRper_SPEC;
    pub type DbgstopRper = crate::EnumBitfieldStruct<u8, DbgstopRper_SPEC>;
    impl DbgstopRper {
        #[doc = "Enable SRAM parity error reset/interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mask SRAM parity error reset/interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgstopReccr_SPEC;
    pub type DbgstopReccr = crate::EnumBitfieldStruct<u8, DbgstopReccr_SPEC>;
    impl DbgstopReccr {
        #[doc = "Enable SRAM ECC error reset/interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mask SRAM ECC error reset/interrupt"]
        pub const _1: Self = Self::new(1);
    }
}
