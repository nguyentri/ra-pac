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
#[doc = r"Port 0 Control"]
unsafe impl ::core::marker::Send for super::Port0 {}
unsafe impl ::core::marker::Sync for super::Port0 {}
impl super::Port0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Port 0 Output Data Register"]
    #[inline(always)]
    pub const fn podr0(&self) -> &'static crate::common::Reg<self::Podr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Podr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Port 0 Direction Register"]
    #[inline(always)]
    pub const fn pdr0(&self) -> &'static crate::common::Reg<self::Pdr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Port 0 State Register"]
    #[inline(always)]
    pub const fn pidr0(&self) -> &'static crate::common::Reg<self::Pidr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pidr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Port 0 Output Reset Register"]
    #[inline(always)]
    pub const fn porr0(&self) -> &'static crate::common::Reg<self::Porr0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Porr0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Port 0 Output Set Register"]
    #[inline(always)]
    pub const fn posr0(&self) -> &'static crate::common::Reg<self::Posr0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Posr0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Podr0_SPEC;
impl crate::sealed::RegSpec for Podr0_SPEC {
    type DataType = u16;
}

#[doc = "Port 0 Output Data Register"]
pub type Podr0 = crate::RegValueT<Podr0_SPEC>;

impl Podr0 {
    #[doc = "P008 Output Data"]
    #[inline(always)]
    pub fn podr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        podr0::Podr08,
        podr0::Podr08,
        Podr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            podr0::Podr08,
            podr0::Podr08,
            Podr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P009 Output Data"]
    #[inline(always)]
    pub fn podr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        podr0::Podr09,
        podr0::Podr09,
        Podr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            podr0::Podr09,
            podr0::Podr09,
            Podr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P010 Output Data"]
    #[inline(always)]
    pub fn podr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        podr0::Podr10,
        podr0::Podr10,
        Podr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            podr0::Podr10,
            podr0::Podr10,
            Podr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P011 Output Data"]
    #[inline(always)]
    pub fn podr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        podr0::Podr11,
        podr0::Podr11,
        Podr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            podr0::Podr11,
            podr0::Podr11,
            Podr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P012 Output Data"]
    #[inline(always)]
    pub fn podr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        podr0::Podr12,
        podr0::Podr12,
        Podr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            podr0::Podr12,
            podr0::Podr12,
            Podr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P013 Output Data"]
    #[inline(always)]
    pub fn podr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        podr0::Podr13,
        podr0::Podr13,
        Podr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            podr0::Podr13,
            podr0::Podr13,
            Podr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P014 Output Data"]
    #[inline(always)]
    pub fn podr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        podr0::Podr14,
        podr0::Podr14,
        Podr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            podr0::Podr14,
            podr0::Podr14,
            Podr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P015 Output Data"]
    #[inline(always)]
    pub fn podr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        podr0::Podr15,
        podr0::Podr15,
        Podr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            podr0::Podr15,
            podr0::Podr15,
            Podr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Podr0 {
    #[inline(always)]
    fn default() -> Podr0 {
        <crate::RegValueT<Podr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod podr0 {

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
    pub struct Podr11_SPEC;
    pub type Podr11 = crate::EnumBitfieldStruct<u8, Podr11_SPEC>;
    impl Podr11 {
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr14_SPEC;
    pub type Podr14 = crate::EnumBitfieldStruct<u8, Podr14_SPEC>;
    impl Podr14 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr15_SPEC;
    pub type Podr15 = crate::EnumBitfieldStruct<u8, Podr15_SPEC>;
    impl Podr15 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr0_SPEC;
impl crate::sealed::RegSpec for Pdr0_SPEC {
    type DataType = u16;
}

#[doc = "Port 0 Direction Register"]
pub type Pdr0 = crate::RegValueT<Pdr0_SPEC>;

impl Pdr0 {
    #[doc = "P008 Direction"]
    #[inline(always)]
    pub fn pdr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pdr0::Pdr08,
        pdr0::Pdr08,
        Pdr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pdr0::Pdr08,
            pdr0::Pdr08,
            Pdr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P009 Direction"]
    #[inline(always)]
    pub fn pdr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pdr0::Pdr09,
        pdr0::Pdr09,
        Pdr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pdr0::Pdr09,
            pdr0::Pdr09,
            Pdr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P010 Direction"]
    #[inline(always)]
    pub fn pdr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pdr0::Pdr10,
        pdr0::Pdr10,
        Pdr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pdr0::Pdr10,
            pdr0::Pdr10,
            Pdr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P011 Direction"]
    #[inline(always)]
    pub fn pdr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pdr0::Pdr11,
        pdr0::Pdr11,
        Pdr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pdr0::Pdr11,
            pdr0::Pdr11,
            Pdr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P012 Direction"]
    #[inline(always)]
    pub fn pdr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pdr0::Pdr12,
        pdr0::Pdr12,
        Pdr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pdr0::Pdr12,
            pdr0::Pdr12,
            Pdr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P013 Direction"]
    #[inline(always)]
    pub fn pdr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pdr0::Pdr13,
        pdr0::Pdr13,
        Pdr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pdr0::Pdr13,
            pdr0::Pdr13,
            Pdr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P014 Direction"]
    #[inline(always)]
    pub fn pdr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pdr0::Pdr14,
        pdr0::Pdr14,
        Pdr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pdr0::Pdr14,
            pdr0::Pdr14,
            Pdr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P015 Direction"]
    #[inline(always)]
    pub fn pdr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pdr0::Pdr15,
        pdr0::Pdr15,
        Pdr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pdr0::Pdr15,
            pdr0::Pdr15,
            Pdr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdr0 {
    #[inline(always)]
    fn default() -> Pdr0 {
        <crate::RegValueT<Pdr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdr0 {

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
    pub struct Pdr11_SPEC;
    pub type Pdr11 = crate::EnumBitfieldStruct<u8, Pdr11_SPEC>;
    impl Pdr11 {
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr14_SPEC;
    pub type Pdr14 = crate::EnumBitfieldStruct<u8, Pdr14_SPEC>;
    impl Pdr14 {
        #[doc = "Output 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr15_SPEC;
    pub type Pdr15 = crate::EnumBitfieldStruct<u8, Pdr15_SPEC>;
    impl Pdr15 {
        #[doc = "Output 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr0_SPEC;
impl crate::sealed::RegSpec for Pidr0_SPEC {
    type DataType = u16;
}

#[doc = "Port 0 State Register"]
pub type Pidr0 = crate::RegValueT<Pidr0_SPEC>;

impl Pidr0 {
    #[doc = "P008 State"]
    #[inline(always)]
    pub fn pidr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pidr0::Pidr08,
        pidr0::Pidr08,
        Pidr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pidr0::Pidr08,
            pidr0::Pidr08,
            Pidr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P009 State"]
    #[inline(always)]
    pub fn pidr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pidr0::Pidr09,
        pidr0::Pidr09,
        Pidr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pidr0::Pidr09,
            pidr0::Pidr09,
            Pidr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P010 State"]
    #[inline(always)]
    pub fn pidr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pidr0::Pidr10,
        pidr0::Pidr10,
        Pidr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pidr0::Pidr10,
            pidr0::Pidr10,
            Pidr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P011 State"]
    #[inline(always)]
    pub fn pidr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pidr0::Pidr11,
        pidr0::Pidr11,
        Pidr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pidr0::Pidr11,
            pidr0::Pidr11,
            Pidr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P012 State"]
    #[inline(always)]
    pub fn pidr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pidr0::Pidr12,
        pidr0::Pidr12,
        Pidr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pidr0::Pidr12,
            pidr0::Pidr12,
            Pidr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P013 State"]
    #[inline(always)]
    pub fn pidr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pidr0::Pidr13,
        pidr0::Pidr13,
        Pidr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pidr0::Pidr13,
            pidr0::Pidr13,
            Pidr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P014 State"]
    #[inline(always)]
    pub fn pidr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pidr0::Pidr14,
        pidr0::Pidr14,
        Pidr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pidr0::Pidr14,
            pidr0::Pidr14,
            Pidr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P015 State"]
    #[inline(always)]
    pub fn pidr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pidr0::Pidr15,
        pidr0::Pidr15,
        Pidr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pidr0::Pidr15,
            pidr0::Pidr15,
            Pidr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pidr0 {
    #[inline(always)]
    fn default() -> Pidr0 {
        <crate::RegValueT<Pidr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pidr0 {

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
    pub struct Pidr11_SPEC;
    pub type Pidr11 = crate::EnumBitfieldStruct<u8, Pidr11_SPEC>;
    impl Pidr11 {
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
pub struct Porr0_SPEC;
impl crate::sealed::RegSpec for Porr0_SPEC {
    type DataType = u16;
}

#[doc = "Port 0 Output Reset Register"]
pub type Porr0 = crate::RegValueT<Porr0_SPEC>;

impl Porr0 {
    #[doc = "P008 Output Reset"]
    #[inline(always)]
    pub fn porr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        porr0::Porr08,
        porr0::Porr08,
        Porr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            porr0::Porr08,
            porr0::Porr08,
            Porr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P009 Output Reset"]
    #[inline(always)]
    pub fn porr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        porr0::Porr09,
        porr0::Porr09,
        Porr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            porr0::Porr09,
            porr0::Porr09,
            Porr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P010 Output Reset"]
    #[inline(always)]
    pub fn porr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        porr0::Porr10,
        porr0::Porr10,
        Porr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            porr0::Porr10,
            porr0::Porr10,
            Porr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P011 Output Reset"]
    #[inline(always)]
    pub fn porr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        porr0::Porr11,
        porr0::Porr11,
        Porr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            porr0::Porr11,
            porr0::Porr11,
            Porr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P012 Output Reset"]
    #[inline(always)]
    pub fn porr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        porr0::Porr12,
        porr0::Porr12,
        Porr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            porr0::Porr12,
            porr0::Porr12,
            Porr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P013 Output Reset"]
    #[inline(always)]
    pub fn porr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        porr0::Porr13,
        porr0::Porr13,
        Porr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            porr0::Porr13,
            porr0::Porr13,
            Porr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P014 Output Reset"]
    #[inline(always)]
    pub fn porr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        porr0::Porr14,
        porr0::Porr14,
        Porr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            porr0::Porr14,
            porr0::Porr14,
            Porr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P015 Output Reset"]
    #[inline(always)]
    pub fn porr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        porr0::Porr15,
        porr0::Porr15,
        Porr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            porr0::Porr15,
            porr0::Porr15,
            Porr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Porr0 {
    #[inline(always)]
    fn default() -> Porr0 {
        <crate::RegValueT<Porr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod porr0 {

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
    pub struct Porr11_SPEC;
    pub type Porr11 = crate::EnumBitfieldStruct<u8, Porr11_SPEC>;
    impl Porr11 {
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr14_SPEC;
    pub type Porr14 = crate::EnumBitfieldStruct<u8, Porr14_SPEC>;
    impl Porr14 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr15_SPEC;
    pub type Porr15 = crate::EnumBitfieldStruct<u8, Porr15_SPEC>;
    impl Porr15 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posr0_SPEC;
impl crate::sealed::RegSpec for Posr0_SPEC {
    type DataType = u16;
}

#[doc = "Port 0 Output Set Register"]
pub type Posr0 = crate::RegValueT<Posr0_SPEC>;

impl Posr0 {
    #[doc = "P008 Output Set"]
    #[inline(always)]
    pub fn posr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        posr0::Posr08,
        posr0::Posr08,
        Posr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            posr0::Posr08,
            posr0::Posr08,
            Posr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P009 Output Set"]
    #[inline(always)]
    pub fn posr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        posr0::Posr09,
        posr0::Posr09,
        Posr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            posr0::Posr09,
            posr0::Posr09,
            Posr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P010 Output Set"]
    #[inline(always)]
    pub fn posr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        posr0::Posr10,
        posr0::Posr10,
        Posr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            posr0::Posr10,
            posr0::Posr10,
            Posr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P011 Output Set"]
    #[inline(always)]
    pub fn posr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        posr0::Posr11,
        posr0::Posr11,
        Posr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            posr0::Posr11,
            posr0::Posr11,
            Posr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P012 Output Set"]
    #[inline(always)]
    pub fn posr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        posr0::Posr12,
        posr0::Posr12,
        Posr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            posr0::Posr12,
            posr0::Posr12,
            Posr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P013 Output Set"]
    #[inline(always)]
    pub fn posr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        posr0::Posr13,
        posr0::Posr13,
        Posr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            posr0::Posr13,
            posr0::Posr13,
            Posr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P014 Output Set"]
    #[inline(always)]
    pub fn posr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        posr0::Posr14,
        posr0::Posr14,
        Posr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            posr0::Posr14,
            posr0::Posr14,
            Posr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P015 Output Set"]
    #[inline(always)]
    pub fn posr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        posr0::Posr15,
        posr0::Posr15,
        Posr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            posr0::Posr15,
            posr0::Posr15,
            Posr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Posr0 {
    #[inline(always)]
    fn default() -> Posr0 {
        <crate::RegValueT<Posr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod posr0 {

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
    pub struct Posr11_SPEC;
    pub type Posr11 = crate::EnumBitfieldStruct<u8, Posr11_SPEC>;
    impl Posr11 {
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr14_SPEC;
    pub type Posr14 = crate::EnumBitfieldStruct<u8, Posr14_SPEC>;
    impl Posr14 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr15_SPEC;
    pub type Posr15 = crate::EnumBitfieldStruct<u8, Posr15_SPEC>;
    impl Posr15 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
