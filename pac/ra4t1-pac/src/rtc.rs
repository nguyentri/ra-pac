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
// Generated from SVD 1.20.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:50:08 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Realtime Clock"]
unsafe impl ::core::marker::Send for super::Rtc {}
unsafe impl ::core::marker::Sync for super::Rtc {}
impl super::Rtc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Reset Control Register 1"]
    #[inline(always)]
    pub const fn rcr1(&self) -> &'static crate::common::Reg<self::Rcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[doc = "Reset Control Register 2"]
    #[inline(always)]
    pub const fn rcr2(&self) -> &'static crate::common::Reg<self::Rcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Reset Control Register 4"]
    #[inline(always)]
    pub const fn rcr4(&self) -> &'static crate::common::Reg<self::Rcr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rcr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Time Capture Control Register %s"]
    #[inline(always)]
    pub const fn rtccr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Rtccr_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }
    #[inline(always)]
    pub const fn rtccr0(&self) -> &'static crate::common::Reg<self::Rtccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rtccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn rtccr1(&self) -> &'static crate::common::Reg<self::Rtccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rtccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x42usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr1_SPEC;
impl crate::sealed::RegSpec for Rcr1_SPEC {
    type DataType = u8;
}

#[doc = "Reset Control Register 1"]
pub type Rcr1 = crate::RegValueT<Rcr1_SPEC>;

impl NoBitfieldReg<Rcr1_SPEC> for Rcr1 {}
impl ::core::default::Default for Rcr1 {
    #[inline(always)]
    fn default() -> Rcr1 {
        <crate::RegValueT<Rcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr2_SPEC;
impl crate::sealed::RegSpec for Rcr2_SPEC {
    type DataType = u8;
}

#[doc = "Reset Control Register 2"]
pub type Rcr2 = crate::RegValueT<Rcr2_SPEC>;

impl Rcr2 {
    #[doc = "Software Reset"]
    #[inline(always)]
    pub fn reset(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rcr2::Reset,
        rcr2::Reset,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rcr2::Reset,
            rcr2::Reset,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rcr2 {
    #[inline(always)]
    fn default() -> Rcr2 {
        <crate::RegValueT<Rcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reset_SPEC;
    pub type Reset = crate::EnumBitfieldStruct<u8, Reset_SPEC>;
    impl Reset {
        #[doc = "In writing: Invalid (writing 0 has no effect) In reading: Software reset has completed."]
        pub const _0: Self = Self::new(0);

        #[doc = "In writing: The target registers for software reset are initialized. In reading: Software reset in progress."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr4_SPEC;
impl crate::sealed::RegSpec for Rcr4_SPEC {
    type DataType = u8;
}

#[doc = "Reset Control Register 4"]
pub type Rcr4 = crate::RegValueT<Rcr4_SPEC>;

impl Rcr4 {
    #[doc = "Count Source Select"]
    #[inline(always)]
    pub fn rcksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rcr4::Rcksel,
        rcr4::Rcksel,
        Rcr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rcr4::Rcksel,
            rcr4::Rcksel,
            Rcr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rcr4 {
    #[inline(always)]
    fn default() -> Rcr4 {
        <crate::RegValueT<Rcr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rcr4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcksel_SPEC;
    pub type Rcksel = crate::EnumBitfieldStruct<u8, Rcksel_SPEC>;
    impl Rcksel {
        #[doc = "Sub-clock oscillator is selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "LOCO is selected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtccr_SPEC;
impl crate::sealed::RegSpec for Rtccr_SPEC {
    type DataType = u8;
}

#[doc = "Time Capture Control Register %s"]
pub type Rtccr = crate::RegValueT<Rtccr_SPEC>;

impl Rtccr {
    #[doc = "P402/AGTIO and P403/AGTIO input enable"]
    #[inline(always)]
    pub fn tcen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rtccr::Tcen,
        rtccr::Tcen,
        Rtccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rtccr::Tcen,
            rtccr::Tcen,
            Rtccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rtccr {
    #[inline(always)]
    fn default() -> Rtccr {
        <crate::RegValueT<Rtccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rtccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcen_SPEC;
    pub type Tcen = crate::EnumBitfieldStruct<u8, Tcen_SPEC>;
    impl Tcen {
        #[doc = "AGTIO input disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "AGTIO input enable"]
        pub const _1: Self = Self::new(1);
    }
}
