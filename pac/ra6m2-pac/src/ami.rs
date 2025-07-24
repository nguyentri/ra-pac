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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:51:16 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"ADC-DAC Interface"]
unsafe impl ::core::marker::Send for super::Ami {}
unsafe impl ::core::marker::Sync for super::Ami {}
impl super::Ami {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "D/A A/D Synchronous Unit Select Register"]
    #[inline(always)]
    pub const fn daadusr(
        &self,
    ) -> &'static crate::common::Reg<self::Daadusr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Daadusr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Daadusr_SPEC;
impl crate::sealed::RegSpec for Daadusr_SPEC {
    type DataType = u8;
}

#[doc = "D/A A/D Synchronous Unit Select Register"]
pub type Daadusr = crate::RegValueT<Daadusr_SPEC>;

impl Daadusr {
    #[doc = "A/D Unit 1 Select"]
    #[inline(always)]
    pub fn amadsel1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        daadusr::Amadsel1,
        daadusr::Amadsel1,
        Daadusr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            daadusr::Amadsel1,
            daadusr::Amadsel1,
            Daadusr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Daadusr {
    #[inline(always)]
    fn default() -> Daadusr {
        <crate::RegValueT<Daadusr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod daadusr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amadsel1_SPEC;
    pub type Amadsel1 = crate::EnumBitfieldStruct<u8, Amadsel1_SPEC>;
    impl Amadsel1 {
        #[doc = "Unit 1 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unit 1 is selected."]
        pub const _1: Self = Self::new(1);
    }
}
