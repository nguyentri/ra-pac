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
#[doc = r"Port Output Enable for GPT"]
unsafe impl ::core::marker::Send for super::Poeg {}
unsafe impl ::core::marker::Sync for super::Poeg {}
impl super::Poeg {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "POEG Group A Setting Register"]
    #[inline(always)]
    pub const fn poegga(
        &self,
    ) -> &'static crate::common::Reg<self::Poegga_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Poegga_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "GPT Output Stopping Control Group A Write Protection Register"]
    #[inline(always)]
    pub const fn gtoncwpa(
        &self,
    ) -> &'static crate::common::Reg<self::Gtoncwpa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtoncwpa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "GPT Output Stopping Control Group A Controlling Register"]
    #[inline(always)]
    pub const fn gtonccra(
        &self,
    ) -> &'static crate::common::Reg<self::Gtonccra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtonccra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "POEG Group B Setting Register"]
    #[inline(always)]
    pub const fn poeggb(
        &self,
    ) -> &'static crate::common::Reg<self::Poeggb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Poeggb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "GPT Output Stopping Control Group B Write Protection Register"]
    #[inline(always)]
    pub const fn gtoncwpb(
        &self,
    ) -> &'static crate::common::Reg<self::Gtoncwpb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtoncwpb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[doc = "GPT Output Stopping Control Group B Controlling Register"]
    #[inline(always)]
    pub const fn gtonccrb(
        &self,
    ) -> &'static crate::common::Reg<self::Gtonccrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtonccrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(324usize),
            )
        }
    }

    #[doc = "POEG Group C Setting Register"]
    #[inline(always)]
    pub const fn poeggc(
        &self,
    ) -> &'static crate::common::Reg<self::Poeggc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Poeggc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "GPT Output Stopping Control Group C Write Protection Register"]
    #[inline(always)]
    pub const fn gtoncwpc(
        &self,
    ) -> &'static crate::common::Reg<self::Gtoncwpc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtoncwpc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(576usize),
            )
        }
    }

    #[doc = "GPT Output Stopping Control Group C Controlling Register"]
    #[inline(always)]
    pub const fn gtonccrc(
        &self,
    ) -> &'static crate::common::Reg<self::Gtonccrc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtonccrc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(580usize),
            )
        }
    }

    #[doc = "POEG Group D Setting Register"]
    #[inline(always)]
    pub const fn poeggd(
        &self,
    ) -> &'static crate::common::Reg<self::Poeggd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Poeggd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[doc = "GPT Output Stopping Control Group D Write Protection Register"]
    #[inline(always)]
    pub const fn gtoncwpd(
        &self,
    ) -> &'static crate::common::Reg<self::Gtoncwpd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtoncwpd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(832usize),
            )
        }
    }

    #[doc = "GPT Output Stopping Control Group D Controlling Register"]
    #[inline(always)]
    pub const fn gtonccrd(
        &self,
    ) -> &'static crate::common::Reg<self::Gtonccrd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtonccrd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(836usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Poegga_SPEC;
impl crate::sealed::RegSpec for Poegga_SPEC {
    type DataType = u32;
}

#[doc = "POEG Group A Setting Register"]
pub type Poegga = crate::RegValueT<Poegga_SPEC>;

