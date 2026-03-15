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
// Generated from SVD 1.1, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:00:31 +0000

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
                self._svd2pac_as_ptr().add(0usize),
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
                self._svd2pac_as_ptr().add(1usize),
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
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Operational amplifier control register"]
    #[inline(always)]
    pub const fn ampc(&self) -> &'static crate::common::Reg<self::Ampc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ampc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "Operational amplifier monitor register"]
    #[inline(always)]
    pub const fn ampmon(&self) -> &'static crate::common::Reg<self::Ampmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ampmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Operational Amplifier 0 Output Select Register"]
    #[inline(always)]
    pub const fn amp0os(
        &self,
    ) -> &'static crate::common::Reg<self::Amp0Os_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp0Os_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Operational Amplifier 0 Minus Input Select Register"]
    #[inline(always)]
    pub const fn amp0ms(
        &self,
    ) -> &'static crate::common::Reg<self::Amp0Ms_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp0Ms_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[doc = "Operational Amplifier 0 Plus Input Select Register"]
    #[inline(always)]
    pub const fn amp0ps(
        &self,
    ) -> &'static crate::common::Reg<self::Amp0Ps_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp0Ps_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Operational Amplifier 1 Minus Input Select Register"]
    #[inline(always)]
    pub const fn amp1ms(
        &self,
    ) -> &'static crate::common::Reg<self::Amp1Ms_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp1Ms_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "Operational Amplifier 1 Plus Input Select Register"]
    #[inline(always)]
    pub const fn amp1ps(
        &self,
    ) -> &'static crate::common::Reg<self::Amp1Ps_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp1Ps_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[doc = "Operational Amplifier 2 Minus Input Select Register"]
    #[inline(always)]
    pub const fn amp2ms(
        &self,
    ) -> &'static crate::common::Reg<self::Amp2Ms_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp2Ms_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[doc = "Operational Amplifier 2 Plus Input Select Register"]
    #[inline(always)]
    pub const fn amp2ps(
        &self,
    ) -> &'static crate::common::Reg<self::Amp2Ps_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp2Ps_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Operational Amplifier Switch Charge Pump Control Register"]
    #[inline(always)]
    pub const fn ampcpc(
        &self,
    ) -> &'static crate::common::Reg<self::Ampcpc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ampcpc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "Operational Amplifier User Offset Trimming Enable Register"]
    #[inline(always)]
    pub const fn ampuote(
        &self,
    ) -> &'static crate::common::Reg<self::Ampuote_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ampuote_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(23usize),
            )
        }
    }

    #[doc = "Operational Amplifier 0 Offset Trimming Pch Register"]
    #[inline(always)]
    pub const fn amp0otp(
        &self,
    ) -> &'static crate::common::Reg<self::Amp0Otp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp0Otp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Operational Amplifier 0 Offset Trimming Nch Register"]
    #[inline(always)]
    pub const fn amp0otn(
        &self,
    ) -> &'static crate::common::Reg<self::Amp0Otn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp0Otn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(25usize),
            )
        }
    }

    #[doc = "Operational Amplifier 1 Offset Trimming Pch Register"]
    #[inline(always)]
    pub const fn amp1otp(
        &self,
    ) -> &'static crate::common::Reg<self::Amp1Otp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp1Otp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[doc = "Operational Amplifier 1 Offset Trimming Nch Register"]
    #[inline(always)]
    pub const fn amp1otn(
        &self,
    ) -> &'static crate::common::Reg<self::Amp1Otn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp1Otn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(27usize),
            )
        }
    }

    #[doc = "Operational Amplifier 2 Offset Trimming Pch Register"]
    #[inline(always)]
    pub const fn amp2otp(
        &self,
    ) -> &'static crate::common::Reg<self::Amp2Otp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp2Otp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Operational Amplifier 2 Offset Trimming Nch Register"]
    #[inline(always)]
    pub const fn amp2otn(
        &self,
    ) -> &'static crate::common::Reg<self::Amp2Otn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp2Otn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(29usize),
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
    #[doc = "OPAMP Operation mode selection"]
    #[inline(always)]
    pub fn ampsp(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        ampmc::Ampsp,
        ampmc::Ampsp,
        Ampmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            ampmc::Ampsp,
            ampmc::Ampsp,
            Ampmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Ampmc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Ampmc_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Low-power mode (Low-speed)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Low-power mode (Low-speed)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Middle-speed mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "High-speed mode"]
        pub const _11: Self = Self::new(3);
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
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Amptrm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Amptrm_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "OPAMP function activation/stop trigger control"]
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

    #[doc = "OPAMP function activation/stop trigger control"]
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

    #[doc = "OPAMP function activation/stop trigger control"]
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

    #[doc = "OPAMP function activation/stop trigger control"]
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

    #[doc = "OPAMP function activation/stop trigger control"]
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

    #[doc = "OPAMP function activation/stop trigger control"]
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
    #[doc = "Activation Trigger SelectionNote: Do not change the value of the AMPTRS register after setting the AMPTRM register."]
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
        #[doc = "OPAMPn: OPAMP activation trigger n (n = 0 to 2)"]
        pub const _00: Self = Self::new(0);

        #[doc = "OPAMPn: OPAMP activation trigger 0 (n = 0, 1), OPAMP2: OPAMP activation trigger 1"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "OPAMPn: OPAMP activation trigger 0 (n = 0 to 2)."]
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
    #[doc = "Reference Current Circuit Operation Control"]
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

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, Ampc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,Ampc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Operation control of operational amplifier 2"]
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

    #[doc = "Operation control of operational amplifier 1"]
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

    #[doc = "Operation control of operational amplifier 0"]
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
        #[doc = "Reference current circuit is stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operation of reference current circuit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampe2_SPEC;
    pub type Ampe2 = crate::EnumBitfieldStruct<u8, Ampe2_SPEC>;
    impl Ampe2 {
        #[doc = "OPAMP is stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "OPAMP is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampe1_SPEC;
    pub type Ampe1 = crate::EnumBitfieldStruct<u8, Ampe1_SPEC>;
    impl Ampe1 {
        #[doc = "OPAMP is stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "OPAMP is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampe0_SPEC;
    pub type Ampe0 = crate::EnumBitfieldStruct<u8, Ampe0_SPEC>;
    impl Ampe0 {
        #[doc = "OPAMP is stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "OPAMP is enabled."]
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
    #[doc = "These bits are read as 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Ampmon_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Ampmon_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "OPAMP2 Status"]
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

    #[doc = "OPAMP1 Status"]
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

    #[doc = "OPAMP0 Status"]
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
    pub struct Ampmon2_SPEC;
    pub type Ampmon2 = crate::EnumBitfieldStruct<u8, Ampmon2_SPEC>;
    impl Ampmon2 {
        #[doc = "OPAMP is stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "OPAMP is operating"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampmon1_SPEC;
    pub type Ampmon1 = crate::EnumBitfieldStruct<u8, Ampmon1_SPEC>;
    impl Ampmon1 {
        #[doc = "OPAMP is stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "OPAMP is operating"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampmon0_SPEC;
    pub type Ampmon0 = crate::EnumBitfieldStruct<u8, Ampmon0_SPEC>;
    impl Ampmon0 {
        #[doc = "OPAMP is stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "OPAMP is operating"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp0Os_SPEC;
impl crate::sealed::RegSpec for Amp0Os_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier 0 Output Select Register"]
pub type Amp0Os = crate::RegValueT<Amp0Os_SPEC>;

impl Amp0Os {
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Amp0Os_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Amp0Os_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AMP2+ pin select"]
    #[inline(always)]
    pub fn ampos3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        amp0os::Ampos3,
        amp0os::Ampos3,
        Amp0Os_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            amp0os::Ampos3,
            amp0os::Ampos3,
            Amp0Os_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP2- pin select"]
    #[inline(always)]
    pub fn ampos2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        amp0os::Ampos2,
        amp0os::Ampos2,
        Amp0Os_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            amp0os::Ampos2,
            amp0os::Ampos2,
            Amp0Os_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP1+ pin select"]
    #[inline(always)]
    pub fn ampos1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        amp0os::Ampos1,
        amp0os::Ampos1,
        Amp0Os_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            amp0os::Ampos1,
            amp0os::Ampos1,
            Amp0Os_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP1- pin select"]
    #[inline(always)]
    pub fn ampos0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amp0os::Ampos0,
        amp0os::Ampos0,
        Amp0Os_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amp0os::Ampos0,
            amp0os::Ampos0,
            Amp0Os_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amp0Os {
    #[inline(always)]
    fn default() -> Amp0Os {
        <crate::RegValueT<Amp0Os_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amp0os {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampos3_SPEC;
    pub type Ampos3 = crate::EnumBitfieldStruct<u8, Ampos3_SPEC>;
    impl Ampos3 {
        #[doc = "AMP2+ pin is not connected to the OPAMP0 output"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP2+ pin is connected to the OPAMP0 output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampos2_SPEC;
    pub type Ampos2 = crate::EnumBitfieldStruct<u8, Ampos2_SPEC>;
    impl Ampos2 {
        #[doc = "AMP2- pin is not connected to the OPAMP0 output"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP2- pin is connected to the OPAMP0 output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampos1_SPEC;
    pub type Ampos1 = crate::EnumBitfieldStruct<u8, Ampos1_SPEC>;
    impl Ampos1 {
        #[doc = "AMP1+ pin is not connected to the OPAMP0 output"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP1+ pin is connected to the OPAMP0 output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampos0_SPEC;
    pub type Ampos0 = crate::EnumBitfieldStruct<u8, Ampos0_SPEC>;
    impl Ampos0 {
        #[doc = "AMP1- pin is not connected to the OPAMP0 output"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP1- pin is connected to the OPAMP0 output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp0Ms_SPEC;
impl crate::sealed::RegSpec for Amp0Ms_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier 0 Minus Input Select Register"]
pub type Amp0Ms = crate::RegValueT<Amp0Ms_SPEC>;

impl Amp0Ms {
    #[doc = "OPAMP0 output select"]
    #[inline(always)]
    pub fn ampms7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        amp0ms::Ampms7,
        amp0ms::Ampms7,
        Amp0Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            amp0ms::Ampms7,
            amp0ms::Ampms7,
            Amp0Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, Amp0Ms_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,Amp0Ms_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AMP2- pin select"]
    #[inline(always)]
    pub fn ampms4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        amp0ms::Ampms4,
        amp0ms::Ampms4,
        Amp0Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            amp0ms::Ampms4,
            amp0ms::Ampms4,
            Amp0Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP1+ pin select"]
    #[inline(always)]
    pub fn ampms3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        amp0ms::Ampms3,
        amp0ms::Ampms3,
        Amp0Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            amp0ms::Ampms3,
            amp0ms::Ampms3,
            Amp0Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP1- pin select"]
    #[inline(always)]
    pub fn ampms2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        amp0ms::Ampms2,
        amp0ms::Ampms2,
        Amp0Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            amp0ms::Ampms2,
            amp0ms::Ampms2,
            Amp0Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP0+ pin select"]
    #[inline(always)]
    pub fn ampms1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        amp0ms::Ampms1,
        amp0ms::Ampms1,
        Amp0Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            amp0ms::Ampms1,
            amp0ms::Ampms1,
            Amp0Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP0- pin select"]
    #[inline(always)]
    pub fn ampms0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amp0ms::Ampms0,
        amp0ms::Ampms0,
        Amp0Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amp0ms::Ampms0,
            amp0ms::Ampms0,
            Amp0Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amp0Ms {
    #[inline(always)]
    fn default() -> Amp0Ms {
        <crate::RegValueT<Amp0Ms_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amp0ms {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms7_SPEC;
    pub type Ampms7 = crate::EnumBitfieldStruct<u8, Ampms7_SPEC>;
    impl Ampms7 {
        #[doc = "OPAMP0 output is not connected to the AMP0 minus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "OPAMP0 output is connected to the AMP0 minus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms4_SPEC;
    pub type Ampms4 = crate::EnumBitfieldStruct<u8, Ampms4_SPEC>;
    impl Ampms4 {
        #[doc = "AMP2- pin is not connected to the AMP0 minus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP2- pin is connected to the AMP0 minus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms3_SPEC;
    pub type Ampms3 = crate::EnumBitfieldStruct<u8, Ampms3_SPEC>;
    impl Ampms3 {
        #[doc = "AMP1+ pin is not connected to the AMP0 minus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP1+ pin is connected to the AMP0 minus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms2_SPEC;
    pub type Ampms2 = crate::EnumBitfieldStruct<u8, Ampms2_SPEC>;
    impl Ampms2 {
        #[doc = "AMP1- pin is not connected to the AMP0 minus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP1- pin is connected to the AMP0 minus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms1_SPEC;
    pub type Ampms1 = crate::EnumBitfieldStruct<u8, Ampms1_SPEC>;
    impl Ampms1 {
        #[doc = "AMP0+ pin is not connected to the AMP0 minus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP0+ pin is connected to the AMP0 minus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms0_SPEC;
    pub type Ampms0 = crate::EnumBitfieldStruct<u8, Ampms0_SPEC>;
    impl Ampms0 {
        #[doc = "AMP0- pin is not connected to the AMP0 minus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP0- pin is connected to the AMP0 minus input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp0Ps_SPEC;
impl crate::sealed::RegSpec for Amp0Ps_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier 0 Plus Input Select Register"]
pub type Amp0Ps = crate::RegValueT<Amp0Ps_SPEC>;

impl Amp0Ps {
    #[doc = "DAC12 output select"]
    #[inline(always)]
    pub fn ampms7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        amp0ps::Ampms7,
        amp0ps::Ampms7,
        Amp0Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            amp0ps::Ampms7,
            amp0ps::Ampms7,
            Amp0Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Amp0Ps_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Amp0Ps_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AMP2+ pin select"]
    #[inline(always)]
    pub fn ampps3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        amp0ps::Ampps3,
        amp0ps::Ampps3,
        Amp0Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            amp0ps::Ampps3,
            amp0ps::Ampps3,
            Amp0Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP1+pin select"]
    #[inline(always)]
    pub fn ampps2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        amp0ps::Ampps2,
        amp0ps::Ampps2,
        Amp0Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            amp0ps::Ampps2,
            amp0ps::Ampps2,
            Amp0Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP1- pin select"]
    #[inline(always)]
    pub fn ampps1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        amp0ps::Ampps1,
        amp0ps::Ampps1,
        Amp0Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            amp0ps::Ampps1,
            amp0ps::Ampps1,
            Amp0Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP0+ pin select"]
    #[inline(always)]
    pub fn ampps0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amp0ps::Ampps0,
        amp0ps::Ampps0,
        Amp0Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amp0ps::Ampps0,
            amp0ps::Ampps0,
            Amp0Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amp0Ps {
    #[inline(always)]
    fn default() -> Amp0Ps {
        <crate::RegValueT<Amp0Ps_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amp0ps {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms7_SPEC;
    pub type Ampms7 = crate::EnumBitfieldStruct<u8, Ampms7_SPEC>;
    impl Ampms7 {
        #[doc = "DAC12 output is not connected to the AMP0 plus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "DAC12 output is connected to the AMP0 plus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps3_SPEC;
    pub type Ampps3 = crate::EnumBitfieldStruct<u8, Ampps3_SPEC>;
    impl Ampps3 {
        #[doc = "AMP2+ pin is not connected to the AMP0 plus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP2+ pin is connected to the AMP0 plus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps2_SPEC;
    pub type Ampps2 = crate::EnumBitfieldStruct<u8, Ampps2_SPEC>;
    impl Ampps2 {
        #[doc = "AMP1+ pin is not connected to the AMP0 plus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP1+ pin is connected to the AMP0 plus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps1_SPEC;
    pub type Ampps1 = crate::EnumBitfieldStruct<u8, Ampps1_SPEC>;
    impl Ampps1 {
        #[doc = "AMP1- pin is not connected to the AMP0 plus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP1- pin is connected to the AMP0 plus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps0_SPEC;
    pub type Ampps0 = crate::EnumBitfieldStruct<u8, Ampps0_SPEC>;
    impl Ampps0 {
        #[doc = "AMP0+ pin is not connected to the AMP0 plus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP0+ pin is connected to the AMP0 plus input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp1Ms_SPEC;
impl crate::sealed::RegSpec for Amp1Ms_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier 1 Minus Input Select Register"]
pub type Amp1Ms = crate::RegValueT<Amp1Ms_SPEC>;

impl Amp1Ms {
    #[doc = "OPAMP1 output select"]
    #[inline(always)]
    pub fn ampms7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        amp1ms::Ampms7,
        amp1ms::Ampms7,
        Amp1Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            amp1ms::Ampms7,
            amp1ms::Ampms7,
            Amp1Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, Amp1Ms_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,Amp1Ms_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AMP1- pin select"]
    #[inline(always)]
    pub fn ampms0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amp1ms::Ampms0,
        amp1ms::Ampms0,
        Amp1Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amp1ms::Ampms0,
            amp1ms::Ampms0,
            Amp1Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amp1Ms {
    #[inline(always)]
    fn default() -> Amp1Ms {
        <crate::RegValueT<Amp1Ms_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amp1ms {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms7_SPEC;
    pub type Ampms7 = crate::EnumBitfieldStruct<u8, Ampms7_SPEC>;
    impl Ampms7 {
        #[doc = "OPAMP1 output is not connected to the AMP1 minus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "OPAMP1 output is connected to the AMP1 minus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms0_SPEC;
    pub type Ampms0 = crate::EnumBitfieldStruct<u8, Ampms0_SPEC>;
    impl Ampms0 {
        #[doc = "AMP1- pin is not connected to the AMP1 minus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP1- pin is connected to the AMP1 minus input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp1Ps_SPEC;
impl crate::sealed::RegSpec for Amp1Ps_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier 1 Plus Input Select Register"]
pub type Amp1Ps = crate::RegValueT<Amp1Ps_SPEC>;

impl Amp1Ps {
    #[doc = "OPAMP2 output select"]
    #[inline(always)]
    pub fn ampms7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        amp1ps::Ampms7,
        amp1ps::Ampms7,
        Amp1Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            amp1ps::Ampms7,
            amp1ps::Ampms7,
            Amp1Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Amp1Ps_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Amp1Ps_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AMP2+ pin select"]
    #[inline(always)]
    pub fn ampps3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        amp1ps::Ampps3,
        amp1ps::Ampps3,
        Amp1Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            amp1ps::Ampps3,
            amp1ps::Ampps3,
            Amp1Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP2- pin select"]
    #[inline(always)]
    pub fn ampps2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        amp1ps::Ampps2,
        amp1ps::Ampps2,
        Amp1Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            amp1ps::Ampps2,
            amp1ps::Ampps2,
            Amp1Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP1+ pin select"]
    #[inline(always)]
    pub fn ampps1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        amp1ps::Ampps1,
        amp1ps::Ampps1,
        Amp1Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            amp1ps::Ampps1,
            amp1ps::Ampps1,
            Amp1Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP1- pin select"]
    #[inline(always)]
    pub fn ampps0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amp1ps::Ampps0,
        amp1ps::Ampps0,
        Amp1Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amp1ps::Ampps0,
            amp1ps::Ampps0,
            Amp1Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amp1Ps {
    #[inline(always)]
    fn default() -> Amp1Ps {
        <crate::RegValueT<Amp1Ps_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amp1ps {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms7_SPEC;
    pub type Ampms7 = crate::EnumBitfieldStruct<u8, Ampms7_SPEC>;
    impl Ampms7 {
        #[doc = "OPAMP2 output is not connected to the AMP2 minus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "OPAMP2 output is connected to the AMP2 minus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps3_SPEC;
    pub type Ampps3 = crate::EnumBitfieldStruct<u8, Ampps3_SPEC>;
    impl Ampps3 {
        #[doc = "AMP2+ pin is not connected to the AMP1 plus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP2+ pin is connected to the AMP1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps2_SPEC;
    pub type Ampps2 = crate::EnumBitfieldStruct<u8, Ampps2_SPEC>;
    impl Ampps2 {
        #[doc = "AMP2- pin is not connected to the AMP1 plus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP2- pin is connected to the AMP1 plus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps1_SPEC;
    pub type Ampps1 = crate::EnumBitfieldStruct<u8, Ampps1_SPEC>;
    impl Ampps1 {
        #[doc = "AMP1+ pin is not connected to the AMP1 plus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP1+ pin is connected to the AMP1 plus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps0_SPEC;
    pub type Ampps0 = crate::EnumBitfieldStruct<u8, Ampps0_SPEC>;
    impl Ampps0 {
        #[doc = "AMP1- pin is not connected to the AMP1 plus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP1- pin is connected to the AMP1 plus input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp2Ms_SPEC;
impl crate::sealed::RegSpec for Amp2Ms_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier 2 Minus Input Select Register"]
pub type Amp2Ms = crate::RegValueT<Amp2Ms_SPEC>;

impl Amp2Ms {
    #[doc = "OPAMP2 output select"]
    #[inline(always)]
    pub fn ampms7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        amp2ms::Ampms7,
        amp2ms::Ampms7,
        Amp2Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            amp2ms::Ampms7,
            amp2ms::Ampms7,
            Amp2Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, Amp2Ms_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,Amp2Ms_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AMP2- pin select"]
    #[inline(always)]
    pub fn ampms0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amp2ms::Ampms0,
        amp2ms::Ampms0,
        Amp2Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amp2ms::Ampms0,
            amp2ms::Ampms0,
            Amp2Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amp2Ms {
    #[inline(always)]
    fn default() -> Amp2Ms {
        <crate::RegValueT<Amp2Ms_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amp2ms {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms7_SPEC;
    pub type Ampms7 = crate::EnumBitfieldStruct<u8, Ampms7_SPEC>;
    impl Ampms7 {
        #[doc = "OPAMP2 output is not connected to the AMP2 minus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "OPAMP2 output is connected to the AMP2 minus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms0_SPEC;
    pub type Ampms0 = crate::EnumBitfieldStruct<u8, Ampms0_SPEC>;
    impl Ampms0 {
        #[doc = "AMP2- pin is not connected to the AMP2 minus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP2- pin is connected to the AMP2 minus input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp2Ps_SPEC;
impl crate::sealed::RegSpec for Amp2Ps_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier 2 Plus Input Select Register"]
pub type Amp2Ps = crate::RegValueT<Amp2Ps_SPEC>;

impl Amp2Ps {
    #[doc = "DAC8 channel 1output select"]
    #[inline(always)]
    pub fn ampps7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        amp2ps::Ampps7,
        amp2ps::Ampps7,
        Amp2Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            amp2ps::Ampps7,
            amp2ps::Ampps7,
            Amp2Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, Amp2Ps_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,Amp2Ps_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AMP2+ pin select"]
    #[inline(always)]
    pub fn ampps1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        amp2ps::Ampps1,
        amp2ps::Ampps1,
        Amp2Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            amp2ps::Ampps1,
            amp2ps::Ampps1,
            Amp2Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP2- pin select"]
    #[inline(always)]
    pub fn ampps0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amp2ps::Ampps0,
        amp2ps::Ampps0,
        Amp2Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amp2ps::Ampps0,
            amp2ps::Ampps0,
            Amp2Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amp2Ps {
    #[inline(always)]
    fn default() -> Amp2Ps {
        <crate::RegValueT<Amp2Ps_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amp2ps {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps7_SPEC;
    pub type Ampps7 = crate::EnumBitfieldStruct<u8, Ampps7_SPEC>;
    impl Ampps7 {
        #[doc = "DAC8 channel 1 output is not connected to the AMP2 plus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "DAC8 channel 1 output is connected to the AMP2 plus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps1_SPEC;
    pub type Ampps1 = crate::EnumBitfieldStruct<u8, Ampps1_SPEC>;
    impl Ampps1 {
        #[doc = "AMP2+ pin is not connected to the AMP2 plus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP2+ pin is connected to the AMP2 plus input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps0_SPEC;
    pub type Ampps0 = crate::EnumBitfieldStruct<u8, Ampps0_SPEC>;
    impl Ampps0 {
        #[doc = "AMP2- pin is not connected to the AMP2 plus input"]
        pub const _0: Self = Self::new(0);

        #[doc = "AMP2- pin is connected to the AMP2 plus input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ampcpc_SPEC;
impl crate::sealed::RegSpec for Ampcpc_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier Switch Charge Pump Control Register"]
pub type Ampcpc = crate::RegValueT<Ampcpc_SPEC>;

impl Ampcpc {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Ampcpc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Ampcpc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Charge Pump for AMP2 Enable"]
    #[inline(always)]
    pub fn pump2en(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ampcpc::Pump2En,
        ampcpc::Pump2En,
        Ampcpc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ampcpc::Pump2En,
            ampcpc::Pump2En,
            Ampcpc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Charge Pump for AMP1 Enable"]
    #[inline(always)]
    pub fn pump1en(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ampcpc::Pump1En,
        ampcpc::Pump1En,
        Ampcpc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ampcpc::Pump1En,
            ampcpc::Pump1En,
            Ampcpc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Charge Pump for AMP0 Enable"]
    #[inline(always)]
    pub fn pump0en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ampcpc::Pump0En,
        ampcpc::Pump0En,
        Ampcpc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ampcpc::Pump0En,
            ampcpc::Pump0En,
            Ampcpc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ampcpc {
    #[inline(always)]
    fn default() -> Ampcpc {
        <crate::RegValueT<Ampcpc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ampcpc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pump2En_SPEC;
    pub type Pump2En = crate::EnumBitfieldStruct<u8, Pump2En_SPEC>;
    impl Pump2En {
        #[doc = "Charge Pump for the AMP2 disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Charge Pump for the AMP2 enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pump1En_SPEC;
    pub type Pump1En = crate::EnumBitfieldStruct<u8, Pump1En_SPEC>;
    impl Pump1En {
        #[doc = "Charge Pump for the AMP1 disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Charge Pump for the AMP1 enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pump0En_SPEC;
    pub type Pump0En = crate::EnumBitfieldStruct<u8, Pump0En_SPEC>;
    impl Pump0En {
        #[doc = "Charge Pump for the AMP0 disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Charge Pump for the AMP0 enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ampuote_SPEC;
impl crate::sealed::RegSpec for Ampuote_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier User Offset Trimming Enable Register"]
pub type Ampuote = crate::RegValueT<Ampuote_SPEC>;

impl Ampuote {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Ampuote_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Ampuote_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AMP2OT write enable"]
    #[inline(always)]
    pub fn amp2te(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ampuote::Amp2Te,
        ampuote::Amp2Te,
        Ampuote_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ampuote::Amp2Te,
            ampuote::Amp2Te,
            Ampuote_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP1OT write enable"]
    #[inline(always)]
    pub fn amp1te(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ampuote::Amp1Te,
        ampuote::Amp1Te,
        Ampuote_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ampuote::Amp1Te,
            ampuote::Amp1Te,
            Ampuote_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AMP0OT write enable"]
    #[inline(always)]
    pub fn amp0te(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ampuote::Amp0Te,
        ampuote::Amp0Te,
        Ampuote_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ampuote::Amp0Te,
            ampuote::Amp0Te,
            Ampuote_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ampuote {
    #[inline(always)]
    fn default() -> Ampuote {
        <crate::RegValueT<Ampuote_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ampuote {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amp2Te_SPEC;
    pub type Amp2Te = crate::EnumBitfieldStruct<u8, Amp2Te_SPEC>;
    impl Amp2Te {
        #[doc = "Not possible to write to the AMP2OTP and AMP2OTN registers"]
        pub const _0: Self = Self::new(0);

        #[doc = "Possible to write to the AMP2OTP and AMP2OTN registers"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amp1Te_SPEC;
    pub type Amp1Te = crate::EnumBitfieldStruct<u8, Amp1Te_SPEC>;
    impl Amp1Te {
        #[doc = "Not possible to write to the AMP1OTP and AMP1OTN registers"]
        pub const _0: Self = Self::new(0);

        #[doc = "Possible to write to the AMP1OTP and AMP1OTN registers"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amp0Te_SPEC;
    pub type Amp0Te = crate::EnumBitfieldStruct<u8, Amp0Te_SPEC>;
    impl Amp0Te {
        #[doc = "Not possible to write to the AMP0OTP and AMP0OTN registers"]
        pub const _0: Self = Self::new(0);

        #[doc = "Possible to write to the AMP0OTP and AMP0OTN registers"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp0Otp_SPEC;
impl crate::sealed::RegSpec for Amp0Otp_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier 0 Offset Trimming Pch Register"]
pub type Amp0Otp = crate::RegValueT<Amp0Otp_SPEC>;

impl Amp0Otp {
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Amp0Otp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Amp0Otp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AMP0 input offset trimming Pch side"]
    #[inline(always)]
    pub fn trmp(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Amp0Otp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Amp0Otp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Amp0Otp {
    #[inline(always)]
    fn default() -> Amp0Otp {
        <crate::RegValueT<Amp0Otp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp0Otn_SPEC;
impl crate::sealed::RegSpec for Amp0Otn_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier 0 Offset Trimming Nch Register"]
pub type Amp0Otn = crate::RegValueT<Amp0Otn_SPEC>;

impl Amp0Otn {
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Amp0Otn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Amp0Otn_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AMP0 input offset trimming Nch side"]
    #[inline(always)]
    pub fn trmn(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Amp0Otn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Amp0Otn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Amp0Otn {
    #[inline(always)]
    fn default() -> Amp0Otn {
        <crate::RegValueT<Amp0Otn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp1Otp_SPEC;
impl crate::sealed::RegSpec for Amp1Otp_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier 1 Offset Trimming Pch Register"]
pub type Amp1Otp = crate::RegValueT<Amp1Otp_SPEC>;

impl Amp1Otp {
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Amp1Otp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Amp1Otp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AMP1 input offset trimming Pch side"]
    #[inline(always)]
    pub fn trmp(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Amp1Otp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Amp1Otp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Amp1Otp {
    #[inline(always)]
    fn default() -> Amp1Otp {
        <crate::RegValueT<Amp1Otp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp1Otn_SPEC;
impl crate::sealed::RegSpec for Amp1Otn_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier 1 Offset Trimming Nch Register"]
pub type Amp1Otn = crate::RegValueT<Amp1Otn_SPEC>;

impl Amp1Otn {
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Amp1Otn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Amp1Otn_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AMP1 input offset trimming Nch side"]
    #[inline(always)]
    pub fn trmn(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Amp1Otn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Amp1Otn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Amp1Otn {
    #[inline(always)]
    fn default() -> Amp1Otn {
        <crate::RegValueT<Amp1Otn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp2Otp_SPEC;
impl crate::sealed::RegSpec for Amp2Otp_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier 2 Offset Trimming Pch Register"]
pub type Amp2Otp = crate::RegValueT<Amp2Otp_SPEC>;

impl Amp2Otp {
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Amp2Otp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Amp2Otp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AMP2 input offset trimming Pch side"]
    #[inline(always)]
    pub fn trmp(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Amp2Otp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Amp2Otp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Amp2Otp {
    #[inline(always)]
    fn default() -> Amp2Otp {
        <crate::RegValueT<Amp2Otp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp2Otn_SPEC;
impl crate::sealed::RegSpec for Amp2Otn_SPEC {
    type DataType = u8;
}

#[doc = "Operational Amplifier 2 Offset Trimming Nch Register"]
pub type Amp2Otn = crate::RegValueT<Amp2Otn_SPEC>;

impl Amp2Otn {
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Amp2Otn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Amp2Otn_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AMP2 input offset trimming Nch side"]
    #[inline(always)]
    pub fn trmn(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Amp2Otn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Amp2Otn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Amp2Otn {
    #[inline(always)]
    fn default() -> Amp2Otn {
        <crate::RegValueT<Amp2Otn_SPEC> as RegisterValue<_>>::new(0)
    }
}
