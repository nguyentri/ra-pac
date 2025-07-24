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
#[doc = r"Port Output Enable Module for GPT"]
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

    #[doc = "Detection Flag for GPT Output-Disable Request"]
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

    #[doc = "Enable for GPT Output-Disable Request"]
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

    #[doc = "Oscillation Stop Detection Enable"]
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

    #[doc = "GTETRGn Input Reverse"]
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

    #[doc = "Noise Filter Enable"]
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

    #[doc = "Noise Filter Clock Select"]
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
        #[doc = "No output-disable request from the GTETRGn pin occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from the GTETRGn pin occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iocf_SPEC;
    pub type Iocf = crate::EnumBitfieldStruct<u8, Iocf_SPEC>;
    impl Iocf {
        #[doc = "No output-disable request from GPT occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from GPT occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpf_SPEC;
    pub type Ostpf = crate::EnumBitfieldStruct<u8, Ostpf_SPEC>;
    impl Ostpf {
        #[doc = "No output-disable request from oscillation stop detection occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from oscillation stop detection occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssf_SPEC;
    pub type Ssf = crate::EnumBitfieldStruct<u8, Ssf_SPEC>;
    impl Ssf {
        #[doc = "No output-disable request from software occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from software occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pide_SPEC;
    pub type Pide = crate::EnumBitfieldStruct<u8, Pide_SPEC>;
    impl Pide {
        #[doc = "Disable output-disable requests from the GTETRGn pins"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable output-disable requests from the GTETRGn pins"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ioce_SPEC;
    pub type Ioce = crate::EnumBitfieldStruct<u8, Ioce_SPEC>;
    impl Ioce {
        #[doc = "Disable output-disable requests from GPT"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable output-disable requests from GPT"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpe_SPEC;
    pub type Ostpe = crate::EnumBitfieldStruct<u8, Ostpe_SPEC>;
    impl Ostpe {
        #[doc = "Disable output-disable requests from oscillation stop detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable output-disable requests from oscillation stop detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "GTETRGn input after filtering was 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTETRGn input after filtering was 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Input GTETRGn as-is"]
        pub const _0: Self = Self::new(0);

        #[doc = "Input GTETRGn in reverse"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Disable noise filtering"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable noise filtering"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "Sample GTETRGn pin input level three times every PCLKB"]
        pub const _00: Self = Self::new(0);

        #[doc = "Sample GTETRGn pin input level three times every PCLKB/8"]
        pub const _01: Self = Self::new(1);

        #[doc = "Sample GTETRGn pin input level three times every PCLKB/32"]
        pub const _10: Self = Self::new(2);

        #[doc = "Sample GTETRGn pin input level three times every PCLKB/128"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Detection Flag for GPT Output-Disable Request"]
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

    #[doc = "Enable for GPT Output-Disable Request"]
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

    #[doc = "Oscillation Stop Detection Enable"]
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

    #[doc = "GTETRGn Input Reverse"]
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

    #[doc = "Noise Filter Enable"]
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

    #[doc = "Noise Filter Clock Select"]
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
        #[doc = "No output-disable request from the GTETRGn pin occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from the GTETRGn pin occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iocf_SPEC;
    pub type Iocf = crate::EnumBitfieldStruct<u8, Iocf_SPEC>;
    impl Iocf {
        #[doc = "No output-disable request from GPT occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from GPT occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpf_SPEC;
    pub type Ostpf = crate::EnumBitfieldStruct<u8, Ostpf_SPEC>;
    impl Ostpf {
        #[doc = "No output-disable request from oscillation stop detection occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from oscillation stop detection occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssf_SPEC;
    pub type Ssf = crate::EnumBitfieldStruct<u8, Ssf_SPEC>;
    impl Ssf {
        #[doc = "No output-disable request from software occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from software occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pide_SPEC;
    pub type Pide = crate::EnumBitfieldStruct<u8, Pide_SPEC>;
    impl Pide {
        #[doc = "Disable output-disable requests from the GTETRGn pins"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable output-disable requests from the GTETRGn pins"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ioce_SPEC;
    pub type Ioce = crate::EnumBitfieldStruct<u8, Ioce_SPEC>;
    impl Ioce {
        #[doc = "Disable output-disable requests from GPT"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable output-disable requests from GPT"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpe_SPEC;
    pub type Ostpe = crate::EnumBitfieldStruct<u8, Ostpe_SPEC>;
    impl Ostpe {
        #[doc = "Disable output-disable requests from oscillation stop detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable output-disable requests from oscillation stop detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "GTETRGn input after filtering was 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTETRGn input after filtering was 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Input GTETRGn as-is"]
        pub const _0: Self = Self::new(0);

        #[doc = "Input GTETRGn in reverse"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Disable noise filtering"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable noise filtering"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "Sample GTETRGn pin input level three times every PCLKB"]
        pub const _00: Self = Self::new(0);

        #[doc = "Sample GTETRGn pin input level three times every PCLKB/8"]
        pub const _01: Self = Self::new(1);

        #[doc = "Sample GTETRGn pin input level three times every PCLKB/32"]
        pub const _10: Self = Self::new(2);

        #[doc = "Sample GTETRGn pin input level three times every PCLKB/128"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Detection Flag for GPT Output-Disable Request"]
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

    #[doc = "Enable for GPT Output-Disable Request"]
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

    #[doc = "Oscillation Stop Detection Enable"]
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

    #[doc = "GTETRGn Input Reverse"]
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

    #[doc = "Noise Filter Enable"]
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

    #[doc = "Noise Filter Clock Select"]
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
        #[doc = "No output-disable request from the GTETRGn pin occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from the GTETRGn pin occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iocf_SPEC;
    pub type Iocf = crate::EnumBitfieldStruct<u8, Iocf_SPEC>;
    impl Iocf {
        #[doc = "No output-disable request from GPT occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from GPT occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpf_SPEC;
    pub type Ostpf = crate::EnumBitfieldStruct<u8, Ostpf_SPEC>;
    impl Ostpf {
        #[doc = "No output-disable request from oscillation stop detection occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from oscillation stop detection occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssf_SPEC;
    pub type Ssf = crate::EnumBitfieldStruct<u8, Ssf_SPEC>;
    impl Ssf {
        #[doc = "No output-disable request from software occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from software occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pide_SPEC;
    pub type Pide = crate::EnumBitfieldStruct<u8, Pide_SPEC>;
    impl Pide {
        #[doc = "Disable output-disable requests from the GTETRGn pins"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable output-disable requests from the GTETRGn pins"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ioce_SPEC;
    pub type Ioce = crate::EnumBitfieldStruct<u8, Ioce_SPEC>;
    impl Ioce {
        #[doc = "Disable output-disable requests from GPT"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable output-disable requests from GPT"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpe_SPEC;
    pub type Ostpe = crate::EnumBitfieldStruct<u8, Ostpe_SPEC>;
    impl Ostpe {
        #[doc = "Disable output-disable requests from oscillation stop detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable output-disable requests from oscillation stop detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "GTETRGn input after filtering was 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTETRGn input after filtering was 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Input GTETRGn as-is"]
        pub const _0: Self = Self::new(0);

        #[doc = "Input GTETRGn in reverse"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Disable noise filtering"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable noise filtering"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "Sample GTETRGn pin input level three times every PCLKB"]
        pub const _00: Self = Self::new(0);

        #[doc = "Sample GTETRGn pin input level three times every PCLKB/8"]
        pub const _01: Self = Self::new(1);

        #[doc = "Sample GTETRGn pin input level three times every PCLKB/32"]
        pub const _10: Self = Self::new(2);

        #[doc = "Sample GTETRGn pin input level three times every PCLKB/128"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Detection Flag for GPT Output-Disable Request"]
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

    #[doc = "Enable for GPT Output-Disable Request"]
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

    #[doc = "Oscillation Stop Detection Enable"]
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

    #[doc = "GTETRGn Input Reverse"]
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

    #[doc = "Noise Filter Enable"]
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

    #[doc = "Noise Filter Clock Select"]
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
        #[doc = "No output-disable request from the GTETRGn pin occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from the GTETRGn pin occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iocf_SPEC;
    pub type Iocf = crate::EnumBitfieldStruct<u8, Iocf_SPEC>;
    impl Iocf {
        #[doc = "No output-disable request from GPT occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from GPT occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpf_SPEC;
    pub type Ostpf = crate::EnumBitfieldStruct<u8, Ostpf_SPEC>;
    impl Ostpf {
        #[doc = "No output-disable request from oscillation stop detection occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from oscillation stop detection occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssf_SPEC;
    pub type Ssf = crate::EnumBitfieldStruct<u8, Ssf_SPEC>;
    impl Ssf {
        #[doc = "No output-disable request from software occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from software occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pide_SPEC;
    pub type Pide = crate::EnumBitfieldStruct<u8, Pide_SPEC>;
    impl Pide {
        #[doc = "Disable output-disable requests from the GTETRGn pins"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable output-disable requests from the GTETRGn pins"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ioce_SPEC;
    pub type Ioce = crate::EnumBitfieldStruct<u8, Ioce_SPEC>;
    impl Ioce {
        #[doc = "Disable output-disable requests from GPT"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable output-disable requests from GPT"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpe_SPEC;
    pub type Ostpe = crate::EnumBitfieldStruct<u8, Ostpe_SPEC>;
    impl Ostpe {
        #[doc = "Disable output-disable requests from oscillation stop detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable output-disable requests from oscillation stop detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "GTETRGn input after filtering was 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTETRGn input after filtering was 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Input GTETRGn as-is"]
        pub const _0: Self = Self::new(0);

        #[doc = "Input GTETRGn in reverse"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Disable noise filtering"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable noise filtering"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "Sample GTETRGn pin input level three times every PCLKB"]
        pub const _00: Self = Self::new(0);

        #[doc = "Sample GTETRGn pin input level three times every PCLKB/8"]
        pub const _01: Self = Self::new(1);

        #[doc = "Sample GTETRGn pin input level three times every PCLKB/32"]
        pub const _10: Self = Self::new(2);

        #[doc = "Sample GTETRGn pin input level three times every PCLKB/128"]
        pub const _11: Self = Self::new(3);
    }
}
