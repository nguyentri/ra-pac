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
#[doc = r"Port 9 Control"]
unsafe impl ::core::marker::Send for super::Port9 {}
unsafe impl ::core::marker::Sync for super::Port9 {}
impl super::Port9 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Port 9 Output Data Register"]
    #[inline(always)]
    pub const fn podr9(&self) -> &'static crate::common::Reg<self::Podr9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Podr9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Port 9 Direction Register"]
    #[inline(always)]
    pub const fn pdr9(&self) -> &'static crate::common::Reg<self::Pdr9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdr9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Port 9 State Register"]
    #[inline(always)]
    pub const fn pidr9(&self) -> &'static crate::common::Reg<self::Pidr9_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pidr9_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Port 9 Output Reset Register"]
    #[inline(always)]
    pub const fn porr9(&self) -> &'static crate::common::Reg<self::Porr9_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Porr9_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Port 9 Output Set Register"]
    #[inline(always)]
    pub const fn posr9(&self) -> &'static crate::common::Reg<self::Posr9_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Posr9_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Podr9_SPEC;
impl crate::sealed::RegSpec for Podr9_SPEC {
    type DataType = u16;
}

#[doc = "Port 9 Output Data Register"]
pub type Podr9 = crate::RegValueT<Podr9_SPEC>;

impl Podr9 {
    #[doc = "P913 Output Data"]
    #[inline(always)]
    pub fn podr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        podr9::Podr13,
        podr9::Podr13,
        Podr9_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            podr9::Podr13,
            podr9::Podr13,
            Podr9_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P914 Output Data"]
    #[inline(always)]
    pub fn podr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        podr9::Podr14,
        podr9::Podr14,
        Podr9_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            podr9::Podr14,
            podr9::Podr14,
            Podr9_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Podr9 {
    #[inline(always)]
    fn default() -> Podr9 {
        <crate::RegValueT<Podr9_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod podr9 {

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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr9_SPEC;
impl crate::sealed::RegSpec for Pdr9_SPEC {
    type DataType = u16;
}

#[doc = "Port 9 Direction Register"]
pub type Pdr9 = crate::RegValueT<Pdr9_SPEC>;

impl Pdr9 {
    #[doc = "P913 Direction"]
    #[inline(always)]
    pub fn pdr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pdr9::Pdr13,
        pdr9::Pdr13,
        Pdr9_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pdr9::Pdr13,
            pdr9::Pdr13,
            Pdr9_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P914 Direction"]
    #[inline(always)]
    pub fn pdr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pdr9::Pdr14,
        pdr9::Pdr14,
        Pdr9_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pdr9::Pdr14,
            pdr9::Pdr14,
            Pdr9_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdr9 {
    #[inline(always)]
    fn default() -> Pdr9 {
        <crate::RegValueT<Pdr9_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdr9 {

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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr9_SPEC;
impl crate::sealed::RegSpec for Pidr9_SPEC {
    type DataType = u16;
}

#[doc = "Port 9 State Register"]
pub type Pidr9 = crate::RegValueT<Pidr9_SPEC>;

impl Pidr9 {
    #[doc = "P913 State"]
    #[inline(always)]
    pub fn pidr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pidr9::Pidr13,
        pidr9::Pidr13,
        Pidr9_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pidr9::Pidr13,
            pidr9::Pidr13,
            Pidr9_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P914 State"]
    #[inline(always)]
    pub fn pidr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pidr9::Pidr14,
        pidr9::Pidr14,
        Pidr9_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pidr9::Pidr14,
            pidr9::Pidr14,
            Pidr9_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pidr9 {
    #[inline(always)]
    fn default() -> Pidr9 {
        <crate::RegValueT<Pidr9_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pidr9 {

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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Porr9_SPEC;
impl crate::sealed::RegSpec for Porr9_SPEC {
    type DataType = u16;
}

#[doc = "Port 9 Output Reset Register"]
pub type Porr9 = crate::RegValueT<Porr9_SPEC>;

impl Porr9 {
    #[doc = "P913 Output Reset"]
    #[inline(always)]
    pub fn porr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        porr9::Porr13,
        porr9::Porr13,
        Porr9_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            porr9::Porr13,
            porr9::Porr13,
            Porr9_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P914 Output Reset"]
    #[inline(always)]
    pub fn porr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        porr9::Porr14,
        porr9::Porr14,
        Porr9_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            porr9::Porr14,
            porr9::Porr14,
            Porr9_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Porr9 {
    #[inline(always)]
    fn default() -> Porr9 {
        <crate::RegValueT<Porr9_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod porr9 {

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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posr9_SPEC;
impl crate::sealed::RegSpec for Posr9_SPEC {
    type DataType = u16;
}

#[doc = "Port 9 Output Set Register"]
pub type Posr9 = crate::RegValueT<Posr9_SPEC>;

impl Posr9 {
    #[doc = "P913 Output Set"]
    #[inline(always)]
    pub fn posr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        posr9::Posr13,
        posr9::Posr13,
        Posr9_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            posr9::Posr13,
            posr9::Posr13,
            Posr9_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "P914 Output Set"]
    #[inline(always)]
    pub fn posr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        posr9::Posr14,
        posr9::Posr14,
        Posr9_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            posr9::Posr14,
            posr9::Posr14,
            Posr9_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Posr9 {
    #[inline(always)]
    fn default() -> Posr9 {
        <crate::RegValueT<Posr9_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod posr9 {

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
}
