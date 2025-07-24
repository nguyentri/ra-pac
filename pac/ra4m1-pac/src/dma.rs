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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:49:18 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"DMAC Module Activation"]
unsafe impl ::core::marker::Send for super::Dma {}
unsafe impl ::core::marker::Sync for super::Dma {}
impl super::Dma {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "DMAC Module Activation Register"]
    #[inline(always)]
    pub const fn dmast(&self) -> &'static crate::common::Reg<self::Dmast_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmast_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmast_SPEC;
impl crate::sealed::RegSpec for Dmast_SPEC {
    type DataType = u8;
}

#[doc = "DMAC Module Activation Register"]
pub type Dmast = crate::RegValueT<Dmast_SPEC>;

impl Dmast {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Dmast_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Dmast_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DMAC Operation Enable"]
    #[inline(always)]
    pub fn dmst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmast::Dmst,
        dmast::Dmst,
        Dmast_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmast::Dmst,
            dmast::Dmst,
            Dmast_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmast {
    #[inline(always)]
    fn default() -> Dmast {
        <crate::RegValueT<Dmast_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmast {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dmst_SPEC;
    pub type Dmst = crate::EnumBitfieldStruct<u8, Dmst_SPEC>;
    impl Dmst {
        #[doc = "Disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled."]
        pub const _1: Self = Self::new(1);
    }
}
