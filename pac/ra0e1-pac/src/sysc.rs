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
#[doc = r"System Control"]
unsafe impl ::core::marker::Send for super::Sysc {}
unsafe impl ::core::marker::Sync for super::Sysc {}
impl super::Sysc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Clock Operation Mode Control Register"]
    #[inline(always)]
    pub const fn cmc(&self) -> &'static crate::common::Reg<self::Cmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2048usize),
            )
        }
    }

    #[doc = "Sub-clock Oscillator Margin Check Register"]
    #[inline(always)]
    pub const fn somrg(&self) -> &'static crate::common::Reg<self::Somrg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Somrg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2051usize),
            )
        }
    }

    #[doc = "Middle-speed On-chip Oscillator Trimming Register"]
    #[inline(always)]
    pub const fn miotrm(
        &self,
    ) -> &'static crate::common::Reg<self::Miotrm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Miotrm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2052usize),
            )
        }
    }

    #[doc = "Low-speed On-chip Oscillator Trimming Register"]
    #[inline(always)]
    pub const fn liotrm(
        &self,
    ) -> &'static crate::common::Reg<self::Liotrm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Liotrm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2053usize),
            )
        }
    }

    #[doc = "High-speed On-chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn hococr(
        &self,
    ) -> &'static crate::common::Reg<self::Hococr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hococr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2056usize),
            )
        }
    }

    #[doc = "Middle-speed On-chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn mococr(
        &self,
    ) -> &'static crate::common::Reg<self::Mococr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mococr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2057usize),
            )
        }
    }

    #[doc = "Low-speed On-chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn lococr(
        &self,
    ) -> &'static crate::common::Reg<self::Lococr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lococr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2058usize),
            )
        }
    }

    #[doc = "Main Clock Oscillator Control Register"]
    #[inline(always)]
    pub const fn mosccr(
        &self,
    ) -> &'static crate::common::Reg<self::Mosccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mosccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2059usize),
            )
        }
    }

    #[doc = "Sub-clock Oscillator Control Register"]
    #[inline(always)]
    pub const fn sosccr(
        &self,
    ) -> &'static crate::common::Reg<self::Sosccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sosccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2060usize),
            )
        }
    }

    #[doc = "Oscillation Stabilization Time Counter Status Register"]
    #[inline(always)]
    pub const fn ostc(&self) -> &'static crate::common::Reg<self::Ostc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ostc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2064usize),
            )
        }
    }

    #[doc = "Oscillation Stabilization Time Select Register"]
    #[inline(always)]
    pub const fn osts(&self) -> &'static crate::common::Reg<self::Osts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Osts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2065usize),
            )
        }
    }

    #[doc = "Oscillation Stabilization Flag Register"]
    #[inline(always)]
    pub const fn oscsf(&self) -> &'static crate::common::Reg<self::Oscsf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Oscsf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2066usize),
            )
        }
    }

    #[doc = "High-speed On-chip Oscillator Frequency Select Register"]
    #[inline(always)]
    pub const fn hocodiv(
        &self,
    ) -> &'static crate::common::Reg<self::Hocodiv_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hocodiv_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2072usize),
            )
        }
    }

    #[doc = "Middle-speed On-chip Oscillator Frequency Select Register"]
    #[inline(always)]
    pub const fn mocodiv(
        &self,
    ) -> &'static crate::common::Reg<self::Mocodiv_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mocodiv_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2073usize),
            )
        }
    }

    #[doc = "MOSC Clock Division Register"]
    #[inline(always)]
    pub const fn moscdiv(
        &self,
    ) -> &'static crate::common::Reg<self::Moscdiv_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Moscdiv_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2074usize),
            )
        }
    }

    #[doc = "FOCO Clock Source Control Register"]
    #[inline(always)]
    pub const fn focoscr(
        &self,
    ) -> &'static crate::common::Reg<self::Focoscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Focoscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2080usize),
            )
        }
    }

    #[doc = "FMAIN Clock Source Control Register"]
    #[inline(always)]
    pub const fn fmainscr(
        &self,
    ) -> &'static crate::common::Reg<self::Fmainscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fmainscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2081usize),
            )
        }
    }

    #[doc = "FSUB Clock Source Control Register"]
    #[inline(always)]
    pub const fn fsubscr(
        &self,
    ) -> &'static crate::common::Reg<self::Fsubscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fsubscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2082usize),
            )
        }
    }

    #[doc = "ICLK Clock Source Control Register"]
    #[inline(always)]
    pub const fn iclkscr(
        &self,
    ) -> &'static crate::common::Reg<self::Iclkscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iclkscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2083usize),
            )
        }
    }

    #[doc = "Subsystem Clock Supply Mode Control Register"]
    #[inline(always)]
    pub const fn osmc(&self) -> &'static crate::common::Reg<self::Osmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Osmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2084usize),
            )
        }
    }

    #[doc = "Reset Status Flag Register"]
    #[inline(always)]
    pub const fn resf(&self) -> &'static crate::common::Reg<self::Resf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Resf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2096usize),
            )
        }
    }

    #[doc = "Power-On Reset Status Register"]
    #[inline(always)]
    pub const fn porsr(&self) -> &'static crate::common::Reg<self::Porsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Porsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2097usize),
            )
        }
    }

    #[doc = "Voltage Monitor 1 Circuit Control Register"]
    #[inline(always)]
    pub const fn lvd1cr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd1Cr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd1Cr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2112usize),
            )
        }
    }

    #[doc = "Voltage Monitor 1 Circuit Mask Register"]
    #[inline(always)]
    pub const fn lvd1mkr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd1Mkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd1Mkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2113usize),
            )
        }
    }

    #[doc = "Voltage Monitor 1 Circuit Status Register"]
    #[inline(always)]
    pub const fn lvd1sr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd1Sr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd1Sr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2115usize),
            )
        }
    }

    #[doc = "Standby Control Register"]
    #[inline(always)]
    pub const fn sbycr(&self) -> &'static crate::common::Reg<self::Sbycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sbycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2144usize),
            )
        }
    }

    #[doc = "Power Save Memory Control Register"]
    #[inline(always)]
    pub const fn psmcr(&self) -> &'static crate::common::Reg<self::Psmcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psmcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2146usize),
            )
        }
    }

    #[doc = "System Control OCD Control Register"]
    #[inline(always)]
    pub const fn syocdcr(
        &self,
    ) -> &'static crate::common::Reg<self::Syocdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syocdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2147usize),
            )
        }
    }

    #[doc = "Protect Register"]
    #[inline(always)]
    pub const fn prcr(&self) -> &'static crate::common::Reg<self::Prcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Prcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2302usize),
            )
        }
    }

    #[doc = "Module Stop Control Register A"]
    #[inline(always)]
    pub const fn mstpcra(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3074usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmc_SPEC;
impl crate::sealed::RegSpec for Cmc_SPEC {
    type DataType = u8;
}

#[doc = "Clock Operation Mode Control Register"]
pub type Cmc = crate::RegValueT<Cmc_SPEC>;

impl Cmc {
    #[doc = "Main Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    pub fn modrv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cmc::Modrv,
        cmc::Modrv,
        Cmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cmc::Modrv,
            cmc::Modrv,
            Cmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sub-Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    pub fn sodrv(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        cmc::Sodrv,
        cmc::Sodrv,
        Cmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            cmc::Sodrv,
            cmc::Sodrv,
            Cmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selecting Clock Oscillator"]
    #[inline(always)]
    pub fn xtsel(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cmc::Xtsel,
        cmc::Xtsel,
        Cmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cmc::Xtsel,
            cmc::Xtsel,
            Cmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sub Clock Oscillator Switching"]
    #[inline(always)]
    pub fn sosel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cmc::Sosel,
        cmc::Sosel,
        Cmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cmc::Sosel,
            cmc::Sosel,
            Cmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Main Clock Oscillator Switching"]
    #[inline(always)]
    pub fn mosel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        cmc::Mosel,
        cmc::Mosel,
        Cmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            cmc::Mosel,
            cmc::Mosel,
            Cmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cmc {
    #[inline(always)]
    fn default() -> Cmc {
        <crate::RegValueT<Cmc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cmc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Modrv_SPEC;
    pub type Modrv = crate::EnumBitfieldStruct<u8, Modrv_SPEC>;
    impl Modrv {
        #[doc = "1 MHz to 10 MHz"]
        pub const _0: Self = Self::new(0);

        #[doc = "10 MHz to 20 MHz"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sodrv_SPEC;
    pub type Sodrv = crate::EnumBitfieldStruct<u8, Sodrv_SPEC>;
    impl Sodrv {
        #[doc = "Low Power Mode 1"]
        pub const _00: Self = Self::new(0);

        #[doc = "Normal Mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "Low Power Mode 2"]
        pub const _10: Self = Self::new(2);

        #[doc = "Low Power Mode 3"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Xtsel_SPEC;
    pub type Xtsel = crate::EnumBitfieldStruct<u8, Xtsel_SPEC>;
    impl Xtsel {
        #[doc = "Select MOSEL Contents"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select SOSEL Contents"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sosel_SPEC;
    pub type Sosel = crate::EnumBitfieldStruct<u8, Sosel_SPEC>;
    impl Sosel {
        #[doc = "Port mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Resonator"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mosel_SPEC;
    pub type Mosel = crate::EnumBitfieldStruct<u8, Mosel_SPEC>;
    impl Mosel {
        #[doc = "Resonator"]
        pub const _01: Self = Self::new(1);

        #[doc = "External clock input mode"]
        pub const _11: Self = Self::new(3);

        #[doc = "Port mode"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Somrg_SPEC;
impl crate::sealed::RegSpec for Somrg_SPEC {
    type DataType = u8;
}

#[doc = "Sub-clock Oscillator Margin Check Register"]
pub type Somrg = crate::RegValueT<Somrg_SPEC>;

impl Somrg {
    #[doc = "Sub Clock Oscillator Margin Check Switching"]
    #[inline(always)]
    pub fn soscmrg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        somrg::Soscmrg,
        somrg::Soscmrg,
        Somrg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            somrg::Soscmrg,
            somrg::Soscmrg,
            Somrg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Somrg {
    #[inline(always)]
    fn default() -> Somrg {
        <crate::RegValueT<Somrg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod somrg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Soscmrg_SPEC;
    pub type Soscmrg = crate::EnumBitfieldStruct<u8, Soscmrg_SPEC>;
    impl Soscmrg {
        #[doc = "Normal Current"]
        pub const _00: Self = Self::new(0);

        #[doc = "Lower Margin check"]
        pub const _01: Self = Self::new(1);

        #[doc = "Upper Margin check"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Miotrm_SPEC;
impl crate::sealed::RegSpec for Miotrm_SPEC {
    type DataType = u8;
}

#[doc = "Middle-speed On-chip Oscillator Trimming Register"]
pub type Miotrm = crate::RegValueT<Miotrm_SPEC>;

impl Miotrm {
    #[doc = "MOCO User Trimming"]
    #[inline(always)]
    pub fn miotrm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Miotrm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Miotrm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Miotrm {
    #[inline(always)]
    fn default() -> Miotrm {
        <crate::RegValueT<Miotrm_SPEC> as RegisterValue<_>>::new(144)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Liotrm_SPEC;
impl crate::sealed::RegSpec for Liotrm_SPEC {
    type DataType = u8;
}

#[doc = "Low-speed On-chip Oscillator Trimming Register"]
pub type Liotrm = crate::RegValueT<Liotrm_SPEC>;

impl Liotrm {
    #[doc = "LOCO User Trimming"]
    #[inline(always)]
    pub fn liotrm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Liotrm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Liotrm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Liotrm {
    #[inline(always)]
    fn default() -> Liotrm {
        <crate::RegValueT<Liotrm_SPEC> as RegisterValue<_>>::new(128)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hococr_SPEC;
impl crate::sealed::RegSpec for Hococr_SPEC {
    type DataType = u8;
}

#[doc = "High-speed On-chip Oscillator Control Register"]
pub type Hococr = crate::RegValueT<Hococr_SPEC>;

impl Hococr {
    #[doc = "HOCO Stop"]
    #[inline(always)]
    pub fn hcstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        hococr::Hcstp,
        hococr::Hcstp,
        Hococr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            hococr::Hcstp,
            hococr::Hcstp,
            Hococr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Hococr {
    #[inline(always)]
    fn default() -> Hococr {
        <crate::RegValueT<Hococr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hococr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hcstp_SPEC;
    pub type Hcstp = crate::EnumBitfieldStruct<u8, Hcstp_SPEC>;
    impl Hcstp {
        #[doc = "Operate the HOCO clock"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the HOCO clock"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mococr_SPEC;
impl crate::sealed::RegSpec for Mococr_SPEC {
    type DataType = u8;
}

#[doc = "Middle-speed On-chip Oscillator Control Register"]
pub type Mococr = crate::RegValueT<Mococr_SPEC>;

impl Mococr {
    #[doc = "MOCO Stop"]
    #[inline(always)]
    pub fn mcstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mococr::Mcstp,
        mococr::Mcstp,
        Mococr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mococr::Mcstp,
            mococr::Mcstp,
            Mococr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mococr {
    #[inline(always)]
    fn default() -> Mococr {
        <crate::RegValueT<Mococr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod mococr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcstp_SPEC;
    pub type Mcstp = crate::EnumBitfieldStruct<u8, Mcstp_SPEC>;
    impl Mcstp {
        #[doc = "MOCO clock is operating"]
        pub const _0: Self = Self::new(0);

        #[doc = "MOCO clock is stopped"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lococr_SPEC;
impl crate::sealed::RegSpec for Lococr_SPEC {
    type DataType = u8;
}

#[doc = "Low-speed On-chip Oscillator Control Register"]
pub type Lococr = crate::RegValueT<Lococr_SPEC>;

impl Lococr {
    #[doc = "LOCO Stop"]
    #[inline(always)]
    pub fn lcstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lococr::Lcstp,
        lococr::Lcstp,
        Lococr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lococr::Lcstp,
            lococr::Lcstp,
            Lococr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lococr {
    #[inline(always)]
    fn default() -> Lococr {
        <crate::RegValueT<Lococr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod lococr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcstp_SPEC;
    pub type Lcstp = crate::EnumBitfieldStruct<u8, Lcstp_SPEC>;
    impl Lcstp {
        #[doc = "Operate the LOCO clock"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the LOCO clock"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mosccr_SPEC;
impl crate::sealed::RegSpec for Mosccr_SPEC {
    type DataType = u8;
}

#[doc = "Main Clock Oscillator Control Register"]
pub type Mosccr = crate::RegValueT<Mosccr_SPEC>;

impl Mosccr {
    #[doc = "Main Clock Oscillator Stop"]
    #[inline(always)]
    pub fn mostp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mosccr::Mostp,
        mosccr::Mostp,
        Mosccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mosccr::Mostp,
            mosccr::Mostp,
            Mosccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mosccr {
    #[inline(always)]
    fn default() -> Mosccr {
        <crate::RegValueT<Mosccr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod mosccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mostp_SPEC;
    pub type Mostp = crate::EnumBitfieldStruct<u8, Mostp_SPEC>;
    impl Mostp {
        #[doc = "Operate the main clock oscillator"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the main clock oscillator"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccr_SPEC;
impl crate::sealed::RegSpec for Sosccr_SPEC {
    type DataType = u8;
}

#[doc = "Sub-clock Oscillator Control Register"]
pub type Sosccr = crate::RegValueT<Sosccr_SPEC>;

impl Sosccr {
    #[doc = "Sub Clock Oscillator Stop"]
    #[inline(always)]
    pub fn sostp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sosccr::Sostp,
        sosccr::Sostp,
        Sosccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sosccr::Sostp,
            sosccr::Sostp,
            Sosccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sosccr {
    #[inline(always)]
    fn default() -> Sosccr {
        <crate::RegValueT<Sosccr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod sosccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sostp_SPEC;
    pub type Sostp = crate::EnumBitfieldStruct<u8, Sostp_SPEC>;
    impl Sostp {
        #[doc = "Operate the sub-clock oscillator"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the sub-clock oscillator"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostc_SPEC;
impl crate::sealed::RegSpec for Ostc_SPEC {
    type DataType = u8;
}

#[doc = "Oscillation Stabilization Time Counter Status Register"]
pub type Ostc = crate::RegValueT<Ostc_SPEC>;

impl Ostc {
    #[doc = "Selection of the Oscillation Stabilization Time"]
    #[inline(always)]
    pub fn most(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        ostc::Most,
        ostc::Most,
        Ostc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            ostc::Most,
            ostc::Most,
            Ostc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ostc {
    #[inline(always)]
    fn default() -> Ostc {
        <crate::RegValueT<Ostc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ostc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Most_SPEC;
    pub type Most = crate::EnumBitfieldStruct<u8, Most_SPEC>;
    impl Most {
        #[doc = "Less than 28/fMOSC"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "28/fMOSC min"]
        pub const _0_X_80: Self = Self::new(128);

        #[doc = "29/fMOSC min"]
        pub const _0_X_C_0: Self = Self::new(192);

        #[doc = "210/fMOSC min"]
        pub const _0_X_E_0: Self = Self::new(224);

        #[doc = "211/fMOSC min"]
        pub const _0_X_F_0: Self = Self::new(240);

        #[doc = "213/fMOSC min"]
        pub const _0_X_F_8: Self = Self::new(248);

        #[doc = "215/fMOSC min"]
        pub const _0_X_FC: Self = Self::new(252);

        #[doc = "217/fMOSC min"]
        pub const _0_X_FE: Self = Self::new(254);

        #[doc = "218/fMOSC min"]
        pub const _0_X_FF: Self = Self::new(255);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osts_SPEC;
impl crate::sealed::RegSpec for Osts_SPEC {
    type DataType = u8;
}

#[doc = "Oscillation Stabilization Time Select Register"]
pub type Osts = crate::RegValueT<Osts_SPEC>;

impl Osts {
    #[doc = "Selection of the Oscillation Stabilization Time"]
    #[inline(always)]
    pub fn ostsb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        osts::Ostsb,
        osts::Ostsb,
        Osts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            osts::Ostsb,
            osts::Ostsb,
            Osts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Osts {
    #[inline(always)]
    fn default() -> Osts {
        <crate::RegValueT<Osts_SPEC> as RegisterValue<_>>::new(7)
    }
}
pub mod osts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostsb_SPEC;
    pub type Ostsb = crate::EnumBitfieldStruct<u8, Ostsb_SPEC>;
    impl Ostsb {
        #[doc = "28/fMOSC"]
        pub const _000: Self = Self::new(0);

        #[doc = "29/fMOSC"]
        pub const _001: Self = Self::new(1);

        #[doc = "210/fMOSC"]
        pub const _010: Self = Self::new(2);

        #[doc = "211/fMOSC"]
        pub const _011: Self = Self::new(3);

        #[doc = "213/fMOSC"]
        pub const _100: Self = Self::new(4);

        #[doc = "215/fMOSC"]
        pub const _101: Self = Self::new(5);

        #[doc = "217/fMOSC"]
        pub const _110: Self = Self::new(6);

        #[doc = "218/fMOSC"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscsf_SPEC;
impl crate::sealed::RegSpec for Oscsf_SPEC {
    type DataType = u8;
}

#[doc = "Oscillation Stabilization Flag Register"]
pub type Oscsf = crate::RegValueT<Oscsf_SPEC>;

impl Oscsf {
    #[doc = "HOCO Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn hocosf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        oscsf::Hocosf,
        oscsf::Hocosf,
        Oscsf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            oscsf::Hocosf,
            oscsf::Hocosf,
            Oscsf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Oscsf {
    #[inline(always)]
    fn default() -> Oscsf {
        <crate::RegValueT<Oscsf_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod oscsf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hocosf_SPEC;
    pub type Hocosf = crate::EnumBitfieldStruct<u8, Hocosf_SPEC>;
    impl Hocosf {
        #[doc = "The HOCO clock is being started at high speed and waiting for the precision of its oscillation to become stable is in progress."]
        pub const _0: Self = Self::new(0);

        #[doc = "The HOCO clock is operating with high precision."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hocodiv_SPEC;
impl crate::sealed::RegSpec for Hocodiv_SPEC {
    type DataType = u8;
}

#[doc = "High-speed On-chip Oscillator Frequency Select Register"]
pub type Hocodiv = crate::RegValueT<Hocodiv_SPEC>;

impl Hocodiv {
    #[doc = "High-speed On-chip Oscillator Clock Division Ratio"]
    #[inline(always)]
    pub fn div(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        hocodiv::Div,
        hocodiv::Div,
        Hocodiv_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            hocodiv::Div,
            hocodiv::Div,
            Hocodiv_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Hocodiv {
    #[inline(always)]
    fn default() -> Hocodiv {
        <crate::RegValueT<Hocodiv_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod hocodiv {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Div_SPEC;
    pub type Div = crate::EnumBitfieldStruct<u8, Div_SPEC>;
    impl Div {
        #[doc = "× 1/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "× 1/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "× 1/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "× 1/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "× 1/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "× 1/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mocodiv_SPEC;
impl crate::sealed::RegSpec for Mocodiv_SPEC {
    type DataType = u8;
}

#[doc = "Middle-speed On-chip Oscillator Frequency Select Register"]
pub type Mocodiv = crate::RegValueT<Mocodiv_SPEC>;

impl Mocodiv {
    #[doc = "Selection of the Middle-speed On-chip Oscillator Clock Frequency"]
    #[inline(always)]
    pub fn div(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        mocodiv::Div,
        mocodiv::Div,
        Mocodiv_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            mocodiv::Div,
            mocodiv::Div,
            Mocodiv_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mocodiv {
    #[inline(always)]
    fn default() -> Mocodiv {
        <crate::RegValueT<Mocodiv_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mocodiv {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Div_SPEC;
    pub type Div = crate::EnumBitfieldStruct<u8, Div_SPEC>;
    impl Div {
        #[doc = "× 1/1"]
        pub const _00: Self = Self::new(0);

        #[doc = "× 1/2"]
        pub const _01: Self = Self::new(1);

        #[doc = "× 1/4"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Moscdiv_SPEC;
impl crate::sealed::RegSpec for Moscdiv_SPEC {
    type DataType = u8;
}

#[doc = "MOSC Clock Division Register"]
pub type Moscdiv = crate::RegValueT<Moscdiv_SPEC>;

impl Moscdiv {
    #[doc = "Selection Division Ratio for the MOSC Clock"]
    #[inline(always)]
    pub fn div(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        moscdiv::Div,
        moscdiv::Div,
        Moscdiv_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            moscdiv::Div,
            moscdiv::Div,
            Moscdiv_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Moscdiv {
    #[inline(always)]
    fn default() -> Moscdiv {
        <crate::RegValueT<Moscdiv_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod moscdiv {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Div_SPEC;
    pub type Div = crate::EnumBitfieldStruct<u8, Div_SPEC>;
    impl Div {
        #[doc = "× 1/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "× 1/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "× 1/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "× 1/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "× 1/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Focoscr_SPEC;
impl crate::sealed::RegSpec for Focoscr_SPEC {
    type DataType = u8;
}

#[doc = "FOCO Clock Source Control Register"]
pub type Focoscr = crate::RegValueT<Focoscr_SPEC>;

impl Focoscr {
    #[doc = "FOCO Clock Source Select"]
    #[inline(always)]
    pub fn cksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        focoscr::Cksel,
        focoscr::Cksel,
        Focoscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            focoscr::Cksel,
            focoscr::Cksel,
            Focoscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FOCO Clock Source Status"]
    #[inline(always)]
    pub fn ckst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        focoscr::Ckst,
        focoscr::Ckst,
        Focoscr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            focoscr::Ckst,
            focoscr::Ckst,
            Focoscr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Focoscr {
    #[inline(always)]
    fn default() -> Focoscr {
        <crate::RegValueT<Focoscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod focoscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksel_SPEC;
    pub type Cksel = crate::EnumBitfieldStruct<u8, Cksel_SPEC>;
    impl Cksel {
        #[doc = "HOCO"]
        pub const _0: Self = Self::new(0);

        #[doc = "MOCO"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckst_SPEC;
    pub type Ckst = crate::EnumBitfieldStruct<u8, Ckst_SPEC>;
    impl Ckst {
        #[doc = "HOCO"]
        pub const _0: Self = Self::new(0);

        #[doc = "MOCO"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmainscr_SPEC;
impl crate::sealed::RegSpec for Fmainscr_SPEC {
    type DataType = u8;
}

#[doc = "FMAIN Clock Source Control Register"]
pub type Fmainscr = crate::RegValueT<Fmainscr_SPEC>;

impl Fmainscr {
    #[doc = "FMAIN Clock Source Select"]
    #[inline(always)]
    pub fn cksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fmainscr::Cksel,
        fmainscr::Cksel,
        Fmainscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fmainscr::Cksel,
            fmainscr::Cksel,
            Fmainscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FMAIN Clock Source Status"]
    #[inline(always)]
    pub fn ckst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fmainscr::Ckst,
        fmainscr::Ckst,
        Fmainscr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fmainscr::Ckst,
            fmainscr::Ckst,
            Fmainscr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fmainscr {
    #[inline(always)]
    fn default() -> Fmainscr {
        <crate::RegValueT<Fmainscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fmainscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksel_SPEC;
    pub type Cksel = crate::EnumBitfieldStruct<u8, Cksel_SPEC>;
    impl Cksel {
        #[doc = "FOCO"]
        pub const _0: Self = Self::new(0);

        #[doc = "MOSC"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckst_SPEC;
    pub type Ckst = crate::EnumBitfieldStruct<u8, Ckst_SPEC>;
    impl Ckst {
        #[doc = "FOCO"]
        pub const _0: Self = Self::new(0);

        #[doc = "MOSC"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsubscr_SPEC;
impl crate::sealed::RegSpec for Fsubscr_SPEC {
    type DataType = u8;
}

#[doc = "FSUB Clock Source Control Register"]
pub type Fsubscr = crate::RegValueT<Fsubscr_SPEC>;

impl Fsubscr {
    #[doc = "FSUB Clock Source Select"]
    #[inline(always)]
    pub fn cksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fsubscr::Cksel,
        fsubscr::Cksel,
        Fsubscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fsubscr::Cksel,
            fsubscr::Cksel,
            Fsubscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fsubscr {
    #[inline(always)]
    fn default() -> Fsubscr {
        <crate::RegValueT<Fsubscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fsubscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksel_SPEC;
    pub type Cksel = crate::EnumBitfieldStruct<u8, Cksel_SPEC>;
    impl Cksel {
        #[doc = "SOSC"]
        pub const _0: Self = Self::new(0);

        #[doc = "LOCO"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iclkscr_SPEC;
impl crate::sealed::RegSpec for Iclkscr_SPEC {
    type DataType = u8;
}

#[doc = "ICLK Clock Source Control Register"]
pub type Iclkscr = crate::RegValueT<Iclkscr_SPEC>;

impl Iclkscr {
    #[doc = "ICLK Clock Source Select"]
    #[inline(always)]
    pub fn cksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iclkscr::Cksel,
        iclkscr::Cksel,
        Iclkscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iclkscr::Cksel,
            iclkscr::Cksel,
            Iclkscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ICLK Clock Source Status"]
    #[inline(always)]
    pub fn ckst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iclkscr::Ckst,
        iclkscr::Ckst,
        Iclkscr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iclkscr::Ckst,
            iclkscr::Ckst,
            Iclkscr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iclkscr {
    #[inline(always)]
    fn default() -> Iclkscr {
        <crate::RegValueT<Iclkscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iclkscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksel_SPEC;
    pub type Cksel = crate::EnumBitfieldStruct<u8, Cksel_SPEC>;
    impl Cksel {
        #[doc = "FMAIN"]
        pub const _0: Self = Self::new(0);

        #[doc = "FSUB"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckst_SPEC;
    pub type Ckst = crate::EnumBitfieldStruct<u8, Ckst_SPEC>;
    impl Ckst {
        #[doc = "FMAIN"]
        pub const _0: Self = Self::new(0);

        #[doc = "FSUB"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osmc_SPEC;
impl crate::sealed::RegSpec for Osmc_SPEC {
    type DataType = u8;
}

#[doc = "Subsystem Clock Supply Mode Control Register"]
pub type Osmc = crate::RegValueT<Osmc_SPEC>;

impl Osmc {
    #[doc = "Selection of the Operating clock source for the Realtime Clock, 32-bit Interval Timer, Serial Interface UARTA"]
    #[inline(always)]
    pub fn wutmmck0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        osmc::Wutmmck0,
        osmc::Wutmmck0,
        Osmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            osmc::Wutmmck0,
            osmc::Wutmmck0,
            Osmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Osmc {
    #[inline(always)]
    fn default() -> Osmc {
        <crate::RegValueT<Osmc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod osmc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wutmmck0_SPEC;
    pub type Wutmmck0 = crate::EnumBitfieldStruct<u8, Wutmmck0_SPEC>;
    impl Wutmmck0 {
        #[doc = "SOSC"]
        pub const _0: Self = Self::new(0);

        #[doc = "LOCO"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resf_SPEC;
impl crate::sealed::RegSpec for Resf_SPEC {
    type DataType = u8;
}

#[doc = "Reset Status Flag Register"]
pub type Resf = crate::RegValueT<Resf_SPEC>;

impl Resf {
    #[doc = "Internal Reset Request by Voltage Detector (LVD0 or LVD1)"]
    #[inline(always)]
    pub fn lvirf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        resf::Lvirf,
        resf::Lvirf,
        Resf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            resf::Lvirf,
            resf::Lvirf,
            Resf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Internal Reset Request by RAM Parity Error"]
    #[inline(always)]
    pub fn rperf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        resf::Rperf,
        resf::Rperf,
        Resf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            resf::Rperf,
            resf::Rperf,
            Resf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Internal Reset Request by Independent Watchdog Timer (IWDT)"]
    #[inline(always)]
    pub fn iwdtrf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        resf::Iwdtrf,
        resf::Iwdtrf,
        Resf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            resf::Iwdtrf,
            resf::Iwdtrf,
            Resf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Internal Reset Request by Software Reset"]
    #[inline(always)]
    pub fn swrf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        resf::Swrf,
        resf::Swrf,
        Resf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            resf::Swrf,
            resf::Swrf,
            Resf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Resf {
    #[inline(always)]
    fn default() -> Resf {
        <crate::RegValueT<Resf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod resf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvirf_SPEC;
    pub type Lvirf = crate::EnumBitfieldStruct<u8, Lvirf_SPEC>;
    impl Lvirf {
        #[doc = "Internal reset request is not generated, or the RESF register is cleared."]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal reset request is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rperf_SPEC;
    pub type Rperf = crate::EnumBitfieldStruct<u8, Rperf_SPEC>;
    impl Rperf {
        #[doc = "Internal reset request is not generated, or the RESF register is cleared."]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal reset request is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtrf_SPEC;
    pub type Iwdtrf = crate::EnumBitfieldStruct<u8, Iwdtrf_SPEC>;
    impl Iwdtrf {
        #[doc = "Internal reset request is not generated, or the RESF register is cleared."]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal reset request is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrf_SPEC;
    pub type Swrf = crate::EnumBitfieldStruct<u8, Swrf_SPEC>;
    impl Swrf {
        #[doc = "Internal reset request is not generated, or the RESF register is cleared."]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal reset request is generated."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Porsr_SPEC;
impl crate::sealed::RegSpec for Porsr_SPEC {
    type DataType = u8;
}

#[doc = "Power-On Reset Status Register"]
pub type Porsr = crate::RegValueT<Porsr_SPEC>;

impl Porsr {
    #[doc = "Checking Occurrence of Power-on Reset"]
    #[inline(always)]
    pub fn porf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        porsr::Porf,
        porsr::Porf,
        Porsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            porsr::Porf,
            porsr::Porf,
            Porsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Porsr {
    #[inline(always)]
    fn default() -> Porsr {
        <crate::RegValueT<Porsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod porsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porf_SPEC;
    pub type Porf = crate::EnumBitfieldStruct<u8, Porf_SPEC>;
    impl Porf {
        #[doc = "A value 1 has not been written, or a power-on reset has occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "No power-on reset has occurred."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd1Cr_SPEC;
impl crate::sealed::RegSpec for Lvd1Cr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 1 Circuit Control Register"]
pub type Lvd1Cr = crate::RegValueT<Lvd1Cr_SPEC>;

impl Lvd1Cr {
    #[doc = "Voltage Detection 1 Level Select"]
    #[inline(always)]
    pub fn lvd1v(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        lvd1cr::Lvd1V,
        lvd1cr::Lvd1V,
        Lvd1Cr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            lvd1cr::Lvd1V,
            lvd1cr::Lvd1V,
            Lvd1Cr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Interrupt Type Select"]
    #[inline(always)]
    pub fn irqsel(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        lvd1cr::Irqsel,
        lvd1cr::Irqsel,
        Lvd1Cr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            lvd1cr::Irqsel,
            lvd1cr::Irqsel,
            Lvd1Cr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operation mode of LVD1"]
    #[inline(always)]
    pub fn lvd1sel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        lvd1cr::Lvd1Sel,
        lvd1cr::Lvd1Sel,
        Lvd1Cr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            lvd1cr::Lvd1Sel,
            lvd1cr::Lvd1Sel,
            Lvd1Cr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enabling Operation of LVD1"]
    #[inline(always)]
    pub fn lvd1en(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        lvd1cr::Lvd1En,
        lvd1cr::Lvd1En,
        Lvd1Cr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            lvd1cr::Lvd1En,
            lvd1cr::Lvd1En,
            Lvd1Cr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd1Cr {
    #[inline(always)]
    fn default() -> Lvd1Cr {
        <crate::RegValueT<Lvd1Cr_SPEC> as RegisterValue<_>>::new(25)
    }
}
pub mod lvd1cr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1V_SPEC;
    pub type Lvd1V = crate::EnumBitfieldStruct<u8, Lvd1V_SPEC>;
    impl Lvd1V {
        #[doc = "Vdet1_0"]
        pub const _0_X_0_E: Self = Self::new(14);

        #[doc = "Vdet1_1"]
        pub const _0_X_0_F: Self = Self::new(15);

        #[doc = "Vdet1_2"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "Vdet1_3"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "Vdet1_4"]
        pub const _0_X_12: Self = Self::new(18);

        #[doc = "Vdet1_5"]
        pub const _0_X_13: Self = Self::new(19);

        #[doc = "Vdet1_6"]
        pub const _0_X_14: Self = Self::new(20);

        #[doc = "Vdet1_7"]
        pub const _0_X_15: Self = Self::new(21);

        #[doc = "Vdet1_8"]
        pub const _0_X_16: Self = Self::new(22);

        #[doc = "Vdet1_9"]
        pub const _0_X_17: Self = Self::new(23);

        #[doc = "Vdet1_A"]
        pub const _0_X_18: Self = Self::new(24);

        #[doc = "Vdet1_B"]
        pub const _0_X_19: Self = Self::new(25);

        #[doc = "Vdet1_C"]
        pub const _0_X_1_A: Self = Self::new(26);

        #[doc = "Vdet1_D"]
        pub const _0_X_1_B: Self = Self::new(27);

        #[doc = "Vdet1_E"]
        pub const _0_X_1_C: Self = Self::new(28);

        #[doc = "Vdet1_F"]
        pub const _0_X_1_D: Self = Self::new(29);

        #[doc = "Vdet1_10"]
        pub const _0_X_1_E: Self = Self::new(30);

        #[doc = "Vdet1_11"]
        pub const _0_X_1_F: Self = Self::new(31);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqsel_SPEC;
    pub type Irqsel = crate::EnumBitfieldStruct<u8, Irqsel_SPEC>;
    impl Irqsel {
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Maskable interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Sel_SPEC;
    pub type Lvd1Sel = crate::EnumBitfieldStruct<u8, Lvd1Sel_SPEC>;
    impl Lvd1Sel {
        #[doc = "Interrupt mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1En_SPEC;
    pub type Lvd1En = crate::EnumBitfieldStruct<u8, Lvd1En_SPEC>;
    impl Lvd1En {
        #[doc = "Operation stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operation enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd1Mkr_SPEC;
impl crate::sealed::RegSpec for Lvd1Mkr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 1 Circuit Mask Register"]
pub type Lvd1Mkr = crate::RegValueT<Lvd1Mkr_SPEC>;

impl Lvd1Mkr {
    #[doc = "Specification of Whether to Enable or Disable Rewriting th LVD1CR Register"]
    #[inline(always)]
    pub fn mk(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lvd1mkr::Mk,
        lvd1mkr::Mk,
        Lvd1Mkr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lvd1mkr::Mk,
            lvd1mkr::Mk,
            Lvd1Mkr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd1Mkr {
    #[inline(always)]
    fn default() -> Lvd1Mkr {
        <crate::RegValueT<Lvd1Mkr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lvd1mkr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mk_SPEC;
    pub type Mk = crate::EnumBitfieldStruct<u8, Mk_SPEC>;
    impl Mk {
        #[doc = "Rewriting of the LVD1CR register is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Rewriting of the LVD1CR register is enabled (reset and interrupt generation by LVD1 are masked)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd1Sr_SPEC;
impl crate::sealed::RegSpec for Lvd1Sr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 1 Circuit Status Register"]
pub type Lvd1Sr = crate::RegValueT<Lvd1Sr_SPEC>;

impl Lvd1Sr {
    #[doc = "Voltage Monitor 1 Voltage Variation Detection Flag"]
    #[inline(always)]
    pub fn det(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lvd1sr::Det,
        lvd1sr::Det,
        Lvd1Sr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lvd1sr::Det,
            lvd1sr::Det,
            Lvd1Sr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Signal Monitor Flag"]
    #[inline(always)]
    pub fn mon(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        lvd1sr::Mon,
        lvd1sr::Mon,
        Lvd1Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            lvd1sr::Mon,
            lvd1sr::Mon,
            Lvd1Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd1Sr {
    #[inline(always)]
    fn default() -> Lvd1Sr {
        <crate::RegValueT<Lvd1Sr_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod lvd1sr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Det_SPEC;
    pub type Det = crate::EnumBitfieldStruct<u8, Det_SPEC>;
    impl Det {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Vdet1 crossing is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mon_SPEC;
    pub type Mon = crate::EnumBitfieldStruct<u8, Mon_SPEC>;
    impl Mon {
        #[doc = "VCC < Vdet1"]
        pub const _0: Self = Self::new(0);

        #[doc = "VCC ≥ Vdet1 or MON is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbycr_SPEC;
impl crate::sealed::RegSpec for Sbycr_SPEC {
    type DataType = u16;
}

#[doc = "Standby Control Register"]
pub type Sbycr = crate::RegValueT<Sbycr_SPEC>;

impl Sbycr {
    #[doc = "Flash Mode in Sleep Mode or in Snooze Mode"]
    #[inline(always)]
    pub fn flstp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sbycr::Flstp,
        sbycr::Flstp,
        Sbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sbycr::Flstp,
            sbycr::Flstp,
            Sbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting for Starting the High-speed On-chip Oscillator at the times of release from Software Standby Mode and of Transitions to Snooze Mode"]
    #[inline(always)]
    pub fn fwkup(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sbycr::Fwkup,
        sbycr::Fwkup,
        Sbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sbycr::Fwkup,
            sbycr::Fwkup,
            Sbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SOSC Setting in Software Standby Mode or in Snooze Mode"]
    #[inline(always)]
    pub fn rtclpc(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        sbycr::Rtclpc,
        sbycr::Rtclpc,
        Sbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            sbycr::Rtclpc,
            sbycr::Rtclpc,
            Sbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software Standby Mode Select"]
    #[inline(always)]
    pub fn ssby(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        sbycr::Ssby,
        sbycr::Ssby,
        Sbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            sbycr::Ssby,
            sbycr::Ssby,
            Sbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sbycr {
    #[inline(always)]
    fn default() -> Sbycr {
        <crate::RegValueT<Sbycr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sbycr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flstp_SPEC;
    pub type Flstp = crate::EnumBitfieldStruct<u8, Flstp_SPEC>;
    impl Flstp {
        #[doc = "Flash active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Flash stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fwkup_SPEC;
    pub type Fwkup = crate::EnumBitfieldStruct<u8, Fwkup_SPEC>;
    impl Fwkup {
        #[doc = "Starting of the high-speed on-chip oscillator is at normal speed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Starting of the high-speed on-chip oscillator is at high speed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtclpc_SPEC;
    pub type Rtclpc = crate::EnumBitfieldStruct<u8, Rtclpc_SPEC>;
    impl Rtclpc {
        #[doc = "Enables supply of SOSC clock to peripheral functions"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stops supply SOSC clock to peripheral functions other than the Realtime clock"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssby_SPEC;
    pub type Ssby = crate::EnumBitfieldStruct<u8, Ssby_SPEC>;
    impl Ssby {
        #[doc = "Sleep mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psmcr_SPEC;
impl crate::sealed::RegSpec for Psmcr_SPEC {
    type DataType = u8;
}

#[doc = "Power Save Memory Control Register"]
pub type Psmcr = crate::RegValueT<Psmcr_SPEC>;

impl Psmcr {
    #[doc = "Operating Mode of the RAM"]
    #[inline(always)]
    pub fn ramsd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        psmcr::Ramsd,
        psmcr::Ramsd,
        Psmcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            psmcr::Ramsd,
            psmcr::Ramsd,
            Psmcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Psmcr {
    #[inline(always)]
    fn default() -> Psmcr {
        <crate::RegValueT<Psmcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod psmcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ramsd_SPEC;
    pub type Ramsd = crate::EnumBitfieldStruct<u8, Ramsd_SPEC>;
    impl Ramsd {
        #[doc = "Normal mode (continues to operate)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Standby mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Shutdown mode"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syocdcr_SPEC;
impl crate::sealed::RegSpec for Syocdcr_SPEC {
    type DataType = u8;
}

#[doc = "System Control OCD Control Register"]
pub type Syocdcr = crate::RegValueT<Syocdcr_SPEC>;

impl Syocdcr {
    #[doc = "Debugger Enable bit"]
    #[inline(always)]
    pub fn dbgen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        syocdcr::Dbgen,
        syocdcr::Dbgen,
        Syocdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            syocdcr::Dbgen,
            syocdcr::Dbgen,
            Syocdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syocdcr {
    #[inline(always)]
    fn default() -> Syocdcr {
        <crate::RegValueT<Syocdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syocdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbgen_SPEC;
    pub type Dbgen = crate::EnumBitfieldStruct<u8, Dbgen_SPEC>;
    impl Dbgen {
        #[doc = "On-chip debugger is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "On-chip debugger is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prcr_SPEC;
impl crate::sealed::RegSpec for Prcr_SPEC {
    type DataType = u16;
}

#[doc = "Protect Register"]
pub type Prcr = crate::RegValueT<Prcr_SPEC>;

impl Prcr {
    #[doc = "Enable writing to the registers related to the clock generation circuit"]
    #[inline(always)]
    pub fn prc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        prcr::Prc0,
        prcr::Prc0,
        Prcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            prcr::Prc0,
            prcr::Prc0,
            Prcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable writing to the registers related to the low power modes"]
    #[inline(always)]
    pub fn prc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        prcr::Prc1,
        prcr::Prc1,
        Prcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            prcr::Prc1,
            prcr::Prc1,
            Prcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable writing to the registers related to the LVD"]
    #[inline(always)]
    pub fn prc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        prcr::Prc3,
        prcr::Prc3,
        Prcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            prcr::Prc3,
            prcr::Prc3,
            Prcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PRC Key Code"]
    #[inline(always)]
    pub fn prkey(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Prcr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Prcr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Prcr {
    #[inline(always)]
    fn default() -> Prcr {
        <crate::RegValueT<Prcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod prcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc0_SPEC;
    pub type Prc0 = crate::EnumBitfieldStruct<u8, Prc0_SPEC>;
    impl Prc0 {
        #[doc = "Disable writes"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc1_SPEC;
    pub type Prc1 = crate::EnumBitfieldStruct<u8, Prc1_SPEC>;
    impl Prc1 {
        #[doc = "Disable writes"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc3_SPEC;
    pub type Prc3 = crate::EnumBitfieldStruct<u8, Prc3_SPEC>;
    impl Prc3 {
        #[doc = "Disable writes"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcra_SPEC;
impl crate::sealed::RegSpec for Mstpcra_SPEC {
    type DataType = u16;
}

#[doc = "Module Stop Control Register A"]
pub type Mstpcra = crate::RegValueT<Mstpcra_SPEC>;

impl Mstpcra {
    #[doc = "DTC Module Stop"]
    #[inline(always)]
    pub fn mstpa22(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mstpcra::Mstpa22,
        mstpcra::Mstpa22,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mstpcra::Mstpa22,
            mstpcra::Mstpa22,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mstpcra {
    #[inline(always)]
    fn default() -> Mstpcra {
        <crate::RegValueT<Mstpcra_SPEC> as RegisterValue<_>>::new(65471)
    }
}
pub mod mstpcra {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa22_SPEC;
    pub type Mstpa22 = crate::EnumBitfieldStruct<u8, Mstpa22_SPEC>;
    impl Mstpa22 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
}
