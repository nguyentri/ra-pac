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
#[doc = r"Port 1 Control"]
unsafe impl ::core::marker::Send for super::Port1 {}
unsafe impl ::core::marker::Sync for super::Port1 {}
impl super::Port1 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Port 1 Output Data Register"]
    #[inline(always)]
    pub const fn podr1(&self) -> &'static crate::common::Reg<self::Podr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Podr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Port 1 Direction Register"]
    #[inline(always)]
    pub const fn pdr1(&self) -> &'static crate::common::Reg<self::Pdr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Port 1 State Register"]
    #[inline(always)]
    pub const fn pidr1(&self) -> &'static crate::common::Reg<self::Pidr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pidr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Port 1 Output Reset Register"]
    #[inline(always)]
    pub const fn porr1(&self) -> &'static crate::common::Reg<self::Porr1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Porr1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Port 1 Output Set Register"]
    #[inline(always)]
    pub const fn posr1(&self) -> &'static crate::common::Reg<self::Posr1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Posr1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "Port 1 Event Output Reset Register"]
    #[inline(always)]
    pub const fn eorr1(&self) -> &'static crate::common::Reg<self::Eorr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eorr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Port 1 Event Output Set Register"]
    #[inline(always)]
    pub const fn eosr1(&self) -> &'static crate::common::Reg<self::Eosr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eosr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Podr1_SPEC;
impl crate::sealed::RegSpec for Podr1_SPEC {
    type DataType = u16;
}

#[doc = "Port 1 Output Data Register"]
pub type Podr1 = crate::RegValueT<Podr1_SPEC>;

