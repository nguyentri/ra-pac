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
#[doc = r"A/D Converter"]
unsafe impl ::core::marker::Send for super::AdcD {}
unsafe impl ::core::marker::Sync for super::AdcD {}
impl super::AdcD {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "A/D Converter Mode Register 0"]
    #[inline(always)]
    pub const fn adm0(&self) -> &'static crate::common::Reg<self::Adm0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adm0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Analog Input Channel Specification Register"]
    #[inline(always)]
    pub const fn ads(&self) -> &'static crate::common::Reg<self::Ads_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ads_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "A/D Converter Mode Register 1"]
    #[inline(always)]
    pub const fn adm1(&self) -> &'static crate::common::Reg<self::Adm1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adm1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "12-bit or 10-bit A/D Conversion Result Register"]
    #[inline(always)]
    pub const fn adcr(&self) -> &'static crate::common::Reg<self::Adcr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adcr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "8-bit A/D Conversion Result Register"]
    #[inline(always)]
    pub const fn adcrh(&self) -> &'static crate::common::Reg<self::Adcrh_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adcrh_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[doc = "A/D Converter Mode Register 2"]
    #[inline(always)]
    pub const fn adm2(&self) -> &'static crate::common::Reg<self::Adm2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adm2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "Conversion Result Comparison Upper Limit Setting Register"]
    #[inline(always)]
    pub const fn adul(&self) -> &'static crate::common::Reg<self::Adul_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adul_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(273usize),
            )
        }
    }

    #[doc = "Conversion Result Comparison Lower Limit Setting Register"]
    #[inline(always)]
    pub const fn adll(&self) -> &'static crate::common::Reg<self::Adll_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adll_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(274usize),
            )
        }
    }

    #[doc = "A/D Test Register"]
    #[inline(always)]
    pub const fn adtes(&self) -> &'static crate::common::Reg<self::Adtes_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adtes_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(275usize),
            )
        }
    }

    #[doc = "12-bit or 10-bit A/D Conversion Result Register 0"]
    #[inline(always)]
    pub const fn adcr0(&self) -> &'static crate::common::Reg<self::Adcr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adcr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "8-bit A/D Conversion Result Register 0"]
    #[inline(always)]
    pub const fn adcr0h(&self) -> &'static crate::common::Reg<self::Adcr0H_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adcr0H_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(289usize),
            )
        }
    }

    #[doc = "12-bit or 10-bit A/D Conversion Result Register 1"]
    #[inline(always)]
    pub const fn adcr1(&self) -> &'static crate::common::Reg<self::Adcr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adcr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(290usize),
            )
        }
    }

    #[doc = "8-bit A/D Conversion Result Register 1"]
    #[inline(always)]
    pub const fn adcr1h(&self) -> &'static crate::common::Reg<self::Adcr1H_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adcr1H_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(291usize),
            )
        }
    }

    #[doc = "12-bit or 10-bit A/D Conversion Result Register 2"]
    #[inline(always)]
    pub const fn adcr2(&self) -> &'static crate::common::Reg<self::Adcr2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adcr2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[doc = "8-bit A/D Conversion Result Register 2"]
    #[inline(always)]
    pub const fn adcr2h(&self) -> &'static crate::common::Reg<self::Adcr2H_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adcr2H_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(293usize),
            )
        }
    }

    #[doc = "12-bit or 10-bit A/D Conversion Result Register 3"]
    #[inline(always)]
    pub const fn adcr3(&self) -> &'static crate::common::Reg<self::Adcr3_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adcr3_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(294usize),
            )
        }
    }

    #[doc = "8-bit A/D Conversion Result Register 3"]
    #[inline(always)]
    pub const fn adcr3h(&self) -> &'static crate::common::Reg<self::Adcr3H_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adcr3H_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(295usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adm0_SPEC;
impl crate::sealed::RegSpec for Adm0_SPEC {
    type DataType = u8;
}

#[doc = "A/D Converter Mode Register 0"]
pub type Adm0 = crate::RegValueT<Adm0_SPEC>;

