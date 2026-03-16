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
// Generated from SVD 0.90.02, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:05:50 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Code Flash"]
unsafe impl ::core::marker::Send for super::Cibc {}
unsafe impl ::core::marker::Sync for super::Cibc {}
impl super::Cibc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "CTSU Trimming Register A"]
    #[inline(always)]
    pub const fn ctsutrima(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsutrima_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsutrima_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(420usize),
            )
        }
    }

    #[doc = "CTSU Trimming Register B"]
    #[inline(always)]
    pub const fn ctsutrimb(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsutrimb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsutrimb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(424usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsutrima_SPEC;
impl crate::sealed::RegSpec for Ctsutrima_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Trimming Register A"]
pub type Ctsutrima = crate::RegValueT<Ctsutrima_SPEC>;

impl Ctsutrima {
    #[doc = "CTSU Reference Resistance Adjustment"]
    #[inline(always)]
    pub fn rtrim(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsutrima_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsutrima_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Linearity Adjustment of Offset Current"]
    #[inline(always)]
    pub fn dactrim(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsutrima_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsutrima_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CTSU SUCLK Frequency Adjustment"]
    #[inline(always)]
    pub fn suadjd(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ctsutrima_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ctsutrima_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Coefficient of variation for the reference load resistance"]
    #[inline(always)]
    pub fn tresult4(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ctsutrima_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ctsutrima_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsutrima {
    #[inline(always)]
    fn default() -> Ctsutrima {
        <crate::RegValueT<Ctsutrima_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsutrimb_SPEC;
impl crate::sealed::RegSpec for Ctsutrimb_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Trimming Register B"]
pub type Ctsutrimb = crate::RegValueT<Ctsutrimb_SPEC>;

impl Ctsutrimb {
    #[doc = "The coefficient of variation for the 7.5 kΩ reference load resistance is stored."]
    #[inline(always)]
    pub fn tresult0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsutrimb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsutrimb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The coefficient of variation for the 15 kΩ reference load resistance is stored."]
    #[inline(always)]
    pub fn tresult1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsutrimb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsutrimb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The coefficient of variation for the 30 kΩ reference load resistance is stored."]
    #[inline(always)]
    pub fn tresult2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ctsutrimb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ctsutrimb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The coefficient of variation for the 60 kΩ reference load resistance is stored."]
    #[inline(always)]
    pub fn tresult3(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ctsutrimb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ctsutrimb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsutrimb {
    #[inline(always)]
    fn default() -> Ctsutrimb {
        <crate::RegValueT<Ctsutrimb_SPEC> as RegisterValue<_>>::new(0)
    }
}
