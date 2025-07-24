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
// Generated from SVD 1.00.01, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:53:56 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Module Stop Control"]
unsafe impl ::core::marker::Send for super::MstpNs {}
unsafe impl ::core::marker::Sync for super::MstpNs {}
impl super::MstpNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Module Stop Control Register A"]
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

    #[doc = "Module Stop Control Register B"]
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

    #[doc = "Module Stop Control Register C"]
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

    #[doc = "Module Stop Control Register D"]
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

    #[doc = "Module Stop Control Register E"]
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

#[doc = "Module Stop Control Register A"]
pub type Mstpcra = crate::RegValueT<Mstpcra_SPEC>;

impl Mstpcra {
    #[doc = "Unnecessary Circuit Module Stop"]
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

    #[doc = "SRAM1 Module Stop"]
    #[inline(always)]
    pub fn mstpa1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mstpcra::Mstpa1,
        mstpcra::Mstpa1,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mstpcra::Mstpa1,
            mstpcra::Mstpa1,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Standby SRAM Module Stop"]
    #[inline(always)]
    pub fn mstpa15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        mstpcra::Mstpa15,
        mstpcra::Mstpa15,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            mstpcra::Mstpa15,
            mstpcra::Mstpa15,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DMA Controller and Data Transfer Controller Module Stop"]
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
        <crate::RegValueT<Mstpcra_SPEC> as RegisterValue<_>>::new(4290740220)
    }
}
pub mod mstpcra {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa0_SPEC;
    pub type Mstpa0 = crate::EnumBitfieldStruct<u8, Mstpa0_SPEC>;
    impl Mstpa0 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa1_SPEC;
    pub type Mstpa1 = crate::EnumBitfieldStruct<u8, Mstpa1_SPEC>;
    impl Mstpa1 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa15_SPEC;
    pub type Mstpa15 = crate::EnumBitfieldStruct<u8, Mstpa15_SPEC>;
    impl Mstpa15 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa22_SPEC;
    pub type Mstpa22 = crate::EnumBitfieldStruct<u8, Mstpa22_SPEC>;
    impl Mstpa22 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
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

