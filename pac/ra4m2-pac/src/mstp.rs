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
// Generated from SVD 1.30.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:18:36 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Module Stop Control"]
unsafe impl ::core::marker::Send for super::Mstp {}
unsafe impl ::core::marker::Sync for super::Mstp {}
impl super::Mstp {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn mstpcra(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mstpcrb(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mstpcrc(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcrc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcrc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mstpcrd(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcrd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcrd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mstpcre(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcre_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcre_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcra_SPEC;
impl crate::sealed::RegSpec for Mstpcra_SPEC {
    type DataType = u32;
}

pub type Mstpcra = crate::RegValueT<Mstpcra_SPEC>;

impl Mstpcra {
    #[inline(always)]
    pub fn mstpa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mstpcra::Mstpa0,
        mstpcra::Mstpa0,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mstpcra::Mstpa0,
            mstpcra::Mstpa0,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpa7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mstpcra::Mstpa7,
        mstpcra::Mstpa7,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mstpcra::Mstpa7,
            mstpcra::Mstpa7,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpa22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mstpcra::Mstpa22,
        mstpcra::Mstpa22,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mstpcra::Mstpa22,
            mstpcra::Mstpa22,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mstpcra {
    #[inline(always)]
    fn default() -> Mstpcra {
        <crate::RegValueT<Mstpcra_SPEC> as RegisterValue<_>>::new(4290772862)
    }
}
pub mod mstpcra {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa0_SPEC;
    pub type Mstpa0 = crate::EnumBitfieldStruct<u8, Mstpa0_SPEC>;
    impl Mstpa0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa7_SPEC;
    pub type Mstpa7 = crate::EnumBitfieldStruct<u8, Mstpa7_SPEC>;
    impl Mstpa7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa22_SPEC;
    pub type Mstpa22 = crate::EnumBitfieldStruct<u8, Mstpa22_SPEC>;
    impl Mstpa22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrb_SPEC;
impl crate::sealed::RegSpec for Mstpcrb_SPEC {
    type DataType = u32;
}

pub type Mstpcrb = crate::RegValueT<Mstpcrb_SPEC>;

impl Mstpcrb {
    #[inline(always)]
    pub fn mstpb2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mstpcrb::Mstpb2,
        mstpcrb::Mstpb2,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mstpcrb::Mstpb2,
            mstpcrb::Mstpb2,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpb6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mstpcrb::Mstpb6,
        mstpcrb::Mstpb6,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mstpcrb::Mstpb6,
            mstpcrb::Mstpb6,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpb8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mstpcrb::Mstpb8,
        mstpcrb::Mstpb8,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mstpcrb::Mstpb8,
            mstpcrb::Mstpb8,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpb9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        mstpcrb::Mstpb9,
        mstpcrb::Mstpb9,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            mstpcrb::Mstpb9,
            mstpcrb::Mstpb9,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpb11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        mstpcrb::Mstpb11,
        mstpcrb::Mstpb11,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            mstpcrb::Mstpb11,
            mstpcrb::Mstpb11,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpb19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mstpcrb::Mstpb19,
        mstpcrb::Mstpb19,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mstpcrb::Mstpb19,
            mstpcrb::Mstpb19,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpb22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mstpcrb::Mstpb22,
        mstpcrb::Mstpb22,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mstpcrb::Mstpb22,
            mstpcrb::Mstpb22,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpb27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        mstpcrb::Mstpb27,
        mstpcrb::Mstpb27,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            mstpcrb::Mstpb27,
            mstpcrb::Mstpb27,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpb28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        mstpcrb::Mstpb28,
        mstpcrb::Mstpb28,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            mstpcrb::Mstpb28,
            mstpcrb::Mstpb28,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpb29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        mstpcrb::Mstpb29,
        mstpcrb::Mstpb29,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            mstpcrb::Mstpb29,
            mstpcrb::Mstpb29,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpb30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        mstpcrb::Mstpb30,
        mstpcrb::Mstpb30,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            mstpcrb::Mstpb30,
            mstpcrb::Mstpb30,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpb31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        mstpcrb::Mstpb31,
        mstpcrb::Mstpb31,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            mstpcrb::Mstpb31,
            mstpcrb::Mstpb31,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mstpcrb {
    #[inline(always)]
    fn default() -> Mstpcrb {
        <crate::RegValueT<Mstpcrb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod mstpcrb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb2_SPEC;
    pub type Mstpb2 = crate::EnumBitfieldStruct<u8, Mstpb2_SPEC>;
    impl Mstpb2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb6_SPEC;
    pub type Mstpb6 = crate::EnumBitfieldStruct<u8, Mstpb6_SPEC>;
    impl Mstpb6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb8_SPEC;
    pub type Mstpb8 = crate::EnumBitfieldStruct<u8, Mstpb8_SPEC>;
    impl Mstpb8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb9_SPEC;
    pub type Mstpb9 = crate::EnumBitfieldStruct<u8, Mstpb9_SPEC>;
    impl Mstpb9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb11_SPEC;
    pub type Mstpb11 = crate::EnumBitfieldStruct<u8, Mstpb11_SPEC>;
    impl Mstpb11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb19_SPEC;
    pub type Mstpb19 = crate::EnumBitfieldStruct<u8, Mstpb19_SPEC>;
    impl Mstpb19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb22_SPEC;
    pub type Mstpb22 = crate::EnumBitfieldStruct<u8, Mstpb22_SPEC>;
    impl Mstpb22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb27_SPEC;
    pub type Mstpb27 = crate::EnumBitfieldStruct<u8, Mstpb27_SPEC>;
    impl Mstpb27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb28_SPEC;
    pub type Mstpb28 = crate::EnumBitfieldStruct<u8, Mstpb28_SPEC>;
    impl Mstpb28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb29_SPEC;
    pub type Mstpb29 = crate::EnumBitfieldStruct<u8, Mstpb29_SPEC>;
    impl Mstpb29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb30_SPEC;
    pub type Mstpb30 = crate::EnumBitfieldStruct<u8, Mstpb30_SPEC>;
    impl Mstpb30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb31_SPEC;
    pub type Mstpb31 = crate::EnumBitfieldStruct<u8, Mstpb31_SPEC>;
    impl Mstpb31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrc_SPEC;
impl crate::sealed::RegSpec for Mstpcrc_SPEC {
    type DataType = u32;
}

pub type Mstpcrc = crate::RegValueT<Mstpcrc_SPEC>;

impl Mstpcrc {
    #[inline(always)]
    pub fn mstpc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mstpcrc::Mstpc0,
        mstpcrc::Mstpc0,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mstpcrc::Mstpc0,
            mstpcrc::Mstpc0,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mstpcrc::Mstpc1,
        mstpcrc::Mstpc1,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mstpcrc::Mstpc1,
            mstpcrc::Mstpc1,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mstpcrc::Mstpc3,
        mstpcrc::Mstpc3,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mstpcrc::Mstpc3,
            mstpcrc::Mstpc3,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpc8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mstpcrc::Mstpc8,
        mstpcrc::Mstpc8,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mstpcrc::Mstpc8,
            mstpcrc::Mstpc8,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpc12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        mstpcrc::Mstpc12,
        mstpcrc::Mstpc12,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            mstpcrc::Mstpc12,
            mstpcrc::Mstpc12,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpc13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        mstpcrc::Mstpc13,
        mstpcrc::Mstpc13,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            mstpcrc::Mstpc13,
            mstpcrc::Mstpc13,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpc14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        mstpcrc::Mstpc14,
        mstpcrc::Mstpc14,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            mstpcrc::Mstpc14,
            mstpcrc::Mstpc14,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpc31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        mstpcrc::Mstpc31,
        mstpcrc::Mstpc31,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            mstpcrc::Mstpc31,
            mstpcrc::Mstpc31,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mstpcrc {
    #[inline(always)]
    fn default() -> Mstpcrc {
        <crate::RegValueT<Mstpcrc_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod mstpcrc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc0_SPEC;
    pub type Mstpc0 = crate::EnumBitfieldStruct<u8, Mstpc0_SPEC>;
    impl Mstpc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc1_SPEC;
    pub type Mstpc1 = crate::EnumBitfieldStruct<u8, Mstpc1_SPEC>;
    impl Mstpc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc3_SPEC;
    pub type Mstpc3 = crate::EnumBitfieldStruct<u8, Mstpc3_SPEC>;
    impl Mstpc3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc8_SPEC;
    pub type Mstpc8 = crate::EnumBitfieldStruct<u8, Mstpc8_SPEC>;
    impl Mstpc8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc12_SPEC;
    pub type Mstpc12 = crate::EnumBitfieldStruct<u8, Mstpc12_SPEC>;
    impl Mstpc12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc13_SPEC;
    pub type Mstpc13 = crate::EnumBitfieldStruct<u8, Mstpc13_SPEC>;
    impl Mstpc13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc14_SPEC;
    pub type Mstpc14 = crate::EnumBitfieldStruct<u8, Mstpc14_SPEC>;
    impl Mstpc14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc31_SPEC;
    pub type Mstpc31 = crate::EnumBitfieldStruct<u8, Mstpc31_SPEC>;
    impl Mstpc31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrd_SPEC;
impl crate::sealed::RegSpec for Mstpcrd_SPEC {
    type DataType = u32;
}

pub type Mstpcrd = crate::RegValueT<Mstpcrd_SPEC>;

impl Mstpcrd {
    #[inline(always)]
    pub fn mstpd0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mstpcrd::Mstpd0,
        mstpcrd::Mstpd0,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mstpcrd::Mstpd0,
            mstpcrd::Mstpd0,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpd1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mstpcrd::Mstpd1,
        mstpcrd::Mstpd1,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mstpcrd::Mstpd1,
            mstpcrd::Mstpd1,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpd2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mstpcrd::Mstpd2,
        mstpcrd::Mstpd2,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mstpcrd::Mstpd2,
            mstpcrd::Mstpd2,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpd3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mstpcrd::Mstpd3,
        mstpcrd::Mstpd3,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mstpcrd::Mstpd3,
            mstpcrd::Mstpd3,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpd11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        mstpcrd::Mstpd11,
        mstpcrd::Mstpd11,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            mstpcrd::Mstpd11,
            mstpcrd::Mstpd11,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpd12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        mstpcrd::Mstpd12,
        mstpcrd::Mstpd12,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            mstpcrd::Mstpd12,
            mstpcrd::Mstpd12,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpd13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        mstpcrd::Mstpd13,
        mstpcrd::Mstpd13,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            mstpcrd::Mstpd13,
            mstpcrd::Mstpd13,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpd14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        mstpcrd::Mstpd14,
        mstpcrd::Mstpd14,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            mstpcrd::Mstpd14,
            mstpcrd::Mstpd14,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpd16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mstpcrd::Mstpd16,
        mstpcrd::Mstpd16,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mstpcrd::Mstpd16,
            mstpcrd::Mstpd16,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpd20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        mstpcrd::Mstpd20,
        mstpcrd::Mstpd20,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            mstpcrd::Mstpd20,
            mstpcrd::Mstpd20,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpd22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mstpcrd::Mstpd22,
        mstpcrd::Mstpd22,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mstpcrd::Mstpd22,
            mstpcrd::Mstpd22,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mstpcrd {
    #[inline(always)]
    fn default() -> Mstpcrd {
        <crate::RegValueT<Mstpcrd_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod mstpcrd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd0_SPEC;
    pub type Mstpd0 = crate::EnumBitfieldStruct<u8, Mstpd0_SPEC>;
    impl Mstpd0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd1_SPEC;
    pub type Mstpd1 = crate::EnumBitfieldStruct<u8, Mstpd1_SPEC>;
    impl Mstpd1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd2_SPEC;
    pub type Mstpd2 = crate::EnumBitfieldStruct<u8, Mstpd2_SPEC>;
    impl Mstpd2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd3_SPEC;
    pub type Mstpd3 = crate::EnumBitfieldStruct<u8, Mstpd3_SPEC>;
    impl Mstpd3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd11_SPEC;
    pub type Mstpd11 = crate::EnumBitfieldStruct<u8, Mstpd11_SPEC>;
    impl Mstpd11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd12_SPEC;
    pub type Mstpd12 = crate::EnumBitfieldStruct<u8, Mstpd12_SPEC>;
    impl Mstpd12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd13_SPEC;
    pub type Mstpd13 = crate::EnumBitfieldStruct<u8, Mstpd13_SPEC>;
    impl Mstpd13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd14_SPEC;
    pub type Mstpd14 = crate::EnumBitfieldStruct<u8, Mstpd14_SPEC>;
    impl Mstpd14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd16_SPEC;
    pub type Mstpd16 = crate::EnumBitfieldStruct<u8, Mstpd16_SPEC>;
    impl Mstpd16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd20_SPEC;
    pub type Mstpd20 = crate::EnumBitfieldStruct<u8, Mstpd20_SPEC>;
    impl Mstpd20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd22_SPEC;
    pub type Mstpd22 = crate::EnumBitfieldStruct<u8, Mstpd22_SPEC>;
    impl Mstpd22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcre_SPEC;
impl crate::sealed::RegSpec for Mstpcre_SPEC {
    type DataType = u32;
}

pub type Mstpcre = crate::RegValueT<Mstpcre_SPEC>;

impl Mstpcre {
    #[inline(always)]
    pub fn mstpe14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        mstpcre::Mstpe14,
        mstpcre::Mstpe14,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            mstpcre::Mstpe14,
            mstpcre::Mstpe14,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpe15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        mstpcre::Mstpe15,
        mstpcre::Mstpe15,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            mstpcre::Mstpe15,
            mstpcre::Mstpe15,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpe24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mstpcre::Mstpe24,
        mstpcre::Mstpe24,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mstpcre::Mstpe24,
            mstpcre::Mstpe24,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpe25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mstpcre::Mstpe25,
        mstpcre::Mstpe25,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mstpcre::Mstpe25,
            mstpcre::Mstpe25,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpe26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        mstpcre::Mstpe26,
        mstpcre::Mstpe26,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            mstpcre::Mstpe26,
            mstpcre::Mstpe26,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpe27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        mstpcre::Mstpe27,
        mstpcre::Mstpe27,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            mstpcre::Mstpe27,
            mstpcre::Mstpe27,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpe28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        mstpcre::Mstpe28,
        mstpcre::Mstpe28,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            mstpcre::Mstpe28,
            mstpcre::Mstpe28,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpe29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        mstpcre::Mstpe29,
        mstpcre::Mstpe29,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            mstpcre::Mstpe29,
            mstpcre::Mstpe29,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpe30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        mstpcre::Mstpe30,
        mstpcre::Mstpe30,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            mstpcre::Mstpe30,
            mstpcre::Mstpe30,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstpe31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        mstpcre::Mstpe31,
        mstpcre::Mstpe31,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            mstpcre::Mstpe31,
            mstpcre::Mstpe31,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mstpcre {
    #[inline(always)]
    fn default() -> Mstpcre {
        <crate::RegValueT<Mstpcre_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod mstpcre {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe14_SPEC;
    pub type Mstpe14 = crate::EnumBitfieldStruct<u8, Mstpe14_SPEC>;
    impl Mstpe14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe15_SPEC;
    pub type Mstpe15 = crate::EnumBitfieldStruct<u8, Mstpe15_SPEC>;
    impl Mstpe15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe24_SPEC;
    pub type Mstpe24 = crate::EnumBitfieldStruct<u8, Mstpe24_SPEC>;
    impl Mstpe24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe25_SPEC;
    pub type Mstpe25 = crate::EnumBitfieldStruct<u8, Mstpe25_SPEC>;
    impl Mstpe25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe26_SPEC;
    pub type Mstpe26 = crate::EnumBitfieldStruct<u8, Mstpe26_SPEC>;
    impl Mstpe26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe27_SPEC;
    pub type Mstpe27 = crate::EnumBitfieldStruct<u8, Mstpe27_SPEC>;
    impl Mstpe27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe28_SPEC;
    pub type Mstpe28 = crate::EnumBitfieldStruct<u8, Mstpe28_SPEC>;
    impl Mstpe28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe29_SPEC;
    pub type Mstpe29 = crate::EnumBitfieldStruct<u8, Mstpe29_SPEC>;
    impl Mstpe29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe30_SPEC;
    pub type Mstpe30 = crate::EnumBitfieldStruct<u8, Mstpe30_SPEC>;
    impl Mstpe30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe31_SPEC;
    pub type Mstpe31 = crate::EnumBitfieldStruct<u8, Mstpe31_SPEC>;
    impl Mstpe31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
