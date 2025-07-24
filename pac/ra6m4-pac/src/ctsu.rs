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
// Generated from SVD 1.50.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:51:51 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Capacitive Touch Sensing Unit"]
unsafe impl ::core::marker::Send for super::Ctsu {}
unsafe impl ::core::marker::Sync for super::Ctsu {}
impl super::Ctsu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "CTSU Control Register 0"]
    #[inline(always)]
    pub const fn ctsucr0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "CTSU Control Register 1"]
    #[inline(always)]
    pub const fn ctsucr1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "CTSU Synchronous Noise Reduction Setting Register"]
    #[inline(always)]
    pub const fn ctsusdprs(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusdprs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusdprs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "CTSU Sensor Stabilization Wait Control Register"]
    #[inline(always)]
    pub const fn ctsusst(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "CTSU Measurement Channel Register 0"]
    #[inline(always)]
    pub const fn ctsumch0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumch0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumch0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "CTSU Measurement Channel Register 1"]
    #[inline(always)]
    pub const fn ctsumch1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumch1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ctsumch1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[doc = "CTSU Channel Enable Control Register 0"]
    #[inline(always)]
    pub const fn ctsuchac0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "CTSU Channel Enable Control Register 1"]
    #[inline(always)]
    pub const fn ctsuchac1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[doc = "CTSU Channel Enable Control Register 2"]
    #[inline(always)]
    pub const fn ctsuchac2(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchac2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchac2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "CTSU Channel Transmit/Receive Control Register 0"]
    #[inline(always)]
    pub const fn ctsuchtrc0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[doc = "CTSU Channel Transmit/Receive Control Register 1"]
    #[inline(always)]
    pub const fn ctsuchtrc1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "CTSU Channel Transmit/Receive Control Register 2"]
    #[inline(always)]
    pub const fn ctsuchtrc2(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[doc = "CTSU High-Pass Noise Reduction Control Register"]
    #[inline(always)]
    pub const fn ctsudclkc(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsudclkc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsudclkc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "CTSU Status Register"]
    #[inline(always)]
    pub const fn ctsust(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsust_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsust_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17usize),
            )
        }
    }

    #[doc = "CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register"]
    #[inline(always)]
    pub const fn ctsussc(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsussc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsussc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "CTSU Sensor Offset Register 0"]
    #[inline(always)]
    pub const fn ctsuso0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuso0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuso0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "CTSU Sensor Offset Register 1"]
    #[inline(always)]
    pub const fn ctsuso1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuso1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuso1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "CTSU Sensor Counter"]
    #[inline(always)]
    pub const fn ctsusc(&self) -> &'static crate::common::Reg<self::Ctsusc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ctsusc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "CTSU Reference Counter"]
    #[inline(always)]
    pub const fn ctsurc(&self) -> &'static crate::common::Reg<self::Ctsurc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ctsurc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[doc = "CTSU Error Status Register"]
    #[inline(always)]
    pub const fn ctsuerrs(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuerrs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuerrs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "CTSU Reference Current Calibration Register"]
    #[inline(always)]
    pub const fn ctsutrmr(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsutrmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsutrmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucr0_SPEC;
impl crate::sealed::RegSpec for Ctsucr0_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Control Register 0"]
pub type Ctsucr0 = crate::RegValueT<Ctsucr0_SPEC>;

impl NoBitfieldReg<Ctsucr0_SPEC> for Ctsucr0 {}
impl ::core::default::Default for Ctsucr0 {
    #[inline(always)]
    fn default() -> Ctsucr0 {
        <crate::RegValueT<Ctsucr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucr1_SPEC;
impl crate::sealed::RegSpec for Ctsucr1_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Control Register 1"]
pub type Ctsucr1 = crate::RegValueT<Ctsucr1_SPEC>;

impl NoBitfieldReg<Ctsucr1_SPEC> for Ctsucr1 {}
impl ::core::default::Default for Ctsucr1 {
    #[inline(always)]
    fn default() -> Ctsucr1 {
        <crate::RegValueT<Ctsucr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusdprs_SPEC;
impl crate::sealed::RegSpec for Ctsusdprs_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Synchronous Noise Reduction Setting Register"]
pub type Ctsusdprs = crate::RegValueT<Ctsusdprs_SPEC>;

impl NoBitfieldReg<Ctsusdprs_SPEC> for Ctsusdprs {}
impl ::core::default::Default for Ctsusdprs {
    #[inline(always)]
    fn default() -> Ctsusdprs {
        <crate::RegValueT<Ctsusdprs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusst_SPEC;
impl crate::sealed::RegSpec for Ctsusst_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Sensor Stabilization Wait Control Register"]
pub type Ctsusst = crate::RegValueT<Ctsusst_SPEC>;

impl NoBitfieldReg<Ctsusst_SPEC> for Ctsusst {}
impl ::core::default::Default for Ctsusst {
    #[inline(always)]
    fn default() -> Ctsusst {
        <crate::RegValueT<Ctsusst_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumch0_SPEC;
impl crate::sealed::RegSpec for Ctsumch0_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Measurement Channel Register 0"]
pub type Ctsumch0 = crate::RegValueT<Ctsumch0_SPEC>;

impl NoBitfieldReg<Ctsumch0_SPEC> for Ctsumch0 {}
impl ::core::default::Default for Ctsumch0 {
    #[inline(always)]
    fn default() -> Ctsumch0 {
        <crate::RegValueT<Ctsumch0_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumch1_SPEC;
impl crate::sealed::RegSpec for Ctsumch1_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Measurement Channel Register 1"]
pub type Ctsumch1 = crate::RegValueT<Ctsumch1_SPEC>;

impl NoBitfieldReg<Ctsumch1_SPEC> for Ctsumch1 {}
impl ::core::default::Default for Ctsumch1 {
    #[inline(always)]
    fn default() -> Ctsumch1 {
        <crate::RegValueT<Ctsumch1_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac0_SPEC;
impl crate::sealed::RegSpec for Ctsuchac0_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Channel Enable Control Register 0"]
pub type Ctsuchac0 = crate::RegValueT<Ctsuchac0_SPEC>;

impl NoBitfieldReg<Ctsuchac0_SPEC> for Ctsuchac0 {}
impl ::core::default::Default for Ctsuchac0 {
    #[inline(always)]
    fn default() -> Ctsuchac0 {
        <crate::RegValueT<Ctsuchac0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac1_SPEC;
impl crate::sealed::RegSpec for Ctsuchac1_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Channel Enable Control Register 1"]
pub type Ctsuchac1 = crate::RegValueT<Ctsuchac1_SPEC>;

impl NoBitfieldReg<Ctsuchac1_SPEC> for Ctsuchac1 {}
impl ::core::default::Default for Ctsuchac1 {
    #[inline(always)]
    fn default() -> Ctsuchac1 {
        <crate::RegValueT<Ctsuchac1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac2_SPEC;
impl crate::sealed::RegSpec for Ctsuchac2_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Channel Enable Control Register 2"]
pub type Ctsuchac2 = crate::RegValueT<Ctsuchac2_SPEC>;

impl NoBitfieldReg<Ctsuchac2_SPEC> for Ctsuchac2 {}
impl ::core::default::Default for Ctsuchac2 {
    #[inline(always)]
    fn default() -> Ctsuchac2 {
        <crate::RegValueT<Ctsuchac2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc0_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrc0_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Channel Transmit/Receive Control Register 0"]
pub type Ctsuchtrc0 = crate::RegValueT<Ctsuchtrc0_SPEC>;

impl NoBitfieldReg<Ctsuchtrc0_SPEC> for Ctsuchtrc0 {}
impl ::core::default::Default for Ctsuchtrc0 {
    #[inline(always)]
    fn default() -> Ctsuchtrc0 {
        <crate::RegValueT<Ctsuchtrc0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc1_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrc1_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Channel Transmit/Receive Control Register 1"]
pub type Ctsuchtrc1 = crate::RegValueT<Ctsuchtrc1_SPEC>;

impl NoBitfieldReg<Ctsuchtrc1_SPEC> for Ctsuchtrc1 {}
impl ::core::default::Default for Ctsuchtrc1 {
    #[inline(always)]
    fn default() -> Ctsuchtrc1 {
        <crate::RegValueT<Ctsuchtrc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc2_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrc2_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Channel Transmit/Receive Control Register 2"]
pub type Ctsuchtrc2 = crate::RegValueT<Ctsuchtrc2_SPEC>;

impl NoBitfieldReg<Ctsuchtrc2_SPEC> for Ctsuchtrc2 {}
impl ::core::default::Default for Ctsuchtrc2 {
    #[inline(always)]
    fn default() -> Ctsuchtrc2 {
        <crate::RegValueT<Ctsuchtrc2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsudclkc_SPEC;
impl crate::sealed::RegSpec for Ctsudclkc_SPEC {
    type DataType = u8;
}

#[doc = "CTSU High-Pass Noise Reduction Control Register"]
pub type Ctsudclkc = crate::RegValueT<Ctsudclkc_SPEC>;

impl NoBitfieldReg<Ctsudclkc_SPEC> for Ctsudclkc {}
impl ::core::default::Default for Ctsudclkc {
    #[inline(always)]
    fn default() -> Ctsudclkc {
        <crate::RegValueT<Ctsudclkc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsust_SPEC;
impl crate::sealed::RegSpec for Ctsust_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Status Register"]
pub type Ctsust = crate::RegValueT<Ctsust_SPEC>;

impl NoBitfieldReg<Ctsust_SPEC> for Ctsust {}
impl ::core::default::Default for Ctsust {
    #[inline(always)]
    fn default() -> Ctsust {
        <crate::RegValueT<Ctsust_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsussc_SPEC;
impl crate::sealed::RegSpec for Ctsussc_SPEC {
    type DataType = u16;
}

#[doc = "CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register"]
pub type Ctsussc = crate::RegValueT<Ctsussc_SPEC>;

impl Ctsussc {
    #[doc = "CTSU Spectrum Diffusion Frequency Division Setting"]
    #[inline(always)]
    pub fn ctsussdiv(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Ctsussc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Ctsussc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsussc {
    #[inline(always)]
    fn default() -> Ctsussc {
        <crate::RegValueT<Ctsussc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuso0_SPEC;
impl crate::sealed::RegSpec for Ctsuso0_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Sensor Offset Register 0"]
pub type Ctsuso0 = crate::RegValueT<Ctsuso0_SPEC>;

impl NoBitfieldReg<Ctsuso0_SPEC> for Ctsuso0 {}
impl ::core::default::Default for Ctsuso0 {
    #[inline(always)]
    fn default() -> Ctsuso0 {
        <crate::RegValueT<Ctsuso0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuso1_SPEC;
impl crate::sealed::RegSpec for Ctsuso1_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Sensor Offset Register 1"]
pub type Ctsuso1 = crate::RegValueT<Ctsuso1_SPEC>;

impl NoBitfieldReg<Ctsuso1_SPEC> for Ctsuso1 {}
impl ::core::default::Default for Ctsuso1 {
    #[inline(always)]
    fn default() -> Ctsuso1 {
        <crate::RegValueT<Ctsuso1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusc_SPEC;
impl crate::sealed::RegSpec for Ctsusc_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Sensor Counter"]
pub type Ctsusc = crate::RegValueT<Ctsusc_SPEC>;

impl NoBitfieldReg<Ctsusc_SPEC> for Ctsusc {}
impl ::core::default::Default for Ctsusc {
    #[inline(always)]
    fn default() -> Ctsusc {
        <crate::RegValueT<Ctsusc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsurc_SPEC;
impl crate::sealed::RegSpec for Ctsurc_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Reference Counter"]
pub type Ctsurc = crate::RegValueT<Ctsurc_SPEC>;

impl Ctsurc {
    #[doc = "CTSU Reference Counter"]
    #[inline(always)]
    pub fn ctsurc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsurc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsurc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsurc {
    #[inline(always)]
    fn default() -> Ctsurc {
        <crate::RegValueT<Ctsurc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuerrs_SPEC;
impl crate::sealed::RegSpec for Ctsuerrs_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Error Status Register"]
pub type Ctsuerrs = crate::RegValueT<Ctsuerrs_SPEC>;

impl Ctsuerrs {
    #[doc = "Calibration Mode"]
    #[inline(always)]
    pub fn ctsuspmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        ctsuerrs::Ctsuspmd,
        ctsuerrs::Ctsuspmd,
        Ctsuerrs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            ctsuerrs::Ctsuspmd,
            ctsuerrs::Ctsuspmd,
            Ctsuerrs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TS Pin Fixed Output"]
    #[inline(always)]
    pub fn ctsutsod(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuerrs::Ctsutsod,
        ctsuerrs::Ctsutsod,
        Ctsuerrs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuerrs::Ctsutsod,
            ctsuerrs::Ctsutsod,
            Ctsuerrs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Calibration Setting 1"]
    #[inline(always)]
    pub fn ctsudrv(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsuerrs::Ctsudrv,
        ctsuerrs::Ctsudrv,
        Ctsuerrs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsuerrs::Ctsudrv,
            ctsuerrs::Ctsudrv,
            Ctsuerrs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Calibration Setting 3"]
    #[inline(always)]
    pub fn ctsuclksel1(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ctsuerrs::Ctsuclksel1,
        ctsuerrs::Ctsuclksel1,
        Ctsuerrs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ctsuerrs::Ctsuclksel1,
            ctsuerrs::Ctsuclksel1,
            Ctsuerrs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Calibration Setting 2"]
    #[inline(always)]
    pub fn ctsutsoc(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ctsuerrs::Ctsutsoc,
        ctsuerrs::Ctsutsoc,
        Ctsuerrs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ctsuerrs::Ctsutsoc,
            ctsuerrs::Ctsutsoc,
            Ctsuerrs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TSCAP Voltage Error Monitor"]
    #[inline(always)]
    pub fn ctsuicomp(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ctsuerrs::Ctsuicomp,
        ctsuerrs::Ctsuicomp,
        Ctsuerrs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ctsuerrs::Ctsuicomp,
            ctsuerrs::Ctsuicomp,
            Ctsuerrs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuerrs {
    #[inline(always)]
    fn default() -> Ctsuerrs {
        <crate::RegValueT<Ctsuerrs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuerrs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsuspmd_SPEC;
    pub type Ctsuspmd = crate::EnumBitfieldStruct<u8, Ctsuspmd_SPEC>;
    impl Ctsuspmd {
        #[doc = "Capacitance measurement mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "Calibration mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Seting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsutsod_SPEC;
    pub type Ctsutsod = crate::EnumBitfieldStruct<u8, Ctsutsod_SPEC>;
    impl Ctsutsod {
        #[doc = "Capacitance measurement mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "TS pins are forced to be high or low"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsudrv_SPEC;
    pub type Ctsudrv = crate::EnumBitfieldStruct<u8, Ctsudrv_SPEC>;
    impl Ctsudrv {
        #[doc = "Capacitance measurement mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Calibration setting 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsuclksel1_SPEC;
    pub type Ctsuclksel1 = crate::EnumBitfieldStruct<u8, Ctsuclksel1_SPEC>;
    impl Ctsuclksel1 {
        #[doc = "Capacitance measurement mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Calibration setting 3"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsutsoc_SPEC;
    pub type Ctsutsoc = crate::EnumBitfieldStruct<u8, Ctsutsoc_SPEC>;
    impl Ctsutsoc {
        #[doc = "Capacitance measurement mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Calibration setting 2"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsuicomp_SPEC;
    pub type Ctsuicomp = crate::EnumBitfieldStruct<u8, Ctsuicomp_SPEC>;
    impl Ctsuicomp {
        #[doc = "Normal TSCAP voltage"]
        pub const _0: Self = Self::new(0);

        #[doc = "Abnormal TSCAP voltage"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsutrmr_SPEC;
impl crate::sealed::RegSpec for Ctsutrmr_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Reference Current Calibration Register"]
pub type Ctsutrmr = crate::RegValueT<Ctsutrmr_SPEC>;

impl NoBitfieldReg<Ctsutrmr_SPEC> for Ctsutrmr {}
impl ::core::default::Default for Ctsutrmr {
    #[inline(always)]
    fn default() -> Ctsutrmr {
        <crate::RegValueT<Ctsutrmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
