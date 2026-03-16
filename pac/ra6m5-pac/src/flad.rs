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
// Generated from SVD 1.30.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:13:42 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Data Flash"]
unsafe impl ::core::marker::Send for super::Flad {}
unsafe impl ::core::marker::Sync for super::Flad {}
impl super::Flad {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Data Flash Access Frequency Register"]
    #[inline(always)]
    pub const fn fckmhz(
        &self,
    ) -> &'static crate::common::Reg<self::Fckmhz_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fckmhz_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fckmhz_SPEC;
impl crate::sealed::RegSpec for Fckmhz_SPEC {
    type DataType = u8;
}

#[doc = "Data Flash Access Frequency Register"]
pub type Fckmhz = crate::RegValueT<Fckmhz_SPEC>;

impl Fckmhz {
    #[doc = "Data Flash Access Frequency Register"]
    #[inline(always)]
    pub fn fckmhz(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fckmhz_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fckmhz_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fckmhz {
    #[inline(always)]
    fn default() -> Fckmhz {
        <crate::RegValueT<Fckmhz_SPEC> as RegisterValue<_>>::new(60)
    }
}
