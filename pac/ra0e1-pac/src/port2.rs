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
#[doc = r"Port 2 Control"]
unsafe impl ::core::marker::Send for super::Port2 {}
unsafe impl ::core::marker::Sync for super::Port2 {}
impl super::Port2 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Port 2 Output Data Register"]
    #[inline(always)]
    pub const fn podr2(&self) -> &'static crate::common::Reg<self::Podr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Podr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Port 2 Direction Register"]
    #[inline(always)]
    pub const fn pdr2(&self) -> &'static crate::common::Reg<self::Pdr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Port 2 State Register"]
    #[inline(always)]
    pub const fn pidr2(&self) -> &'static crate::common::Reg<self::Pidr2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pidr2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Port 2 Output Reset Register"]
    #[inline(always)]
    pub const fn porr2(&self) -> &'static crate::common::Reg<self::Porr2_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Porr2_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Port 2 Output Set Register"]
    #[inline(always)]
    pub const fn posr2(&self) -> &'static crate::common::Reg<self::Posr2_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Posr2_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "Port 2 Event Output Reset Register"]
    #[inline(always)]
    pub const fn eorr2(&self) -> &'static crate::common::Reg<self::Eorr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eorr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Port 2 Event Output Set Register"]
    #[inline(always)]
    pub const fn eosr2(&self) -> &'static crate::common::Reg<self::Eosr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eosr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Podr2_SPEC;
impl crate::sealed::RegSpec for Podr2_SPEC {
    type DataType = u16;
}

#[doc = "Port 2 Output Data Register"]
pub type Podr2 = crate::RegValueT<Podr2_SPEC>;

