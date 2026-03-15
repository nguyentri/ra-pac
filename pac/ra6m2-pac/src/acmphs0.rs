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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:11:44 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"High-Speed Analog Comparator 0"]
unsafe impl ::core::marker::Send for super::Acmphs0 {}
unsafe impl ::core::marker::Sync for super::Acmphs0 {}
impl super::Acmphs0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Comparator Control Register"]
    #[inline(always)]
    pub const fn cmpctl(
        &self,
    ) -> &'static crate::common::Reg<self::Cmpctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmpctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Comparator Input Select Register"]
    #[inline(always)]
    pub const fn cmpsel0(
        &self,
    ) -> &'static crate::common::Reg<self::Cmpsel0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmpsel0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Comparator Reference Voltage Select Register"]
    #[inline(always)]
    pub const fn cmpsel1(
        &self,
    ) -> &'static crate::common::Reg<self::Cmpsel1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmpsel1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Comparator Output Monitor Register"]
    #[inline(always)]
    pub const fn cmpmon(&self) -> &'static crate::common::Reg<self::Cmpmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cmpmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Comparator Output Control Register"]
    #[inline(always)]
    pub const fn cpioc(&self) -> &'static crate::common::Reg<self::Cpioc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpioc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpctl_SPEC;
impl crate::sealed::RegSpec for Cmpctl_SPEC {
    type DataType = u8;
}

#[doc = "Comparator Control Register"]
pub type Cmpctl = crate::RegValueT<Cmpctl_SPEC>;

