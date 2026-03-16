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
#[doc = r"Temperature Sensor"]
unsafe impl ::core::marker::Send for super::Tsn {}
unsafe impl ::core::marker::Sync for super::Tsn {}
impl super::Tsn {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Temperature Sensor Calibration Data Register H"]
    #[inline(always)]
    pub const fn tscdrh(&self) -> &'static crate::common::Reg<self::Tscdrh_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tscdrh_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(553usize),
            )
        }
    }

    #[doc = "Temperature Sensor Calibration Data Register L"]
    #[inline(always)]
    pub const fn tscdrl(&self) -> &'static crate::common::Reg<self::Tscdrl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tscdrl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(552usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscdrh_SPEC;
impl crate::sealed::RegSpec for Tscdrh_SPEC {
    type DataType = u8;
}

#[doc = "Temperature Sensor Calibration Data Register H"]
pub type Tscdrh = crate::RegValueT<Tscdrh_SPEC>;

impl Tscdrh {
    #[doc = "The calibration data stores the higher 8 bits of the converted\nvalue."]
    #[inline(always)]
    pub fn tscdrh(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Tscdrh_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Tscdrh_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tscdrh {
    #[inline(always)]
    fn default() -> Tscdrh {
        <crate::RegValueT<Tscdrh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscdrl_SPEC;
impl crate::sealed::RegSpec for Tscdrl_SPEC {
    type DataType = u8;
}

#[doc = "Temperature Sensor Calibration Data Register L"]
pub type Tscdrl = crate::RegValueT<Tscdrl_SPEC>;

impl Tscdrl {
    #[doc = "The calibration data stores the lower 8 bits of the converted\nvalue."]
    #[inline(always)]
    pub fn tscdrl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Tscdrl_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Tscdrl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tscdrl {
    #[inline(always)]
    fn default() -> Tscdrl {
        <crate::RegValueT<Tscdrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
