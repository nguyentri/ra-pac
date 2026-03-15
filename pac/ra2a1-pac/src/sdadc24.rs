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
#[doc = r"24-Bit Sigma-Delta A/D Converter"]
unsafe impl ::core::marker::Send for super::Sdadc24 {}
unsafe impl ::core::marker::Sync for super::Sdadc24 {}
impl super::Sdadc24 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Startup Control Register 1"]
    #[inline(always)]
    pub const fn stc1(&self) -> &'static crate::common::Reg<self::Stc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Startup Control Register 2"]
    #[inline(always)]
    pub const fn stc2(&self) -> &'static crate::common::Reg<self::Stc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Input Multiplexer %s Setting Register"]
    #[inline(always)]
    pub const fn pgac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pgac_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x8usize))
        }
    }
    #[inline(always)]
    pub const fn pgac0(&self) -> &'static crate::common::Reg<self::Pgac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pgac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pgac1(&self) -> &'static crate::common::Reg<self::Pgac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pgac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pgac2(&self) -> &'static crate::common::Reg<self::Pgac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pgac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pgac3(&self) -> &'static crate::common::Reg<self::Pgac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pgac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pgac4(&self) -> &'static crate::common::Reg<self::Pgac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pgac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }

    #[doc = "Sigma-delta A/D Converter Control Register 1"]
    #[inline(always)]
    pub const fn adc1(&self) -> &'static crate::common::Reg<self::Adc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Sigma-delta A/D Converter Control Register 2"]
    #[inline(always)]
    pub const fn adc2(&self) -> &'static crate::common::Reg<self::Adc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Sigma-delta A/D Converter Conversion Result Register"]
    #[inline(always)]
    pub const fn adcr(&self) -> &'static crate::common::Reg<self::Adcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Sigma-delta A/D Converter Average Value Register"]
    #[inline(always)]
    pub const fn adar(&self) -> &'static crate::common::Reg<self::Adar_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adar_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Calibration Control Register"]
    #[inline(always)]
    pub const fn clbc(&self) -> &'static crate::common::Reg<self::Clbc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Clbc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Calibration Start Control Register"]
    #[inline(always)]
    pub const fn clbstr(
        &self,
    ) -> &'static crate::common::Reg<self::Clbstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Clbstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Calibration Status Register"]
    #[inline(always)]
    pub const fn clbssr(&self) -> &'static crate::common::Reg<self::Clbssr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Clbssr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stc1_SPEC;
impl crate::sealed::RegSpec for Stc1_SPEC {
    type DataType = u16;
}

#[doc = "Startup Control Register 1"]
pub type Stc1 = crate::RegValueT<Stc1_SPEC>;

