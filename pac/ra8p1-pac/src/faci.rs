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
// Generated from SVD 1.00.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:55:39 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Flash/CPU Interface"]
unsafe impl ::core::marker::Send for super::Faci {}
unsafe impl ::core::marker::Sync for super::Faci {}
impl super::Faci {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Flash Block Protection Register"]
    #[inline(always)]
    pub const fn fbprot0(
        &self,
    ) -> &'static crate::common::Reg<self::Fbprot0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fbprot0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "Flash Block Protection for Secure Register"]
    #[inline(always)]
    pub const fn fbprot1(
        &self,
    ) -> &'static crate::common::Reg<self::Fbprot1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fbprot1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbprot0_SPEC;
impl crate::sealed::RegSpec for Fbprot0_SPEC {
    type DataType = u16;
}

#[doc = "Flash Block Protection Register"]
pub type Fbprot0 = crate::RegValueT<Fbprot0_SPEC>;

impl Fbprot0 {
    #[doc = "Block Protection for Non-secure Cancel"]
    #[inline(always)]
    pub fn bpcn0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fbprot0::Bpcn0,
        fbprot0::Bpcn0,
        Fbprot0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fbprot0::Bpcn0,
            fbprot0::Bpcn0,
            Fbprot0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fbprot0_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fbprot0_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fbprot0 {
    #[inline(always)]
    fn default() -> Fbprot0 {
        <crate::RegValueT<Fbprot0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fbprot0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpcn0_SPEC;
    pub type Bpcn0 = crate::EnumBitfieldStruct<u8, Bpcn0_SPEC>;
    impl Bpcn0 {
        #[doc = "Block protection is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Block protection is disabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbprot1_SPEC;
impl crate::sealed::RegSpec for Fbprot1_SPEC {
    type DataType = u16;
}

#[doc = "Flash Block Protection for Secure Register"]
pub type Fbprot1 = crate::RegValueT<Fbprot1_SPEC>;

impl Fbprot1 {
    #[doc = "Block Protection for Secure Cancel"]
    #[inline(always)]
    pub fn bpcn1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fbprot1::Bpcn1,
        fbprot1::Bpcn1,
        Fbprot1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fbprot1::Bpcn1,
            fbprot1::Bpcn1,
            Fbprot1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fbprot1_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fbprot1_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fbprot1 {
    #[inline(always)]
    fn default() -> Fbprot1 {
        <crate::RegValueT<Fbprot1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fbprot1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpcn1_SPEC;
    pub type Bpcn1 = crate::EnumBitfieldStruct<u8, Bpcn1_SPEC>;
    impl Bpcn1 {
        #[doc = "Block protection is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Block protection is disabled."]
        pub const _1: Self = Self::new(1);
    }
}
