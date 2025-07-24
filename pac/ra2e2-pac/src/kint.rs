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
// Generated from SVD 1.40.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:46:37 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Key Interrupt Function"]
unsafe impl ::core::marker::Send for super::Kint {}
unsafe impl ::core::marker::Sync for super::Kint {}
impl super::Kint {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Key Return Control Register"]
    #[inline(always)]
    pub const fn krctl(&self) -> &'static crate::common::Reg<self::Krctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Krctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Key Return Flag Register"]
    #[inline(always)]
    pub const fn krf(&self) -> &'static crate::common::Reg<self::Krf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Krf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Key Return Mode Register"]
    #[inline(always)]
    pub const fn krm(&self) -> &'static crate::common::Reg<self::Krm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Krm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krctl_SPEC;
impl crate::sealed::RegSpec for Krctl_SPEC {
    type DataType = u8;
}

#[doc = "Key Return Control Register"]
pub type Krctl = crate::RegValueT<Krctl_SPEC>;

impl Krctl {
    #[doc = "Detection Edge Selection (KR00 to KR03 pins)"]
    #[inline(always)]
    pub fn kreg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        krctl::Kreg,
        krctl::Kreg,
        Krctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            krctl::Kreg,
            krctl::Kreg,
            Krctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Usage of Key Interrupt Flags (KRF.KIF0 to KRF.KIF3)"]
    #[inline(always)]
    pub fn krmd(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        krctl::Krmd,
        krctl::Krmd,
        Krctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            krctl::Krmd,
            krctl::Krmd,
            Krctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Krctl {
    #[inline(always)]
    fn default() -> Krctl {
        <crate::RegValueT<Krctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod krctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Kreg_SPEC;
    pub type Kreg = crate::EnumBitfieldStruct<u8, Kreg_SPEC>;
    impl Kreg {
        #[doc = "Falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krmd_SPEC;
    pub type Krmd = crate::EnumBitfieldStruct<u8, Krmd_SPEC>;
    impl Krmd {
        #[doc = "Do not use key interrupt flags"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use key interrupt flags"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krf_SPEC;
impl crate::sealed::RegSpec for Krf_SPEC {
    type DataType = u8;
}

#[doc = "Key Return Flag Register"]
pub type Krf = crate::RegValueT<Krf_SPEC>;

impl Krf {
    #[doc = "Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krf::Kif0, krf::Kif0, Krf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            krf::Kif0,
            krf::Kif0,
            Krf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, krf::Kif1, krf::Kif1, Krf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            krf::Kif1,
            krf::Kif1,
            Krf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, krf::Kif2, krf::Kif2, Krf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            krf::Kif2,
            krf::Kif2,
            Krf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, krf::Kif3, krf::Kif3, Krf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            krf::Kif3,
            krf::Kif3,
            Krf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Krf {
    #[inline(always)]
    fn default() -> Krf {
        <crate::RegValueT<Krf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod krf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Kif0_SPEC;
    pub type Kif0 = crate::EnumBitfieldStruct<u8, Kif0_SPEC>;
    impl Kif0 {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Kif1_SPEC;
    pub type Kif1 = crate::EnumBitfieldStruct<u8, Kif1_SPEC>;
    impl Kif1 {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Kif2_SPEC;
    pub type Kif2 = crate::EnumBitfieldStruct<u8, Kif2_SPEC>;
    impl Kif2 {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Kif3_SPEC;
    pub type Kif3 = crate::EnumBitfieldStruct<u8, Kif3_SPEC>;
    impl Kif3 {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krm_SPEC;
impl crate::sealed::RegSpec for Krm_SPEC {
    type DataType = u8;
}

#[doc = "Key Return Mode Register"]
pub type Krm = crate::RegValueT<Krm_SPEC>;

impl Krm {
    #[doc = "Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        krm::Kimc0,
        krm::Kimc0,
        Krm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            krm::Kimc0,
            krm::Kimc0,
            Krm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        krm::Kimc1,
        krm::Kimc1,
        Krm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            krm::Kimc1,
            krm::Kimc1,
            Krm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        krm::Kimc2,
        krm::Kimc2,
        Krm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            krm::Kimc2,
            krm::Kimc2,
            Krm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        krm::Kimc3,
        krm::Kimc3,
        Krm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            krm::Kimc3,
            krm::Kimc3,
            Krm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Krm {
    #[inline(always)]
    fn default() -> Krm {
        <crate::RegValueT<Krm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod krm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Kimc0_SPEC;
    pub type Kimc0 = crate::EnumBitfieldStruct<u8, Kimc0_SPEC>;
    impl Kimc0 {
        #[doc = "Do not detect key interrupt signals"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect key interrupt signals"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Kimc1_SPEC;
    pub type Kimc1 = crate::EnumBitfieldStruct<u8, Kimc1_SPEC>;
    impl Kimc1 {
        #[doc = "Do not detect key interrupt signals"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect key interrupt signals"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Kimc2_SPEC;
    pub type Kimc2 = crate::EnumBitfieldStruct<u8, Kimc2_SPEC>;
    impl Kimc2 {
        #[doc = "Do not detect key interrupt signals"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect key interrupt signals"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Kimc3_SPEC;
    pub type Kimc3 = crate::EnumBitfieldStruct<u8, Kimc3_SPEC>;
    impl Kimc3 {
        #[doc = "Do not detect key interrupt signals"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect key interrupt signals"]
        pub const _1: Self = Self::new(1);
    }
}
