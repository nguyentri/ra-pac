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
#[doc = r"Ultra-Low Power Timer 0"]
unsafe impl ::core::marker::Send for super::Ulpt0Ns {}
unsafe impl ::core::marker::Sync for super::Ulpt0Ns {}
impl super::Ulpt0Ns {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "ULPT Counter Register"]
    #[inline(always)]
    pub const fn ulptcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Ulptcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ulptcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "ULPT Compare Match A Register"]
    #[inline(always)]
    pub const fn ulptcma(
        &self,
    ) -> &'static crate::common::Reg<self::Ulptcma_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ulptcma_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "ULPT Compare Match B Register"]
    #[inline(always)]
    pub const fn ulptcmb(
        &self,
    ) -> &'static crate::common::Reg<self::Ulptcmb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ulptcmb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "ULPT Control Register"]
    #[inline(always)]
    pub const fn ulptcr(
        &self,
    ) -> &'static crate::common::Reg<self::Ulptcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ulptcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "ULPT Mode Register 1"]
    #[inline(always)]
    pub const fn ulptmr1(
        &self,
    ) -> &'static crate::common::Reg<self::Ulptmr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ulptmr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[doc = "ULPT Mode Register 2"]
    #[inline(always)]
    pub const fn ulptmr2(
        &self,
    ) -> &'static crate::common::Reg<self::Ulptmr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ulptmr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "ULPT Mode Register 3"]
    #[inline(always)]
    pub const fn ulptmr3(
        &self,
    ) -> &'static crate::common::Reg<self::Ulptmr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ulptmr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(15usize),
            )
        }
    }

    #[doc = "ULPT I/O Control Register"]
    #[inline(always)]
    pub const fn ulptioc(
        &self,
    ) -> &'static crate::common::Reg<self::Ulptioc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ulptioc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "ULPT Event Pin Select Register"]
    #[inline(always)]
    pub const fn ulptisr(
        &self,
    ) -> &'static crate::common::Reg<self::Ulptisr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ulptisr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17usize),
            )
        }
    }

    #[doc = "ULPT Compare Match Function Select Register"]
    #[inline(always)]
    pub const fn ulptcmsr(
        &self,
    ) -> &'static crate::common::Reg<self::Ulptcmsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ulptcmsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulptcnt_SPEC;
impl crate::sealed::RegSpec for Ulptcnt_SPEC {
    type DataType = u32;
}

#[doc = "ULPT Counter Register"]
pub type Ulptcnt = crate::RegValueT<Ulptcnt_SPEC>;

