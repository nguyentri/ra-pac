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
#[doc = r"Port 4 Control"]
unsafe impl ::core::marker::Send for super::Port4 {}
unsafe impl ::core::marker::Sync for super::Port4 {}
impl super::Port4 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Port 4 Output Data Register"]
    #[inline(always)]
    pub const fn podr4(&self) -> &'static crate::common::Reg<self::Podr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Podr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Port 4 Direction Register"]
    #[inline(always)]
    pub const fn pdr4(&self) -> &'static crate::common::Reg<self::Pdr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Port 4 State Register"]
    #[inline(always)]
    pub const fn pidr4(&self) -> &'static crate::common::Reg<self::Pidr4_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pidr4_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Port 4 Output Reset Register"]
    #[inline(always)]
    pub const fn porr4(&self) -> &'static crate::common::Reg<self::Porr4_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Porr4_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Port 4 Output Set Register"]
    #[inline(always)]
    pub const fn posr4(&self) -> &'static crate::common::Reg<self::Posr4_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Posr4_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Podr4_SPEC;
impl crate::sealed::RegSpec for Podr4_SPEC {
    type DataType = u16;
}

#[doc = "Port 4 Output Data Register"]
pub type Podr4 = crate::RegValueT<Podr4_SPEC>;

impl Podr4 {
    #[doc = "P407 Output Data"]
    #[inline(always)]
    pub fn podr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        podr4::Podr07,
        podr4::Podr07,
        Podr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            podr4::Podr07,
            podr4::Podr07,
            Podr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Podr4 {
    #[inline(always)]
    fn default() -> Podr4 {
        <crate::RegValueT<Podr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod podr4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr07_SPEC;
    pub type Podr07 = crate::EnumBitfieldStruct<u8, Podr07_SPEC>;
    impl Podr07 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr4_SPEC;
impl crate::sealed::RegSpec for Pdr4_SPEC {
    type DataType = u16;
}

#[doc = "Port 4 Direction Register"]
pub type Pdr4 = crate::RegValueT<Pdr4_SPEC>;

impl Pdr4 {
    #[doc = "P407 Direction"]
    #[inline(always)]
    pub fn pdr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pdr4::Pdr07,
        pdr4::Pdr07,
        Pdr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pdr4::Pdr07,
            pdr4::Pdr07,
            Pdr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdr4 {
    #[inline(always)]
    fn default() -> Pdr4 {
        <crate::RegValueT<Pdr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdr4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr07_SPEC;
    pub type Pdr07 = crate::EnumBitfieldStruct<u8, Pdr07_SPEC>;
    impl Pdr07 {
        #[doc = "Output 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr4_SPEC;
impl crate::sealed::RegSpec for Pidr4_SPEC {
    type DataType = u16;
}

#[doc = "Port 4 State Register"]
pub type Pidr4 = crate::RegValueT<Pidr4_SPEC>;

impl Pidr4 {
    #[doc = "P407 State"]
    #[inline(always)]
    pub fn pidr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pidr4::Pidr07,
        pidr4::Pidr07,
        Pidr4_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pidr4::Pidr07,
            pidr4::Pidr07,
            Pidr4_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pidr4 {
    #[inline(always)]
    fn default() -> Pidr4 {
        <crate::RegValueT<Pidr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pidr4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr07_SPEC;
    pub type Pidr07 = crate::EnumBitfieldStruct<u8, Pidr07_SPEC>;
    impl Pidr07 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Porr4_SPEC;
impl crate::sealed::RegSpec for Porr4_SPEC {
    type DataType = u16;
}

#[doc = "Port 4 Output Reset Register"]
pub type Porr4 = crate::RegValueT<Porr4_SPEC>;

impl Porr4 {
    #[doc = "P407 Output Reset"]
    #[inline(always)]
    pub fn porr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        porr4::Porr07,
        porr4::Porr07,
        Porr4_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            porr4::Porr07,
            porr4::Porr07,
            Porr4_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Porr4 {
    #[inline(always)]
    fn default() -> Porr4 {
        <crate::RegValueT<Porr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod porr4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr07_SPEC;
    pub type Porr07 = crate::EnumBitfieldStruct<u8, Porr07_SPEC>;
    impl Porr07 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posr4_SPEC;
impl crate::sealed::RegSpec for Posr4_SPEC {
    type DataType = u16;
}

#[doc = "Port 4 Output Set Register"]
pub type Posr4 = crate::RegValueT<Posr4_SPEC>;

impl Posr4 {
    #[doc = "P407 Output Set"]
    #[inline(always)]
    pub fn posr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        posr4::Posr07,
        posr4::Posr07,
        Posr4_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            posr4::Posr07,
            posr4::Posr07,
            Posr4_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Posr4 {
    #[inline(always)]
    fn default() -> Posr4 {
        <crate::RegValueT<Posr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod posr4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr07_SPEC;
    pub type Posr07 = crate::EnumBitfieldStruct<u8, Posr07_SPEC>;
    impl Posr07 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