impl Stc1 {
    #[doc = "VREF mode select"]
    #[inline(always)]
    pub fn vrefsel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        stc1::Vrefsel,
        stc1::Vrefsel,
        Stc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            stc1::Vrefsel,
            stc1::Vrefsel,
            Stc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Reference voltage select"]
    #[inline(always)]
    pub fn vsbias(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        stc1::Vsbias,
        stc1::Vsbias,
        Stc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            stc1::Vsbias,
            stc1::Vsbias,
            Stc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D conversion operation mode select"]
    #[inline(always)]
    pub fn sdadlpm(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        stc1::Sdadlpm,
        stc1::Sdadlpm,
        Stc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            stc1::Sdadlpm,
            stc1::Sdadlpm,
            Stc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Stc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Stc1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SDADC24 reference clock division select"]
    #[inline(always)]
    pub fn clkdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        stc1::Clkdiv,
        stc1::Clkdiv,
        Stc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            stc1::Clkdiv,
            stc1::Clkdiv,
            Stc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Stc1 {
    #[inline(always)]
    fn default() -> Stc1 {
        <crate::RegValueT<Stc1_SPEC> as RegisterValue<_>>::new(32776)
    }
}
pub mod stc1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vrefsel_SPEC;
    pub type Vrefsel = crate::EnumBitfieldStruct<u8, Vrefsel_SPEC>;
    impl Vrefsel {
        #[doc = "Internal VREF mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "External VREF mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vsbias_SPEC;
    pub type Vsbias = crate::EnumBitfieldStruct<u8, Vsbias_SPEC>;
    impl Vsbias {
        #[doc = "0.8 V"]
        pub const _0000: Self = Self::new(0);

        #[doc = "1.0 V"]
        pub const _0001: Self = Self::new(1);

        #[doc = "1.2 V"]
        pub const _0010: Self = Self::new(2);

        #[doc = "1.4 V"]
        pub const _0011: Self = Self::new(3);

        #[doc = "1.6 V"]
        pub const _0100: Self = Self::new(4);

        #[doc = "1.8 V"]
        pub const _0101: Self = Self::new(5);

        #[doc = "2.0 V"]
        pub const _0110: Self = Self::new(6);

        #[doc = "2.2 V"]
        pub const _0111: Self = Self::new(7);

        #[doc = "2.4 V (This voltage can be set only if VREFSEL = 1.  When VREFSEL = 0, 2.2 V is set (rather than 2.4 V))"]
        pub const _1111: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadlpm_SPEC;
    pub type Sdadlpm = crate::EnumBitfieldStruct<u8, Sdadlpm_SPEC>;
    impl Sdadlpm {
        #[doc = "Normal A/D conversion mode: SDADC24 reference clock:4 MHz, Oversampling clock:1 MHz"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low-power A/D conversion mode(1/8 of the clock in normal A/D conversion mode): SDADC24 reference clock:500 kHz,Oversampling clock: 125 kHz"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clkdiv_SPEC;
    pub type Clkdiv = crate::EnumBitfieldStruct<u8, Clkdiv_SPEC>;
    impl Clkdiv {
        #[doc = "SDADCCLK (no division)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "SDADCCLK/2 (1/2)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "SDADCCLK/3 (1/3)"]
        pub const _0010: Self = Self::new(2);

        #[doc = "SDADCCLK/4 (1/4)"]
        pub const _0011: Self = Self::new(3);

        #[doc = "SDADCCLK/5 (1/5)"]
        pub const _0100: Self = Self::new(4);

        #[doc = "SDADCCLK/6 (1/6)"]
        pub const _0101: Self = Self::new(5);

        #[doc = "SDADCCLK/8 (1/8)"]
        pub const _0110: Self = Self::new(6);

        #[doc = "SDADCCLK/12 (1/12)"]
        pub const _0111: Self = Self::new(7);

        #[doc = "SDADCCLK/16 (1/16)."]
        pub const _1000: Self = Self::new(8);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stc2_SPEC;
impl crate::sealed::RegSpec for Stc2_SPEC {
    type DataType = u8;
}

#[doc = "Startup Control Register 2"]
pub type Stc2 = crate::RegValueT<Stc2_SPEC>;

impl Stc2 {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Stc2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Stc2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ADREG forced power-down mode"]
    #[inline(always)]
    pub fn adfpwds(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        stc2::Adfpwds,
        stc2::Adfpwds,
        Stc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            stc2::Adfpwds,
            stc2::Adfpwds,
            Stc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ADC reference supply part power control"]
    #[inline(always)]
    pub fn adcpon(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        stc2::Adcpon,
        stc2::Adcpon,
        Stc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            stc2::Adcpon,
            stc2::Adcpon,
            Stc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BGR part power control"]
    #[inline(always)]
    pub fn bgrpon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        stc2::Bgrpon,
        stc2::Bgrpon,
        Stc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            stc2::Bgrpon,
            stc2::Bgrpon,
            Stc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Stc2 {
    #[inline(always)]
    fn default() -> Stc2 {
        <crate::RegValueT<Stc2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod stc2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adfpwds_SPEC;
    pub type Adfpwds = crate::EnumBitfieldStruct<u8, Adfpwds_SPEC>;
    impl Adfpwds {
        #[doc = "Power of ADREG is controlled by the BGRPON setting"]
        pub const _0: Self = Self::new(0);

        #[doc = "Power of only ADREG is turned off regardless of the BGRPON setting"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adcpon_SPEC;
    pub type Adcpon = crate::EnumBitfieldStruct<u8, Adcpon_SPEC>;
    impl Adcpon {
        #[doc = "Turn off the power of VBIAS, PGA and sigma-delta A/D converter"]
        pub const _0: Self = Self::new(0);

        #[doc = "Turn on the power of VBIAS, PGA and sigma-delta A/D converter"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bgrpon_SPEC;
    pub type Bgrpon = crate::EnumBitfieldStruct<u8, Bgrpon_SPEC>;
    impl Bgrpon {
        #[doc = "Turn off the power of ADBGR, SBIAS/VREFI, and ADREG"]
        pub const _0: Self = Self::new(0);

        #[doc = "Turn on the power of ADBGR, SBIAS/VREFI, and ADREG"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pgac_SPEC;
impl crate::sealed::RegSpec for Pgac_SPEC {
    type DataType = u32;
}

#[doc = "Input Multiplexer %s Setting Register"]
pub type Pgac = crate::RegValueT<Pgac_SPEC>;

impl Pgac {
    #[doc = "Selection of the mode for specifying the number of A/D conversions in ADSCAN"]
    #[inline(always)]
    pub fn pgaasn(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        pgac::Pgaasn,
        pgac::Pgaasn,
        Pgac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            pgac::Pgaasn,
            pgac::Pgaasn,
            Pgac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Calibration enable"]
    #[inline(always)]
    pub fn pgacve(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        pgac::Pgacve,
        pgac::Pgacve,
        Pgac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            pgac::Pgacve,
            pgac::Pgacve,
            Pgac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Single-End Input A/D Converted Data Inversion Select"]
    #[inline(always)]
    pub fn pgarev(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        pgac::Pgarev,
        pgac::Pgarev,
        Pgac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            pgac::Pgarev,
            pgac::Pgarev,
            Pgac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of averaging processing"]
    #[inline(always)]
    pub fn pgaave(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x3,
        1,
        0,
        pgac::Pgaave,
        pgac::Pgaave,
        Pgac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x3,
            1,
            0,
            pgac::Pgaave,
            pgac::Pgaave,
            Pgac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of the number of data to be averaged"]
    #[inline(always)]
    pub fn pgaavn(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        pgac::Pgaavn,
        pgac::Pgaavn,
        Pgac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            pgac::Pgaavn,
            pgac::Pgaavn,
            Pgac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Coefficient (n) selection of the A/D conversion count (N) in AUTOSCAN"]
    #[inline(always)]
    pub fn pgactn(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x7,
        1,
        0,
        pgac::Pgactn,
        pgac::Pgactn,
        Pgac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x7,
            1,
            0,
            pgac::Pgactn,
            pgac::Pgactn,
            Pgac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Coefficient (m) selection of the A/D conversion count (N) in AUTOSCAN"]
    #[inline(always)]
    pub fn pgactm(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Pgac_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Pgac_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Analog Channel Input Mode Select"]
    #[inline(always)]
    pub fn pgasel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pgac::Pgasel,
        pgac::Pgasel,
        Pgac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pgac::Pgasel,
            pgac::Pgasel,
            Pgac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Polarity select"]
    #[inline(always)]
    pub fn pgapol(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pgac::Pgapol,
        pgac::Pgapol,
        Pgac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pgac::Pgapol,
            pgac::Pgapol,
            Pgac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pgac_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Pgac_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Offset voltage select"]
    #[inline(always)]
    pub fn pgaofs(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Pgac_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Pgac_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Oversampling ratio select"]
    #[inline(always)]
    pub fn pgaosr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7,
        1,
        0,
        pgac::Pgaosr,
        pgac::Pgaosr,
        Pgac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7,
            1,
            0,
            pgac::Pgaosr,
            pgac::Pgaosr,
            Pgac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Gain selection of a programmable gain instrumentation amplifier ( Gset1, Gset2, Gtotal )"]
    #[inline(always)]
    pub fn pgagc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        pgac::Pgagc,
        pgac::Pgagc,
        Pgac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            pgac::Pgagc,
            pgac::Pgagc,
            Pgac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pgac {
    #[inline(always)]
    fn default() -> Pgac {
        <crate::RegValueT<Pgac_SPEC> as RegisterValue<_>>::new(65600)
    }
}
pub mod pgac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgaasn_SPEC;
    pub type Pgaasn = crate::EnumBitfieldStruct<u8, Pgaasn_SPEC>;
    impl Pgaasn {
        #[doc = "Specify 1 to 8,032 times by using the value set in the PGACTN\\[2:0\\] and PGACTM\\[4:0\\] bits"]
        pub const _0: Self = Self::new(0);

        #[doc = "Specify 1 to 255 times linearly by using the value set in the PGACTN\\[2:0\\] and PGACTM\\[4:0\\] bits"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgacve_SPEC;
    pub type Pgacve = crate::EnumBitfieldStruct<u8, Pgacve_SPEC>;
    impl Pgacve {
        #[doc = "Do not calculate the calibration correction factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Calculate the calibration correction factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgarev_SPEC;
    pub type Pgarev = crate::EnumBitfieldStruct<u8, Pgarev_SPEC>;
    impl Pgarev {
        #[doc = "Do not invert the conversion result data"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invert the conversion result data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgaave_SPEC;
    pub type Pgaave = crate::EnumBitfieldStruct<u8, Pgaave_SPEC>;
    impl Pgaave {
        #[doc = "Do not average the A/D conversion results"]
        pub const _00: Self = Self::new(0);

        #[doc = "Do not average the A/D conversion results"]
        pub const _01: Self = Self::new(1);

        #[doc = "Average the A/D conversion results and generates SDADC_ADI each time an A/D conversion occurs"]
        pub const _10: Self = Self::new(2);

        #[doc = "Perform averaging, and generate SDADC_ADI at each time of average value output (A/D conversion is performed N times)."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgaavn_SPEC;
    pub type Pgaavn = crate::EnumBitfieldStruct<u8, Pgaavn_SPEC>;
    impl Pgaavn {
        #[doc = "8"]
        pub const _00: Self = Self::new(0);

        #[doc = "16"]
        pub const _01: Self = Self::new(1);

        #[doc = "32"]
        pub const _10: Self = Self::new(2);

        #[doc = "64"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgactn_SPEC;
    pub type Pgactn = crate::EnumBitfieldStruct<u8, Pgactn_SPEC>;
    impl Pgactn {
        #[doc = "0"]
        pub const _000: Self = Self::new(0);

        #[doc = "1"]
        pub const _001: Self = Self::new(1);

        #[doc = "2"]
        pub const _010: Self = Self::new(2);

        #[doc = "3"]
        pub const _011: Self = Self::new(3);

        #[doc = "4"]
        pub const _100: Self = Self::new(4);

        #[doc = "5"]
        pub const _101: Self = Self::new(5);

        #[doc = "6"]
        pub const _110: Self = Self::new(6);

        #[doc = "7"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgasel_SPEC;
    pub type Pgasel = crate::EnumBitfieldStruct<u8, Pgasel_SPEC>;
    impl Pgasel {
        #[doc = "Differential input mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Single-end input mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgapol_SPEC;
    pub type Pgapol = crate::EnumBitfieldStruct<u8, Pgapol_SPEC>;
    impl Pgapol {
        #[doc = "Positive-side single-end input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Negative-side single-end input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgaosr_SPEC;
    pub type Pgaosr = crate::EnumBitfieldStruct<u8, Pgaosr_SPEC>;
    impl Pgaosr {
        #[doc = "64"]
        pub const _000: Self = Self::new(0);

        #[doc = "128"]
        pub const _001: Self = Self::new(1);

        #[doc = "256"]
        pub const _010: Self = Self::new(2);

        #[doc = "512"]
        pub const _011: Self = Self::new(3);

        #[doc = "1024"]
        pub const _100: Self = Self::new(4);

        #[doc = "2048"]
        pub const _101: Self = Self::new(5);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgagc_SPEC;
    pub type Pgagc = crate::EnumBitfieldStruct<u8, Pgagc_SPEC>;
    impl Pgagc {
        #[doc = "(1, 1, 1)"]
        pub const _00000: Self = Self::new(0);

        #[doc = "(2, 1, 2)"]
        pub const _00100: Self = Self::new(4);

        #[doc = "(3, 1, 3)"]
        pub const _01000: Self = Self::new(8);

        #[doc = "(4, 1, 4)"]
        pub const _01100: Self = Self::new(12);

        #[doc = "(8, 1, 8)"]
        pub const _10000: Self = Self::new(16);

        #[doc = "(1, 2, 2)"]
        pub const _00001: Self = Self::new(1);

        #[doc = "(2, 2, 4)"]
        pub const _00101: Self = Self::new(5);

        #[doc = "(3, 2, 6)"]
        pub const _01001: Self = Self::new(9);

        #[doc = "(4, 2, 8)"]
        pub const _01101: Self = Self::new(13);

        #[doc = "(8, 2, 16)"]
        pub const _10001: Self = Self::new(17);

        #[doc = "(1, 4, 4)"]
        pub const _00010: Self = Self::new(2);

        #[doc = "(2, 4, 8)"]
        pub const _00110: Self = Self::new(6);

        #[doc = "(3, 4, 12)"]
        pub const _01010: Self = Self::new(10);

        #[doc = "(4, 4, 16)"]
        pub const _01110: Self = Self::new(14);

        #[doc = "(8, 4, 32)"]
        pub const _10010: Self = Self::new(18);

        #[doc = "(1, 8, 8)"]
        pub const _00011: Self = Self::new(3);

        #[doc = "(2, 8, 16)"]
        pub const _00111: Self = Self::new(7);

        #[doc = "(3, 8, 24)"]
        pub const _01011: Self = Self::new(11);

        #[doc = "(4, 8, 32)."]
        pub const _01111: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc1_SPEC;
impl crate::sealed::RegSpec for Adc1_SPEC {
    type DataType = u32;
}

#[doc = "Sigma-delta A/D Converter Control Register 1"]
pub type Adc1 = crate::RegValueT<Adc1_SPEC>;

impl Adc1 {
    #[doc = "PGA offset self-diagnosis enable"]
    #[inline(always)]
    pub fn pgaslft(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        adc1::Pgaslft,
        adc1::Pgaslft,
        Adc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            adc1::Pgaslft,
            adc1::Pgaslft,
            Adc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Disconnection Detection Assist Setting"]
    #[inline(always)]
    pub fn pgadisc(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adc1::Pgadisc,
        adc1::Pgadisc,
        Adc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adc1::Pgadisc,
            adc1::Pgadisc,
            Adc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control of disconnection detection"]
    #[inline(always)]
    pub fn pgadisa(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adc1::Pgadisa,
        adc1::Pgadisa,
        Adc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adc1::Pgadisa,
            adc1::Pgadisa,
            Adc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D conversion control of the signal from input multiplexer"]
    #[inline(always)]
    pub fn sdadbmp(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Adc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Adc1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selection of A/D conversion trigger signal"]
    #[inline(always)]
    pub fn sdadtmd(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adc1::Sdadtmd,
        adc1::Sdadtmd,
        Adc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adc1::Sdadtmd,
            adc1::Sdadtmd,
            Adc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, u8, Adc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7,1,0,u8,u8,Adc1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selection of autoscan mode"]
    #[inline(always)]
    pub fn sdadscm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adc1::Sdadscm,
        adc1::Sdadscm,
        Adc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adc1::Sdadscm,
            adc1::Sdadscm,
            Adc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adc1 {
    #[inline(always)]
    fn default() -> Adc1 {
        <crate::RegValueT<Adc1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adc1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgaslft_SPEC;
    pub type Pgaslft = crate::EnumBitfieldStruct<u8, Pgaslft_SPEC>;
    impl Pgaslft {
        #[doc = "Disable PGA offset self-diagnosis"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable PGA offset self-diagnosis"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgadisc_SPEC;
    pub type Pgadisc = crate::EnumBitfieldStruct<u8, Pgadisc_SPEC>;
    impl Pgadisc {
        #[doc = "Discharge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pre-charge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgadisa_SPEC;
    pub type Pgadisa = crate::EnumBitfieldStruct<u8, Pgadisa_SPEC>;
    impl Pgadisa {
        #[doc = "Normal operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "State of disconnection detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadtmd_SPEC;
    pub type Sdadtmd = crate::EnumBitfieldStruct<u8, Sdadtmd_SPEC>;
    impl Sdadtmd {
        #[doc = "Software trigger (conversion is started by a write to SFR)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Hardware trigger (conversion is started in synchronization with the event signal selected by ELC_SDADC24)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadscm_SPEC;
    pub type Sdadscm = crate::EnumBitfieldStruct<u8, Sdadscm_SPEC>;
    impl Sdadscm {
        #[doc = "Continuous scan mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Single scan mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc2_SPEC;
impl crate::sealed::RegSpec for Adc2_SPEC {
    type DataType = u8;
}

#[doc = "Sigma-delta A/D Converter Control Register 2"]
pub type Adc2 = crate::RegValueT<Adc2_SPEC>;

impl Adc2 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Adc2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Adc2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Control of A/D conversion"]
    #[inline(always)]
    pub fn sdadst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adc2::Sdadst,
        adc2::Sdadst,
        Adc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adc2::Sdadst,
            adc2::Sdadst,
            Adc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adc2 {
    #[inline(always)]
    fn default() -> Adc2 {
        <crate::RegValueT<Adc2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adc2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadst_SPEC;
    pub type Sdadst = crate::EnumBitfieldStruct<u8, Sdadst_SPEC>;
    impl Sdadst {
        #[doc = "Stop A/D conversion"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start A/D conversion"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcr_SPEC;
impl crate::sealed::RegSpec for Adcr_SPEC {
    type DataType = u32;
}

#[doc = "Sigma-delta A/D Converter Conversion Result Register"]
pub type Adcr = crate::RegValueT<Adcr_SPEC>;

impl Adcr {
    #[doc = "Channel number for an A/D conversion result"]
    #[inline(always)]
    pub fn sdadcrc(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x7,
        1,
        0,
        adcr::Sdadcrc,
        adcr::Sdadcrc,
        Adcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x7,
            1,
            0,
            adcr::Sdadcrc,
            adcr::Sdadcrc,
            Adcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Status of an A/D conversion result"]
    #[inline(always)]
    pub fn sdadcrs(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        adcr::Sdadcrs,
        adcr::Sdadcrs,
        Adcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            adcr::Sdadcrs,
            adcr::Sdadcrs,
            Adcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "The 24-bit A/D conversion result"]
    #[inline(always)]
    pub fn sdadcrd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Adcr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Adcr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adcr {
    #[inline(always)]
    fn default() -> Adcr {
        <crate::RegValueT<Adcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadcrc_SPEC;
    pub type Sdadcrc = crate::EnumBitfieldStruct<u8, Sdadcrc_SPEC>;
    impl Sdadcrc {
        #[doc = "Reset value (Conversion result is invalid)"]
        pub const _000: Self = Self::new(0);

        #[doc = "Input multiplexer 0 (ANSD0P / ANSD0N)"]
        pub const _001: Self = Self::new(1);

        #[doc = "Input multiplexer 1 (ANSD1P / ANSD1N)"]
        pub const _010: Self = Self::new(2);

        #[doc = "Input multiplexer 2 (ANSD2P / ANSD2N)"]
        pub const _011: Self = Self::new(3);

        #[doc = "Input multiplexer 3 (ANSD3P / ANSD3N)"]
        pub const _100: Self = Self::new(4);

        #[doc = "Input multiplexer 4 (AMP0O / AMP1O)"]
        pub const _101: Self = Self::new(5);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadcrs_SPEC;
    pub type Sdadcrs = crate::EnumBitfieldStruct<u8, Sdadcrs_SPEC>;
    impl Sdadcrs {
        #[doc = "Normal status (within the range)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Overflow occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adar_SPEC;
impl crate::sealed::RegSpec for Adar_SPEC {
    type DataType = u32;
}

#[doc = "Sigma-delta A/D Converter Average Value Register"]
pub type Adar = crate::RegValueT<Adar_SPEC>;

impl Adar {
    #[doc = "Channel number for an A/D conversion result"]
    #[inline(always)]
    pub fn sdadmvc(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x7,
        1,
        0,
        adar::Sdadmvc,
        adar::Sdadmvc,
        Adar_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x7,
            1,
            0,
            adar::Sdadmvc,
            adar::Sdadmvc,
            Adar_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Status of an A/D conversion result"]
    #[inline(always)]
    pub fn sdadmvs(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        adar::Sdadmvs,
        adar::Sdadmvs,
        Adar_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            adar::Sdadmvs,
            adar::Sdadmvs,
            Adar_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "The 24-bit A/D average value"]
    #[inline(always)]
    pub fn sdadmvd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Adar_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Adar_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adar_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adar_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Adar {
    #[inline(always)]
    fn default() -> Adar {
        <crate::RegValueT<Adar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadmvc_SPEC;
    pub type Sdadmvc = crate::EnumBitfieldStruct<u8, Sdadmvc_SPEC>;
    impl Sdadmvc {
        #[doc = "Reset value (Conversion result is invalid)"]
        pub const _000: Self = Self::new(0);

        #[doc = "Input multiplexer 0 (ANSD0P / ANSD0N)"]
        pub const _001: Self = Self::new(1);

        #[doc = "Input multiplexer 1 (ANSD1P / ANSD1N)"]
        pub const _010: Self = Self::new(2);

        #[doc = "Input multiplexer 2 (ANSD2P / ANSD2N)"]
        pub const _011: Self = Self::new(3);

        #[doc = "Input multiplexer 3 (ANSD3P / ANSD3N)"]
        pub const _100: Self = Self::new(4);

        #[doc = "Input multiplexer 4 (AMP0O / AMP1O)."]
        pub const _101: Self = Self::new(5);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadmvs_SPEC;
    pub type Sdadmvs = crate::EnumBitfieldStruct<u8, Sdadmvs_SPEC>;
    impl Sdadmvs {
        #[doc = "Normal status (within the range)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Overflow occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clbc_SPEC;
impl crate::sealed::RegSpec for Clbc_SPEC {
    type DataType = u8;
}

#[doc = "Calibration Control Register"]
pub type Clbc = crate::RegValueT<Clbc_SPEC>;

impl Clbc {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Clbc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Clbc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn clbmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        clbc::Clbmd,
        clbc::Clbmd,
        Clbc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            clbc::Clbmd,
            clbc::Clbmd,
            Clbc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Clbc {
    #[inline(always)]
    fn default() -> Clbc {
        <crate::RegValueT<Clbc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod clbc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clbmd_SPEC;
    pub type Clbmd = crate::EnumBitfieldStruct<u8, Clbmd_SPEC>;
    impl Clbmd {
        #[doc = "Internal calibration mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "External offset calibration mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "External gain calibration mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Settings are prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clbstr_SPEC;
impl crate::sealed::RegSpec for Clbstr_SPEC {
    type DataType = u8;
}

#[doc = "Calibration Start Control Register"]
pub type Clbstr = crate::RegValueT<Clbstr_SPEC>;

impl Clbstr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Clbstr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Clbstr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Calibration start control"]
    #[inline(always)]
    pub fn clbst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        clbstr::Clbst,
        clbstr::Clbst,
        Clbstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            clbstr::Clbst,
            clbstr::Clbst,
            Clbstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Clbstr {
    #[inline(always)]
    fn default() -> Clbstr {
        <crate::RegValueT<Clbstr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod clbstr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clbst_SPEC;
    pub type Clbst = crate::EnumBitfieldStruct<u8, Clbst_SPEC>;
    impl Clbst {
        #[doc = "Disable writing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start calibration"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clbssr_SPEC;
impl crate::sealed::RegSpec for Clbssr_SPEC {
    type DataType = u8;
}

#[doc = "Calibration Status Register"]
pub type Clbssr = crate::RegValueT<Clbssr_SPEC>;

impl Clbssr {
    #[doc = "These bits are read as 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Clbssr_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Clbssr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Calibration status"]
    #[inline(always)]
    pub fn clbss(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        clbssr::Clbss,
        clbssr::Clbss,
        Clbssr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            clbssr::Clbss,
            clbssr::Clbss,
            Clbssr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Clbssr {
    #[inline(always)]
    fn default() -> Clbssr {
        <crate::RegValueT<Clbssr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod clbssr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clbss_SPEC;
    pub type Clbss = crate::EnumBitfieldStruct<u8, Clbss_SPEC>;
    impl Clbss {
        #[doc = "Calibration is not running"]
        pub const _0: Self = Self::new(0);

        #[doc = "Calibration is running"]
        pub const _1: Self = Self::new(1);
    }
}