impl Podr2 {
    #[doc = "P201 Output Data"]
    #[inline(always)]
    pub fn podr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        podr2::Podr01,
        podr2::Podr01,
        Podr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            podr2::Podr01,
            podr2::Podr01,
            Podr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P206 Output Data"]
    #[inline(always)]
    pub fn podr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        podr2::Podr06,
        podr2::Podr06,
        Podr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            podr2::Podr06,
            podr2::Podr06,
            Podr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P207 Output Data"]
    #[inline(always)]
    pub fn podr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        podr2::Podr07,
        podr2::Podr07,
        Podr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            podr2::Podr07,
            podr2::Podr07,
            Podr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P208 Output Data"]
    #[inline(always)]
    pub fn podr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        podr2::Podr08,
        podr2::Podr08,
        Podr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            podr2::Podr08,
            podr2::Podr08,
            Podr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P212 Output Data"]
    #[inline(always)]
    pub fn podr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        podr2::Podr12,
        podr2::Podr12,
        Podr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            podr2::Podr12,
            podr2::Podr12,
            Podr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P213 Output Data"]
    #[inline(always)]
    pub fn podr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        podr2::Podr13,
        podr2::Podr13,
        Podr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            podr2::Podr13,
            podr2::Podr13,
            Podr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Podr2 {
    #[inline(always)]
    fn default() -> Podr2 {
        <crate::RegValueT<Podr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod podr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr01_SPEC;
    pub type Podr01 = crate::EnumBitfieldStruct<u8, Podr01_SPEC>;
    impl Podr01 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr06_SPEC;
    pub type Podr06 = crate::EnumBitfieldStruct<u8, Podr06_SPEC>;
    impl Podr06 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr07_SPEC;
    pub type Podr07 = crate::EnumBitfieldStruct<u8, Podr07_SPEC>;
    impl Podr07 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr08_SPEC;
    pub type Podr08 = crate::EnumBitfieldStruct<u8, Podr08_SPEC>;
    impl Podr08 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr12_SPEC;
    pub type Podr12 = crate::EnumBitfieldStruct<u8, Podr12_SPEC>;
    impl Podr12 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr13_SPEC;
    pub type Podr13 = crate::EnumBitfieldStruct<u8, Podr13_SPEC>;
    impl Podr13 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr2_SPEC;
impl crate::sealed::RegSpec for Pdr2_SPEC {
    type DataType = u16;
}

#[doc = "Port 2 Direction Register"]
pub type Pdr2 = crate::RegValueT<Pdr2_SPEC>;

impl Pdr2 {
    #[doc = "P201 Direction"]
    #[inline(always)]
    pub fn pdr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdr2::Pdr01,
        pdr2::Pdr01,
        Pdr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdr2::Pdr01,
            pdr2::Pdr01,
            Pdr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P206 Direction"]
    #[inline(always)]
    pub fn pdr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pdr2::Pdr06,
        pdr2::Pdr06,
        Pdr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pdr2::Pdr06,
            pdr2::Pdr06,
            Pdr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P207 Direction"]
    #[inline(always)]
    pub fn pdr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pdr2::Pdr07,
        pdr2::Pdr07,
        Pdr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pdr2::Pdr07,
            pdr2::Pdr07,
            Pdr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P208 Direction"]
    #[inline(always)]
    pub fn pdr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pdr2::Pdr08,
        pdr2::Pdr08,
        Pdr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pdr2::Pdr08,
            pdr2::Pdr08,
            Pdr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P212 Direction"]
    #[inline(always)]
    pub fn pdr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pdr2::Pdr12,
        pdr2::Pdr12,
        Pdr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pdr2::Pdr12,
            pdr2::Pdr12,
            Pdr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P213 Direction"]
    #[inline(always)]
    pub fn pdr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pdr2::Pdr13,
        pdr2::Pdr13,
        Pdr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pdr2::Pdr13,
            pdr2::Pdr13,
            Pdr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdr2 {
    #[inline(always)]
    fn default() -> Pdr2 {
        <crate::RegValueT<Pdr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr01_SPEC;
    pub type Pdr01 = crate::EnumBitfieldStruct<u8, Pdr01_SPEC>;
    impl Pdr01 {
        #[doc = "Output 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr06_SPEC;
    pub type Pdr06 = crate::EnumBitfieldStruct<u8, Pdr06_SPEC>;
    impl Pdr06 {
        #[doc = "Output 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr07_SPEC;
    pub type Pdr07 = crate::EnumBitfieldStruct<u8, Pdr07_SPEC>;
    impl Pdr07 {
        #[doc = "Output 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr08_SPEC;
    pub type Pdr08 = crate::EnumBitfieldStruct<u8, Pdr08_SPEC>;
    impl Pdr08 {
        #[doc = "Output 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr12_SPEC;
    pub type Pdr12 = crate::EnumBitfieldStruct<u8, Pdr12_SPEC>;
    impl Pdr12 {
        #[doc = "Output 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr13_SPEC;
    pub type Pdr13 = crate::EnumBitfieldStruct<u8, Pdr13_SPEC>;
    impl Pdr13 {
        #[doc = "Output 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr2_SPEC;
impl crate::sealed::RegSpec for Pidr2_SPEC {
    type DataType = u16;
}

#[doc = "Port 2 State Register"]
pub type Pidr2 = crate::RegValueT<Pidr2_SPEC>;

impl Pidr2 {
    #[doc = "P200 State"]
    #[inline(always)]
    pub fn pidr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pidr2::Pidr00,
        pidr2::Pidr00,
        Pidr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pidr2::Pidr00,
            pidr2::Pidr00,
            Pidr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P201 State"]
    #[inline(always)]
    pub fn pidr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pidr2::Pidr01,
        pidr2::Pidr01,
        Pidr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pidr2::Pidr01,
            pidr2::Pidr01,
            Pidr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P206 State"]
    #[inline(always)]
    pub fn pidr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pidr2::Pidr06,
        pidr2::Pidr06,
        Pidr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pidr2::Pidr06,
            pidr2::Pidr06,
            Pidr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P207 State"]
    #[inline(always)]
    pub fn pidr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pidr2::Pidr07,
        pidr2::Pidr07,
        Pidr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pidr2::Pidr07,
            pidr2::Pidr07,
            Pidr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P208 State"]
    #[inline(always)]
    pub fn pidr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pidr2::Pidr08,
        pidr2::Pidr08,
        Pidr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pidr2::Pidr08,
            pidr2::Pidr08,
            Pidr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P212 State"]
    #[inline(always)]
    pub fn pidr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pidr2::Pidr12,
        pidr2::Pidr12,
        Pidr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pidr2::Pidr12,
            pidr2::Pidr12,
            Pidr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P213 State"]
    #[inline(always)]
    pub fn pidr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pidr2::Pidr13,
        pidr2::Pidr13,
        Pidr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pidr2::Pidr13,
            pidr2::Pidr13,
            Pidr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P214 State"]
    #[inline(always)]
    pub fn pidr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pidr2::Pidr14,
        pidr2::Pidr14,
        Pidr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pidr2::Pidr14,
            pidr2::Pidr14,
            Pidr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P215 State"]
    #[inline(always)]
    pub fn pidr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pidr2::Pidr15,
        pidr2::Pidr15,
        Pidr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pidr2::Pidr15,
            pidr2::Pidr15,
            Pidr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pidr2 {
    #[inline(always)]
    fn default() -> Pidr2 {
        <crate::RegValueT<Pidr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pidr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr00_SPEC;
    pub type Pidr00 = crate::EnumBitfieldStruct<u8, Pidr00_SPEC>;
    impl Pidr00 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr01_SPEC;
    pub type Pidr01 = crate::EnumBitfieldStruct<u8, Pidr01_SPEC>;
    impl Pidr01 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr06_SPEC;
    pub type Pidr06 = crate::EnumBitfieldStruct<u8, Pidr06_SPEC>;
    impl Pidr06 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr07_SPEC;
    pub type Pidr07 = crate::EnumBitfieldStruct<u8, Pidr07_SPEC>;
    impl Pidr07 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr08_SPEC;
    pub type Pidr08 = crate::EnumBitfieldStruct<u8, Pidr08_SPEC>;
    impl Pidr08 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr12_SPEC;
    pub type Pidr12 = crate::EnumBitfieldStruct<u8, Pidr12_SPEC>;
    impl Pidr12 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr13_SPEC;
    pub type Pidr13 = crate::EnumBitfieldStruct<u8, Pidr13_SPEC>;
    impl Pidr13 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr14_SPEC;
    pub type Pidr14 = crate::EnumBitfieldStruct<u8, Pidr14_SPEC>;
    impl Pidr14 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr15_SPEC;
    pub type Pidr15 = crate::EnumBitfieldStruct<u8, Pidr15_SPEC>;
    impl Pidr15 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Porr2_SPEC;
impl crate::sealed::RegSpec for Porr2_SPEC {
    type DataType = u16;
}

#[doc = "Port 2 Output Reset Register"]
pub type Porr2 = crate::RegValueT<Porr2_SPEC>;

impl Porr2 {
    #[doc = "P201 Output Reset"]
    #[inline(always)]
    pub fn porr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        porr2::Porr01,
        porr2::Porr01,
        Porr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            porr2::Porr01,
            porr2::Porr01,
            Porr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P206 Output Reset"]
    #[inline(always)]
    pub fn porr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        porr2::Porr06,
        porr2::Porr06,
        Porr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            porr2::Porr06,
            porr2::Porr06,
            Porr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P207 Output Reset"]
    #[inline(always)]
    pub fn porr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        porr2::Porr07,
        porr2::Porr07,
        Porr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            porr2::Porr07,
            porr2::Porr07,
            Porr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P208 Output Reset"]
    #[inline(always)]
    pub fn porr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        porr2::Porr08,
        porr2::Porr08,
        Porr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            porr2::Porr08,
            porr2::Porr08,
            Porr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P212 Output Reset"]
    #[inline(always)]
    pub fn porr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        porr2::Porr12,
        porr2::Porr12,
        Porr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            porr2::Porr12,
            porr2::Porr12,
            Porr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P213 Output Reset"]
    #[inline(always)]
    pub fn porr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        porr2::Porr13,
        porr2::Porr13,
        Porr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            porr2::Porr13,
            porr2::Porr13,
            Porr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Porr2 {
    #[inline(always)]
    fn default() -> Porr2 {
        <crate::RegValueT<Porr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod porr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr01_SPEC;
    pub type Porr01 = crate::EnumBitfieldStruct<u8, Porr01_SPEC>;
    impl Porr01 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr06_SPEC;
    pub type Porr06 = crate::EnumBitfieldStruct<u8, Porr06_SPEC>;
    impl Porr06 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr07_SPEC;
    pub type Porr07 = crate::EnumBitfieldStruct<u8, Porr07_SPEC>;
    impl Porr07 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr08_SPEC;
    pub type Porr08 = crate::EnumBitfieldStruct<u8, Porr08_SPEC>;
    impl Porr08 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr12_SPEC;
    pub type Porr12 = crate::EnumBitfieldStruct<u8, Porr12_SPEC>;
    impl Porr12 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr13_SPEC;
    pub type Porr13 = crate::EnumBitfieldStruct<u8, Porr13_SPEC>;
    impl Porr13 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posr2_SPEC;
impl crate::sealed::RegSpec for Posr2_SPEC {
    type DataType = u16;
}

#[doc = "Port 2 Output Set Register"]
pub type Posr2 = crate::RegValueT<Posr2_SPEC>;

impl Posr2 {
    #[doc = "P201 Output Set"]
    #[inline(always)]
    pub fn posr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        posr2::Posr01,
        posr2::Posr01,
        Posr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            posr2::Posr01,
            posr2::Posr01,
            Posr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P206 Output Set"]
    #[inline(always)]
    pub fn posr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        posr2::Posr06,
        posr2::Posr06,
        Posr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            posr2::Posr06,
            posr2::Posr06,
            Posr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P207 Output Set"]
    #[inline(always)]
    pub fn posr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        posr2::Posr07,
        posr2::Posr07,
        Posr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            posr2::Posr07,
            posr2::Posr07,
            Posr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P208 Output Set"]
    #[inline(always)]
    pub fn posr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        posr2::Posr08,
        posr2::Posr08,
        Posr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            posr2::Posr08,
            posr2::Posr08,
            Posr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P212 Output Set"]
    #[inline(always)]
    pub fn posr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        posr2::Posr12,
        posr2::Posr12,
        Posr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            posr2::Posr12,
            posr2::Posr12,
            Posr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P213 Output Set"]
    #[inline(always)]
    pub fn posr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        posr2::Posr13,
        posr2::Posr13,
        Posr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            posr2::Posr13,
            posr2::Posr13,
            Posr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Posr2 {
    #[inline(always)]
    fn default() -> Posr2 {
        <crate::RegValueT<Posr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod posr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr01_SPEC;
    pub type Posr01 = crate::EnumBitfieldStruct<u8, Posr01_SPEC>;
    impl Posr01 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr06_SPEC;
    pub type Posr06 = crate::EnumBitfieldStruct<u8, Posr06_SPEC>;
    impl Posr06 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr07_SPEC;
    pub type Posr07 = crate::EnumBitfieldStruct<u8, Posr07_SPEC>;
    impl Posr07 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr08_SPEC;
    pub type Posr08 = crate::EnumBitfieldStruct<u8, Posr08_SPEC>;
    impl Posr08 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr12_SPEC;
    pub type Posr12 = crate::EnumBitfieldStruct<u8, Posr12_SPEC>;
    impl Posr12 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr13_SPEC;
    pub type Posr13 = crate::EnumBitfieldStruct<u8, Posr13_SPEC>;
    impl Posr13 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eorr2_SPEC;
impl crate::sealed::RegSpec for Eorr2_SPEC {
    type DataType = u16;
}

#[doc = "Port 2 Event Output Reset Register"]
pub type Eorr2 = crate::RegValueT<Eorr2_SPEC>;

impl Eorr2 {
    #[doc = "P201 Event Output Reset"]
    #[inline(always)]
    pub fn eorr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eorr2::Eorr01,
        eorr2::Eorr01,
        Eorr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eorr2::Eorr01,
            eorr2::Eorr01,
            Eorr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P206 Event Output Reset"]
    #[inline(always)]
    pub fn eorr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eorr2::Eorr06,
        eorr2::Eorr06,
        Eorr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eorr2::Eorr06,
            eorr2::Eorr06,
            Eorr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P207 Event Output Reset"]
    #[inline(always)]
    pub fn eorr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eorr2::Eorr07,
        eorr2::Eorr07,
        Eorr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eorr2::Eorr07,
            eorr2::Eorr07,
            Eorr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P208 Event Output Reset"]
    #[inline(always)]
    pub fn eorr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eorr2::Eorr08,
        eorr2::Eorr08,
        Eorr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eorr2::Eorr08,
            eorr2::Eorr08,
            Eorr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P212 Event Output Reset"]
    #[inline(always)]
    pub fn eorr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        eorr2::Eorr12,
        eorr2::Eorr12,
        Eorr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            eorr2::Eorr12,
            eorr2::Eorr12,
            Eorr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P213 Event Output Reset"]
    #[inline(always)]
    pub fn eorr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        eorr2::Eorr13,
        eorr2::Eorr13,
        Eorr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            eorr2::Eorr13,
            eorr2::Eorr13,
            Eorr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eorr2 {
    #[inline(always)]
    fn default() -> Eorr2 {
        <crate::RegValueT<Eorr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eorr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr01_SPEC;
    pub type Eorr01 = crate::EnumBitfieldStruct<u8, Eorr01_SPEC>;
    impl Eorr01 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr06_SPEC;
    pub type Eorr06 = crate::EnumBitfieldStruct<u8, Eorr06_SPEC>;
    impl Eorr06 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr07_SPEC;
    pub type Eorr07 = crate::EnumBitfieldStruct<u8, Eorr07_SPEC>;
    impl Eorr07 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr08_SPEC;
    pub type Eorr08 = crate::EnumBitfieldStruct<u8, Eorr08_SPEC>;
    impl Eorr08 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr12_SPEC;
    pub type Eorr12 = crate::EnumBitfieldStruct<u8, Eorr12_SPEC>;
    impl Eorr12 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr13_SPEC;
    pub type Eorr13 = crate::EnumBitfieldStruct<u8, Eorr13_SPEC>;
    impl Eorr13 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eosr2_SPEC;
impl crate::sealed::RegSpec for Eosr2_SPEC {
    type DataType = u16;
}

#[doc = "Port 2 Event Output Set Register"]
pub type Eosr2 = crate::RegValueT<Eosr2_SPEC>;

impl Eosr2 {
    #[doc = "P201 Event Output Set"]
    #[inline(always)]
    pub fn eosr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eosr2::Eosr01,
        eosr2::Eosr01,
        Eosr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eosr2::Eosr01,
            eosr2::Eosr01,
            Eosr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P206 Event Output Set"]
    #[inline(always)]
    pub fn eosr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eosr2::Eosr06,
        eosr2::Eosr06,
        Eosr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eosr2::Eosr06,
            eosr2::Eosr06,
            Eosr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P207 Event Output Set"]
    #[inline(always)]
    pub fn eosr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eosr2::Eosr07,
        eosr2::Eosr07,
        Eosr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eosr2::Eosr07,
            eosr2::Eosr07,
            Eosr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P208 Event Output Set"]
    #[inline(always)]
    pub fn eosr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eosr2::Eosr08,
        eosr2::Eosr08,
        Eosr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eosr2::Eosr08,
            eosr2::Eosr08,
            Eosr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P212 Event Output Set"]
    #[inline(always)]
    pub fn eosr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        eosr2::Eosr12,
        eosr2::Eosr12,
        Eosr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            eosr2::Eosr12,
            eosr2::Eosr12,
            Eosr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P213 Event Output Set"]
    #[inline(always)]
    pub fn eosr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        eosr2::Eosr13,
        eosr2::Eosr13,
        Eosr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            eosr2::Eosr13,
            eosr2::Eosr13,
            Eosr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eosr2 {
    #[inline(always)]
    fn default() -> Eosr2 {
        <crate::RegValueT<Eosr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eosr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr01_SPEC;
    pub type Eosr01 = crate::EnumBitfieldStruct<u8, Eosr01_SPEC>;
    impl Eosr01 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr06_SPEC;
    pub type Eosr06 = crate::EnumBitfieldStruct<u8, Eosr06_SPEC>;
    impl Eosr06 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr07_SPEC;
    pub type Eosr07 = crate::EnumBitfieldStruct<u8, Eosr07_SPEC>;
    impl Eosr07 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr08_SPEC;
    pub type Eosr08 = crate::EnumBitfieldStruct<u8, Eosr08_SPEC>;
    impl Eosr08 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr12_SPEC;
    pub type Eosr12 = crate::EnumBitfieldStruct<u8, Eosr12_SPEC>;
    impl Eosr12 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr13_SPEC;
    pub type Eosr13 = crate::EnumBitfieldStruct<u8, Eosr13_SPEC>;
    impl Eosr13 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
