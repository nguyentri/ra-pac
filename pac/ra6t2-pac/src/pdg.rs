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
// Generated from SVD 1.40.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:37:20 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"PWM Delay Generation Circuit"]
unsafe impl ::core::marker::Send for super::Pdg {}
unsafe impl ::core::marker::Sync for super::Pdg {}
impl super::Pdg {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "PWM Output Delay Control Register"]
    #[inline(always)]
    pub const fn gtdlycr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "PWM Output Delay Control Register 2"]
    #[inline(always)]
    pub const fn gtdlycr2(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlycr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlycr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "GTIOCnA Rising Output Delay Register"]
    #[inline(always)]
    pub const fn gtdlyra(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gtdlyra_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x18usize))
        }
    }
    #[inline(always)]
    pub const fn gtdlyr0a(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gtdlyr1a(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gtdlyr2a(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gtdlyr3a(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }

    #[doc = "GTIOCnB Rising Output Delay Register"]
    #[inline(always)]
    pub const fn gtdlyrb(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gtdlyrb_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1ausize))
        }
    }
    #[inline(always)]
    pub const fn gtdlyr0b(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn gtdlyr1b(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gtdlyr2b(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gtdlyr3b(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x26usize),
            )
        }
    }

    #[doc = "GTIOCnA Falling Output Delay Register"]
    #[inline(always)]
    pub const fn gtdlyfa(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gtdlyfa_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x28usize))
        }
    }
    #[inline(always)]
    pub const fn gtdlyf0a(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyfa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyfa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gtdlyf1a(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyfa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyfa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gtdlyf2a(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyfa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyfa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gtdlyf3a(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyfa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyfa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }

    #[doc = "GTIOCnB Falling Output Delay Register"]
    #[inline(always)]
    pub const fn gtdlyfb(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gtdlyfb_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2ausize))
        }
    }
    #[inline(always)]
    pub const fn gtdlyf0b(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyfb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyfb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn gtdlyf1b(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyfb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyfb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gtdlyf2b(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyfb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyfb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x32usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gtdlyf3b(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlyfb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlyfb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x36usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdlycr_SPEC;
impl crate::sealed::RegSpec for Gtdlycr_SPEC {
    type DataType = u16;
}

#[doc = "PWM Output Delay Control Register"]
pub type Gtdlycr = crate::RegValueT<Gtdlycr_SPEC>;

impl Gtdlycr {
    #[doc = "DLL Operation Enable"]
    #[inline(always)]
    pub fn dllen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtdlycr::Dllen,
        gtdlycr::Dllen,
        Gtdlycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtdlycr::Dllen,
            gtdlycr::Dllen,
            Gtdlycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PWM Delay Generation Circuit Reset"]
    #[inline(always)]
    pub fn dlyrst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtdlycr::Dlyrst,
        gtdlycr::Dlyrst,
        Gtdlycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtdlycr::Dlyrst,
            gtdlycr::Dlyrst,
            Gtdlycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GPT core clock Frequency Range"]
    #[inline(always)]
    pub fn frange(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtdlycr::Frange,
        gtdlycr::Frange,
        Gtdlycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtdlycr::Frange,
            gtdlycr::Frange,
            Gtdlycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdlycr {
    #[inline(always)]
    fn default() -> Gtdlycr {
        <crate::RegValueT<Gtdlycr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdlycr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dllen_SPEC;
    pub type Dllen = crate::EnumBitfieldStruct<u8, Dllen_SPEC>;
    impl Dllen {
        #[doc = "DLL operation disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "DLL operation enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlyrst_SPEC;
    pub type Dlyrst = crate::EnumBitfieldStruct<u8, Dlyrst_SPEC>;
    impl Dlyrst {
        #[doc = "Normal operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frange_SPEC;
    pub type Frange = crate::EnumBitfieldStruct<u8, Frange_SPEC>;
    impl Frange {
        #[doc = "GPT core clock frequency is 115 MHz to 200 MHz"]
        pub const _0: Self = Self::new(0);

        #[doc = "GPT core clock frequency is 80 MHz to 120 MHz"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdlycr2_SPEC;
impl crate::sealed::RegSpec for Gtdlycr2_SPEC {
    type DataType = u16;
}

#[doc = "PWM Output Delay Control Register 2"]
pub type Gtdlycr2 = crate::RegValueT<Gtdlycr2_SPEC>;

impl Gtdlycr2 {
    #[doc = "PWM Delay Generation Circuit bypass for channel 0"]
    #[inline(always)]
    pub fn dlybs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtdlycr2::Dlybs0,
        gtdlycr2::Dlybs0,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtdlycr2::Dlybs0,
            gtdlycr2::Dlybs0,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PWM Delay Generation Circuit bypass for channel 1"]
    #[inline(always)]
    pub fn dlybs1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtdlycr2::Dlybs1,
        gtdlycr2::Dlybs1,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtdlycr2::Dlybs1,
            gtdlycr2::Dlybs1,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PWM Delay Generation Circuit bypass for channel 2"]
    #[inline(always)]
    pub fn dlybs2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtdlycr2::Dlybs2,
        gtdlycr2::Dlybs2,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtdlycr2::Dlybs2,
            gtdlycr2::Dlybs2,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PWM Delay Generation Circuit bypass for channel 3"]
    #[inline(always)]
    pub fn dlybs3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtdlycr2::Dlybs3,
        gtdlycr2::Dlybs3,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtdlycr2::Dlybs3,
            gtdlycr2::Dlybs3,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PWM Delay Generation Circuit enable for channel 0"]
    #[inline(always)]
    pub fn dlyen0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtdlycr2::Dlyen0,
        gtdlycr2::Dlyen0,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtdlycr2::Dlyen0,
            gtdlycr2::Dlyen0,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PWM Delay Generation Circuit enable for channel 1"]
    #[inline(always)]
    pub fn dlyen1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtdlycr2::Dlyen1,
        gtdlycr2::Dlyen1,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtdlycr2::Dlyen1,
            gtdlycr2::Dlyen1,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PWM Delay Generation Circuit enable for channel 2"]
    #[inline(always)]
    pub fn dlyen2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtdlycr2::Dlyen2,
        gtdlycr2::Dlyen2,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtdlycr2::Dlyen2,
            gtdlycr2::Dlyen2,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PWM Delay Generation Circuit enable for channel 3"]
    #[inline(always)]
    pub fn dlyen3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtdlycr2::Dlyen3,
        gtdlycr2::Dlyen3,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtdlycr2::Dlyen3,
            gtdlycr2::Dlyen3,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdlycr2 {
    #[inline(always)]
    fn default() -> Gtdlycr2 {
        <crate::RegValueT<Gtdlycr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdlycr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlybs0_SPEC;
    pub type Dlybs0 = crate::EnumBitfieldStruct<u8, Dlybs0_SPEC>;
    impl Dlybs0 {
        #[doc = "Delay generation circuit of channel 0 bypassed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Delay generation circuit of channel 0 not bypassed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlybs1_SPEC;
    pub type Dlybs1 = crate::EnumBitfieldStruct<u8, Dlybs1_SPEC>;
    impl Dlybs1 {
        #[doc = "Delay generation circuit of channel 1 bypassed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Delay generation circuit of channel 1 not bypassed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlybs2_SPEC;
    pub type Dlybs2 = crate::EnumBitfieldStruct<u8, Dlybs2_SPEC>;
    impl Dlybs2 {
        #[doc = "Delay generation circuit of channel 2 bypassed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Delay generation circuit of channel 2 not bypassed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlybs3_SPEC;
    pub type Dlybs3 = crate::EnumBitfieldStruct<u8, Dlybs3_SPEC>;
    impl Dlybs3 {
        #[doc = "Delay generation circuit of channel 3 bypassed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Delay generation circuit of channel 3 not bypassed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlyen0_SPEC;
    pub type Dlyen0 = crate::EnumBitfieldStruct<u8, Dlyen0_SPEC>;
    impl Dlyen0 {
        #[doc = "Delay generation circuit of channel 0 enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Delay generation circuit of channel 0 disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlyen1_SPEC;
    pub type Dlyen1 = crate::EnumBitfieldStruct<u8, Dlyen1_SPEC>;
    impl Dlyen1 {
        #[doc = "Delay generation circuit of channel 1 enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Delay generation circuit of channel 1 disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlyen2_SPEC;
    pub type Dlyen2 = crate::EnumBitfieldStruct<u8, Dlyen2_SPEC>;
    impl Dlyen2 {
        #[doc = "Delay generation circuit of channel 2 enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Delay generation circuit of channel 2 disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlyen3_SPEC;
    pub type Dlyen3 = crate::EnumBitfieldStruct<u8, Dlyen3_SPEC>;
    impl Dlyen3 {
        #[doc = "Delay generation circuit of channel 3 enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Delay generation circuit of channel 3 disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdlyra_SPEC;
impl crate::sealed::RegSpec for Gtdlyra_SPEC {
    type DataType = u16;
}

#[doc = "GTIOCnA Rising Output Delay Register"]
pub type Gtdlyra = crate::RegValueT<Gtdlyra_SPEC>;

impl Gtdlyra {
    #[doc = "GTIOCnA Output Rising Edge Delay Setting"]
    #[inline(always)]
    pub fn dly(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        gtdlyra::Dly,
        gtdlyra::Dly,
        Gtdlyra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            gtdlyra::Dly,
            gtdlyra::Dly,
            Gtdlyra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdlyra {
    #[inline(always)]
    fn default() -> Gtdlyra {
        <crate::RegValueT<Gtdlyra_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdlyra {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dly_SPEC;
    pub type Dly = crate::EnumBitfieldStruct<u8, Dly_SPEC>;
    impl Dly {
        #[doc = "Delay on rising edges is not applied"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "Delay of 1/32 times GTCLK period applied"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "Delay of 2/32 times GTCLK period applied"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "Delay of 3/32 times GTCLK period applied"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "Delay of 4/32 times GTCLK period applied"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "Delay of 5/32 times GTCLK period applied"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "Delay of 6/32 times GTCLK period applied"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "Delay of 7/32 times GTCLK period applied"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "Delay of 8/32 times GTCLK period applied"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "Delay of 9/32 times GTCLK period applied"]
        pub const _0_X_09: Self = Self::new(9);

        #[doc = "Delay of 10/32 times GTCLK period applied"]
        pub const _0_X_0_A: Self = Self::new(10);

        #[doc = "Delay of 11/32 times GTCLK period applied"]
        pub const _0_X_0_B: Self = Self::new(11);

        #[doc = "Delay of 12/32 times GTCLK period applied"]
        pub const _0_X_0_C: Self = Self::new(12);

        #[doc = "Delay of 13/32 times GTCLK period applied"]
        pub const _0_X_0_D: Self = Self::new(13);

        #[doc = "Delay of 14/32 times GTCLK period applied"]
        pub const _0_X_0_E: Self = Self::new(14);

        #[doc = "Delay of 15/32 times GTCLK period applied"]
        pub const _0_X_0_F: Self = Self::new(15);

        #[doc = "Delay of 16/32 times GTCLK period applied"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "Delay of 17/32 times GTCLK period applied"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "Delay of 18/32 times GTCLK period applied"]
        pub const _0_X_12: Self = Self::new(18);

        #[doc = "Delay of 19/32 times GTCLK period applied"]
        pub const _0_X_13: Self = Self::new(19);

        #[doc = "Delay of 20/32 times GTCLK period applied"]
        pub const _0_X_14: Self = Self::new(20);

        #[doc = "Delay of 21/32 times GTCLK period applied"]
        pub const _0_X_15: Self = Self::new(21);

        #[doc = "Delay of 22/32 times GTCLK period applied"]
        pub const _0_X_16: Self = Self::new(22);

        #[doc = "Delay of 23/32 times GTCLK period applied"]
        pub const _0_X_17: Self = Self::new(23);

        #[doc = "Delay of 24/32 times GTCLK period applied"]
        pub const _0_X_18: Self = Self::new(24);

        #[doc = "Delay of 25/32 times GTCLK period applied"]
        pub const _0_X_19: Self = Self::new(25);

        #[doc = "Delay of 26/32 times GTCLK period applied"]
        pub const _0_X_1_A: Self = Self::new(26);

        #[doc = "Delay of 27/32 times GTCLK period applied"]
        pub const _0_X_1_B: Self = Self::new(27);

        #[doc = "Delay of 28/32 times GTCLK period applied"]
        pub const _0_X_1_C: Self = Self::new(28);

        #[doc = "Delay of 29/32 times GTCLK period applied"]
        pub const _0_X_1_D: Self = Self::new(29);

        #[doc = "Delay of 30/32 times GTCLK period applied"]
        pub const _0_X_1_E: Self = Self::new(30);

        #[doc = "Delay of 31/32 times GTCLK period applied"]
        pub const _0_X_1_F: Self = Self::new(31);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdlyrb_SPEC;
impl crate::sealed::RegSpec for Gtdlyrb_SPEC {
    type DataType = u16;
}

#[doc = "GTIOCnB Rising Output Delay Register"]
pub type Gtdlyrb = crate::RegValueT<Gtdlyrb_SPEC>;

impl Gtdlyrb {
    #[doc = "GTIOCnB Output Rising Edge Delay Setting"]
    #[inline(always)]
    pub fn dly(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        gtdlyrb::Dly,
        gtdlyrb::Dly,
        Gtdlyrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            gtdlyrb::Dly,
            gtdlyrb::Dly,
            Gtdlyrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdlyrb {
    #[inline(always)]
    fn default() -> Gtdlyrb {
        <crate::RegValueT<Gtdlyrb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdlyrb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dly_SPEC;
    pub type Dly = crate::EnumBitfieldStruct<u8, Dly_SPEC>;
    impl Dly {
        #[doc = "Delay on rising edges is not applied"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "Delay of 1/32 times GTCLK period applied"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "Delay of 2/32 times GTCLK period applied"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "Delay of 3/32 times GTCLK period applied"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "Delay of 4/32 times GTCLK period applied"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "Delay of 5/32 times GTCLK period applied"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "Delay of 6/32 times GTCLK period applied"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "Delay of 7/32 times GTCLK period applied"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "Delay of 8/32 times GTCLK period applied"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "Delay of 9/32 times GTCLK period applied"]
        pub const _0_X_09: Self = Self::new(9);

        #[doc = "Delay of 10/32 times GTCLK period applied"]
        pub const _0_X_0_A: Self = Self::new(10);

        #[doc = "Delay of 11/32 times GTCLK period applied"]
        pub const _0_X_0_B: Self = Self::new(11);

        #[doc = "Delay of 12/32 times GTCLK period applied"]
        pub const _0_X_0_C: Self = Self::new(12);

        #[doc = "Delay of 13/32 times GTCLK period applied"]
        pub const _0_X_0_D: Self = Self::new(13);

        #[doc = "Delay of 14/32 times GTCLK period applied"]
        pub const _0_X_0_E: Self = Self::new(14);

        #[doc = "Delay of 15/32 times GTCLK period applied"]
        pub const _0_X_0_F: Self = Self::new(15);

        #[doc = "Delay of 16/32 times GTCLK period applied"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "Delay of 17/32 times GTCLK period applied"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "Delay of 18/32 times GTCLK period applied"]
        pub const _0_X_12: Self = Self::new(18);

        #[doc = "Delay of 19/32 times GTCLK period applied"]
        pub const _0_X_13: Self = Self::new(19);

        #[doc = "Delay of 20/32 times GTCLK period applied"]
        pub const _0_X_14: Self = Self::new(20);

        #[doc = "Delay of 21/32 times GTCLK period applied"]
        pub const _0_X_15: Self = Self::new(21);

        #[doc = "Delay of 22/32 times GTCLK period applied"]
        pub const _0_X_16: Self = Self::new(22);

        #[doc = "Delay of 23/32 times GTCLK period applied"]
        pub const _0_X_17: Self = Self::new(23);

        #[doc = "Delay of 24/32 times GTCLK period applied"]
        pub const _0_X_18: Self = Self::new(24);

        #[doc = "Delay of 25/32 times GTCLK period applied"]
        pub const _0_X_19: Self = Self::new(25);

        #[doc = "Delay of 26/32 times GTCLK period applied"]
        pub const _0_X_1_A: Self = Self::new(26);

        #[doc = "Delay of 27/32 times GTCLK period applied"]
        pub const _0_X_1_B: Self = Self::new(27);

        #[doc = "Delay of 28/32 times GTCLK period applied"]
        pub const _0_X_1_C: Self = Self::new(28);

        #[doc = "Delay of 29/32 times GTCLK period applied"]
        pub const _0_X_1_D: Self = Self::new(29);

        #[doc = "Delay of 30/32 times GTCLK period applied"]
        pub const _0_X_1_E: Self = Self::new(30);

        #[doc = "Delay of 31/32 times GTCLK period applied"]
        pub const _0_X_1_F: Self = Self::new(31);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdlyfa_SPEC;
impl crate::sealed::RegSpec for Gtdlyfa_SPEC {
    type DataType = u16;
}

#[doc = "GTIOCnA Falling Output Delay Register"]
pub type Gtdlyfa = crate::RegValueT<Gtdlyfa_SPEC>;

impl Gtdlyfa {
    #[doc = "GTIOCnA Output Falling Edge Delay Setting"]
    #[inline(always)]
    pub fn dly(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        gtdlyfa::Dly,
        gtdlyfa::Dly,
        Gtdlyfa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            gtdlyfa::Dly,
            gtdlyfa::Dly,
            Gtdlyfa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdlyfa {
    #[inline(always)]
    fn default() -> Gtdlyfa {
        <crate::RegValueT<Gtdlyfa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdlyfa {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dly_SPEC;
    pub type Dly = crate::EnumBitfieldStruct<u8, Dly_SPEC>;
    impl Dly {
        #[doc = "Delay on falling edges is not applied"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "Delay of 1/32 times GTCLK period applied"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "Delay of 2/32 times GTCLK period applied"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "Delay of 3/32 times GTCLK period applied"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "Delay of 4/32 times GTCLK period applied"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "Delay of 5/32 times GTCLK period applied"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "Delay of 6/32 times GTCLK period applied"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "Delay of 7/32 times GTCLK period applied"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "Delay of 8/32 times GTCLK period applied"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "Delay of 9/32 times GTCLK period applied"]
        pub const _0_X_09: Self = Self::new(9);

        #[doc = "Delay of 10/32 times GTCLK period applied"]
        pub const _0_X_0_A: Self = Self::new(10);

        #[doc = "Delay of 11/32 times GTCLK period applied"]
        pub const _0_X_0_B: Self = Self::new(11);

        #[doc = "Delay of 12/32 times GTCLK period applied"]
        pub const _0_X_0_C: Self = Self::new(12);

        #[doc = "Delay of 13/32 times GTCLK period applied"]
        pub const _0_X_0_D: Self = Self::new(13);

        #[doc = "Delay of 14/32 times GTCLK period applied"]
        pub const _0_X_0_E: Self = Self::new(14);

        #[doc = "Delay of 15/32 times GTCLK period applied"]
        pub const _0_X_0_F: Self = Self::new(15);

        #[doc = "Delay of 16/32 times GTCLK period applied"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "Delay of 17/32 times GTCLK period applied"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "Delay of 18/32 times GTCLK period applied"]
        pub const _0_X_12: Self = Self::new(18);

        #[doc = "Delay of 19/32 times GTCLK period applied"]
        pub const _0_X_13: Self = Self::new(19);

        #[doc = "Delay of 20/32 times GTCLK period applied"]
        pub const _0_X_14: Self = Self::new(20);

        #[doc = "Delay of 21/32 times GTCLK period applied"]
        pub const _0_X_15: Self = Self::new(21);

        #[doc = "Delay of 22/32 times GTCLK period applied"]
        pub const _0_X_16: Self = Self::new(22);

        #[doc = "Delay of 23/32 times GTCLK period applied"]
        pub const _0_X_17: Self = Self::new(23);

        #[doc = "Delay of 24/32 times GTCLK period applied"]
        pub const _0_X_18: Self = Self::new(24);

        #[doc = "Delay of 25/32 times GTCLK period applied"]
        pub const _0_X_19: Self = Self::new(25);

        #[doc = "Delay of 26/32 times GTCLK period applied"]
        pub const _0_X_1_A: Self = Self::new(26);

        #[doc = "Delay of 27/32 times GTCLK period applied"]
        pub const _0_X_1_B: Self = Self::new(27);

        #[doc = "Delay of 28/32 times GTCLK period applied"]
        pub const _0_X_1_C: Self = Self::new(28);

        #[doc = "Delay of 29/32 times GTCLK period applied"]
        pub const _0_X_1_D: Self = Self::new(29);

        #[doc = "Delay of 30/32 times GTCLK period applied"]
        pub const _0_X_1_E: Self = Self::new(30);

        #[doc = "Delay of 31/32 times GTCLK period applied"]
        pub const _0_X_1_F: Self = Self::new(31);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdlyfb_SPEC;
impl crate::sealed::RegSpec for Gtdlyfb_SPEC {
    type DataType = u16;
}

#[doc = "GTIOCnB Falling Output Delay Register"]
pub type Gtdlyfb = crate::RegValueT<Gtdlyfb_SPEC>;

impl Gtdlyfb {
    #[doc = "GTIOCnB Output Falling Edge Delay Setting"]
    #[inline(always)]
    pub fn dly(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        gtdlyfb::Dly,
        gtdlyfb::Dly,
        Gtdlyfb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            gtdlyfb::Dly,
            gtdlyfb::Dly,
            Gtdlyfb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdlyfb {
    #[inline(always)]
    fn default() -> Gtdlyfb {
        <crate::RegValueT<Gtdlyfb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdlyfb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dly_SPEC;
    pub type Dly = crate::EnumBitfieldStruct<u8, Dly_SPEC>;
    impl Dly {
        #[doc = "Delay on falling edges is not applied"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "Delay of 1/32 times GTCLK period applied"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "Delay of 2/32 times GTCLK period applied"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "Delay of 3/32 times GTCLK period applied"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "Delay of 4/32 times GTCLK period applied"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "Delay of 5/32 times GTCLK period applied"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "Delay of 6/32 times GTCLK period applied"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "Delay of 7/32 times GTCLK period applied"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "Delay of 8/32 times GTCLK period applied"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "Delay of 9/32 times GTCLK period applied"]
        pub const _0_X_09: Self = Self::new(9);

        #[doc = "Delay of 10/32 times GTCLK period applied"]
        pub const _0_X_0_A: Self = Self::new(10);

        #[doc = "Delay of 11/32 times GTCLK period applied"]
        pub const _0_X_0_B: Self = Self::new(11);

        #[doc = "Delay of 12/32 times GTCLK period applied"]
        pub const _0_X_0_C: Self = Self::new(12);

        #[doc = "Delay of 13/32 times GTCLK period applied"]
        pub const _0_X_0_D: Self = Self::new(13);

        #[doc = "Delay of 14/32 times GTCLK period applied"]
        pub const _0_X_0_E: Self = Self::new(14);

        #[doc = "Delay of 15/32 times GTCLK period applied"]
        pub const _0_X_0_F: Self = Self::new(15);

        #[doc = "Delay of 16/32 times GTCLK period applied"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "Delay of 17/32 times GTCLK period applied"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "Delay of 18/32 times GTCLK period applied"]
        pub const _0_X_12: Self = Self::new(18);

        #[doc = "Delay of 19/32 times GTCLK period applied"]
        pub const _0_X_13: Self = Self::new(19);

        #[doc = "Delay of 20/32 times GTCLK period applied"]
        pub const _0_X_14: Self = Self::new(20);

        #[doc = "Delay of 21/32 times GTCLK period applied"]
        pub const _0_X_15: Self = Self::new(21);

        #[doc = "Delay of 22/32 times GTCLK period applied"]
        pub const _0_X_16: Self = Self::new(22);

        #[doc = "Delay of 23/32 times GTCLK period applied"]
        pub const _0_X_17: Self = Self::new(23);

        #[doc = "Delay of 24/32 times GTCLK period applied"]
        pub const _0_X_18: Self = Self::new(24);

        #[doc = "Delay of 25/32 times GTCLK period applied"]
        pub const _0_X_19: Self = Self::new(25);

        #[doc = "Delay of 26/32 times GTCLK period applied"]
        pub const _0_X_1_A: Self = Self::new(26);

        #[doc = "Delay of 27/32 times GTCLK period applied"]
        pub const _0_X_1_B: Self = Self::new(27);

        #[doc = "Delay of 28/32 times GTCLK period applied"]
        pub const _0_X_1_C: Self = Self::new(28);

        #[doc = "Delay of 29/32 times GTCLK period applied"]
        pub const _0_X_1_D: Self = Self::new(29);

        #[doc = "Delay of 30/32 times GTCLK period applied"]
        pub const _0_X_1_E: Self = Self::new(30);

        #[doc = "Delay of 31/32 times GTCLK period applied"]
        pub const _0_X_1_F: Self = Self::new(31);
    }
}
