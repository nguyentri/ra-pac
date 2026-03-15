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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:57:20 +0000

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
        4,
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
    #[inline(always)]
    pub const fn poeggc(&self) -> &'static crate::common::Reg<self::Poegg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Poegg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn poeggd(&self) -> &'static crate::common::Reg<self::Poegg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Poegg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x300usize),
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

    #[doc = "Real Time Overcurrent Detection Flag"]
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

    #[doc = "Enable for GPT Output-Disable RequestNote: Can be modified only once after a reset."]
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

    #[doc = "Comparator Disable Request Enable 0Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn cdre0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        poegg::Cdre0,
        poegg::Cdre0,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            poegg::Cdre0,
            poegg::Cdre0,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparator Disable Request Enable 1Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn cdre1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        poegg::Cdre1,
        poegg::Cdre1,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            poegg::Cdre1,
            poegg::Cdre1,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparator Disable Request Enable 2Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn cdre2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        poegg::Cdre2,
        poegg::Cdre2,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            poegg::Cdre2,
            poegg::Cdre2,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparator Disable Request Enable 3Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn cdre3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        poegg::Cdre3,
        poegg::Cdre3,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            poegg::Cdre3,
            poegg::Cdre3,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparator Disable Request Enable 4Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn cdre4(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        poegg::Cdre4,
        poegg::Cdre4,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            poegg::Cdre4,
            poegg::Cdre4,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparator Disable Request Enable 5Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn cdre5(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        poegg::Cdre5,
        poegg::Cdre5,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            poegg::Cdre5,
            poegg::Cdre5,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

    #[doc = "POEG group A Cooperation Enable"]
    #[inline(always)]
    pub fn coopa(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        poegg::Coopa,
        poegg::Coopa,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            poegg::Coopa,
            poegg::Coopa,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "POEG group B Cooperation Enable"]
    #[inline(always)]
    pub fn coopb(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        poegg::Coopb,
        poegg::Coopb,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            poegg::Coopb,
            poegg::Coopb,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "POEG group C Cooperation Enable"]
    #[inline(always)]
    pub fn coopc(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        poegg::Coopc,
        poegg::Coopc,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            poegg::Coopc,
            poegg::Coopc,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "POEG group D Cooperation Enable"]
    #[inline(always)]
    pub fn coopd(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        poegg::Coopd,
        poegg::Coopd,
        Poegg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            poegg::Coopd,
            poegg::Coopd,
            Poegg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Poegg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Poegg_SPEC,crate::common::RW>::from_register(self,0)
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
}
impl ::core::default::Default for Poegg {
    #[inline(always)]
    fn default() -> Poegg {
        <crate::RegValueT<Poegg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod poegg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidf_SPEC;
    pub type Pidf = crate::EnumBitfieldStruct<u8, Pidf_SPEC>;
    impl Pidf {
        #[doc = "A output-disable request from the GTETRG pin has not been generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A output-disable request from the GTETRG pin has been generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iocf_SPEC;
    pub type Iocf = crate::EnumBitfieldStruct<u8, Iocf_SPEC>;
    impl Iocf {
        #[doc = "A output-disable request from GPT disable request or comparator interrupt has not been generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A output-disable request from GPT disable request or comparator interrupt has been generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostpf_SPEC;
    pub type Ostpf = crate::EnumBitfieldStruct<u8, Ostpf_SPEC>;
    impl Ostpf {
        #[doc = "A output-disable request from the oscillation stop detection has not been generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A output-disable request from the oscillation stop detection has been generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssf_SPEC;
    pub type Ssf = crate::EnumBitfieldStruct<u8, Ssf_SPEC>;
    impl Ssf {
        #[doc = "A output-disable request from software has not been generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A output-disable request from software has been generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pide_SPEC;
    pub type Pide = crate::EnumBitfieldStruct<u8, Pide_SPEC>;
    impl Pide {
        #[doc = "A output-disable request from the GTETRG pins disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "A output-disable request from the GTETRG pins enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ioce_SPEC;
    pub type Ioce = crate::EnumBitfieldStruct<u8, Ioce_SPEC>;
    impl Ioce {
        #[doc = "Disable output-disable requests from GPT disable request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable output-disable requests from GPT disable request"]
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
    pub struct Cdre0_SPEC;
    pub type Cdre0 = crate::EnumBitfieldStruct<u8, Cdre0_SPEC>;
    impl Cdre0 {
        #[doc = "A disable request of comparator 0 disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "A disable request of comparator 0 enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre1_SPEC;
    pub type Cdre1 = crate::EnumBitfieldStruct<u8, Cdre1_SPEC>;
    impl Cdre1 {
        #[doc = "A disable request of comparator 1 disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "A disable request of comparator 1 enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre2_SPEC;
    pub type Cdre2 = crate::EnumBitfieldStruct<u8, Cdre2_SPEC>;
    impl Cdre2 {
        #[doc = "A disable request of comparator 2 disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "A disable request of comparator 2 enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre3_SPEC;
    pub type Cdre3 = crate::EnumBitfieldStruct<u8, Cdre3_SPEC>;
    impl Cdre3 {
        #[doc = "A disable request of comparator 3 disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "A disable request of comparator 3 enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre4_SPEC;
    pub type Cdre4 = crate::EnumBitfieldStruct<u8, Cdre4_SPEC>;
    impl Cdre4 {
        #[doc = "A disable request of comparator 4 disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "A disable request of comparator 4 enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdre5_SPEC;
    pub type Cdre5 = crate::EnumBitfieldStruct<u8, Cdre5_SPEC>;
    impl Cdre5 {
        #[doc = "A disable request of comparator 5 disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "A disable request of comparator 5 enabled."]
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
    pub struct Coopa_SPEC;
    pub type Coopa = crate::EnumBitfieldStruct<u8, Coopa_SPEC>;
    impl Coopa {
        #[doc = "Disable cooperation with POEG group A"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable cooperation with POEG group A."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Coopb_SPEC;
    pub type Coopb = crate::EnumBitfieldStruct<u8, Coopb_SPEC>;
    impl Coopb {
        #[doc = "Disable cooperation with POEG group B"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable cooperation with POEG group B."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Coopc_SPEC;
    pub type Coopc = crate::EnumBitfieldStruct<u8, Coopc_SPEC>;
    impl Coopc {
        #[doc = "Disable cooperation with POEG group C"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable cooperation with POEG group C."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Coopd_SPEC;
    pub type Coopd = crate::EnumBitfieldStruct<u8, Coopd_SPEC>;
    impl Coopd {
        #[doc = "Disable cooperation with POEG group D"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable cooperation with POEG group D."]
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
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Filtering noise disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Filtering noise enabled"]
        pub const _1: Self = Self::new(1);
    }
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
}
