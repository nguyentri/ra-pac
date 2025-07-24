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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:55:06 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Ultra-Low Power Timer 0"]
unsafe impl ::core::marker::Send for super::Ulpt0 {}
unsafe impl ::core::marker::Sync for super::Ulpt0 {}
impl super::Ulpt0 {
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
    #[doc = "32bit counter and reload registerNOTE : When 1 is written to the TSTOP bit in the ULPTCR register, the 32-bit counter is forcibly stopped and set to FFFFFFFFH."]
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
    #[doc = "ULPT Compare Match A RegisterNOTE : When 1 is written to the TSTOP bit in the ULPTCR register, set to FFFFFFFFH"]
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
    #[doc = "AGT Compare Match B RegisterNOTE : When 1 is written to the TSTOP bit in the ULPTCR register, set to FFFFFFFFH"]
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
    #[doc = "ULPT count start"]
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

    #[doc = "ULPT count status flag"]
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

    #[doc = "ULPT count forced stop"]
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

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, Ulptcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,Ulptcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ULPT underflow flag"]
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

    #[doc = "ULPT compare match A flag"]
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

    #[doc = "ULPT compare match B flag"]
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
        #[doc = "Count stops"]
        pub const _0: Self = Self::new(0);

        #[doc = "Count starts"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcstf_SPEC;
    pub type Tcstf = crate::EnumBitfieldStruct<u8, Tcstf_SPEC>;
    impl Tcstf {
        #[doc = "Count stops"]
        pub const _0: Self = Self::new(0);

        #[doc = "Count running"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tstop_SPEC;
    pub type Tstop = crate::EnumBitfieldStruct<u8, Tstop_SPEC>;
    impl Tstop {
        #[doc = "no effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "The count is forcibly stopped."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tundf_SPEC;
    pub type Tundf = crate::EnumBitfieldStruct<u8, Tundf_SPEC>;
    impl Tundf {
        #[doc = "No underflow"]
        pub const _0: Self = Self::new(0);

        #[doc = "Underflow"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcmaf_SPEC;
    pub type Tcmaf = crate::EnumBitfieldStruct<u8, Tcmaf_SPEC>;
    impl Tcmaf {
        #[doc = "No Match"]
        pub const _0: Self = Self::new(0);

        #[doc = "Match"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcmbf_SPEC;
    pub type Tcmbf = crate::EnumBitfieldStruct<u8, Tcmbf_SPEC>;
    impl Tcmbf {
        #[doc = "No Match"]
        pub const _0: Self = Self::new(0);

        #[doc = "Match"]
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
    #[doc = "ULPT operating mode select"]
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

    #[doc = "ULPTEVI edge polarity select"]
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

    #[doc = "ULPT count source select"]
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

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Ulptmr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Ulptmr1_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "One edge(rise)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Both edges"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tck1_SPEC;
    pub type Tck1 = crate::EnumBitfieldStruct<u8, Tck1_SPEC>;
    impl Tck1 {
        #[doc = "Divided clock LOCO specified by ULPTMR2.CKS bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "Divided clock fSUB specified by ULPTMR2.CKS bit."]
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
    #[doc = "fsub/LOCO count source clock frequency division ratio select"]
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

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, Ulptmr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,Ulptmr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ULPT Low Power Mode"]
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

        #[doc = "Low Power mode"]
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
    #[doc = "ULPT count function select"]
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

    #[doc = "ULPTEVI polarity switch"]
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

    #[doc = "ULPTO polarity select"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ulptmr3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ulptmr3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ULPTEE function select"]
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

    #[doc = "ULPTEE edge polarity select"]
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

        #[doc = "One shot mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tevpol_SPEC;
    pub type Tevpol = crate::EnumBitfieldStruct<u8, Tevpol_SPEC>;
    impl Tevpol {
        #[doc = "Extarnal event input(ULPTEVI port) forward rotation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extarnal event input(ULPTEVI port) inversion"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Topol_SPEC;
    pub type Topol = crate::EnumBitfieldStruct<u8, Topol_SPEC>;
    impl Topol {
        #[doc = "ULPTO Output is started at low"]
        pub const _0: Self = Self::new(0);

        #[doc = "ULPTO Output is started at high"]
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

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teepol_SPEC;
    pub type Teepol = crate::EnumBitfieldStruct<u8, Teepol_SPEC>;
    impl Teepol {
        #[doc = "Rise edge"]
        pub const _00: Self = Self::new(0);

        #[doc = "Fall edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Both edges"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
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
    #[doc = "ULPTO output enable"]
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

    #[doc = "ULPTEVI input filter select"]
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

    #[doc = "ULPTEVI count control"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ulptioc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ulptioc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
        #[doc = "ULPTO output disabled (port)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ULPTO output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tipf_SPEC;
    pub type Tipf = crate::EnumBitfieldStruct<u8, Tipf_SPEC>;
    impl Tipf {
        #[doc = "No filter"]
        pub const _00: Self = Self::new(0);

        #[doc = "Filter sampled at PCLKB"]
        pub const _01: Self = Self::new(1);

        #[doc = "Filter sampled at PCLKB/8"]
        pub const _10: Self = Self::new(2);

        #[doc = "Filter sampled at PCLKB/32"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tiogt0_SPEC;
    pub type Tiogt0 = crate::EnumBitfieldStruct<u8, Tiogt0_SPEC>;
    impl Tiogt0 {
        #[doc = "Extarnal event is always counted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extarnal event is counted while the ULPTEE pin is active"]
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
    #[doc = "ULPTEE polarty selection"]
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

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Ulptisr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Ulptisr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "An external event is counted during the low-level period"]
        pub const _0: Self = Self::new(0);

        #[doc = "An external event is counted during the high-level period"]
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
    #[doc = "Compare match A register enable"]
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

    #[doc = "ULPTOA output enable"]
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

    #[doc = "ULPTOA polarity select"]
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

    #[doc = "Compare match B register enable"]
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

    #[doc = "ULPTOB output enable"]
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

    #[doc = "ULPTOB polarity select"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ulptcmsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ulptcmsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
        #[doc = "Disable compare match A register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare match A register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toea_SPEC;
    pub type Toea = crate::EnumBitfieldStruct<u8, Toea_SPEC>;
    impl Toea {
        #[doc = "ULPTOA output disabled (port)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ULPTOA output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Topola_SPEC;
    pub type Topola = crate::EnumBitfieldStruct<u8, Topola_SPEC>;
    impl Topola {
        #[doc = "ULPTOA Output is started at low"]
        pub const _0: Self = Self::new(0);

        #[doc = "ULPTOA Output is started at high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcmeb_SPEC;
    pub type Tcmeb = crate::EnumBitfieldStruct<u8, Tcmeb_SPEC>;
    impl Tcmeb {
        #[doc = "Disable compare match B register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare match B register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toeb_SPEC;
    pub type Toeb = crate::EnumBitfieldStruct<u8, Toeb_SPEC>;
    impl Toeb {
        #[doc = "ULPTOB output disabled (port)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ULPTOB output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Topolb_SPEC;
    pub type Topolb = crate::EnumBitfieldStruct<u8, Topolb_SPEC>;
    impl Topolb {
        #[doc = "ULPTOB Output is started at low"]
        pub const _0: Self = Self::new(0);

        #[doc = "ULPTOB Output is started at high"]
        pub const _1: Self = Self::new(1);
    }
}
