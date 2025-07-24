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
// Generated from SVD 1.00.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:55:39 +0000

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
    #[doc = "SRAM0 Module Stop"]
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

    #[doc = "SRAM2 Module Stop"]
    #[inline(always)]
    pub fn mstpa2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mstpcra::Mstpa2,
        mstpcra::Mstpa2,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mstpcra::Mstpa2,
            mstpcra::Mstpa2,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SRAM3 Module Stop"]
    #[inline(always)]
    pub fn mstpa3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mstpcra::Mstpa3,
        mstpcra::Mstpa3,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mstpcra::Mstpa3,
            mstpcra::Mstpa3,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NPU Module Stop"]
    #[inline(always)]
    pub fn mstpa16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mstpcra::Mstpa16,
        mstpcra::Mstpa16,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mstpcra::Mstpa16,
            mstpcra::Mstpa16,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DMA Controller 0 and Data Transfer Controller 0 Module Stop"]
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

    #[doc = "DMA Controller 1 and Data Transfer Controller 1 Module Stop"]
    #[inline(always)]
    pub fn mstpa23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mstpcra::Mstpa23,
        mstpcra::Mstpa23,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mstpcra::Mstpa23,
            mstpcra::Mstpa23,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mstpcra {
    #[inline(always)]
    fn default() -> Mstpcra {
        <crate::RegValueT<Mstpcra_SPEC> as RegisterValue<_>>::new(4282384368)
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
    pub struct Mstpa2_SPEC;
    pub type Mstpa2 = crate::EnumBitfieldStruct<u8, Mstpa2_SPEC>;
    impl Mstpa2 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa3_SPEC;
    pub type Mstpa3 = crate::EnumBitfieldStruct<u8, Mstpa3_SPEC>;
    impl Mstpa3 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa16_SPEC;
    pub type Mstpa16 = crate::EnumBitfieldStruct<u8, Mstpa16_SPEC>;
    impl Mstpa16 {
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa23_SPEC;
    pub type Mstpa23 = crate::EnumBitfieldStruct<u8, Mstpa23_SPEC>;
    impl Mstpa23 {
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
    #[doc = "I3C Bus Interface Module Stop"]
    #[inline(always)]
    pub fn mstpb4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mstpcrb::Mstpb4,
        mstpcrb::Mstpb4,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mstpcrb::Mstpb4,
            mstpcrb::Mstpb4,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IIC Bus Interface 2 Module Stop"]
    #[inline(always)]
    pub fn mstpb7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mstpcrb::Mstpb7,
        mstpcrb::Mstpb7,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mstpcrb::Mstpb7,
            mstpcrb::Mstpb7,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IIC Bus Interface 1 Module Stop"]
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

    #[doc = "IIC Bus Interface 0 Module Stop"]
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

    #[doc = "Universal Serial Bus 2.0 FS Interface 0 Module Stop"]
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

    #[doc = "Universal Serial Bus 2.0 HS Interface Module Stop"]
    #[inline(always)]
    pub fn mstpb12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        mstpcrb::Mstpb12,
        mstpcrb::Mstpb12,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            mstpcrb::Mstpb12,
            mstpcrb::Mstpb12,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Octal Serial Peripheral Interface 0 and Decryption On The Fly 0 Module Stop"]
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

    #[doc = "Octal Serial Peripheral Interface 1 and Decryption On The Fly 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mstpcrb::Mstpb17,
        mstpcrb::Mstpb17,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mstpcrb::Mstpb17,
            mstpcrb::Mstpb17,
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

    #[doc = "Serial Communication Interface 7 Module Stop"]
    #[inline(always)]
    pub fn mstpb24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mstpcrb::Mstpb24,
        mstpcrb::Mstpb24,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mstpcrb::Mstpb24,
            mstpcrb::Mstpb24,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial Communication Interface 6 Module Stop"]
    #[inline(always)]
    pub fn mstpb25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mstpcrb::Mstpb25,
        mstpcrb::Mstpb25,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mstpcrb::Mstpb25,
            mstpcrb::Mstpb25,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial Communication Interface 5 Module Stop"]
    #[inline(always)]
    pub fn mstpb26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        mstpcrb::Mstpb26,
        mstpcrb::Mstpb26,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            mstpcrb::Mstpb26,
            mstpcrb::Mstpb26,
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
    pub struct Mstpb4_SPEC;
    pub type Mstpb4 = crate::EnumBitfieldStruct<u8, Mstpb4_SPEC>;
    impl Mstpb4 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb7_SPEC;
    pub type Mstpb7 = crate::EnumBitfieldStruct<u8, Mstpb7_SPEC>;
    impl Mstpb7 {
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
    pub struct Mstpb12_SPEC;
    pub type Mstpb12 = crate::EnumBitfieldStruct<u8, Mstpb12_SPEC>;
    impl Mstpb12 {
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
    pub struct Mstpb17_SPEC;
    pub type Mstpb17 = crate::EnumBitfieldStruct<u8, Mstpb17_SPEC>;
    impl Mstpb17 {
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
    pub struct Mstpb23_SPEC;
    pub type Mstpb23 = crate::EnumBitfieldStruct<u8, Mstpb23_SPEC>;
    impl Mstpb23 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb24_SPEC;
    pub type Mstpb24 = crate::EnumBitfieldStruct<u8, Mstpb24_SPEC>;
    impl Mstpb24 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb25_SPEC;
    pub type Mstpb25 = crate::EnumBitfieldStruct<u8, Mstpb25_SPEC>;
    impl Mstpb25 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb26_SPEC;
    pub type Mstpb26 = crate::EnumBitfieldStruct<u8, Mstpb26_SPEC>;
    impl Mstpb26 {
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

    #[doc = "Graphics LCD Controller Module Stop"]
    #[inline(always)]
    pub fn mstpc4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mstpcrc::Mstpc4,
        mstpcrc::Mstpc4,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mstpcrc::Mstpc4,
            mstpcrc::Mstpc4,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "2D Drawing Engine Module Stop"]
    #[inline(always)]
    pub fn mstpc6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mstpcrc::Mstpc6,
        mstpcrc::Mstpc6,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mstpcrc::Mstpc6,
            mstpcrc::Mstpc6,
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

    #[doc = "MIPI Display Serial Interface Module Stop"]
    #[inline(always)]
    pub fn mstpc10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        mstpcrc::Mstpc10,
        mstpcrc::Mstpc10,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            mstpcrc::Mstpc10,
            mstpcrc::Mstpc10,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Secure Digital Host IF 1 Module Stop"]
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

    #[doc = "Secure Digital Host IF 0 Module Stop"]
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

    #[doc = "MIPI Camera Serial Interface Module Stop"]
    #[inline(always)]
    pub fn mstpc17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mstpcrc::Mstpc17,
        mstpcrc::Mstpc17,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mstpcrc::Mstpc17,
            mstpcrc::Mstpc17,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Density Modulation Interface Module Stop"]
    #[inline(always)]
    pub fn mstpc24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mstpcrc::Mstpc24,
        mstpcrc::Mstpc24,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mstpcrc::Mstpc24,
            mstpcrc::Mstpc24,
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

    #[doc = "Ether-PHY clock Module Stop"]
    #[inline(always)]
    pub fn mstpc28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        mstpcrc::Mstpc28,
        mstpcrc::Mstpc28,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            mstpcrc::Mstpc28,
            mstpcrc::Mstpc28,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 3 Ethernet Switch Module Module Stop"]
    #[inline(always)]
    pub fn mstpc30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        mstpcrc::Mstpc30,
        mstpcrc::Mstpc30,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            mstpcrc::Mstpc30,
            mstpcrc::Mstpc30,
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
    pub struct Mstpc4_SPEC;
    pub type Mstpc4 = crate::EnumBitfieldStruct<u8, Mstpc4_SPEC>;
    impl Mstpc4 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc6_SPEC;
    pub type Mstpc6 = crate::EnumBitfieldStruct<u8, Mstpc6_SPEC>;
    impl Mstpc6 {
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
    pub struct Mstpc10_SPEC;
    pub type Mstpc10 = crate::EnumBitfieldStruct<u8, Mstpc10_SPEC>;
    impl Mstpc10 {
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
    pub struct Mstpc12_SPEC;
    pub type Mstpc12 = crate::EnumBitfieldStruct<u8, Mstpc12_SPEC>;
    impl Mstpc12 {
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
    pub struct Mstpc17_SPEC;
    pub type Mstpc17 = crate::EnumBitfieldStruct<u8, Mstpc17_SPEC>;
    impl Mstpc17 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc24_SPEC;
    pub type Mstpc24 = crate::EnumBitfieldStruct<u8, Mstpc24_SPEC>;
    impl Mstpc24 {
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
    pub struct Mstpc28_SPEC;
    pub type Mstpc28 = crate::EnumBitfieldStruct<u8, Mstpc28_SPEC>;
    impl Mstpc28 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc30_SPEC;
    pub type Mstpc30 = crate::EnumBitfieldStruct<u8, Mstpc30_SPEC>;
    impl Mstpc30 {
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
    #[doc = "Low power Asynchronous General Purpose Timer 1 Module Stop"]
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

    #[doc = "Low power Asynchronous General Purpose Timer 0 Module Stop"]
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

    #[doc = "PWM Delay Generation Circuit Module Stop"]
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

    #[doc = "12-bit D/A Converter 1 Module Stop"]
    #[inline(always)]
    pub fn mstpd19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mstpcrd::Mstpd19,
        mstpcrd::Mstpd19,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mstpcrd::Mstpd19,
            mstpcrd::Mstpd19,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "12-bit D/A Converter 0 Module Stop"]
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

    #[doc = "16-Bit A/D Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mstpcrd::Mstpd21,
        mstpcrd::Mstpd21,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mstpcrd::Mstpd21,
            mstpcrd::Mstpd21,
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
    pub struct Mstpd6_SPEC;
    pub type Mstpd6 = crate::EnumBitfieldStruct<u8, Mstpd6_SPEC>;
    impl Mstpd6 {
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
    pub struct Mstpd19_SPEC;
    pub type Mstpd19 = crate::EnumBitfieldStruct<u8, Mstpd19_SPEC>;
    impl Mstpd19 {
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
    pub struct Mstpd21_SPEC;
    pub type Mstpd21 = crate::EnumBitfieldStruct<u8, Mstpd21_SPEC>;
    impl Mstpd21 {
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
    pub struct Mstpd25_SPEC;
    pub type Mstpd25 = crate::EnumBitfieldStruct<u8, Mstpd25_SPEC>;
    impl Mstpd25 {
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

    #[doc = "General PWM Timer 13 Module Stop"]
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

    #[doc = "General PWM Timer 12 Module Stop"]
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

    #[doc = "General PWM Timer 11 Module Stop"]
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

    #[doc = "General PWM Timer 10 Module Stop"]
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

    #[doc = "General PWM Timer 4-9 Module Stop"]
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

    #[doc = "General PWM Timer 3 Module Stop"]
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

    #[doc = "General PWM Timer 2 Module Stop"]
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

    #[doc = "General PWM Timer 1 Module Stop"]
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

    #[doc = "General PWM Timer 0 Module Stop"]
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
