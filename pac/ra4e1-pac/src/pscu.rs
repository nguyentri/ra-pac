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
// Generated from SVD 1.20.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:48:16 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Peripheral Security Control Unit"]
unsafe impl ::core::marker::Send for super::Pscu {}
unsafe impl ::core::marker::Sync for super::Pscu {}
impl super::Pscu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Peripheral Security Attribution Register B"]
    #[inline(always)]
    pub const fn psarb(&self) -> &'static crate::common::Reg<self::Psarb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psarb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Peripheral Security Attribution Register C"]
    #[inline(always)]
    pub const fn psarc(&self) -> &'static crate::common::Reg<self::Psarc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psarc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Peripheral Security Attribution Register D"]
    #[inline(always)]
    pub const fn psard(&self) -> &'static crate::common::Reg<self::Psard_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psard_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Peripheral Security Attribution Register E"]
    #[inline(always)]
    pub const fn psare(&self) -> &'static crate::common::Reg<self::Psare_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psare_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Module Stop Security Attribution Register"]
    #[inline(always)]
    pub const fn mssar(&self) -> &'static crate::common::Reg<self::Mssar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mssar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Code Flash Security Attribution Monitor Register A"]
    #[inline(always)]
    pub const fn cfsamona(
        &self,
    ) -> &'static crate::common::Reg<self::Cfsamona_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfsamona_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Code Flash Security Attribution Monitor Register B"]
    #[inline(always)]
    pub const fn cfsamonb(
        &self,
    ) -> &'static crate::common::Reg<self::Cfsamonb_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfsamonb_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Data Flash Security Attribution Monitor Register"]
    #[inline(always)]
    pub const fn dfsamon(
        &self,
    ) -> &'static crate::common::Reg<self::Dfsamon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dfsamon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "SRAM Security Attribution Monitor Register A"]
    #[inline(always)]
    pub const fn ssamona(
        &self,
    ) -> &'static crate::common::Reg<self::Ssamona_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ssamona_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "SRAM Security Attribution Monitor Register B"]
    #[inline(always)]
    pub const fn ssamonb(
        &self,
    ) -> &'static crate::common::Reg<self::Ssamonb_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ssamonb_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Device Lifecycle Management State Monitor Register"]
    #[inline(always)]
    pub const fn dlmmon(&self) -> &'static crate::common::Reg<self::Dlmmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dlmmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psarb_SPEC;
impl crate::sealed::RegSpec for Psarb_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Security Attribution Register B"]
pub type Psarb = crate::RegValueT<Psarb_SPEC>;