impl Ulptcnt {
    #[doc = "Setting range : 0x00000000 to 0xFFFFFFFF"]
    #[inline(always)]
    pub fn ulptcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ulptcnt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ulptcnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ulptcnt {
    #[inline(always)]
    fn default() -> Ulptcnt {
        <crate::RegValueT<Ulptcnt_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulptcma_SPEC;
impl crate::sealed::RegSpec for Ulptcma_SPEC {
    type DataType = u32;
}

#[doc = "ULPT Compare Match A Register"]
pub type Ulptcma = crate::RegValueT<Ulptcma_SPEC>;

impl Ulptcma {
    #[doc = "32-bit Compare Match A Data"]
    #[inline(always)]
    pub fn ulptcma(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ulptcma_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ulptcma_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ulptcma {
    #[inline(always)]
    fn default() -> Ulptcma {
        <crate::RegValueT<Ulptcma_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulptcmb_SPEC;
impl crate::sealed::RegSpec for Ulptcmb_SPEC {
    type DataType = u32;
}

#[doc = "ULPT Compare Match B Register"]
pub type Ulptcmb = crate::RegValueT<Ulptcmb_SPEC>;

impl Ulptcmb {
    #[doc = "32-bit Compare Match B Data"]
    #[inline(always)]
    pub fn ulptcmb(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ulptcmb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ulptcmb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ulptcmb {
    #[inline(always)]
    fn default() -> Ulptcmb {
        <crate::RegValueT<Ulptcmb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulptcr_SPEC;
impl crate::sealed::RegSpec for Ulptcr_SPEC {
    type DataType = u8;
}

#[doc = "ULPT Control Register"]
pub type Ulptcr = crate::RegValueT<Ulptcr_SPEC>;

impl Ulptcr {
    #[doc = "Counter Start"]
    #[inline(always)]
    pub fn tstart(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ulptcr::Tstart,
        ulptcr::Tstart,
        Ulptcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ulptcr::Tstart,
            ulptcr::Tstart,
            Ulptcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Counter Status Flag"]
    #[inline(always)]
    pub fn tcstf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ulptcr::Tcstf,
        ulptcr::Tcstf,
        Ulptcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ulptcr::Tcstf,
            ulptcr::Tcstf,
            Ulptcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Counter Forcible Stop"]
    #[inline(always)]
    pub fn tstop(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ulptcr::Tstop,
        ulptcr::Tstop,
        Ulptcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ulptcr::Tstop,
            ulptcr::Tstop,
            Ulptcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Underflow Flag"]
    #[inline(always)]
    pub fn tundf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ulptcr::Tundf,
        ulptcr::Tundf,
        Ulptcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ulptcr::Tundf,
            ulptcr::Tundf,
            Ulptcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Match A Flag"]
    #[inline(always)]
    pub fn tcmaf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ulptcr::Tcmaf,
        ulptcr::Tcmaf,
        Ulptcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ulptcr::Tcmaf,
            ulptcr::Tcmaf,
            Ulptcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Match B Flag"]
    #[inline(always)]
    pub fn tcmbf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ulptcr::Tcmbf,
        ulptcr::Tcmbf,
        Ulptcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ulptcr::Tcmbf,
            ulptcr::Tcmbf,
            Ulptcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ulptcr {
    #[inline(always)]
    fn default() -> Ulptcr {
        <crate::RegValueT<Ulptcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ulptcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tstart_SPEC;
    pub type Tstart = crate::EnumBitfieldStruct<u8, Tstart_SPEC>;
    impl Tstart {
        #[doc = "Stop the counter."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start the counter."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcstf_SPEC;
    pub type Tcstf = crate::EnumBitfieldStruct<u8, Tcstf_SPEC>;
    impl Tcstf {
        #[doc = "Counter stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter running"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tstop_SPEC;
    pub type Tstop = crate::EnumBitfieldStruct<u8, Tstop_SPEC>;
    impl Tstop {
        #[doc = "Writing is invalid."]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the counter forcibly."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tundf_SPEC;
    pub type Tundf = crate::EnumBitfieldStruct<u8, Tundf_SPEC>;
    impl Tundf {
        #[doc = "No underflow occurred (counter ≠ 0x00000000)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Underflow occurred (counter = 0x00000000)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcmaf_SPEC;
    pub type Tcmaf = crate::EnumBitfieldStruct<u8, Tcmaf_SPEC>;
    impl Tcmaf {
        #[doc = "Not matched (counter ≠ ULPTCMA\\[31:0\\])"]
        pub const _0: Self = Self::new(0);

        #[doc = "Matched (counter = ULPTCMA\\[31:0\\])"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcmbf_SPEC;
    pub type Tcmbf = crate::EnumBitfieldStruct<u8, Tcmbf_SPEC>;
    impl Tcmbf {
        #[doc = "Not matched (counter ≠ ULPTCMB\\[31:0\\])"]
        pub const _0: Self = Self::new(0);

        #[doc = "Matched (counter = ULPTCMB\\[31:0\\])"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulptmr1_SPEC;
impl crate::sealed::RegSpec for Ulptmr1_SPEC {
    type DataType = u8;
}

#[doc = "ULPT Mode Register 1"]
pub type Ulptmr1 = crate::RegValueT<Ulptmr1_SPEC>;

impl Ulptmr1 {
    #[doc = "Operating Mode"]
    #[inline(always)]
    pub fn tmod1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ulptmr1::Tmod1,
        ulptmr1::Tmod1,
        Ulptmr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ulptmr1::Tmod1,
            ulptmr1::Tmod1,
            Ulptmr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPTEVIn Edge Polarity"]
    #[inline(always)]
    pub fn tedgpl(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ulptmr1::Tedgpl,
        ulptmr1::Tedgpl,
        Ulptmr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ulptmr1::Tedgpl,
            ulptmr1::Tedgpl,
            Ulptmr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Count Source"]
    #[inline(always)]
    pub fn tck1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ulptmr1::Tck1,
        ulptmr1::Tck1,
        Ulptmr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ulptmr1::Tck1,
            ulptmr1::Tck1,
            Ulptmr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ulptmr1 {
    #[inline(always)]
    fn default() -> Ulptmr1 {
        <crate::RegValueT<Ulptmr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ulptmr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmod1_SPEC;
    pub type Tmod1 = crate::EnumBitfieldStruct<u8, Tmod1_SPEC>;
    impl Tmod1 {
        #[doc = "Timer mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Event counter mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tedgpl_SPEC;
    pub type Tedgpl = crate::EnumBitfieldStruct<u8, Tedgpl_SPEC>;
    impl Tedgpl {
        #[doc = "Either edge (rising)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Both edges"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tck1_SPEC;
    pub type Tck1 = crate::EnumBitfieldStruct<u8, Tck1_SPEC>;
    impl Tck1 {
        #[doc = "Divided clock specified by the ULPTMR2.CKS\\[2:0\\] bits (ULPTLCLK)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Divided clock specified by the ULPTMR2.CKS\\[2:0\\] bits (ULPTSCLK)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulptmr2_SPEC;
impl crate::sealed::RegSpec for Ulptmr2_SPEC {
    type DataType = u8;
}

#[doc = "ULPT Mode Register 2"]
pub type Ulptmr2 = crate::RegValueT<Ulptmr2_SPEC>;

impl Ulptmr2 {
    #[doc = "ULPTLCLK/ULPTSCLK Count Source Clock Division Ratio"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        ulptmr2::Cks,
        ulptmr2::Cks,
        Ulptmr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            ulptmr2::Cks,
            ulptmr2::Cks,
            Ulptmr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub fn lpm(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ulptmr2::Lpm,
        ulptmr2::Lpm,
        Ulptmr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ulptmr2::Lpm,
            ulptmr2::Lpm,
            Ulptmr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ulptmr2 {
    #[inline(always)]
    fn default() -> Ulptmr2 {
        <crate::RegValueT<Ulptmr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ulptmr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "1/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "1/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "1/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "1/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "1/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "1/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "1/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "1/128"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lpm_SPEC;
    pub type Lpm = crate::EnumBitfieldStruct<u8, Lpm_SPEC>;
    impl Lpm {
        #[doc = "Normal mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low power mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulptmr3_SPEC;
impl crate::sealed::RegSpec for Ulptmr3_SPEC {
    type DataType = u8;
}

#[doc = "ULPT Mode Register 3"]
pub type Ulptmr3 = crate::RegValueT<Ulptmr3_SPEC>;

impl Ulptmr3 {
    #[doc = "Count Function Select"]
    #[inline(always)]
    pub fn tcntctl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ulptmr3::Tcntctl,
        ulptmr3::Tcntctl,
        Ulptmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ulptmr3::Tcntctl,
            ulptmr3::Tcntctl,
            Ulptmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPTEVIn Polarity Switch"]
    #[inline(always)]
    pub fn tevpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ulptmr3::Tevpol,
        ulptmr3::Tevpol,
        Ulptmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ulptmr3::Tevpol,
            ulptmr3::Tevpol,
            Ulptmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPTOn Polarity Select"]
    #[inline(always)]
    pub fn topol(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ulptmr3::Topol,
        ulptmr3::Topol,
        Ulptmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ulptmr3::Topol,
            ulptmr3::Topol,
            Ulptmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPTEEn Function Select"]
    #[inline(always)]
    pub fn teectl(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ulptmr3::Teectl,
        ulptmr3::Teectl,
        Ulptmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ulptmr3::Teectl,
            ulptmr3::Teectl,
            Ulptmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPTEEn Edge Polarity Select"]
    #[inline(always)]
    pub fn teepol(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        ulptmr3::Teepol,
        ulptmr3::Teepol,
        Ulptmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            ulptmr3::Teepol,
            ulptmr3::Teepol,
            Ulptmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ulptmr3 {
    #[inline(always)]
    fn default() -> Ulptmr3 {
        <crate::RegValueT<Ulptmr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ulptmr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcntctl_SPEC;
    pub type Tcntctl = crate::EnumBitfieldStruct<u8, Tcntctl_SPEC>;
    impl Tcntctl {
        #[doc = "Continuous mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "One-shot mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tevpol_SPEC;
    pub type Tevpol = crate::EnumBitfieldStruct<u8, Tevpol_SPEC>;
    impl Tevpol {
        #[doc = "External event input (ULPTEVIn pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "External event input (ULPTEVIn pin) in reverse"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Topol_SPEC;
    pub type Topol = crate::EnumBitfieldStruct<u8, Topol_SPEC>;
    impl Topol {
        #[doc = "Start the ULPTOn output with low level."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start the ULPTOn output with high level."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teectl_SPEC;
    pub type Teectl = crate::EnumBitfieldStruct<u8, Teectl_SPEC>;
    impl Teectl {
        #[doc = "Count enable mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "Count start mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Count restart mode"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teepol_SPEC;
    pub type Teepol = crate::EnumBitfieldStruct<u8, Teepol_SPEC>;
    impl Teepol {
        #[doc = "Rising edge"]
        pub const _00: Self = Self::new(0);

        #[doc = "Falling edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Both edges"]
        pub const _10: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulptioc_SPEC;
impl crate::sealed::RegSpec for Ulptioc_SPEC {
    type DataType = u8;
}

#[doc = "ULPT I/O Control Register"]
pub type Ulptioc = crate::RegValueT<Ulptioc_SPEC>;

impl Ulptioc {
    #[doc = "ULPTOn Output Enable"]
    #[inline(always)]
    pub fn toe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ulptioc::Toe,
        ulptioc::Toe,
        Ulptioc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ulptioc::Toe,
            ulptioc::Toe,
            Ulptioc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPTEVIn Input Filter"]
    #[inline(always)]
    pub fn tipf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ulptioc::Tipf,
        ulptioc::Tipf,
        Ulptioc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ulptioc::Tipf,
            ulptioc::Tipf,
            Ulptioc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPTEVIn Count Control"]
    #[inline(always)]
    pub fn tiogt0(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ulptioc::Tiogt0,
        ulptioc::Tiogt0,
        Ulptioc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ulptioc::Tiogt0,
            ulptioc::Tiogt0,
            Ulptioc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ulptioc {
    #[inline(always)]
    fn default() -> Ulptioc {
        <crate::RegValueT<Ulptioc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ulptioc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toe_SPEC;
    pub type Toe = crate::EnumBitfieldStruct<u8, Toe_SPEC>;
    impl Toe {
        #[doc = "Disable the ULPTOn output."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the ULPTOn output."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tipf_SPEC;
    pub type Tipf = crate::EnumBitfieldStruct<u8, Tipf_SPEC>;
    impl Tipf {
        #[doc = "No filter"]
        pub const _00: Self = Self::new(0);

        #[doc = "Filter sampling at PCLKB"]
        pub const _01: Self = Self::new(1);

        #[doc = "Filter sampling at PCLKB/8"]
        pub const _10: Self = Self::new(2);

        #[doc = "Filter sampling at PCLKB/32"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tiogt0_SPEC;
    pub type Tiogt0 = crate::EnumBitfieldStruct<u8, Tiogt0_SPEC>;
    impl Tiogt0 {
        #[doc = "Always count external events."]
        pub const _0: Self = Self::new(0);

        #[doc = "Count external events while the ULPTEVIn pin is valid."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulptisr_SPEC;
impl crate::sealed::RegSpec for Ulptisr_SPEC {
    type DataType = u8;
}

#[doc = "ULPT Event Pin Select Register"]
pub type Ulptisr = crate::RegValueT<Ulptisr_SPEC>;

impl Ulptisr {
    #[doc = "ULPTEEn Polarity Select"]
    #[inline(always)]
    pub fn rccpsel2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ulptisr::Rccpsel2,
        ulptisr::Rccpsel2,
        Ulptisr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ulptisr::Rccpsel2,
            ulptisr::Rccpsel2,
            Ulptisr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ulptisr {
    #[inline(always)]
    fn default() -> Ulptisr {
        <crate::RegValueT<Ulptisr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ulptisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rccpsel2_SPEC;
    pub type Rccpsel2 = crate::EnumBitfieldStruct<u8, Rccpsel2_SPEC>;
    impl Rccpsel2 {
        #[doc = "Count external events when low level."]
        pub const _0: Self = Self::new(0);

        #[doc = "Count external events when high level."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulptcmsr_SPEC;
impl crate::sealed::RegSpec for Ulptcmsr_SPEC {
    type DataType = u8;
}

#[doc = "ULPT Compare Match Function Select Register"]
pub type Ulptcmsr = crate::RegValueT<Ulptcmsr_SPEC>;

impl Ulptcmsr {
    #[doc = "Compare Match A Register Enable"]
    #[inline(always)]
    pub fn tcmea(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ulptcmsr::Tcmea,
        ulptcmsr::Tcmea,
        Ulptcmsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ulptcmsr::Tcmea,
            ulptcmsr::Tcmea,
            Ulptcmsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPTOAn Output Enable"]
    #[inline(always)]
    pub fn toea(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ulptcmsr::Toea,
        ulptcmsr::Toea,
        Ulptcmsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ulptcmsr::Toea,
            ulptcmsr::Toea,
            Ulptcmsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPTOAn Polarity Select"]
    #[inline(always)]
    pub fn topola(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ulptcmsr::Topola,
        ulptcmsr::Topola,
        Ulptcmsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ulptcmsr::Topola,
            ulptcmsr::Topola,
            Ulptcmsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Match B Register Enable"]
    #[inline(always)]
    pub fn tcmeb(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ulptcmsr::Tcmeb,
        ulptcmsr::Tcmeb,
        Ulptcmsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ulptcmsr::Tcmeb,
            ulptcmsr::Tcmeb,
            Ulptcmsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPTOBn Output Enable"]
    #[inline(always)]
    pub fn toeb(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ulptcmsr::Toeb,
        ulptcmsr::Toeb,
        Ulptcmsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ulptcmsr::Toeb,
            ulptcmsr::Toeb,
            Ulptcmsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPTOBn Polarity Select"]
    #[inline(always)]
    pub fn topolb(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ulptcmsr::Topolb,
        ulptcmsr::Topolb,
        Ulptcmsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ulptcmsr::Topolb,
            ulptcmsr::Topolb,
            Ulptcmsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ulptcmsr {
    #[inline(always)]
    fn default() -> Ulptcmsr {
        <crate::RegValueT<Ulptcmsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ulptcmsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcmea_SPEC;
    pub type Tcmea = crate::EnumBitfieldStruct<u8, Tcmea_SPEC>;
    impl Tcmea {
        #[doc = "Disable compare match A register."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare match A register."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toea_SPEC;
    pub type Toea = crate::EnumBitfieldStruct<u8, Toea_SPEC>;
    impl Toea {
        #[doc = "Disable the ULPTOAn output."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the ULPTOAn output."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Topola_SPEC;
    pub type Topola = crate::EnumBitfieldStruct<u8, Topola_SPEC>;
    impl Topola {
        #[doc = "Start the ULPTOAn output with low."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start the ULPTOAn output with high."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcmeb_SPEC;
    pub type Tcmeb = crate::EnumBitfieldStruct<u8, Tcmeb_SPEC>;
    impl Tcmeb {
        #[doc = "Disable compare match B register."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare match B register."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toeb_SPEC;
    pub type Toeb = crate::EnumBitfieldStruct<u8, Toeb_SPEC>;
    impl Toeb {
        #[doc = "Disable the ULPTOBn output."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the ULPTOBn output."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Topolb_SPEC;
    pub type Topolb = crate::EnumBitfieldStruct<u8, Topolb_SPEC>;
    impl Topolb {
        #[doc = "Start the ULPTOBn output with low."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start the ULPTOBn output with high."]
        pub const _1: Self = Self::new(1);
    }
}
