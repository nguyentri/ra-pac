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
#[doc = r"Port 3 Control"]
unsafe impl ::core::marker::Send for super::Port3 {}
unsafe impl ::core::marker::Sync for super::Port3 {}
impl super::Port3 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Port 3 Output Data Register"]
    #[inline(always)]
    pub const fn podr3(&self) -> &'static crate::common::Reg<self::Podr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Podr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Port 3 Direction Register"]
    #[inline(always)]
    pub const fn pdr3(&self) -> &'static crate::common::Reg<self::Pdr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Port 3 State Register"]
    #[inline(always)]
    pub const fn pidr3(&self) -> &'static crate::common::Reg<self::Pidr3_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pidr3_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Port 3 Output Reset Register"]
    #[inline(always)]
    pub const fn porr3(&self) -> &'static crate::common::Reg<self::Porr3_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Porr3_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Port 3 Output Set Register"]
    #[inline(always)]
    pub const fn posr3(&self) -> &'static crate::common::Reg<self::Posr3_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Posr3_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Podr3_SPEC;
impl crate::sealed::RegSpec for Podr3_SPEC {
    type DataType = u16;
}

#[doc = "Port 3 Output Data Register"]
pub type Podr3 = crate::RegValueT<Podr3_SPEC>;

impl Podr3 {
    #[doc = "P300 Output Data"]
    #[inline(always)]
    pub fn podr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        podr3::Podr00,
        podr3::Podr00,
        Podr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            podr3::Podr00,
            podr3::Podr00,
            Podr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Podr3 {
    #[inline(always)]
    fn default() -> Podr3 {
        <crate::RegValueT<Podr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod podr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr00_SPEC;
    pub type Podr00 = crate::EnumBitfieldStruct<u8, Podr00_SPEC>;
    impl Podr00 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr3_SPEC;
impl crate::sealed::RegSpec for Pdr3_SPEC {
    type DataType = u16;
}

#[doc = "Port 3 Direction Register"]
pub type Pdr3 = crate::RegValueT<Pdr3_SPEC>;

impl Pdr3 {
    #[doc = "P300 Direction"]
    #[inline(always)]
    pub fn pdr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdr3::Pdr00,
        pdr3::Pdr00,
        Pdr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdr3::Pdr00,
            pdr3::Pdr00,
            Pdr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdr3 {
    #[inline(always)]
    fn default() -> Pdr3 {
        <crate::RegValueT<Pdr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr00_SPEC;
    pub type Pdr00 = crate::EnumBitfieldStruct<u8, Pdr00_SPEC>;
    impl Pdr00 {
        #[doc = "Output 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr3_SPEC;
impl crate::sealed::RegSpec for Pidr3_SPEC {
    type DataType = u16;
}

#[doc = "Port 3 State Register"]
pub type Pidr3 = crate::RegValueT<Pidr3_SPEC>;

impl Pidr3 {
    #[doc = "P300 State"]
    #[inline(always)]
    pub fn pidr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pidr3::Pidr00,
        pidr3::Pidr00,
        Pidr3_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pidr3::Pidr00,
            pidr3::Pidr00,
            Pidr3_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pidr3 {
    #[inline(always)]
    fn default() -> Pidr3 {
        <crate::RegValueT<Pidr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pidr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr00_SPEC;
    pub type Pidr00 = crate::EnumBitfieldStruct<u8, Pidr00_SPEC>;
    impl Pidr00 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Porr3_SPEC;
impl crate::sealed::RegSpec for Porr3_SPEC {
    type DataType = u16;
}

#[doc = "Port 3 Output Reset Register"]
pub type Porr3 = crate::RegValueT<Porr3_SPEC>;

impl Porr3 {
    #[doc = "P300 Output Reset"]
    #[inline(always)]
    pub fn porr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        porr3::Porr00,
        porr3::Porr00,
        Porr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            porr3::Porr00,
            porr3::Porr00,
            Porr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Porr3 {
    #[inline(always)]
    fn default() -> Porr3 {
        <crate::RegValueT<Porr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod porr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr00_SPEC;
    pub type Porr00 = crate::EnumBitfieldStruct<u8, Porr00_SPEC>;
    impl Porr00 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posr3_SPEC;
impl crate::sealed::RegSpec for Posr3_SPEC {
    type DataType = u16;
}

#[doc = "Port 3 Output Set Register"]
pub type Posr3 = crate::RegValueT<Posr3_SPEC>;

impl Posr3 {
    #[doc = "P300 Output Set"]
    #[inline(always)]
    pub fn posr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        posr3::Posr00,
        posr3::Posr00,
        Posr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            posr3::Posr00,
            posr3::Posr00,
            Posr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Posr3 {
    #[inline(always)]
    fn default() -> Posr3 {
        <crate::RegValueT<Posr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod posr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr00_SPEC;
    pub type Posr00 = crate::EnumBitfieldStruct<u8, Posr00_SPEC>;
    impl Posr00 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