impl Podr1 {
    #[doc = "P100 Output Data"]
    #[inline(always)]
    pub fn podr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        podr1::Podr00,
        podr1::Podr00,
        Podr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            podr1::Podr00,
            podr1::Podr00,
            Podr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P101 Output Data"]
    #[inline(always)]
    pub fn podr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        podr1::Podr01,
        podr1::Podr01,
        Podr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            podr1::Podr01,
            podr1::Podr01,
            Podr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P102 Output Data"]
    #[inline(always)]
    pub fn podr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        podr1::Podr02,
        podr1::Podr02,
        Podr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            podr1::Podr02,
            podr1::Podr02,
            Podr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P103 Output Data"]
    #[inline(always)]
    pub fn podr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        podr1::Podr03,
        podr1::Podr03,
        Podr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            podr1::Podr03,
            podr1::Podr03,
            Podr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P108 Output Data"]
    #[inline(always)]
    pub fn podr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        podr1::Podr08,
        podr1::Podr08,
        Podr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            podr1::Podr08,
            podr1::Podr08,
            Podr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P109 Output Data"]
    #[inline(always)]
    pub fn podr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        podr1::Podr09,
        podr1::Podr09,
        Podr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            podr1::Podr09,
            podr1::Podr09,
            Podr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P110 Output Data"]
    #[inline(always)]
    pub fn podr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        podr1::Podr10,
        podr1::Podr10,
        Podr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            podr1::Podr10,
            podr1::Podr10,
            Podr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P112 Output Data"]
    #[inline(always)]
    pub fn podr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        podr1::Podr12,
        podr1::Podr12,
        Podr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            podr1::Podr12,
            podr1::Podr12,
            Podr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Podr1 {
    #[inline(always)]
    fn default() -> Podr1 {
        <crate::RegValueT<Podr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod podr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr00_SPEC;
    pub type Podr00 = crate::EnumBitfieldStruct<u8, Podr00_SPEC>;
    impl Podr00 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
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
    pub struct Podr02_SPEC;
    pub type Podr02 = crate::EnumBitfieldStruct<u8, Podr02_SPEC>;
    impl Podr02 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr03_SPEC;
    pub type Podr03 = crate::EnumBitfieldStruct<u8, Podr03_SPEC>;
    impl Podr03 {
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
    pub struct Podr09_SPEC;
    pub type Podr09 = crate::EnumBitfieldStruct<u8, Podr09_SPEC>;
    impl Podr09 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr10_SPEC;
    pub type Podr10 = crate::EnumBitfieldStruct<u8, Podr10_SPEC>;
    impl Podr10 {
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr1_SPEC;
impl crate::sealed::RegSpec for Pdr1_SPEC {
    type DataType = u16;
}

#[doc = "Port 1 Direction Register"]
pub type Pdr1 = crate::RegValueT<Pdr1_SPEC>;

impl Pdr1 {
    #[doc = "P100 Direction"]
    #[inline(always)]
    pub fn pdr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdr1::Pdr00,
        pdr1::Pdr00,
        Pdr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdr1::Pdr00,
            pdr1::Pdr00,
            Pdr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P101 Direction"]
    #[inline(always)]
    pub fn pdr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdr1::Pdr01,
        pdr1::Pdr01,
        Pdr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdr1::Pdr01,
            pdr1::Pdr01,
            Pdr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P102 Direction"]
    #[inline(always)]
    pub fn pdr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pdr1::Pdr02,
        pdr1::Pdr02,
        Pdr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pdr1::Pdr02,
            pdr1::Pdr02,
            Pdr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P103 Direction"]
    #[inline(always)]
    pub fn pdr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pdr1::Pdr03,
        pdr1::Pdr03,
        Pdr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pdr1::Pdr03,
            pdr1::Pdr03,
            Pdr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P108 Direction"]
    #[inline(always)]
    pub fn pdr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pdr1::Pdr08,
        pdr1::Pdr08,
        Pdr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pdr1::Pdr08,
            pdr1::Pdr08,
            Pdr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P109 Direction"]
    #[inline(always)]
    pub fn pdr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pdr1::Pdr09,
        pdr1::Pdr09,
        Pdr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pdr1::Pdr09,
            pdr1::Pdr09,
            Pdr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P110 Direction"]
    #[inline(always)]
    pub fn pdr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pdr1::Pdr10,
        pdr1::Pdr10,
        Pdr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pdr1::Pdr10,
            pdr1::Pdr10,
            Pdr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P112 Direction"]
    #[inline(always)]
    pub fn pdr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pdr1::Pdr12,
        pdr1::Pdr12,
        Pdr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pdr1::Pdr12,
            pdr1::Pdr12,
            Pdr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdr1 {
    #[inline(always)]
    fn default() -> Pdr1 {
        <crate::RegValueT<Pdr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr00_SPEC;
    pub type Pdr00 = crate::EnumBitfieldStruct<u8, Pdr00_SPEC>;
    impl Pdr00 {
        #[doc = "Output 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output 1"]
        pub const _1: Self = Self::new(1);
    }
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
    pub struct Pdr02_SPEC;
    pub type Pdr02 = crate::EnumBitfieldStruct<u8, Pdr02_SPEC>;
    impl Pdr02 {
        #[doc = "Output 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr03_SPEC;
    pub type Pdr03 = crate::EnumBitfieldStruct<u8, Pdr03_SPEC>;
    impl Pdr03 {
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
    pub struct Pdr09_SPEC;
    pub type Pdr09 = crate::EnumBitfieldStruct<u8, Pdr09_SPEC>;
    impl Pdr09 {
        #[doc = "Output 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr10_SPEC;
    pub type Pdr10 = crate::EnumBitfieldStruct<u8, Pdr10_SPEC>;
    impl Pdr10 {
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr1_SPEC;
impl crate::sealed::RegSpec for Pidr1_SPEC {
    type DataType = u16;
}

#[doc = "Port 1 State Register"]
pub type Pidr1 = crate::RegValueT<Pidr1_SPEC>;

impl Pidr1 {
    #[doc = "P100 State"]
    #[inline(always)]
    pub fn pidr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pidr1::Pidr00,
        pidr1::Pidr00,
        Pidr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pidr1::Pidr00,
            pidr1::Pidr00,
            Pidr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P101 State"]
    #[inline(always)]
    pub fn pidr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pidr1::Pidr01,
        pidr1::Pidr01,
        Pidr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pidr1::Pidr01,
            pidr1::Pidr01,
            Pidr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P102 State"]
    #[inline(always)]
    pub fn pidr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pidr1::Pidr02,
        pidr1::Pidr02,
        Pidr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pidr1::Pidr02,
            pidr1::Pidr02,
            Pidr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P103 State"]
    #[inline(always)]
    pub fn pidr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pidr1::Pidr03,
        pidr1::Pidr03,
        Pidr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pidr1::Pidr03,
            pidr1::Pidr03,
            Pidr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P108 State"]
    #[inline(always)]
    pub fn pidr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pidr1::Pidr08,
        pidr1::Pidr08,
        Pidr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pidr1::Pidr08,
            pidr1::Pidr08,
            Pidr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P109 State"]
    #[inline(always)]
    pub fn pidr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pidr1::Pidr09,
        pidr1::Pidr09,
        Pidr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pidr1::Pidr09,
            pidr1::Pidr09,
            Pidr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P110 State"]
    #[inline(always)]
    pub fn pidr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pidr1::Pidr10,
        pidr1::Pidr10,
        Pidr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pidr1::Pidr10,
            pidr1::Pidr10,
            Pidr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P112 State"]
    #[inline(always)]
    pub fn pidr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pidr1::Pidr12,
        pidr1::Pidr12,
        Pidr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pidr1::Pidr12,
            pidr1::Pidr12,
            Pidr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pidr1 {
    #[inline(always)]
    fn default() -> Pidr1 {
        <crate::RegValueT<Pidr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pidr1 {

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
    pub struct Pidr02_SPEC;
    pub type Pidr02 = crate::EnumBitfieldStruct<u8, Pidr02_SPEC>;
    impl Pidr02 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr03_SPEC;
    pub type Pidr03 = crate::EnumBitfieldStruct<u8, Pidr03_SPEC>;
    impl Pidr03 {
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
    pub struct Pidr09_SPEC;
    pub type Pidr09 = crate::EnumBitfieldStruct<u8, Pidr09_SPEC>;
    impl Pidr09 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr10_SPEC;
    pub type Pidr10 = crate::EnumBitfieldStruct<u8, Pidr10_SPEC>;
    impl Pidr10 {
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Porr1_SPEC;
impl crate::sealed::RegSpec for Porr1_SPEC {
    type DataType = u16;
}

#[doc = "Port 1 Output Reset Register"]
pub type Porr1 = crate::RegValueT<Porr1_SPEC>;

impl Porr1 {
    #[doc = "P100 Output Reset"]
    #[inline(always)]
    pub fn porr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        porr1::Porr00,
        porr1::Porr00,
        Porr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            porr1::Porr00,
            porr1::Porr00,
            Porr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P101 Output Reset"]
    #[inline(always)]
    pub fn porr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        porr1::Porr01,
        porr1::Porr01,
        Porr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            porr1::Porr01,
            porr1::Porr01,
            Porr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P102 Output Reset"]
    #[inline(always)]
    pub fn porr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        porr1::Porr02,
        porr1::Porr02,
        Porr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            porr1::Porr02,
            porr1::Porr02,
            Porr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P103 Output Reset"]
    #[inline(always)]
    pub fn porr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        porr1::Porr03,
        porr1::Porr03,
        Porr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            porr1::Porr03,
            porr1::Porr03,
            Porr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P108 Output Reset"]
    #[inline(always)]
    pub fn porr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        porr1::Porr08,
        porr1::Porr08,
        Porr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            porr1::Porr08,
            porr1::Porr08,
            Porr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P109 Output Reset"]
    #[inline(always)]
    pub fn porr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        porr1::Porr09,
        porr1::Porr09,
        Porr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            porr1::Porr09,
            porr1::Porr09,
            Porr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P110 Output Reset"]
    #[inline(always)]
    pub fn porr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        porr1::Porr10,
        porr1::Porr10,
        Porr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            porr1::Porr10,
            porr1::Porr10,
            Porr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P112 Output Reset"]
    #[inline(always)]
    pub fn porr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        porr1::Porr12,
        porr1::Porr12,
        Porr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            porr1::Porr12,
            porr1::Porr12,
            Porr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Porr1 {
    #[inline(always)]
    fn default() -> Porr1 {
        <crate::RegValueT<Porr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod porr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr00_SPEC;
    pub type Porr00 = crate::EnumBitfieldStruct<u8, Porr00_SPEC>;
    impl Porr00 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
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
    pub struct Porr02_SPEC;
    pub type Porr02 = crate::EnumBitfieldStruct<u8, Porr02_SPEC>;
    impl Porr02 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr03_SPEC;
    pub type Porr03 = crate::EnumBitfieldStruct<u8, Porr03_SPEC>;
    impl Porr03 {
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
    pub struct Porr09_SPEC;
    pub type Porr09 = crate::EnumBitfieldStruct<u8, Porr09_SPEC>;
    impl Porr09 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr10_SPEC;
    pub type Porr10 = crate::EnumBitfieldStruct<u8, Porr10_SPEC>;
    impl Porr10 {
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posr1_SPEC;
impl crate::sealed::RegSpec for Posr1_SPEC {
    type DataType = u16;
}

#[doc = "Port 1 Output Set Register"]
pub type Posr1 = crate::RegValueT<Posr1_SPEC>;

impl Posr1 {
    #[doc = "P100 Output Set"]
    #[inline(always)]
    pub fn posr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        posr1::Posr00,
        posr1::Posr00,
        Posr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            posr1::Posr00,
            posr1::Posr00,
            Posr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P101 Output Set"]
    #[inline(always)]
    pub fn posr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        posr1::Posr01,
        posr1::Posr01,
        Posr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            posr1::Posr01,
            posr1::Posr01,
            Posr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P102 Output Set"]
    #[inline(always)]
    pub fn posr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        posr1::Posr02,
        posr1::Posr02,
        Posr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            posr1::Posr02,
            posr1::Posr02,
            Posr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P103 Output Set"]
    #[inline(always)]
    pub fn posr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        posr1::Posr03,
        posr1::Posr03,
        Posr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            posr1::Posr03,
            posr1::Posr03,
            Posr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P108 Output Set"]
    #[inline(always)]
    pub fn posr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        posr1::Posr08,
        posr1::Posr08,
        Posr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            posr1::Posr08,
            posr1::Posr08,
            Posr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P109 Output Set"]
    #[inline(always)]
    pub fn posr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        posr1::Posr09,
        posr1::Posr09,
        Posr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            posr1::Posr09,
            posr1::Posr09,
            Posr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P110 Output Set"]
    #[inline(always)]
    pub fn posr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        posr1::Posr10,
        posr1::Posr10,
        Posr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            posr1::Posr10,
            posr1::Posr10,
            Posr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P112 Output Set"]
    #[inline(always)]
    pub fn posr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        posr1::Posr12,
        posr1::Posr12,
        Posr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            posr1::Posr12,
            posr1::Posr12,
            Posr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Posr1 {
    #[inline(always)]
    fn default() -> Posr1 {
        <crate::RegValueT<Posr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod posr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr00_SPEC;
    pub type Posr00 = crate::EnumBitfieldStruct<u8, Posr00_SPEC>;
    impl Posr00 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
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
    pub struct Posr02_SPEC;
    pub type Posr02 = crate::EnumBitfieldStruct<u8, Posr02_SPEC>;
    impl Posr02 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr03_SPEC;
    pub type Posr03 = crate::EnumBitfieldStruct<u8, Posr03_SPEC>;
    impl Posr03 {
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
    pub struct Posr09_SPEC;
    pub type Posr09 = crate::EnumBitfieldStruct<u8, Posr09_SPEC>;
    impl Posr09 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr10_SPEC;
    pub type Posr10 = crate::EnumBitfieldStruct<u8, Posr10_SPEC>;
    impl Posr10 {
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eorr1_SPEC;
impl crate::sealed::RegSpec for Eorr1_SPEC {
    type DataType = u16;
}

#[doc = "Port 1 Event Output Reset Register"]
pub type Eorr1 = crate::RegValueT<Eorr1_SPEC>;

impl Eorr1 {
    #[doc = "P100 Event Output Reset"]
    #[inline(always)]
    pub fn eorr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eorr1::Eorr00,
        eorr1::Eorr00,
        Eorr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eorr1::Eorr00,
            eorr1::Eorr00,
            Eorr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P101 Event Output Reset"]
    #[inline(always)]
    pub fn eorr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eorr1::Eorr01,
        eorr1::Eorr01,
        Eorr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eorr1::Eorr01,
            eorr1::Eorr01,
            Eorr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P102 Event Output Reset"]
    #[inline(always)]
    pub fn eorr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eorr1::Eorr02,
        eorr1::Eorr02,
        Eorr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eorr1::Eorr02,
            eorr1::Eorr02,
            Eorr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P103 Event Output Reset"]
    #[inline(always)]
    pub fn eorr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eorr1::Eorr03,
        eorr1::Eorr03,
        Eorr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eorr1::Eorr03,
            eorr1::Eorr03,
            Eorr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P108 Event Output Reset"]
    #[inline(always)]
    pub fn eorr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eorr1::Eorr08,
        eorr1::Eorr08,
        Eorr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eorr1::Eorr08,
            eorr1::Eorr08,
            Eorr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P109 Event Output Reset"]
    #[inline(always)]
    pub fn eorr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        eorr1::Eorr09,
        eorr1::Eorr09,
        Eorr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            eorr1::Eorr09,
            eorr1::Eorr09,
            Eorr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P110 Event Output Reset"]
    #[inline(always)]
    pub fn eorr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        eorr1::Eorr10,
        eorr1::Eorr10,
        Eorr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            eorr1::Eorr10,
            eorr1::Eorr10,
            Eorr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P112 Event Output Reset"]
    #[inline(always)]
    pub fn eorr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        eorr1::Eorr12,
        eorr1::Eorr12,
        Eorr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            eorr1::Eorr12,
            eorr1::Eorr12,
            Eorr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eorr1 {
    #[inline(always)]
    fn default() -> Eorr1 {
        <crate::RegValueT<Eorr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eorr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr00_SPEC;
    pub type Eorr00 = crate::EnumBitfieldStruct<u8, Eorr00_SPEC>;
    impl Eorr00 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
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
    pub struct Eorr02_SPEC;
    pub type Eorr02 = crate::EnumBitfieldStruct<u8, Eorr02_SPEC>;
    impl Eorr02 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr03_SPEC;
    pub type Eorr03 = crate::EnumBitfieldStruct<u8, Eorr03_SPEC>;
    impl Eorr03 {
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
    pub struct Eorr09_SPEC;
    pub type Eorr09 = crate::EnumBitfieldStruct<u8, Eorr09_SPEC>;
    impl Eorr09 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr10_SPEC;
    pub type Eorr10 = crate::EnumBitfieldStruct<u8, Eorr10_SPEC>;
    impl Eorr10 {
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eosr1_SPEC;
impl crate::sealed::RegSpec for Eosr1_SPEC {
    type DataType = u16;
}

#[doc = "Port 1 Event Output Set Register"]
pub type Eosr1 = crate::RegValueT<Eosr1_SPEC>;

impl Eosr1 {
    #[doc = "P100 Event Output Set"]
    #[inline(always)]
    pub fn eosr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eosr1::Eosr00,
        eosr1::Eosr00,
        Eosr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eosr1::Eosr00,
            eosr1::Eosr00,
            Eosr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P101 Event Output Set"]
    #[inline(always)]
    pub fn eosr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eosr1::Eosr01,
        eosr1::Eosr01,
        Eosr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eosr1::Eosr01,
            eosr1::Eosr01,
            Eosr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P102 Event Output Set"]
    #[inline(always)]
    pub fn eosr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eosr1::Eosr02,
        eosr1::Eosr02,
        Eosr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eosr1::Eosr02,
            eosr1::Eosr02,
            Eosr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P103 Event Output Set"]
    #[inline(always)]
    pub fn eosr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eosr1::Eosr03,
        eosr1::Eosr03,
        Eosr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eosr1::Eosr03,
            eosr1::Eosr03,
            Eosr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P108 Event Output Set"]
    #[inline(always)]
    pub fn eosr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eosr1::Eosr08,
        eosr1::Eosr08,
        Eosr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eosr1::Eosr08,
            eosr1::Eosr08,
            Eosr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P109 Event Output Set"]
    #[inline(always)]
    pub fn eosr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        eosr1::Eosr09,
        eosr1::Eosr09,
        Eosr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            eosr1::Eosr09,
            eosr1::Eosr09,
            Eosr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P110 Event Output Set"]
    #[inline(always)]
    pub fn eosr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        eosr1::Eosr10,
        eosr1::Eosr10,
        Eosr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            eosr1::Eosr10,
            eosr1::Eosr10,
            Eosr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P112 Event Output Set"]
    #[inline(always)]
    pub fn eosr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        eosr1::Eosr12,
        eosr1::Eosr12,
        Eosr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            eosr1::Eosr12,
            eosr1::Eosr12,
            Eosr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eosr1 {
    #[inline(always)]
    fn default() -> Eosr1 {
        <crate::RegValueT<Eosr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eosr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr00_SPEC;
    pub type Eosr00 = crate::EnumBitfieldStruct<u8, Eosr00_SPEC>;
    impl Eosr00 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
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
    pub struct Eosr02_SPEC;
    pub type Eosr02 = crate::EnumBitfieldStruct<u8, Eosr02_SPEC>;
    impl Eosr02 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr03_SPEC;
    pub type Eosr03 = crate::EnumBitfieldStruct<u8, Eosr03_SPEC>;
    impl Eosr03 {
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
    pub struct Eosr09_SPEC;
    pub type Eosr09 = crate::EnumBitfieldStruct<u8, Eosr09_SPEC>;
    impl Eosr09 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr10_SPEC;
    pub type Eosr10 = crate::EnumBitfieldStruct<u8, Eosr10_SPEC>;
    impl Eosr10 {
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
}