impl Poegga {
    #[doc = "Port Input Detection Flag"]
    #[inline(always)]
    pub fn pidf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        poegga::Pidf,
        poegga::Pidf,
        Poegga_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            poegga::Pidf,
            poegga::Pidf,
            Poegga_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GPT or ACMPHS Output Stop Request Detection Flag"]
    #[inline(always)]
    pub fn iocf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        poegga::Iocf,
        poegga::Iocf,
        Poegga_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            poegga::Iocf,
            poegga::Iocf,
            Poegga_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub fn ostpf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        poegga::Ostpf,
        poegga::Ostpf,
        Poegga_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            poegga::Ostpf,
            poegga::Ostpf,
            Poegga_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software Stop Flag"]
    #[inline(always)]
    pub fn ssf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        poegga::Ssf,
        poegga::Ssf,
        Poegga_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            poegga::Ssf,
            poegga::Ssf,
            Poegga_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Detection Enable"]
    #[inline(always)]
    pub fn pide(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        poegga::Pide,
        poegga::Pide,
        Poegga_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            poegga::Pide,
            poegga::Pide,
            Poegga_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GPT Output Stop Request Enable"]
    #[inline(always)]
    pub fn ioce(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        poegga::Ioce,
        poegga::Ioce,
        Poegga_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            poegga::Ioce,
            poegga::Ioce,
            Poegga_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable Stopping Output on Stopping of Oscillation"]
    #[inline(always)]
    pub fn ostpe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        poegga::Ostpe,
        poegga::Ostpe,
        Poegga_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            poegga::Ostpe,
            poegga::Ostpe,
            Poegga_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS0 Enable"]
    #[inline(always)]
    pub fn cdre0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        poegga::Cdre0,
        poegga::Cdre0,
        Poegga_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            poegga::Cdre0,
            poegga::Cdre0,
            Poegga_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS1 Enable"]
    #[inline(always)]
    pub fn cdre1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        poegga::Cdre1,
        poegga::Cdre1,
        Poegga_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            poegga::Cdre1,
            poegga::Cdre1,
            Poegga_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS2 Enable"]
    #[inline(always)]
    pub fn cdre2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        poegga::Cdre2,
        poegga::Cdre2,
        Poegga_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            poegga::Cdre2,
            poegga::Cdre2,
            Poegga_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS3 Enable"]
    #[inline(always)]
    pub fn cdre3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        poegga::Cdre3,
        poegga::Cdre3,
        Poegga_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            poegga::Cdre3,
            poegga::Cdre3,
            Poegga_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGn Input Status Flag"]
    #[inline(always)]
    pub fn st(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        poegga::St,
        poegga::St,
        Poegga_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            poegga::St,
            poegga::St,
            Poegga_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGn Input Inverting"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        poegga::Inv,
        poegga::Inv,
        Poegga_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            poegga::Inv,
            poegga::Inv,
            Poegga_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise filter Enable"]
    #[inline(always)]
    pub fn nfen(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        poegga::Nfen,
        poegga::Nfen,
        Poegga_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            poegga::Nfen,
            poegga::Nfen,
            Poegga_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        poegga::Nfcs,
        poegga::Nfcs,
        Poegga_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            poegga::Nfcs,
            poegga::Nfcs,
            Poegga_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Poegga {
    #[inline(always)]
    fn default() -> Poegga {
        <crate::RegValueT<Poegga_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod poegga {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidf_SPEC;
    pub type Pidf = crate::EnumBitfieldStruct<u8, Pidf_SPEC>;
    impl Pidf {
        #[doc = "The selected input level was not detected on the GTETRGn pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "The selected input level was detected on the GTETRGn pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iocf_SPEC;
    pub type Iocf = crate::EnumBitfieldStruct<u8, Iocf_SPEC>;
    impl Iocf {
        #[doc = "Neither stopping of GPT output nor a comparator edge was detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Either stopping of GPT output or comparator edge was detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpf_SPEC;
    pub type Ostpf = crate::EnumBitfieldStruct<u8, Ostpf_SPEC>;
    impl Ostpf {
        #[doc = "Stopping of oscillation was not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stopping of oscillation was detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssf_SPEC;
    pub type Ssf = crate::EnumBitfieldStruct<u8, Ssf_SPEC>;
    impl Ssf {
        #[doc = "Software has not stopped output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software has stopped output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pide_SPEC;
    pub type Pide = crate::EnumBitfieldStruct<u8, Pide_SPEC>;
    impl Pide {
        #[doc = "Detection of input levels on the corresponding GTETRGn pin is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection of input levels on the corresponding GTETRGn pin is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ioce_SPEC;
    pub type Ioce = crate::EnumBitfieldStruct<u8, Ioce_SPEC>;
    impl Ioce {
        #[doc = "Detection of stopping of output from the GPT is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection of stopping of output from the GPT is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpe_SPEC;
    pub type Ostpe = crate::EnumBitfieldStruct<u8, Ostpe_SPEC>;
    impl Ostpe {
        #[doc = "Detection of stopping of oscillation is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection of stopping of oscillation is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre0_SPEC;
    pub type Cdre0 = crate::EnumBitfieldStruct<u8, Cdre0_SPEC>;
    impl Cdre0 {
        #[doc = "Comparator edge detection 0 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 0 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre1_SPEC;
    pub type Cdre1 = crate::EnumBitfieldStruct<u8, Cdre1_SPEC>;
    impl Cdre1 {
        #[doc = "Comparator edge detection 1 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 1 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre2_SPEC;
    pub type Cdre2 = crate::EnumBitfieldStruct<u8, Cdre2_SPEC>;
    impl Cdre2 {
        #[doc = "Comparator edge detection 2 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 2 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre3_SPEC;
    pub type Cdre3 = crate::EnumBitfieldStruct<u8, Cdre3_SPEC>;
    impl Cdre3 {
        #[doc = "Comparator edge detection 3 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 3 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "The corresponding external trigger for output to the GPT is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "The corresponding external trigger for output to the GPT is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Input on the GTETRGn pin is not inverted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Input on the GTETRGn pin is inverted"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Digital noise filter on the GTETRGn pin is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Digital noise filter on the GTETRGn pin is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/1 clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/8 clock"]
        pub const _01: Self = Self::new(1);

        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/32 clock"]
        pub const _10: Self = Self::new(2);

        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/128 clock"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtoncwpa_SPEC;
impl crate::sealed::RegSpec for Gtoncwpa_SPEC {
    type DataType = u16;
}

#[doc = "GPT Output Stopping Control Group A Write Protection Register"]
pub type Gtoncwpa = crate::RegValueT<Gtoncwpa_SPEC>;

impl Gtoncwpa {
    #[doc = "Register Writing Disable"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtoncwpa::Wp,
        gtoncwpa::Wp,
        Gtoncwpa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtoncwpa::Wp,
            gtoncwpa::Wp,
            Gtoncwpa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn prkey(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gtoncwpa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gtoncwpa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtoncwpa {
    #[inline(always)]
    fn default() -> Gtoncwpa {
        <crate::RegValueT<Gtoncwpa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtoncwpa {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Writing to the GTONCCRn register is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing to the GTONCCRn register is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtonccra_SPEC;
impl crate::sealed::RegSpec for Gtonccra_SPEC {
    type DataType = u16;
}

#[doc = "GPT Output Stopping Control Group A Controlling Register"]
pub type Gtonccra = crate::RegValueT<Gtonccra_SPEC>;

impl Gtonccra {
    #[doc = "Direct Stopping Request Setting"]
    #[inline(always)]
    pub fn ne(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtonccra::Ne,
        gtonccra::Ne,
        Gtonccra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtonccra::Ne,
            gtonccra::Ne,
            Gtonccra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Stopping Request Selection"]
    #[inline(always)]
    pub fn nfs(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        gtonccra::Nfs,
        gtonccra::Nfs,
        Gtonccra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            gtonccra::Nfs,
            gtonccra::Nfs,
            Gtonccra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Stopping Request Active Sense"]
    #[inline(always)]
    pub fn nfv(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtonccra::Nfv,
        gtonccra::Nfv,
        Gtonccra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtonccra::Nfv,
            gtonccra::Nfv,
            Gtonccra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtonccra {
    #[inline(always)]
    fn default() -> Gtonccra {
        <crate::RegValueT<Gtonccra_SPEC> as RegisterValue<_>>::new(256)
    }
}
pub mod gtonccra {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ne_SPEC;
    pub type Ne = crate::EnumBitfieldStruct<u8, Ne_SPEC>;
    impl Ne {
        #[doc = "The signal for detection is not set as a direct stopping request signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "The signal for detection is set as a direct stopping request signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfs_SPEC;
    pub type Nfs = crate::EnumBitfieldStruct<u8, Nfs_SPEC>;
    impl Nfs {
        #[doc = "Comparator level detection 0"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Comparator level detection 1"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "Comparator level detection 2"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "Comparator level detection 3"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "GTETRGn pin input level detection (n = A to D)"]
        pub const _0_X_7: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfv_SPEC;
    pub type Nfv = crate::EnumBitfieldStruct<u8, Nfv_SPEC>;
    impl Nfv {
        #[doc = "Stopping output is requested when the output stopping detection signal is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stopping output is requested when the output stopping detection signal is 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Poeggb_SPEC;
impl crate::sealed::RegSpec for Poeggb_SPEC {
    type DataType = u32;
}

#[doc = "POEG Group B Setting Register"]
pub type Poeggb = crate::RegValueT<Poeggb_SPEC>;

impl Poeggb {
    #[doc = "Port Input Detection Flag"]
    #[inline(always)]
    pub fn pidf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        poeggb::Pidf,
        poeggb::Pidf,
        Poeggb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            poeggb::Pidf,
            poeggb::Pidf,
            Poeggb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GPT or ACMPHS Output Stop Request Detection Flag"]
    #[inline(always)]
    pub fn iocf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        poeggb::Iocf,
        poeggb::Iocf,
        Poeggb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            poeggb::Iocf,
            poeggb::Iocf,
            Poeggb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub fn ostpf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        poeggb::Ostpf,
        poeggb::Ostpf,
        Poeggb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            poeggb::Ostpf,
            poeggb::Ostpf,
            Poeggb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software Stop Flag"]
    #[inline(always)]
    pub fn ssf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        poeggb::Ssf,
        poeggb::Ssf,
        Poeggb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            poeggb::Ssf,
            poeggb::Ssf,
            Poeggb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Detection Enable"]
    #[inline(always)]
    pub fn pide(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        poeggb::Pide,
        poeggb::Pide,
        Poeggb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            poeggb::Pide,
            poeggb::Pide,
            Poeggb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GPT Output Stop Request Enable"]
    #[inline(always)]
    pub fn ioce(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        poeggb::Ioce,
        poeggb::Ioce,
        Poeggb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            poeggb::Ioce,
            poeggb::Ioce,
            Poeggb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable Stopping Output on Stopping of Oscillation"]
    #[inline(always)]
    pub fn ostpe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        poeggb::Ostpe,
        poeggb::Ostpe,
        Poeggb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            poeggb::Ostpe,
            poeggb::Ostpe,
            Poeggb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS0 Enable"]
    #[inline(always)]
    pub fn cdre0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        poeggb::Cdre0,
        poeggb::Cdre0,
        Poeggb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            poeggb::Cdre0,
            poeggb::Cdre0,
            Poeggb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS1 Enable"]
    #[inline(always)]
    pub fn cdre1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        poeggb::Cdre1,
        poeggb::Cdre1,
        Poeggb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            poeggb::Cdre1,
            poeggb::Cdre1,
            Poeggb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS2 Enable"]
    #[inline(always)]
    pub fn cdre2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        poeggb::Cdre2,
        poeggb::Cdre2,
        Poeggb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            poeggb::Cdre2,
            poeggb::Cdre2,
            Poeggb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS3 Enable"]
    #[inline(always)]
    pub fn cdre3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        poeggb::Cdre3,
        poeggb::Cdre3,
        Poeggb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            poeggb::Cdre3,
            poeggb::Cdre3,
            Poeggb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGn Input Status Flag"]
    #[inline(always)]
    pub fn st(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        poeggb::St,
        poeggb::St,
        Poeggb_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            poeggb::St,
            poeggb::St,
            Poeggb_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGn Input Inverting"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        poeggb::Inv,
        poeggb::Inv,
        Poeggb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            poeggb::Inv,
            poeggb::Inv,
            Poeggb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise filter Enable"]
    #[inline(always)]
    pub fn nfen(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        poeggb::Nfen,
        poeggb::Nfen,
        Poeggb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            poeggb::Nfen,
            poeggb::Nfen,
            Poeggb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        poeggb::Nfcs,
        poeggb::Nfcs,
        Poeggb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            poeggb::Nfcs,
            poeggb::Nfcs,
            Poeggb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Poeggb {
    #[inline(always)]
    fn default() -> Poeggb {
        <crate::RegValueT<Poeggb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod poeggb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidf_SPEC;
    pub type Pidf = crate::EnumBitfieldStruct<u8, Pidf_SPEC>;
    impl Pidf {
        #[doc = "The selected input level was not detected on the GTETRGn pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "The selected input level was detected on the GTETRGn pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iocf_SPEC;
    pub type Iocf = crate::EnumBitfieldStruct<u8, Iocf_SPEC>;
    impl Iocf {
        #[doc = "Neither stopping of GPT output nor a comparator edge was detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Either stopping of GPT output or comparator edge was detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpf_SPEC;
    pub type Ostpf = crate::EnumBitfieldStruct<u8, Ostpf_SPEC>;
    impl Ostpf {
        #[doc = "Stopping of oscillation was not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stopping of oscillation was detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssf_SPEC;
    pub type Ssf = crate::EnumBitfieldStruct<u8, Ssf_SPEC>;
    impl Ssf {
        #[doc = "Software has not stopped output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software has stopped output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pide_SPEC;
    pub type Pide = crate::EnumBitfieldStruct<u8, Pide_SPEC>;
    impl Pide {
        #[doc = "Detection of input levels on the corresponding GTETRGn pin is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection of input levels on the corresponding GTETRGn pin is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ioce_SPEC;
    pub type Ioce = crate::EnumBitfieldStruct<u8, Ioce_SPEC>;
    impl Ioce {
        #[doc = "Detection of stopping of output from the GPT is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection of stopping of output from the GPT is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpe_SPEC;
    pub type Ostpe = crate::EnumBitfieldStruct<u8, Ostpe_SPEC>;
    impl Ostpe {
        #[doc = "Detection of stopping of oscillation is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection of stopping of oscillation is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre0_SPEC;
    pub type Cdre0 = crate::EnumBitfieldStruct<u8, Cdre0_SPEC>;
    impl Cdre0 {
        #[doc = "Comparator edge detection 0 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 0 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre1_SPEC;
    pub type Cdre1 = crate::EnumBitfieldStruct<u8, Cdre1_SPEC>;
    impl Cdre1 {
        #[doc = "Comparator edge detection 1 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 1 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre2_SPEC;
    pub type Cdre2 = crate::EnumBitfieldStruct<u8, Cdre2_SPEC>;
    impl Cdre2 {
        #[doc = "Comparator edge detection 2 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 2 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre3_SPEC;
    pub type Cdre3 = crate::EnumBitfieldStruct<u8, Cdre3_SPEC>;
    impl Cdre3 {
        #[doc = "Comparator edge detection 3 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 3 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "The corresponding external trigger for output to the GPT is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "The corresponding external trigger for output to the GPT is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Input on the GTETRGn pin is not inverted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Input on the GTETRGn pin is inverted"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Digital noise filter on the GTETRGn pin is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Digital noise filter on the GTETRGn pin is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/1 clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/8 clock"]
        pub const _01: Self = Self::new(1);

        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/32 clock"]
        pub const _10: Self = Self::new(2);

        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/128 clock"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtoncwpb_SPEC;
impl crate::sealed::RegSpec for Gtoncwpb_SPEC {
    type DataType = u16;
}

#[doc = "GPT Output Stopping Control Group B Write Protection Register"]
pub type Gtoncwpb = crate::RegValueT<Gtoncwpb_SPEC>;

impl Gtoncwpb {
    #[doc = "Register Writing Disable"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtoncwpb::Wp,
        gtoncwpb::Wp,
        Gtoncwpb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtoncwpb::Wp,
            gtoncwpb::Wp,
            Gtoncwpb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn prkey(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gtoncwpb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gtoncwpb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtoncwpb {
    #[inline(always)]
    fn default() -> Gtoncwpb {
        <crate::RegValueT<Gtoncwpb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtoncwpb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Writing to the GTONCCRn register is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing to the GTONCCRn register is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtonccrb_SPEC;
impl crate::sealed::RegSpec for Gtonccrb_SPEC {
    type DataType = u16;
}

#[doc = "GPT Output Stopping Control Group B Controlling Register"]
pub type Gtonccrb = crate::RegValueT<Gtonccrb_SPEC>;

impl Gtonccrb {
    #[doc = "Direct Stopping Request Setting"]
    #[inline(always)]
    pub fn ne(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtonccrb::Ne,
        gtonccrb::Ne,
        Gtonccrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtonccrb::Ne,
            gtonccrb::Ne,
            Gtonccrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Stopping Request Selection"]
    #[inline(always)]
    pub fn nfs(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        gtonccrb::Nfs,
        gtonccrb::Nfs,
        Gtonccrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            gtonccrb::Nfs,
            gtonccrb::Nfs,
            Gtonccrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Stopping Request Active Sense"]
    #[inline(always)]
    pub fn nfv(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtonccrb::Nfv,
        gtonccrb::Nfv,
        Gtonccrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtonccrb::Nfv,
            gtonccrb::Nfv,
            Gtonccrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtonccrb {
    #[inline(always)]
    fn default() -> Gtonccrb {
        <crate::RegValueT<Gtonccrb_SPEC> as RegisterValue<_>>::new(256)
    }
}
pub mod gtonccrb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ne_SPEC;
    pub type Ne = crate::EnumBitfieldStruct<u8, Ne_SPEC>;
    impl Ne {
        #[doc = "The signal for detection is not set as a direct stopping request signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "The signal for detection is set as a direct stopping request signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfs_SPEC;
    pub type Nfs = crate::EnumBitfieldStruct<u8, Nfs_SPEC>;
    impl Nfs {
        #[doc = "Comparator level detection 0"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Comparator level detection 1"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "Comparator level detection 2"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "Comparator level detection 3"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "GTETRGn pin input level detection (n = A to D)"]
        pub const _0_X_7: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfv_SPEC;
    pub type Nfv = crate::EnumBitfieldStruct<u8, Nfv_SPEC>;
    impl Nfv {
        #[doc = "Stopping output is requested when the output stopping detection signal is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stopping output is requested when the output stopping detection signal is 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Poeggc_SPEC;
impl crate::sealed::RegSpec for Poeggc_SPEC {
    type DataType = u32;
}

#[doc = "POEG Group C Setting Register"]
pub type Poeggc = crate::RegValueT<Poeggc_SPEC>;

impl Poeggc {
    #[doc = "Port Input Detection Flag"]
    #[inline(always)]
    pub fn pidf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        poeggc::Pidf,
        poeggc::Pidf,
        Poeggc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            poeggc::Pidf,
            poeggc::Pidf,
            Poeggc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GPT or ACMPHS Output Stop Request Detection Flag"]
    #[inline(always)]
    pub fn iocf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        poeggc::Iocf,
        poeggc::Iocf,
        Poeggc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            poeggc::Iocf,
            poeggc::Iocf,
            Poeggc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub fn ostpf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        poeggc::Ostpf,
        poeggc::Ostpf,
        Poeggc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            poeggc::Ostpf,
            poeggc::Ostpf,
            Poeggc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software Stop Flag"]
    #[inline(always)]
    pub fn ssf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        poeggc::Ssf,
        poeggc::Ssf,
        Poeggc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            poeggc::Ssf,
            poeggc::Ssf,
            Poeggc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Detection Enable"]
    #[inline(always)]
    pub fn pide(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        poeggc::Pide,
        poeggc::Pide,
        Poeggc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            poeggc::Pide,
            poeggc::Pide,
            Poeggc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GPT Output Stop Request Enable"]
    #[inline(always)]
    pub fn ioce(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        poeggc::Ioce,
        poeggc::Ioce,
        Poeggc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            poeggc::Ioce,
            poeggc::Ioce,
            Poeggc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable Stopping Output on Stopping of Oscillation"]
    #[inline(always)]
    pub fn ostpe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        poeggc::Ostpe,
        poeggc::Ostpe,
        Poeggc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            poeggc::Ostpe,
            poeggc::Ostpe,
            Poeggc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS0 Enable"]
    #[inline(always)]
    pub fn cdre0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        poeggc::Cdre0,
        poeggc::Cdre0,
        Poeggc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            poeggc::Cdre0,
            poeggc::Cdre0,
            Poeggc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS1 Enable"]
    #[inline(always)]
    pub fn cdre1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        poeggc::Cdre1,
        poeggc::Cdre1,
        Poeggc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            poeggc::Cdre1,
            poeggc::Cdre1,
            Poeggc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS2 Enable"]
    #[inline(always)]
    pub fn cdre2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        poeggc::Cdre2,
        poeggc::Cdre2,
        Poeggc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            poeggc::Cdre2,
            poeggc::Cdre2,
            Poeggc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS3 Enable"]
    #[inline(always)]
    pub fn cdre3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        poeggc::Cdre3,
        poeggc::Cdre3,
        Poeggc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            poeggc::Cdre3,
            poeggc::Cdre3,
            Poeggc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGn Input Status Flag"]
    #[inline(always)]
    pub fn st(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        poeggc::St,
        poeggc::St,
        Poeggc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            poeggc::St,
            poeggc::St,
            Poeggc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGn Input Inverting"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        poeggc::Inv,
        poeggc::Inv,
        Poeggc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            poeggc::Inv,
            poeggc::Inv,
            Poeggc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise filter Enable"]
    #[inline(always)]
    pub fn nfen(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        poeggc::Nfen,
        poeggc::Nfen,
        Poeggc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            poeggc::Nfen,
            poeggc::Nfen,
            Poeggc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        poeggc::Nfcs,
        poeggc::Nfcs,
        Poeggc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            poeggc::Nfcs,
            poeggc::Nfcs,
            Poeggc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Poeggc {
    #[inline(always)]
    fn default() -> Poeggc {
        <crate::RegValueT<Poeggc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod poeggc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidf_SPEC;
    pub type Pidf = crate::EnumBitfieldStruct<u8, Pidf_SPEC>;
    impl Pidf {
        #[doc = "The selected input level was not detected on the GTETRGn pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "The selected input level was detected on the GTETRGn pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iocf_SPEC;
    pub type Iocf = crate::EnumBitfieldStruct<u8, Iocf_SPEC>;
    impl Iocf {
        #[doc = "Neither stopping of GPT output nor a comparator edge was detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Either stopping of GPT output or comparator edge was detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpf_SPEC;
    pub type Ostpf = crate::EnumBitfieldStruct<u8, Ostpf_SPEC>;
    impl Ostpf {
        #[doc = "Stopping of oscillation was not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stopping of oscillation was detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssf_SPEC;
    pub type Ssf = crate::EnumBitfieldStruct<u8, Ssf_SPEC>;
    impl Ssf {
        #[doc = "Software has not stopped output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software has stopped output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pide_SPEC;
    pub type Pide = crate::EnumBitfieldStruct<u8, Pide_SPEC>;
    impl Pide {
        #[doc = "Detection of input levels on the corresponding GTETRGn pin is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection of input levels on the corresponding GTETRGn pin is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ioce_SPEC;
    pub type Ioce = crate::EnumBitfieldStruct<u8, Ioce_SPEC>;
    impl Ioce {
        #[doc = "Detection of stopping of output from the GPT is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection of stopping of output from the GPT is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpe_SPEC;
    pub type Ostpe = crate::EnumBitfieldStruct<u8, Ostpe_SPEC>;
    impl Ostpe {
        #[doc = "Detection of stopping of oscillation is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection of stopping of oscillation is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre0_SPEC;
    pub type Cdre0 = crate::EnumBitfieldStruct<u8, Cdre0_SPEC>;
    impl Cdre0 {
        #[doc = "Comparator edge detection 0 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 0 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre1_SPEC;
    pub type Cdre1 = crate::EnumBitfieldStruct<u8, Cdre1_SPEC>;
    impl Cdre1 {
        #[doc = "Comparator edge detection 1 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 1 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre2_SPEC;
    pub type Cdre2 = crate::EnumBitfieldStruct<u8, Cdre2_SPEC>;
    impl Cdre2 {
        #[doc = "Comparator edge detection 2 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 2 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre3_SPEC;
    pub type Cdre3 = crate::EnumBitfieldStruct<u8, Cdre3_SPEC>;
    impl Cdre3 {
        #[doc = "Comparator edge detection 3 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 3 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "The corresponding external trigger for output to the GPT is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "The corresponding external trigger for output to the GPT is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Input on the GTETRGn pin is not inverted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Input on the GTETRGn pin is inverted"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Digital noise filter on the GTETRGn pin is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Digital noise filter on the GTETRGn pin is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/1 clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/8 clock"]
        pub const _01: Self = Self::new(1);

        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/32 clock"]
        pub const _10: Self = Self::new(2);

        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/128 clock"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtoncwpc_SPEC;
impl crate::sealed::RegSpec for Gtoncwpc_SPEC {
    type DataType = u16;
}

#[doc = "GPT Output Stopping Control Group C Write Protection Register"]
pub type Gtoncwpc = crate::RegValueT<Gtoncwpc_SPEC>;

impl Gtoncwpc {
    #[doc = "Register Writing Disable"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtoncwpc::Wp,
        gtoncwpc::Wp,
        Gtoncwpc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtoncwpc::Wp,
            gtoncwpc::Wp,
            Gtoncwpc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn prkey(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gtoncwpc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gtoncwpc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtoncwpc {
    #[inline(always)]
    fn default() -> Gtoncwpc {
        <crate::RegValueT<Gtoncwpc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtoncwpc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Writing to the GTONCCRn register is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing to the GTONCCRn register is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtonccrc_SPEC;
impl crate::sealed::RegSpec for Gtonccrc_SPEC {
    type DataType = u16;
}

#[doc = "GPT Output Stopping Control Group C Controlling Register"]
pub type Gtonccrc = crate::RegValueT<Gtonccrc_SPEC>;

impl Gtonccrc {
    #[doc = "Direct Stopping Request Setting"]
    #[inline(always)]
    pub fn ne(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtonccrc::Ne,
        gtonccrc::Ne,
        Gtonccrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtonccrc::Ne,
            gtonccrc::Ne,
            Gtonccrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Stopping Request Selection"]
    #[inline(always)]
    pub fn nfs(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        gtonccrc::Nfs,
        gtonccrc::Nfs,
        Gtonccrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            gtonccrc::Nfs,
            gtonccrc::Nfs,
            Gtonccrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Stopping Request Active Sense"]
    #[inline(always)]
    pub fn nfv(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtonccrc::Nfv,
        gtonccrc::Nfv,
        Gtonccrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtonccrc::Nfv,
            gtonccrc::Nfv,
            Gtonccrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtonccrc {
    #[inline(always)]
    fn default() -> Gtonccrc {
        <crate::RegValueT<Gtonccrc_SPEC> as RegisterValue<_>>::new(256)
    }
}
pub mod gtonccrc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ne_SPEC;
    pub type Ne = crate::EnumBitfieldStruct<u8, Ne_SPEC>;
    impl Ne {
        #[doc = "The signal for detection is not set as a direct stopping request signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "The signal for detection is set as a direct stopping request signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfs_SPEC;
    pub type Nfs = crate::EnumBitfieldStruct<u8, Nfs_SPEC>;
    impl Nfs {
        #[doc = "Comparator level detection 0"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Comparator level detection 1"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "Comparator level detection 2"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "Comparator level detection 3"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "GTETRGn pin input level detection (n = A to D)"]
        pub const _0_X_7: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfv_SPEC;
    pub type Nfv = crate::EnumBitfieldStruct<u8, Nfv_SPEC>;
    impl Nfv {
        #[doc = "Stopping output is requested when the output stopping detection signal is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stopping output is requested when the output stopping detection signal is 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Poeggd_SPEC;
impl crate::sealed::RegSpec for Poeggd_SPEC {
    type DataType = u32;
}

#[doc = "POEG Group D Setting Register"]
pub type Poeggd = crate::RegValueT<Poeggd_SPEC>;

impl Poeggd {
    #[doc = "Port Input Detection Flag"]
    #[inline(always)]
    pub fn pidf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        poeggd::Pidf,
        poeggd::Pidf,
        Poeggd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            poeggd::Pidf,
            poeggd::Pidf,
            Poeggd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GPT or ACMPHS Output Stop Request Detection Flag"]
    #[inline(always)]
    pub fn iocf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        poeggd::Iocf,
        poeggd::Iocf,
        Poeggd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            poeggd::Iocf,
            poeggd::Iocf,
            Poeggd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub fn ostpf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        poeggd::Ostpf,
        poeggd::Ostpf,
        Poeggd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            poeggd::Ostpf,
            poeggd::Ostpf,
            Poeggd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software Stop Flag"]
    #[inline(always)]
    pub fn ssf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        poeggd::Ssf,
        poeggd::Ssf,
        Poeggd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            poeggd::Ssf,
            poeggd::Ssf,
            Poeggd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Detection Enable"]
    #[inline(always)]
    pub fn pide(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        poeggd::Pide,
        poeggd::Pide,
        Poeggd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            poeggd::Pide,
            poeggd::Pide,
            Poeggd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GPT Output Stop Request Enable"]
    #[inline(always)]
    pub fn ioce(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        poeggd::Ioce,
        poeggd::Ioce,
        Poeggd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            poeggd::Ioce,
            poeggd::Ioce,
            Poeggd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable Stopping Output on Stopping of Oscillation"]
    #[inline(always)]
    pub fn ostpe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        poeggd::Ostpe,
        poeggd::Ostpe,
        Poeggd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            poeggd::Ostpe,
            poeggd::Ostpe,
            Poeggd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS0 Enable"]
    #[inline(always)]
    pub fn cdre0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        poeggd::Cdre0,
        poeggd::Cdre0,
        Poeggd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            poeggd::Cdre0,
            poeggd::Cdre0,
            Poeggd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS1 Enable"]
    #[inline(always)]
    pub fn cdre1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        poeggd::Cdre1,
        poeggd::Cdre1,
        Poeggd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            poeggd::Cdre1,
            poeggd::Cdre1,
            Poeggd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS2 Enable"]
    #[inline(always)]
    pub fn cdre2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        poeggd::Cdre2,
        poeggd::Cdre2,
        Poeggd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            poeggd::Cdre2,
            poeggd::Cdre2,
            Poeggd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS3 Enable"]
    #[inline(always)]
    pub fn cdre3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        poeggd::Cdre3,
        poeggd::Cdre3,
        Poeggd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            poeggd::Cdre3,
            poeggd::Cdre3,
            Poeggd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGn Input Status Flag"]
    #[inline(always)]
    pub fn st(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        poeggd::St,
        poeggd::St,
        Poeggd_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            poeggd::St,
            poeggd::St,
            Poeggd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "GTETRGn Input Inverting"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        poeggd::Inv,
        poeggd::Inv,
        Poeggd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            poeggd::Inv,
            poeggd::Inv,
            Poeggd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise filter Enable"]
    #[inline(always)]
    pub fn nfen(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        poeggd::Nfen,
        poeggd::Nfen,
        Poeggd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            poeggd::Nfen,
            poeggd::Nfen,
            Poeggd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        poeggd::Nfcs,
        poeggd::Nfcs,
        Poeggd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            poeggd::Nfcs,
            poeggd::Nfcs,
            Poeggd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Poeggd {
    #[inline(always)]
    fn default() -> Poeggd {
        <crate::RegValueT<Poeggd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod poeggd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidf_SPEC;
    pub type Pidf = crate::EnumBitfieldStruct<u8, Pidf_SPEC>;
    impl Pidf {
        #[doc = "The selected input level was not detected on the GTETRGn pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "The selected input level was detected on the GTETRGn pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iocf_SPEC;
    pub type Iocf = crate::EnumBitfieldStruct<u8, Iocf_SPEC>;
    impl Iocf {
        #[doc = "Neither stopping of GPT output nor a comparator edge was detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Either stopping of GPT output or comparator edge was detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpf_SPEC;
    pub type Ostpf = crate::EnumBitfieldStruct<u8, Ostpf_SPEC>;
    impl Ostpf {
        #[doc = "Stopping of oscillation was not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stopping of oscillation was detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssf_SPEC;
    pub type Ssf = crate::EnumBitfieldStruct<u8, Ssf_SPEC>;
    impl Ssf {
        #[doc = "Software has not stopped output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software has stopped output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pide_SPEC;
    pub type Pide = crate::EnumBitfieldStruct<u8, Pide_SPEC>;
    impl Pide {
        #[doc = "Detection of input levels on the corresponding GTETRGn pin is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection of input levels on the corresponding GTETRGn pin is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ioce_SPEC;
    pub type Ioce = crate::EnumBitfieldStruct<u8, Ioce_SPEC>;
    impl Ioce {
        #[doc = "Detection of stopping of output from the GPT is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection of stopping of output from the GPT is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpe_SPEC;
    pub type Ostpe = crate::EnumBitfieldStruct<u8, Ostpe_SPEC>;
    impl Ostpe {
        #[doc = "Detection of stopping of oscillation is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection of stopping of oscillation is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre0_SPEC;
    pub type Cdre0 = crate::EnumBitfieldStruct<u8, Cdre0_SPEC>;
    impl Cdre0 {
        #[doc = "Comparator edge detection 0 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 0 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre1_SPEC;
    pub type Cdre1 = crate::EnumBitfieldStruct<u8, Cdre1_SPEC>;
    impl Cdre1 {
        #[doc = "Comparator edge detection 1 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 1 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre2_SPEC;
    pub type Cdre2 = crate::EnumBitfieldStruct<u8, Cdre2_SPEC>;
    impl Cdre2 {
        #[doc = "Comparator edge detection 2 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 2 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre3_SPEC;
    pub type Cdre3 = crate::EnumBitfieldStruct<u8, Cdre3_SPEC>;
    impl Cdre3 {
        #[doc = "Comparator edge detection 3 is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparator edge detection 3 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "The corresponding external trigger for output to the GPT is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "The corresponding external trigger for output to the GPT is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Input on the GTETRGn pin is not inverted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Input on the GTETRGn pin is inverted"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Digital noise filter on the GTETRGn pin is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Digital noise filter on the GTETRGn pin is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/1 clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/8 clock"]
        pub const _01: Self = Self::new(1);

        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/32 clock"]
        pub const _10: Self = Self::new(2);

        #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/128 clock"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtoncwpd_SPEC;
impl crate::sealed::RegSpec for Gtoncwpd_SPEC {
    type DataType = u16;
}

#[doc = "GPT Output Stopping Control Group D Write Protection Register"]
pub type Gtoncwpd = crate::RegValueT<Gtoncwpd_SPEC>;

impl Gtoncwpd {
    #[doc = "Register Writing Disable"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtoncwpd::Wp,
        gtoncwpd::Wp,
        Gtoncwpd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtoncwpd::Wp,
            gtoncwpd::Wp,
            Gtoncwpd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn prkey(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gtoncwpd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gtoncwpd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtoncwpd {
    #[inline(always)]
    fn default() -> Gtoncwpd {
        <crate::RegValueT<Gtoncwpd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtoncwpd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Writing to the GTONCCRn register is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing to the GTONCCRn register is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtonccrd_SPEC;
impl crate::sealed::RegSpec for Gtonccrd_SPEC {
    type DataType = u16;
}

#[doc = "GPT Output Stopping Control Group D Controlling Register"]
pub type Gtonccrd = crate::RegValueT<Gtonccrd_SPEC>;

impl Gtonccrd {
    #[doc = "Direct Stopping Request Setting"]
    #[inline(always)]
    pub fn ne(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtonccrd::Ne,
        gtonccrd::Ne,
        Gtonccrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtonccrd::Ne,
            gtonccrd::Ne,
            Gtonccrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Stopping Request Selection"]
    #[inline(always)]
    pub fn nfs(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        gtonccrd::Nfs,
        gtonccrd::Nfs,
        Gtonccrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            gtonccrd::Nfs,
            gtonccrd::Nfs,
            Gtonccrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Stopping Request Active Sense"]
    #[inline(always)]
    pub fn nfv(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtonccrd::Nfv,
        gtonccrd::Nfv,
        Gtonccrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtonccrd::Nfv,
            gtonccrd::Nfv,
            Gtonccrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtonccrd {
    #[inline(always)]
    fn default() -> Gtonccrd {
        <crate::RegValueT<Gtonccrd_SPEC> as RegisterValue<_>>::new(256)
    }
}
pub mod gtonccrd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ne_SPEC;
    pub type Ne = crate::EnumBitfieldStruct<u8, Ne_SPEC>;
    impl Ne {
        #[doc = "The signal for detection is not set as a direct stopping request signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "The signal for detection is set as a direct stopping request signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfs_SPEC;
    pub type Nfs = crate::EnumBitfieldStruct<u8, Nfs_SPEC>;
    impl Nfs {
        #[doc = "Comparator level detection 0"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Comparator level detection 1"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "Comparator level detection 2"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "Comparator level detection 3"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "GTETRGn pin input level detection (n = A to D)"]
        pub const _0_X_7: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfv_SPEC;
    pub type Nfv = crate::EnumBitfieldStruct<u8, Nfv_SPEC>;
    impl Nfv {
        #[doc = "Stopping output is requested when the output stopping detection signal is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stopping output is requested when the output stopping detection signal is 1"]
        pub const _1: Self = Self::new(1);
    }
}
