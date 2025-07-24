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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:51:32 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Quad-SPI"]
unsafe impl ::core::marker::Send for super::Qspi {}
unsafe impl ::core::marker::Sync for super::Qspi {}
impl super::Qspi {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Transfer Mode Control Register"]
    #[inline(always)]
    pub const fn sfmsmd(
        &self,
    ) -> &'static crate::common::Reg<self::Sfmsmd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sfmsmd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Chip Selection Control Register"]
    #[inline(always)]
    pub const fn sfmssc(
        &self,
    ) -> &'static crate::common::Reg<self::Sfmssc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sfmssc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Clock Control Register"]
    #[inline(always)]
    pub const fn sfmskc(
        &self,
    ) -> &'static crate::common::Reg<self::Sfmskc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sfmskc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sfmsst(&self) -> &'static crate::common::Reg<self::Sfmsst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sfmsst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Communication Port Register"]
    #[inline(always)]
    pub const fn sfmcom(
        &self,
    ) -> &'static crate::common::Reg<self::Sfmcom_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sfmcom_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Communication Mode Control Register"]
    #[inline(always)]
    pub const fn sfmcmd(
        &self,
    ) -> &'static crate::common::Reg<self::Sfmcmd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sfmcmd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Communication Status Register"]
    #[inline(always)]
    pub const fn sfmcst(
        &self,
    ) -> &'static crate::common::Reg<self::Sfmcst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sfmcst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Instruction Code Register"]
    #[inline(always)]
    pub const fn sfmsic(
        &self,
    ) -> &'static crate::common::Reg<self::Sfmsic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sfmsic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Address Mode Control Register"]
    #[inline(always)]
    pub const fn sfmsac(
        &self,
    ) -> &'static crate::common::Reg<self::Sfmsac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sfmsac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Dummy Cycle Control Register"]
    #[inline(always)]
    pub const fn sfmsdc(
        &self,
    ) -> &'static crate::common::Reg<self::Sfmsdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sfmsdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "SPI Protocol Control Register"]
    #[inline(always)]
    pub const fn sfmspc(
        &self,
    ) -> &'static crate::common::Reg<self::Sfmspc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sfmspc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Port Control Register"]
    #[inline(always)]
    pub const fn sfmpmd(
        &self,
    ) -> &'static crate::common::Reg<self::Sfmpmd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sfmpmd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "External QSPI Address Register 1"]
    #[inline(always)]
    pub const fn sfmcnt1(
        &self,
    ) -> &'static crate::common::Reg<self::Sfmcnt1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sfmcnt1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2052usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfmsmd_SPEC;
impl crate::sealed::RegSpec for Sfmsmd_SPEC {
    type DataType = u32;
}

#[doc = "Transfer Mode Control Register"]
pub type Sfmsmd = crate::RegValueT<Sfmsmd_SPEC>;

impl Sfmsmd {
    #[doc = "Read instruction code selection."]
    #[inline(always)]
    pub fn sfmcce(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        sfmsmd::Sfmcce,
        sfmsmd::Sfmcce,
        Sfmsmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            sfmsmd::Sfmcce,
            sfmsmd::Sfmcce,
            Sfmsmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setup time adjustment for serial transmission"]
    #[inline(always)]
    pub fn sfmosw(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        sfmsmd::Sfmosw,
        sfmsmd::Sfmosw,
        Sfmsmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            sfmsmd::Sfmosw,
            sfmsmd::Sfmosw,
            Sfmsmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Hold time adjustment for serial transmission"]
    #[inline(always)]
    pub fn sfmohw(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        sfmsmd::Sfmohw,
        sfmsmd::Sfmohw,
        Sfmsmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            sfmsmd::Sfmohw,
            sfmsmd::Sfmohw,
            Sfmsmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Extension of the I/O buffer output enable signal for the serial interface"]
    #[inline(always)]
    pub fn sfmoex(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        sfmsmd::Sfmoex,
        sfmsmd::Sfmoex,
        Sfmsmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            sfmsmd::Sfmoex,
            sfmsmd::Sfmoex,
            Sfmsmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI mode selection. An initial value is determined by input to CFGMD3."]
    #[inline(always)]
    pub fn sfmmd3(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sfmsmd::Sfmmd3,
        sfmsmd::Sfmmd3,
        Sfmsmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sfmsmd::Sfmmd3,
            sfmsmd::Sfmmd3,
            Sfmsmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of the function for stopping prefetch at locations other than on byte boundaries"]
    #[inline(always)]
    pub fn sfmpae(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sfmsmd::Sfmpae,
        sfmsmd::Sfmpae,
        Sfmsmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sfmsmd::Sfmpae,
            sfmsmd::Sfmpae,
            Sfmsmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of the prefetch function"]
    #[inline(always)]
    pub fn sfmpfe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sfmsmd::Sfmpfe,
        sfmsmd::Sfmpfe,
        Sfmsmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sfmsmd::Sfmpfe,
            sfmsmd::Sfmpfe,
            Sfmsmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of the prefetch function"]
    #[inline(always)]
    pub fn sfmse(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        sfmsmd::Sfmse,
        sfmsmd::Sfmse,
        Sfmsmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            sfmsmd::Sfmse,
            sfmsmd::Sfmse,
            Sfmsmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial interface read mode selection"]
    #[inline(always)]
    pub fn sfmrm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        sfmsmd::Sfmrm,
        sfmsmd::Sfmrm,
        Sfmsmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            sfmsmd::Sfmrm,
            sfmsmd::Sfmrm,
            Sfmsmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sfmsmd {
    #[inline(always)]
    fn default() -> Sfmsmd {
        <crate::RegValueT<Sfmsmd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sfmsmd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmcce_SPEC;
    pub type Sfmcce = crate::EnumBitfieldStruct<u8, Sfmcce_SPEC>;
    impl Sfmcce {
        #[doc = "Default instruction code set for each instruction"]
        pub const _0: Self = Self::new(0);

        #[doc = "Instruction code written in the SFMSIC register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmosw_SPEC;
    pub type Sfmosw = crate::EnumBitfieldStruct<u8, Sfmosw_SPEC>;
    impl Sfmosw {
        #[doc = "Does not extend the low-level width of SCK at transmission time"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extends the low-level width of SCK by 1*PCLKA at transmission time"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmohw_SPEC;
    pub type Sfmohw = crate::EnumBitfieldStruct<u8, Sfmohw_SPEC>;
    impl Sfmohw {
        #[doc = "Does not extend the high-level width of SCK at transmission time"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extends the high-level width of SCK by 1*PCLKA at transmission time"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmoex_SPEC;
    pub type Sfmoex = crate::EnumBitfieldStruct<u8, Sfmoex_SPEC>;
    impl Sfmoex {
        #[doc = "Does not extend the output enable signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extends the output enable signal by 1*QSPCLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmmd3_SPEC;
    pub type Sfmmd3 = crate::EnumBitfieldStruct<u8, Sfmmd3_SPEC>;
    impl Sfmmd3 {
        #[doc = "SPI mode 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "SPI mode 3"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmpae_SPEC;
    pub type Sfmpae = crate::EnumBitfieldStruct<u8, Sfmpae_SPEC>;
    impl Sfmpae {
        #[doc = "Disables prefetch stopping at locations other than on byte boundaries"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables prefetch stopping at locations other than on byte boundaries"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmpfe_SPEC;
    pub type Sfmpfe = crate::EnumBitfieldStruct<u8, Sfmpfe_SPEC>;
    impl Sfmpfe {
        #[doc = "Disables prefetch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables prefetch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmse_SPEC;
    pub type Sfmse = crate::EnumBitfieldStruct<u8, Sfmse_SPEC>;
    impl Sfmse {
        #[doc = "Does not extend QSSL"]
        pub const _00: Self = Self::new(0);

        #[doc = "Extends QSSL by 33*QSPCLK"]
        pub const _01: Self = Self::new(1);

        #[doc = "Extends QSSL by 129*QSPCLK"]
        pub const _10: Self = Self::new(2);

        #[doc = "Extends QSSL infinitely"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmrm_SPEC;
    pub type Sfmrm = crate::EnumBitfieldStruct<u8, Sfmrm_SPEC>;
    impl Sfmrm {
        #[doc = "Standard Read"]
        pub const _000: Self = Self::new(0);

        #[doc = "Fast Read"]
        pub const _001: Self = Self::new(1);

        #[doc = "Fast Read Dual Output"]
        pub const _010: Self = Self::new(2);

        #[doc = "Fast Read Dual I/O"]
        pub const _011: Self = Self::new(3);

        #[doc = "Fast Read Quad Output"]
        pub const _100: Self = Self::new(4);

        #[doc = "Fast Read Quad I/O"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfmssc_SPEC;
impl crate::sealed::RegSpec for Sfmssc_SPEC {
    type DataType = u32;
}

#[doc = "Chip Selection Control Register"]
pub type Sfmssc = crate::RegValueT<Sfmssc_SPEC>;

impl Sfmssc {
    #[doc = "QSSL signal output timing selection"]
    #[inline(always)]
    pub fn sfmsld(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sfmssc::Sfmsld,
        sfmssc::Sfmsld,
        Sfmssc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sfmssc::Sfmsld,
            sfmssc::Sfmsld,
            Sfmssc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "QSSL signal release timing selection"]
    #[inline(always)]
    pub fn sfmshd(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sfmssc::Sfmshd,
        sfmssc::Sfmshd,
        Sfmssc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sfmssc::Sfmshd,
            sfmssc::Sfmshd,
            Sfmssc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of a minimum high-level width of the QSSL signal"]
    #[inline(always)]
    pub fn sfmsw(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        sfmssc::Sfmsw,
        sfmssc::Sfmsw,
        Sfmssc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            sfmssc::Sfmsw,
            sfmssc::Sfmsw,
            Sfmssc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sfmssc {
    #[inline(always)]
    fn default() -> Sfmssc {
        <crate::RegValueT<Sfmssc_SPEC> as RegisterValue<_>>::new(55)
    }
}
pub mod sfmssc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmsld_SPEC;
    pub type Sfmsld = crate::EnumBitfieldStruct<u8, Sfmsld_SPEC>;
    impl Sfmsld {
        #[doc = "Outputs QSSL 0.5*SCK before the first rising edge of QSPCLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "Outputs QSSL 1.5*SCK before the first rising edge of QSPCLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmshd_SPEC;
    pub type Sfmshd = crate::EnumBitfieldStruct<u8, Sfmshd_SPEC>;
    impl Sfmshd {
        #[doc = "Releases QSSL 0.5*SCK after the last rising edge of QSPCLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "Releases QSSL 1.5*SCK after the last rising edge of QSPCLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmsw_SPEC;
    pub type Sfmsw = crate::EnumBitfieldStruct<u8, Sfmsw_SPEC>;
    impl Sfmsw {
        #[doc = "1 x QSPCLK"]
        pub const _0000: Self = Self::new(0);

        #[doc = "2 x QSPCLK"]
        pub const _0001: Self = Self::new(1);

        #[doc = "3 x QSPCLK"]
        pub const _0010: Self = Self::new(2);

        #[doc = "4 x QSPCLK"]
        pub const _0011: Self = Self::new(3);

        #[doc = "5 x QSPCLK"]
        pub const _0100: Self = Self::new(4);

        #[doc = "6 x QSPCLK"]
        pub const _0101: Self = Self::new(5);

        #[doc = "7 x QSPCLK"]
        pub const _0110: Self = Self::new(6);

        #[doc = "8 x QSPCLK"]
        pub const _0111: Self = Self::new(7);

        #[doc = "9 x QSPCLK"]
        pub const _1000: Self = Self::new(8);

        #[doc = "10 x QSPCLK"]
        pub const _1001: Self = Self::new(9);

        #[doc = "11 x QSPCLK"]
        pub const _1010: Self = Self::new(10);

        #[doc = "12 x QSPCLK"]
        pub const _1011: Self = Self::new(11);

        #[doc = "13 x QSPCLK"]
        pub const _1100: Self = Self::new(12);

        #[doc = "14 x QSPCLK"]
        pub const _1101: Self = Self::new(13);

        #[doc = "15 x QSPCLK"]
        pub const _1110: Self = Self::new(14);

        #[doc = "16 x QSPCLK"]
        pub const _1111: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfmskc_SPEC;
impl crate::sealed::RegSpec for Sfmskc_SPEC {
    type DataType = u32;
}

#[doc = "Clock Control Register"]
pub type Sfmskc = crate::RegValueT<Sfmskc_SPEC>;

impl Sfmskc {
    #[doc = "Selection of a duty ratio correction function for the SCK signal"]
    #[inline(always)]
    pub fn sfmdty(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sfmskc::Sfmdty,
        sfmskc::Sfmdty,
        Sfmskc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sfmskc::Sfmdty,
            sfmskc::Sfmdty,
            Sfmskc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial interface reference cycle selection (* Pay attention to the irregularity.)NOTE: When PCLKA multiplied by an odd number is selected, the high-level width of the SCK signal is longer than the low-level width by 1 x PCLKA before duty ratio correction."]
    #[inline(always)]
    pub fn sfmdv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        sfmskc::Sfmdv,
        sfmskc::Sfmdv,
        Sfmskc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            sfmskc::Sfmdv,
            sfmskc::Sfmdv,
            Sfmskc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sfmskc {
    #[inline(always)]
    fn default() -> Sfmskc {
        <crate::RegValueT<Sfmskc_SPEC> as RegisterValue<_>>::new(8)
    }
}
pub mod sfmskc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmdty_SPEC;
    pub type Sfmdty = crate::EnumBitfieldStruct<u8, Sfmdty_SPEC>;
    impl Sfmdty {
        #[doc = "Serial interface reference cycle selection (* Pay attention to the irregularity.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Delays the rising of the SCK signal by 0.5*PCLKA.(* Valid with PCLKA multiplied by an odd number)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmdv_SPEC;
    pub type Sfmdv = crate::EnumBitfieldStruct<u8, Sfmdv_SPEC>;
    impl Sfmdv {
        #[doc = "18 x PCLKA"]
        pub const _10000: Self = Self::new(16);

        #[doc = "20 x PCLKA"]
        pub const _10001: Self = Self::new(17);

        #[doc = "22 x PCLKA"]
        pub const _10010: Self = Self::new(18);

        #[doc = "24 x PCLKA"]
        pub const _10011: Self = Self::new(19);

        #[doc = "26 x PCLKA"]
        pub const _10100: Self = Self::new(20);

        #[doc = "28 x PCLKA"]
        pub const _10101: Self = Self::new(21);

        #[doc = "30 x PCLKA"]
        pub const _10110: Self = Self::new(22);

        #[doc = "32 x PCLKA"]
        pub const _10111: Self = Self::new(23);

        #[doc = "34 x PCLKA"]
        pub const _11000: Self = Self::new(24);

        #[doc = "36 x PCLKA"]
        pub const _11001: Self = Self::new(25);

        #[doc = "38 x PCLKA"]
        pub const _11010: Self = Self::new(26);

        #[doc = "40 x PCLKA"]
        pub const _11011: Self = Self::new(27);

        #[doc = "42 x PCLKA"]
        pub const _11100: Self = Self::new(28);

        #[doc = "44 x PCLKA"]
        pub const _11101: Self = Self::new(29);

        #[doc = "46 x PCLKA"]
        pub const _11110: Self = Self::new(30);

        #[doc = "48 x PCLKA"]
        pub const _11111: Self = Self::new(31);

        #[doc = "( SFMDV + 2 ) x PCLKA"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfmsst_SPEC;
impl crate::sealed::RegSpec for Sfmsst_SPEC {
    type DataType = u32;
}

#[doc = "Status Register"]
pub type Sfmsst = crate::RegValueT<Sfmsst_SPEC>;

impl Sfmsst {
    #[doc = "Prefetch function operation state"]
    #[inline(always)]
    pub fn pfoff(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sfmsst::Pfoff,
        sfmsst::Pfoff,
        Sfmsst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sfmsst::Pfoff,
            sfmsst::Pfoff,
            Sfmsst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Prefetch buffer state"]
    #[inline(always)]
    pub fn pfful(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sfmsst::Pfful,
        sfmsst::Pfful,
        Sfmsst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sfmsst::Pfful,
            sfmsst::Pfful,
            Sfmsst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Number of bytes of prefetched dataRange: 00000 - 10010  (No combination other than the above is available.)"]
    #[inline(always)]
    pub fn pfcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        sfmsst::Pfcnt,
        sfmsst::Pfcnt,
        Sfmsst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            sfmsst::Pfcnt,
            sfmsst::Pfcnt,
            Sfmsst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sfmsst {
    #[inline(always)]
    fn default() -> Sfmsst {
        <crate::RegValueT<Sfmsst_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod sfmsst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfoff_SPEC;
    pub type Pfoff = crate::EnumBitfieldStruct<u8, Pfoff_SPEC>;
    impl Pfoff {
        #[doc = "The prefetch function is operating."]
        pub const _0: Self = Self::new(0);

        #[doc = "The prefetch function is not enabled or is not operating."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfful_SPEC;
    pub type Pfful = crate::EnumBitfieldStruct<u8, Pfful_SPEC>;
    impl Pfful {
        #[doc = "The prefetch buffer has a free space."]
        pub const _0: Self = Self::new(0);

        #[doc = "The prefetch buffer is full."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcnt_SPEC;
    pub type Pfcnt = crate::EnumBitfieldStruct<u8, Pfcnt_SPEC>;
    impl Pfcnt {
        #[doc = "Nodata has been prefetched."]
        pub const _00000: Self = Self::new(0);

        #[doc = "Data of (PFCNT) bytes hs been prefetched."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfmcom_SPEC;
impl crate::sealed::RegSpec for Sfmcom_SPEC {
    type DataType = u32;
}

#[doc = "Communication Port Register"]
pub type Sfmcom = crate::RegValueT<Sfmcom_SPEC>;

impl Sfmcom {
    #[doc = "Port for direct communication with the SPI bus.Input/output to and from this port is converted to an SPI bus cycle. This port is accessible in the direct communication mode (DCOM=1) only.Access to this port is ignored in the ROM access mode."]
    #[inline(always)]
    pub fn sfmd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Sfmcom_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Sfmcom_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sfmcom {
    #[inline(always)]
    fn default() -> Sfmcom {
        <crate::RegValueT<Sfmcom_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfmcmd_SPEC;
impl crate::sealed::RegSpec for Sfmcmd_SPEC {
    type DataType = u32;
}

#[doc = "Communication Mode Control Register"]
pub type Sfmcmd = crate::RegValueT<Sfmcmd_SPEC>;

impl Sfmcmd {
    #[doc = "Selection of a mode of communication with the SPI bus"]
    #[inline(always)]
    pub fn dcom(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sfmcmd::Dcom,
        sfmcmd::Dcom,
        Sfmcmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sfmcmd::Dcom,
            sfmcmd::Dcom,
            Sfmcmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sfmcmd {
    #[inline(always)]
    fn default() -> Sfmcmd {
        <crate::RegValueT<Sfmcmd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sfmcmd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcom_SPEC;
    pub type Dcom = crate::EnumBitfieldStruct<u8, Dcom_SPEC>;
    impl Dcom {
        #[doc = "ROM access mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Direct communication mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfmcst_SPEC;
impl crate::sealed::RegSpec for Sfmcst_SPEC {
    type DataType = u32;
}

#[doc = "Communication Status Register"]
pub type Sfmcst = crate::RegValueT<Sfmcst_SPEC>;

impl Sfmcst {
    #[doc = "Status of ROM access detection in the direct communication modeNOTE: Writing of 0 only is possible. Writing of 1 is ignored."]
    #[inline(always)]
    pub fn eromr(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sfmcst::Eromr,
        sfmcst::Eromr,
        Sfmcst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sfmcst::Eromr,
            sfmcst::Eromr,
            Sfmcst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SPI bus cycle completion state in direct communication"]
    #[inline(always)]
    pub fn combsy(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sfmcst::Combsy,
        sfmcst::Combsy,
        Sfmcst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sfmcst::Combsy,
            sfmcst::Combsy,
            Sfmcst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sfmcst {
    #[inline(always)]
    fn default() -> Sfmcst {
        <crate::RegValueT<Sfmcst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sfmcst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eromr_SPEC;
    pub type Eromr = crate::EnumBitfieldStruct<u8, Eromr_SPEC>;
    impl Eromr {
        #[doc = "ROM access is not detected in direct communication mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "ROM access is detected in direct communication mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Combsy_SPEC;
    pub type Combsy = crate::EnumBitfieldStruct<u8, Combsy_SPEC>;
    impl Combsy {
        #[doc = "There is no serial transfer being processed."]
        pub const _0: Self = Self::new(0);

        #[doc = "There is a serial transfer being processed."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfmsic_SPEC;
impl crate::sealed::RegSpec for Sfmsic_SPEC {
    type DataType = u32;
}

#[doc = "Instruction Code Register"]
pub type Sfmsic = crate::RegValueT<Sfmsic_SPEC>;

impl Sfmsic {
    #[doc = "Serial ROM instruction code to substitute"]
    #[inline(always)]
    pub fn sfmcic(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Sfmsic_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Sfmsic_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sfmsic {
    #[inline(always)]
    fn default() -> Sfmsic {
        <crate::RegValueT<Sfmsic_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfmsac_SPEC;
impl crate::sealed::RegSpec for Sfmsac_SPEC {
    type DataType = u32;
}

#[doc = "Address Mode Control Register"]
pub type Sfmsac = crate::RegValueT<Sfmsac_SPEC>;

impl Sfmsac {
    #[doc = "Selection of a default instruction code, when Serial Interface address width is selected 4 bytes."]
    #[inline(always)]
    pub fn sfm4bc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sfmsac::Sfm4Bc,
        sfmsac::Sfm4Bc,
        Sfmsac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sfmsac::Sfm4Bc,
            sfmsac::Sfm4Bc,
            Sfmsac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection the number of address bits of the serial interface"]
    #[inline(always)]
    pub fn sfmas(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        sfmsac::Sfmas,
        sfmsac::Sfmas,
        Sfmsac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            sfmsac::Sfmas,
            sfmsac::Sfmas,
            Sfmsac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sfmsac {
    #[inline(always)]
    fn default() -> Sfmsac {
        <crate::RegValueT<Sfmsac_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod sfmsac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfm4Bc_SPEC;
    pub type Sfm4Bc = crate::EnumBitfieldStruct<u8, Sfm4Bc_SPEC>;
    impl Sfm4Bc {
        #[doc = "Does not use 4 Byte address read Instruction code"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use 4 Byte address read Instruction code"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmas_SPEC;
    pub type Sfmas = crate::EnumBitfieldStruct<u8, Sfmas_SPEC>;
    impl Sfmas {
        #[doc = "1byte"]
        pub const _00: Self = Self::new(0);

        #[doc = "2bytes"]
        pub const _01: Self = Self::new(1);

        #[doc = "3bytes"]
        pub const _10: Self = Self::new(2);

        #[doc = "4 bytes"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfmsdc_SPEC;
impl crate::sealed::RegSpec for Sfmsdc_SPEC {
    type DataType = u32;
}

#[doc = "Dummy Cycle Control Register"]
pub type Sfmsdc = crate::RegValueT<Sfmsdc_SPEC>;

impl Sfmsdc {
    #[doc = "Mode data for serial ROM. (Control XIP mode)"]
    #[inline(always)]
    pub fn sfmxd(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        sfmsdc::Sfmxd,
        sfmsdc::Sfmxd,
        Sfmsdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            sfmsdc::Sfmxd,
            sfmsdc::Sfmxd,
            Sfmsdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "XIP mode permission"]
    #[inline(always)]
    pub fn sfmxen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sfmsdc::Sfmxen,
        sfmsdc::Sfmxen,
        Sfmsdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sfmsdc::Sfmxen,
            sfmsdc::Sfmxen,
            Sfmsdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "XIP mode status"]
    #[inline(always)]
    pub fn sfmxst(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sfmsdc::Sfmxst,
        sfmsdc::Sfmxst,
        Sfmsdc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sfmsdc::Sfmxst,
            sfmsdc::Sfmxst,
            Sfmsdc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Selection of the number of dummy cycles of Fast Read instructions"]
    #[inline(always)]
    pub fn sfmdn(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        sfmsdc::Sfmdn,
        sfmsdc::Sfmdn,
        Sfmsdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            sfmsdc::Sfmdn,
            sfmsdc::Sfmdn,
            Sfmsdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sfmsdc {
    #[inline(always)]
    fn default() -> Sfmsdc {
        <crate::RegValueT<Sfmsdc_SPEC> as RegisterValue<_>>::new(65280)
    }
}
pub mod sfmsdc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmxd_SPEC;
    pub type Sfmxd = crate::EnumBitfieldStruct<u8, Sfmxd_SPEC>;
    impl Sfmxd {
        #[doc = "XIP mode is prohibited"]
        pub const _0: Self = Self::new(0);

        #[doc = "XIP mode is permitted"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmxen_SPEC;
    pub type Sfmxen = crate::EnumBitfieldStruct<u8, Sfmxen_SPEC>;
    impl Sfmxen {
        #[doc = "XIP mode is prohibited"]
        pub const _0: Self = Self::new(0);

        #[doc = "XIP mode is permitted"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmxst_SPEC;
    pub type Sfmxst = crate::EnumBitfieldStruct<u8, Sfmxst_SPEC>;
    impl Sfmxst {
        #[doc = "Normal (non-XIP) mode is operating"]
        pub const _0: Self = Self::new(0);

        #[doc = "XIP mode is operating"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmdn_SPEC;
    pub type Sfmdn = crate::EnumBitfieldStruct<u8, Sfmdn_SPEC>;
    impl Sfmdn {
        #[doc = "Default dummy cycles of each instruction."]
        pub const _0000: Self = Self::new(0);

        #[doc = "( SFMDN + 2 ) x SCK"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfmspc_SPEC;
impl crate::sealed::RegSpec for Sfmspc_SPEC {
    type DataType = u32;
}

#[doc = "SPI Protocol Control Register"]
pub type Sfmspc = crate::RegValueT<Sfmspc_SPEC>;

impl Sfmspc {
    #[doc = "Selection of the minimum time of input output switch, when Dual SPI protocol or Quad SPI protocol is selected."]
    #[inline(always)]
    pub fn sfmsde(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sfmspc::Sfmsde,
        sfmspc::Sfmsde,
        Sfmspc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sfmspc::Sfmsde,
            sfmspc::Sfmsde,
            Sfmspc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of SPI protocolNOTE: Serial ROM\'s SPI protocol is required to be set by software separately."]
    #[inline(always)]
    pub fn sfmspi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        sfmspc::Sfmspi,
        sfmspc::Sfmspi,
        Sfmspc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            sfmspc::Sfmspi,
            sfmspc::Sfmspi,
            Sfmspc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sfmspc {
    #[inline(always)]
    fn default() -> Sfmspc {
        <crate::RegValueT<Sfmspc_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod sfmspc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmsde_SPEC;
    pub type Sfmsde = crate::EnumBitfieldStruct<u8, Sfmsde_SPEC>;
    impl Sfmsde {
        #[doc = "Does not allocate minimum switch time"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allocate the minimum switch time equivalent to 1*QSPXLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmspi_SPEC;
    pub type Sfmspi = crate::EnumBitfieldStruct<u8, Sfmspi_SPEC>;
    impl Sfmspi {
        #[doc = "Extended SPI protocol"]
        pub const _00: Self = Self::new(0);

        #[doc = "Dual SPI protocol"]
        pub const _01: Self = Self::new(1);

        #[doc = "Quad SPI protocol"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfmpmd_SPEC;
impl crate::sealed::RegSpec for Sfmpmd_SPEC {
    type DataType = u32;
}

#[doc = "Port Control Register"]
pub type Sfmpmd = crate::RegValueT<Sfmpmd_SPEC>;

impl Sfmpmd {
    #[doc = "Specify level of WP pin"]
    #[inline(always)]
    pub fn sfmwpl(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sfmpmd::Sfmwpl,
        sfmpmd::Sfmwpl,
        Sfmpmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sfmpmd::Sfmwpl,
            sfmpmd::Sfmwpl,
            Sfmpmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sfmpmd {
    #[inline(always)]
    fn default() -> Sfmpmd {
        <crate::RegValueT<Sfmpmd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sfmpmd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmwpl_SPEC;
    pub type Sfmwpl = crate::EnumBitfieldStruct<u8, Sfmwpl_SPEC>;
    impl Sfmwpl {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfmcnt1_SPEC;
impl crate::sealed::RegSpec for Sfmcnt1_SPEC {
    type DataType = u32;
}

#[doc = "External QSPI Address Register 1"]
pub type Sfmcnt1 = crate::RegValueT<Sfmcnt1_SPEC>;

impl Sfmcnt1 {
    #[doc = "BANK Switching AddressWhen accessing from 0x6000_0000 to 0x63FF_FFFF, Addres bus is Set QSPI_EXT\\[5:0\\] to high-order 6bits of SHADDR\\[31:0\\]NOTE: Setting 6\'h3F is prihibited."]
    #[inline(always)]
    pub fn qspi_ext(
        self,
    ) -> crate::common::RegisterField<26, 0x3f, 1, 0, u8, u8, Sfmcnt1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3f,1,0,u8,u8,Sfmcnt1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sfmcnt1 {
    #[inline(always)]
    fn default() -> Sfmcnt1 {
        <crate::RegValueT<Sfmcnt1_SPEC> as RegisterValue<_>>::new(0)
    }
}