impl Psarb {
    #[doc = "CAN0 and the MSTPCRB.MSTPB2 bit security attribution"]
    #[inline(always)]
    pub fn psarb2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        psarb::Psarb2,
        psarb::Psarb2,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            psarb::Psarb2,
            psarb::Psarb2,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "QSPI and the MSTPCRB.MSTPB6 bit security attribution"]
    #[inline(always)]
    pub fn psarb6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Psarb_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Psarb_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "IIC0 and the MSTPCRB.MSTPB9 bit security attribution"]
    #[inline(always)]
    pub fn psarb9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        psarb::Psarb9,
        psarb::Psarb9,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            psarb::Psarb9,
            psarb::Psarb9,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USBFS and the MSTPCRB.MSTPB11 bit security attribution"]
    #[inline(always)]
    pub fn psarb11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        psarb::Psarb11,
        psarb::Psarb11,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            psarb::Psarb11,
            psarb::Psarb11,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI0 and the MSTPCRB.MSTPB19 bit security attribution"]
    #[inline(always)]
    pub fn psarb19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        psarb::Psarb19,
        psarb::Psarb19,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            psarb::Psarb19,
            psarb::Psarb19,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCI9 and the MSTPCRB.MSTPB22 bit security attribution"]
    #[inline(always)]
    pub fn psarb22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        psarb::Psarb22,
        psarb::Psarb22,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            psarb::Psarb22,
            psarb::Psarb22,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCI4 and the MSTPCRB.MSTPB27 bit security attribution"]
    #[inline(always)]
    pub fn psarb27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        psarb::Psarb27,
        psarb::Psarb27,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            psarb::Psarb27,
            psarb::Psarb27,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCI3 and the MSTPCRB.MSTPB28 bit security attribution"]
    #[inline(always)]
    pub fn psarb28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        psarb::Psarb28,
        psarb::Psarb28,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            psarb::Psarb28,
            psarb::Psarb28,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCI2 and the MSTPCRB.MSTPB29 bit security attribution"]
    #[inline(always)]
    pub fn psarb29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        psarb::Psarb29,
        psarb::Psarb29,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            psarb::Psarb29,
            psarb::Psarb29,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCI1 and the MSTPCRB.MSTPB30 bit security attribution"]
    #[inline(always)]
    pub fn psarb30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        psarb::Psarb30,
        psarb::Psarb30,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            psarb::Psarb30,
            psarb::Psarb30,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCI0 and the MSTPCRB.MSTPB31 bit security attribution"]
    #[inline(always)]
    pub fn psarb31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        psarb::Psarb31,
        psarb::Psarb31,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            psarb::Psarb31,
            psarb::Psarb31,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Psarb {
    #[inline(always)]
    fn default() -> Psarb {
        <crate::RegValueT<Psarb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod psarb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb2_SPEC;
    pub type Psarb2 = crate::EnumBitfieldStruct<u8, Psarb2_SPEC>;
    impl Psarb2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb9_SPEC;
    pub type Psarb9 = crate::EnumBitfieldStruct<u8, Psarb9_SPEC>;
    impl Psarb9 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb11_SPEC;
    pub type Psarb11 = crate::EnumBitfieldStruct<u8, Psarb11_SPEC>;
    impl Psarb11 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb19_SPEC;
    pub type Psarb19 = crate::EnumBitfieldStruct<u8, Psarb19_SPEC>;
    impl Psarb19 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb22_SPEC;
    pub type Psarb22 = crate::EnumBitfieldStruct<u8, Psarb22_SPEC>;
    impl Psarb22 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb27_SPEC;
    pub type Psarb27 = crate::EnumBitfieldStruct<u8, Psarb27_SPEC>;
    impl Psarb27 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb28_SPEC;
    pub type Psarb28 = crate::EnumBitfieldStruct<u8, Psarb28_SPEC>;
    impl Psarb28 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb29_SPEC;
    pub type Psarb29 = crate::EnumBitfieldStruct<u8, Psarb29_SPEC>;
    impl Psarb29 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb30_SPEC;
    pub type Psarb30 = crate::EnumBitfieldStruct<u8, Psarb30_SPEC>;
    impl Psarb30 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb31_SPEC;
    pub type Psarb31 = crate::EnumBitfieldStruct<u8, Psarb31_SPEC>;
    impl Psarb31 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psarc_SPEC;
impl crate::sealed::RegSpec for Psarc_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Security Attribution Register C"]
pub type Psarc = crate::RegValueT<Psarc_SPEC>;

impl Psarc {
    #[doc = "CAC and the MSTPCRC.MSTPC0 bit security attribution"]
    #[inline(always)]
    pub fn psarc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        psarc::Psarc0,
        psarc::Psarc0,
        Psarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            psarc::Psarc0,
            psarc::Psarc0,
            Psarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CRC and the MSTPCRC.MSTPC1 bit security attribution"]
    #[inline(always)]
    pub fn psarc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        psarc::Psarc1,
        psarc::Psarc1,
        Psarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            psarc::Psarc1,
            psarc::Psarc1,
            Psarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DOC and the MSTPCRC.MSTPC13 bit security attribution"]
    #[inline(always)]
    pub fn psarc13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        psarc::Psarc13,
        psarc::Psarc13,
        Psarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            psarc::Psarc13,
            psarc::Psarc13,
            Psarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCE9 and the MSTPCRC.MSTPC31 bit security attribution"]
    #[inline(always)]
    pub fn psarc31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        psarc::Psarc31,
        psarc::Psarc31,
        Psarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            psarc::Psarc31,
            psarc::Psarc31,
            Psarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Psarc {
    #[inline(always)]
    fn default() -> Psarc {
        <crate::RegValueT<Psarc_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod psarc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarc0_SPEC;
    pub type Psarc0 = crate::EnumBitfieldStruct<u8, Psarc0_SPEC>;
    impl Psarc0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarc1_SPEC;
    pub type Psarc1 = crate::EnumBitfieldStruct<u8, Psarc1_SPEC>;
    impl Psarc1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarc13_SPEC;
    pub type Psarc13 = crate::EnumBitfieldStruct<u8, Psarc13_SPEC>;
    impl Psarc13 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarc31_SPEC;
    pub type Psarc31 = crate::EnumBitfieldStruct<u8, Psarc31_SPEC>;
    impl Psarc31 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psard_SPEC;
impl crate::sealed::RegSpec for Psard_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Security Attribution Register D"]
pub type Psard = crate::RegValueT<Psard_SPEC>;

impl Psard {
    #[doc = "AGT3 and the MSTPCRD.MSTPD0 bit security attribution"]
    #[inline(always)]
    pub fn psard0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        psard::Psard0,
        psard::Psard0,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            psard::Psard0,
            psard::Psard0,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT2 and the MSTPCRD.MSTPD1 bit security attribution"]
    #[inline(always)]
    pub fn psard1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        psard::Psard1,
        psard::Psard1,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            psard::Psard1,
            psard::Psard1,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT1 and the MSTPCRD.MSTPD2 bit security attribution"]
    #[inline(always)]
    pub fn psard2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        psard::Psard2,
        psard::Psard2,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            psard::Psard2,
            psard::Psard2,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT0 and the MSTPCRD.MSTPD3 bit security attribution"]
    #[inline(always)]
    pub fn psard3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        psard::Psard3,
        psard::Psard3,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            psard::Psard3,
            psard::Psard3,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "POEG Group D and the MSTPCRD.MSTPD11 bit security attribution"]
    #[inline(always)]
    pub fn psard11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        psard::Psard11,
        psard::Psard11,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            psard::Psard11,
            psard::Psard11,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "POEG Group C and the MSTPCRD.MSTPD12 bit security attribution"]
    #[inline(always)]
    pub fn psard12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        psard::Psard12,
        psard::Psard12,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            psard::Psard12,
            psard::Psard12,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "POEG Group B and the MSTPCRD.MSTPD13 bit security attribution"]
    #[inline(always)]
    pub fn psard13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        psard::Psard13,
        psard::Psard13,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            psard::Psard13,
            psard::Psard13,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "POEG Group A and the MSTPCRD.MSTPD14 bit security attribution"]
    #[inline(always)]
    pub fn psard14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        psard::Psard14,
        psard::Psard14,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            psard::Psard14,
            psard::Psard14,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ADC120 and the MSTPCRD.MSTPD16 bit security attribution"]
    #[inline(always)]
    pub fn psard16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        psard::Psard16,
        psard::Psard16,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            psard::Psard16,
            psard::Psard16,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DAC12 and the MSTPCRD.MSTPD20 bit security attribution"]
    #[inline(always)]
    pub fn psard20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        psard::Psard20,
        psard::Psard20,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            psard::Psard20,
            psard::Psard20,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Psard {
    #[inline(always)]
    fn default() -> Psard {
        <crate::RegValueT<Psard_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod psard {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard0_SPEC;
    pub type Psard0 = crate::EnumBitfieldStruct<u8, Psard0_SPEC>;
    impl Psard0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard1_SPEC;
    pub type Psard1 = crate::EnumBitfieldStruct<u8, Psard1_SPEC>;
    impl Psard1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard2_SPEC;
    pub type Psard2 = crate::EnumBitfieldStruct<u8, Psard2_SPEC>;
    impl Psard2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard3_SPEC;
    pub type Psard3 = crate::EnumBitfieldStruct<u8, Psard3_SPEC>;
    impl Psard3 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard11_SPEC;
    pub type Psard11 = crate::EnumBitfieldStruct<u8, Psard11_SPEC>;
    impl Psard11 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard12_SPEC;
    pub type Psard12 = crate::EnumBitfieldStruct<u8, Psard12_SPEC>;
    impl Psard12 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard13_SPEC;
    pub type Psard13 = crate::EnumBitfieldStruct<u8, Psard13_SPEC>;
    impl Psard13 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard14_SPEC;
    pub type Psard14 = crate::EnumBitfieldStruct<u8, Psard14_SPEC>;
    impl Psard14 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard16_SPEC;
    pub type Psard16 = crate::EnumBitfieldStruct<u8, Psard16_SPEC>;
    impl Psard16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard20_SPEC;
    pub type Psard20 = crate::EnumBitfieldStruct<u8, Psard20_SPEC>;
    impl Psard20 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psare_SPEC;
impl crate::sealed::RegSpec for Psare_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Security Attribution Register E"]
pub type Psare = crate::RegValueT<Psare_SPEC>;

impl Psare {
    #[doc = "WDT security attribution"]
    #[inline(always)]
    pub fn psare0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        psare::Psare0,
        psare::Psare0,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            psare::Psare0,
            psare::Psare0,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IWDT security attribution"]
    #[inline(always)]
    pub fn psare1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        psare::Psare1,
        psare::Psare1,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            psare::Psare1,
            psare::Psare1,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC security attribution"]
    #[inline(always)]
    pub fn psare2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        psare::Psare2,
        psare::Psare2,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            psare::Psare2,
            psare::Psare2,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT5 and the MSTPCRE.MSTPE14 bit security attribution"]
    #[inline(always)]
    pub fn psare14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        psare::Psare14,
        psare::Psare14,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            psare::Psare14,
            psare::Psare14,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT4 and the MSTPCRE.MSTPE15 bit security attribution"]
    #[inline(always)]
    pub fn psare15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        psare::Psare15,
        psare::Psare15,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            psare::Psare15,
            psare::Psare15,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GPT5 and the MSTPCRE.MSTPE26 bit security attribution"]
    #[inline(always)]
    pub fn psare26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        psare::Psare26,
        psare::Psare26,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            psare::Psare26,
            psare::Psare26,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GPT4 and the MSTPCRE.MSTPE27 bit security attribution"]
    #[inline(always)]
    pub fn psare27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        psare::Psare27,
        psare::Psare27,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            psare::Psare27,
            psare::Psare27,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GPT2 and the MSTPCRE.MSTPE29 bit security attribution"]
    #[inline(always)]
    pub fn psare29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        psare::Psare29,
        psare::Psare29,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            psare::Psare29,
            psare::Psare29,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GPT1 and the MSTPCRE.MSTPE30 bit security attribution"]
    #[inline(always)]
    pub fn psare30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        psare::Psare30,
        psare::Psare30,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            psare::Psare30,
            psare::Psare30,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Psare {
    #[inline(always)]
    fn default() -> Psare {
        <crate::RegValueT<Psare_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod psare {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare0_SPEC;
    pub type Psare0 = crate::EnumBitfieldStruct<u8, Psare0_SPEC>;
    impl Psare0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare1_SPEC;
    pub type Psare1 = crate::EnumBitfieldStruct<u8, Psare1_SPEC>;
    impl Psare1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare2_SPEC;
    pub type Psare2 = crate::EnumBitfieldStruct<u8, Psare2_SPEC>;
    impl Psare2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare14_SPEC;
    pub type Psare14 = crate::EnumBitfieldStruct<u8, Psare14_SPEC>;
    impl Psare14 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare15_SPEC;
    pub type Psare15 = crate::EnumBitfieldStruct<u8, Psare15_SPEC>;
    impl Psare15 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare26_SPEC;
    pub type Psare26 = crate::EnumBitfieldStruct<u8, Psare26_SPEC>;
    impl Psare26 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare27_SPEC;
    pub type Psare27 = crate::EnumBitfieldStruct<u8, Psare27_SPEC>;
    impl Psare27 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare29_SPEC;
    pub type Psare29 = crate::EnumBitfieldStruct<u8, Psare29_SPEC>;
    impl Psare29 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare30_SPEC;
    pub type Psare30 = crate::EnumBitfieldStruct<u8, Psare30_SPEC>;
    impl Psare30 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mssar_SPEC;
impl crate::sealed::RegSpec for Mssar_SPEC {
    type DataType = u32;
}

#[doc = "Module Stop Security Attribution Register"]
pub type Mssar = crate::RegValueT<Mssar_SPEC>;

impl Mssar {
    #[doc = "The MSTPCRC.MSTPC14 bit security attribution"]
    #[inline(always)]
    pub fn mssar0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mssar::Mssar0,
        mssar::Mssar0,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mssar::Mssar0,
            mssar::Mssar0,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The MSTPCRA.MSTPA22 bit security attribution"]
    #[inline(always)]
    pub fn mssar1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mssar::Mssar1,
        mssar::Mssar1,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mssar::Mssar1,
            mssar::Mssar1,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The MSTPCRA.MSTPA7 bit security attribution"]
    #[inline(always)]
    pub fn mssar2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mssar::Mssar2,
        mssar::Mssar2,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mssar::Mssar2,
            mssar::Mssar2,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The MSTPCRA.MSTPA0 bit security attribution"]
    #[inline(always)]
    pub fn mssar3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mssar::Mssar3,
        mssar::Mssar3,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mssar::Mssar3,
            mssar::Mssar3,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mssar {
    #[inline(always)]
    fn default() -> Mssar {
        <crate::RegValueT<Mssar_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod mssar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar0_SPEC;
    pub type Mssar0 = crate::EnumBitfieldStruct<u8, Mssar0_SPEC>;
    impl Mssar0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar1_SPEC;
    pub type Mssar1 = crate::EnumBitfieldStruct<u8, Mssar1_SPEC>;
    impl Mssar1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar2_SPEC;
    pub type Mssar2 = crate::EnumBitfieldStruct<u8, Mssar2_SPEC>;
    impl Mssar2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar3_SPEC;
    pub type Mssar3 = crate::EnumBitfieldStruct<u8, Mssar3_SPEC>;
    impl Mssar3 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfsamona_SPEC;
impl crate::sealed::RegSpec for Cfsamona_SPEC {
    type DataType = u32;
}

#[doc = "Code Flash Security Attribution Monitor Register A"]
pub type Cfsamona = crate::RegValueT<Cfsamona_SPEC>;

impl Cfsamona {
    #[doc = "Code Flash Secure area 2"]
    #[inline(always)]
    pub fn cfs2(
        self,
    ) -> crate::common::RegisterField<15, 0x1ff, 1, 0, u16, u16, Cfsamona_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1ff,1,0,u16,u16,Cfsamona_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfsamona {
    #[inline(always)]
    fn default() -> Cfsamona {
        <crate::RegValueT<Cfsamona_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfsamonb_SPEC;
impl crate::sealed::RegSpec for Cfsamonb_SPEC {
    type DataType = u32;
}

#[doc = "Code Flash Security Attribution Monitor Register B"]
pub type Cfsamonb = crate::RegValueT<Cfsamonb_SPEC>;

impl Cfsamonb {
    #[doc = "Code Flash Secure area 1"]
    #[inline(always)]
    pub fn cfs1(
        self,
    ) -> crate::common::RegisterField<10, 0x3fff, 1, 0, u16, u16, Cfsamonb_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<10,0x3fff,1,0,u16,u16,Cfsamonb_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfsamonb {
    #[inline(always)]
    fn default() -> Cfsamonb {
        <crate::RegValueT<Cfsamonb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsamon_SPEC;
impl crate::sealed::RegSpec for Dfsamon_SPEC {
    type DataType = u32;
}

#[doc = "Data Flash Security Attribution Monitor Register"]
pub type Dfsamon = crate::RegValueT<Dfsamon_SPEC>;

impl Dfsamon {
    #[doc = "Data flash Secure area"]
    #[inline(always)]
    pub fn dfs(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Dfsamon_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Dfsamon_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dfsamon {
    #[inline(always)]
    fn default() -> Dfsamon {
        <crate::RegValueT<Dfsamon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssamona_SPEC;
impl crate::sealed::RegSpec for Ssamona_SPEC {
    type DataType = u32;
}

#[doc = "SRAM Security Attribution Monitor Register A"]
pub type Ssamona = crate::RegValueT<Ssamona_SPEC>;

impl Ssamona {
    #[doc = "SRAM Secure area 2"]
    #[inline(always)]
    pub fn ss2(
        self,
    ) -> crate::common::RegisterField<13, 0xff, 1, 0, u8, u8, Ssamona_SPEC, crate::common::R> {
        crate::common::RegisterField::<13,0xff,1,0,u8,u8,Ssamona_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ssamona {
    #[inline(always)]
    fn default() -> Ssamona {
        <crate::RegValueT<Ssamona_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssamonb_SPEC;
impl crate::sealed::RegSpec for Ssamonb_SPEC {
    type DataType = u32;
}

#[doc = "SRAM Security Attribution Monitor Register B"]
pub type Ssamonb = crate::RegValueT<Ssamonb_SPEC>;

impl Ssamonb {
    #[doc = "SRAM secure area 1"]
    #[inline(always)]
    pub fn ss1(
        self,
    ) -> crate::common::RegisterField<10, 0x7ff, 1, 0, u16, u16, Ssamonb_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<10,0x7ff,1,0,u16,u16,Ssamonb_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ssamonb {
    #[inline(always)]
    fn default() -> Ssamonb {
        <crate::RegValueT<Ssamonb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlmmon_SPEC;
impl crate::sealed::RegSpec for Dlmmon_SPEC {
    type DataType = u32;
}

#[doc = "Device Lifecycle Management State Monitor Register"]
pub type Dlmmon = crate::RegValueT<Dlmmon_SPEC>;

impl Dlmmon {
    #[doc = "Device Lifecycle Management State Monitor"]
    #[inline(always)]
    pub fn dlmmon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        dlmmon::Dlmmon,
        dlmmon::Dlmmon,
        Dlmmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            dlmmon::Dlmmon,
            dlmmon::Dlmmon,
            Dlmmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dlmmon {
    #[inline(always)]
    fn default() -> Dlmmon {
        <crate::RegValueT<Dlmmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dlmmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlmmon_SPEC;
    pub type Dlmmon = crate::EnumBitfieldStruct<u8, Dlmmon_SPEC>;
    impl Dlmmon {
        #[doc = "CM"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "SSD"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "NSECSD"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "DPL"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "LCK_DBG"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "LCK_BOOT"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "RMA_REQ"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "RMA_ACK"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "Reserved"]
        pub const OTHERS: Self = Self::new(0);
    }
}
