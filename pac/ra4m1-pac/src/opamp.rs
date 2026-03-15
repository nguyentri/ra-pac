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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:06:30 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"OperationalAmplifier"]
unsafe impl ::core::marker::Send for super::Opamp {}
unsafe impl ::core::marker::Sync for super::Opamp {}
impl super::Opamp {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Operational amplifier mode control register"]
    #[inline(always)]
    pub const fn ampmc(&self) -> &'static crate::common::Reg<self::Ampmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ampmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Operational amplifier trigger mode control register"]
    #[inline(always)]
    pub const fn amptrm(
        &self,
    ) -> &'static crate::common::Reg<self::Amptrm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amptrm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }

    #[doc = "Operational Amplifier Activation Trigger Select Register"]
    #[inline(always)]
    pub const fn amptrs(
        &self,
    ) -> &'static crate::common::Reg<self::Amptrs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amptrs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "Operational amplifier control register"]
    #[inline(always)]
    pub const fn ampc(&self) -> &'static crate::common::Reg<self::Ampc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ampc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[doc = "Operational amplifier monitor register"]
    #[inline(always)]
    pub const fn ampmon(&self) -> &'static crate::common::Reg<self::Ampmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ampmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ampmc_SPEC;
impl crate::sealed::RegSpec for Ampmc_SPEC {
    type DataType = u8;
}

#[doc = "Operational amplifier mode control register"]
pub type Ampmc = crate::RegValueT<Ampmc_SPEC>;

impl Ampmc {
    #[doc = "Operation mode selection"]
    #[inline(always)]
    pub fn ampsp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ampmc::Ampsp,
        ampmc::Ampsp,
        Ampmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ampmc::Ampsp,
            ampmc::Ampsp,
            Ampmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Ampmc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Ampmc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ampmc::Amppc3,
        ampmc::Amppc3,
        Ampmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ampmc::Amppc3,
            ampmc::Amppc3,
            Ampmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ampmc::Amppc2,
        ampmc::Amppc2,
        Ampmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ampmc::Amppc2,
            ampmc::Amppc2,
            Ampmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ampmc::Amppc1,
        ampmc::Amppc1,
        Ampmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ampmc::Amppc1,
            ampmc::Amppc1,
            Ampmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operational amplifier precharge control status"]
    #[inline(always)]
    pub fn amppc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ampmc::Amppc0,
        ampmc::Amppc0,
        Ampmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ampmc::Amppc0,
            ampmc::Amppc0,
            Ampmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ampmc {
    #[inline(always)]
    fn default() -> Ampmc {
        <crate::RegValueT<Ampmc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ampmc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampsp_SPEC;
    pub type Ampsp = crate::EnumBitfieldStruct<u8, Ampsp_SPEC>;
    impl Ampsp {
        #[doc = "Low-power mode (low-speed)."]
        pub const _0: Self = Self::new(0);

        #[doc = "High-speed mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amppc3_SPEC;
    pub type Amppc3 = crate::EnumBitfieldStruct<u8, Amppc3_SPEC>;
    impl Amppc3 {
        #[doc = "Precharging is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Precharging is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amppc2_SPEC;
    pub type Amppc2 = crate::EnumBitfieldStruct<u8, Amppc2_SPEC>;
    impl Amppc2 {
        #[doc = "Precharging is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Precharging is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amppc1_SPEC;
    pub type Amppc1 = crate::EnumBitfieldStruct<u8, Amppc1_SPEC>;
    impl Amppc1 {
        #[doc = "Precharging is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Precharging is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amppc0_SPEC;
    pub type Amppc0 = crate::EnumBitfieldStruct<u8, Amppc0_SPEC>;
    impl Amppc0 {
        #[doc = "Precharging is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Precharging is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amptrm_SPEC;
impl crate::sealed::RegSpec for Amptrm_SPEC {
    type DataType = u8;
}

#[doc = "Operational amplifier trigger mode control register"]
pub type Amptrm = crate::RegValueT<Amptrm_SPEC>;

impl Amptrm {
    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm31(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        amptrm::Amptrm31,
        amptrm::Amptrm31,
        Amptrm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            amptrm::Amptrm31,
            amptrm::Amptrm31,
            Amptrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm30(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        amptrm::Amptrm30,
        amptrm::Amptrm30,
        Amptrm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            amptrm::Amptrm30,
            amptrm::Amptrm30,
            Amptrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        amptrm::Amptrm21,
        amptrm::Amptrm21,
        Amptrm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            amptrm::Amptrm21,
            amptrm::Amptrm21,
            Amptrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        amptrm::Amptrm20,
        amptrm::Amptrm20,
        Amptrm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            amptrm::Amptrm20,
            amptrm::Amptrm20,
            Amptrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm11(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        amptrm::Amptrm11,
        amptrm::Amptrm11,
        Amptrm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            amptrm::Amptrm11,
            amptrm::Amptrm11,
            Amptrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm10(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        amptrm::Amptrm10,
        amptrm::Amptrm10,
        Amptrm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            amptrm::Amptrm10,
            amptrm::Amptrm10,
            Amptrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        amptrm::Amptrm01,
        amptrm::Amptrm01,
        Amptrm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            amptrm::Amptrm01,
            amptrm::Amptrm01,
            Amptrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operational amplifier function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amptrm::Amptrm00,
        amptrm::Amptrm00,
        Amptrm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amptrm::Amptrm00,
            amptrm::Amptrm00,
            Amptrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amptrm {
    #[inline(always)]
    fn default() -> Amptrm {
        <crate::RegValueT<Amptrm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amptrm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrm31_SPEC;
    pub type Amptrm31 = crate::EnumBitfieldStruct<u8, Amptrm31_SPEC>;
    impl Amptrm31 {
        #[doc = "Software trigger mode(AMPTRM30=0)/An activation trigger mode(AMPTRM30=1)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited(AMPTRM30=0)/An activation and A/D trigger mode(AMPTRM30=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrm30_SPEC;
    pub type Amptrm30 = crate::EnumBitfieldStruct<u8, Amptrm30_SPEC>;
    impl Amptrm30 {
        #[doc = "Software trigger mode(AMPTRM31=0)/Setting prohibited(AMPTRM31=1)."]
        pub const _0: Self = Self::new(0);

        #[doc = "An activation trigger mode(AMPTRM31=0)/An activation and A/D trigger mode(AMPTRM31=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrm21_SPEC;
    pub type Amptrm21 = crate::EnumBitfieldStruct<u8, Amptrm21_SPEC>;
    impl Amptrm21 {
        #[doc = "Software trigger mode(AMPTRM20=0)/An activation trigger mode(AMPTRM20=1)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited(AMPTRM20=0)/An activation and A/D trigger mode(AMPTRM20=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrm20_SPEC;
    pub type Amptrm20 = crate::EnumBitfieldStruct<u8, Amptrm20_SPEC>;
    impl Amptrm20 {
        #[doc = "Software trigger mode(AMPTRM21=0)/Setting prohibited(AMPTRM21=1)."]
        pub const _0: Self = Self::new(0);

        #[doc = "An activation trigger mode(AMPTRM21=0)/An activation and A/D trigger mode(AMPTRM21=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrm11_SPEC;
    pub type Amptrm11 = crate::EnumBitfieldStruct<u8, Amptrm11_SPEC>;
    impl Amptrm11 {
        #[doc = "Software trigger mode(AMPTRM10=0)/An activation trigger mode(AMPTRM10=1)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited(AMPTRM10=0)/An activation and A/D trigger mode(AMPTRM10=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrm10_SPEC;
    pub type Amptrm10 = crate::EnumBitfieldStruct<u8, Amptrm10_SPEC>;
    impl Amptrm10 {
        #[doc = "Software trigger mode(AMPTRM11=0)/Setting prohibited(AMPTRM11=1)."]
        pub const _0: Self = Self::new(0);

        #[doc = "An activation trigger mode(AMPTRM11=0)/An activation and A/D trigger mode(AMPTRM11=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrm01_SPEC;
    pub type Amptrm01 = crate::EnumBitfieldStruct<u8, Amptrm01_SPEC>;
    impl Amptrm01 {
        #[doc = "Software trigger mode(AMPTRM00=0)/An activation trigger mode(AMPTRM00=1)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited(AMPTRM00=0)/An activation and A/D trigger mode(AMPTRM00=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrm00_SPEC;
    pub type Amptrm00 = crate::EnumBitfieldStruct<u8, Amptrm00_SPEC>;
    impl Amptrm00 {
        #[doc = "Software trigger mode(AMPTRM01=0)/Setting prohibited(AMPTRM01=1)."]
        pub const _0: Self = Self::new(0);

        #[doc = "An activation trigger mode(AMPTRM01=0)/An activation and A/D trigger mode(AMPTRM01=1)."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amptrs_SPEC;
impl crate::sealed::RegSpec for Amptrs_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier Activation Trigger Select Register"]
pub type Amptrs = crate::RegValueT<Amptrs_SPEC>;

impl Amptrs {
    #[doc = "ELC trigger selection Do not change the value of the AMPTRS register after setting the AMPTRM register."]
    #[inline(always)]
    pub fn amptrs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        amptrs::Amptrs,
        amptrs::Amptrs,
        Amptrs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            amptrs::Amptrs,
            amptrs::Amptrs,
            Amptrs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amptrs {
    #[inline(always)]
    fn default() -> Amptrs {
        <crate::RegValueT<Amptrs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amptrs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrs_SPEC;
    pub type Amptrs = crate::EnumBitfieldStruct<u8, Amptrs_SPEC>;
    impl Amptrs {
        #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 1.Operational amplifier 2: Operational amplifier An activation trigger 2.Operational amplifier 3: Operational amplifier An activation trigger 3"]
        pub const _00: Self = Self::new(0);

        #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 1.Operational amplifier 3: Operational amplifier An activation trigger 1"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "Operational amplifier 0: Operational amplifier An activation trigger 0.Operational amplifier 1: Operational amplifier An activation trigger 0.Operational amplifier 2: Operational amplifier An activation trigger 0.Operational amplifier 3: Operational amplifier An activation trigger 0"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ampc_SPEC;
impl crate::sealed::RegSpec for Ampc_SPEC {
    type DataType = u8;
}

#[doc = "Operational amplifier control register"]
pub type Ampc = crate::RegValueT<Ampc_SPEC>;

impl Ampc {
    #[doc = "Operation control of operational amplifier reference current circuit"]
    #[inline(always)]
    pub fn irefe(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ampc::Irefe,
        ampc::Irefe,
        Ampc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ampc::Irefe,
            ampc::Irefe,
            Ampc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Ampc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Ampc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Operation control of operational amplifier(UNIT3)"]
    #[inline(always)]
    pub fn ampe3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ampc::Ampe3,
        ampc::Ampe3,
        Ampc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ampc::Ampe3,
            ampc::Ampe3,
            Ampc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operation control of operational amplifier(UNIT2)"]
    #[inline(always)]
    pub fn ampe2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ampc::Ampe2,
        ampc::Ampe2,
        Ampc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ampc::Ampe2,
            ampc::Ampe2,
            Ampc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operation control of operational amplifier(UNIT1)"]
    #[inline(always)]
    pub fn ampe1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ampc::Ampe1,
        ampc::Ampe1,
        Ampc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ampc::Ampe1,
            ampc::Ampe1,
            Ampc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operation control of operational amplifier(UNIT0)"]
    #[inline(always)]
    pub fn ampe0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ampc::Ampe0,
        ampc::Ampe0,
        Ampc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ampc::Ampe0,
            ampc::Ampe0,
            Ampc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ampc {
    #[inline(always)]
    fn default() -> Ampc {
        <crate::RegValueT<Ampc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ampc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irefe_SPEC;
    pub type Irefe = crate::EnumBitfieldStruct<u8, Irefe_SPEC>;
    impl Irefe {
        #[doc = "Operational amplifier reference current circuit is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Operation of operational amplifier reference current circuit is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampe3_SPEC;
    pub type Ampe3 = crate::EnumBitfieldStruct<u8, Ampe3_SPEC>;
    impl Ampe3 {
        #[doc = "Operation amplifier is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampe2_SPEC;
    pub type Ampe2 = crate::EnumBitfieldStruct<u8, Ampe2_SPEC>;
    impl Ampe2 {
        #[doc = "Operation amplifier is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampe1_SPEC;
    pub type Ampe1 = crate::EnumBitfieldStruct<u8, Ampe1_SPEC>;
    impl Ampe1 {
        #[doc = "Operation amplifier is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampe0_SPEC;
    pub type Ampe0 = crate::EnumBitfieldStruct<u8, Ampe0_SPEC>;
    impl Ampe0 {
        #[doc = "Operation amplifier is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for AGT is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ampmon_SPEC;
impl crate::sealed::RegSpec for Ampmon_SPEC {
    type DataType = u8;
}

#[doc = "Operational amplifier monitor register"]
pub type Ampmon = crate::RegValueT<Ampmon_SPEC>;

impl Ampmon {
    #[doc = "These bits are read as 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Ampmon_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Ampmon_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Operational amplifier status(UNIT3)"]
    #[inline(always)]
    pub fn ampmon3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ampmon::Ampmon3,
        ampmon::Ampmon3,
        Ampmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ampmon::Ampmon3,
            ampmon::Ampmon3,
            Ampmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Operational amplifier status(UNIT2)"]
    #[inline(always)]
    pub fn ampmon2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ampmon::Ampmon2,
        ampmon::Ampmon2,
        Ampmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ampmon::Ampmon2,
            ampmon::Ampmon2,
            Ampmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Operational amplifier status(UNIT1)"]
    #[inline(always)]
    pub fn ampmon1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ampmon::Ampmon1,
        ampmon::Ampmon1,
        Ampmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ampmon::Ampmon1,
            ampmon::Ampmon1,
            Ampmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Operational amplifier status(UNIT0)"]
    #[inline(always)]
    pub fn ampmon0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ampmon::Ampmon0,
        ampmon::Ampmon0,
        Ampmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ampmon::Ampmon0,
            ampmon::Ampmon0,
            Ampmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ampmon {
    #[inline(always)]
    fn default() -> Ampmon {
        <crate::RegValueT<Ampmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ampmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampmon3_SPEC;
    pub type Ampmon3 = crate::EnumBitfieldStruct<u8, Ampmon3_SPEC>;
    impl Ampmon3 {
        #[doc = "Operational amplifier 3 is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Operational amplifier 3 is operating."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampmon2_SPEC;
    pub type Ampmon2 = crate::EnumBitfieldStruct<u8, Ampmon2_SPEC>;
    impl Ampmon2 {
        #[doc = "Operational amplifier 2 is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Operational amplifier 2 is operating."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampmon1_SPEC;
    pub type Ampmon1 = crate::EnumBitfieldStruct<u8, Ampmon1_SPEC>;
    impl Ampmon1 {
        #[doc = "Operational amplifier 1 is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Operational amplifier 1 is operating."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampmon0_SPEC;
    pub type Ampmon0 = crate::EnumBitfieldStruct<u8, Ampmon0_SPEC>;
    impl Ampmon0 {
        #[doc = "Operational amplifier 0 is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Operational amplifier 0 is operating."]
        pub const _1: Self = Self::new(1);
    }
}
