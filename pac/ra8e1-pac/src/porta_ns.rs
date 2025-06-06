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
// Generated from SVD 1.00.01, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:22:22 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Pmn Pin Function Port A Control Registers Pmn Pin Function Control Register"]
unsafe impl ::core::marker::Send for super::PortaNs {}
unsafe impl ::core::marker::Sync for super::PortaNs {}
impl super::PortaNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn pcntr2(&self) -> &'static crate::common::Reg<self::Pcntr2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pcntr2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pidr(&self) -> &'static crate::common::Reg<self::Pidr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pidr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pcntr3(&self) -> &'static crate::common::Reg<self::Pcntr3_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pcntr3_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn posr(&self) -> &'static crate::common::Reg<self::Posr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Posr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn porr(&self) -> &'static crate::common::Reg<self::Porr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Porr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcntr2_SPEC;
impl crate::sealed::RegSpec for Pcntr2_SPEC {
    type DataType = u32;
}

pub type Pcntr2 = crate::RegValueT<Pcntr2_SPEC>;

impl Pcntr2 {
    #[inline(always)]
    pub fn pidr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pcntr2::Pidr00,
        pcntr2::Pidr00,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pcntr2::Pidr00,
            pcntr2::Pidr00,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pcntr2::Pidr01,
        pcntr2::Pidr01,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pcntr2::Pidr01,
            pcntr2::Pidr01,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pcntr2::Pidr02,
        pcntr2::Pidr02,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pcntr2::Pidr02,
            pcntr2::Pidr02,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pcntr2::Pidr03,
        pcntr2::Pidr03,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pcntr2::Pidr03,
            pcntr2::Pidr03,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pcntr2::Pidr04,
        pcntr2::Pidr04,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pcntr2::Pidr04,
            pcntr2::Pidr04,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pcntr2::Pidr05,
        pcntr2::Pidr05,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pcntr2::Pidr05,
            pcntr2::Pidr05,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pcntr2::Pidr06,
        pcntr2::Pidr06,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pcntr2::Pidr06,
            pcntr2::Pidr06,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pcntr2::Pidr07,
        pcntr2::Pidr07,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pcntr2::Pidr07,
            pcntr2::Pidr07,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pcntr2::Pidr08,
        pcntr2::Pidr08,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pcntr2::Pidr08,
            pcntr2::Pidr08,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pcntr2::Pidr09,
        pcntr2::Pidr09,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pcntr2::Pidr09,
            pcntr2::Pidr09,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pcntr2::Pidr10,
        pcntr2::Pidr10,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pcntr2::Pidr10,
            pcntr2::Pidr10,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pcntr2::Pidr11,
        pcntr2::Pidr11,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pcntr2::Pidr11,
            pcntr2::Pidr11,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pcntr2::Pidr12,
        pcntr2::Pidr12,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pcntr2::Pidr12,
            pcntr2::Pidr12,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pcntr2::Pidr13,
        pcntr2::Pidr13,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pcntr2::Pidr13,
            pcntr2::Pidr13,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pcntr2::Pidr14,
        pcntr2::Pidr14,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pcntr2::Pidr14,
            pcntr2::Pidr14,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pcntr2::Pidr15,
        pcntr2::Pidr15,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pcntr2::Pidr15,
            pcntr2::Pidr15,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pcntr2 {
    #[inline(always)]
    fn default() -> Pcntr2 {
        <crate::RegValueT<Pcntr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pcntr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr00_SPEC;
    pub type Pidr00 = crate::EnumBitfieldStruct<u8, Pidr00_SPEC>;
    impl Pidr00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr01_SPEC;
    pub type Pidr01 = crate::EnumBitfieldStruct<u8, Pidr01_SPEC>;
    impl Pidr01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr02_SPEC;
    pub type Pidr02 = crate::EnumBitfieldStruct<u8, Pidr02_SPEC>;
    impl Pidr02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr03_SPEC;
    pub type Pidr03 = crate::EnumBitfieldStruct<u8, Pidr03_SPEC>;
    impl Pidr03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr04_SPEC;
    pub type Pidr04 = crate::EnumBitfieldStruct<u8, Pidr04_SPEC>;
    impl Pidr04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr05_SPEC;
    pub type Pidr05 = crate::EnumBitfieldStruct<u8, Pidr05_SPEC>;
    impl Pidr05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr06_SPEC;
    pub type Pidr06 = crate::EnumBitfieldStruct<u8, Pidr06_SPEC>;
    impl Pidr06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr07_SPEC;
    pub type Pidr07 = crate::EnumBitfieldStruct<u8, Pidr07_SPEC>;
    impl Pidr07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr08_SPEC;
    pub type Pidr08 = crate::EnumBitfieldStruct<u8, Pidr08_SPEC>;
    impl Pidr08 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr09_SPEC;
    pub type Pidr09 = crate::EnumBitfieldStruct<u8, Pidr09_SPEC>;
    impl Pidr09 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr10_SPEC;
    pub type Pidr10 = crate::EnumBitfieldStruct<u8, Pidr10_SPEC>;
    impl Pidr10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr11_SPEC;
    pub type Pidr11 = crate::EnumBitfieldStruct<u8, Pidr11_SPEC>;
    impl Pidr11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr12_SPEC;
    pub type Pidr12 = crate::EnumBitfieldStruct<u8, Pidr12_SPEC>;
    impl Pidr12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr13_SPEC;
    pub type Pidr13 = crate::EnumBitfieldStruct<u8, Pidr13_SPEC>;
    impl Pidr13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr14_SPEC;
    pub type Pidr14 = crate::EnumBitfieldStruct<u8, Pidr14_SPEC>;
    impl Pidr14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr15_SPEC;
    pub type Pidr15 = crate::EnumBitfieldStruct<u8, Pidr15_SPEC>;
    impl Pidr15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr_SPEC;
impl crate::sealed::RegSpec for Pidr_SPEC {
    type DataType = u16;
}

pub type Pidr = crate::RegValueT<Pidr_SPEC>;

impl NoBitfieldReg<Pidr_SPEC> for Pidr {}
impl ::core::default::Default for Pidr {
    #[inline(always)]
    fn default() -> Pidr {
        <crate::RegValueT<Pidr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcntr3_SPEC;
impl crate::sealed::RegSpec for Pcntr3_SPEC {
    type DataType = u32;
}

pub type Pcntr3 = crate::RegValueT<Pcntr3_SPEC>;

impl Pcntr3 {
    #[inline(always)]
    pub fn posr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pcntr3::Posr00,
        pcntr3::Posr00,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pcntr3::Posr00,
            pcntr3::Posr00,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pcntr3::Posr01,
        pcntr3::Posr01,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pcntr3::Posr01,
            pcntr3::Posr01,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pcntr3::Posr02,
        pcntr3::Posr02,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pcntr3::Posr02,
            pcntr3::Posr02,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pcntr3::Posr03,
        pcntr3::Posr03,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pcntr3::Posr03,
            pcntr3::Posr03,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pcntr3::Posr04,
        pcntr3::Posr04,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pcntr3::Posr04,
            pcntr3::Posr04,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pcntr3::Posr05,
        pcntr3::Posr05,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pcntr3::Posr05,
            pcntr3::Posr05,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pcntr3::Posr06,
        pcntr3::Posr06,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pcntr3::Posr06,
            pcntr3::Posr06,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pcntr3::Posr07,
        pcntr3::Posr07,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pcntr3::Posr07,
            pcntr3::Posr07,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pcntr3::Posr08,
        pcntr3::Posr08,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pcntr3::Posr08,
            pcntr3::Posr08,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pcntr3::Posr09,
        pcntr3::Posr09,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pcntr3::Posr09,
            pcntr3::Posr09,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pcntr3::Posr10,
        pcntr3::Posr10,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pcntr3::Posr10,
            pcntr3::Posr10,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pcntr3::Posr11,
        pcntr3::Posr11,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pcntr3::Posr11,
            pcntr3::Posr11,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pcntr3::Posr12,
        pcntr3::Posr12,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pcntr3::Posr12,
            pcntr3::Posr12,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pcntr3::Posr13,
        pcntr3::Posr13,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pcntr3::Posr13,
            pcntr3::Posr13,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pcntr3::Posr14,
        pcntr3::Posr14,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pcntr3::Posr14,
            pcntr3::Posr14,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pcntr3::Posr15,
        pcntr3::Posr15,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pcntr3::Posr15,
            pcntr3::Posr15,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr00(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        pcntr3::Porr00,
        pcntr3::Porr00,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pcntr3::Porr00,
            pcntr3::Porr00,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr01(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        pcntr3::Porr01,
        pcntr3::Porr01,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            pcntr3::Porr01,
            pcntr3::Porr01,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr02(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        pcntr3::Porr02,
        pcntr3::Porr02,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            pcntr3::Porr02,
            pcntr3::Porr02,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr03(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        pcntr3::Porr03,
        pcntr3::Porr03,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            pcntr3::Porr03,
            pcntr3::Porr03,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr04(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        pcntr3::Porr04,
        pcntr3::Porr04,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            pcntr3::Porr04,
            pcntr3::Porr04,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr05(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        pcntr3::Porr05,
        pcntr3::Porr05,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            pcntr3::Porr05,
            pcntr3::Porr05,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr06(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        pcntr3::Porr06,
        pcntr3::Porr06,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            pcntr3::Porr06,
            pcntr3::Porr06,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr07(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        pcntr3::Porr07,
        pcntr3::Porr07,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            pcntr3::Porr07,
            pcntr3::Porr07,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr08(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        pcntr3::Porr08,
        pcntr3::Porr08,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            pcntr3::Porr08,
            pcntr3::Porr08,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr09(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        pcntr3::Porr09,
        pcntr3::Porr09,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            pcntr3::Porr09,
            pcntr3::Porr09,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr10(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        pcntr3::Porr10,
        pcntr3::Porr10,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            pcntr3::Porr10,
            pcntr3::Porr10,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr11(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        pcntr3::Porr11,
        pcntr3::Porr11,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            pcntr3::Porr11,
            pcntr3::Porr11,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr12(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        pcntr3::Porr12,
        pcntr3::Porr12,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            pcntr3::Porr12,
            pcntr3::Porr12,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr13(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        pcntr3::Porr13,
        pcntr3::Porr13,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            pcntr3::Porr13,
            pcntr3::Porr13,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr14(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        pcntr3::Porr14,
        pcntr3::Porr14,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            pcntr3::Porr14,
            pcntr3::Porr14,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr15(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        pcntr3::Porr15,
        pcntr3::Porr15,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            pcntr3::Porr15,
            pcntr3::Porr15,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pcntr3 {
    #[inline(always)]
    fn default() -> Pcntr3 {
        <crate::RegValueT<Pcntr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pcntr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr00_SPEC;
    pub type Posr00 = crate::EnumBitfieldStruct<u8, Posr00_SPEC>;
    impl Posr00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr01_SPEC;
    pub type Posr01 = crate::EnumBitfieldStruct<u8, Posr01_SPEC>;
    impl Posr01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr02_SPEC;
    pub type Posr02 = crate::EnumBitfieldStruct<u8, Posr02_SPEC>;
    impl Posr02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr03_SPEC;
    pub type Posr03 = crate::EnumBitfieldStruct<u8, Posr03_SPEC>;
    impl Posr03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr04_SPEC;
    pub type Posr04 = crate::EnumBitfieldStruct<u8, Posr04_SPEC>;
    impl Posr04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr05_SPEC;
    pub type Posr05 = crate::EnumBitfieldStruct<u8, Posr05_SPEC>;
    impl Posr05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr06_SPEC;
    pub type Posr06 = crate::EnumBitfieldStruct<u8, Posr06_SPEC>;
    impl Posr06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr07_SPEC;
    pub type Posr07 = crate::EnumBitfieldStruct<u8, Posr07_SPEC>;
    impl Posr07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr08_SPEC;
    pub type Posr08 = crate::EnumBitfieldStruct<u8, Posr08_SPEC>;
    impl Posr08 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr09_SPEC;
    pub type Posr09 = crate::EnumBitfieldStruct<u8, Posr09_SPEC>;
    impl Posr09 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr10_SPEC;
    pub type Posr10 = crate::EnumBitfieldStruct<u8, Posr10_SPEC>;
    impl Posr10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr11_SPEC;
    pub type Posr11 = crate::EnumBitfieldStruct<u8, Posr11_SPEC>;
    impl Posr11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr12_SPEC;
    pub type Posr12 = crate::EnumBitfieldStruct<u8, Posr12_SPEC>;
    impl Posr12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr13_SPEC;
    pub type Posr13 = crate::EnumBitfieldStruct<u8, Posr13_SPEC>;
    impl Posr13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr14_SPEC;
    pub type Posr14 = crate::EnumBitfieldStruct<u8, Posr14_SPEC>;
    impl Posr14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr15_SPEC;
    pub type Posr15 = crate::EnumBitfieldStruct<u8, Posr15_SPEC>;
    impl Posr15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr00_SPEC;
    pub type Porr00 = crate::EnumBitfieldStruct<u8, Porr00_SPEC>;
    impl Porr00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr01_SPEC;
    pub type Porr01 = crate::EnumBitfieldStruct<u8, Porr01_SPEC>;
    impl Porr01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr02_SPEC;
    pub type Porr02 = crate::EnumBitfieldStruct<u8, Porr02_SPEC>;
    impl Porr02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr03_SPEC;
    pub type Porr03 = crate::EnumBitfieldStruct<u8, Porr03_SPEC>;
    impl Porr03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr04_SPEC;
    pub type Porr04 = crate::EnumBitfieldStruct<u8, Porr04_SPEC>;
    impl Porr04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr05_SPEC;
    pub type Porr05 = crate::EnumBitfieldStruct<u8, Porr05_SPEC>;
    impl Porr05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr06_SPEC;
    pub type Porr06 = crate::EnumBitfieldStruct<u8, Porr06_SPEC>;
    impl Porr06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr07_SPEC;
    pub type Porr07 = crate::EnumBitfieldStruct<u8, Porr07_SPEC>;
    impl Porr07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr08_SPEC;
    pub type Porr08 = crate::EnumBitfieldStruct<u8, Porr08_SPEC>;
    impl Porr08 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr09_SPEC;
    pub type Porr09 = crate::EnumBitfieldStruct<u8, Porr09_SPEC>;
    impl Porr09 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr10_SPEC;
    pub type Porr10 = crate::EnumBitfieldStruct<u8, Porr10_SPEC>;
    impl Porr10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr11_SPEC;
    pub type Porr11 = crate::EnumBitfieldStruct<u8, Porr11_SPEC>;
    impl Porr11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr12_SPEC;
    pub type Porr12 = crate::EnumBitfieldStruct<u8, Porr12_SPEC>;
    impl Porr12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr13_SPEC;
    pub type Porr13 = crate::EnumBitfieldStruct<u8, Porr13_SPEC>;
    impl Porr13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr14_SPEC;
    pub type Porr14 = crate::EnumBitfieldStruct<u8, Porr14_SPEC>;
    impl Porr14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr15_SPEC;
    pub type Porr15 = crate::EnumBitfieldStruct<u8, Porr15_SPEC>;
    impl Porr15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posr_SPEC;
impl crate::sealed::RegSpec for Posr_SPEC {
    type DataType = u16;
}

pub type Posr = crate::RegValueT<Posr_SPEC>;

impl Posr {
    #[inline(always)]
    pub fn porr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        posr::Porr00,
        posr::Porr00,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            posr::Porr00,
            posr::Porr00,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        posr::Porr01,
        posr::Porr01,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            posr::Porr01,
            posr::Porr01,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        posr::Porr02,
        posr::Porr02,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            posr::Porr02,
            posr::Porr02,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        posr::Porr03,
        posr::Porr03,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            posr::Porr03,
            posr::Porr03,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        posr::Porr04,
        posr::Porr04,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            posr::Porr04,
            posr::Porr04,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        posr::Porr05,
        posr::Porr05,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            posr::Porr05,
            posr::Porr05,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        posr::Porr06,
        posr::Porr06,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            posr::Porr06,
            posr::Porr06,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        posr::Porr07,
        posr::Porr07,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            posr::Porr07,
            posr::Porr07,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        posr::Porr08,
        posr::Porr08,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            posr::Porr08,
            posr::Porr08,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        posr::Porr09,
        posr::Porr09,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            posr::Porr09,
            posr::Porr09,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        posr::Porr10,
        posr::Porr10,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            posr::Porr10,
            posr::Porr10,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        posr::Porr11,
        posr::Porr11,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            posr::Porr11,
            posr::Porr11,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        posr::Porr12,
        posr::Porr12,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            posr::Porr12,
            posr::Porr12,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        posr::Porr13,
        posr::Porr13,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            posr::Porr13,
            posr::Porr13,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        posr::Porr14,
        posr::Porr14,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            posr::Porr14,
            posr::Porr14,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn porr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        posr::Porr15,
        posr::Porr15,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            posr::Porr15,
            posr::Porr15,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Posr {
    #[inline(always)]
    fn default() -> Posr {
        <crate::RegValueT<Posr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod posr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr00_SPEC;
    pub type Porr00 = crate::EnumBitfieldStruct<u8, Porr00_SPEC>;
    impl Porr00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr01_SPEC;
    pub type Porr01 = crate::EnumBitfieldStruct<u8, Porr01_SPEC>;
    impl Porr01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr02_SPEC;
    pub type Porr02 = crate::EnumBitfieldStruct<u8, Porr02_SPEC>;
    impl Porr02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr03_SPEC;
    pub type Porr03 = crate::EnumBitfieldStruct<u8, Porr03_SPEC>;
    impl Porr03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr04_SPEC;
    pub type Porr04 = crate::EnumBitfieldStruct<u8, Porr04_SPEC>;
    impl Porr04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr05_SPEC;
    pub type Porr05 = crate::EnumBitfieldStruct<u8, Porr05_SPEC>;
    impl Porr05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr06_SPEC;
    pub type Porr06 = crate::EnumBitfieldStruct<u8, Porr06_SPEC>;
    impl Porr06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr07_SPEC;
    pub type Porr07 = crate::EnumBitfieldStruct<u8, Porr07_SPEC>;
    impl Porr07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr08_SPEC;
    pub type Porr08 = crate::EnumBitfieldStruct<u8, Porr08_SPEC>;
    impl Porr08 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr09_SPEC;
    pub type Porr09 = crate::EnumBitfieldStruct<u8, Porr09_SPEC>;
    impl Porr09 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr10_SPEC;
    pub type Porr10 = crate::EnumBitfieldStruct<u8, Porr10_SPEC>;
    impl Porr10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr11_SPEC;
    pub type Porr11 = crate::EnumBitfieldStruct<u8, Porr11_SPEC>;
    impl Porr11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr12_SPEC;
    pub type Porr12 = crate::EnumBitfieldStruct<u8, Porr12_SPEC>;
    impl Porr12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr13_SPEC;
    pub type Porr13 = crate::EnumBitfieldStruct<u8, Porr13_SPEC>;
    impl Porr13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr14_SPEC;
    pub type Porr14 = crate::EnumBitfieldStruct<u8, Porr14_SPEC>;
    impl Porr14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr15_SPEC;
    pub type Porr15 = crate::EnumBitfieldStruct<u8, Porr15_SPEC>;
    impl Porr15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Porr_SPEC;
impl crate::sealed::RegSpec for Porr_SPEC {
    type DataType = u16;
}

pub type Porr = crate::RegValueT<Porr_SPEC>;

impl Porr {
    #[inline(always)]
    pub fn posr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        porr::Posr00,
        porr::Posr00,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            porr::Posr00,
            porr::Posr00,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        porr::Posr01,
        porr::Posr01,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            porr::Posr01,
            porr::Posr01,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        porr::Posr02,
        porr::Posr02,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            porr::Posr02,
            porr::Posr02,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        porr::Posr03,
        porr::Posr03,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            porr::Posr03,
            porr::Posr03,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        porr::Posr04,
        porr::Posr04,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            porr::Posr04,
            porr::Posr04,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        porr::Posr05,
        porr::Posr05,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            porr::Posr05,
            porr::Posr05,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        porr::Posr06,
        porr::Posr06,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            porr::Posr06,
            porr::Posr06,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        porr::Posr07,
        porr::Posr07,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            porr::Posr07,
            porr::Posr07,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        porr::Posr08,
        porr::Posr08,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            porr::Posr08,
            porr::Posr08,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        porr::Posr09,
        porr::Posr09,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            porr::Posr09,
            porr::Posr09,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        porr::Posr10,
        porr::Posr10,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            porr::Posr10,
            porr::Posr10,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        porr::Posr11,
        porr::Posr11,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            porr::Posr11,
            porr::Posr11,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        porr::Posr12,
        porr::Posr12,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            porr::Posr12,
            porr::Posr12,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        porr::Posr13,
        porr::Posr13,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            porr::Posr13,
            porr::Posr13,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        porr::Posr14,
        porr::Posr14,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            porr::Posr14,
            porr::Posr14,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        porr::Posr15,
        porr::Posr15,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            porr::Posr15,
            porr::Posr15,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Porr {
    #[inline(always)]
    fn default() -> Porr {
        <crate::RegValueT<Porr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod porr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr00_SPEC;
    pub type Posr00 = crate::EnumBitfieldStruct<u8, Posr00_SPEC>;
    impl Posr00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr01_SPEC;
    pub type Posr01 = crate::EnumBitfieldStruct<u8, Posr01_SPEC>;
    impl Posr01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr02_SPEC;
    pub type Posr02 = crate::EnumBitfieldStruct<u8, Posr02_SPEC>;
    impl Posr02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr03_SPEC;
    pub type Posr03 = crate::EnumBitfieldStruct<u8, Posr03_SPEC>;
    impl Posr03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr04_SPEC;
    pub type Posr04 = crate::EnumBitfieldStruct<u8, Posr04_SPEC>;
    impl Posr04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr05_SPEC;
    pub type Posr05 = crate::EnumBitfieldStruct<u8, Posr05_SPEC>;
    impl Posr05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr06_SPEC;
    pub type Posr06 = crate::EnumBitfieldStruct<u8, Posr06_SPEC>;
    impl Posr06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr07_SPEC;
    pub type Posr07 = crate::EnumBitfieldStruct<u8, Posr07_SPEC>;
    impl Posr07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr08_SPEC;
    pub type Posr08 = crate::EnumBitfieldStruct<u8, Posr08_SPEC>;
    impl Posr08 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr09_SPEC;
    pub type Posr09 = crate::EnumBitfieldStruct<u8, Posr09_SPEC>;
    impl Posr09 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr10_SPEC;
    pub type Posr10 = crate::EnumBitfieldStruct<u8, Posr10_SPEC>;
    impl Posr10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr11_SPEC;
    pub type Posr11 = crate::EnumBitfieldStruct<u8, Posr11_SPEC>;
    impl Posr11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr12_SPEC;
    pub type Posr12 = crate::EnumBitfieldStruct<u8, Posr12_SPEC>;
    impl Posr12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr13_SPEC;
    pub type Posr13 = crate::EnumBitfieldStruct<u8, Posr13_SPEC>;
    impl Posr13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr14_SPEC;
    pub type Posr14 = crate::EnumBitfieldStruct<u8, Posr14_SPEC>;
    impl Posr14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr15_SPEC;
    pub type Posr15 = crate::EnumBitfieldStruct<u8, Posr15_SPEC>;
    impl Posr15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
