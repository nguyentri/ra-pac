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
// Generated from SVD 1.00.01, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:40:06 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"General PWM 16-bit Timer 8"]
unsafe impl ::core::marker::Send for super::Gpt168Ns {}
unsafe impl ::core::marker::Sync for super::Gpt168Ns {}
impl super::Gpt168Ns {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "General PWM Timer Software Start Register"]
    #[inline(always)]
    pub const fn gtstr(&self) -> &'static crate::common::Reg<self::Gtstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "General PWM Timer Software Stop Register"]
    #[inline(always)]
    pub const fn gtstp(&self) -> &'static crate::common::Reg<self::Gtstp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtstp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "General PWM Timer Software Clear Register"]
    #[inline(always)]
    pub const fn gtclr(&self) -> &'static crate::common::Reg<self::Gtclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Gtclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "General PWM Timer Start Source Select Register"]
    #[inline(always)]
    pub const fn gtssr(&self) -> &'static crate::common::Reg<self::Gtssr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtssr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "General PWM Timer Stop Source Select Register"]
    #[inline(always)]
    pub const fn gtpsr(&self) -> &'static crate::common::Reg<self::Gtpsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtpsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "General PWM Timer Clear Source Select Register"]
    #[inline(always)]
    pub const fn gtcsr(&self) -> &'static crate::common::Reg<self::Gtcsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtcsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "General PWM Timer Up Count Source Select Register"]
    #[inline(always)]
    pub const fn gtupsr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtupsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtupsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "General PWM Timer Down Count Source Select Register"]
    #[inline(always)]
    pub const fn gtdnsr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdnsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdnsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "General PWM Timer Input Capture Source Select Register A"]
    #[inline(always)]
    pub const fn gticasr(
        &self,
    ) -> &'static crate::common::Reg<self::Gticasr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gticasr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "General PWM Timer Input Capture Source Select Register B"]
    #[inline(always)]
    pub const fn gticbsr(
        &self,
    ) -> &'static crate::common::Reg<self::Gticbsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gticbsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "General PWM Timer Control Register"]
    #[inline(always)]
    pub const fn gtcr(&self) -> &'static crate::common::Reg<self::Gtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "General PWM Timer Interrupt Output Setting Register"]
    #[inline(always)]
    pub const fn gtintad(
        &self,
    ) -> &'static crate::common::Reg<self::Gtintad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtintad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "A/D Conversion Start Request Timing Register A"]
    #[inline(always)]
    pub const fn gtadtra(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadtra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadtra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "A/D Conversion Start Request Timing Buffer Register A"]
    #[inline(always)]
    pub const fn gtadtbra(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadtbra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadtbra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "A/D Conversion Start Request Timing Double-Buffer Register A"]
    #[inline(always)]
    pub const fn gtadtdbra(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadtdbra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadtdbra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "A/D Conversion Start Request Timing Register B"]
    #[inline(always)]
    pub const fn gtadtrb(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadtrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadtrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = "A/D Conversion Start Request Timing Buffer Register B"]
    #[inline(always)]
    pub const fn gtadtbrb(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadtbrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadtbrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "A/D Conversion Start Request Timing Double-Buffer Register B"]
    #[inline(always)]
    pub const fn gtadtdbrb(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadtdbrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadtdbrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "General PWM Timer A/D Conversion Start Request Signal Monitoring Register"]
    #[inline(always)]
    pub const fn gtadsmr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadsmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadsmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "General PWM Timer Inter Channel Logical Operation Function Setting Register"]
    #[inline(always)]
    pub const fn gticlf(
        &self,
    ) -> &'static crate::common::Reg<self::Gticlf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gticlf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtstr_SPEC;
impl crate::sealed::RegSpec for Gtstr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Software Start Register"]
pub type Gtstr = crate::RegValueT<Gtstr_SPEC>;

