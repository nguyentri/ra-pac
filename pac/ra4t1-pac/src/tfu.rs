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
#[doc = r"Trigonometric Function Unit"]
unsafe impl ::core::marker::Send for super::Tfu {}
unsafe impl ::core::marker::Sync for super::Tfu {}
impl super::Tfu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Trigonometric Status Register"]
    #[inline(always)]
    pub const fn trgsts(&self) -> &'static crate::common::Reg<self::Trgsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Trgsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Sine Cosine Data Register 0"]
    #[inline(always)]
    pub const fn scdt0(&self) -> &'static crate::common::Reg<self::Scdt0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scdt0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Sine Cosine Data Register 1"]
    #[inline(always)]
    pub const fn scdt1(&self) -> &'static crate::common::Reg<self::Scdt1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scdt1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Arctangent Data Register 0"]
    #[inline(always)]
    pub const fn atdt0(&self) -> &'static crate::common::Reg<self::Atdt0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Atdt0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Arctangent Data Register 1"]
    #[inline(always)]
    pub const fn atdt1(&self) -> &'static crate::common::Reg<self::Atdt1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Atdt1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trgsts_SPEC;
impl crate::sealed::RegSpec for Trgsts_SPEC {
    type DataType = u8;
}

#[doc = "Trigonometric Status Register"]
pub type Trgsts = crate::RegValueT<Trgsts_SPEC>;

impl Trgsts {
    #[doc = "Calculation in progress flag"]
    #[inline(always)]
    pub fn bsyf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        trgsts::Bsyf,
        trgsts::Bsyf,
        Trgsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            trgsts::Bsyf,
            trgsts::Bsyf,
            Trgsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Input error flag"]
    #[inline(always)]
    pub fn errf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        trgsts::Errf,
        trgsts::Errf,
        Trgsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            trgsts::Errf,
            trgsts::Errf,
            Trgsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Trgsts {
    #[inline(always)]
    fn default() -> Trgsts {
        <crate::RegValueT<Trgsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trgsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsyf_SPEC;
    pub type Bsyf = crate::EnumBitfieldStruct<u8, Bsyf_SPEC>;
    impl Bsyf {
        #[doc = "No calculating"]
        pub const _0: Self = Self::new(0);

        #[doc = "Calculating"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Errf_SPEC;
    pub type Errf = crate::EnumBitfieldStruct<u8, Errf_SPEC>;
    impl Errf {
        #[doc = "No input error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Input error occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scdt0_SPEC;
impl crate::sealed::RegSpec for Scdt0_SPEC {
    type DataType = u32;
}

#[doc = "Sine Cosine Data Register 0"]
pub type Scdt0 = crate::RegValueT<Scdt0_SPEC>;

impl Scdt0 {
    #[doc = "Sine Cosine Data Register 0 (single-precision floating-point)"]
    #[inline(always)]
    pub fn scdt0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Scdt0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Scdt0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Scdt0 {
    #[inline(always)]
    fn default() -> Scdt0 {
        <crate::RegValueT<Scdt0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scdt1_SPEC;
impl crate::sealed::RegSpec for Scdt1_SPEC {
    type DataType = u32;
}

#[doc = "Sine Cosine Data Register 1"]
pub type Scdt1 = crate::RegValueT<Scdt1_SPEC>;

impl Scdt1 {
    #[doc = "Sine Cosine Data Register 1 (single-precision floating-point)"]
    #[inline(always)]
    pub fn scdt1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Scdt1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Scdt1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Scdt1 {
    #[inline(always)]
    fn default() -> Scdt1 {
        <crate::RegValueT<Scdt1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atdt0_SPEC;
impl crate::sealed::RegSpec for Atdt0_SPEC {
    type DataType = u32;
}

#[doc = "Arctangent Data Register 0"]
pub type Atdt0 = crate::RegValueT<Atdt0_SPEC>;

impl Atdt0 {
    #[doc = "Arctangent Data Register 0 (single-precision floating-point)"]
    #[inline(always)]
    pub fn atdt0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Atdt0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Atdt0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Atdt0 {
    #[inline(always)]
    fn default() -> Atdt0 {
        <crate::RegValueT<Atdt0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atdt1_SPEC;
impl crate::sealed::RegSpec for Atdt1_SPEC {
    type DataType = u32;
}

#[doc = "Arctangent Data Register 1"]
pub type Atdt1 = crate::RegValueT<Atdt1_SPEC>;

impl Atdt1 {
    #[doc = "Arctangent Data Register 1 (single-precision floating-point)"]
    #[inline(always)]
    pub fn atdt1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Atdt1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Atdt1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Atdt1 {
    #[inline(always)]
    fn default() -> Atdt1 {
        <crate::RegValueT<Atdt1_SPEC> as RegisterValue<_>>::new(0)
    }
}
