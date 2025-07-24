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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:51:01 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Module Stop Control B,C,D"]
unsafe impl ::core::marker::Send for super::Mstp {}
unsafe impl ::core::marker::Sync for super::Mstp {}
impl super::Mstp {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Module Stop Control Register B"]
    #[inline(always)]
    pub const fn mstpcrb(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Module Stop Control Register C"]
    #[inline(always)]
    pub const fn mstpcrc(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcrc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcrc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Module Stop Control Register D"]
    #[inline(always)]
    pub const fn mstpcrd(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcrd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcrd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrb_SPEC;
impl crate::sealed::RegSpec for Mstpcrb_SPEC {
    type DataType = u32;
}

#[doc = "Module Stop Control Register B"]
pub type Mstpcrb = crate::RegValueT<Mstpcrb_SPEC>;

impl Mstpcrb {
    #[doc = "Serial Communication Interface 0 Module Stop"]
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

    #[doc = "Serial Communication Interface 1 Module Stop"]
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

    #[doc = "Serial Communication Interface 2 Module Stop"]
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

    #[doc = "Serial Communication Interface 3 Module Stop"]
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

    #[doc = "Serial Communication Interface 4 Module Stop"]
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

    #[doc = "Serial Communication Interface 8 Module Stop"]
    #[inline(always)]
    pub fn mstpb23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mstpcrb::Mstpb23,
        mstpcrb::Mstpb23,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mstpcrb::Mstpb23,
            mstpcrb::Mstpb23,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial Communication Interface 9 Module Stop"]
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

    #[doc = "Serial Peripheral Interface 0 Module Stop"]
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

    #[doc = "Serial Peripheral Interface Module Stop"]
    #[inline(always)]
    pub fn mstpb18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        mstpcrb::Mstpb18,
        mstpcrb::Mstpb18,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            mstpcrb::Mstpb18,
            mstpcrb::Mstpb18,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Universal Serial Bus 2.0 FS Interface Module Stop"]
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

    #[doc = "I2C Bus Interface 0 Module Stop"]
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

    #[doc = "I2C Bus Interface 1 Module Stop"]
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

    #[doc = "Queued Serial Peripheral Interface Module Stop"]
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

    #[doc = "IrDA Module Stop"]
    #[inline(always)]
    pub fn mstpb5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mstpcrb::Mstpb5,
        mstpcrb::Mstpb5,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mstpcrb::Mstpb5,
            mstpcrb::Mstpb5,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Controller Area Network 0 Module Stop"]
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

    #[doc = "Controller Area Network 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mstpcrb::Mstpb1,
        mstpcrb::Mstpb1,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mstpcrb::Mstpb1,
            mstpcrb::Mstpb1,
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
    pub struct Mstpb31_SPEC;
    pub type Mstpb31 = crate::EnumBitfieldStruct<u8, Mstpb31_SPEC>;
    impl Mstpb31 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb30_SPEC;
    pub type Mstpb30 = crate::EnumBitfieldStruct<u8, Mstpb30_SPEC>;
    impl Mstpb30 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb29_SPEC;
    pub type Mstpb29 = crate::EnumBitfieldStruct<u8, Mstpb29_SPEC>;
    impl Mstpb29 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb28_SPEC;
    pub type Mstpb28 = crate::EnumBitfieldStruct<u8, Mstpb28_SPEC>;
    impl Mstpb28 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb27_SPEC;
    pub type Mstpb27 = crate::EnumBitfieldStruct<u8, Mstpb27_SPEC>;
    impl Mstpb27 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb23_SPEC;
    pub type Mstpb23 = crate::EnumBitfieldStruct<u8, Mstpb23_SPEC>;
    impl Mstpb23 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb22_SPEC;
    pub type Mstpb22 = crate::EnumBitfieldStruct<u8, Mstpb22_SPEC>;
    impl Mstpb22 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb19_SPEC;
    pub type Mstpb19 = crate::EnumBitfieldStruct<u8, Mstpb19_SPEC>;
    impl Mstpb19 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb18_SPEC;
    pub type Mstpb18 = crate::EnumBitfieldStruct<u8, Mstpb18_SPEC>;
    impl Mstpb18 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb11_SPEC;
    pub type Mstpb11 = crate::EnumBitfieldStruct<u8, Mstpb11_SPEC>;
    impl Mstpb11 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb9_SPEC;
    pub type Mstpb9 = crate::EnumBitfieldStruct<u8, Mstpb9_SPEC>;
    impl Mstpb9 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb8_SPEC;
    pub type Mstpb8 = crate::EnumBitfieldStruct<u8, Mstpb8_SPEC>;
    impl Mstpb8 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb6_SPEC;
    pub type Mstpb6 = crate::EnumBitfieldStruct<u8, Mstpb6_SPEC>;
    impl Mstpb6 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb5_SPEC;
    pub type Mstpb5 = crate::EnumBitfieldStruct<u8, Mstpb5_SPEC>;
    impl Mstpb5 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb2_SPEC;
    pub type Mstpb2 = crate::EnumBitfieldStruct<u8, Mstpb2_SPEC>;
    impl Mstpb2 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb1_SPEC;
    pub type Mstpb1 = crate::EnumBitfieldStruct<u8, Mstpb1_SPEC>;
    impl Mstpb1 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrc_SPEC;
impl crate::sealed::RegSpec for Mstpcrc_SPEC {
    type DataType = u32;
}

#[doc = "Module Stop Control Register C"]
pub type Mstpcrc = crate::RegValueT<Mstpcrc_SPEC>;

impl Mstpcrc {
    #[doc = "SCE7 Module Stop"]
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

    #[doc = "Event Link Controller Module Stop"]
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

    #[doc = "Data Operation Circuit Module Stop"]
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

    #[doc = "Secure Digital Host IF/MultiMediaCard 0 Module Stop"]
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

    #[doc = "Secure Digital Host IF/MultiMediaCard 1 Module Stop"]
    #[inline(always)]
    pub fn mstpc11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        mstpcrc::Mstpc11,
        mstpcrc::Mstpc11,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            mstpcrc::Mstpc11,
            mstpcrc::Mstpc11,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sampling Rate Converter Module Stop"]
    #[inline(always)]
    pub fn mstpc9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        mstpcrc::Mstpc9,
        mstpcrc::Mstpc9,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            mstpcrc::Mstpc9,
            mstpcrc::Mstpc9,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial Sound Interface Enhanced (channel 0) Module Stop"]
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

    #[doc = "Capacitive Touch Sensing Unit Module Stop"]
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

    #[doc = "Cyclic Redundancy Check Calculator Module Stop"]
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

    #[doc = "Clock Frequency Accuracy Measurement Circuit Module Stop"]
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
}
impl ::core::default::Default for Mstpcrc {
    #[inline(always)]
    fn default() -> Mstpcrc {
        <crate::RegValueT<Mstpcrc_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod mstpcrc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc31_SPEC;
    pub type Mstpc31 = crate::EnumBitfieldStruct<u8, Mstpc31_SPEC>;
    impl Mstpc31 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc14_SPEC;
    pub type Mstpc14 = crate::EnumBitfieldStruct<u8, Mstpc14_SPEC>;
    impl Mstpc14 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc13_SPEC;
    pub type Mstpc13 = crate::EnumBitfieldStruct<u8, Mstpc13_SPEC>;
    impl Mstpc13 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc12_SPEC;
    pub type Mstpc12 = crate::EnumBitfieldStruct<u8, Mstpc12_SPEC>;
    impl Mstpc12 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc11_SPEC;
    pub type Mstpc11 = crate::EnumBitfieldStruct<u8, Mstpc11_SPEC>;
    impl Mstpc11 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc9_SPEC;
    pub type Mstpc9 = crate::EnumBitfieldStruct<u8, Mstpc9_SPEC>;
    impl Mstpc9 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc8_SPEC;
    pub type Mstpc8 = crate::EnumBitfieldStruct<u8, Mstpc8_SPEC>;
    impl Mstpc8 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc3_SPEC;
    pub type Mstpc3 = crate::EnumBitfieldStruct<u8, Mstpc3_SPEC>;
    impl Mstpc3 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc1_SPEC;
    pub type Mstpc1 = crate::EnumBitfieldStruct<u8, Mstpc1_SPEC>;
    impl Mstpc1 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc0_SPEC;
    pub type Mstpc0 = crate::EnumBitfieldStruct<u8, Mstpc0_SPEC>;
    impl Mstpc0 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrd_SPEC;
impl crate::sealed::RegSpec for Mstpcrd_SPEC {
    type DataType = u32;
}

#[doc = "Module Stop Control Register D"]
pub type Mstpcrd = crate::RegValueT<Mstpcrd_SPEC>;

impl Mstpcrd {
    #[doc = "High-Speed Analog Comparator 0 Module Stop"]
    #[inline(always)]
    pub fn mstpd28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        mstpcrd::Mstpd28,
        mstpcrd::Mstpd28,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            mstpcrd::Mstpd28,
            mstpcrd::Mstpd28,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "High-Speed Analog Comparator 1 Module Stop"]
    #[inline(always)]
    pub fn mstpd27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        mstpcrd::Mstpd27,
        mstpcrd::Mstpd27,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            mstpcrd::Mstpd27,
            mstpcrd::Mstpd27,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "High-Speed Analog Comparator 2 Module Stop"]
    #[inline(always)]
    pub fn mstpd26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        mstpcrd::Mstpd26,
        mstpcrd::Mstpd26,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            mstpcrd::Mstpd26,
            mstpcrd::Mstpd26,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "High-Speed Analog Comparator 3 Module Stop"]
    #[inline(always)]
    pub fn mstpd25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mstpcrd::Mstpd25,
        mstpcrd::Mstpd25,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mstpcrd::Mstpd25,
            mstpcrd::Mstpd25,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "High-Speed Analog Comparator 4 Module Stop"]
    #[inline(always)]
    pub fn mstpd24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mstpcrd::Mstpd24,
        mstpcrd::Mstpd24,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mstpcrd::Mstpd24,
            mstpcrd::Mstpd24,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "High-Speed Analog Comparator 5 Module Stop"]
    #[inline(always)]
    pub fn mstpd23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mstpcrd::Mstpd23,
        mstpcrd::Mstpd23,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mstpcrd::Mstpd23,
            mstpcrd::Mstpd23,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Temperature Sensor Module Stop"]
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

    #[doc = "12-Bit D/A Converter Module Stop"]
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

    #[doc = "12-Bit A/D Converter 0 Module Stop"]
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

    #[doc = "12-Bit A/D Converter 1 Module Stop"]
    #[inline(always)]
    pub fn mstpd15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        mstpcrd::Mstpd15,
        mstpcrd::Mstpd15,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            mstpcrd::Mstpd15,
            mstpcrd::Mstpd15,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Output Enable for GPT Module Stop"]
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

    #[doc = "General PWM Timer 32_8 to 32_12 Module Stop"]
    #[inline(always)]
    pub fn mstpd6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mstpcrd::Mstpd6,
        mstpcrd::Mstpd6,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mstpcrd::Mstpd6,
            mstpcrd::Mstpd6,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "General PWM Timer 32EH0 to 32EH3 and 32E4 to 32E7 and PWM Delay Generation Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpd5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mstpcrd::Mstpd5,
        mstpcrd::Mstpd5,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mstpcrd::Mstpd5,
            mstpcrd::Mstpd5,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT0 Module Stop  \n Note: AGT0 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT0."]
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

    #[doc = "AGT1 Module Stop\n Note: AGT1 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT1."]
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
}
impl ::core::default::Default for Mstpcrd {
    #[inline(always)]
    fn default() -> Mstpcrd {
        <crate::RegValueT<Mstpcrd_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod mstpcrd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd28_SPEC;
    pub type Mstpd28 = crate::EnumBitfieldStruct<u8, Mstpd28_SPEC>;
    impl Mstpd28 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd27_SPEC;
    pub type Mstpd27 = crate::EnumBitfieldStruct<u8, Mstpd27_SPEC>;
    impl Mstpd27 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd26_SPEC;
    pub type Mstpd26 = crate::EnumBitfieldStruct<u8, Mstpd26_SPEC>;
    impl Mstpd26 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd25_SPEC;
    pub type Mstpd25 = crate::EnumBitfieldStruct<u8, Mstpd25_SPEC>;
    impl Mstpd25 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd24_SPEC;
    pub type Mstpd24 = crate::EnumBitfieldStruct<u8, Mstpd24_SPEC>;
    impl Mstpd24 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd23_SPEC;
    pub type Mstpd23 = crate::EnumBitfieldStruct<u8, Mstpd23_SPEC>;
    impl Mstpd23 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd22_SPEC;
    pub type Mstpd22 = crate::EnumBitfieldStruct<u8, Mstpd22_SPEC>;
    impl Mstpd22 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd20_SPEC;
    pub type Mstpd20 = crate::EnumBitfieldStruct<u8, Mstpd20_SPEC>;
    impl Mstpd20 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd16_SPEC;
    pub type Mstpd16 = crate::EnumBitfieldStruct<u8, Mstpd16_SPEC>;
    impl Mstpd16 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd15_SPEC;
    pub type Mstpd15 = crate::EnumBitfieldStruct<u8, Mstpd15_SPEC>;
    impl Mstpd15 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd14_SPEC;
    pub type Mstpd14 = crate::EnumBitfieldStruct<u8, Mstpd14_SPEC>;
    impl Mstpd14 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd6_SPEC;
    pub type Mstpd6 = crate::EnumBitfieldStruct<u8, Mstpd6_SPEC>;
    impl Mstpd6 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd5_SPEC;
    pub type Mstpd5 = crate::EnumBitfieldStruct<u8, Mstpd5_SPEC>;
    impl Mstpd5 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd3_SPEC;
    pub type Mstpd3 = crate::EnumBitfieldStruct<u8, Mstpd3_SPEC>;
    impl Mstpd3 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd2_SPEC;
    pub type Mstpd2 = crate::EnumBitfieldStruct<u8, Mstpd2_SPEC>;
    impl Mstpd2 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
}
