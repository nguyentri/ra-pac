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
// Generated from SVD 1.20.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:50:08 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"TrustZone Filter"]
unsafe impl ::core::marker::Send for super::Tzf {}
unsafe impl ::core::marker::Sync for super::Tzf {}
impl super::Tzf {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "TrustZone Filter Operation After Detection Register"]
    #[inline(always)]
    pub const fn tzfoad(
        &self,
    ) -> &'static crate::common::Reg<self::Tzfoad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tzfoad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "TrustZone Filter Protect Register"]
    #[inline(always)]
    pub const fn tzfpt(&self) -> &'static crate::common::Reg<self::Tzfpt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tzfpt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tzfoad_SPEC;
impl crate::sealed::RegSpec for Tzfoad_SPEC {
    type DataType = u16;
}

#[doc = "TrustZone Filter Operation After Detection Register"]
pub type Tzfoad = crate::RegValueT<Tzfoad_SPEC>;

impl Tzfoad {
    #[doc = "Operation after detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tzfoad::Oad,
        tzfoad::Oad,
        Tzfoad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tzfoad::Oad,
            tzfoad::Oad,
            Tzfoad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "KeyCode"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Tzfoad_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Tzfoad_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Tzfoad {
    #[inline(always)]
    fn default() -> Tzfoad {
        <crate::RegValueT<Tzfoad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tzfoad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tzfpt_SPEC;
impl crate::sealed::RegSpec for Tzfpt_SPEC {
    type DataType = u16;
}

#[doc = "TrustZone Filter Protect Register"]
pub type Tzfpt = crate::RegValueT<Tzfpt_SPEC>;

impl Tzfpt {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tzfpt::Protect,
        tzfpt::Protect,
        Tzfpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tzfpt::Protect,
            tzfpt::Protect,
            Tzfpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "KeyCode"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Tzfpt_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Tzfpt_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Tzfpt {
    #[inline(always)]
    fn default() -> Tzfpt {
        <crate::RegValueT<Tzfpt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tzfpt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "All Bus TrustZone Filter register writing is protected. Read is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "All Bus TrustZone Filter register writing is possible."]
        pub const _1: Self = Self::new(1);
    }
}
