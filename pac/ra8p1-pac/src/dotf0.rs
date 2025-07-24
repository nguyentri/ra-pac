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
#[doc = r"DOTF0"]
unsafe impl ::core::marker::Send for super::Dotf0 {}
unsafe impl ::core::marker::Sync for super::Dotf0 {}
impl super::Dotf0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "DOTF Conversion Area Start Address Register"]
    #[inline(always)]
    pub const fn convareast(
        &self,
    ) -> &'static crate::common::Reg<self::Convareast_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Convareast_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "DOTF Conversion Area End Address Register"]
    #[inline(always)]
    pub const fn convaread(
        &self,
    ) -> &'static crate::common::Reg<self::Convaread_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Convaread_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Convareast_SPEC;
impl crate::sealed::RegSpec for Convareast_SPEC {
    type DataType = u32;
}

#[doc = "DOTF Conversion Area Start Address Register"]
pub type Convareast = crate::RegValueT<Convareast_SPEC>;

impl Convareast {
    #[doc = "The first address of the decryption processing area. The actual address is CONVAREAST\\[31:12\\] × 0x1000."]
    #[inline(always)]
    pub fn convareast(
        self,
    ) -> crate::common::RegisterField<12, 0xfffff, 1, 0, u32, u32, Convareast_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            12,
            0xfffff,
            1,
            0,
            u32,
            u32,
            Convareast_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Convareast {
    #[inline(always)]
    fn default() -> Convareast {
        <crate::RegValueT<Convareast_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Convaread_SPEC;
impl crate::sealed::RegSpec for Convaread_SPEC {
    type DataType = u32;
}

#[doc = "DOTF Conversion Area End Address Register"]
pub type Convaread = crate::RegValueT<Convaread_SPEC>;

impl Convaread {
    #[doc = "The end address of the decryption processing area. The actual address is CONVAREAED\\[31:12\\] ×0x1000."]
    #[inline(always)]
    pub fn convareaed(
        self,
    ) -> crate::common::RegisterField<12, 0xfffff, 1, 0, u32, u32, Convaread_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xfffff,1,0,u32,u32,Convaread_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Convaread {
    #[inline(always)]
    fn default() -> Convaread {
        <crate::RegValueT<Convaread_SPEC> as RegisterValue<_>>::new(0)
    }
}
