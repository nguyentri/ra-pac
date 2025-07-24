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
// Generated from SVD 1.0, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:52:24 +0000

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

    #[doc = "KEY Return Control Register"]
    #[inline(always)]
    pub const fn krctl(&self) -> &'static crate::common::Reg<self::Krctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Krctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "KEY Return Flag Register"]
    #[inline(always)]
    pub const fn krf(&self) -> &'static crate::common::Reg<self::Krf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Krf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "KEY Return Mode Register"]
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

#[doc = "KEY Return Control Register"]
pub type Krctl = crate::RegValueT<Krctl_SPEC>;

impl Krctl {
    #[doc = "Usage of Key Interrupt Flags(KR0 to KR7)"]
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

    #[doc = "Detection Edge Selection (KRF0 to KRF7)"]
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
}
impl ::core::default::Default for Krctl {
    #[inline(always)]
    fn default() -> Krctl {
        <crate::RegValueT<Krctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod krctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krmd_SPEC;
    pub type Krmd = crate::EnumBitfieldStruct<u8, Krmd_SPEC>;
    impl Krmd {
        #[doc = "Do not use key interrupt flags"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use key interrupt flags."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Kreg_SPEC;
    pub type Kreg = crate::EnumBitfieldStruct<u8, Kreg_SPEC>;
    impl Kreg {
        #[doc = "Falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edge"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krf_SPEC;
impl crate::sealed::RegSpec for Krf_SPEC {
    type DataType = u8;
}

#[doc = "KEY Return Flag Register"]
pub type Krf = crate::RegValueT<Krf_SPEC>;

impl Krf {
    #[doc = "Key interrupt flag 7"]
    #[inline(always)]
    pub fn krf7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, krf::Krf7, krf::Krf7, Krf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            krf::Krf7,
            krf::Krf7,
            Krf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key interrupt flag 6"]
    #[inline(always)]
    pub fn krf6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, krf::Krf6, krf::Krf6, Krf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            krf::Krf6,
            krf::Krf6,
            Krf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key interrupt flag 5"]
    #[inline(always)]
    pub fn krf5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, krf::Krf5, krf::Krf5, Krf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            krf::Krf5,
            krf::Krf5,
            Krf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key interrupt flag 4"]
    #[inline(always)]
    pub fn krf4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, krf::Krf4, krf::Krf4, Krf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            krf::Krf4,
            krf::Krf4,
            Krf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key interrupt flag 3"]
    #[inline(always)]
    pub fn krf3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, krf::Krf3, krf::Krf3, Krf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            krf::Krf3,
            krf::Krf3,
            Krf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key interrupt flag 2"]
    #[inline(always)]
    pub fn krf2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, krf::Krf2, krf::Krf2, Krf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            krf::Krf2,
            krf::Krf2,
            Krf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key interrupt flag 1"]
    #[inline(always)]
    pub fn krf1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, krf::Krf1, krf::Krf1, Krf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            krf::Krf1,
            krf::Krf1,
            Krf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key interrupt flag 0"]
    #[inline(always)]
    pub fn krf0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krf::Krf0, krf::Krf0, Krf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            krf::Krf0,
            krf::Krf0,
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
    pub struct Krf7_SPEC;
    pub type Krf7 = crate::EnumBitfieldStruct<u8, Krf7_SPEC>;
    impl Krf7 {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krf6_SPEC;
    pub type Krf6 = crate::EnumBitfieldStruct<u8, Krf6_SPEC>;
    impl Krf6 {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krf5_SPEC;
    pub type Krf5 = crate::EnumBitfieldStruct<u8, Krf5_SPEC>;
    impl Krf5 {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krf4_SPEC;
    pub type Krf4 = crate::EnumBitfieldStruct<u8, Krf4_SPEC>;
    impl Krf4 {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krf3_SPEC;
    pub type Krf3 = crate::EnumBitfieldStruct<u8, Krf3_SPEC>;
    impl Krf3 {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krf2_SPEC;
    pub type Krf2 = crate::EnumBitfieldStruct<u8, Krf2_SPEC>;
    impl Krf2 {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krf1_SPEC;
    pub type Krf1 = crate::EnumBitfieldStruct<u8, Krf1_SPEC>;
    impl Krf1 {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krf0_SPEC;
    pub type Krf0 = crate::EnumBitfieldStruct<u8, Krf0_SPEC>;
    impl Krf0 {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krm_SPEC;
impl crate::sealed::RegSpec for Krm_SPEC {
    type DataType = u8;
}

#[doc = "KEY Return Mode Register"]
pub type Krm = crate::RegValueT<Krm_SPEC>;

impl Krm {
    #[doc = "Key interrupt mode control 7"]
    #[inline(always)]
    pub fn krm7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, krm::Krm7, krm::Krm7, Krm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            krm::Krm7,
            krm::Krm7,
            Krm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key interrupt mode control 6"]
    #[inline(always)]
    pub fn krm6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, krm::Krm6, krm::Krm6, Krm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            krm::Krm6,
            krm::Krm6,
            Krm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key interrupt mode control 5"]
    #[inline(always)]
    pub fn krm5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, krm::Krm5, krm::Krm5, Krm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            krm::Krm5,
            krm::Krm5,
            Krm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key interrupt mode control 4"]
    #[inline(always)]
    pub fn krm4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, krm::Krm4, krm::Krm4, Krm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            krm::Krm4,
            krm::Krm4,
            Krm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key interrupt mode control 3"]
    #[inline(always)]
    pub fn krm3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, krm::Krm3, krm::Krm3, Krm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            krm::Krm3,
            krm::Krm3,
            Krm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key interrupt mode control 2"]
    #[inline(always)]
    pub fn krm2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, krm::Krm2, krm::Krm2, Krm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            krm::Krm2,
            krm::Krm2,
            Krm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key interrupt mode control 1"]
    #[inline(always)]
    pub fn krm1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, krm::Krm1, krm::Krm1, Krm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            krm::Krm1,
            krm::Krm1,
            Krm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key interrupt mode control 0"]
    #[inline(always)]
    pub fn krm0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krm::Krm0, krm::Krm0, Krm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            krm::Krm0,
            krm::Krm0,
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
    pub struct Krm7_SPEC;
    pub type Krm7 = crate::EnumBitfieldStruct<u8, Krm7_SPEC>;
    impl Krm7 {
        #[doc = "Does not detect key interrupt signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect key interrupt signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krm6_SPEC;
    pub type Krm6 = crate::EnumBitfieldStruct<u8, Krm6_SPEC>;
    impl Krm6 {
        #[doc = "Does not detect key interrupt signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect key interrupt signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krm5_SPEC;
    pub type Krm5 = crate::EnumBitfieldStruct<u8, Krm5_SPEC>;
    impl Krm5 {
        #[doc = "Does not detect key interrupt signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect key interrupt signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krm4_SPEC;
    pub type Krm4 = crate::EnumBitfieldStruct<u8, Krm4_SPEC>;
    impl Krm4 {
        #[doc = "Does not detect key interrupt signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect key interrupt signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krm3_SPEC;
    pub type Krm3 = crate::EnumBitfieldStruct<u8, Krm3_SPEC>;
    impl Krm3 {
        #[doc = "Does not detect key interrupt signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect key interrupt signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krm2_SPEC;
    pub type Krm2 = crate::EnumBitfieldStruct<u8, Krm2_SPEC>;
    impl Krm2 {
        #[doc = "Does not detect key interrupt signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect key interrupt signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krm1_SPEC;
    pub type Krm1 = crate::EnumBitfieldStruct<u8, Krm1_SPEC>;
    impl Krm1 {
        #[doc = "Does not detect key interrupt signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect key interrupt signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Krm0_SPEC;
    pub type Krm0 = crate::EnumBitfieldStruct<u8, Krm0_SPEC>;
    impl Krm0 {
        #[doc = "Does not detect key interrupt signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect key interrupt signal."]
        pub const _1: Self = Self::new(1);
    }
}
