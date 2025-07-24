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
// Generated from SVD 1.20.01, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:48:57 +0000

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

    #[doc = "Sub Oscillation Stop Detection Module Stop"]
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

    #[doc = "DMA Controller/Data Transfer Controller Module Stop"]
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
        <crate::RegValueT<Mstpcra_SPEC> as RegisterValue<_>>::new(4290772990)
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
    #[doc = "Serial Interface UARTA Module Stop"]
    #[inline(always)]
    pub fn mstpb0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mstpcrb::Mstpb0,
        mstpcrb::Mstpb0,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mstpcrb::Mstpb0,
            mstpcrb::Mstpb0,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

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

    #[doc = "IrDA Interface Module Stop"]
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

    #[doc = "Quad Serial Peripheral Interface Module Stop"]
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
    pub struct Mstpb0_SPEC;
    pub type Mstpb0 = crate::EnumBitfieldStruct<u8, Mstpb0_SPEC>;
    impl Mstpb0 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
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
    pub struct Mstpb5_SPEC;
    pub type Mstpb5 = crate::EnumBitfieldStruct<u8, Mstpb5_SPEC>;
    impl Mstpb5 {
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

    #[doc = "Segment LCD Controller Module Stop"]
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

    #[doc = "Serial Sound Interface Enhanced Module Stop"]
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

    #[doc = "Controller Area Network with Flexible Data-Rate Module Stop"]
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

    #[doc = "RSIP-E11A Module Stop"]
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
    pub struct Mstpc3_SPEC;
    pub type Mstpc3 = crate::EnumBitfieldStruct<u8, Mstpc3_SPEC>;
    impl Mstpc3 {
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

    #[doc = "Low Power Asynchronous General Purpose Timer 0 Module Stop"]
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

    #[doc = "Low-Power Analog Comparator Module Stop"]
    #[inline(always)]
    pub fn mstpd29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        mstpcrd::Mstpd29,
        mstpcrd::Mstpd29,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            mstpcrd::Mstpd29,
            mstpcrd::Mstpd29,
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
    pub struct Mstpd2_SPEC;
    pub type Mstpd2 = crate::EnumBitfieldStruct<u8, Mstpd2_SPEC>;
    impl Mstpd2 {
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
    pub struct Mstpd29_SPEC;
    pub type Mstpd29 = crate::EnumBitfieldStruct<u8, Mstpd29_SPEC>;
    impl Mstpd29 {
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
    #[doc = "GPT5 Module Stop"]
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

    #[doc = "GPT4 Module Stop"]
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

    #[doc = "GPT3 Module Stop"]
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

    #[doc = "GPT2 Module Stop"]
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

    #[doc = "GPT1 Module Stop"]
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

    #[doc = "GPT0 Module Stop"]
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
