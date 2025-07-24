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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:49:18 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Watchdog Timer"]
unsafe impl ::core::marker::Send for super::Wdt {}
unsafe impl ::core::marker::Sync for super::Wdt {}
impl super::Wdt {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "WDT Refresh Register"]
    #[inline(always)]
    pub const fn wdtrr(&self) -> &'static crate::common::Reg<self::Wdtrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wdtrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "WDT Control Register"]
    #[inline(always)]
    pub const fn wdtcr(&self) -> &'static crate::common::Reg<self::Wdtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wdtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "WDT Status Register"]
    #[inline(always)]
    pub const fn wdtsr(&self) -> &'static crate::common::Reg<self::Wdtsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wdtsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "WDT Reset Control Register"]
    #[inline(always)]
    pub const fn wdtrcr(
        &self,
    ) -> &'static crate::common::Reg<self::Wdtrcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wdtrcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "WDT Count Stop Control Register"]
    #[inline(always)]
    pub const fn wdtcstpr(
        &self,
    ) -> &'static crate::common::Reg<self::Wdtcstpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wdtcstpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtrr_SPEC;
impl crate::sealed::RegSpec for Wdtrr_SPEC {
    type DataType = u8;
}

#[doc = "WDT Refresh Register"]
pub type Wdtrr = crate::RegValueT<Wdtrr_SPEC>;

