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
// Generated from SVD 1.1, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:00:31 +0000

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

    #[doc = "Port Control Register 1"]
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

    #[doc = "Output data register"]
    #[inline(always)]
    pub const fn podr(&self) -> &'static crate::common::Reg<self::Podr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Podr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Direction register"]
    #[inline(always)]
    pub const fn pdr(&self) -> &'static crate::common::Reg<self::Pdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Port Control Register 2"]
    #[inline(always)]
    pub const fn pcntr2(&self) -> &'static crate::common::Reg<self::Pcntr2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pcntr2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Input data register"]
    #[inline(always)]
    pub const fn pidr(&self) -> &'static crate::common::Reg<self::Pidr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pidr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Port Control Register 3"]
    #[inline(always)]
    pub const fn pcntr3(&self) -> &'static crate::common::Reg<self::Pcntr3_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pcntr3_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Output reset register"]
    #[inline(always)]
    pub const fn porr(&self) -> &'static crate::common::Reg<self::Porr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Porr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Output set register"]
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

#[doc = "Port Control Register 1"]
pub type Pcntr1 = crate::RegValueT<Pcntr1_SPEC>;

impl NoBitfieldReg<Pcntr1_SPEC> for Pcntr1 {}
impl ::core::default::Default for Pcntr1 {
    #[inline(always)]
    fn default() -> Pcntr1 {
        <crate::RegValueT<Pcntr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Podr_SPEC;
impl crate::sealed::RegSpec for Podr_SPEC {
    type DataType = u16;
}

#[doc = "Output data register"]
pub type Podr = crate::RegValueT<Podr_SPEC>;

impl NoBitfieldReg<Podr_SPEC> for Podr {}
impl ::core::default::Default for Podr {
    #[inline(always)]
    fn default() -> Podr {
        <crate::RegValueT<Podr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr_SPEC;
impl crate::sealed::RegSpec for Pdr_SPEC {
    type DataType = u16;
}

#[doc = "Direction register"]
pub type Pdr = crate::RegValueT<Pdr_SPEC>;

impl NoBitfieldReg<Pdr_SPEC> for Pdr {}
impl ::core::default::Default for Pdr {
    #[inline(always)]
    fn default() -> Pdr {
        <crate::RegValueT<Pdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcntr2_SPEC;
impl crate::sealed::RegSpec for Pcntr2_SPEC {
    type DataType = u32;
}

#[doc = "Port Control Register 2"]
pub type Pcntr2 = crate::RegValueT<Pcntr2_SPEC>;

impl NoBitfieldReg<Pcntr2_SPEC> for Pcntr2 {}
impl ::core::default::Default for Pcntr2 {
    #[inline(always)]
    fn default() -> Pcntr2 {
        <crate::RegValueT<Pcntr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr_SPEC;
impl crate::sealed::RegSpec for Pidr_SPEC {
    type DataType = u16;
}

#[doc = "Input data register"]
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

#[doc = "Port Control Register 3"]
pub type Pcntr3 = crate::RegValueT<Pcntr3_SPEC>;

impl NoBitfieldReg<Pcntr3_SPEC> for Pcntr3 {}
impl ::core::default::Default for Pcntr3 {
    #[inline(always)]
    fn default() -> Pcntr3 {
        <crate::RegValueT<Pcntr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Porr_SPEC;
impl crate::sealed::RegSpec for Porr_SPEC {
    type DataType = u16;
}

#[doc = "Output reset register"]
pub type Porr = crate::RegValueT<Porr_SPEC>;

impl NoBitfieldReg<Porr_SPEC> for Porr {}
impl ::core::default::Default for Porr {
    #[inline(always)]
    fn default() -> Porr {
        <crate::RegValueT<Porr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posr_SPEC;
impl crate::sealed::RegSpec for Posr_SPEC {
    type DataType = u16;
}

#[doc = "Output set register"]
pub type Posr = crate::RegValueT<Posr_SPEC>;

impl NoBitfieldReg<Posr_SPEC> for Posr {}
impl ::core::default::Default for Posr {
    #[inline(always)]
    fn default() -> Posr {
        <crate::RegValueT<Posr_SPEC> as RegisterValue<_>>::new(0)
    }
}
