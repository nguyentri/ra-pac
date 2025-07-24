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
// Generated from SVD 1.10.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:59 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"True Random Number Generator"]
unsafe impl ::core::marker::Send for super::Trng {}
unsafe impl ::core::marker::Sync for super::Trng {}
impl super::Trng {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "TRNG Seed Data Register"]
    #[inline(always)]
    pub const fn trngsdr(
        &self,
    ) -> &'static crate::common::Reg<self::Trngsdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Trngsdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "TRNG Seed Command Register 0"]
    #[inline(always)]
    pub const fn trngscr0(
        &self,
    ) -> &'static crate::common::Reg<self::Trngscr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Trngscr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "TRNG Seed Command Register 1"]
    #[inline(always)]
    pub const fn trngscr1(
        &self,
    ) -> &'static crate::common::Reg<self::Trngscr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Trngscr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trngsdr_SPEC;
impl crate::sealed::RegSpec for Trngsdr_SPEC {
    type DataType = u8;
}

#[doc = "TRNG Seed Data Register"]
pub type Trngsdr = crate::RegValueT<Trngsdr_SPEC>;

impl NoBitfieldReg<Trngsdr_SPEC> for Trngsdr {}
impl ::core::default::Default for Trngsdr {
    #[inline(always)]
    fn default() -> Trngsdr {
        <crate::RegValueT<Trngsdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trngscr0_SPEC;
impl crate::sealed::RegSpec for Trngscr0_SPEC {
    type DataType = u8;
}

#[doc = "TRNG Seed Command Register 0"]
pub type Trngscr0 = crate::RegValueT<Trngscr0_SPEC>;

impl Trngscr0 {
    #[doc = "Seed Generation Start"]
    #[inline(always)]
    pub fn sgstart(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        trngscr0::Sgstart,
        trngscr0::Sgstart,
        Trngscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            trngscr0::Sgstart,
            trngscr0::Sgstart,
            Trngscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Seed Generation Circuit Enable"]
    #[inline(always)]
    pub fn sgcen(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        trngscr0::Sgcen,
        trngscr0::Sgcen,
        Trngscr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            trngscr0::Sgcen,
            trngscr0::Sgcen,
            Trngscr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read Ready"]
    #[inline(always)]
    pub fn rdrdy(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Trngscr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Trngscr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Trngscr0 {
    #[inline(always)]
    fn default() -> Trngscr0 {
        <crate::RegValueT<Trngscr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trngscr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgstart_SPEC;
    pub type Sgstart = crate::EnumBitfieldStruct<u8, Sgstart_SPEC>;
    impl Sgstart {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start to generate the seed data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgcen_SPEC;
    pub type Sgcen = crate::EnumBitfieldStruct<u8, Sgcen_SPEC>;
    impl Sgcen {
        #[doc = "Seed generation circuit is disable."]
        pub const _0: Self = Self::new(0);

        #[doc = "Seed generation circuit is enable."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trngscr1_SPEC;
impl crate::sealed::RegSpec for Trngscr1_SPEC {
    type DataType = u8;
}

#[doc = "TRNG Seed Command Register 1"]
pub type Trngscr1 = crate::RegValueT<Trngscr1_SPEC>;

impl Trngscr1 {
    #[doc = "TRNG Interrupt Enable"]
    #[inline(always)]
    pub fn inten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        trngscr1::Inten,
        trngscr1::Inten,
        Trngscr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            trngscr1::Inten,
            trngscr1::Inten,
            Trngscr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Trngscr1 {
    #[inline(always)]
    fn default() -> Trngscr1 {
        <crate::RegValueT<Trngscr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trngscr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inten_SPEC;
    pub type Inten = crate::EnumBitfieldStruct<u8, Inten_SPEC>;
    impl Inten {
        #[doc = "TRNG interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "TRNG interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