impl Wdtrr {
    #[doc = "WDTRR is an 8-bit register that  refreshes the down-counter of the WDT."]
    #[inline(always)]
    pub fn wdtrr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Wdtrr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Wdtrr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Wdtrr {
    #[inline(always)]
    fn default() -> Wdtrr {
        <crate::RegValueT<Wdtrr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtcr_SPEC;
impl crate::sealed::RegSpec for Wdtcr_SPEC {
    type DataType = u16;
}

#[doc = "WDT Control Register"]
pub type Wdtcr = crate::RegValueT<Wdtcr_SPEC>;

impl Wdtcr {
    #[doc = "Window Start Position Selection"]
    #[inline(always)]
    pub fn rpss(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        wdtcr::Rpss,
        wdtcr::Rpss,
        Wdtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            wdtcr::Rpss,
            wdtcr::Rpss,
            Wdtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Window End Position Selection"]
    #[inline(always)]
    pub fn rpes(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        wdtcr::Rpes,
        wdtcr::Rpes,
        Wdtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            wdtcr::Rpes,
            wdtcr::Rpes,
            Wdtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Division Ratio Selection"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        wdtcr::Cks,
        wdtcr::Cks,
        Wdtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            wdtcr::Cks,
            wdtcr::Cks,
            Wdtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Wdtcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Wdtcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Timeout Period Selection"]
    #[inline(always)]
    pub fn tops(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        wdtcr::Tops,
        wdtcr::Tops,
        Wdtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            wdtcr::Tops,
            wdtcr::Tops,
            Wdtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Wdtcr {
    #[inline(always)]
    fn default() -> Wdtcr {
        <crate::RegValueT<Wdtcr_SPEC> as RegisterValue<_>>::new(13299)
    }
}
pub mod wdtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpss_SPEC;
    pub type Rpss = crate::EnumBitfieldStruct<u8, Rpss_SPEC>;
    impl Rpss {
        #[doc = "25 percent"]
        pub const _00: Self = Self::new(0);

        #[doc = "50 percent"]
        pub const _01: Self = Self::new(1);

        #[doc = "75 percent"]
        pub const _10: Self = Self::new(2);

        #[doc = "100 percent (window start position is not specified)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpes_SPEC;
    pub type Rpes = crate::EnumBitfieldStruct<u8, Rpes_SPEC>;
    impl Rpes {
        #[doc = "75 percent"]
        pub const _00: Self = Self::new(0);

        #[doc = "50 percent"]
        pub const _01: Self = Self::new(1);

        #[doc = "25 percent"]
        pub const _10: Self = Self::new(2);

        #[doc = "0 percent (window end position is not specified)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "PCLK/4"]
        pub const _0001: Self = Self::new(1);

        #[doc = "PCLK/64"]
        pub const _0100: Self = Self::new(4);

        #[doc = "PCLK/128"]
        pub const _1111: Self = Self::new(15);

        #[doc = "PCLK/512"]
        pub const _0110: Self = Self::new(6);

        #[doc = "PCLK/2048"]
        pub const _0111: Self = Self::new(7);

        #[doc = "PCLK/8192"]
        pub const _1000: Self = Self::new(8);

        #[doc = "setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tops_SPEC;
    pub type Tops = crate::EnumBitfieldStruct<u8, Tops_SPEC>;
    impl Tops {
        #[doc = "1,024 cycles (03FFh)"]
        pub const _00: Self = Self::new(0);

        #[doc = "4,096 cycles (0FFFh)"]
        pub const _01: Self = Self::new(1);

        #[doc = "8,192 cycles (1FFFh)"]
        pub const _10: Self = Self::new(2);

        #[doc = "16,384 cycles (3FFFh)"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtsr_SPEC;
impl crate::sealed::RegSpec for Wdtsr_SPEC {
    type DataType = u16;
}

#[doc = "WDT Status Register"]
pub type Wdtsr = crate::RegValueT<Wdtsr_SPEC>;

impl Wdtsr {
    #[doc = "Refresh Error Flag"]
    #[inline(always)]
    pub fn refef(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        wdtsr::Refef,
        wdtsr::Refef,
        Wdtsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            wdtsr::Refef,
            wdtsr::Refef,
            Wdtsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Underflow Flag"]
    #[inline(always)]
    pub fn undff(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        wdtsr::Undff,
        wdtsr::Undff,
        Wdtsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            wdtsr::Undff,
            wdtsr::Undff,
            Wdtsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Down-Counter Value\nValue counted by the down-counter"]
    #[inline(always)]
    pub fn cntval(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, Wdtsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16,u16,Wdtsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Wdtsr {
    #[inline(always)]
    fn default() -> Wdtsr {
        <crate::RegValueT<Wdtsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod wdtsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Refef_SPEC;
    pub type Refef = crate::EnumBitfieldStruct<u8, Refef_SPEC>;
    impl Refef {
        #[doc = "No refresh error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Refresh error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Undff_SPEC;
    pub type Undff = crate::EnumBitfieldStruct<u8, Undff_SPEC>;
    impl Undff {
        #[doc = "No underflow occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Underflow occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtrcr_SPEC;
impl crate::sealed::RegSpec for Wdtrcr_SPEC {
    type DataType = u8;
}

#[doc = "WDT Reset Control Register"]
pub type Wdtrcr = crate::RegValueT<Wdtrcr_SPEC>;

impl Wdtrcr {
    #[doc = "Reset Interrupt Request Selection"]
    #[inline(always)]
    pub fn rstirqs(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        wdtrcr::Rstirqs,
        wdtrcr::Rstirqs,
        Wdtrcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            wdtrcr::Rstirqs,
            wdtrcr::Rstirqs,
            Wdtrcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Wdtrcr {
    #[inline(always)]
    fn default() -> Wdtrcr {
        <crate::RegValueT<Wdtrcr_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod wdtrcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstirqs_SPEC;
    pub type Rstirqs = crate::EnumBitfieldStruct<u8, Rstirqs_SPEC>;
    impl Rstirqs {
        #[doc = "Non-maskable interrupt request or interrupt request output is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset output is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtcstpr_SPEC;
impl crate::sealed::RegSpec for Wdtcstpr_SPEC {
    type DataType = u8;
}

#[doc = "WDT Count Stop Control Register"]
pub type Wdtcstpr = crate::RegValueT<Wdtcstpr_SPEC>;

impl Wdtcstpr {
    #[doc = "Sleep-Mode Count Stop Control"]
    #[inline(always)]
    pub fn slcstp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        wdtcstpr::Slcstp,
        wdtcstpr::Slcstp,
        Wdtcstpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            wdtcstpr::Slcstp,
            wdtcstpr::Slcstp,
            Wdtcstpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Wdtcstpr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Wdtcstpr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Wdtcstpr {
    #[inline(always)]
    fn default() -> Wdtcstpr {
        <crate::RegValueT<Wdtcstpr_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod wdtcstpr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slcstp_SPEC;
    pub type Slcstp = crate::EnumBitfieldStruct<u8, Slcstp_SPEC>;
    impl Slcstp {
        #[doc = "Count stop is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Count is stopped at a transition to sleep mode."]
        pub const _1: Self = Self::new(1);
    }
}
