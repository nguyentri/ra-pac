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
// Generated from SVD 1.0, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:50:19 +0000

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

    #[doc = "POEG Group %s Setting Register"]
    #[inline(always)]
    pub const fn poegg(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Poegg_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub const fn poegga(&self) -> &'static crate::common::Reg<self::Poegg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Poegg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn poeggb(&self) -> &'static crate::common::Reg<self::Poegg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Poegg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Poegg_SPEC;
impl crate::sealed::RegSpec for Poegg_SPEC {
    type DataType = u32;
}

#[doc = "POEG Group %s Setting Register"]
pub type Poegg = crate::RegValueT<Poegg_SPEC>;

impl Poegg {
    #[doc = "Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        poegg::Nfcs,
        poegg::Nfcs,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            poegg::Nfcs,
            poegg::Nfcs,
            Poegg_SPEC,
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
        poegg::Nfen,
        poegg::Nfen,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            poegg::Nfen,
            poegg::Nfen,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTETRG Input Reverse"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        poegg::Inv,
        poegg::Inv,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            poegg::Inv,
            poegg::Inv,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000000. The write value should be 00000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7ff, 1, 0, u16, u16, Poegg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x7ff,1,0,u16,u16,Poegg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "GTETRG Input Status Flag"]
    #[inline(always)]
    pub fn st(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        poegg::St,
        poegg::St,
        Poegg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            poegg::St,
            poegg::St,
            Poegg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Oscillation Stop Detection EnableNote: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn ostpe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        poegg::Ostpe,
        poegg::Ostpe,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            poegg::Ostpe,
            poegg::Ostpe,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Output-disable Request Enable from GPTNote: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn ioce(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        poegg::Ioce,
        poegg::Ioce,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            poegg::Ioce,
            poegg::Ioce,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Detection EnableNote: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn pide(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        poegg::Pide,
        poegg::Pide,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            poegg::Pide,
            poegg::Pide,
            Poegg_SPEC,
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
        poegg::Ssf,
        poegg::Ssf,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            poegg::Ssf,
            poegg::Ssf,
            Poegg_SPEC,
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
        poegg::Ostpf,
        poegg::Ostpf,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            poegg::Ostpf,
            poegg::Ostpf,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Output-disable Request Detection Flag from GPT"]
    #[inline(always)]
    pub fn iocf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        poegg::Iocf,
        poegg::Iocf,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            poegg::Iocf,
            poegg::Iocf,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Detection Flag"]
    #[inline(always)]
    pub fn pidf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        poegg::Pidf,
        poegg::Pidf,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            poegg::Pidf,
            poegg::Pidf,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Poegg {
    #[inline(always)]
    fn default() -> Poegg {
        <crate::RegValueT<Poegg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod poegg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "Sampling GTETRG pin input level for three times in every PCLKB."]
        pub const _00: Self = Self::new(0);

        #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /8."]
        pub const _01: Self = Self::new(1);

        #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /32."]
        pub const _10: Self = Self::new(2);

        #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /128."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Filtering noise disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Filtering noise enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "GTETRG Input"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTETRG Input Reversed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "GTETRG input after filtering is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "GTETRG input after filtering is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpe_SPEC;
    pub type Ostpe = crate::EnumBitfieldStruct<u8, Ostpe_SPEC>;
    impl Ostpe {
        #[doc = "A output-disable request from the oscillation stop detection disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "A output-disable request from the oscillation stop detection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ioce_SPEC;
    pub type Ioce = crate::EnumBitfieldStruct<u8, Ioce_SPEC>;
    impl Ioce {
        #[doc = "Output-disable request from the GPT disable request disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from the GPT disable request enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pide_SPEC;
    pub type Pide = crate::EnumBitfieldStruct<u8, Pide_SPEC>;
    impl Pide {
        #[doc = "Output-disable request from the GTETRG pins disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from the GTETRG pins enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssf_SPEC;
    pub type Ssf = crate::EnumBitfieldStruct<u8, Ssf_SPEC>;
    impl Ssf {
        #[doc = "No output-disable request from software has occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from software occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpf_SPEC;
    pub type Ostpf = crate::EnumBitfieldStruct<u8, Ostpf_SPEC>;
    impl Ostpf {
        #[doc = "No output-disable request from oscillation stop detection has occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from oscillation stop detection occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iocf_SPEC;
    pub type Iocf = crate::EnumBitfieldStruct<u8, Iocf_SPEC>;
    impl Iocf {
        #[doc = "No output-disable request from the GPT disable request has occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from the GPT disable request occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidf_SPEC;
    pub type Pidf = crate::EnumBitfieldStruct<u8, Pidf_SPEC>;
    impl Pidf {
        #[doc = "No output-disable request from the GTETRGn pin has occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output-disable request from the GTETRGn pin occurred."]
        pub const _1: Self = Self::new(1);
    }
}