impl Adm0 {
    #[doc = "A/D voltage comparator operation control"]
    #[inline(always)]
    pub fn adce(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adm0::Adce,
        adm0::Adce,
        Adm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adm0::Adce,
            adm0::Adce,
            Adm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Select Operation voltage mode"]
    #[inline(always)]
    pub fn lv(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, adm0::Lv, adm0::Lv, Adm0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,adm0::Lv,adm0::Lv,Adm0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select Conversion Clock (fAD)"]
    #[inline(always)]
    pub fn fr(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, adm0::Fr, adm0::Fr, Adm0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,adm0::Fr,adm0::Fr,Adm0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Specification of the A/D conversion channel selection mode"]
    #[inline(always)]
    pub fn admd(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adm0::Admd,
        adm0::Admd,
        Adm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adm0::Admd,
            adm0::Admd,
            Adm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D conversion operation control"]
    #[inline(always)]
    pub fn adcs(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adm0::Adcs,
        adm0::Adcs,
        Adm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adm0::Adcs,
            adm0::Adcs,
            Adm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adm0 {
    #[inline(always)]
    fn default() -> Adm0 {
        <crate::RegValueT<Adm0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adm0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adce_SPEC;
    pub type Adce = crate::EnumBitfieldStruct<u8, Adce_SPEC>;
    impl Adce {
        #[doc = "Stops A/D voltage comparator operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables A/D voltage comparator operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lv_SPEC;
    pub type Lv = crate::EnumBitfieldStruct<u8, Lv_SPEC>;
    impl Lv {
        #[doc = "Normal mode 1"]
        pub const _00: Self = Self::new(0);

        #[doc = "Normal mode 2"]
        pub const _01: Self = Self::new(1);

        #[doc = "Low voltage mode 1"]
        pub const _10: Self = Self::new(2);

        #[doc = "Low voltage mode 2"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fr_SPEC;
    pub type Fr = crate::EnumBitfieldStruct<u8, Fr_SPEC>;
    impl Fr {
        #[doc = "PCLKB/32"]
        pub const _000: Self = Self::new(0);

        #[doc = "PCLKB/16"]
        pub const _001: Self = Self::new(1);

        #[doc = "PCLKB/8"]
        pub const _010: Self = Self::new(2);

        #[doc = "PCLKB/4"]
        pub const _011: Self = Self::new(3);

        #[doc = "PCLKB/2"]
        pub const _100: Self = Self::new(4);

        #[doc = "PCLKB"]
        pub const _101: Self = Self::new(5);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Admd_SPEC;
    pub type Admd = crate::EnumBitfieldStruct<u8, Admd_SPEC>;
    impl Admd {
        #[doc = "Select mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Scan mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adcs_SPEC;
    pub type Adcs = crate::EnumBitfieldStruct<u8, Adcs_SPEC>;
    impl Adcs {
        #[doc = "Stops conversion operation \\[When read\\] Conversion is stopped or in standby"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables conversion operation \\[When read\\] While in the no wait mode (both software and hardware trigger mode):Conversion is enabledWhile in the wait mode (both software and hardware trigger mode):A/D power supply stabilization wait time + conversion"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ads_SPEC;
impl crate::sealed::RegSpec for Ads_SPEC {
    type DataType = u8;
}

#[doc = "Analog Input Channel Specification Register"]
pub type Ads = crate::RegValueT<Ads_SPEC>;

impl Ads {
    #[doc = "Selection of the Analog Input Channel (See to )"]
    #[inline(always)]
    pub fn ads(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Ads_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Ads_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select Internal or External of Analog Input (See to )"]
    #[inline(always)]
    pub fn adiss(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ads::Adiss,
        ads::Adiss,
        Ads_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ads::Adiss,
            ads::Adiss,
            Ads_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ads {
    #[inline(always)]
    fn default() -> Ads {
        <crate::RegValueT<Ads_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ads {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adiss_SPEC;
    pub type Adiss = crate::EnumBitfieldStruct<u8, Adiss_SPEC>;
    impl Adiss {
        #[doc = "External input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal circuit input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adm1_SPEC;
impl crate::sealed::RegSpec for Adm1_SPEC {
    type DataType = u8;
}

#[doc = "A/D Converter Mode Register 1"]
pub type Adm1 = crate::RegValueT<Adm1_SPEC>;

impl Adm1 {
    #[doc = "Selection of the Hardware Trigger Signal"]
    #[inline(always)]
    pub fn adtrs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        adm1::Adtrs,
        adm1::Adtrs,
        Adm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            adm1::Adtrs,
            adm1::Adtrs,
            Adm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PCLKB Input Frequency Setting"]
    #[inline(always)]
    pub fn adlsp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adm1::Adlsp,
        adm1::Adlsp,
        Adm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adm1::Adlsp,
            adm1::Adlsp,
            Adm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Specification of the A/D Conversion Mode"]
    #[inline(always)]
    pub fn adscm(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adm1::Adscm,
        adm1::Adscm,
        Adm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adm1::Adscm,
            adm1::Adscm,
            Adm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of the A/D Conversion Trigger Mode"]
    #[inline(always)]
    pub fn adtmd(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        adm1::Adtmd,
        adm1::Adtmd,
        Adm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            adm1::Adtmd,
            adm1::Adtmd,
            Adm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adm1 {
    #[inline(always)]
    fn default() -> Adm1 {
        <crate::RegValueT<Adm1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adm1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtrs_SPEC;
    pub type Adtrs = crate::EnumBitfieldStruct<u8, Adtrs_SPEC>;
    impl Adtrs {
        #[doc = "Timer Array Unit channel 1 count or capture end interrupt signal (TAU0_TMI01)"]
        pub const _000: Self = Self::new(0);

        #[doc = "Realtime clock interrupt signal (RTC_ALM_OR_PRD)"]
        pub const _010: Self = Self::new(2);

        #[doc = "32-bit interval timer event signal (ADITL0 (= TML32_ITL0))"]
        pub const _011: Self = Self::new(3);

        #[doc = "Event input signal (ELC_AD)"]
        pub const _100: Self = Self::new(4);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adlsp_SPEC;
    pub type Adlsp = crate::EnumBitfieldStruct<u8, Adlsp_SPEC>;
    impl Adlsp {
        #[doc = "4 MHz < PCLKB ≤ 32 MHz"]
        pub const _0: Self = Self::new(0);

        #[doc = "1 MHz ≤ PCLKB ≤ 4 MHz"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adscm_SPEC;
    pub type Adscm = crate::EnumBitfieldStruct<u8, Adscm_SPEC>;
    impl Adscm {
        #[doc = "Sequential conversion mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "One-shot conversion mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtmd_SPEC;
    pub type Adtmd = crate::EnumBitfieldStruct<u8, Adtmd_SPEC>;
    impl Adtmd {
        #[doc = "Hardware trigger no-wait mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Hardware trigger wait mode"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcr_SPEC;
impl crate::sealed::RegSpec for Adcr_SPEC {
    type DataType = u16;
}

#[doc = "12-bit or 10-bit A/D Conversion Result Register"]
pub type Adcr = crate::RegValueT<Adcr_SPEC>;

impl NoBitfieldReg<Adcr_SPEC> for Adcr {}
impl ::core::default::Default for Adcr {
    #[inline(always)]
    fn default() -> Adcr {
        <crate::RegValueT<Adcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcrh_SPEC;
impl crate::sealed::RegSpec for Adcrh_SPEC {
    type DataType = u8;
}

#[doc = "8-bit A/D Conversion Result Register"]
pub type Adcrh = crate::RegValueT<Adcrh_SPEC>;

impl NoBitfieldReg<Adcrh_SPEC> for Adcrh {}
impl ::core::default::Default for Adcrh {
    #[inline(always)]
    fn default() -> Adcrh {
        <crate::RegValueT<Adcrh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adm2_SPEC;
impl crate::sealed::RegSpec for Adm2_SPEC {
    type DataType = u8;
}

#[doc = "A/D Converter Mode Register 2"]
pub type Adm2 = crate::RegValueT<Adm2_SPEC>;

impl Adm2 {
    #[inline(always)]
    pub fn adtyp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adm2::Adtyp,
        adm2::Adtyp,
        Adm2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adm2::Adtyp,
            adm2::Adtyp,
            Adm2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn awc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adm2::Awc,
        adm2::Awc,
        Adm2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adm2::Awc,
            adm2::Awc,
            Adm2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adrck(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adm2::Adrck,
        adm2::Adrck,
        Adm2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adm2::Adrck,
            adm2::Adrck,
            Adm2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adrefm(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adm2::Adrefm,
        adm2::Adrefm,
        Adm2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adm2::Adrefm,
            adm2::Adrefm,
            Adm2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adrefp(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        adm2::Adrefp,
        adm2::Adrefp,
        Adm2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            adm2::Adrefp,
            adm2::Adrefp,
            Adm2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adm2 {
    #[inline(always)]
    fn default() -> Adm2 {
        <crate::RegValueT<Adm2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adm2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtyp_SPEC;
    pub type Adtyp = crate::EnumBitfieldStruct<u8, Adtyp_SPEC>;
    impl Adtyp {
        #[doc = "10-bit resolution"]
        pub const _00: Self = Self::new(0);

        #[doc = "8-bit resolution"]
        pub const _01: Self = Self::new(1);

        #[doc = "12-bit resolution"]
        pub const _10: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Awc_SPEC;
    pub type Awc = crate::EnumBitfieldStruct<u8, Awc_SPEC>;
    impl Awc {
        #[doc = "Do not use the Snooze mode function."]
        pub const _0: Self = Self::new(0);

        #[doc = "Use the Snooze mode function."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adrck_SPEC;
    pub type Adrck = crate::EnumBitfieldStruct<u8, Adrck_SPEC>;
    impl Adrck {
        #[doc = "The interrupt signal (ADC12_ADI) is output when the ADLL register ≤ the ADCRn register ≤ the ADUL register (AREA 1)."]
        pub const _0: Self = Self::new(0);

        #[doc = "The interrupt signal (ADC12_ADI) is output when the ADCRn register < the ADLL register (AREA 2) or the ADUL register < the ADCRn register (AREA 3)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adrefm_SPEC;
    pub type Adrefm = crate::EnumBitfieldStruct<u8, Adrefm_SPEC>;
    impl Adrefm {
        #[doc = "Supplied from VSS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Supplied from VREFL0/AN001"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adrefp_SPEC;
    pub type Adrefp = crate::EnumBitfieldStruct<u8, Adrefp_SPEC>;
    impl Adrefp {
        #[doc = "Supplied from VCC"]
        pub const _00: Self = Self::new(0);

        #[doc = "Supplied from VREFH0/AN000"]
        pub const _01: Self = Self::new(1);

        #[doc = "Supplied from the internal reference voltage"]
        pub const _10: Self = Self::new(2);

        #[doc = "Discharge the internal circuitry"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adul_SPEC;
impl crate::sealed::RegSpec for Adul_SPEC {
    type DataType = u8;
}

#[doc = "Conversion Result Comparison Upper Limit Setting Register"]
pub type Adul = crate::RegValueT<Adul_SPEC>;

impl NoBitfieldReg<Adul_SPEC> for Adul {}
impl ::core::default::Default for Adul {
    #[inline(always)]
    fn default() -> Adul {
        <crate::RegValueT<Adul_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adll_SPEC;
impl crate::sealed::RegSpec for Adll_SPEC {
    type DataType = u8;
}

#[doc = "Conversion Result Comparison Lower Limit Setting Register"]
pub type Adll = crate::RegValueT<Adll_SPEC>;

impl NoBitfieldReg<Adll_SPEC> for Adll {}
impl ::core::default::Default for Adll {
    #[inline(always)]
    fn default() -> Adll {
        <crate::RegValueT<Adll_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adtes_SPEC;
impl crate::sealed::RegSpec for Adtes_SPEC {
    type DataType = u8;
}

#[doc = "A/D Test Register"]
pub type Adtes = crate::RegValueT<Adtes_SPEC>;

impl Adtes {
    #[doc = "Selection of A/D Conversion Target for Testing"]
    #[inline(always)]
    pub fn adtes(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adtes::Adtes,
        adtes::Adtes,
        Adtes_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adtes::Adtes,
            adtes::Adtes,
            Adtes_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adtes {
    #[inline(always)]
    fn default() -> Adtes {
        <crate::RegValueT<Adtes_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adtes {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtes_SPEC;
    pub type Adtes = crate::EnumBitfieldStruct<u8, Adtes_SPEC>;
    impl Adtes {
        #[doc = "ANxxx, temperature sensor output voltage or internal reference voltage (Set by analog input channel specification register (ADS))"]
        pub const _00: Self = Self::new(0);

        #[doc = "The ‘-’ side reference voltage (selected by the ADREFM bit of the ADM2 register)"]
        pub const _10: Self = Self::new(2);

        #[doc = "The ‘+’ side reference voltage (selected by the ADREFP\\[1:0\\] bits of the ADM2 register)"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcr0_SPEC;
impl crate::sealed::RegSpec for Adcr0_SPEC {
    type DataType = u16;
}

#[doc = "12-bit or 10-bit A/D Conversion Result Register 0"]
pub type Adcr0 = crate::RegValueT<Adcr0_SPEC>;

impl NoBitfieldReg<Adcr0_SPEC> for Adcr0 {}
impl ::core::default::Default for Adcr0 {
    #[inline(always)]
    fn default() -> Adcr0 {
        <crate::RegValueT<Adcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcr0H_SPEC;
impl crate::sealed::RegSpec for Adcr0H_SPEC {
    type DataType = u8;
}

#[doc = "8-bit A/D Conversion Result Register 0"]
pub type Adcr0H = crate::RegValueT<Adcr0H_SPEC>;

impl NoBitfieldReg<Adcr0H_SPEC> for Adcr0H {}
impl ::core::default::Default for Adcr0H {
    #[inline(always)]
    fn default() -> Adcr0H {
        <crate::RegValueT<Adcr0H_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcr1_SPEC;
impl crate::sealed::RegSpec for Adcr1_SPEC {
    type DataType = u16;
}

#[doc = "12-bit or 10-bit A/D Conversion Result Register 1"]
pub type Adcr1 = crate::RegValueT<Adcr1_SPEC>;

impl NoBitfieldReg<Adcr1_SPEC> for Adcr1 {}
impl ::core::default::Default for Adcr1 {
    #[inline(always)]
    fn default() -> Adcr1 {
        <crate::RegValueT<Adcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcr1H_SPEC;
impl crate::sealed::RegSpec for Adcr1H_SPEC {
    type DataType = u8;
}

#[doc = "8-bit A/D Conversion Result Register 1"]
pub type Adcr1H = crate::RegValueT<Adcr1H_SPEC>;

impl NoBitfieldReg<Adcr1H_SPEC> for Adcr1H {}
impl ::core::default::Default for Adcr1H {
    #[inline(always)]
    fn default() -> Adcr1H {
        <crate::RegValueT<Adcr1H_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcr2_SPEC;
impl crate::sealed::RegSpec for Adcr2_SPEC {
    type DataType = u16;
}

#[doc = "12-bit or 10-bit A/D Conversion Result Register 2"]
pub type Adcr2 = crate::RegValueT<Adcr2_SPEC>;

impl NoBitfieldReg<Adcr2_SPEC> for Adcr2 {}
impl ::core::default::Default for Adcr2 {
    #[inline(always)]
    fn default() -> Adcr2 {
        <crate::RegValueT<Adcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcr2H_SPEC;
impl crate::sealed::RegSpec for Adcr2H_SPEC {
    type DataType = u8;
}

#[doc = "8-bit A/D Conversion Result Register 2"]
pub type Adcr2H = crate::RegValueT<Adcr2H_SPEC>;

impl NoBitfieldReg<Adcr2H_SPEC> for Adcr2H {}
impl ::core::default::Default for Adcr2H {
    #[inline(always)]
    fn default() -> Adcr2H {
        <crate::RegValueT<Adcr2H_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcr3_SPEC;
impl crate::sealed::RegSpec for Adcr3_SPEC {
    type DataType = u16;
}

#[doc = "12-bit or 10-bit A/D Conversion Result Register 3"]
pub type Adcr3 = crate::RegValueT<Adcr3_SPEC>;

impl NoBitfieldReg<Adcr3_SPEC> for Adcr3 {}
impl ::core::default::Default for Adcr3 {
    #[inline(always)]
    fn default() -> Adcr3 {
        <crate::RegValueT<Adcr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcr3H_SPEC;
impl crate::sealed::RegSpec for Adcr3H_SPEC {
    type DataType = u8;
}

#[doc = "8-bit A/D Conversion Result Register 3"]
pub type Adcr3H = crate::RegValueT<Adcr3H_SPEC>;

impl NoBitfieldReg<Adcr3H_SPEC> for Adcr3H {}
impl ::core::default::Default for Adcr3H {
    #[inline(always)]
    fn default() -> Adcr3H {
        <crate::RegValueT<Adcr3H_SPEC> as RegisterValue<_>>::new(0)
    }
}
