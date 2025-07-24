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
// Generated from SVD 1.20.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:50:30 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Quad Serial Peripheral Interface"]
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

    #[doc = "External QSPI Address Register"]
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
    #[doc = "Serial interface read mode select"]
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

    #[doc = "QSSL extension function select after SPI bus access"]
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

    #[doc = "Prefetch function select"]
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

    #[doc = "Function select for stopping prefetch at locations other than on byte boundaries"]
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

    #[doc = "SPI mode select."]
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

    #[doc = "Extension select for the I/O buffer output enable signal for the serial interface"]
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

    #[doc = "Read instruction code select"]
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
}
impl ::core::default::Default for Sfmsmd {
    #[inline(always)]
    fn default() -> Sfmsmd {
        <crate::RegValueT<Sfmsmd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sfmsmd {

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
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmse_SPEC;
    pub type Sfmse = crate::EnumBitfieldStruct<u8, Sfmse_SPEC>;
    impl Sfmse {
        #[doc = "Do not extend QSSL"]
        pub const _00: Self = Self::new(0);

        #[doc = "Extend QSSL by 33 QSPCLK"]
        pub const _01: Self = Self::new(1);

        #[doc = "Extend QSSL by 129 QSPCLK"]
        pub const _10: Self = Self::new(2);

        #[doc = "Extend QSSL infinitely"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmpfe_SPEC;
    pub type Sfmpfe = crate::EnumBitfieldStruct<u8, Sfmpfe_SPEC>;
    impl Sfmpfe {
        #[doc = "Disable function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable function"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmpae_SPEC;
    pub type Sfmpae = crate::EnumBitfieldStruct<u8, Sfmpae_SPEC>;
    impl Sfmpae {
        #[doc = "Disable function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable function"]
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
    pub struct Sfmoex_SPEC;
    pub type Sfmoex = crate::EnumBitfieldStruct<u8, Sfmoex_SPEC>;
    impl Sfmoex {
        #[doc = "Do not extend"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extend by 1 QSPCLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmohw_SPEC;
    pub type Sfmohw = crate::EnumBitfieldStruct<u8, Sfmohw_SPEC>;
    impl Sfmohw {
        #[doc = "Do not extend high-level width of QSPCLK during transmission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extend high-level width of QSPCLK by 1 PCLKA during transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmosw_SPEC;
    pub type Sfmosw = crate::EnumBitfieldStruct<u8, Sfmosw_SPEC>;
    impl Sfmosw {
        #[doc = "Do not extend low-level width of QSPCLK during transmission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extend low-level width of QSPCLK by 1 PCLKA during transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmcce_SPEC;
    pub type Sfmcce = crate::EnumBitfieldStruct<u8, Sfmcce_SPEC>;
    impl Sfmcce {
        #[doc = "Uses automatically generated SPI instruction code"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use instruction code in the SFMSIC register"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Minimum High-level Width Select for QSSL Signal"]
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

    #[doc = "QSSL Signal Hold Time"]
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

    #[doc = "QSSL Signal Setup Time"]
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
}
impl ::core::default::Default for Sfmssc {
    #[inline(always)]
    fn default() -> Sfmssc {
        <crate::RegValueT<Sfmssc_SPEC> as RegisterValue<_>>::new(55)
    }
}
pub mod sfmssc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmsw_SPEC;
    pub type Sfmsw = crate::EnumBitfieldStruct<u8, Sfmsw_SPEC>;
    impl Sfmsw {
        #[doc = "1 QSPCLK"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "2 QSPCLK"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "3 QSPCLK"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "4 QSPCLK"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "5 QSPCLK"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "6 QSPCLK"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "7 QSPCLK"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "8 QSPCLK"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "9 QSPCLK"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "10 QSPCLK"]
        pub const _0_X_9: Self = Self::new(9);

        #[doc = "11 QSPCLK"]
        pub const _0_X_A: Self = Self::new(10);

        #[doc = "12 QSPCLK"]
        pub const _0_X_B: Self = Self::new(11);

        #[doc = "13 QSPCLK"]
        pub const _0_X_C: Self = Self::new(12);

        #[doc = "14 QSPCLK"]
        pub const _0_X_D: Self = Self::new(13);

        #[doc = "15 QSPCLK"]
        pub const _0_X_E: Self = Self::new(14);

        #[doc = "16 QSPCLK"]
        pub const _0_X_F: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmshd_SPEC;
    pub type Sfmshd = crate::EnumBitfieldStruct<u8, Sfmshd_SPEC>;
    impl Sfmshd {
        #[doc = "QSSL outputs high after 0.5 QSPCLK cycles from the last rising edge of QSPCLK."]
        pub const _0: Self = Self::new(0);

        #[doc = "QSSL outputs high after 1.5 QSPCLK cycles from the last rising edge of QSPCLK."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmsld_SPEC;
    pub type Sfmsld = crate::EnumBitfieldStruct<u8, Sfmsld_SPEC>;
    impl Sfmsld {
        #[doc = "QSSL outputs low before 0.5 QSPCLK cycles from the first rising edge of QSPCLK."]
        pub const _0: Self = Self::new(0);

        #[doc = "QSSL outputs low before 1.5 QSPCLK cycles from the first rising edge of QSPCLK."]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Serial interface reference cycle select. (Pay attention to irregularities.)"]
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

    #[doc = "Duty ratio correction function select for the QSPCLK signal when divided by an odd number"]
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
}
impl ::core::default::Default for Sfmskc {
    #[inline(always)]
    fn default() -> Sfmskc {
        <crate::RegValueT<Sfmskc_SPEC> as RegisterValue<_>>::new(8)
    }
}
pub mod sfmskc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmdv_SPEC;
    pub type Sfmdv = crate::EnumBitfieldStruct<u8, Sfmdv_SPEC>;
    impl Sfmdv {
        #[doc = "2 PCLKA"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "3 PCLKA (divided by an odd number)"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "4 PCLKA"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "5 PCLKA (divided by an odd number)"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "6 PCLKA"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "7 PCLKA (divided by an odd number)"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "8 PCLKA"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "9 PCLKA (divided by an odd number)"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "10 PCLKA"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "11 PCLKA (divided by an odd number)"]
        pub const _0_X_09: Self = Self::new(9);

        #[doc = "12 PCLKA"]
        pub const _0_X_0_A: Self = Self::new(10);

        #[doc = "13 PCLKA (divided by an odd number)"]
        pub const _0_X_0_B: Self = Self::new(11);

        #[doc = "14 PCLKA"]
        pub const _0_X_0_C: Self = Self::new(12);

        #[doc = "15 PCLKA (divided by an odd number)"]
        pub const _0_X_0_D: Self = Self::new(13);

        #[doc = "16 PCLKA"]
        pub const _0_X_0_E: Self = Self::new(14);

        #[doc = "17 PCLKA (divided by an odd number)"]
        pub const _0_X_0_F: Self = Self::new(15);

        #[doc = "18 PCLKA"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "20 PCLKA"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "22 PCLKA"]
        pub const _0_X_12: Self = Self::new(18);

        #[doc = "24 PCLKA"]
        pub const _0_X_13: Self = Self::new(19);

        #[doc = "26 PCLKA"]
        pub const _0_X_14: Self = Self::new(20);

        #[doc = "28 PCLKA"]
        pub const _0_X_15: Self = Self::new(21);

        #[doc = "30 PCLKA"]
        pub const _0_X_16: Self = Self::new(22);

        #[doc = "32 PCLKA"]
        pub const _0_X_17: Self = Self::new(23);

        #[doc = "34 PCLKA"]
        pub const _0_X_18: Self = Self::new(24);

        #[doc = "36 PCLKA"]
        pub const _0_X_19: Self = Self::new(25);

        #[doc = "38 PCLKA"]
        pub const _0_X_1_A: Self = Self::new(26);

        #[doc = "40 PCLKA"]
        pub const _0_X_1_B: Self = Self::new(27);

        #[doc = "42 PCLKA"]
        pub const _0_X_1_C: Self = Self::new(28);

        #[doc = "44 PCLKA"]
        pub const _0_X_1_D: Self = Self::new(29);

        #[doc = "46 PCLKA"]
        pub const _0_X_1_E: Self = Self::new(30);

        #[doc = "48 PCLKA"]
        pub const _0_X_1_F: Self = Self::new(31);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmdty_SPEC;
    pub type Sfmdty = crate::EnumBitfieldStruct<u8, Sfmdty_SPEC>;
    impl Sfmdty {
        #[doc = "Make no correction"]
        pub const _0: Self = Self::new(0);

        #[doc = "Make correction"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Number of bytes of prefetched data"]
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

    #[doc = "Prefetch function operating state"]
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
}
impl ::core::default::Default for Sfmsst {
    #[inline(always)]
    fn default() -> Sfmsst {
        <crate::RegValueT<Sfmsst_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod sfmsst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcnt_SPEC;
    pub type Pfcnt = crate::EnumBitfieldStruct<u8, Pfcnt_SPEC>;
    impl Pfcnt {
        #[doc = "0 byte"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "1 byte"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "2 bytes"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "3 bytes"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "4 bytes"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "5 bytes"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "6 bytes"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "7 bytes"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "8 bytes"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "9 bytes"]
        pub const _0_X_09: Self = Self::new(9);

        #[doc = "10 bytes"]
        pub const _0_X_0_A: Self = Self::new(10);

        #[doc = "11 bytes"]
        pub const _0_X_0_B: Self = Self::new(11);

        #[doc = "12 bytes"]
        pub const _0_X_0_C: Self = Self::new(12);

        #[doc = "13 bytes"]
        pub const _0_X_0_D: Self = Self::new(13);

        #[doc = "14 bytes"]
        pub const _0_X_0_E: Self = Self::new(14);

        #[doc = "15 bytes"]
        pub const _0_X_0_F: Self = Self::new(15);

        #[doc = "16 bytes"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "17 bytes"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "18 bytes"]
        pub const _0_X_12: Self = Self::new(18);

        #[doc = "Reserved"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfful_SPEC;
    pub type Pfful = crate::EnumBitfieldStruct<u8, Pfful_SPEC>;
    impl Pfful {
        #[doc = "Prefetch buffer has free space"]
        pub const _0: Self = Self::new(0);

        #[doc = "Prefetch buffer is full"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfoff_SPEC;
    pub type Pfoff = crate::EnumBitfieldStruct<u8, Pfoff_SPEC>;
    impl Pfoff {
        #[doc = "Prefetch function operating"]
        pub const _0: Self = Self::new(0);

        #[doc = "Prefetch function not enabled or not operating"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Port for direct communication with the SPI bus"]
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
    #[doc = "Mode select for communication with the SPI bus"]
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

    #[doc = "ROM access detection status in direct communication mode"]
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
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sfmcst::Eromr,
            sfmcst::Eromr,
            Sfmcst_SPEC,
            crate::common::RW,
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
    pub struct Combsy_SPEC;
    pub type Combsy = crate::EnumBitfieldStruct<u8, Combsy_SPEC>;
    impl Combsy {
        #[doc = "No serial transfer being processed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Serial transfer being processed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eromr_SPEC;
    pub type Eromr = crate::EnumBitfieldStruct<u8, Eromr_SPEC>;
    impl Eromr {
        #[doc = "ROM access not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "ROM access detected"]
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
    #[doc = "Serial flash instruction code to substitute"]
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
    #[doc = "Number of address bytes select for the serial interface"]
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

    #[doc = "Selection of instruction code automatically generated when the serial interface address width is 4 bytes"]
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
}
impl ::core::default::Default for Sfmsac {
    #[inline(always)]
    fn default() -> Sfmsac {
        <crate::RegValueT<Sfmsac_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod sfmsac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmas_SPEC;
    pub type Sfmas = crate::EnumBitfieldStruct<u8, Sfmas_SPEC>;
    impl Sfmas {
        #[doc = "1 byte"]
        pub const _00: Self = Self::new(0);

        #[doc = "2 bytes"]
        pub const _01: Self = Self::new(1);

        #[doc = "3 bytes"]
        pub const _10: Self = Self::new(2);

        #[doc = "4 bytes"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfm4Bc_SPEC;
    pub type Sfm4Bc = crate::EnumBitfieldStruct<u8, Sfm4Bc_SPEC>;
    impl Sfm4Bc {
        #[doc = "Do not use 4-byte address read instruction code"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use 4-byte address read instruction code"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Number of dummy cycles select for Fast Read instructions"]
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

    #[doc = "Mode data for serial flash (Controls XIP mode.)"]
    #[inline(always)]
    pub fn sfmxd(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Sfmsdc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Sfmsdc_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Sfmdn_SPEC;
    pub type Sfmdn = crate::EnumBitfieldStruct<u8, Sfmdn_SPEC>;
    impl Sfmdn {
        #[doc = "Default dummy cycles for each instruction: - Fast Read Quad I/O: 6 QSPCLK - Fast Read Quad Output: 8 QSPCLK - Fast Read Dual I/O: 4 QSPCLK - Fast Read Dual Output: 8 QSPCLK - Fast Read: 8 QSPCLK"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "3 QSPCLK"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "4 QSPCLK"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "5 QSPCLK"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "6 QSPCLK"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "7 QSPCLK"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "8 QSPCLK"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "9 QSPCLK"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "10 QSPCLK"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "11 QSPCLK"]
        pub const _0_X_9: Self = Self::new(9);

        #[doc = "12 QSPCLK"]
        pub const _0_X_A: Self = Self::new(10);

        #[doc = "13 QSPCLK"]
        pub const _0_X_B: Self = Self::new(11);

        #[doc = "14 QSPCLK"]
        pub const _0_X_C: Self = Self::new(12);

        #[doc = "15 QSPCLK"]
        pub const _0_X_D: Self = Self::new(13);

        #[doc = "16 QSPCLK"]
        pub const _0_X_E: Self = Self::new(14);

        #[doc = "17 QSPCLK"]
        pub const _0_X_F: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmxst_SPEC;
    pub type Sfmxst = crate::EnumBitfieldStruct<u8, Sfmxst_SPEC>;
    impl Sfmxst {
        #[doc = "Normal (non-XIP) mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "XIP mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmxen_SPEC;
    pub type Sfmxen = crate::EnumBitfieldStruct<u8, Sfmxen_SPEC>;
    impl Sfmxen {
        #[doc = "Prohibit XIP mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Permit XIP mode"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "SPI protocol select"]
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

    #[doc = "QSPCLK extended selection bit when switching I/O of QIOn pin"]
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
}
impl ::core::default::Default for Sfmspc {
    #[inline(always)]
    fn default() -> Sfmspc {
        <crate::RegValueT<Sfmspc_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod sfmspc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmspi_SPEC;
    pub type Sfmspi = crate::EnumBitfieldStruct<u8, Sfmspi_SPEC>;
    impl Sfmspi {
        #[doc = "Single SPI Protocol, Extended SPI protocol"]
        pub const _00: Self = Self::new(0);

        #[doc = "Dual SPI protocol"]
        pub const _01: Self = Self::new(1);

        #[doc = "Quad SPI protocol"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmsde_SPEC;
    pub type Sfmsde = crate::EnumBitfieldStruct<u8, Sfmsde_SPEC>;
    impl Sfmsde {
        #[doc = "No QSPCLK extension"]
        pub const _0: Self = Self::new(0);

        #[doc = "QSPCLK expansion when switching I/O direction of QIOn pin"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "WP pin level specification"]
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

#[doc = "External QSPI Address Register"]
pub type Sfmcnt1 = crate::RegValueT<Sfmcnt1_SPEC>;

impl Sfmcnt1 {
    #[doc = "Bank switching address"]
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
