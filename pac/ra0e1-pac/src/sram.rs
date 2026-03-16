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
// Generated from SVD 1.10.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:00:13 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"SRAM Control"]
unsafe impl ::core::marker::Send for super::Sram {}
unsafe impl ::core::marker::Sync for super::Sram {}
impl super::Sram {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "SRAM Parity Error Operation After Detection Register"]
    #[inline(always)]
    pub const fn parioad(
        &self,
    ) -> &'static crate::common::Reg<self::Parioad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Parioad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "SRAM Protection Register"]
    #[inline(always)]
    pub const fn sramprcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sramprcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramprcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Parioad_SPEC;
impl crate::sealed::RegSpec for Parioad_SPEC {
    type DataType = u8;
}

#[doc = "SRAM Parity Error Operation After Detection Register"]
pub type Parioad = crate::RegValueT<Parioad_SPEC>;

impl Parioad {
    #[doc = "Operation After Detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        parioad::Oad,
        parioad::Oad,
        Parioad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            parioad::Oad,
            parioad::Oad,
            Parioad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Parioad {
    #[inline(always)]
    fn default() -> Parioad {
        <crate::RegValueT<Parioad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod parioad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramprcr_SPEC;
impl crate::sealed::RegSpec for Sramprcr_SPEC {
    type DataType = u8;
}

#[doc = "SRAM Protection Register"]
pub type Sramprcr = crate::RegValueT<Sramprcr_SPEC>;

impl Sramprcr {
    #[doc = "Register Write Control"]
    #[inline(always)]
    pub fn sramprcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramprcr::Sramprcr,
        sramprcr::Sramprcr,
        Sramprcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramprcr::Sramprcr,
            sramprcr::Sramprcr,
            Sramprcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write Key Code"]
    #[inline(always)]
    pub fn kw(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sramprcr_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sramprcr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Sramprcr {
    #[inline(always)]
    fn default() -> Sramprcr {
        <crate::RegValueT<Sramprcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramprcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramprcr_SPEC;
    pub type Sramprcr = crate::EnumBitfieldStruct<u8, Sramprcr_SPEC>;
    impl Sramprcr {
        #[doc = "Disable writes to protected registers"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes to protected registers"]
        pub const _1: Self = Self::new(1);
    }
}
