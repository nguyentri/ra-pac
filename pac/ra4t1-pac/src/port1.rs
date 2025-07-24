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
#[doc = r"Pmn Pin FunctionPort 1 Control RegistersPmn Pin Function Control Register"]
unsafe impl ::core::marker::Send for super::Port1 {}
unsafe impl ::core::marker::Sync for super::Port1 {}
impl super::Port1 {
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
    pub const fn eidr(&self) -> &'static crate::common::Reg<self::Eidr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eidr_SPEC, crate::common::R>::from_ptr(
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

    #[doc = "Port Control Register 4"]
    #[inline(always)]
    pub const fn pcntr4(
        &self,
    ) -> &'static crate::common::Reg<self::Pcntr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pcntr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Port Control Register 4"]
    #[inline(always)]
    pub const fn eorr(&self) -> &'static crate::common::Reg<self::Eorr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eorr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Port Control Register 4"]
    #[inline(always)]
    pub const fn eosr(&self) -> &'static crate::common::Reg<self::Eosr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eosr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
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

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr00(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        pcntr2::Eidr00,
        pcntr2::Eidr00,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pcntr2::Eidr00,
            pcntr2::Eidr00,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr01(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        pcntr2::Eidr01,
        pcntr2::Eidr01,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            pcntr2::Eidr01,
            pcntr2::Eidr01,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr02(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        pcntr2::Eidr02,
        pcntr2::Eidr02,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            pcntr2::Eidr02,
            pcntr2::Eidr02,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr03(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        pcntr2::Eidr03,
        pcntr2::Eidr03,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            pcntr2::Eidr03,
            pcntr2::Eidr03,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr04(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        pcntr2::Eidr04,
        pcntr2::Eidr04,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            pcntr2::Eidr04,
            pcntr2::Eidr04,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr05(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        pcntr2::Eidr05,
        pcntr2::Eidr05,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            pcntr2::Eidr05,
            pcntr2::Eidr05,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr06(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        pcntr2::Eidr06,
        pcntr2::Eidr06,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            pcntr2::Eidr06,
            pcntr2::Eidr06,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr07(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        pcntr2::Eidr07,
        pcntr2::Eidr07,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            pcntr2::Eidr07,
            pcntr2::Eidr07,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr08(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        pcntr2::Eidr08,
        pcntr2::Eidr08,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            pcntr2::Eidr08,
            pcntr2::Eidr08,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr09(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        pcntr2::Eidr09,
        pcntr2::Eidr09,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            pcntr2::Eidr09,
            pcntr2::Eidr09,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr10(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        pcntr2::Eidr10,
        pcntr2::Eidr10,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            pcntr2::Eidr10,
            pcntr2::Eidr10,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr11(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        pcntr2::Eidr11,
        pcntr2::Eidr11,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            pcntr2::Eidr11,
            pcntr2::Eidr11,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr12(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        pcntr2::Eidr12,
        pcntr2::Eidr12,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            pcntr2::Eidr12,
            pcntr2::Eidr12,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr13(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        pcntr2::Eidr13,
        pcntr2::Eidr13,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            pcntr2::Eidr13,
            pcntr2::Eidr13,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr14(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        pcntr2::Eidr14,
        pcntr2::Eidr14,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            pcntr2::Eidr14,
            pcntr2::Eidr14,
            Pcntr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr15(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        pcntr2::Eidr15,
        pcntr2::Eidr15,
        Pcntr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            pcntr2::Eidr15,
            pcntr2::Eidr15,
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr00_SPEC;
    pub type Eidr00 = crate::EnumBitfieldStruct<u8, Eidr00_SPEC>;
    impl Eidr00 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr01_SPEC;
    pub type Eidr01 = crate::EnumBitfieldStruct<u8, Eidr01_SPEC>;
    impl Eidr01 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr02_SPEC;
    pub type Eidr02 = crate::EnumBitfieldStruct<u8, Eidr02_SPEC>;
    impl Eidr02 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr03_SPEC;
    pub type Eidr03 = crate::EnumBitfieldStruct<u8, Eidr03_SPEC>;
    impl Eidr03 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr04_SPEC;
    pub type Eidr04 = crate::EnumBitfieldStruct<u8, Eidr04_SPEC>;
    impl Eidr04 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr05_SPEC;
    pub type Eidr05 = crate::EnumBitfieldStruct<u8, Eidr05_SPEC>;
    impl Eidr05 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr06_SPEC;
    pub type Eidr06 = crate::EnumBitfieldStruct<u8, Eidr06_SPEC>;
    impl Eidr06 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr07_SPEC;
    pub type Eidr07 = crate::EnumBitfieldStruct<u8, Eidr07_SPEC>;
    impl Eidr07 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr08_SPEC;
    pub type Eidr08 = crate::EnumBitfieldStruct<u8, Eidr08_SPEC>;
    impl Eidr08 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr09_SPEC;
    pub type Eidr09 = crate::EnumBitfieldStruct<u8, Eidr09_SPEC>;
    impl Eidr09 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr10_SPEC;
    pub type Eidr10 = crate::EnumBitfieldStruct<u8, Eidr10_SPEC>;
    impl Eidr10 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr11_SPEC;
    pub type Eidr11 = crate::EnumBitfieldStruct<u8, Eidr11_SPEC>;
    impl Eidr11 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr12_SPEC;
    pub type Eidr12 = crate::EnumBitfieldStruct<u8, Eidr12_SPEC>;
    impl Eidr12 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr13_SPEC;
    pub type Eidr13 = crate::EnumBitfieldStruct<u8, Eidr13_SPEC>;
    impl Eidr13 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr14_SPEC;
    pub type Eidr14 = crate::EnumBitfieldStruct<u8, Eidr14_SPEC>;
    impl Eidr14 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr15_SPEC;
    pub type Eidr15 = crate::EnumBitfieldStruct<u8, Eidr15_SPEC>;
    impl Eidr15 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eidr_SPEC;
impl crate::sealed::RegSpec for Eidr_SPEC {
    type DataType = u16;
}

#[doc = "Port Control Register 2"]
pub type Eidr = crate::RegValueT<Eidr_SPEC>;

impl Eidr {
    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eidr::Eidr00,
        eidr::Eidr00,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eidr::Eidr00,
            eidr::Eidr00,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eidr::Eidr01,
        eidr::Eidr01,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eidr::Eidr01,
            eidr::Eidr01,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eidr::Eidr02,
        eidr::Eidr02,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eidr::Eidr02,
            eidr::Eidr02,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eidr::Eidr03,
        eidr::Eidr03,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eidr::Eidr03,
            eidr::Eidr03,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eidr::Eidr04,
        eidr::Eidr04,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eidr::Eidr04,
            eidr::Eidr04,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eidr::Eidr05,
        eidr::Eidr05,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eidr::Eidr05,
            eidr::Eidr05,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eidr::Eidr06,
        eidr::Eidr06,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eidr::Eidr06,
            eidr::Eidr06,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eidr::Eidr07,
        eidr::Eidr07,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eidr::Eidr07,
            eidr::Eidr07,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eidr::Eidr08,
        eidr::Eidr08,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eidr::Eidr08,
            eidr::Eidr08,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        eidr::Eidr09,
        eidr::Eidr09,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            eidr::Eidr09,
            eidr::Eidr09,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        eidr::Eidr10,
        eidr::Eidr10,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            eidr::Eidr10,
            eidr::Eidr10,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        eidr::Eidr11,
        eidr::Eidr11,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            eidr::Eidr11,
            eidr::Eidr11,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        eidr::Eidr12,
        eidr::Eidr12,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            eidr::Eidr12,
            eidr::Eidr12,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        eidr::Eidr13,
        eidr::Eidr13,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            eidr::Eidr13,
            eidr::Eidr13,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        eidr::Eidr14,
        eidr::Eidr14,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            eidr::Eidr14,
            eidr::Eidr14,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Event Input Data"]
    #[inline(always)]
    pub fn eidr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        eidr::Eidr15,
        eidr::Eidr15,
        Eidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            eidr::Eidr15,
            eidr::Eidr15,
            Eidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eidr {
    #[inline(always)]
    fn default() -> Eidr {
        <crate::RegValueT<Eidr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eidr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr00_SPEC;
    pub type Eidr00 = crate::EnumBitfieldStruct<u8, Eidr00_SPEC>;
    impl Eidr00 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr01_SPEC;
    pub type Eidr01 = crate::EnumBitfieldStruct<u8, Eidr01_SPEC>;
    impl Eidr01 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr02_SPEC;
    pub type Eidr02 = crate::EnumBitfieldStruct<u8, Eidr02_SPEC>;
    impl Eidr02 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr03_SPEC;
    pub type Eidr03 = crate::EnumBitfieldStruct<u8, Eidr03_SPEC>;
    impl Eidr03 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr04_SPEC;
    pub type Eidr04 = crate::EnumBitfieldStruct<u8, Eidr04_SPEC>;
    impl Eidr04 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr05_SPEC;
    pub type Eidr05 = crate::EnumBitfieldStruct<u8, Eidr05_SPEC>;
    impl Eidr05 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr06_SPEC;
    pub type Eidr06 = crate::EnumBitfieldStruct<u8, Eidr06_SPEC>;
    impl Eidr06 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr07_SPEC;
    pub type Eidr07 = crate::EnumBitfieldStruct<u8, Eidr07_SPEC>;
    impl Eidr07 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr08_SPEC;
    pub type Eidr08 = crate::EnumBitfieldStruct<u8, Eidr08_SPEC>;
    impl Eidr08 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr09_SPEC;
    pub type Eidr09 = crate::EnumBitfieldStruct<u8, Eidr09_SPEC>;
    impl Eidr09 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr10_SPEC;
    pub type Eidr10 = crate::EnumBitfieldStruct<u8, Eidr10_SPEC>;
    impl Eidr10 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr11_SPEC;
    pub type Eidr11 = crate::EnumBitfieldStruct<u8, Eidr11_SPEC>;
    impl Eidr11 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr12_SPEC;
    pub type Eidr12 = crate::EnumBitfieldStruct<u8, Eidr12_SPEC>;
    impl Eidr12 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr13_SPEC;
    pub type Eidr13 = crate::EnumBitfieldStruct<u8, Eidr13_SPEC>;
    impl Eidr13 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr14_SPEC;
    pub type Eidr14 = crate::EnumBitfieldStruct<u8, Eidr14_SPEC>;
    impl Eidr14 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eidr15_SPEC;
    pub type Eidr15 = crate::EnumBitfieldStruct<u8, Eidr15_SPEC>;
    impl Eidr15 {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
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
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcntr4_SPEC;
impl crate::sealed::RegSpec for Pcntr4_SPEC {
    type DataType = u32;
}

#[doc = "Port Control Register 4"]
pub type Pcntr4 = crate::RegValueT<Pcntr4_SPEC>;

impl Pcntr4 {
    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pcntr4::Eosr00,
        pcntr4::Eosr00,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pcntr4::Eosr00,
            pcntr4::Eosr00,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pcntr4::Eosr01,
        pcntr4::Eosr01,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pcntr4::Eosr01,
            pcntr4::Eosr01,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pcntr4::Eosr02,
        pcntr4::Eosr02,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pcntr4::Eosr02,
            pcntr4::Eosr02,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pcntr4::Eosr03,
        pcntr4::Eosr03,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pcntr4::Eosr03,
            pcntr4::Eosr03,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pcntr4::Eosr04,
        pcntr4::Eosr04,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pcntr4::Eosr04,
            pcntr4::Eosr04,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pcntr4::Eosr05,
        pcntr4::Eosr05,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pcntr4::Eosr05,
            pcntr4::Eosr05,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pcntr4::Eosr06,
        pcntr4::Eosr06,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pcntr4::Eosr06,
            pcntr4::Eosr06,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pcntr4::Eosr07,
        pcntr4::Eosr07,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pcntr4::Eosr07,
            pcntr4::Eosr07,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pcntr4::Eosr08,
        pcntr4::Eosr08,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pcntr4::Eosr08,
            pcntr4::Eosr08,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pcntr4::Eosr09,
        pcntr4::Eosr09,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pcntr4::Eosr09,
            pcntr4::Eosr09,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pcntr4::Eosr10,
        pcntr4::Eosr10,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pcntr4::Eosr10,
            pcntr4::Eosr10,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pcntr4::Eosr11,
        pcntr4::Eosr11,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pcntr4::Eosr11,
            pcntr4::Eosr11,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pcntr4::Eosr12,
        pcntr4::Eosr12,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pcntr4::Eosr12,
            pcntr4::Eosr12,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pcntr4::Eosr13,
        pcntr4::Eosr13,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pcntr4::Eosr13,
            pcntr4::Eosr13,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pcntr4::Eosr14,
        pcntr4::Eosr14,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pcntr4::Eosr14,
            pcntr4::Eosr14,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pcntr4::Eosr15,
        pcntr4::Eosr15,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pcntr4::Eosr15,
            pcntr4::Eosr15,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr00(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        pcntr4::Eorr00,
        pcntr4::Eorr00,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pcntr4::Eorr00,
            pcntr4::Eorr00,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr01(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        pcntr4::Eorr01,
        pcntr4::Eorr01,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            pcntr4::Eorr01,
            pcntr4::Eorr01,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr02(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        pcntr4::Eorr02,
        pcntr4::Eorr02,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            pcntr4::Eorr02,
            pcntr4::Eorr02,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr03(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        pcntr4::Eorr03,
        pcntr4::Eorr03,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            pcntr4::Eorr03,
            pcntr4::Eorr03,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr04(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        pcntr4::Eorr04,
        pcntr4::Eorr04,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            pcntr4::Eorr04,
            pcntr4::Eorr04,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr05(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        pcntr4::Eorr05,
        pcntr4::Eorr05,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            pcntr4::Eorr05,
            pcntr4::Eorr05,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr06(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        pcntr4::Eorr06,
        pcntr4::Eorr06,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            pcntr4::Eorr06,
            pcntr4::Eorr06,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr07(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        pcntr4::Eorr07,
        pcntr4::Eorr07,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            pcntr4::Eorr07,
            pcntr4::Eorr07,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr08(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        pcntr4::Eorr08,
        pcntr4::Eorr08,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            pcntr4::Eorr08,
            pcntr4::Eorr08,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr09(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        pcntr4::Eorr09,
        pcntr4::Eorr09,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            pcntr4::Eorr09,
            pcntr4::Eorr09,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr10(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        pcntr4::Eorr10,
        pcntr4::Eorr10,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            pcntr4::Eorr10,
            pcntr4::Eorr10,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr11(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        pcntr4::Eorr11,
        pcntr4::Eorr11,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            pcntr4::Eorr11,
            pcntr4::Eorr11,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr12(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        pcntr4::Eorr12,
        pcntr4::Eorr12,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            pcntr4::Eorr12,
            pcntr4::Eorr12,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr13(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        pcntr4::Eorr13,
        pcntr4::Eorr13,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            pcntr4::Eorr13,
            pcntr4::Eorr13,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr14(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        pcntr4::Eorr14,
        pcntr4::Eorr14,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            pcntr4::Eorr14,
            pcntr4::Eorr14,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr15(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        pcntr4::Eorr15,
        pcntr4::Eorr15,
        Pcntr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            pcntr4::Eorr15,
            pcntr4::Eorr15,
            Pcntr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pcntr4 {
    #[inline(always)]
    fn default() -> Pcntr4 {
        <crate::RegValueT<Pcntr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pcntr4 {

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
    pub struct Eosr04_SPEC;
    pub type Eosr04 = crate::EnumBitfieldStruct<u8, Eosr04_SPEC>;
    impl Eosr04 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr05_SPEC;
    pub type Eosr05 = crate::EnumBitfieldStruct<u8, Eosr05_SPEC>;
    impl Eosr05 {
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
    pub struct Eosr11_SPEC;
    pub type Eosr11 = crate::EnumBitfieldStruct<u8, Eosr11_SPEC>;
    impl Eosr11 {
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr14_SPEC;
    pub type Eosr14 = crate::EnumBitfieldStruct<u8, Eosr14_SPEC>;
    impl Eosr14 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr15_SPEC;
    pub type Eosr15 = crate::EnumBitfieldStruct<u8, Eosr15_SPEC>;
    impl Eosr15 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
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
    pub struct Eorr04_SPEC;
    pub type Eorr04 = crate::EnumBitfieldStruct<u8, Eorr04_SPEC>;
    impl Eorr04 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr05_SPEC;
    pub type Eorr05 = crate::EnumBitfieldStruct<u8, Eorr05_SPEC>;
    impl Eorr05 {
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
    pub struct Eorr11_SPEC;
    pub type Eorr11 = crate::EnumBitfieldStruct<u8, Eorr11_SPEC>;
    impl Eorr11 {
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr14_SPEC;
    pub type Eorr14 = crate::EnumBitfieldStruct<u8, Eorr14_SPEC>;
    impl Eorr14 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr15_SPEC;
    pub type Eorr15 = crate::EnumBitfieldStruct<u8, Eorr15_SPEC>;
    impl Eorr15 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eorr_SPEC;
impl crate::sealed::RegSpec for Eorr_SPEC {
    type DataType = u16;
}

#[doc = "Port Control Register 4"]
pub type Eorr = crate::RegValueT<Eorr_SPEC>;

impl Eorr {
    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eorr::Eorr00,
        eorr::Eorr00,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eorr::Eorr00,
            eorr::Eorr00,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eorr::Eorr01,
        eorr::Eorr01,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eorr::Eorr01,
            eorr::Eorr01,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eorr::Eorr02,
        eorr::Eorr02,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eorr::Eorr02,
            eorr::Eorr02,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eorr::Eorr03,
        eorr::Eorr03,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eorr::Eorr03,
            eorr::Eorr03,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eorr::Eorr04,
        eorr::Eorr04,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eorr::Eorr04,
            eorr::Eorr04,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eorr::Eorr05,
        eorr::Eorr05,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eorr::Eorr05,
            eorr::Eorr05,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eorr::Eorr06,
        eorr::Eorr06,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eorr::Eorr06,
            eorr::Eorr06,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eorr::Eorr07,
        eorr::Eorr07,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eorr::Eorr07,
            eorr::Eorr07,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eorr::Eorr08,
        eorr::Eorr08,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eorr::Eorr08,
            eorr::Eorr08,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        eorr::Eorr09,
        eorr::Eorr09,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            eorr::Eorr09,
            eorr::Eorr09,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        eorr::Eorr10,
        eorr::Eorr10,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            eorr::Eorr10,
            eorr::Eorr10,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        eorr::Eorr11,
        eorr::Eorr11,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            eorr::Eorr11,
            eorr::Eorr11,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        eorr::Eorr12,
        eorr::Eorr12,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            eorr::Eorr12,
            eorr::Eorr12,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        eorr::Eorr13,
        eorr::Eorr13,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            eorr::Eorr13,
            eorr::Eorr13,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        eorr::Eorr14,
        eorr::Eorr14,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            eorr::Eorr14,
            eorr::Eorr14,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        eorr::Eorr15,
        eorr::Eorr15,
        Eorr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            eorr::Eorr15,
            eorr::Eorr15,
            Eorr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eorr {
    #[inline(always)]
    fn default() -> Eorr {
        <crate::RegValueT<Eorr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eorr {

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
    pub struct Eorr04_SPEC;
    pub type Eorr04 = crate::EnumBitfieldStruct<u8, Eorr04_SPEC>;
    impl Eorr04 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr05_SPEC;
    pub type Eorr05 = crate::EnumBitfieldStruct<u8, Eorr05_SPEC>;
    impl Eorr05 {
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
    pub struct Eorr11_SPEC;
    pub type Eorr11 = crate::EnumBitfieldStruct<u8, Eorr11_SPEC>;
    impl Eorr11 {
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr14_SPEC;
    pub type Eorr14 = crate::EnumBitfieldStruct<u8, Eorr14_SPEC>;
    impl Eorr14 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eorr15_SPEC;
    pub type Eorr15 = crate::EnumBitfieldStruct<u8, Eorr15_SPEC>;
    impl Eorr15 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eosr_SPEC;
impl crate::sealed::RegSpec for Eosr_SPEC {
    type DataType = u16;
}

#[doc = "Port Control Register 4"]
pub type Eosr = crate::RegValueT<Eosr_SPEC>;

impl Eosr {
    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eosr::Eosr00,
        eosr::Eosr00,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eosr::Eosr00,
            eosr::Eosr00,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eosr::Eosr01,
        eosr::Eosr01,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eosr::Eosr01,
            eosr::Eosr01,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eosr::Eosr02,
        eosr::Eosr02,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eosr::Eosr02,
            eosr::Eosr02,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eosr::Eosr03,
        eosr::Eosr03,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eosr::Eosr03,
            eosr::Eosr03,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eosr::Eosr04,
        eosr::Eosr04,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eosr::Eosr04,
            eosr::Eosr04,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eosr::Eosr05,
        eosr::Eosr05,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eosr::Eosr05,
            eosr::Eosr05,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eosr::Eosr06,
        eosr::Eosr06,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eosr::Eosr06,
            eosr::Eosr06,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eosr::Eosr07,
        eosr::Eosr07,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eosr::Eosr07,
            eosr::Eosr07,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eosr::Eosr08,
        eosr::Eosr08,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eosr::Eosr08,
            eosr::Eosr08,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        eosr::Eosr09,
        eosr::Eosr09,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            eosr::Eosr09,
            eosr::Eosr09,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        eosr::Eosr10,
        eosr::Eosr10,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            eosr::Eosr10,
            eosr::Eosr10,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        eosr::Eosr11,
        eosr::Eosr11,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            eosr::Eosr11,
            eosr::Eosr11,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        eosr::Eosr12,
        eosr::Eosr12,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            eosr::Eosr12,
            eosr::Eosr12,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        eosr::Eosr13,
        eosr::Eosr13,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            eosr::Eosr13,
            eosr::Eosr13,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        eosr::Eosr14,
        eosr::Eosr14,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            eosr::Eosr14,
            eosr::Eosr14,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn Event Output Set"]
    #[inline(always)]
    pub fn eosr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        eosr::Eosr15,
        eosr::Eosr15,
        Eosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            eosr::Eosr15,
            eosr::Eosr15,
            Eosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eosr {
    #[inline(always)]
    fn default() -> Eosr {
        <crate::RegValueT<Eosr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eosr {

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
    pub struct Eosr04_SPEC;
    pub type Eosr04 = crate::EnumBitfieldStruct<u8, Eosr04_SPEC>;
    impl Eosr04 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr05_SPEC;
    pub type Eosr05 = crate::EnumBitfieldStruct<u8, Eosr05_SPEC>;
    impl Eosr05 {
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
    pub struct Eosr11_SPEC;
    pub type Eosr11 = crate::EnumBitfieldStruct<u8, Eosr11_SPEC>;
    impl Eosr11 {
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr14_SPEC;
    pub type Eosr14 = crate::EnumBitfieldStruct<u8, Eosr14_SPEC>;
    impl Eosr14 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eosr15_SPEC;
    pub type Eosr15 = crate::EnumBitfieldStruct<u8, Eosr15_SPEC>;
    impl Eosr15 {
        #[doc = "No effect on output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