impl Gtstr {
    #[doc = "Channel n GTCNT Count Start (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstrt0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtstr::Cstrt0,
        gtstr::Cstrt0,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtstr::Cstrt0,
            gtstr::Cstrt0,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Start (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstrt1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtstr::Cstrt1,
        gtstr::Cstrt1,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtstr::Cstrt1,
            gtstr::Cstrt1,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Start (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstrt2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtstr::Cstrt2,
        gtstr::Cstrt2,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtstr::Cstrt2,
            gtstr::Cstrt2,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Start (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstrt3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtstr::Cstrt3,
        gtstr::Cstrt3,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtstr::Cstrt3,
            gtstr::Cstrt3,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Start (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstrt4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtstr::Cstrt4,
        gtstr::Cstrt4,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtstr::Cstrt4,
            gtstr::Cstrt4,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Start (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstrt5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtstr::Cstrt5,
        gtstr::Cstrt5,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtstr::Cstrt5,
            gtstr::Cstrt5,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Start (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstrt6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtstr::Cstrt6,
        gtstr::Cstrt6,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtstr::Cstrt6,
            gtstr::Cstrt6,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Start (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstrt7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtstr::Cstrt7,
        gtstr::Cstrt7,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtstr::Cstrt7,
            gtstr::Cstrt7,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Start (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstrt8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtstr::Cstrt8,
        gtstr::Cstrt8,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtstr::Cstrt8,
            gtstr::Cstrt8,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Start (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstrt9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtstr::Cstrt9,
        gtstr::Cstrt9,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtstr::Cstrt9,
            gtstr::Cstrt9,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtstr {
    #[inline(always)]
    fn default() -> Gtstr {
        <crate::RegValueT<Gtstr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtstr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt0_SPEC;
    pub type Cstrt0 = crate::EnumBitfieldStruct<u8, Cstrt0_SPEC>;
    impl Cstrt0 {
        #[doc = "GTCNT counter not start"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter start"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt1_SPEC;
    pub type Cstrt1 = crate::EnumBitfieldStruct<u8, Cstrt1_SPEC>;
    impl Cstrt1 {
        #[doc = "GTCNT counter not start"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter start"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt2_SPEC;
    pub type Cstrt2 = crate::EnumBitfieldStruct<u8, Cstrt2_SPEC>;
    impl Cstrt2 {
        #[doc = "GTCNT counter not start"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter start"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt3_SPEC;
    pub type Cstrt3 = crate::EnumBitfieldStruct<u8, Cstrt3_SPEC>;
    impl Cstrt3 {
        #[doc = "GTCNT counter not start"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter start"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt4_SPEC;
    pub type Cstrt4 = crate::EnumBitfieldStruct<u8, Cstrt4_SPEC>;
    impl Cstrt4 {
        #[doc = "GTCNT counter not start"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter start"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt5_SPEC;
    pub type Cstrt5 = crate::EnumBitfieldStruct<u8, Cstrt5_SPEC>;
    impl Cstrt5 {
        #[doc = "GTCNT counter not start"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter start"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt6_SPEC;
    pub type Cstrt6 = crate::EnumBitfieldStruct<u8, Cstrt6_SPEC>;
    impl Cstrt6 {
        #[doc = "GTCNT counter not start"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter start"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt7_SPEC;
    pub type Cstrt7 = crate::EnumBitfieldStruct<u8, Cstrt7_SPEC>;
    impl Cstrt7 {
        #[doc = "GTCNT counter not start"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter start"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt8_SPEC;
    pub type Cstrt8 = crate::EnumBitfieldStruct<u8, Cstrt8_SPEC>;
    impl Cstrt8 {
        #[doc = "GTCNT counter not start"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter start"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt9_SPEC;
    pub type Cstrt9 = crate::EnumBitfieldStruct<u8, Cstrt9_SPEC>;
    impl Cstrt9 {
        #[doc = "GTCNT counter not start"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter start"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtstp_SPEC;
impl crate::sealed::RegSpec for Gtstp_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Software Stop Register"]
pub type Gtstp = crate::RegValueT<Gtstp_SPEC>;

impl Gtstp {
    #[doc = "Channel n GTCNT Count Stop (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstop0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtstp::Cstop0,
        gtstp::Cstop0,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtstp::Cstop0,
            gtstp::Cstop0,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Stop (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstop1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtstp::Cstop1,
        gtstp::Cstop1,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtstp::Cstop1,
            gtstp::Cstop1,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Stop (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstop2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtstp::Cstop2,
        gtstp::Cstop2,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtstp::Cstop2,
            gtstp::Cstop2,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Stop (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstop3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtstp::Cstop3,
        gtstp::Cstop3,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtstp::Cstop3,
            gtstp::Cstop3,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Stop (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstop4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtstp::Cstop4,
        gtstp::Cstop4,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtstp::Cstop4,
            gtstp::Cstop4,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Stop (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstop5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtstp::Cstop5,
        gtstp::Cstop5,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtstp::Cstop5,
            gtstp::Cstop5,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Stop (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstop6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtstp::Cstop6,
        gtstp::Cstop6,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtstp::Cstop6,
            gtstp::Cstop6,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Stop (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstop7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtstp::Cstop7,
        gtstp::Cstop7,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtstp::Cstop7,
            gtstp::Cstop7,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Stop (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstop8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtstp::Cstop8,
        gtstp::Cstop8,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtstp::Cstop8,
            gtstp::Cstop8,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Stop (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cstop9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtstp::Cstop9,
        gtstp::Cstop9,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtstp::Cstop9,
            gtstp::Cstop9,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtstp {
    #[inline(always)]
    fn default() -> Gtstp {
        <crate::RegValueT<Gtstp_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod gtstp {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop0_SPEC;
    pub type Cstop0 = crate::EnumBitfieldStruct<u8, Cstop0_SPEC>;
    impl Cstop0 {
        #[doc = "GTCNT counter not stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop1_SPEC;
    pub type Cstop1 = crate::EnumBitfieldStruct<u8, Cstop1_SPEC>;
    impl Cstop1 {
        #[doc = "GTCNT counter not stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop2_SPEC;
    pub type Cstop2 = crate::EnumBitfieldStruct<u8, Cstop2_SPEC>;
    impl Cstop2 {
        #[doc = "GTCNT counter not stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop3_SPEC;
    pub type Cstop3 = crate::EnumBitfieldStruct<u8, Cstop3_SPEC>;
    impl Cstop3 {
        #[doc = "GTCNT counter not stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop4_SPEC;
    pub type Cstop4 = crate::EnumBitfieldStruct<u8, Cstop4_SPEC>;
    impl Cstop4 {
        #[doc = "GTCNT counter not stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop5_SPEC;
    pub type Cstop5 = crate::EnumBitfieldStruct<u8, Cstop5_SPEC>;
    impl Cstop5 {
        #[doc = "GTCNT counter not stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop6_SPEC;
    pub type Cstop6 = crate::EnumBitfieldStruct<u8, Cstop6_SPEC>;
    impl Cstop6 {
        #[doc = "GTCNT counter not stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop7_SPEC;
    pub type Cstop7 = crate::EnumBitfieldStruct<u8, Cstop7_SPEC>;
    impl Cstop7 {
        #[doc = "GTCNT counter not stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop8_SPEC;
    pub type Cstop8 = crate::EnumBitfieldStruct<u8, Cstop8_SPEC>;
    impl Cstop8 {
        #[doc = "GTCNT counter not stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop9_SPEC;
    pub type Cstop9 = crate::EnumBitfieldStruct<u8, Cstop9_SPEC>;
    impl Cstop9 {
        #[doc = "GTCNT counter not stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter stop"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtclr_SPEC;
impl crate::sealed::RegSpec for Gtclr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Software Clear Register"]
pub type Gtclr = crate::RegValueT<Gtclr_SPEC>;

impl Gtclr {
    #[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cclr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtclr::Cclr0,
        gtclr::Cclr0,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtclr::Cclr0,
            gtclr::Cclr0,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cclr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtclr::Cclr1,
        gtclr::Cclr1,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtclr::Cclr1,
            gtclr::Cclr1,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cclr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtclr::Cclr2,
        gtclr::Cclr2,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtclr::Cclr2,
            gtclr::Cclr2,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cclr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtclr::Cclr3,
        gtclr::Cclr3,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtclr::Cclr3,
            gtclr::Cclr3,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cclr4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtclr::Cclr4,
        gtclr::Cclr4,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtclr::Cclr4,
            gtclr::Cclr4,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cclr5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtclr::Cclr5,
        gtclr::Cclr5,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtclr::Cclr5,
            gtclr::Cclr5,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cclr6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtclr::Cclr6,
        gtclr::Cclr6,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtclr::Cclr6,
            gtclr::Cclr6,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cclr7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtclr::Cclr7,
        gtclr::Cclr7,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtclr::Cclr7,
            gtclr::Cclr7,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cclr8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtclr::Cclr8,
        gtclr::Cclr8,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtclr::Cclr8,
            gtclr::Cclr8,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel n GTCNT Count Clear (n : the same as bit position value)"]
    #[inline(always)]
    pub fn cclr9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtclr::Cclr9,
        gtclr::Cclr9,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtclr::Cclr9,
            gtclr::Cclr9,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtclr {
    #[inline(always)]
    fn default() -> Gtclr {
        <crate::RegValueT<Gtclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr0_SPEC;
    pub type Cclr0 = crate::EnumBitfieldStruct<u8, Cclr0_SPEC>;
    impl Cclr0 {
        #[doc = "GTCNT counter is not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter is cleared"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr1_SPEC;
    pub type Cclr1 = crate::EnumBitfieldStruct<u8, Cclr1_SPEC>;
    impl Cclr1 {
        #[doc = "GTCNT counter is not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter is cleared"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr2_SPEC;
    pub type Cclr2 = crate::EnumBitfieldStruct<u8, Cclr2_SPEC>;
    impl Cclr2 {
        #[doc = "GTCNT counter is not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter is cleared"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr3_SPEC;
    pub type Cclr3 = crate::EnumBitfieldStruct<u8, Cclr3_SPEC>;
    impl Cclr3 {
        #[doc = "GTCNT counter is not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter is cleared"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr4_SPEC;
    pub type Cclr4 = crate::EnumBitfieldStruct<u8, Cclr4_SPEC>;
    impl Cclr4 {
        #[doc = "GTCNT counter is not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter is cleared"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr5_SPEC;
    pub type Cclr5 = crate::EnumBitfieldStruct<u8, Cclr5_SPEC>;
    impl Cclr5 {
        #[doc = "GTCNT counter is not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter is cleared"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr6_SPEC;
    pub type Cclr6 = crate::EnumBitfieldStruct<u8, Cclr6_SPEC>;
    impl Cclr6 {
        #[doc = "GTCNT counter is not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter is cleared"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr7_SPEC;
    pub type Cclr7 = crate::EnumBitfieldStruct<u8, Cclr7_SPEC>;
    impl Cclr7 {
        #[doc = "GTCNT counter is not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter is cleared"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr8_SPEC;
    pub type Cclr8 = crate::EnumBitfieldStruct<u8, Cclr8_SPEC>;
    impl Cclr8 {
        #[doc = "GTCNT counter is not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter is cleared"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr9_SPEC;
    pub type Cclr9 = crate::EnumBitfieldStruct<u8, Cclr9_SPEC>;
    impl Cclr9 {
        #[doc = "GTCNT counter is not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter is cleared"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtssr_SPEC;
impl crate::sealed::RegSpec for Gtssr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Start Source Select Register"]
pub type Gtssr = crate::RegValueT<Gtssr_SPEC>;

impl Gtssr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgar(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtssr::Ssgtrgar,
        gtssr::Ssgtrgar,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtssr::Ssgtrgar,
            gtssr::Ssgtrgar,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGA Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgaf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtssr::Ssgtrgaf,
        gtssr::Ssgtrgaf,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtssr::Ssgtrgaf,
            gtssr::Ssgtrgaf,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGB Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtssr::Ssgtrgbr,
        gtssr::Ssgtrgbr,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtssr::Ssgtrgbr,
            gtssr::Ssgtrgbr,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGB Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgbf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtssr::Ssgtrgbf,
        gtssr::Ssgtrgbf,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtssr::Ssgtrgbf,
            gtssr::Ssgtrgbf,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGC Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtssr::Ssgtrgcr,
        gtssr::Ssgtrgcr,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtssr::Ssgtrgcr,
            gtssr::Ssgtrgcr,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGC Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtssr::Ssgtrgcf,
        gtssr::Ssgtrgcf,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtssr::Ssgtrgcf,
            gtssr::Ssgtrgcf,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGD Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgdr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtssr::Ssgtrgdr,
        gtssr::Ssgtrgdr,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtssr::Ssgtrgdr,
            gtssr::Ssgtrgdr,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGD Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgdf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtssr::Ssgtrgdf,
        gtssr::Ssgtrgdf,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtssr::Ssgtrgdf,
            gtssr::Ssgtrgdf,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscarbl(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtssr::Sscarbl,
        gtssr::Sscarbl,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtssr::Sscarbl,
            gtssr::Sscarbl,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscarbh(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtssr::Sscarbh,
        gtssr::Sscarbh,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtssr::Sscarbh,
            gtssr::Sscarbh,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscafbl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtssr::Sscafbl,
        gtssr::Sscafbl,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtssr::Sscafbl,
            gtssr::Sscafbl,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscafbh(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtssr::Sscafbh,
        gtssr::Sscafbh,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtssr::Sscafbh,
            gtssr::Sscafbh,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbral(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtssr::Sscbral,
        gtssr::Sscbral,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtssr::Sscbral,
            gtssr::Sscbral,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbrah(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtssr::Sscbrah,
        gtssr::Sscbrah,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtssr::Sscbrah,
            gtssr::Sscbrah,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbfal(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gtssr::Sscbfal,
        gtssr::Sscbfal,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gtssr::Sscbfal,
            gtssr::Sscbfal,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbfah(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtssr::Sscbfah,
        gtssr::Sscbfah,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtssr::Sscbfah,
            gtssr::Sscbfah,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTA Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselca(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtssr::Sselca,
        gtssr::Sselca,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtssr::Sselca,
            gtssr::Sselca,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTB Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcb(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtssr::Sselcb,
        gtssr::Sselcb,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtssr::Sselcb,
            gtssr::Sselcb,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTC Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcc(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtssr::Sselcc,
        gtssr::Sselcc,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtssr::Sselcc,
            gtssr::Sselcc,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTD Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcd(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtssr::Sselcd,
        gtssr::Sselcd,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtssr::Sselcd,
            gtssr::Sselcd,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTE Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselce(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gtssr::Sselce,
        gtssr::Sselce,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gtssr::Sselce,
            gtssr::Sselce,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTF Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcf(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gtssr::Sselcf,
        gtssr::Sselcf,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gtssr::Sselcf,
            gtssr::Sselcf,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTG Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcg(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gtssr::Sselcg,
        gtssr::Sselcg,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gtssr::Sselcg,
            gtssr::Sselcg,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTH Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselch(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gtssr::Sselch,
        gtssr::Sselch,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gtssr::Sselch,
            gtssr::Sselch,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software Source Counter Start Enable"]
    #[inline(always)]
    pub fn cstrt(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        gtssr::Cstrt,
        gtssr::Cstrt,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            gtssr::Cstrt,
            gtssr::Cstrt,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtssr {
    #[inline(always)]
    fn default() -> Gtssr {
        <crate::RegValueT<Gtssr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtssr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgar_SPEC;
    pub type Ssgtrgar = crate::EnumBitfieldStruct<u8, Ssgtrgar_SPEC>;
    impl Ssgtrgar {
        #[doc = "Counter start disabled on the rising edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the rising edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgaf_SPEC;
    pub type Ssgtrgaf = crate::EnumBitfieldStruct<u8, Ssgtrgaf_SPEC>;
    impl Ssgtrgaf {
        #[doc = "Counter start disabled on the falling edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the falling edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgbr_SPEC;
    pub type Ssgtrgbr = crate::EnumBitfieldStruct<u8, Ssgtrgbr_SPEC>;
    impl Ssgtrgbr {
        #[doc = "Counter start disabled on the rising edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the rising edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgbf_SPEC;
    pub type Ssgtrgbf = crate::EnumBitfieldStruct<u8, Ssgtrgbf_SPEC>;
    impl Ssgtrgbf {
        #[doc = "Counter start disabled on the falling edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the falling edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgcr_SPEC;
    pub type Ssgtrgcr = crate::EnumBitfieldStruct<u8, Ssgtrgcr_SPEC>;
    impl Ssgtrgcr {
        #[doc = "Counter start disabled on the rising edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the rising edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgcf_SPEC;
    pub type Ssgtrgcf = crate::EnumBitfieldStruct<u8, Ssgtrgcf_SPEC>;
    impl Ssgtrgcf {
        #[doc = "Counter start disabled on the falling edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the falling edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgdr_SPEC;
    pub type Ssgtrgdr = crate::EnumBitfieldStruct<u8, Ssgtrgdr_SPEC>;
    impl Ssgtrgdr {
        #[doc = "Counter start disabled on the rising edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the rising edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgdf_SPEC;
    pub type Ssgtrgdf = crate::EnumBitfieldStruct<u8, Ssgtrgdf_SPEC>;
    impl Ssgtrgdf {
        #[doc = "Counter start disabled on the falling edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the falling edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscarbl_SPEC;
    pub type Sscarbl = crate::EnumBitfieldStruct<u8, Sscarbl_SPEC>;
    impl Sscarbl {
        #[doc = "Counter start disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscarbh_SPEC;
    pub type Sscarbh = crate::EnumBitfieldStruct<u8, Sscarbh_SPEC>;
    impl Sscarbh {
        #[doc = "Counter start disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscafbl_SPEC;
    pub type Sscafbl = crate::EnumBitfieldStruct<u8, Sscafbl_SPEC>;
    impl Sscafbl {
        #[doc = "Counter start disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscafbh_SPEC;
    pub type Sscafbh = crate::EnumBitfieldStruct<u8, Sscafbh_SPEC>;
    impl Sscafbh {
        #[doc = "Counter start disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscbral_SPEC;
    pub type Sscbral = crate::EnumBitfieldStruct<u8, Sscbral_SPEC>;
    impl Sscbral {
        #[doc = "Counter start disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscbrah_SPEC;
    pub type Sscbrah = crate::EnumBitfieldStruct<u8, Sscbrah_SPEC>;
    impl Sscbrah {
        #[doc = "Counter start disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscbfal_SPEC;
    pub type Sscbfal = crate::EnumBitfieldStruct<u8, Sscbfal_SPEC>;
    impl Sscbfal {
        #[doc = "Counter start disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscbfah_SPEC;
    pub type Sscbfah = crate::EnumBitfieldStruct<u8, Sscbfah_SPEC>;
    impl Sscbfah {
        #[doc = "Counter start disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselca_SPEC;
    pub type Sselca = crate::EnumBitfieldStruct<u8, Sselca_SPEC>;
    impl Sselca {
        #[doc = "Counter start disabled at the ELC_GPTA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled at the ELC_GPTA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcb_SPEC;
    pub type Sselcb = crate::EnumBitfieldStruct<u8, Sselcb_SPEC>;
    impl Sselcb {
        #[doc = "Counter start disabled at the ELC_GPTB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled at the ELC_GPTB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcc_SPEC;
    pub type Sselcc = crate::EnumBitfieldStruct<u8, Sselcc_SPEC>;
    impl Sselcc {
        #[doc = "Counter start disabled at the ELC_GPTC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled at the ELC_GPTC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcd_SPEC;
    pub type Sselcd = crate::EnumBitfieldStruct<u8, Sselcd_SPEC>;
    impl Sselcd {
        #[doc = "Counter start disabled at the ELC_GPTD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled at the ELC_GPTD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselce_SPEC;
    pub type Sselce = crate::EnumBitfieldStruct<u8, Sselce_SPEC>;
    impl Sselce {
        #[doc = "Counter start disabled at the ELC_GPTE input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled at the ELC_GPTE input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcf_SPEC;
    pub type Sselcf = crate::EnumBitfieldStruct<u8, Sselcf_SPEC>;
    impl Sselcf {
        #[doc = "Counter start disabled at the ELC_GPTF input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled at the ELC_GPTF input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcg_SPEC;
    pub type Sselcg = crate::EnumBitfieldStruct<u8, Sselcg_SPEC>;
    impl Sselcg {
        #[doc = "Counter start disabled at the ELC_GPTG input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled at the ELC_GPTG input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselch_SPEC;
    pub type Sselch = crate::EnumBitfieldStruct<u8, Sselch_SPEC>;
    impl Sselch {
        #[doc = "Counter start disabled at the ELC_GPTH input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled at the ELC_GPTH input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt_SPEC;
    pub type Cstrt = crate::EnumBitfieldStruct<u8, Cstrt_SPEC>;
    impl Cstrt {
        #[doc = "Counter start disabled by the GTSTR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter start enabled by the GTSTR register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpsr_SPEC;
impl crate::sealed::RegSpec for Gtpsr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Stop Source Select Register"]
pub type Gtpsr = crate::RegValueT<Gtpsr_SPEC>;

impl Gtpsr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgar(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtpsr::Psgtrgar,
        gtpsr::Psgtrgar,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtpsr::Psgtrgar,
            gtpsr::Psgtrgar,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGA Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgaf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtpsr::Psgtrgaf,
        gtpsr::Psgtrgaf,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtpsr::Psgtrgaf,
            gtpsr::Psgtrgaf,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGB Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtpsr::Psgtrgbr,
        gtpsr::Psgtrgbr,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtpsr::Psgtrgbr,
            gtpsr::Psgtrgbr,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGB Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgbf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtpsr::Psgtrgbf,
        gtpsr::Psgtrgbf,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtpsr::Psgtrgbf,
            gtpsr::Psgtrgbf,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGC Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtpsr::Psgtrgcr,
        gtpsr::Psgtrgcr,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtpsr::Psgtrgcr,
            gtpsr::Psgtrgcr,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGC Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtpsr::Psgtrgcf,
        gtpsr::Psgtrgcf,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtpsr::Psgtrgcf,
            gtpsr::Psgtrgcf,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGD Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgdr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtpsr::Psgtrgdr,
        gtpsr::Psgtrgdr,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtpsr::Psgtrgdr,
            gtpsr::Psgtrgdr,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGD Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgdf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtpsr::Psgtrgdf,
        gtpsr::Psgtrgdf,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtpsr::Psgtrgdf,
            gtpsr::Psgtrgdf,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscarbl(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtpsr::Pscarbl,
        gtpsr::Pscarbl,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtpsr::Pscarbl,
            gtpsr::Pscarbl,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscarbh(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtpsr::Pscarbh,
        gtpsr::Pscarbh,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtpsr::Pscarbh,
            gtpsr::Pscarbh,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscafbl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtpsr::Pscafbl,
        gtpsr::Pscafbl,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtpsr::Pscafbl,
            gtpsr::Pscafbl,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscafbh(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtpsr::Pscafbh,
        gtpsr::Pscafbh,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtpsr::Pscafbh,
            gtpsr::Pscafbh,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbral(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtpsr::Pscbral,
        gtpsr::Pscbral,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtpsr::Pscbral,
            gtpsr::Pscbral,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbrah(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtpsr::Pscbrah,
        gtpsr::Pscbrah,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtpsr::Pscbrah,
            gtpsr::Pscbrah,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbfal(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gtpsr::Pscbfal,
        gtpsr::Pscbfal,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gtpsr::Pscbfal,
            gtpsr::Pscbfal,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbfah(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtpsr::Pscbfah,
        gtpsr::Pscbfah,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtpsr::Pscbfah,
            gtpsr::Pscbfah,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTA Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselca(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtpsr::Pselca,
        gtpsr::Pselca,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtpsr::Pselca,
            gtpsr::Pselca,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTB Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcb(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtpsr::Pselcb,
        gtpsr::Pselcb,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtpsr::Pselcb,
            gtpsr::Pselcb,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTC Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcc(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtpsr::Pselcc,
        gtpsr::Pselcc,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtpsr::Pselcc,
            gtpsr::Pselcc,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTD Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcd(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtpsr::Pselcd,
        gtpsr::Pselcd,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtpsr::Pselcd,
            gtpsr::Pselcd,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTE Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselce(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gtpsr::Pselce,
        gtpsr::Pselce,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gtpsr::Pselce,
            gtpsr::Pselce,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTF Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcf(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gtpsr::Pselcf,
        gtpsr::Pselcf,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gtpsr::Pselcf,
            gtpsr::Pselcf,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTG Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcg(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gtpsr::Pselcg,
        gtpsr::Pselcg,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gtpsr::Pselcg,
            gtpsr::Pselcg,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTH Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselch(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gtpsr::Pselch,
        gtpsr::Pselch,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gtpsr::Pselch,
            gtpsr::Pselch,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software Source Counter Stop Enable"]
    #[inline(always)]
    pub fn cstop(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        gtpsr::Cstop,
        gtpsr::Cstop,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            gtpsr::Cstop,
            gtpsr::Cstop,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtpsr {
    #[inline(always)]
    fn default() -> Gtpsr {
        <crate::RegValueT<Gtpsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtpsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgar_SPEC;
    pub type Psgtrgar = crate::EnumBitfieldStruct<u8, Psgtrgar_SPEC>;
    impl Psgtrgar {
        #[doc = "Counter stop disabled on the rising edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the rising edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgaf_SPEC;
    pub type Psgtrgaf = crate::EnumBitfieldStruct<u8, Psgtrgaf_SPEC>;
    impl Psgtrgaf {
        #[doc = "Counter stop disabled on the falling edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the falling edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgbr_SPEC;
    pub type Psgtrgbr = crate::EnumBitfieldStruct<u8, Psgtrgbr_SPEC>;
    impl Psgtrgbr {
        #[doc = "Counter stop disabled on the rising edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the rising edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgbf_SPEC;
    pub type Psgtrgbf = crate::EnumBitfieldStruct<u8, Psgtrgbf_SPEC>;
    impl Psgtrgbf {
        #[doc = "Counter stop disabled on the falling edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the falling edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgcr_SPEC;
    pub type Psgtrgcr = crate::EnumBitfieldStruct<u8, Psgtrgcr_SPEC>;
    impl Psgtrgcr {
        #[doc = "Counter stop disabled on the rising edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the rising edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgcf_SPEC;
    pub type Psgtrgcf = crate::EnumBitfieldStruct<u8, Psgtrgcf_SPEC>;
    impl Psgtrgcf {
        #[doc = "Counter stop disabled on the falling edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the falling edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgdr_SPEC;
    pub type Psgtrgdr = crate::EnumBitfieldStruct<u8, Psgtrgdr_SPEC>;
    impl Psgtrgdr {
        #[doc = "Counter stop disabled on the rising edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the rising edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgdf_SPEC;
    pub type Psgtrgdf = crate::EnumBitfieldStruct<u8, Psgtrgdf_SPEC>;
    impl Psgtrgdf {
        #[doc = "Counter stop disabled on the falling edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the falling edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscarbl_SPEC;
    pub type Pscarbl = crate::EnumBitfieldStruct<u8, Pscarbl_SPEC>;
    impl Pscarbl {
        #[doc = "Counter stop disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscarbh_SPEC;
    pub type Pscarbh = crate::EnumBitfieldStruct<u8, Pscarbh_SPEC>;
    impl Pscarbh {
        #[doc = "Counter stop disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscafbl_SPEC;
    pub type Pscafbl = crate::EnumBitfieldStruct<u8, Pscafbl_SPEC>;
    impl Pscafbl {
        #[doc = "Counter stop disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscafbh_SPEC;
    pub type Pscafbh = crate::EnumBitfieldStruct<u8, Pscafbh_SPEC>;
    impl Pscafbh {
        #[doc = "Counter stop disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscbral_SPEC;
    pub type Pscbral = crate::EnumBitfieldStruct<u8, Pscbral_SPEC>;
    impl Pscbral {
        #[doc = "Counter stop disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscbrah_SPEC;
    pub type Pscbrah = crate::EnumBitfieldStruct<u8, Pscbrah_SPEC>;
    impl Pscbrah {
        #[doc = "Counter stop disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscbfal_SPEC;
    pub type Pscbfal = crate::EnumBitfieldStruct<u8, Pscbfal_SPEC>;
    impl Pscbfal {
        #[doc = "Counter stop disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscbfah_SPEC;
    pub type Pscbfah = crate::EnumBitfieldStruct<u8, Pscbfah_SPEC>;
    impl Pscbfah {
        #[doc = "Counter stop disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselca_SPEC;
    pub type Pselca = crate::EnumBitfieldStruct<u8, Pselca_SPEC>;
    impl Pselca {
        #[doc = "Counter stop disabled at the ELC_GPTA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled at the ELC_GPTA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcb_SPEC;
    pub type Pselcb = crate::EnumBitfieldStruct<u8, Pselcb_SPEC>;
    impl Pselcb {
        #[doc = "Counter stop disabled at the ELC_GPTB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled at the ELC_GPTB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcc_SPEC;
    pub type Pselcc = crate::EnumBitfieldStruct<u8, Pselcc_SPEC>;
    impl Pselcc {
        #[doc = "Counter stop disabled at the ELC_GPTC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled at the ELC_GPTC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcd_SPEC;
    pub type Pselcd = crate::EnumBitfieldStruct<u8, Pselcd_SPEC>;
    impl Pselcd {
        #[doc = "Counter stop disabled at the ELC_GPTD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled at the ELC_GPTD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselce_SPEC;
    pub type Pselce = crate::EnumBitfieldStruct<u8, Pselce_SPEC>;
    impl Pselce {
        #[doc = "Counter stop disabled at the ELC_GPTE input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled at the ELC_GPTE input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcf_SPEC;
    pub type Pselcf = crate::EnumBitfieldStruct<u8, Pselcf_SPEC>;
    impl Pselcf {
        #[doc = "Counter stop disabled at the ELC_GPTF input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled at the ELC_GPTF input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcg_SPEC;
    pub type Pselcg = crate::EnumBitfieldStruct<u8, Pselcg_SPEC>;
    impl Pselcg {
        #[doc = "Counter stop disabled at the ELC_GPTG input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled at the ELC_GPTG input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselch_SPEC;
    pub type Pselch = crate::EnumBitfieldStruct<u8, Pselch_SPEC>;
    impl Pselch {
        #[doc = "Counter stop disabled at the ELC_GPTH input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled at the ELC_GPTH input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop_SPEC;
    pub type Cstop = crate::EnumBitfieldStruct<u8, Cstop_SPEC>;
    impl Cstop {
        #[doc = "Counter stop disabled by the GTSTP register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter stop enabled by the GTSTP register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtcsr_SPEC;
impl crate::sealed::RegSpec for Gtcsr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Clear Source Select Register"]
pub type Gtcsr = crate::RegValueT<Gtcsr_SPEC>;

impl Gtcsr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgar(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtcsr::Csgtrgar,
        gtcsr::Csgtrgar,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtcsr::Csgtrgar,
            gtcsr::Csgtrgar,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGA Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgaf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtcsr::Csgtrgaf,
        gtcsr::Csgtrgaf,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtcsr::Csgtrgaf,
            gtcsr::Csgtrgaf,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGB Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtcsr::Csgtrgbr,
        gtcsr::Csgtrgbr,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtcsr::Csgtrgbr,
            gtcsr::Csgtrgbr,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGB Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgbf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtcsr::Csgtrgbf,
        gtcsr::Csgtrgbf,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtcsr::Csgtrgbf,
            gtcsr::Csgtrgbf,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGC Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtcsr::Csgtrgcr,
        gtcsr::Csgtrgcr,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtcsr::Csgtrgcr,
            gtcsr::Csgtrgcr,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGC Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtcsr::Csgtrgcf,
        gtcsr::Csgtrgcf,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtcsr::Csgtrgcf,
            gtcsr::Csgtrgcf,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGD Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgdr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtcsr::Csgtrgdr,
        gtcsr::Csgtrgdr,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtcsr::Csgtrgdr,
            gtcsr::Csgtrgdr,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGD Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgdf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtcsr::Csgtrgdf,
        gtcsr::Csgtrgdf,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtcsr::Csgtrgdf,
            gtcsr::Csgtrgdf,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscarbl(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtcsr::Cscarbl,
        gtcsr::Cscarbl,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtcsr::Cscarbl,
            gtcsr::Cscarbl,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscarbh(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtcsr::Cscarbh,
        gtcsr::Cscarbh,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtcsr::Cscarbh,
            gtcsr::Cscarbh,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscafbl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtcsr::Cscafbl,
        gtcsr::Cscafbl,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtcsr::Cscafbl,
            gtcsr::Cscafbl,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscafbh(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtcsr::Cscafbh,
        gtcsr::Cscafbh,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtcsr::Cscafbh,
            gtcsr::Cscafbh,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbral(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtcsr::Cscbral,
        gtcsr::Cscbral,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtcsr::Cscbral,
            gtcsr::Cscbral,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbrah(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtcsr::Cscbrah,
        gtcsr::Cscbrah,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtcsr::Cscbrah,
            gtcsr::Cscbrah,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbfal(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gtcsr::Cscbfal,
        gtcsr::Cscbfal,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gtcsr::Cscbfal,
            gtcsr::Cscbfal,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbfah(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtcsr::Cscbfah,
        gtcsr::Cscbfah,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtcsr::Cscbfah,
            gtcsr::Cscbfah,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTA Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselca(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtcsr::Cselca,
        gtcsr::Cselca,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtcsr::Cselca,
            gtcsr::Cselca,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTB Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcb(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtcsr::Cselcb,
        gtcsr::Cselcb,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtcsr::Cselcb,
            gtcsr::Cselcb,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTC Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcc(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtcsr::Cselcc,
        gtcsr::Cselcc,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtcsr::Cselcc,
            gtcsr::Cselcc,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTD Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcd(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtcsr::Cselcd,
        gtcsr::Cselcd,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtcsr::Cselcd,
            gtcsr::Cselcd,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTE Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselce(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gtcsr::Cselce,
        gtcsr::Cselce,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gtcsr::Cselce,
            gtcsr::Cselce,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTF Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcf(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gtcsr::Cselcf,
        gtcsr::Cselcf,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gtcsr::Cselcf,
            gtcsr::Cselcf,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTG Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcg(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gtcsr::Cselcg,
        gtcsr::Cselcg,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gtcsr::Cselcg,
            gtcsr::Cselcg,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTH Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselch(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gtcsr::Cselch,
        gtcsr::Cselch,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gtcsr::Cselch,
            gtcsr::Cselch,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cclr(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        gtcsr::Cclr,
        gtcsr::Cclr,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            gtcsr::Cclr,
            gtcsr::Cclr,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtcsr {
    #[inline(always)]
    fn default() -> Gtcsr {
        <crate::RegValueT<Gtcsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtcsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgar_SPEC;
    pub type Csgtrgar = crate::EnumBitfieldStruct<u8, Csgtrgar_SPEC>;
    impl Csgtrgar {
        #[doc = "Counter clear disabled on the rising edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled on the rising edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgaf_SPEC;
    pub type Csgtrgaf = crate::EnumBitfieldStruct<u8, Csgtrgaf_SPEC>;
    impl Csgtrgaf {
        #[doc = "Counter clear disabled on the falling edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled on the falling edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgbr_SPEC;
    pub type Csgtrgbr = crate::EnumBitfieldStruct<u8, Csgtrgbr_SPEC>;
    impl Csgtrgbr {
        #[doc = "Disable counter clear on the rising edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable counter clear on the rising edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgbf_SPEC;
    pub type Csgtrgbf = crate::EnumBitfieldStruct<u8, Csgtrgbf_SPEC>;
    impl Csgtrgbf {
        #[doc = "Counter clear disabled on the falling edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled on the falling edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgcr_SPEC;
    pub type Csgtrgcr = crate::EnumBitfieldStruct<u8, Csgtrgcr_SPEC>;
    impl Csgtrgcr {
        #[doc = "Disable counter clear on the rising edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable counter clear on the rising edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgcf_SPEC;
    pub type Csgtrgcf = crate::EnumBitfieldStruct<u8, Csgtrgcf_SPEC>;
    impl Csgtrgcf {
        #[doc = "Counter clear disabled on the falling edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled on the falling edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgdr_SPEC;
    pub type Csgtrgdr = crate::EnumBitfieldStruct<u8, Csgtrgdr_SPEC>;
    impl Csgtrgdr {
        #[doc = "Disable counter clear on the rising edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable counter clear on the rising edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgdf_SPEC;
    pub type Csgtrgdf = crate::EnumBitfieldStruct<u8, Csgtrgdf_SPEC>;
    impl Csgtrgdf {
        #[doc = "Counter clear disabled on the falling edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled on the falling edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscarbl_SPEC;
    pub type Cscarbl = crate::EnumBitfieldStruct<u8, Cscarbl_SPEC>;
    impl Cscarbl {
        #[doc = "Counter clear disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscarbh_SPEC;
    pub type Cscarbh = crate::EnumBitfieldStruct<u8, Cscarbh_SPEC>;
    impl Cscarbh {
        #[doc = "Counter clear disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscafbl_SPEC;
    pub type Cscafbl = crate::EnumBitfieldStruct<u8, Cscafbl_SPEC>;
    impl Cscafbl {
        #[doc = "Counter clear disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscafbh_SPEC;
    pub type Cscafbh = crate::EnumBitfieldStruct<u8, Cscafbh_SPEC>;
    impl Cscafbh {
        #[doc = "Counter clear disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscbral_SPEC;
    pub type Cscbral = crate::EnumBitfieldStruct<u8, Cscbral_SPEC>;
    impl Cscbral {
        #[doc = "Counter clear disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscbrah_SPEC;
    pub type Cscbrah = crate::EnumBitfieldStruct<u8, Cscbrah_SPEC>;
    impl Cscbrah {
        #[doc = "Counter clear disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscbfal_SPEC;
    pub type Cscbfal = crate::EnumBitfieldStruct<u8, Cscbfal_SPEC>;
    impl Cscbfal {
        #[doc = "Counter clear disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscbfah_SPEC;
    pub type Cscbfah = crate::EnumBitfieldStruct<u8, Cscbfah_SPEC>;
    impl Cscbfah {
        #[doc = "Counter clear disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselca_SPEC;
    pub type Cselca = crate::EnumBitfieldStruct<u8, Cselca_SPEC>;
    impl Cselca {
        #[doc = "Counter clear disabled at the ELC_GPTA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled at the ELC_GPTA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcb_SPEC;
    pub type Cselcb = crate::EnumBitfieldStruct<u8, Cselcb_SPEC>;
    impl Cselcb {
        #[doc = "Counter clear disabled at the ELC_GPTB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled at the ELC_GPTB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcc_SPEC;
    pub type Cselcc = crate::EnumBitfieldStruct<u8, Cselcc_SPEC>;
    impl Cselcc {
        #[doc = "Counter clear disabled at the ELC_GPTC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled at the ELC_GPTC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcd_SPEC;
    pub type Cselcd = crate::EnumBitfieldStruct<u8, Cselcd_SPEC>;
    impl Cselcd {
        #[doc = "Counter clear disabled at the ELC_GPTD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled at the ELC_GPTD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselce_SPEC;
    pub type Cselce = crate::EnumBitfieldStruct<u8, Cselce_SPEC>;
    impl Cselce {
        #[doc = "Counter clear disabled at the ELC_GPTE input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled at the ELC_GPTE input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcf_SPEC;
    pub type Cselcf = crate::EnumBitfieldStruct<u8, Cselcf_SPEC>;
    impl Cselcf {
        #[doc = "Counter clear disabled at the ELC_GPTF input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled at the ELC_GPTF input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcg_SPEC;
    pub type Cselcg = crate::EnumBitfieldStruct<u8, Cselcg_SPEC>;
    impl Cselcg {
        #[doc = "Counter clear disabled at the ELC_GPTG input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled at the ELC_GPTG input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselch_SPEC;
    pub type Cselch = crate::EnumBitfieldStruct<u8, Cselch_SPEC>;
    impl Cselch {
        #[doc = "Counter clear disabled at the ELC_GPTH input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled at the ELC_GPTH input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr_SPEC;
    pub type Cclr = crate::EnumBitfieldStruct<u8, Cclr_SPEC>;
    impl Cclr {
        #[doc = "Counter clear disabled by the GTCLR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter clear enabled by the GTCLR register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtupsr_SPEC;
impl crate::sealed::RegSpec for Gtupsr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Up Count Source Select Register"]
pub type Gtupsr = crate::RegValueT<Gtupsr_SPEC>;

impl Gtupsr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgar(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtupsr::Usgtrgar,
        gtupsr::Usgtrgar,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtupsr::Usgtrgar,
            gtupsr::Usgtrgar,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGA Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgaf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtupsr::Usgtrgaf,
        gtupsr::Usgtrgaf,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtupsr::Usgtrgaf,
            gtupsr::Usgtrgaf,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGB Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtupsr::Usgtrgbr,
        gtupsr::Usgtrgbr,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtupsr::Usgtrgbr,
            gtupsr::Usgtrgbr,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGB Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgbf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtupsr::Usgtrgbf,
        gtupsr::Usgtrgbf,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtupsr::Usgtrgbf,
            gtupsr::Usgtrgbf,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGC Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtupsr::Usgtrgcr,
        gtupsr::Usgtrgcr,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtupsr::Usgtrgcr,
            gtupsr::Usgtrgcr,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGC Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtupsr::Usgtrgcf,
        gtupsr::Usgtrgcf,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtupsr::Usgtrgcf,
            gtupsr::Usgtrgcf,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGD Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgdr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtupsr::Usgtrgdr,
        gtupsr::Usgtrgdr,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtupsr::Usgtrgdr,
            gtupsr::Usgtrgdr,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGD Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgdf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtupsr::Usgtrgdf,
        gtupsr::Usgtrgdf,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtupsr::Usgtrgdf,
            gtupsr::Usgtrgdf,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscarbl(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtupsr::Uscarbl,
        gtupsr::Uscarbl,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtupsr::Uscarbl,
            gtupsr::Uscarbl,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscarbh(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtupsr::Uscarbh,
        gtupsr::Uscarbh,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtupsr::Uscarbh,
            gtupsr::Uscarbh,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscafbl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtupsr::Uscafbl,
        gtupsr::Uscafbl,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtupsr::Uscafbl,
            gtupsr::Uscafbl,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscafbh(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtupsr::Uscafbh,
        gtupsr::Uscafbh,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtupsr::Uscafbh,
            gtupsr::Uscafbh,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbral(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtupsr::Uscbral,
        gtupsr::Uscbral,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtupsr::Uscbral,
            gtupsr::Uscbral,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbrah(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtupsr::Uscbrah,
        gtupsr::Uscbrah,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtupsr::Uscbrah,
            gtupsr::Uscbrah,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbfal(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gtupsr::Uscbfal,
        gtupsr::Uscbfal,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gtupsr::Uscbfal,
            gtupsr::Uscbfal,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbfah(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtupsr::Uscbfah,
        gtupsr::Uscbfah,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtupsr::Uscbfah,
            gtupsr::Uscbfah,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTA Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselca(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtupsr::Uselca,
        gtupsr::Uselca,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtupsr::Uselca,
            gtupsr::Uselca,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTB Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcb(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtupsr::Uselcb,
        gtupsr::Uselcb,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtupsr::Uselcb,
            gtupsr::Uselcb,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTC Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcc(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtupsr::Uselcc,
        gtupsr::Uselcc,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtupsr::Uselcc,
            gtupsr::Uselcc,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTD Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcd(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtupsr::Uselcd,
        gtupsr::Uselcd,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtupsr::Uselcd,
            gtupsr::Uselcd,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTE Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselce(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gtupsr::Uselce,
        gtupsr::Uselce,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gtupsr::Uselce,
            gtupsr::Uselce,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTF Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcf(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gtupsr::Uselcf,
        gtupsr::Uselcf,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gtupsr::Uselcf,
            gtupsr::Uselcf,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTG Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcg(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gtupsr::Uselcg,
        gtupsr::Uselcg,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gtupsr::Uselcg,
            gtupsr::Uselcg,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTH Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselch(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gtupsr::Uselch,
        gtupsr::Uselch,
        Gtupsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gtupsr::Uselch,
            gtupsr::Uselch,
            Gtupsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtupsr {
    #[inline(always)]
    fn default() -> Gtupsr {
        <crate::RegValueT<Gtupsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtupsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgar_SPEC;
    pub type Usgtrgar = crate::EnumBitfieldStruct<u8, Usgtrgar_SPEC>;
    impl Usgtrgar {
        #[doc = "Counter count up disabled on the rising edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the rising edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgaf_SPEC;
    pub type Usgtrgaf = crate::EnumBitfieldStruct<u8, Usgtrgaf_SPEC>;
    impl Usgtrgaf {
        #[doc = "Counter count up disabled on the falling edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the falling edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgbr_SPEC;
    pub type Usgtrgbr = crate::EnumBitfieldStruct<u8, Usgtrgbr_SPEC>;
    impl Usgtrgbr {
        #[doc = "Counter count up disabled on the rising edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the rising edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgbf_SPEC;
    pub type Usgtrgbf = crate::EnumBitfieldStruct<u8, Usgtrgbf_SPEC>;
    impl Usgtrgbf {
        #[doc = "Counter count up disabled on the falling edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the falling edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgcr_SPEC;
    pub type Usgtrgcr = crate::EnumBitfieldStruct<u8, Usgtrgcr_SPEC>;
    impl Usgtrgcr {
        #[doc = "Counter count up disabled on the rising edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the rising edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgcf_SPEC;
    pub type Usgtrgcf = crate::EnumBitfieldStruct<u8, Usgtrgcf_SPEC>;
    impl Usgtrgcf {
        #[doc = "Counter count up disabled on the falling edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the falling edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgdr_SPEC;
    pub type Usgtrgdr = crate::EnumBitfieldStruct<u8, Usgtrgdr_SPEC>;
    impl Usgtrgdr {
        #[doc = "Counter count up disabled on the rising edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the rising edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgdf_SPEC;
    pub type Usgtrgdf = crate::EnumBitfieldStruct<u8, Usgtrgdf_SPEC>;
    impl Usgtrgdf {
        #[doc = "Counter count up disabled on the falling edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the falling edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscarbl_SPEC;
    pub type Uscarbl = crate::EnumBitfieldStruct<u8, Uscarbl_SPEC>;
    impl Uscarbl {
        #[doc = "Counter count up disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscarbh_SPEC;
    pub type Uscarbh = crate::EnumBitfieldStruct<u8, Uscarbh_SPEC>;
    impl Uscarbh {
        #[doc = "Counter count up disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscafbl_SPEC;
    pub type Uscafbl = crate::EnumBitfieldStruct<u8, Uscafbl_SPEC>;
    impl Uscafbl {
        #[doc = "Counter count up disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscafbh_SPEC;
    pub type Uscafbh = crate::EnumBitfieldStruct<u8, Uscafbh_SPEC>;
    impl Uscafbh {
        #[doc = "Counter count up disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscbral_SPEC;
    pub type Uscbral = crate::EnumBitfieldStruct<u8, Uscbral_SPEC>;
    impl Uscbral {
        #[doc = "Counter count up disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscbrah_SPEC;
    pub type Uscbrah = crate::EnumBitfieldStruct<u8, Uscbrah_SPEC>;
    impl Uscbrah {
        #[doc = "Counter count up disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscbfal_SPEC;
    pub type Uscbfal = crate::EnumBitfieldStruct<u8, Uscbfal_SPEC>;
    impl Uscbfal {
        #[doc = "Counter count up disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscbfah_SPEC;
    pub type Uscbfah = crate::EnumBitfieldStruct<u8, Uscbfah_SPEC>;
    impl Uscbfah {
        #[doc = "Counter count up disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselca_SPEC;
    pub type Uselca = crate::EnumBitfieldStruct<u8, Uselca_SPEC>;
    impl Uselca {
        #[doc = "Counter count up disabled at the ELC_GPTA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled at the ELC_GPTA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselcb_SPEC;
    pub type Uselcb = crate::EnumBitfieldStruct<u8, Uselcb_SPEC>;
    impl Uselcb {
        #[doc = "Counter count up disabled at the ELC_GPTB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled at the ELC_GPTB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselcc_SPEC;
    pub type Uselcc = crate::EnumBitfieldStruct<u8, Uselcc_SPEC>;
    impl Uselcc {
        #[doc = "Counter count up disabled at the ELC_GPTC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled at the ELC_GPTC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselcd_SPEC;
    pub type Uselcd = crate::EnumBitfieldStruct<u8, Uselcd_SPEC>;
    impl Uselcd {
        #[doc = "Counter count up disabled at the ELC_GPTD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled at the ELC_GPTD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselce_SPEC;
    pub type Uselce = crate::EnumBitfieldStruct<u8, Uselce_SPEC>;
    impl Uselce {
        #[doc = "Counter count up disabled at the ELC_GPTE input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled at the ELC_GPTE input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselcf_SPEC;
    pub type Uselcf = crate::EnumBitfieldStruct<u8, Uselcf_SPEC>;
    impl Uselcf {
        #[doc = "Counter count up disabled at the ELC_GPTF input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled at the ELC_GPTF input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselcg_SPEC;
    pub type Uselcg = crate::EnumBitfieldStruct<u8, Uselcg_SPEC>;
    impl Uselcg {
        #[doc = "Counter count up disabled at the ELC_GPTG input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled at the ELC_GPTG input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselch_SPEC;
    pub type Uselch = crate::EnumBitfieldStruct<u8, Uselch_SPEC>;
    impl Uselch {
        #[doc = "Counter count up disabled at the ELC_GPTH input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count up enabled at the ELC_GPTH input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdnsr_SPEC;
impl crate::sealed::RegSpec for Gtdnsr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Down Count Source Select Register"]
pub type Gtdnsr = crate::RegValueT<Gtdnsr_SPEC>;

impl Gtdnsr {
    #[doc = "GTETRGA Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgar(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtdnsr::Dsgtrgar,
        gtdnsr::Dsgtrgar,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtdnsr::Dsgtrgar,
            gtdnsr::Dsgtrgar,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGA Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgaf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtdnsr::Dsgtrgaf,
        gtdnsr::Dsgtrgaf,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtdnsr::Dsgtrgaf,
            gtdnsr::Dsgtrgaf,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGB Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtdnsr::Dsgtrgbr,
        gtdnsr::Dsgtrgbr,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtdnsr::Dsgtrgbr,
            gtdnsr::Dsgtrgbr,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGB Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgbf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtdnsr::Dsgtrgbf,
        gtdnsr::Dsgtrgbf,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtdnsr::Dsgtrgbf,
            gtdnsr::Dsgtrgbf,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGC Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtdnsr::Dsgtrgcr,
        gtdnsr::Dsgtrgcr,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtdnsr::Dsgtrgcr,
            gtdnsr::Dsgtrgcr,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGC Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtdnsr::Dsgtrgcf,
        gtdnsr::Dsgtrgcf,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtdnsr::Dsgtrgcf,
            gtdnsr::Dsgtrgcf,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGD Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgdr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtdnsr::Dsgtrgdr,
        gtdnsr::Dsgtrgdr,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtdnsr::Dsgtrgdr,
            gtdnsr::Dsgtrgdr,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGD Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgdf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtdnsr::Dsgtrgdf,
        gtdnsr::Dsgtrgdf,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtdnsr::Dsgtrgdf,
            gtdnsr::Dsgtrgdf,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscarbl(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtdnsr::Dscarbl,
        gtdnsr::Dscarbl,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtdnsr::Dscarbl,
            gtdnsr::Dscarbl,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscarbh(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtdnsr::Dscarbh,
        gtdnsr::Dscarbh,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtdnsr::Dscarbh,
            gtdnsr::Dscarbh,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscafbl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtdnsr::Dscafbl,
        gtdnsr::Dscafbl,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtdnsr::Dscafbl,
            gtdnsr::Dscafbl,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscafbh(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtdnsr::Dscafbh,
        gtdnsr::Dscafbh,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtdnsr::Dscafbh,
            gtdnsr::Dscafbh,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbral(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtdnsr::Dscbral,
        gtdnsr::Dscbral,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtdnsr::Dscbral,
            gtdnsr::Dscbral,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbrah(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtdnsr::Dscbrah,
        gtdnsr::Dscbrah,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtdnsr::Dscbrah,
            gtdnsr::Dscbrah,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbfal(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gtdnsr::Dscbfal,
        gtdnsr::Dscbfal,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gtdnsr::Dscbfal,
            gtdnsr::Dscbfal,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbfah(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtdnsr::Dscbfah,
        gtdnsr::Dscbfah,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtdnsr::Dscbfah,
            gtdnsr::Dscbfah,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTA Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselca(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtdnsr::Dselca,
        gtdnsr::Dselca,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtdnsr::Dselca,
            gtdnsr::Dselca,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTB Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcb(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtdnsr::Dselcb,
        gtdnsr::Dselcb,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtdnsr::Dselcb,
            gtdnsr::Dselcb,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTC Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcc(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtdnsr::Dselcc,
        gtdnsr::Dselcc,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtdnsr::Dselcc,
            gtdnsr::Dselcc,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTD Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcd(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtdnsr::Dselcd,
        gtdnsr::Dselcd,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtdnsr::Dselcd,
            gtdnsr::Dselcd,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTE Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselce(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gtdnsr::Dselce,
        gtdnsr::Dselce,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gtdnsr::Dselce,
            gtdnsr::Dselce,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTF Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcf(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gtdnsr::Dselcf,
        gtdnsr::Dselcf,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gtdnsr::Dselcf,
            gtdnsr::Dselcf,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTG Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcg(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gtdnsr::Dselcg,
        gtdnsr::Dselcg,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gtdnsr::Dselcg,
            gtdnsr::Dselcg,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTF Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselch(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gtdnsr::Dselch,
        gtdnsr::Dselch,
        Gtdnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gtdnsr::Dselch,
            gtdnsr::Dselch,
            Gtdnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdnsr {
    #[inline(always)]
    fn default() -> Gtdnsr {
        <crate::RegValueT<Gtdnsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdnsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgar_SPEC;
    pub type Dsgtrgar = crate::EnumBitfieldStruct<u8, Dsgtrgar_SPEC>;
    impl Dsgtrgar {
        #[doc = "Counter count down disabled on the rising edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the rising edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgaf_SPEC;
    pub type Dsgtrgaf = crate::EnumBitfieldStruct<u8, Dsgtrgaf_SPEC>;
    impl Dsgtrgaf {
        #[doc = "Counter count down disabled on the falling edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the falling edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgbr_SPEC;
    pub type Dsgtrgbr = crate::EnumBitfieldStruct<u8, Dsgtrgbr_SPEC>;
    impl Dsgtrgbr {
        #[doc = "Counter count down disabled on the rising edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the rising edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgbf_SPEC;
    pub type Dsgtrgbf = crate::EnumBitfieldStruct<u8, Dsgtrgbf_SPEC>;
    impl Dsgtrgbf {
        #[doc = "Counter count down disabled on the falling edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the falling edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgcr_SPEC;
    pub type Dsgtrgcr = crate::EnumBitfieldStruct<u8, Dsgtrgcr_SPEC>;
    impl Dsgtrgcr {
        #[doc = "Counter count down disabled on the rising edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the rising edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgcf_SPEC;
    pub type Dsgtrgcf = crate::EnumBitfieldStruct<u8, Dsgtrgcf_SPEC>;
    impl Dsgtrgcf {
        #[doc = "Counter count down disabled on the falling edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the falling edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgdr_SPEC;
    pub type Dsgtrgdr = crate::EnumBitfieldStruct<u8, Dsgtrgdr_SPEC>;
    impl Dsgtrgdr {
        #[doc = "Counter count down disabled on the rising edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the rising edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgdf_SPEC;
    pub type Dsgtrgdf = crate::EnumBitfieldStruct<u8, Dsgtrgdf_SPEC>;
    impl Dsgtrgdf {
        #[doc = "Counter count down disabled on the falling edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the falling edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscarbl_SPEC;
    pub type Dscarbl = crate::EnumBitfieldStruct<u8, Dscarbl_SPEC>;
    impl Dscarbl {
        #[doc = "Counter count down disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscarbh_SPEC;
    pub type Dscarbh = crate::EnumBitfieldStruct<u8, Dscarbh_SPEC>;
    impl Dscarbh {
        #[doc = "Counter count down disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscafbl_SPEC;
    pub type Dscafbl = crate::EnumBitfieldStruct<u8, Dscafbl_SPEC>;
    impl Dscafbl {
        #[doc = "Counter count down disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscafbh_SPEC;
    pub type Dscafbh = crate::EnumBitfieldStruct<u8, Dscafbh_SPEC>;
    impl Dscafbh {
        #[doc = "Counter count down disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscbral_SPEC;
    pub type Dscbral = crate::EnumBitfieldStruct<u8, Dscbral_SPEC>;
    impl Dscbral {
        #[doc = "Counter count down disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscbrah_SPEC;
    pub type Dscbrah = crate::EnumBitfieldStruct<u8, Dscbrah_SPEC>;
    impl Dscbrah {
        #[doc = "Counter count down disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscbfal_SPEC;
    pub type Dscbfal = crate::EnumBitfieldStruct<u8, Dscbfal_SPEC>;
    impl Dscbfal {
        #[doc = "Counter count down disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscbfah_SPEC;
    pub type Dscbfah = crate::EnumBitfieldStruct<u8, Dscbfah_SPEC>;
    impl Dscbfah {
        #[doc = "Counter count down disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselca_SPEC;
    pub type Dselca = crate::EnumBitfieldStruct<u8, Dselca_SPEC>;
    impl Dselca {
        #[doc = "Counter count down disabled at the ELC_GPTA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled at the ELC_GPTA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselcb_SPEC;
    pub type Dselcb = crate::EnumBitfieldStruct<u8, Dselcb_SPEC>;
    impl Dselcb {
        #[doc = "Counter count down disabled at the ELC_GPTB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled at the ELC_GPTB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselcc_SPEC;
    pub type Dselcc = crate::EnumBitfieldStruct<u8, Dselcc_SPEC>;
    impl Dselcc {
        #[doc = "Counter count down disabled at the ELC_GPTC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled at the ELC_GPTC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselcd_SPEC;
    pub type Dselcd = crate::EnumBitfieldStruct<u8, Dselcd_SPEC>;
    impl Dselcd {
        #[doc = "Counter count down disabled at the ELC_GPTD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled at the ELC_GPTD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselce_SPEC;
    pub type Dselce = crate::EnumBitfieldStruct<u8, Dselce_SPEC>;
    impl Dselce {
        #[doc = "Counter count down disabled at the ELC_GPTE input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled at the ELC_GPTE input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselcf_SPEC;
    pub type Dselcf = crate::EnumBitfieldStruct<u8, Dselcf_SPEC>;
    impl Dselcf {
        #[doc = "Counter count down disabled at the ELC_GPTF input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled at the ELC_GPTF input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselcg_SPEC;
    pub type Dselcg = crate::EnumBitfieldStruct<u8, Dselcg_SPEC>;
    impl Dselcg {
        #[doc = "Counter count down disabled at the ELC_GPTG input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled at the ELC_GPTG input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselch_SPEC;
    pub type Dselch = crate::EnumBitfieldStruct<u8, Dselch_SPEC>;
    impl Dselch {
        #[doc = "Counter count down disabled at the ELC_GPTF input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter count down enabled at the ELC_GPTF input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gticasr_SPEC;
impl crate::sealed::RegSpec for Gticasr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Input Capture Source Select Register A"]
pub type Gticasr = crate::RegValueT<Gticasr_SPEC>;

impl Gticasr {
    #[doc = "GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgar(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gticasr::Asgtrgar,
        gticasr::Asgtrgar,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gticasr::Asgtrgar,
            gticasr::Asgtrgar,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgaf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gticasr::Asgtrgaf,
        gticasr::Asgtrgaf,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gticasr::Asgtrgaf,
            gticasr::Asgtrgaf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gticasr::Asgtrgbr,
        gticasr::Asgtrgbr,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gticasr::Asgtrgbr,
            gticasr::Asgtrgbr,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgbf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gticasr::Asgtrgbf,
        gticasr::Asgtrgbf,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gticasr::Asgtrgbf,
            gticasr::Asgtrgbf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGC Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gticasr::Asgtrgcr,
        gticasr::Asgtrgcr,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gticasr::Asgtrgcr,
            gticasr::Asgtrgcr,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGC Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gticasr::Asgtrgcf,
        gticasr::Asgtrgcf,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gticasr::Asgtrgcf,
            gticasr::Asgtrgcf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGD Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgdr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gticasr::Asgtrgdr,
        gticasr::Asgtrgdr,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gticasr::Asgtrgdr,
            gticasr::Asgtrgdr,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGD Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgdf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gticasr::Asgtrgdf,
        gticasr::Asgtrgdf,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gticasr::Asgtrgdf,
            gticasr::Asgtrgdf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascarbl(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gticasr::Ascarbl,
        gticasr::Ascarbl,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gticasr::Ascarbl,
            gticasr::Ascarbl,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascarbh(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gticasr::Ascarbh,
        gticasr::Ascarbh,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gticasr::Ascarbh,
            gticasr::Ascarbh,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascafbl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gticasr::Ascafbl,
        gticasr::Ascafbl,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gticasr::Ascafbl,
            gticasr::Ascafbl,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascafbh(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gticasr::Ascafbh,
        gticasr::Ascafbh,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gticasr::Ascafbh,
            gticasr::Ascafbh,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbral(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gticasr::Ascbral,
        gticasr::Ascbral,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gticasr::Ascbral,
            gticasr::Ascbral,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbrah(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gticasr::Ascbrah,
        gticasr::Ascbrah,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gticasr::Ascbrah,
            gticasr::Ascbrah,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbfal(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gticasr::Ascbfal,
        gticasr::Ascbfal,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gticasr::Ascbfal,
            gticasr::Ascbfal,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbfah(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gticasr::Ascbfah,
        gticasr::Ascbfah,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gticasr::Ascbfah,
            gticasr::Ascbfah,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTA Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselca(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gticasr::Aselca,
        gticasr::Aselca,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gticasr::Aselca,
            gticasr::Aselca,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTB Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcb(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gticasr::Aselcb,
        gticasr::Aselcb,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gticasr::Aselcb,
            gticasr::Aselcb,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTC Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcc(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gticasr::Aselcc,
        gticasr::Aselcc,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gticasr::Aselcc,
            gticasr::Aselcc,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTD Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcd(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gticasr::Aselcd,
        gticasr::Aselcd,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gticasr::Aselcd,
            gticasr::Aselcd,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTE Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselce(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gticasr::Aselce,
        gticasr::Aselce,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gticasr::Aselce,
            gticasr::Aselce,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTF Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcf(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gticasr::Aselcf,
        gticasr::Aselcf,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gticasr::Aselcf,
            gticasr::Aselcf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTG Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcg(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gticasr::Aselcg,
        gticasr::Aselcg,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gticasr::Aselcg,
            gticasr::Aselcg,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTH Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselch(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gticasr::Aselch,
        gticasr::Aselch,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gticasr::Aselch,
            gticasr::Aselch,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gticasr {
    #[inline(always)]
    fn default() -> Gticasr {
        <crate::RegValueT<Gticasr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gticasr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgar_SPEC;
    pub type Asgtrgar = crate::EnumBitfieldStruct<u8, Asgtrgar_SPEC>;
    impl Asgtrgar {
        #[doc = "GTCCRA input capture disabled on the rising edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the rising edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgaf_SPEC;
    pub type Asgtrgaf = crate::EnumBitfieldStruct<u8, Asgtrgaf_SPEC>;
    impl Asgtrgaf {
        #[doc = "GTCCRA input capture disabled on the falling edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the falling edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgbr_SPEC;
    pub type Asgtrgbr = crate::EnumBitfieldStruct<u8, Asgtrgbr_SPEC>;
    impl Asgtrgbr {
        #[doc = "GTCCRA input capture disabled on the rising edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the rising edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgbf_SPEC;
    pub type Asgtrgbf = crate::EnumBitfieldStruct<u8, Asgtrgbf_SPEC>;
    impl Asgtrgbf {
        #[doc = "GTCCRA input capture disabled on the falling edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the falling edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgcr_SPEC;
    pub type Asgtrgcr = crate::EnumBitfieldStruct<u8, Asgtrgcr_SPEC>;
    impl Asgtrgcr {
        #[doc = "GTCCRA input capture disabled on the rising edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the rising edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgcf_SPEC;
    pub type Asgtrgcf = crate::EnumBitfieldStruct<u8, Asgtrgcf_SPEC>;
    impl Asgtrgcf {
        #[doc = "GTCCRA input capture disabled on the falling edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the falling edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgdr_SPEC;
    pub type Asgtrgdr = crate::EnumBitfieldStruct<u8, Asgtrgdr_SPEC>;
    impl Asgtrgdr {
        #[doc = "GTCCRA input capture disabled on the rising edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the rising edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgdf_SPEC;
    pub type Asgtrgdf = crate::EnumBitfieldStruct<u8, Asgtrgdf_SPEC>;
    impl Asgtrgdf {
        #[doc = "GTCCRA input capture disabled on the falling edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the falling edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascarbl_SPEC;
    pub type Ascarbl = crate::EnumBitfieldStruct<u8, Ascarbl_SPEC>;
    impl Ascarbl {
        #[doc = "GTCCRA input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascarbh_SPEC;
    pub type Ascarbh = crate::EnumBitfieldStruct<u8, Ascarbh_SPEC>;
    impl Ascarbh {
        #[doc = "GTCCRA input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascafbl_SPEC;
    pub type Ascafbl = crate::EnumBitfieldStruct<u8, Ascafbl_SPEC>;
    impl Ascafbl {
        #[doc = "GTCCRA input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascafbh_SPEC;
    pub type Ascafbh = crate::EnumBitfieldStruct<u8, Ascafbh_SPEC>;
    impl Ascafbh {
        #[doc = "GTCCRA input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascbral_SPEC;
    pub type Ascbral = crate::EnumBitfieldStruct<u8, Ascbral_SPEC>;
    impl Ascbral {
        #[doc = "GTCCRA input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascbrah_SPEC;
    pub type Ascbrah = crate::EnumBitfieldStruct<u8, Ascbrah_SPEC>;
    impl Ascbrah {
        #[doc = "GTCCRA input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascbfal_SPEC;
    pub type Ascbfal = crate::EnumBitfieldStruct<u8, Ascbfal_SPEC>;
    impl Ascbfal {
        #[doc = "GTCCRA input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascbfah_SPEC;
    pub type Ascbfah = crate::EnumBitfieldStruct<u8, Ascbfah_SPEC>;
    impl Ascbfah {
        #[doc = "GTCCRA input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselca_SPEC;
    pub type Aselca = crate::EnumBitfieldStruct<u8, Aselca_SPEC>;
    impl Aselca {
        #[doc = "GTCCRA input capture disabled at the ELC_GPTA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled at the ELC_GPTA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcb_SPEC;
    pub type Aselcb = crate::EnumBitfieldStruct<u8, Aselcb_SPEC>;
    impl Aselcb {
        #[doc = "GTCCRA input capture disabled at the ELC_GPTB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled at the ELC_GPTB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcc_SPEC;
    pub type Aselcc = crate::EnumBitfieldStruct<u8, Aselcc_SPEC>;
    impl Aselcc {
        #[doc = "GTCCRA input capture disabled at the ELC_GPTC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled at the ELC_GPTC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcd_SPEC;
    pub type Aselcd = crate::EnumBitfieldStruct<u8, Aselcd_SPEC>;
    impl Aselcd {
        #[doc = "GTCCRA input capture disabled at the ELC_GPTD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled at the ELC_GPTD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselce_SPEC;
    pub type Aselce = crate::EnumBitfieldStruct<u8, Aselce_SPEC>;
    impl Aselce {
        #[doc = "GTCCRA input capture disabled at the ELC_GPTE input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled at the ELC_GPTE input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcf_SPEC;
    pub type Aselcf = crate::EnumBitfieldStruct<u8, Aselcf_SPEC>;
    impl Aselcf {
        #[doc = "GTCCRA input capture disabled at the ELC_GPTF input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled at the ELC_GPTF input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcg_SPEC;
    pub type Aselcg = crate::EnumBitfieldStruct<u8, Aselcg_SPEC>;
    impl Aselcg {
        #[doc = "GTCCRA input capture disabled at the ELC_GPTG input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled at the ELC_GPTG input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselch_SPEC;
    pub type Aselch = crate::EnumBitfieldStruct<u8, Aselch_SPEC>;
    impl Aselch {
        #[doc = "GTCCRA input capture disabled at the ELC_GPTH input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRA input capture enabled at the ELC_GPTH input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gticbsr_SPEC;
impl crate::sealed::RegSpec for Gticbsr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Input Capture Source Select Register B"]
pub type Gticbsr = crate::RegValueT<Gticbsr_SPEC>;

impl Gticbsr {
    #[doc = "GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgar(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgar,
        gticbsr::Bsgtrgar,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgar,
            gticbsr::Bsgtrgar,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgaf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgaf,
        gticbsr::Bsgtrgaf,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgaf,
            gticbsr::Bsgtrgaf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgbr,
        gticbsr::Bsgtrgbr,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgbr,
            gticbsr::Bsgtrgbr,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgbf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgbf,
        gticbsr::Bsgtrgbf,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgbf,
            gticbsr::Bsgtrgbf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGC Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgcr,
        gticbsr::Bsgtrgcr,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgcr,
            gticbsr::Bsgtrgcr,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGC Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgcf,
        gticbsr::Bsgtrgcf,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgcf,
            gticbsr::Bsgtrgcf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGD Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgdr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgdr,
        gticbsr::Bsgtrgdr,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgdr,
            gticbsr::Bsgtrgdr,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGD Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgdf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgdf,
        gticbsr::Bsgtrgdf,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgdf,
            gticbsr::Bsgtrgdf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscarbl(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gticbsr::Bscarbl,
        gticbsr::Bscarbl,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gticbsr::Bscarbl,
            gticbsr::Bscarbl,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscarbh(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gticbsr::Bscarbh,
        gticbsr::Bscarbh,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gticbsr::Bscarbh,
            gticbsr::Bscarbh,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscafbl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gticbsr::Bscafbl,
        gticbsr::Bscafbl,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gticbsr::Bscafbl,
            gticbsr::Bscafbl,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscafbh(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gticbsr::Bscafbh,
        gticbsr::Bscafbh,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gticbsr::Bscafbh,
            gticbsr::Bscafbh,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbral(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gticbsr::Bscbral,
        gticbsr::Bscbral,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gticbsr::Bscbral,
            gticbsr::Bscbral,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbrah(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gticbsr::Bscbrah,
        gticbsr::Bscbrah,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gticbsr::Bscbrah,
            gticbsr::Bscbrah,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbfal(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gticbsr::Bscbfal,
        gticbsr::Bscbfal,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gticbsr::Bscbfal,
            gticbsr::Bscbfal,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbfah(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gticbsr::Bscbfah,
        gticbsr::Bscbfah,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gticbsr::Bscbfah,
            gticbsr::Bscbfah,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTA Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselca(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gticbsr::Bselca,
        gticbsr::Bselca,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gticbsr::Bselca,
            gticbsr::Bselca,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTB Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcb(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gticbsr::Bselcb,
        gticbsr::Bselcb,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gticbsr::Bselcb,
            gticbsr::Bselcb,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTC Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcc(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gticbsr::Bselcc,
        gticbsr::Bselcc,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gticbsr::Bselcc,
            gticbsr::Bselcc,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTD Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcd(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gticbsr::Bselcd,
        gticbsr::Bselcd,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gticbsr::Bselcd,
            gticbsr::Bselcd,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTE Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselce(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gticbsr::Bselce,
        gticbsr::Bselce,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gticbsr::Bselce,
            gticbsr::Bselce,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTF Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcf(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gticbsr::Bselcf,
        gticbsr::Bselcf,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gticbsr::Bselcf,
            gticbsr::Bselcf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTG Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcg(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gticbsr::Bselcg,
        gticbsr::Bselcg,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gticbsr::Bselcg,
            gticbsr::Bselcg,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELC_GPTH Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselch(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gticbsr::Bselch,
        gticbsr::Bselch,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gticbsr::Bselch,
            gticbsr::Bselch,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gticbsr {
    #[inline(always)]
    fn default() -> Gticbsr {
        <crate::RegValueT<Gticbsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gticbsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgar_SPEC;
    pub type Bsgtrgar = crate::EnumBitfieldStruct<u8, Bsgtrgar_SPEC>;
    impl Bsgtrgar {
        #[doc = "GTCCRB input capture disabled on the rising edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the rising edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgaf_SPEC;
    pub type Bsgtrgaf = crate::EnumBitfieldStruct<u8, Bsgtrgaf_SPEC>;
    impl Bsgtrgaf {
        #[doc = "GTCCRB input capture disabled on the falling edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the falling edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgbr_SPEC;
    pub type Bsgtrgbr = crate::EnumBitfieldStruct<u8, Bsgtrgbr_SPEC>;
    impl Bsgtrgbr {
        #[doc = "GTCCRB input capture disabled on the rising edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the rising edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgbf_SPEC;
    pub type Bsgtrgbf = crate::EnumBitfieldStruct<u8, Bsgtrgbf_SPEC>;
    impl Bsgtrgbf {
        #[doc = "GTCCRB input capture disabled on the falling edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the falling edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgcr_SPEC;
    pub type Bsgtrgcr = crate::EnumBitfieldStruct<u8, Bsgtrgcr_SPEC>;
    impl Bsgtrgcr {
        #[doc = "GTCCRB input capture disabled on the rising edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the rising edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgcf_SPEC;
    pub type Bsgtrgcf = crate::EnumBitfieldStruct<u8, Bsgtrgcf_SPEC>;
    impl Bsgtrgcf {
        #[doc = "GTCCRB input capture disabled on the falling edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the falling edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgdr_SPEC;
    pub type Bsgtrgdr = crate::EnumBitfieldStruct<u8, Bsgtrgdr_SPEC>;
    impl Bsgtrgdr {
        #[doc = "GTCCRB input capture disabled on the rising edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the rising edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgdf_SPEC;
    pub type Bsgtrgdf = crate::EnumBitfieldStruct<u8, Bsgtrgdf_SPEC>;
    impl Bsgtrgdf {
        #[doc = "GTCCRB input capture disabled on the falling edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the falling edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscarbl_SPEC;
    pub type Bscarbl = crate::EnumBitfieldStruct<u8, Bscarbl_SPEC>;
    impl Bscarbl {
        #[doc = "GTCCRB input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscarbh_SPEC;
    pub type Bscarbh = crate::EnumBitfieldStruct<u8, Bscarbh_SPEC>;
    impl Bscarbh {
        #[doc = "GTCCRB input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscafbl_SPEC;
    pub type Bscafbl = crate::EnumBitfieldStruct<u8, Bscafbl_SPEC>;
    impl Bscafbl {
        #[doc = "GTCCRB input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscafbh_SPEC;
    pub type Bscafbh = crate::EnumBitfieldStruct<u8, Bscafbh_SPEC>;
    impl Bscafbh {
        #[doc = "GTCCRB input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscbral_SPEC;
    pub type Bscbral = crate::EnumBitfieldStruct<u8, Bscbral_SPEC>;
    impl Bscbral {
        #[doc = "GTCCRB input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscbrah_SPEC;
    pub type Bscbrah = crate::EnumBitfieldStruct<u8, Bscbrah_SPEC>;
    impl Bscbrah {
        #[doc = "GTCCRB input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscbfal_SPEC;
    pub type Bscbfal = crate::EnumBitfieldStruct<u8, Bscbfal_SPEC>;
    impl Bscbfal {
        #[doc = "GTCCRB input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscbfah_SPEC;
    pub type Bscbfah = crate::EnumBitfieldStruct<u8, Bscbfah_SPEC>;
    impl Bscbfah {
        #[doc = "GTCCRB input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselca_SPEC;
    pub type Bselca = crate::EnumBitfieldStruct<u8, Bselca_SPEC>;
    impl Bselca {
        #[doc = "GTCCRB input capture disabled at the ELC_GPTA input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled at the ELC_GPTA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcb_SPEC;
    pub type Bselcb = crate::EnumBitfieldStruct<u8, Bselcb_SPEC>;
    impl Bselcb {
        #[doc = "GTCCRB input capture disabled at the ELC_GPTB input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled at the ELC_GPTB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcc_SPEC;
    pub type Bselcc = crate::EnumBitfieldStruct<u8, Bselcc_SPEC>;
    impl Bselcc {
        #[doc = "GTCCRB input capture disabled at the ELC_GPTC input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled at the ELC_GPTC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcd_SPEC;
    pub type Bselcd = crate::EnumBitfieldStruct<u8, Bselcd_SPEC>;
    impl Bselcd {
        #[doc = "GTCCRB input capture disabled at the ELC_GPTD input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled at the ELC_GPTD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselce_SPEC;
    pub type Bselce = crate::EnumBitfieldStruct<u8, Bselce_SPEC>;
    impl Bselce {
        #[doc = "GTCCRB input capture disabled at the ELC_GPTE input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled at the ELC_GPTE input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcf_SPEC;
    pub type Bselcf = crate::EnumBitfieldStruct<u8, Bselcf_SPEC>;
    impl Bselcf {
        #[doc = "GTCCRB input capture disabled at the ELC_GPTF input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled at the ELC_GPTF input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcg_SPEC;
    pub type Bselcg = crate::EnumBitfieldStruct<u8, Bselcg_SPEC>;
    impl Bselcg {
        #[doc = "GTCCRB input capture disabled at the ELC_GPTG input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled at the ELC_GPTG input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselch_SPEC;
    pub type Bselch = crate::EnumBitfieldStruct<u8, Bselch_SPEC>;
    impl Bselch {
        #[doc = "GTCCRB input capture disabled at the ELC_GPTH input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCCRB input capture enabled at the ELC_GPTH input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtcr_SPEC;
impl crate::sealed::RegSpec for Gtcr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Control Register"]
pub type Gtcr = crate::RegValueT<Gtcr_SPEC>;

impl Gtcr {
    #[doc = "Count Start"]
    #[inline(always)]
    pub fn cst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtcr::Cst,
        gtcr::Cst,
        Gtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtcr::Cst,
            gtcr::Cst,
            Gtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mode Select"]
    #[inline(always)]
    pub fn md(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, gtcr::Md, gtcr::Md, Gtcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            gtcr::Md,
            gtcr::Md,
            Gtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timer Prescaler Select"]
    #[inline(always)]
    pub fn tpcs(
        self,
    ) -> crate::common::RegisterField<
        23,
        0xf,
        1,
        0,
        gtcr::Tpcs,
        gtcr::Tpcs,
        Gtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0xf,
            1,
            0,
            gtcr::Tpcs,
            gtcr::Tpcs,
            Gtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtcr {
    #[inline(always)]
    fn default() -> Gtcr {
        <crate::RegValueT<Gtcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cst_SPEC;
    pub type Cst = crate::EnumBitfieldStruct<u8, Cst_SPEC>;
    impl Cst {
        #[doc = "Count operation is stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Count operation is performed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md_SPEC;
    pub type Md = crate::EnumBitfieldStruct<u8, Md_SPEC>;
    impl Md {
        #[doc = "Saw-wave PWM mode (single buffer or double buffer possible)"]
        pub const _000: Self = Self::new(0);

        #[doc = "Saw-wave one-shot pulse mode (fixed buffer operation)"]
        pub const _001: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _010: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _011: Self = Self::new(3);

        #[doc = "Triangle-wave PWM mode 1 (32-bit transfer at trough) (single buffer or double buffer is possible)"]
        pub const _100: Self = Self::new(4);

        #[doc = "Triangle-wave PWM mode 2 (32-bit transfer at crest and trough) (single buffer or double buffer is possible)"]
        pub const _101: Self = Self::new(5);

        #[doc = "Triangle-wave PWM mode 3 (64-bit transfer at trough) (fixed buffer operation)"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tpcs_SPEC;
    pub type Tpcs = crate::EnumBitfieldStruct<u8, Tpcs_SPEC>;
    impl Tpcs {
        #[doc = "PCLKD/1"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "PCLKD/2"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "PCLKD/4"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "PCLKD/8"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "PCLKD/16"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "PCLKD/32"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "PCLKD/64"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "PCLKD/256"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "Setting prohibited"]
        pub const _0_X_9: Self = Self::new(9);

        #[doc = "PCLKD/1024"]
        pub const _0_X_A: Self = Self::new(10);

        #[doc = "Setting prohibited"]
        pub const _0_X_B: Self = Self::new(11);

        #[doc = "GTETRGA (Via the POEG)"]
        pub const _0_X_C: Self = Self::new(12);

        #[doc = "GTETRGB (Via the POEG)"]
        pub const _0_X_D: Self = Self::new(13);

        #[doc = "GTETRGC (Via the POEG)"]
        pub const _0_X_E: Self = Self::new(14);

        #[doc = "GTETRGD (Via the POEG)"]
        pub const _0_X_F: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtintad_SPEC;
impl crate::sealed::RegSpec for Gtintad_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Interrupt Output Setting Register"]
pub type Gtintad = crate::RegValueT<Gtintad_SPEC>;

impl Gtintad {
    #[doc = "Output Disable Source Select"]
    #[inline(always)]
    pub fn grp(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        gtintad::Grp,
        gtintad::Grp,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            gtintad::Grp,
            gtintad::Grp,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub fn grpabh(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        gtintad::Grpabh,
        gtintad::Grpabh,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            gtintad::Grpabh,
            gtintad::Grpabh,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub fn grpabl(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        gtintad::Grpabl,
        gtintad::Grpabl,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            gtintad::Grpabl,
            gtintad::Grpabl,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtintad {
    #[inline(always)]
    fn default() -> Gtintad {
        <crate::RegValueT<Gtintad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtintad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grp_SPEC;
    pub type Grp = crate::EnumBitfieldStruct<u8, Grp_SPEC>;
    impl Grp {
        #[doc = "Group A output disable request is selected"]
        pub const _00: Self = Self::new(0);

        #[doc = "Group B output disable request is selected"]
        pub const _01: Self = Self::new(1);

        #[doc = "Group C output disable request is selected"]
        pub const _10: Self = Self::new(2);

        #[doc = "Group D output disable request is selected"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grpabh_SPEC;
    pub type Grpabh = crate::EnumBitfieldStruct<u8, Grpabh_SPEC>;
    impl Grpabh {
        #[doc = "Same time output level high disable request disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Same time output level high disable request enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grpabl_SPEC;
    pub type Grpabl = crate::EnumBitfieldStruct<u8, Grpabl_SPEC>;
    impl Grpabl {
        #[doc = "Same time output level low disable request disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Same time output level low disable request enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadtra_SPEC;
impl crate::sealed::RegSpec for Gtadtra_SPEC {
    type DataType = u32;
}

#[doc = "A/D Conversion Start Request Timing Register A"]
pub type Gtadtra = crate::RegValueT<Gtadtra_SPEC>;

impl NoBitfieldReg<Gtadtra_SPEC> for Gtadtra {}
impl ::core::default::Default for Gtadtra {
    #[inline(always)]
    fn default() -> Gtadtra {
        <crate::RegValueT<Gtadtra_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadtbra_SPEC;
impl crate::sealed::RegSpec for Gtadtbra_SPEC {
    type DataType = u32;
}

#[doc = "A/D Conversion Start Request Timing Buffer Register A"]
pub type Gtadtbra = crate::RegValueT<Gtadtbra_SPEC>;

impl NoBitfieldReg<Gtadtbra_SPEC> for Gtadtbra {}
impl ::core::default::Default for Gtadtbra {
    #[inline(always)]
    fn default() -> Gtadtbra {
        <crate::RegValueT<Gtadtbra_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadtdbra_SPEC;
impl crate::sealed::RegSpec for Gtadtdbra_SPEC {
    type DataType = u32;
}

#[doc = "A/D Conversion Start Request Timing Double-Buffer Register A"]
pub type Gtadtdbra = crate::RegValueT<Gtadtdbra_SPEC>;

impl NoBitfieldReg<Gtadtdbra_SPEC> for Gtadtdbra {}
impl ::core::default::Default for Gtadtdbra {
    #[inline(always)]
    fn default() -> Gtadtdbra {
        <crate::RegValueT<Gtadtdbra_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadtrb_SPEC;
impl crate::sealed::RegSpec for Gtadtrb_SPEC {
    type DataType = u32;
}

#[doc = "A/D Conversion Start Request Timing Register B"]
pub type Gtadtrb = crate::RegValueT<Gtadtrb_SPEC>;

impl NoBitfieldReg<Gtadtrb_SPEC> for Gtadtrb {}
impl ::core::default::Default for Gtadtrb {
    #[inline(always)]
    fn default() -> Gtadtrb {
        <crate::RegValueT<Gtadtrb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadtbrb_SPEC;
impl crate::sealed::RegSpec for Gtadtbrb_SPEC {
    type DataType = u32;
}

#[doc = "A/D Conversion Start Request Timing Buffer Register B"]
pub type Gtadtbrb = crate::RegValueT<Gtadtbrb_SPEC>;

impl NoBitfieldReg<Gtadtbrb_SPEC> for Gtadtbrb {}
impl ::core::default::Default for Gtadtbrb {
    #[inline(always)]
    fn default() -> Gtadtbrb {
        <crate::RegValueT<Gtadtbrb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadtdbrb_SPEC;
impl crate::sealed::RegSpec for Gtadtdbrb_SPEC {
    type DataType = u32;
}

#[doc = "A/D Conversion Start Request Timing Double-Buffer Register B"]
pub type Gtadtdbrb = crate::RegValueT<Gtadtdbrb_SPEC>;

impl NoBitfieldReg<Gtadtdbrb_SPEC> for Gtadtdbrb {}
impl ::core::default::Default for Gtadtdbrb {
    #[inline(always)]
    fn default() -> Gtadtdbrb {
        <crate::RegValueT<Gtadtdbrb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadsmr_SPEC;
impl crate::sealed::RegSpec for Gtadsmr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer A/D Conversion Start Request Signal Monitoring Register"]
pub type Gtadsmr = crate::RegValueT<Gtadsmr_SPEC>;

impl Gtadsmr {
    #[doc = "A/D Conversion Start Request Signal Monitor 0 Selection"]
    #[inline(always)]
    pub fn adsms0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        gtadsmr::Adsms0,
        gtadsmr::Adsms0,
        Gtadsmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            gtadsmr::Adsms0,
            gtadsmr::Adsms0,
            Gtadsmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Start Request Signal Monitor 0 Output Enabling"]
    #[inline(always)]
    pub fn adsmen0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtadsmr::Adsmen0,
        gtadsmr::Adsmen0,
        Gtadsmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtadsmr::Adsmen0,
            gtadsmr::Adsmen0,
            Gtadsmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Start Request Signal Monitor 1 Selection"]
    #[inline(always)]
    pub fn adsms1(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        gtadsmr::Adsms1,
        gtadsmr::Adsms1,
        Gtadsmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            gtadsmr::Adsms1,
            gtadsmr::Adsms1,
            Gtadsmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Start Request Signal Monitor 1 Output Enabling"]
    #[inline(always)]
    pub fn adsmen1(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gtadsmr::Adsmen1,
        gtadsmr::Adsmen1,
        Gtadsmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gtadsmr::Adsmen1,
            gtadsmr::Adsmen1,
            Gtadsmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtadsmr {
    #[inline(always)]
    fn default() -> Gtadsmr {
        <crate::RegValueT<Gtadsmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtadsmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsms0_SPEC;
    pub type Adsms0 = crate::EnumBitfieldStruct<u8, Adsms0_SPEC>;
    impl Adsms0 {
        #[doc = "A/D conversion start request signal generated by the GTADTRA register during up-counting."]
        pub const _00: Self = Self::new(0);

        #[doc = "A/D conversion start request signal generated by the GTADTRA register during down-counting."]
        pub const _01: Self = Self::new(1);

        #[doc = "A/D conversion start request signal generated by the GTADTRB register during up-counting."]
        pub const _10: Self = Self::new(2);

        #[doc = "A/D conversion start request signal generated by the GTADTRB register during down-counting."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsmen0_SPEC;
    pub type Adsmen0 = crate::EnumBitfieldStruct<u8, Adsmen0_SPEC>;
    impl Adsmen0 {
        #[doc = "Output of A/D conversion start request signal monitor 0 is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Output of A/D conversion start request signal monitor 0 is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsms1_SPEC;
    pub type Adsms1 = crate::EnumBitfieldStruct<u8, Adsms1_SPEC>;
    impl Adsms1 {
        #[doc = "A/D conversion start request signal generated by the GTADTRA register during up-counting."]
        pub const _00: Self = Self::new(0);

        #[doc = "A/D conversion start request signal generated by the GTADTRA register during down-counting."]
        pub const _01: Self = Self::new(1);

        #[doc = "A/D conversion start request signal generated by the GTADTRB register during up-counting."]
        pub const _10: Self = Self::new(2);

        #[doc = "A/D conversion start request signal generated by the GTADTRB register during down-counting."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsmen1_SPEC;
    pub type Adsmen1 = crate::EnumBitfieldStruct<u8, Adsmen1_SPEC>;
    impl Adsmen1 {
        #[doc = "Output of A/D conversion start request signal monitor 1 is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Output of A/D conversion start request signal monitor 1 is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gticlf_SPEC;
impl crate::sealed::RegSpec for Gticlf_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Inter Channel Logical Operation Function Setting Register"]
pub type Gticlf = crate::RegValueT<Gticlf_SPEC>;

impl Gticlf {
    #[doc = "GTIOCnA Output Logical Operation Function Select"]
    #[inline(always)]
    pub fn iclfa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        gticlf::Iclfa,
        gticlf::Iclfa,
        Gticlf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gticlf::Iclfa,
            gticlf::Iclfa,
            Gticlf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Inter Channel Signal C Select"]
    #[inline(always)]
    pub fn iclfselc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3f,
        1,
        0,
        gticlf::Iclfselc,
        gticlf::Iclfselc,
        Gticlf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3f,
            1,
            0,
            gticlf::Iclfselc,
            gticlf::Iclfselc,
            Gticlf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Output Logical Operation Function Select"]
    #[inline(always)]
    pub fn iclfb(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        gticlf::Iclfb,
        gticlf::Iclfb,
        Gticlf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            gticlf::Iclfb,
            gticlf::Iclfb,
            Gticlf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Inter Channel Signal D Select"]
    #[inline(always)]
    pub fn iclfseld(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3f,
        1,
        0,
        gticlf::Iclfseld,
        gticlf::Iclfseld,
        Gticlf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3f,
            1,
            0,
            gticlf::Iclfseld,
            gticlf::Iclfseld,
            Gticlf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gticlf {
    #[inline(always)]
    fn default() -> Gticlf {
        <crate::RegValueT<Gticlf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gticlf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iclfa_SPEC;
    pub type Iclfa = crate::EnumBitfieldStruct<u8, Iclfa_SPEC>;
    impl Iclfa {
        #[doc = "A (no delay)"]
        pub const _000: Self = Self::new(0);

        #[doc = "NOT A (no delay)"]
        pub const _001: Self = Self::new(1);

        #[doc = "C (1PCLKD delay)"]
        pub const _010: Self = Self::new(2);

        #[doc = "NOT C (1PCLKD delay)"]
        pub const _011: Self = Self::new(3);

        #[doc = "A AND C (1PCLKD delay)"]
        pub const _100: Self = Self::new(4);

        #[doc = "A OR C (1PCLKD delay)"]
        pub const _101: Self = Self::new(5);

        #[doc = "A EXOR C (1PCLKD delay)"]
        pub const _110: Self = Self::new(6);

        #[doc = "A NOR C (1PCLKD delay)"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iclfselc_SPEC;
    pub type Iclfselc = crate::EnumBitfieldStruct<u8, Iclfselc_SPEC>;
    impl Iclfselc {
        #[doc = "GTIOC0A"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "GTIOC0B"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "GTIOC1A"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "GTIOC1B"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "GTIOC2A"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "GTIOC2B"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "GTIOC3A"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "GTIOC3B"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "GTIOC4A"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "GTIOC4B"]
        pub const _0_X_09: Self = Self::new(9);

        #[doc = "GTIOC5A"]
        pub const _0_X_0_A: Self = Self::new(10);

        #[doc = "GTIOC5B"]
        pub const _0_X_0_B: Self = Self::new(11);

        #[doc = "GTIOC6A"]
        pub const _0_X_0_C: Self = Self::new(12);

        #[doc = "GTIOC6B"]
        pub const _0_X_0_D: Self = Self::new(13);

        #[doc = "GTIOC7A"]
        pub const _0_X_0_E: Self = Self::new(14);

        #[doc = "GTIOC7B"]
        pub const _0_X_0_F: Self = Self::new(15);

        #[doc = "GTIOC8A"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "GTIOC8B"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "GTIOC9A"]
        pub const _0_X_12: Self = Self::new(18);

        #[doc = "GTIOC9B"]
        pub const _0_X_13: Self = Self::new(19);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iclfb_SPEC;
    pub type Iclfb = crate::EnumBitfieldStruct<u8, Iclfb_SPEC>;
    impl Iclfb {
        #[doc = "B (no delay)"]
        pub const _000: Self = Self::new(0);

        #[doc = "NOT B (no delay)"]
        pub const _001: Self = Self::new(1);

        #[doc = "D (1PCLKD delay)"]
        pub const _010: Self = Self::new(2);

        #[doc = "NOT D (1PCLKD delay)"]
        pub const _011: Self = Self::new(3);

        #[doc = "B AND D (1PCLKD delay)"]
        pub const _100: Self = Self::new(4);

        #[doc = "B OR D (1PCLKDn delay)"]
        pub const _101: Self = Self::new(5);

        #[doc = "B EXOR D (1PCLKD delay)"]
        pub const _110: Self = Self::new(6);

        #[doc = "B NOR D (1PCLKD delay)"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iclfseld_SPEC;
    pub type Iclfseld = crate::EnumBitfieldStruct<u8, Iclfseld_SPEC>;
    impl Iclfseld {
        #[doc = "GTIOC0A"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "GTIOC0B"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "GTIOC1A"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "GTIOC1B"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "GTIOC2A"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "GTIOC2B"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "GTIOC3A"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "GTIOC3B"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "GTIOC4A"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "GTIOC4B"]
        pub const _0_X_09: Self = Self::new(9);

        #[doc = "GTIOC5A"]
        pub const _0_X_0_A: Self = Self::new(10);

        #[doc = "GTIOC5B"]
        pub const _0_X_0_B: Self = Self::new(11);

        #[doc = "GTIOC6A"]
        pub const _0_X_0_C: Self = Self::new(12);

        #[doc = "GTIOC6B"]
        pub const _0_X_0_D: Self = Self::new(13);

        #[doc = "GTIOC7A"]
        pub const _0_X_0_E: Self = Self::new(14);

        #[doc = "GTIOC7B"]
        pub const _0_X_0_F: Self = Self::new(15);

        #[doc = "GTIOC8A"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "GTIOC8B"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "GTIOC9A"]
        pub const _0_X_12: Self = Self::new(18);

        #[doc = "GTIOC9B"]
        pub const _0_X_13: Self = Self::new(19);
    }
}
