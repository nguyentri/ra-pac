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
#[doc = r"Output Phase Switching Controller"]
unsafe impl ::core::marker::Send for super::GptOps {}
unsafe impl ::core::marker::Sync for super::GptOps {}
impl super::GptOps {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Output Phase Switching Control Register"]
    #[inline(always)]
    pub const fn opscr(&self) -> &'static crate::common::Reg<self::Opscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Opscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opscr_SPEC;
impl crate::sealed::RegSpec for Opscr_SPEC {
    type DataType = u32;
}

#[doc = "Output Phase Switching Control Register"]
pub type Opscr = crate::RegValueT<Opscr_SPEC>;

impl Opscr {
    #[doc = "External Input Noise Filter Clock selection\nNoise filter sampling clock setting of the external input."]
    #[inline(always)]
    pub fn nfcs(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        opscr::Nfcs,
        opscr::Nfcs,
        Opscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            opscr::Nfcs,
            opscr::Nfcs,
            Opscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "External Input Noise Filter Enable"]
    #[inline(always)]
    pub fn nfen(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        opscr::Nfen,
        opscr::Nfen,
        Opscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            opscr::Nfen,
            opscr::Nfen,
            Opscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Group output disable function"]
    #[inline(always)]
    pub fn godf(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        opscr::Godf,
        opscr::Godf,
        Opscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            opscr::Godf,
            opscr::Godf,
            Opscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Output disabled source selection"]
    #[inline(always)]
    pub fn grp(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        opscr::Grp,
        opscr::Grp,
        Opscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            opscr::Grp,
            opscr::Grp,
            Opscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Input phase alignment"]
    #[inline(always)]
    pub fn align(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        opscr::Align,
        opscr::Align,
        Opscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            opscr::Align,
            opscr::Align,
            Opscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Output phase rotation direction reversal"]
    #[inline(always)]
    pub fn rv(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        opscr::Rv,
        opscr::Rv,
        Opscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            opscr::Rv,
            opscr::Rv,
            Opscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Invert-Phase Output Control"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        opscr::Inv,
        opscr::Inv,
        Opscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            opscr::Inv,
            opscr::Inv,
            Opscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Negative-Phase Output (N) Control"]
    #[inline(always)]
    pub fn n(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        opscr::N,
        opscr::N,
        Opscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            opscr::N,
            opscr::N,
            Opscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Positive-Phase Output (P) Control"]
    #[inline(always)]
    pub fn p(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        opscr::P,
        opscr::P,
        Opscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            opscr::P,
            opscr::P,
            Opscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "External Feedback Signal Enable\nThis bit selects the input phase from the software settings and external input."]
    #[inline(always)]
    pub fn fb(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        opscr::Fb,
        opscr::Fb,
        Opscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            opscr::Fb,
            opscr::Fb,
            Opscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable-Phase Output Control"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        opscr::En,
        opscr::En,
        Opscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            opscr::En,
            opscr::En,
            Opscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Input W-Phase Monitor\nThis bit monitors the state of the input phase.\nOPSCR.FB=0:External input monitoring by PCLK\nOPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[inline(always)]
    pub fn w(self) -> crate::common::RegisterFieldBool<6, 1, 0, Opscr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Opscr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Input V-Phase Monitor\nThis bit monitors the state of the input phase.\nOPSCR.FB=0:External input monitoring by PCLK\nOPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[inline(always)]
    pub fn v(self) -> crate::common::RegisterFieldBool<5, 1, 0, Opscr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Opscr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Input U-Phase Monitor\nThis bit monitors the state of the input phase.\nOPSCR.FB=0:External input monitoring by PCLK\nOPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[inline(always)]
    pub fn u(self) -> crate::common::RegisterFieldBool<4, 1, 0, Opscr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Opscr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Opscr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Opscr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Input Phase Soft Setting UF\nThis bit sets the input phase by the software settings.\nThis bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub fn wf(self) -> crate::common::RegisterFieldBool<2, 1, 0, Opscr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Opscr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Input Phase Soft Setting VF\nThis bit sets the input phase by the software settings.\nThis bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub fn vf(self) -> crate::common::RegisterFieldBool<1, 1, 0, Opscr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Opscr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Input Phase Soft Setting WF\nThis bit sets the input phase by the software settings.\nThis bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub fn uf(self) -> crate::common::RegisterFieldBool<0, 1, 0, Opscr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Opscr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Opscr {
    #[inline(always)]
    fn default() -> Opscr {
        <crate::RegValueT<Opscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod opscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "PCLK/1"]
        pub const _00: Self = Self::new(0);

        #[doc = "PCLK/4"]
        pub const _01: Self = Self::new(1);

        #[doc = "PCLK/16"]
        pub const _10: Self = Self::new(2);

        #[doc = "PCLK/64"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Do not use a noise filter to the external input."]
        pub const _0: Self = Self::new(0);

        #[doc = "Use a noise filter to the external input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Godf_SPEC;
    pub type Godf = crate::EnumBitfieldStruct<u8, Godf_SPEC>;
    impl Godf {
        #[doc = "This bit function is ignored."]
        pub const _0: Self = Self::new(0);

        #[doc = "Group disable will clear OPSCR.EN Bit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grp_SPEC;
    pub type Grp = crate::EnumBitfieldStruct<u8, Grp_SPEC>;
    impl Grp {
        #[doc = "Select Group A output disable source"]
        pub const _00: Self = Self::new(0);

        #[doc = "Select Group B output disable source"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Align_SPEC;
    pub type Align = crate::EnumBitfieldStruct<u8, Align_SPEC>;
    impl Align {
        #[doc = "Input phase is aligned to PCLK."]
        pub const _0: Self = Self::new(0);

        #[doc = "Input phase is aligned PWM."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rv_SPEC;
    pub type Rv = crate::EnumBitfieldStruct<u8, Rv_SPEC>;
    impl Rv {
        #[doc = "U/V/W-Phase output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output to reverse the V / W-phase"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Positive Logic (Active High)output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Negative Logic (Active Low)output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct N_SPEC;
    pub type N = crate::EnumBitfieldStruct<u8, N_SPEC>;
    impl N {
        #[doc = "Level signal output"]
        pub const _0: Self = Self::new(0);

        #[doc = "PWM signal output (PWM of GPT0)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P_SPEC;
    pub type P = crate::EnumBitfieldStruct<u8, P_SPEC>;
    impl P {
        #[doc = "Level signal output"]
        pub const _0: Self = Self::new(0);

        #[doc = "PWM signal output (PWM of GPT0)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fb_SPEC;
    pub type Fb = crate::EnumBitfieldStruct<u8, Fb_SPEC>;
    impl Fb {
        #[doc = "Select the external input."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select the soft setting(OPSCR.UF, VF, WF)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Not Output(Hi-Z external terminals)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Output"]
        pub const _1: Self = Self::new(1);
    }
}