    #[doc = "ETHERC0 and EDMAC0 Controller Module Stop"]
    #[inline(always)]
    pub fn mstpb15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        mstpcrb::Mstpb15,
        mstpcrb::Mstpb15,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            mstpcrb::Mstpb15,
            mstpcrb::Mstpb15,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Octal Serial Peripheral Interface Module Stop"]
    #[inline(always)]
    pub fn mstpb16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mstpcrb::Mstpb16,
        mstpcrb::Mstpb16,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mstpcrb::Mstpb16,
            mstpcrb::Mstpb16,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial Peripheral Interface 1 Module Stop"]
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
}
impl ::core::default::Default for Mstpcrb {
    #[inline(always)]
    fn default() -> Mstpcrb {
        <crate::RegValueT<Mstpcrb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod mstpcrb {

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
    pub struct Mstpb9_SPEC;
    pub type Mstpb9 = crate::EnumBitfieldStruct<u8, Mstpb9_SPEC>;
    impl Mstpb9 {
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
    pub struct Mstpb15_SPEC;
    pub type Mstpb15 = crate::EnumBitfieldStruct<u8, Mstpb15_SPEC>;
    impl Mstpb15 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb16_SPEC;
    pub type Mstpb16 = crate::EnumBitfieldStruct<u8, Mstpb16_SPEC>;
    impl Mstpb16 {
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
    pub struct Mstpb19_SPEC;
    pub type Mstpb19 = crate::EnumBitfieldStruct<u8, Mstpb19_SPEC>;
    impl Mstpb19 {
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
    pub struct Mstpb27_SPEC;
    pub type Mstpb27 = crate::EnumBitfieldStruct<u8, Mstpb27_SPEC>;
    impl Mstpb27 {
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
    pub struct Mstpb29_SPEC;
    pub type Mstpb29 = crate::EnumBitfieldStruct<u8, Mstpb29_SPEC>;
    impl Mstpb29 {
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
    pub struct Mstpb31_SPEC;
    pub type Mstpb31 = crate::EnumBitfieldStruct<u8, Mstpb31_SPEC>;
    impl Mstpb31 {
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

    #[doc = "Serial Sound Interface Enhanced 1 Module Stop"]
    #[inline(always)]
    pub fn mstpc7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mstpcrc::Mstpc7,
        mstpcrc::Mstpc7,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mstpcrc::Mstpc7,
            mstpcrc::Mstpc7,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial Sound Interface Enhanced 0 Module Stop"]
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

    #[doc = "Capture Engine Unit Module Stop"]
    #[inline(always)]
    pub fn mstpc16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mstpcrc::Mstpc16,
        mstpcrc::Mstpc16,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mstpcrc::Mstpc16,
            mstpcrc::Mstpc16,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Controller Area Network with Flexible Data-Rate 1 Module Stop"]
    #[inline(always)]
    pub fn mstpc26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        mstpcrc::Mstpc26,
        mstpcrc::Mstpc26,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            mstpcrc::Mstpc26,
            mstpcrc::Mstpc26,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Controller Area Network with Flexible Data-Rate 0 Module Stop"]
    #[inline(always)]
    pub fn mstpc27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        mstpcrc::Mstpc27,
        mstpcrc::Mstpc27,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            mstpcrc::Mstpc27,
            mstpcrc::Mstpc27,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Renesas Secure IP Module Stop"]
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
    pub struct Mstpc7_SPEC;
    pub type Mstpc7 = crate::EnumBitfieldStruct<u8, Mstpc7_SPEC>;
    impl Mstpc7 {
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
    pub struct Mstpc13_SPEC;
    pub type Mstpc13 = crate::EnumBitfieldStruct<u8, Mstpc13_SPEC>;
    impl Mstpc13 {
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
    pub struct Mstpc16_SPEC;
    pub type Mstpc16 = crate::EnumBitfieldStruct<u8, Mstpc16_SPEC>;
    impl Mstpc16 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc26_SPEC;
    pub type Mstpc26 = crate::EnumBitfieldStruct<u8, Mstpc26_SPEC>;
    impl Mstpc26 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc27_SPEC;
    pub type Mstpc27 = crate::EnumBitfieldStruct<u8, Mstpc27_SPEC>;
    impl Mstpc27 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc31_SPEC;
    pub type Mstpc31 = crate::EnumBitfieldStruct<u8, Mstpc31_SPEC>;
    impl Mstpc31 {
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
    #[doc = "Low Power Asynchronous General Purpose Timer 1 Module Stop"]
    #[inline(always)]
    pub fn mstpd4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mstpcrd::Mstpd4,
        mstpcrd::Mstpd4,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mstpcrd::Mstpd4,
            mstpcrd::Mstpd4,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Low Power Asynchronous General Purpose Timer 0 Module Stop"]
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

    #[doc = "Port Output Enable for GPT Group D Module Stop"]
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

    #[doc = "Port Output Enable for GPT Group C Module Stop"]
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

    #[doc = "Port Output Enable for GPT Group B Module Stop"]
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

    #[doc = "Port Output Enable for GPT Group A Module Stop"]
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

    #[doc = "12-bit A/D Converter 1 Module Stop"]
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

    #[doc = "12-bit A/D Converter 0 Module Stop"]
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

    #[doc = "12-bit D/A Converter Module Stop"]
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
}
impl ::core::default::Default for Mstpcrd {
    #[inline(always)]
    fn default() -> Mstpcrd {
        <crate::RegValueT<Mstpcrd_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod mstpcrd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd4_SPEC;
    pub type Mstpd4 = crate::EnumBitfieldStruct<u8, Mstpd4_SPEC>;
    impl Mstpd4 {
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
    pub struct Mstpd11_SPEC;
    pub type Mstpd11 = crate::EnumBitfieldStruct<u8, Mstpd11_SPEC>;
    impl Mstpd11 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd12_SPEC;
    pub type Mstpd12 = crate::EnumBitfieldStruct<u8, Mstpd12_SPEC>;
    impl Mstpd12 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd13_SPEC;
    pub type Mstpd13 = crate::EnumBitfieldStruct<u8, Mstpd13_SPEC>;
    impl Mstpd13 {
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
    pub struct Mstpd15_SPEC;
    pub type Mstpd15 = crate::EnumBitfieldStruct<u8, Mstpd15_SPEC>;
    impl Mstpd15 {
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
    pub struct Mstpd20_SPEC;
    pub type Mstpd20 = crate::EnumBitfieldStruct<u8, Mstpd20_SPEC>;
    impl Mstpd20 {
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
    pub struct Mstpd27_SPEC;
    pub type Mstpd27 = crate::EnumBitfieldStruct<u8, Mstpd27_SPEC>;
    impl Mstpd27 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd28_SPEC;
    pub type Mstpd28 = crate::EnumBitfieldStruct<u8, Mstpd28_SPEC>;
    impl Mstpd28 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcre_SPEC;
impl crate::sealed::RegSpec for Mstpcre_SPEC {
    type DataType = u32;
}

#[doc = "Module Stop Control Register E"]
pub type Mstpcre = crate::RegValueT<Mstpcre_SPEC>;

impl Mstpcre {
    #[doc = "Ultra-Low Power Timer 1 Module Stop"]
    #[inline(always)]
    pub fn mstpe8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mstpcre::Mstpe8,
        mstpcre::Mstpe8,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mstpcre::Mstpe8,
            mstpcre::Mstpe8,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ultra-Low Power Timer 0 Module Stop"]
    #[inline(always)]
    pub fn mstpe9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        mstpcre::Mstpe9,
        mstpcre::Mstpe9,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            mstpcre::Mstpe9,
            mstpcre::Mstpe9,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "General PWM Timer 13 Module"]
    #[inline(always)]
    pub fn mstpe18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        mstpcre::Mstpe18,
        mstpcre::Mstpe18,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            mstpcre::Mstpe18,
            mstpcre::Mstpe18,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "General PWM Timer 12 Module"]
    #[inline(always)]
    pub fn mstpe19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mstpcre::Mstpe19,
        mstpcre::Mstpe19,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mstpcre::Mstpe19,
            mstpcre::Mstpe19,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "General PWM Timer 11 Module"]
    #[inline(always)]
    pub fn mstpe20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        mstpcre::Mstpe20,
        mstpcre::Mstpe20,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            mstpcre::Mstpe20,
            mstpcre::Mstpe20,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "General PWM Timer 10 Module"]
    #[inline(always)]
    pub fn mstpe21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mstpcre::Mstpe21,
        mstpcre::Mstpe21,
        Mstpcre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mstpcre::Mstpe21,
            mstpcre::Mstpe21,
            Mstpcre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "General PWM Timer 5 Module"]
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

    #[doc = "General PWM Timer 4 Module"]
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

    #[doc = "General PWM Timer 3 Module"]
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

    #[doc = "General PWM Timer 2 Module"]
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

    #[doc = "General PWM Timer 1 Module"]
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

    #[doc = "General PWM Timer 0 Module"]
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
    pub struct Mstpe8_SPEC;
    pub type Mstpe8 = crate::EnumBitfieldStruct<u8, Mstpe8_SPEC>;
    impl Mstpe8 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe9_SPEC;
    pub type Mstpe9 = crate::EnumBitfieldStruct<u8, Mstpe9_SPEC>;
    impl Mstpe9 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe18_SPEC;
    pub type Mstpe18 = crate::EnumBitfieldStruct<u8, Mstpe18_SPEC>;
    impl Mstpe18 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe19_SPEC;
    pub type Mstpe19 = crate::EnumBitfieldStruct<u8, Mstpe19_SPEC>;
    impl Mstpe19 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe20_SPEC;
    pub type Mstpe20 = crate::EnumBitfieldStruct<u8, Mstpe20_SPEC>;
    impl Mstpe20 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe21_SPEC;
    pub type Mstpe21 = crate::EnumBitfieldStruct<u8, Mstpe21_SPEC>;
    impl Mstpe21 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe26_SPEC;
    pub type Mstpe26 = crate::EnumBitfieldStruct<u8, Mstpe26_SPEC>;
    impl Mstpe26 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe27_SPEC;
    pub type Mstpe27 = crate::EnumBitfieldStruct<u8, Mstpe27_SPEC>;
    impl Mstpe27 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe28_SPEC;
    pub type Mstpe28 = crate::EnumBitfieldStruct<u8, Mstpe28_SPEC>;
    impl Mstpe28 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe29_SPEC;
    pub type Mstpe29 = crate::EnumBitfieldStruct<u8, Mstpe29_SPEC>;
    impl Mstpe29 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe30_SPEC;
    pub type Mstpe30 = crate::EnumBitfieldStruct<u8, Mstpe30_SPEC>;
    impl Mstpe30 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpe31_SPEC;
    pub type Mstpe31 = crate::EnumBitfieldStruct<u8, Mstpe31_SPEC>;
    impl Mstpe31 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
}
