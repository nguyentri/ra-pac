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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:19:53 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Port 0 Control Registers"]
unsafe impl ::core::marker::Send for super::Port0 {}
unsafe impl ::core::marker::Sync for super::Port0 {}
impl super::Port0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn pcntr1(
        &self,
    ) -> &'static crate::common::Reg<self::Pcntr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pcntr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn podr(&self) -> &'static crate::common::Reg<self::Podr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Podr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pdr(&self) -> &'static crate::common::Reg<self::Pdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
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
                self._svd2pac_as_ptr().add(6usize),
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
    pub const fn porr(&self) -> &'static crate::common::Reg<self::Porr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Porr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn posr(&self) -> &'static crate::common::Reg<self::Posr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Posr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcntr1_SPEC;
impl crate::sealed::RegSpec for Pcntr1_SPEC {
    type DataType = u32;
}

pub type Pcntr1 = crate::RegValueT<Pcntr1_SPEC>;

impl Pcntr1 {
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        pcntr1::Podr,
        pcntr1::Podr,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            pcntr1::Podr,
            pcntr1::Podr,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        pcntr1::Pdr,
        pcntr1::Pdr,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            pcntr1::Pdr,
            pcntr1::Pdr,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pcntr1 {
    #[inline(always)]
    fn default() -> Pcntr1 {
        <crate::RegValueT<Pcntr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pcntr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Podr_SPEC;
impl crate::sealed::RegSpec for Podr_SPEC {
    type DataType = u16;
}

pub type Podr = crate::RegValueT<Podr_SPEC>;

impl Podr {
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        podr::Podr,
        podr::Podr,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            podr::Podr,
            podr::Podr,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Podr {
    #[inline(always)]
    fn default() -> Podr {
        <crate::RegValueT<Podr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod podr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr_SPEC;
impl crate::sealed::RegSpec for Pdr_SPEC {
    type DataType = u16;
}

pub type Pdr = crate::RegValueT<Pdr_SPEC>;

impl Pdr {
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        pdr::Pdr,
        pdr::Pdr,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            pdr::Pdr,
            pdr::Pdr,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdr {
    #[inline(always)]
    fn default() -> Pdr {
        <crate::RegValueT<Pdr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
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
    pub fn eidr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        pcntr2::Eidr,
        pcntr2::Eidr,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            pcntr2::Eidr,
            pcntr2::Eidr,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        pcntr2::Pidr,
        pcntr2::Pidr,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            pcntr2::Pidr,
            pcntr2::Pidr,
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
    pub struct Eidr_SPEC;
    pub type Eidr = crate::EnumBitfieldStruct<u8, Eidr_SPEC>;
    impl Eidr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
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

impl Pidr {
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        pidr::Pidr,
        pidr::Pidr,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            pidr::Pidr,
            pidr::Pidr,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pidr {
    #[inline(always)]
    fn default() -> Pidr {
        <crate::RegValueT<Pidr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pidr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
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
    pub fn porr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        pcntr3::Porr,
        pcntr3::Porr,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            pcntr3::Porr,
            pcntr3::Porr,
            Pcntr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        pcntr3::Posr,
        pcntr3::Posr,
        Pcntr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            pcntr3::Posr,
            pcntr3::Posr,
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
    pub struct Porr_SPEC;
    pub type Porr = crate::EnumBitfieldStruct<u8, Porr_SPEC>;
    impl Porr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr_SPEC;
    pub type Posr = crate::EnumBitfieldStruct<u8, Posr_SPEC>;
    impl Posr {
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
    pub fn porr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        porr::Porr,
        porr::Porr,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            porr::Porr,
            porr::Porr,
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
    pub struct Porr_SPEC;
    pub type Porr = crate::EnumBitfieldStruct<u8, Porr_SPEC>;
    impl Porr {
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
    pub fn posr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        posr::Posr,
        posr::Posr,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            posr::Posr,
            posr::Posr,
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
    pub struct Posr_SPEC;
    pub type Posr = crate::EnumBitfieldStruct<u8, Posr_SPEC>;
    impl Posr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