impl Cmpctl {
    #[doc = "Comparator operation control"]
    #[inline(always)]
    pub fn hcmpon(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cmpctl::Hcmpon,
        cmpctl::Hcmpon,
        Cmpctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cmpctl::Hcmpon,
            cmpctl::Hcmpon,
            Cmpctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise filter selection"]
    #[inline(always)]
    pub fn cdfs(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x3,
        1,
        0,
        cmpctl::Cdfs,
        cmpctl::Cdfs,
        Cmpctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x3,
            1,
            0,
            cmpctl::Cdfs,
            cmpctl::Cdfs,
            Cmpctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of valid edge (Edge selector)"]
    #[inline(always)]
    pub fn ceg(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x3,
        1,
        0,
        cmpctl::Ceg,
        cmpctl::Ceg,
        Cmpctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x3,
            1,
            0,
            cmpctl::Ceg,
            cmpctl::Ceg,
            Cmpctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Select"]
    #[inline(always)]
    pub fn csten(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cmpctl::Csten,
        cmpctl::Csten,
        Cmpctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cmpctl::Csten,
            cmpctl::Csten,
            Cmpctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparator output enable"]
    #[inline(always)]
    pub fn coe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cmpctl::Coe,
        cmpctl::Coe,
        Cmpctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cmpctl::Coe,
            cmpctl::Coe,
            Cmpctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparator output polarity selection"]
    #[inline(always)]
    pub fn cinv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cmpctl::Cinv,
        cmpctl::Cinv,
        Cmpctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cmpctl::Cinv,
            cmpctl::Cinv,
            Cmpctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cmpctl {
    #[inline(always)]
    fn default() -> Cmpctl {
        <crate::RegValueT<Cmpctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cmpctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hcmpon_SPEC;
    pub type Hcmpon = crate::EnumBitfieldStruct<u8, Hcmpon_SPEC>;
    impl Hcmpon {
        #[doc = "Operation stopped (the comparator outputs a low-level signal)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operation enabled (input to the comparator pins is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdfs_SPEC;
    pub type Cdfs = crate::EnumBitfieldStruct<u8, Cdfs_SPEC>;
    impl Cdfs {
        #[doc = "Noise filter not used."]
        pub const _00: Self = Self::new(0);

        #[doc = "Noise filter sampling frequency is 2^3/PCLKB."]
        pub const _01: Self = Self::new(1);

        #[doc = "Noise filter sampling frequency is 2^4/PCLKB."]
        pub const _10: Self = Self::new(2);

        #[doc = "Noise filter sampling frequency is 2^5/PCLKB."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ceg_SPEC;
    pub type Ceg = crate::EnumBitfieldStruct<u8, Ceg_SPEC>;
    impl Ceg {
        #[doc = "No edge selection."]
        pub const _00: Self = Self::new(0);

        #[doc = "Rising edge selection."]
        pub const _01: Self = Self::new(1);

        #[doc = "Falling edge selection"]
        pub const _10: Self = Self::new(2);

        #[doc = "Both-edge selection"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csten_SPEC;
    pub type Csten = crate::EnumBitfieldStruct<u8, Csten_SPEC>;
    impl Csten {
        #[doc = "Output via the Edge selector"]
        pub const _0: Self = Self::new(0);

        #[doc = "Direct output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Coe_SPEC;
    pub type Coe = crate::EnumBitfieldStruct<u8, Coe_SPEC>;
    impl Coe {
        #[doc = "Comparator output disabled (the output signal is low level)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cinv_SPEC;
    pub type Cinv = crate::EnumBitfieldStruct<u8, Cinv_SPEC>;
    impl Cinv {
        #[doc = "Comparator output not inverted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator output inverted"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpsel0_SPEC;
impl crate::sealed::RegSpec for Cmpsel0_SPEC {
    type DataType = u8;
}

#[doc = "Comparator Input Select Register"]
pub type Cmpsel0 = crate::RegValueT<Cmpsel0_SPEC>;

impl Cmpsel0 {
    #[doc = "Comparator input selection"]
    #[inline(always)]
    pub fn cmpsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cmpsel0::Cmpsel,
        cmpsel0::Cmpsel,
        Cmpsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            cmpsel0::Cmpsel,
            cmpsel0::Cmpsel,
            Cmpsel0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cmpsel0 {
    #[inline(always)]
    fn default() -> Cmpsel0 {
        <crate::RegValueT<Cmpsel0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cmpsel0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpsel_SPEC;
    pub type Cmpsel = crate::EnumBitfieldStruct<u8, Cmpsel_SPEC>;
    impl Cmpsel {
        #[doc = "No input"]
        pub const _0000: Self = Self::new(0);

        #[doc = "IVCMP0 selected"]
        pub const _0001: Self = Self::new(1);

        #[doc = "IVCMP1 selected"]
        pub const _0010: Self = Self::new(2);

        #[doc = "IVCMP2 selected"]
        pub const _0100: Self = Self::new(4);

        #[doc = "IVCMP3 selected"]
        pub const _1000: Self = Self::new(8);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpsel1_SPEC;
impl crate::sealed::RegSpec for Cmpsel1_SPEC {
    type DataType = u8;
}

#[doc = "Comparator Reference Voltage Select Register"]
pub type Cmpsel1 = crate::RegValueT<Cmpsel1_SPEC>;

impl Cmpsel1 {
    #[doc = "Reference voltage selection"]
    #[inline(always)]
    pub fn crvs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cmpsel1::Crvs,
        cmpsel1::Crvs,
        Cmpsel1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            cmpsel1::Crvs,
            cmpsel1::Crvs,
            Cmpsel1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cmpsel1 {
    #[inline(always)]
    fn default() -> Cmpsel1 {
        <crate::RegValueT<Cmpsel1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cmpsel1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crvs_SPEC;
    pub type Crvs = crate::EnumBitfieldStruct<u8, Crvs_SPEC>;
    impl Crvs {
        #[doc = "No reference voltage"]
        pub const _0000: Self = Self::new(0);

        #[doc = "IVREF0 selected"]
        pub const _0001: Self = Self::new(1);

        #[doc = "IVREF1 selected"]
        pub const _0010: Self = Self::new(2);

        #[doc = "IVREF2 selected"]
        pub const _0100: Self = Self::new(4);

        #[doc = "IVREF3 selected"]
        pub const _1000: Self = Self::new(8);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpmon_SPEC;
impl crate::sealed::RegSpec for Cmpmon_SPEC {
    type DataType = u8;
}

#[doc = "Comparator Output Monitor Register"]
pub type Cmpmon = crate::RegValueT<Cmpmon_SPEC>;

impl Cmpmon {
    #[doc = "Comparator output monitor"]
    #[inline(always)]
    pub fn cmpmon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cmpmon::Cmpmon,
        cmpmon::Cmpmon,
        Cmpmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cmpmon::Cmpmon,
            cmpmon::Cmpmon,
            Cmpmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cmpmon {
    #[inline(always)]
    fn default() -> Cmpmon {
        <crate::RegValueT<Cmpmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cmpmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpmon_SPEC;
    pub type Cmpmon = crate::EnumBitfieldStruct<u8, Cmpmon_SPEC>;
    impl Cmpmon {
        #[doc = "Comparator output Low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator output High"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpioc_SPEC;
impl crate::sealed::RegSpec for Cpioc_SPEC {
    type DataType = u8;
}

#[doc = "Comparator Output Control Register"]
pub type Cpioc = crate::RegValueT<Cpioc_SPEC>;

impl Cpioc {
    #[doc = "Internal Vref enable"]
    #[inline(always)]
    pub fn vrefen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cpioc::Vrefen,
        cpioc::Vrefen,
        Cpioc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cpioc::Vrefen,
            cpioc::Vrefen,
            Cpioc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparator output selection"]
    #[inline(always)]
    pub fn cpoe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cpioc::Cpoe,
        cpioc::Cpoe,
        Cpioc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cpioc::Cpoe,
            cpioc::Cpoe,
            Cpioc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cpioc {
    #[inline(always)]
    fn default() -> Cpioc {
        <crate::RegValueT<Cpioc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpioc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vrefen_SPEC;
    pub type Vrefen = crate::EnumBitfieldStruct<u8, Vrefen_SPEC>;
    impl Vrefen {
        #[doc = "Internal Vref disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal Vref enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpoe_SPEC;
    pub type Cpoe = crate::EnumBitfieldStruct<u8, Cpoe_SPEC>;
    impl Cpoe {
        #[doc = "VCOUT pin output of the comparator is disabled (the output signal is low level)."]
        pub const _0: Self = Self::new(0);

        #[doc = "VCOUT pin output of the comparator is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
