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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:06:30 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Data Transfer Controller"]
unsafe impl ::core::marker::Send for super::Dtc {}
unsafe impl ::core::marker::Sync for super::Dtc {}
impl super::Dtc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "DTC Control Register"]
    #[inline(always)]
    pub const fn dtccr(&self) -> &'static crate::common::Reg<self::Dtccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dtccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "DTC Vector Base Register"]
    #[inline(always)]
    pub const fn dtcvbr(
        &self,
    ) -> &'static crate::common::Reg<self::Dtcvbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dtcvbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "DTC Module Start Register"]
    #[inline(always)]
    pub const fn dtcst(&self) -> &'static crate::common::Reg<self::Dtcst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dtcst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "DTC Status Register"]
    #[inline(always)]
    pub const fn dtcsts(&self) -> &'static crate::common::Reg<self::Dtcsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dtcsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtccr_SPEC;
impl crate::sealed::RegSpec for Dtccr_SPEC {
    type DataType = u8;
}

#[doc = "DTC Control Register"]
pub type Dtccr = crate::RegValueT<Dtccr_SPEC>;

impl Dtccr {
    #[doc = "DTC Transfer Information Read Skip Enable."]
    #[inline(always)]
    pub fn rrs(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dtccr::Rrs,
        dtccr::Rrs,
        Dtccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dtccr::Rrs,
            dtccr::Rrs,
            Dtccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Dtccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Dtccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dtccr {
    #[inline(always)]
    fn default() -> Dtccr {
        <crate::RegValueT<Dtccr_SPEC> as RegisterValue<_>>::new(8)
    }
}
pub mod dtccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rrs_SPEC;
    pub type Rrs = crate::EnumBitfieldStruct<u8, Rrs_SPEC>;
    impl Rrs {
        #[doc = "Do not skip transfer information read"]
        pub const _0: Self = Self::new(0);

        #[doc = "Skip transfer information read when vector numbers match"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcvbr_SPEC;
impl crate::sealed::RegSpec for Dtcvbr_SPEC {
    type DataType = u32;
}

#[doc = "DTC Vector Base Register"]
pub type Dtcvbr = crate::RegValueT<Dtcvbr_SPEC>;

impl Dtcvbr {
    #[doc = "DTC Vector Base Address.\nNote: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
    #[inline(always)]
    pub fn dtcvbr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Dtcvbr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Dtcvbr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dtcvbr {
    #[inline(always)]
    fn default() -> Dtcvbr {
        <crate::RegValueT<Dtcvbr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcst_SPEC;
impl crate::sealed::RegSpec for Dtcst_SPEC {
    type DataType = u8;
}

#[doc = "DTC Module Start Register"]
pub type Dtcst = crate::RegValueT<Dtcst_SPEC>;

impl Dtcst {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Dtcst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Dtcst_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DTC Module Start"]
    #[inline(always)]
    pub fn dtcst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dtcst::Dtcst,
        dtcst::Dtcst,
        Dtcst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dtcst::Dtcst,
            dtcst::Dtcst,
            Dtcst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dtcst {
    #[inline(always)]
    fn default() -> Dtcst {
        <crate::RegValueT<Dtcst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dtcst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtcst_SPEC;
    pub type Dtcst = crate::EnumBitfieldStruct<u8, Dtcst_SPEC>;
    impl Dtcst {
        #[doc = "DTC module stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC module start"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcsts_SPEC;
impl crate::sealed::RegSpec for Dtcsts_SPEC {
    type DataType = u16;
}

#[doc = "DTC Status Register"]
pub type Dtcsts = crate::RegValueT<Dtcsts_SPEC>;

impl Dtcsts {
    #[doc = "DTC Active Flag"]
    #[inline(always)]
    pub fn act(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        dtcsts::Act,
        dtcsts::Act,
        Dtcsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            dtcsts::Act,
            dtcsts::Act,
            Dtcsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, Dtcsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,Dtcsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DTC-Activating Vector Number Monitoring\nThese bits indicate the vector number for the activating source when DTC transfer is in progress.\nThe value is only valid if DTC transfer is in progress (the value of the ACT flag is 1)"]
    #[inline(always)]
    pub fn vecn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dtcsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dtcsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dtcsts {
    #[inline(always)]
    fn default() -> Dtcsts {
        <crate::RegValueT<Dtcsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dtcsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Act_SPEC;
    pub type Act = crate::EnumBitfieldStruct<u8, Act_SPEC>;
    impl Act {
        #[doc = "DTC transfer operation is not in progress."]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC transfer operation is in progress."]
        pub const _1: Self = Self::new(1);
    }
}
