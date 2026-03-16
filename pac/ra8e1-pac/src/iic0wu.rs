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
// Generated from SVD 1.00.01, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:39:07 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Inter-Integrated Circuit 0 Wake-up Unit"]
unsafe impl ::core::marker::Send for super::Iic0Wu {}
unsafe impl ::core::marker::Sync for super::Iic0Wu {}
impl super::Iic0Wu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "I2C Bus Wakeup Unit Register"]
    #[inline(always)]
    pub const fn icwur(&self) -> &'static crate::common::Reg<self::Icwur_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icwur_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "I2C Bus Wakeup Unit Register 2"]
    #[inline(always)]
    pub const fn icwur2(
        &self,
    ) -> &'static crate::common::Reg<self::Icwur2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icwur2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icwur_SPEC;
impl crate::sealed::RegSpec for Icwur_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Wakeup Unit Register"]
pub type Icwur = crate::RegValueT<Icwur_SPEC>;

impl Icwur {
    #[doc = "Wakeup Analog Filter Additional Selection"]
    #[inline(always)]
    pub fn wuafa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icwur::Wuafa,
        icwur::Wuafa,
        Icwur_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icwur::Wuafa,
            icwur::Wuafa,
            Icwur_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACK Bit for Wakeup Mode"]
    #[inline(always)]
    pub fn wuack(self) -> crate::common::RegisterFieldBool<4, 1, 0, Icwur_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Icwur_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wakeup Event Occurrence Flag"]
    #[inline(always)]
    pub fn wuf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icwur::Wuf,
        icwur::Wuf,
        Icwur_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icwur::Wuf,
            icwur::Wuf,
            Icwur_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Wakeup Interrupt Request Enable"]
    #[inline(always)]
    pub fn wuie(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icwur::Wuie,
        icwur::Wuie,
        Icwur_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icwur::Wuie,
            icwur::Wuie,
            Icwur_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Wakeup Function Enable"]
    #[inline(always)]
    pub fn wue(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icwur::Wue,
        icwur::Wue,
        Icwur_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icwur::Wue,
            icwur::Wue,
            Icwur_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icwur {
    #[inline(always)]
    fn default() -> Icwur {
        <crate::RegValueT<Icwur_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod icwur {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wuafa_SPEC;
    pub type Wuafa = crate::EnumBitfieldStruct<u8, Wuafa_SPEC>;
    impl Wuafa {
        #[doc = "Do not add the wakeup analog filter"]
        pub const _0: Self = Self::new(0);

        #[doc = "Add the wakeup analog filter"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wuf_SPEC;
    pub type Wuf = crate::EnumBitfieldStruct<u8, Wuf_SPEC>;
    impl Wuf {
        #[doc = "Slave address not matching during wakeup"]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave address matching during wakeup"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wuie_SPEC;
    pub type Wuie = crate::EnumBitfieldStruct<u8, Wuie_SPEC>;
    impl Wuie {
        #[doc = "Disable wakeup interrupt request (IIC0_WUI)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable wakeup interrupt request (IIC0_WUI)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wue_SPEC;
    pub type Wue = crate::EnumBitfieldStruct<u8, Wue_SPEC>;
    impl Wue {
        #[doc = "Disable wakeup function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable wakeup function"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icwur2_SPEC;
impl crate::sealed::RegSpec for Icwur2_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Wakeup Unit Register 2"]
pub type Icwur2 = crate::RegValueT<Icwur2_SPEC>;

impl Icwur2 {
    #[doc = "Wakeup Function Synchronous Enable"]
    #[inline(always)]
    pub fn wusen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icwur2::Wusen,
        icwur2::Wusen,
        Icwur2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icwur2::Wusen,
            icwur2::Wusen,
            Icwur2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Wakeup Function Asynchronous Operation Status Flag"]
    #[inline(always)]
    pub fn wuasyf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icwur2::Wuasyf,
        icwur2::Wuasyf,
        Icwur2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icwur2::Wuasyf,
            icwur2::Wuasyf,
            Icwur2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Wakeup Function Synchronous Operation Status Flag"]
    #[inline(always)]
    pub fn wusyf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icwur2::Wusyf,
        icwur2::Wusyf,
        Icwur2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icwur2::Wusyf,
            icwur2::Wusyf,
            Icwur2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icwur2 {
    #[inline(always)]
    fn default() -> Icwur2 {
        <crate::RegValueT<Icwur2_SPEC> as RegisterValue<_>>::new(253)
    }
}
pub mod icwur2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wusen_SPEC;
    pub type Wusen = crate::EnumBitfieldStruct<u8, Wusen_SPEC>;
    impl Wusen {
        #[doc = "IIC asynchronous circuit enable"]
        pub const _0: Self = Self::new(0);

        #[doc = "IIC synchronous circuit enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wuasyf_SPEC;
    pub type Wuasyf = crate::EnumBitfieldStruct<u8, Wuasyf_SPEC>;
    impl Wuasyf {
        #[doc = "IIC synchronous circuit enable condition"]
        pub const _0: Self = Self::new(0);

        #[doc = "IIC asynchronous circuit enable condition"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wusyf_SPEC;
    pub type Wusyf = crate::EnumBitfieldStruct<u8, Wusyf_SPEC>;
    impl Wusyf {
        #[doc = "IIC asynchronous circuit enable condition"]
        pub const _0: Self = Self::new(0);

        #[doc = "IIC synchronous circuit enable condition"]
        pub const _1: Self = Self::new(1);
    }
}
