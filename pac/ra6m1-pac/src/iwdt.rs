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

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Independent Watchdog Timer"]
unsafe impl ::core::marker::Send for super::Iwdt {}
unsafe impl ::core::marker::Sync for super::Iwdt {}
impl super::Iwdt {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "IWDT Refresh Register"]
    #[inline(always)]
    pub const fn iwdtrr(
        &self,
    ) -> &'static crate::common::Reg<self::Iwdtrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iwdtrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "IWDT Status Register"]
    #[inline(always)]
    pub const fn iwdtsr(
        &self,
    ) -> &'static crate::common::Reg<self::Iwdtsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iwdtsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdtrr_SPEC;
impl crate::sealed::RegSpec for Iwdtrr_SPEC {
    type DataType = u8;
}

#[doc = "IWDT Refresh Register"]
pub type Iwdtrr = crate::RegValueT<Iwdtrr_SPEC>;

impl Iwdtrr {
    #[doc = "The counter is refreshed by writing 0x00 and then writing 0xFF to this register."]
    #[inline(always)]
    pub fn iwdtrr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Iwdtrr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Iwdtrr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Iwdtrr {
    #[inline(always)]
    fn default() -> Iwdtrr {
        <crate::RegValueT<Iwdtrr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdtsr_SPEC;
impl crate::sealed::RegSpec for Iwdtsr_SPEC {
    type DataType = u16;
}

#[doc = "IWDT Status Register"]
pub type Iwdtsr = crate::RegValueT<Iwdtsr_SPEC>;

impl Iwdtsr {
    #[doc = "Refresh Error Flag"]
    #[inline(always)]
    pub fn refef(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        iwdtsr::Refef,
        iwdtsr::Refef,
        Iwdtsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            iwdtsr::Refef,
            iwdtsr::Refef,
            Iwdtsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Underflow Flag"]
    #[inline(always)]
    pub fn undff(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        iwdtsr::Undff,
        iwdtsr::Undff,
        Iwdtsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            iwdtsr::Undff,
            iwdtsr::Undff,
            Iwdtsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Counter ValueValue counted by the counter"]
    #[inline(always)]
    pub fn cntval(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, Iwdtsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3fff,1,0,u16,u16,Iwdtsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Iwdtsr {
    #[inline(always)]
    fn default() -> Iwdtsr {
        <crate::RegValueT<Iwdtsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iwdtsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Refef_SPEC;
    pub type Refef = crate::EnumBitfieldStruct<u8, Refef_SPEC>;
    impl Refef {
        #[doc = "Refresh error not occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Refresh error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Undff_SPEC;
    pub type Undff = crate::EnumBitfieldStruct<u8, Undff_SPEC>;
    impl Undff {
        #[doc = "Underflow not occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Underflow occurred"]
        pub const _1: Self = Self::new(1);
    }
}
