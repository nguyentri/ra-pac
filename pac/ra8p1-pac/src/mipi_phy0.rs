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
// Generated from SVD 1.00.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:55:39 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"PHY Controller Top"]
unsafe impl ::core::marker::Send for super::MipiPhy0 {}
unsafe impl ::core::marker::Sync for super::MipiPhy0 {}
impl super::MipiPhy0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "D-PHY Reference Clock Setting Register"]
    #[inline(always)]
    pub const fn dphyrefcr(
        &self,
    ) -> &'static crate::common::Reg<self::Dphyrefcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphyrefcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "D-PHY PLL Frequency Control Register"]
    #[inline(always)]
    pub const fn dphyplfcr(
        &self,
    ) -> &'static crate::common::Reg<self::Dphyplfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphyplfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "D-PHY PLL Operation Control Register"]
    #[inline(always)]
    pub const fn dphyplocr(
        &self,
    ) -> &'static crate::common::Reg<self::Dphyplocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphyplocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "D-PHY Escape Mode Clock Control Register"]
    #[inline(always)]
    pub const fn dphyesccr(
        &self,
    ) -> &'static crate::common::Reg<self::Dphyesccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphyesccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "D-PHY Power Supplying Control Register"]
    #[inline(always)]
    pub const fn dphypwrcr(
        &self,
    ) -> &'static crate::common::Reg<self::Dphypwrcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphypwrcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "D-PHY Status Flag Register"]
    #[inline(always)]
    pub const fn dphysfr(
        &self,
    ) -> &'static crate::common::Reg<self::Dphysfr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dphysfr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "D-PHY Operation Control Register"]
    #[inline(always)]
    pub const fn dphyocr(
        &self,
    ) -> &'static crate::common::Reg<self::Dphyocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphyocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 1"]
    #[inline(always)]
    pub const fn dphytim1(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 2"]
    #[inline(always)]
    pub const fn dphytim2(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 3"]
    #[inline(always)]
    pub const fn dphytim3(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 4"]
    #[inline(always)]
    pub const fn dphytim4(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 5"]
    #[inline(always)]
    pub const fn dphytim5(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 6"]
    #[inline(always)]
    pub const fn dphytim6(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "D-PHY Mode Control Register"]
    #[inline(always)]
    pub const fn dphymdc(
        &self,
    ) -> &'static crate::common::Reg<self::Dphymdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphymdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphyrefcr_SPEC;
impl crate::sealed::RegSpec for Dphyrefcr_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Reference Clock Setting Register"]
pub type Dphyrefcr = crate::RegValueT<Dphyrefcr_SPEC>;

impl Dphyrefcr {
    #[doc = "Reference Clock Frequency Setting"]
    #[inline(always)]
    pub fn rfreq(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphyrefcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphyrefcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphyrefcr {
    #[inline(always)]
    fn default() -> Dphyrefcr {
        <crate::RegValueT<Dphyrefcr_SPEC> as RegisterValue<_>>::new(47)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphyplfcr_SPEC;
impl crate::sealed::RegSpec for Dphyplfcr_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY PLL Frequency Control Register"]
pub type Dphyplfcr = crate::RegValueT<Dphyplfcr_SPEC>;

impl Dphyplfcr {
    #[doc = "D-PHY PLL Input Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn idiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        dphyplfcr::Idiv,
        dphyplfcr::Idiv,
        Dphyplfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            dphyplfcr::Idiv,
            dphyplfcr::Idiv,
            Dphyplfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "D-PHY PLL Frequency Multiplication Factor Select (Fractional Part)"]
    #[inline(always)]
    pub fn nfmul(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        dphyplfcr::Nfmul,
        dphyplfcr::Nfmul,
        Dphyplfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            dphyplfcr::Nfmul,
            dphyplfcr::Nfmul,
            Dphyplfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pmul(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        dphyplfcr::Pmul,
        dphyplfcr::Pmul,
        Dphyplfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            dphyplfcr::Pmul,
            dphyplfcr::Pmul,
            Dphyplfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "D-PHY PLL Frequency Multiplication Factor Select (Integer Part)"]
    #[inline(always)]
    pub fn nmul(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Dphyplfcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Dphyplfcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphyplfcr {
    #[inline(always)]
    fn default() -> Dphyplfcr {
        <crate::RegValueT<Dphyplfcr_SPEC> as RegisterValue<_>>::new(1245184)
    }
}
pub mod dphyplfcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idiv_SPEC;
    pub type Idiv = crate::EnumBitfieldStruct<u8, Idiv_SPEC>;
    impl Idiv {
        #[doc = "1"]
        pub const _00: Self = Self::new(0);

        #[doc = "1/2"]
        pub const _01: Self = Self::new(1);

        #[doc = "1/3"]
        pub const _10: Self = Self::new(2);

        #[doc = "1/4"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfmul_SPEC;
    pub type Nfmul = crate::EnumBitfieldStruct<u8, Nfmul_SPEC>;
    impl Nfmul {
        #[doc = "0.00"]
        pub const _00: Self = Self::new(0);

        #[doc = "0.33"]
        pub const _01: Self = Self::new(1);

        #[doc = "0.66"]
        pub const _10: Self = Self::new(2);

        #[doc = "0.50"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmul_SPEC;
    pub type Pmul = crate::EnumBitfieldStruct<u8, Pmul_SPEC>;
    impl Pmul {
        #[doc = "1"]
        pub const _00: Self = Self::new(0);

        #[doc = "1/2"]
        pub const _01: Self = Self::new(1);

        #[doc = "1/4"]
        pub const _10: Self = Self::new(2);

        #[doc = "1/8"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphyplocr_SPEC;
impl crate::sealed::RegSpec for Dphyplocr_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY PLL Operation Control Register"]
pub type Dphyplocr = crate::RegValueT<Dphyplocr_SPEC>;

impl Dphyplocr {
    #[doc = "D-PHY PLL Operation Control"]
    #[inline(always)]
    pub fn pllstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphyplocr::Pllstp,
        dphyplocr::Pllstp,
        Dphyplocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphyplocr::Pllstp,
            dphyplocr::Pllstp,
            Dphyplocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dphyplocr {
    #[inline(always)]
    fn default() -> Dphyplocr {
        <crate::RegValueT<Dphyplocr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod dphyplocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllstp_SPEC;
    pub type Pllstp = crate::EnumBitfieldStruct<u8, Pllstp_SPEC>;
    impl Pllstp {
        #[doc = "Operate the PLL"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the PLL"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphyesccr_SPEC;
impl crate::sealed::RegSpec for Dphyesccr_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Escape Mode Clock Control Register"]
pub type Dphyesccr = crate::RegValueT<Dphyesccr_SPEC>;

impl Dphyesccr {
    #[doc = "Escape Mode Transfer Clock Division Ratio"]
    #[inline(always)]
    pub fn escdiv(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Dphyesccr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Dphyesccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphyesccr {
    #[inline(always)]
    fn default() -> Dphyesccr {
        <crate::RegValueT<Dphyesccr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphypwrcr_SPEC;
impl crate::sealed::RegSpec for Dphypwrcr_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Power Supplying Control Register"]
pub type Dphypwrcr = crate::RegValueT<Dphypwrcr_SPEC>;

impl Dphypwrcr {
    #[doc = "D-PHY Power Supplying Control"]
    #[inline(always)]
    pub fn pwrsen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphypwrcr::Pwrsen,
        dphypwrcr::Pwrsen,
        Dphypwrcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphypwrcr::Pwrsen,
            dphypwrcr::Pwrsen,
            Dphypwrcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dphypwrcr {
    #[inline(always)]
    fn default() -> Dphypwrcr {
        <crate::RegValueT<Dphypwrcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphypwrcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pwrsen_SPEC;
    pub type Pwrsen = crate::EnumBitfieldStruct<u8, Pwrsen_SPEC>;
    impl Pwrsen {
        #[doc = "Disable D-PHY LDO operation (stop supplying VDD_DPHY)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable D-PHY LDO operation (supply VDD_DPHY)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphysfr_SPEC;
impl crate::sealed::RegSpec for Dphysfr_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Status Flag Register"]
pub type Dphysfr = crate::RegValueT<Dphysfr_SPEC>;

impl Dphysfr {
    #[doc = "D-PHY LDO Power-on Status Flag"]
    #[inline(always)]
    pub fn pwrsf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphysfr::Pwrsf,
        dphysfr::Pwrsf,
        Dphysfr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphysfr::Pwrsf,
            dphysfr::Pwrsf,
            Dphysfr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "D-PHY PLL Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn pllsf(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dphysfr::Pllsf,
        dphysfr::Pllsf,
        Dphysfr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dphysfr::Pllsf,
            dphysfr::Pllsf,
            Dphysfr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dphysfr {
    #[inline(always)]
    fn default() -> Dphysfr {
        <crate::RegValueT<Dphysfr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphysfr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pwrsf_SPEC;
    pub type Pwrsf = crate::EnumBitfieldStruct<u8, Pwrsf_SPEC>;
    impl Pwrsf {
        #[doc = "D-PHY LDO is stopped or starting up"]
        pub const _0: Self = Self::new(0);

        #[doc = "D-PHY LDO startup is completed (VDD_DPHY is stable)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllsf_SPEC;
    pub type Pllsf = crate::EnumBitfieldStruct<u8, Pllsf_SPEC>;
    impl Pllsf {
        #[doc = "D-PHY PLL clock is stopped or is not yet stable"]
        pub const _0: Self = Self::new(0);

        #[doc = "D-PHY PLL clock is stable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphyocr_SPEC;
impl crate::sealed::RegSpec for Dphyocr_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Operation Control Register"]
pub type Dphyocr = crate::RegValueT<Dphyocr_SPEC>;

impl Dphyocr {
    #[doc = "D-PHY Operation Control"]
    #[inline(always)]
    pub fn dphyen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphyocr::Dphyen,
        dphyocr::Dphyen,
        Dphyocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphyocr::Dphyen,
            dphyocr::Dphyen,
            Dphyocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dphyocr {
    #[inline(always)]
    fn default() -> Dphyocr {
        <crate::RegValueT<Dphyocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphyocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dphyen_SPEC;
    pub type Dphyen = crate::EnumBitfieldStruct<u8, Dphyen_SPEC>;
    impl Dphyen {
        #[doc = "Disable D-PHY operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable D-PHY operation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim1_SPEC;
impl crate::sealed::RegSpec for Dphytim1_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Timing Control Register 1"]
pub type Dphytim1 = crate::RegValueT<Dphytim1_SPEC>;

impl Dphytim1 {
    #[doc = "D-PHY T_INIT Parameter Setting"]
    #[inline(always)]
    pub fn tinit(
        self,
    ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, u32, Dphytim1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ffff,1,0,u32,u32,Dphytim1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim1 {
    #[inline(always)]
    fn default() -> Dphytim1 {
        <crate::RegValueT<Dphytim1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim2_SPEC;
impl crate::sealed::RegSpec for Dphytim2_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Timing Control Register 2"]
pub type Dphytim2 = crate::RegValueT<Dphytim2_SPEC>;

impl Dphytim2 {
    #[doc = "D-PHY T_CLK_PREPARE Parameter Setting"]
    #[inline(always)]
    pub fn tclkprep(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_CLK_SETTLE Parameter Setting"]
    #[inline(always)]
    pub fn tclksett(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dphytim2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Dphytim2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_CLK_MISS Parameter Setting"]
    #[inline(always)]
    pub fn tclkmiss(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Dphytim2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Dphytim2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim2 {
    #[inline(always)]
    fn default() -> Dphytim2 {
        <crate::RegValueT<Dphytim2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim3_SPEC;
impl crate::sealed::RegSpec for Dphytim3_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Timing Control Register 3"]
pub type Dphytim3 = crate::RegValueT<Dphytim3_SPEC>;

impl Dphytim3 {
    #[doc = "D-PHY T_THS_PREPARE Parameter Setting"]
    #[inline(always)]
    pub fn thsprep(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_THS_SETTLE Parameter Setting"]
    #[inline(always)]
    pub fn thssett(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dphytim3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Dphytim3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim3 {
    #[inline(always)]
    fn default() -> Dphytim3 {
        <crate::RegValueT<Dphytim3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim4_SPEC;
impl crate::sealed::RegSpec for Dphytim4_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Timing Control Register 4"]
pub type Dphytim4 = crate::RegValueT<Dphytim4_SPEC>;

impl Dphytim4 {
    #[doc = "D-PHY T_CLK_ZERO Parameter Setting"]
    #[inline(always)]
    pub fn tclkzero(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_TCLK_PRE Parameter Setting"]
    #[inline(always)]
    pub fn tclkpre(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dphytim4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Dphytim4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_TCLK_POST Parameter Setting"]
    #[inline(always)]
    pub fn tclkpost(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Dphytim4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Dphytim4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_TCLK_TRAIL Parameter Setting"]
    #[inline(always)]
    pub fn tclktrl(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Dphytim4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Dphytim4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim4 {
    #[inline(always)]
    fn default() -> Dphytim4 {
        <crate::RegValueT<Dphytim4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim5_SPEC;
impl crate::sealed::RegSpec for Dphytim5_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Timing Control Register 5"]
pub type Dphytim5 = crate::RegValueT<Dphytim5_SPEC>;

impl Dphytim5 {
    #[doc = "D-PHY T_THS_ZERO Parameter Setting"]
    #[inline(always)]
    pub fn thszero(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_THS_TRAIL Parameter Setting"]
    #[inline(always)]
    pub fn thstrl(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dphytim5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Dphytim5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_THS_EXIT Parameter Setting"]
    #[inline(always)]
    pub fn thsexit(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Dphytim5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Dphytim5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim5 {
    #[inline(always)]
    fn default() -> Dphytim5 {
        <crate::RegValueT<Dphytim5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim6_SPEC;
impl crate::sealed::RegSpec for Dphytim6_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Timing Control Register 6"]
pub type Dphytim6 = crate::RegValueT<Dphytim6_SPEC>;

impl Dphytim6 {
    #[doc = "D-PHY T_TLPX Parameter Setting"]
    #[inline(always)]
    pub fn tlpx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim6 {
    #[inline(always)]
    fn default() -> Dphytim6 {
        <crate::RegValueT<Dphytim6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphymdc_SPEC;
impl crate::sealed::RegSpec for Dphymdc_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Mode Control Register"]
pub type Dphymdc = crate::RegValueT<Dphymdc_SPEC>;

impl Dphymdc {
    #[doc = "D-PHY Master/Slave Select"]
    #[inline(always)]
    pub fn masteren(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphymdc::Masteren,
        dphymdc::Masteren,
        Dphymdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphymdc::Masteren,
            dphymdc::Masteren,
            Dphymdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dphymdc {
    #[inline(always)]
    fn default() -> Dphymdc {
        <crate::RegValueT<Dphymdc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphymdc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Masteren_SPEC;
    pub type Masteren = crate::EnumBitfieldStruct<u8, Masteren_SPEC>;
    impl Masteren {
        #[doc = "Slave Mode (CSI-2 Version 3.0, D-PHY v2.1 CIL_SCNN,CIL_SFEN)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Master Mode (DSI-2 Version 1.1, D-PHY v2.1 CIL_MCNN,CIL_MFAA)"]
        pub const _1: Self = Self::new(1);
    }
}
