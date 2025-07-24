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
// Generated from SVD 1.50.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:47:04 +0000

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

    #[doc = "Port Control Register 1"]
    #[inline(always)]
    pub const fn podr(&self) -> &'static crate::common::Reg<self::Podr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Podr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Port Control Register 1"]
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

    #[doc = "Port Control Register 2"]
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

    #[doc = "Port Control Register 3"]
    #[inline(always)]
    pub const fn porr(&self) -> &'static crate::common::Reg<self::Porr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Porr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Port Control Register 3"]
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

impl Pcntr1 {
    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pcntr1::Pdr00,
        pcntr1::Pdr00,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pcntr1::Pdr00,
            pcntr1::Pdr00,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pcntr1::Pdr01,
        pcntr1::Pdr01,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pcntr1::Pdr01,
            pcntr1::Pdr01,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pcntr1::Pdr02,
        pcntr1::Pdr02,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pcntr1::Pdr02,
            pcntr1::Pdr02,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pcntr1::Pdr03,
        pcntr1::Pdr03,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pcntr1::Pdr03,
            pcntr1::Pdr03,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pcntr1::Pdr04,
        pcntr1::Pdr04,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pcntr1::Pdr04,
            pcntr1::Pdr04,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pcntr1::Pdr05,
        pcntr1::Pdr05,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pcntr1::Pdr05,
            pcntr1::Pdr05,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pcntr1::Pdr06,
        pcntr1::Pdr06,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pcntr1::Pdr06,
            pcntr1::Pdr06,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pcntr1::Pdr07,
        pcntr1::Pdr07,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pcntr1::Pdr07,
            pcntr1::Pdr07,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pcntr1::Pdr08,
        pcntr1::Pdr08,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pcntr1::Pdr08,
            pcntr1::Pdr08,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pcntr1::Pdr09,
        pcntr1::Pdr09,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pcntr1::Pdr09,
            pcntr1::Pdr09,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pcntr1::Pdr10,
        pcntr1::Pdr10,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pcntr1::Pdr10,
            pcntr1::Pdr10,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pcntr1::Pdr11,
        pcntr1::Pdr11,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pcntr1::Pdr11,
            pcntr1::Pdr11,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pcntr1::Pdr12,
        pcntr1::Pdr12,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pcntr1::Pdr12,
            pcntr1::Pdr12,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pcntr1::Pdr13,
        pcntr1::Pdr13,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pcntr1::Pdr13,
            pcntr1::Pdr13,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pcntr1::Pdr14,
        pcntr1::Pdr14,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pcntr1::Pdr14,
            pcntr1::Pdr14,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pcntr1::Pdr15,
        pcntr1::Pdr15,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pcntr1::Pdr15,
            pcntr1::Pdr15,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr00(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        pcntr1::Podr00,
        pcntr1::Podr00,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pcntr1::Podr00,
            pcntr1::Podr00,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr01(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        pcntr1::Podr01,
        pcntr1::Podr01,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            pcntr1::Podr01,
            pcntr1::Podr01,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr02(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        pcntr1::Podr02,
        pcntr1::Podr02,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            pcntr1::Podr02,
            pcntr1::Podr02,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr03(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        pcntr1::Podr03,
        pcntr1::Podr03,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            pcntr1::Podr03,
            pcntr1::Podr03,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr04(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        pcntr1::Podr04,
        pcntr1::Podr04,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            pcntr1::Podr04,
            pcntr1::Podr04,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr05(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        pcntr1::Podr05,
        pcntr1::Podr05,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            pcntr1::Podr05,
            pcntr1::Podr05,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr06(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        pcntr1::Podr06,
        pcntr1::Podr06,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            pcntr1::Podr06,
            pcntr1::Podr06,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr07(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        pcntr1::Podr07,
        pcntr1::Podr07,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            pcntr1::Podr07,
            pcntr1::Podr07,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr08(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        pcntr1::Podr08,
        pcntr1::Podr08,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            pcntr1::Podr08,
            pcntr1::Podr08,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr09(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        pcntr1::Podr09,
        pcntr1::Podr09,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            pcntr1::Podr09,
            pcntr1::Podr09,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr10(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        pcntr1::Podr10,
        pcntr1::Podr10,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            pcntr1::Podr10,
            pcntr1::Podr10,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr11(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        pcntr1::Podr11,
        pcntr1::Podr11,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            pcntr1::Podr11,
            pcntr1::Podr11,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr12(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        pcntr1::Podr12,
        pcntr1::Podr12,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            pcntr1::Podr12,
            pcntr1::Podr12,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr13(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        pcntr1::Podr13,
        pcntr1::Podr13,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            pcntr1::Podr13,
            pcntr1::Podr13,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr14(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        pcntr1::Podr14,
        pcntr1::Podr14,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            pcntr1::Podr14,
            pcntr1::Podr14,
            Pcntr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr15(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        pcntr1::Podr15,
        pcntr1::Podr15,
        Pcntr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            pcntr1::Podr15,
            pcntr1::Podr15,
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
    pub struct Pdr00_SPEC;
    pub type Pdr00 = crate::EnumBitfieldStruct<u8, Pdr00_SPEC>;
    impl Pdr00 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr01_SPEC;
    pub type Pdr01 = crate::EnumBitfieldStruct<u8, Pdr01_SPEC>;
    impl Pdr01 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr02_SPEC;
    pub type Pdr02 = crate::EnumBitfieldStruct<u8, Pdr02_SPEC>;
    impl Pdr02 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr03_SPEC;
    pub type Pdr03 = crate::EnumBitfieldStruct<u8, Pdr03_SPEC>;
    impl Pdr03 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr04_SPEC;
    pub type Pdr04 = crate::EnumBitfieldStruct<u8, Pdr04_SPEC>;
    impl Pdr04 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr05_SPEC;
    pub type Pdr05 = crate::EnumBitfieldStruct<u8, Pdr05_SPEC>;
    impl Pdr05 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr06_SPEC;
    pub type Pdr06 = crate::EnumBitfieldStruct<u8, Pdr06_SPEC>;
    impl Pdr06 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr07_SPEC;
    pub type Pdr07 = crate::EnumBitfieldStruct<u8, Pdr07_SPEC>;
    impl Pdr07 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr08_SPEC;
    pub type Pdr08 = crate::EnumBitfieldStruct<u8, Pdr08_SPEC>;
    impl Pdr08 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr09_SPEC;
    pub type Pdr09 = crate::EnumBitfieldStruct<u8, Pdr09_SPEC>;
    impl Pdr09 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr10_SPEC;
    pub type Pdr10 = crate::EnumBitfieldStruct<u8, Pdr10_SPEC>;
    impl Pdr10 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr11_SPEC;
    pub type Pdr11 = crate::EnumBitfieldStruct<u8, Pdr11_SPEC>;
    impl Pdr11 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr12_SPEC;
    pub type Pdr12 = crate::EnumBitfieldStruct<u8, Pdr12_SPEC>;
    impl Pdr12 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr13_SPEC;
    pub type Pdr13 = crate::EnumBitfieldStruct<u8, Pdr13_SPEC>;
    impl Pdr13 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr14_SPEC;
    pub type Pdr14 = crate::EnumBitfieldStruct<u8, Pdr14_SPEC>;
    impl Pdr14 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr15_SPEC;
    pub type Pdr15 = crate::EnumBitfieldStruct<u8, Pdr15_SPEC>;
    impl Pdr15 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
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
    pub struct Podr04_SPEC;
    pub type Podr04 = crate::EnumBitfieldStruct<u8, Podr04_SPEC>;
    impl Podr04 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr05_SPEC;
    pub type Podr05 = crate::EnumBitfieldStruct<u8, Podr05_SPEC>;
    impl Podr05 {
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
pub struct Podr_SPEC;
impl crate::sealed::RegSpec for Podr_SPEC {
    type DataType = u16;
}

#[doc = "Port Control Register 1"]
pub type Podr = crate::RegValueT<Podr_SPEC>;

impl Podr {
    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        podr::Podr00,
        podr::Podr00,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            podr::Podr00,
            podr::Podr00,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        podr::Podr01,
        podr::Podr01,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            podr::Podr01,
            podr::Podr01,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        podr::Podr02,
        podr::Podr02,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            podr::Podr02,
            podr::Podr02,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        podr::Podr03,
        podr::Podr03,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            podr::Podr03,
            podr::Podr03,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        podr::Podr04,
        podr::Podr04,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            podr::Podr04,
            podr::Podr04,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        podr::Podr05,
        podr::Podr05,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            podr::Podr05,
            podr::Podr05,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        podr::Podr06,
        podr::Podr06,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            podr::Podr06,
            podr::Podr06,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        podr::Podr07,
        podr::Podr07,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            podr::Podr07,
            podr::Podr07,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        podr::Podr08,
        podr::Podr08,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            podr::Podr08,
            podr::Podr08,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        podr::Podr09,
        podr::Podr09,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            podr::Podr09,
            podr::Podr09,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        podr::Podr10,
        podr::Podr10,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            podr::Podr10,
            podr::Podr10,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        podr::Podr11,
        podr::Podr11,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            podr::Podr11,
            podr::Podr11,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        podr::Podr12,
        podr::Podr12,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            podr::Podr12,
            podr::Podr12,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        podr::Podr13,
        podr::Podr13,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            podr::Podr13,
            podr::Podr13,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        podr::Podr14,
        podr::Podr14,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            podr::Podr14,
            podr::Podr14,
            Podr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Data"]
    #[inline(always)]
    pub fn podr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        podr::Podr15,
        podr::Podr15,
        Podr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            podr::Podr15,
            podr::Podr15,
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
    pub struct Podr04_SPEC;
    pub type Podr04 = crate::EnumBitfieldStruct<u8, Podr04_SPEC>;
    impl Podr04 {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr05_SPEC;
    pub type Podr05 = crate::EnumBitfieldStruct<u8, Podr05_SPEC>;
    impl Podr05 {
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
pub struct Pdr_SPEC;
impl crate::sealed::RegSpec for Pdr_SPEC {
    type DataType = u16;
}

#[doc = "Port Control Register 1"]
pub type Pdr = crate::RegValueT<Pdr_SPEC>;

impl Pdr {
    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdr::Pdr00,
        pdr::Pdr00,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdr::Pdr00,
            pdr::Pdr00,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdr::Pdr01,
        pdr::Pdr01,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdr::Pdr01,
            pdr::Pdr01,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pdr::Pdr02,
        pdr::Pdr02,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pdr::Pdr02,
            pdr::Pdr02,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pdr::Pdr03,
        pdr::Pdr03,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pdr::Pdr03,
            pdr::Pdr03,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pdr::Pdr04,
        pdr::Pdr04,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pdr::Pdr04,
            pdr::Pdr04,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pdr::Pdr05,
        pdr::Pdr05,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pdr::Pdr05,
            pdr::Pdr05,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pdr::Pdr06,
        pdr::Pdr06,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pdr::Pdr06,
            pdr::Pdr06,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pdr::Pdr07,
        pdr::Pdr07,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pdr::Pdr07,
            pdr::Pdr07,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pdr::Pdr08,
        pdr::Pdr08,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pdr::Pdr08,
            pdr::Pdr08,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pdr::Pdr09,
        pdr::Pdr09,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pdr::Pdr09,
            pdr::Pdr09,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pdr::Pdr10,
        pdr::Pdr10,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pdr::Pdr10,
            pdr::Pdr10,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pdr::Pdr11,
        pdr::Pdr11,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pdr::Pdr11,
            pdr::Pdr11,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pdr::Pdr12,
        pdr::Pdr12,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pdr::Pdr12,
            pdr::Pdr12,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pdr::Pdr13,
        pdr::Pdr13,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pdr::Pdr13,
            pdr::Pdr13,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pdr::Pdr14,
        pdr::Pdr14,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pdr::Pdr14,
            pdr::Pdr14,
            Pdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Direction"]
    #[inline(always)]
    pub fn pdr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pdr::Pdr15,
        pdr::Pdr15,
        Pdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pdr::Pdr15,
            pdr::Pdr15,
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
    pub struct Pdr00_SPEC;
    pub type Pdr00 = crate::EnumBitfieldStruct<u8, Pdr00_SPEC>;
    impl Pdr00 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr01_SPEC;
    pub type Pdr01 = crate::EnumBitfieldStruct<u8, Pdr01_SPEC>;
    impl Pdr01 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr02_SPEC;
    pub type Pdr02 = crate::EnumBitfieldStruct<u8, Pdr02_SPEC>;
    impl Pdr02 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr03_SPEC;
    pub type Pdr03 = crate::EnumBitfieldStruct<u8, Pdr03_SPEC>;
    impl Pdr03 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr04_SPEC;
    pub type Pdr04 = crate::EnumBitfieldStruct<u8, Pdr04_SPEC>;
    impl Pdr04 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr05_SPEC;
    pub type Pdr05 = crate::EnumBitfieldStruct<u8, Pdr05_SPEC>;
    impl Pdr05 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr06_SPEC;
    pub type Pdr06 = crate::EnumBitfieldStruct<u8, Pdr06_SPEC>;
    impl Pdr06 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr07_SPEC;
    pub type Pdr07 = crate::EnumBitfieldStruct<u8, Pdr07_SPEC>;
    impl Pdr07 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr08_SPEC;
    pub type Pdr08 = crate::EnumBitfieldStruct<u8, Pdr08_SPEC>;
    impl Pdr08 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr09_SPEC;
    pub type Pdr09 = crate::EnumBitfieldStruct<u8, Pdr09_SPEC>;
    impl Pdr09 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr10_SPEC;
    pub type Pdr10 = crate::EnumBitfieldStruct<u8, Pdr10_SPEC>;
    impl Pdr10 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr11_SPEC;
    pub type Pdr11 = crate::EnumBitfieldStruct<u8, Pdr11_SPEC>;
    impl Pdr11 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr12_SPEC;
    pub type Pdr12 = crate::EnumBitfieldStruct<u8, Pdr12_SPEC>;
    impl Pdr12 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr13_SPEC;
    pub type Pdr13 = crate::EnumBitfieldStruct<u8, Pdr13_SPEC>;
    impl Pdr13 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr14_SPEC;
    pub type Pdr14 = crate::EnumBitfieldStruct<u8, Pdr14_SPEC>;
    impl Pdr14 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr15_SPEC;
    pub type Pdr15 = crate::EnumBitfieldStruct<u8, Pdr15_SPEC>;
    impl Pdr15 {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
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

impl Pcntr2 {
    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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

    #[doc = "Pmn State"]
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
    pub struct Pidr04_SPEC;
    pub type Pidr04 = crate::EnumBitfieldStruct<u8, Pidr04_SPEC>;
    impl Pidr04 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr05_SPEC;
    pub type Pidr05 = crate::EnumBitfieldStruct<u8, Pidr05_SPEC>;
    impl Pidr05 {
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
pub struct Pidr_SPEC;
impl crate::sealed::RegSpec for Pidr_SPEC {
    type DataType = u16;
}

#[doc = "Port Control Register 2"]
pub type Pidr = crate::RegValueT<Pidr_SPEC>;

impl Pidr {
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pidr::Pidr00,
        pidr::Pidr00,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pidr::Pidr00,
            pidr::Pidr00,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pidr::Pidr01,
        pidr::Pidr01,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pidr::Pidr01,
            pidr::Pidr01,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pidr::Pidr02,
        pidr::Pidr02,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pidr::Pidr02,
            pidr::Pidr02,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pidr::Pidr03,
        pidr::Pidr03,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pidr::Pidr03,
            pidr::Pidr03,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pidr::Pidr04,
        pidr::Pidr04,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pidr::Pidr04,
            pidr::Pidr04,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pidr::Pidr05,
        pidr::Pidr05,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pidr::Pidr05,
            pidr::Pidr05,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pidr::Pidr06,
        pidr::Pidr06,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pidr::Pidr06,
            pidr::Pidr06,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pidr::Pidr07,
        pidr::Pidr07,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pidr::Pidr07,
            pidr::Pidr07,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pidr::Pidr08,
        pidr::Pidr08,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pidr::Pidr08,
            pidr::Pidr08,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pidr::Pidr09,
        pidr::Pidr09,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pidr::Pidr09,
            pidr::Pidr09,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pidr::Pidr10,
        pidr::Pidr10,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pidr::Pidr10,
            pidr::Pidr10,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pidr::Pidr11,
        pidr::Pidr11,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pidr::Pidr11,
            pidr::Pidr11,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pidr::Pidr12,
        pidr::Pidr12,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pidr::Pidr12,
            pidr::Pidr12,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pidr::Pidr13,
        pidr::Pidr13,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pidr::Pidr13,
            pidr::Pidr13,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pidr::Pidr14,
        pidr::Pidr14,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pidr::Pidr14,
            pidr::Pidr14,
            Pidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pidr::Pidr15,
        pidr::Pidr15,
        Pidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pidr::Pidr15,
            pidr::Pidr15,
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
    pub struct Pidr04_SPEC;
    pub type Pidr04 = crate::EnumBitfieldStruct<u8, Pidr04_SPEC>;
    impl Pidr04 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr05_SPEC;
    pub type Pidr05 = crate::EnumBitfieldStruct<u8, Pidr05_SPEC>;
    impl Pidr05 {
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
pub struct Pcntr3_SPEC;
impl crate::sealed::RegSpec for Pcntr3_SPEC {
    type DataType = u32;
}

#[doc = "Port Control Register 3"]
pub type Pcntr3 = crate::RegValueT<Pcntr3_SPEC>;

impl Pcntr3 {
    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Set"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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

    #[doc = "Pmn Output Reset"]
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
    pub struct Posr04_SPEC;
    pub type Posr04 = crate::EnumBitfieldStruct<u8, Posr04_SPEC>;
    impl Posr04 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr05_SPEC;
    pub type Posr05 = crate::EnumBitfieldStruct<u8, Posr05_SPEC>;
    impl Posr05 {
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
    pub struct Porr04_SPEC;
    pub type Porr04 = crate::EnumBitfieldStruct<u8, Porr04_SPEC>;
    impl Porr04 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr05_SPEC;
    pub type Porr05 = crate::EnumBitfieldStruct<u8, Porr05_SPEC>;
    impl Porr05 {
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
pub struct Porr_SPEC;
impl crate::sealed::RegSpec for Porr_SPEC {
    type DataType = u16;
}

#[doc = "Port Control Register 3"]
pub type Porr = crate::RegValueT<Porr_SPEC>;

impl Porr {
    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        porr::Porr00,
        porr::Porr00,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            porr::Porr00,
            porr::Porr00,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        porr::Porr01,
        porr::Porr01,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            porr::Porr01,
            porr::Porr01,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        porr::Porr02,
        porr::Porr02,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            porr::Porr02,
            porr::Porr02,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        porr::Porr03,
        porr::Porr03,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            porr::Porr03,
            porr::Porr03,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        porr::Porr04,
        porr::Porr04,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            porr::Porr04,
            porr::Porr04,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        porr::Porr05,
        porr::Porr05,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            porr::Porr05,
            porr::Porr05,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        porr::Porr06,
        porr::Porr06,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            porr::Porr06,
            porr::Porr06,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        porr::Porr07,
        porr::Porr07,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            porr::Porr07,
            porr::Porr07,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        porr::Porr08,
        porr::Porr08,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            porr::Porr08,
            porr::Porr08,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        porr::Porr09,
        porr::Porr09,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            porr::Porr09,
            porr::Porr09,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        porr::Porr10,
        porr::Porr10,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            porr::Porr10,
            porr::Porr10,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        porr::Porr11,
        porr::Porr11,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            porr::Porr11,
            porr::Porr11,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        porr::Porr12,
        porr::Porr12,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            porr::Porr12,
            porr::Porr12,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        porr::Porr13,
        porr::Porr13,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            porr::Porr13,
            porr::Porr13,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        porr::Porr14,
        porr::Porr14,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            porr::Porr14,
            porr::Porr14,
            Porr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Reset"]
    #[inline(always)]
    pub fn porr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        porr::Porr15,
        porr::Porr15,
        Porr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            porr::Porr15,
            porr::Porr15,
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
    pub struct Porr04_SPEC;
    pub type Porr04 = crate::EnumBitfieldStruct<u8, Porr04_SPEC>;
    impl Porr04 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porr05_SPEC;
    pub type Porr05 = crate::EnumBitfieldStruct<u8, Porr05_SPEC>;
    impl Porr05 {
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
pub struct Posr_SPEC;
impl crate::sealed::RegSpec for Posr_SPEC {
    type DataType = u16;
}

#[doc = "Port Control Register 3"]
pub type Posr = crate::RegValueT<Posr_SPEC>;

impl Posr {
    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        posr::Posr00,
        posr::Posr00,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            posr::Posr00,
            posr::Posr00,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        posr::Posr01,
        posr::Posr01,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            posr::Posr01,
            posr::Posr01,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        posr::Posr02,
        posr::Posr02,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            posr::Posr02,
            posr::Posr02,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        posr::Posr03,
        posr::Posr03,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            posr::Posr03,
            posr::Posr03,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        posr::Posr04,
        posr::Posr04,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            posr::Posr04,
            posr::Posr04,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        posr::Posr05,
        posr::Posr05,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            posr::Posr05,
            posr::Posr05,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        posr::Posr06,
        posr::Posr06,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            posr::Posr06,
            posr::Posr06,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        posr::Posr07,
        posr::Posr07,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            posr::Posr07,
            posr::Posr07,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        posr::Posr08,
        posr::Posr08,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            posr::Posr08,
            posr::Posr08,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        posr::Posr09,
        posr::Posr09,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            posr::Posr09,
            posr::Posr09,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        posr::Posr10,
        posr::Posr10,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            posr::Posr10,
            posr::Posr10,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        posr::Posr11,
        posr::Posr11,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            posr::Posr11,
            posr::Posr11,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        posr::Posr12,
        posr::Posr12,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            posr::Posr12,
            posr::Posr12,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        posr::Posr13,
        posr::Posr13,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            posr::Posr13,
            posr::Posr13,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        posr::Posr14,
        posr::Posr14,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            posr::Posr14,
            posr::Posr14,
            Posr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Output Set"]
    #[inline(always)]
    pub fn posr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        posr::Posr15,
        posr::Posr15,
        Posr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            posr::Posr15,
            posr::Posr15,
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
    pub struct Posr04_SPEC;
    pub type Posr04 = crate::EnumBitfieldStruct<u8, Posr04_SPEC>;
    impl Posr04 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posr05_SPEC;
    pub type Posr05 = crate::EnumBitfieldStruct<u8, Posr05_SPEC>;
    impl Posr05 {
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
