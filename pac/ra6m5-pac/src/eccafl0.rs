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
// Generated from SVD 1.30.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:13:42 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"ECCAFL"]
unsafe impl ::core::marker::Send for super::Eccafl0 {}
unsafe impl ::core::marker::Sync for super::Eccafl0 {}
impl super::Eccafl0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "ECC Control Register"]
    #[inline(always)]
    pub const fn ec710ctl(
        &self,
    ) -> &'static crate::common::Reg<self::Ec710Ctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ec710Ctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "ECC Test Mode Control Register"]
    #[inline(always)]
    pub const fn ec710tmc(
        &self,
    ) -> &'static crate::common::Reg<self::Ec710Tmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ec710Tmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "ECC Test Substitute Data Register"]
    #[inline(always)]
    pub const fn ec710ted(
        &self,
    ) -> &'static crate::common::Reg<self::Ec710Ted_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ec710Ted_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "ECC Error Address Register"]
    #[inline(always)]
    pub const fn ec710ead0(
        &self,
    ) -> &'static crate::common::Reg<self::Ec710Ead0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ec710Ead0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ec710Ctl_SPEC;
impl crate::sealed::RegSpec for Ec710Ctl_SPEC {
    type DataType = u32;
}

#[doc = "ECC Control Register"]
pub type Ec710Ctl = crate::RegValueT<Ec710Ctl_SPEC>;

impl Ec710Ctl {
    #[doc = "ECC Error Message Flag"]
    #[inline(always)]
    pub fn ecemf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ec710ctl::Ecemf,
        ec710ctl::Ecemf,
        Ec710Ctl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ec710ctl::Ecemf,
            ec710ctl::Ecemf,
            Ec710Ctl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "ECC Error Detection and Correction Flag"]
    #[inline(always)]
    pub fn ecer1f(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ec710ctl::Ecer1F,
        ec710ctl::Ecer1F,
        Ec710Ctl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ec710ctl::Ecer1F,
            ec710ctl::Ecer1F,
            Ec710Ctl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "2-bit ECC Error Detection Flag"]
    #[inline(always)]
    pub fn ecer2f(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ec710ctl::Ecer2F,
        ec710ctl::Ecer2F,
        Ec710Ctl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ec710ctl::Ecer2F,
            ec710ctl::Ecer2F,
            Ec710Ctl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "ECC 1-bit Error Detection Interrupt Control"]
    #[inline(always)]
    pub fn ec1edic(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ec710ctl::Ec1Edic,
        ec710ctl::Ec1Edic,
        Ec710Ctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ec710ctl::Ec1Edic,
            ec710ctl::Ec1Edic,
            Ec710Ctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC 2-bit Error Detection Interrupt Control"]
    #[inline(always)]
    pub fn ec2edic(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ec710ctl::Ec2Edic,
        ec710ctl::Ec2Edic,
        Ec710Ctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ec710ctl::Ec2Edic,
            ec710ctl::Ec2Edic,
            Ec710Ctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC 1-bit Error Correction Permission"]
    #[inline(always)]
    pub fn ec1ecp(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ec710ctl::Ec1Ecp,
        ec710ctl::Ec1Ecp,
        Ec710Ctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ec710ctl::Ec1Ecp,
            ec710ctl::Ec1Ecp,
            Ec710Ctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC Error Judgment Enable Flag"]
    #[inline(always)]
    pub fn ecervf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ec710ctl::Ecervf,
        ec710ctl::Ecervf,
        Ec710Ctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ec710ctl::Ecervf,
            ec710ctl::Ecervf,
            Ec710Ctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Accumulating ECC Error Detection and Correction Flag Clear"]
    #[inline(always)]
    pub fn ecer1c(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ec710ctl::Ecer1C,
        ec710ctl::Ecer1C,
        Ec710Ctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ec710ctl::Ecer1C,
            ec710ctl::Ecer1C,
            Ec710Ctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "2-bit ECC Error Detection Flag Clear"]
    #[inline(always)]
    pub fn ecer2c(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ec710ctl::Ecer2C,
        ec710ctl::Ecer2C,
        Ec710Ctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ec710ctl::Ecer2C,
            ec710ctl::Ecer2C,
            Ec710Ctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC Overflow Detection Flag"]
    #[inline(always)]
    pub fn ecovff(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ec710ctl::Ecovff,
        ec710ctl::Ecovff,
        Ec710Ctl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ec710ctl::Ecovff,
            ec710ctl::Ecovff,
            Ec710Ctl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Access Control to ECC Mode Select bit"]
    #[inline(always)]
    pub fn emca(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, Ec710Ctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,Ec710Ctl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ECC Single bit Error Address Detection Flag"]
    #[inline(always)]
    pub fn ecsedf0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ec710ctl::Ecsedf0,
        ec710ctl::Ecsedf0,
        Ec710Ctl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ec710ctl::Ecsedf0,
            ec710ctl::Ecsedf0,
            Ec710Ctl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "ECC Dual Bit Error Address Detection Flag"]
    #[inline(always)]
    pub fn ecdedf0(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ec710ctl::Ecdedf0,
        ec710ctl::Ecdedf0,
        Ec710Ctl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ec710ctl::Ecdedf0,
            ec710ctl::Ecdedf0,
            Ec710Ctl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ec710Ctl {
    #[inline(always)]
    fn default() -> Ec710Ctl {
        <crate::RegValueT<Ec710Ctl_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod ec710ctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecemf_SPEC;
    pub type Ecemf = crate::EnumBitfieldStruct<u8, Ecemf_SPEC>;
    impl Ecemf {
        #[doc = "There is no bit error in present RAM output data"]
        pub const _0: Self = Self::new(0);

        #[doc = "There is bit error in present RAM output data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecer1F_SPEC;
    pub type Ecer1F = crate::EnumBitfieldStruct<u8, Ecer1F_SPEC>;
    impl Ecer1F {
        #[doc = "After clearing this bit, 1-bit error correction has not occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "1-bit error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecer2F_SPEC;
    pub type Ecer2F = crate::EnumBitfieldStruct<u8, Ecer2F_SPEC>;
    impl Ecer2F {
        #[doc = "After clearing this bit, 2-bit error has not occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "2-bit error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ec1Edic_SPEC;
    pub type Ec1Edic = crate::EnumBitfieldStruct<u8, Ec1Edic_SPEC>;
    impl Ec1Edic {
        #[doc = "Disable 1-bit error detection interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable 1-bit error detection interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ec2Edic_SPEC;
    pub type Ec2Edic = crate::EnumBitfieldStruct<u8, Ec2Edic_SPEC>;
    impl Ec2Edic {
        #[doc = "Disable 2-bit error detection interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable 2-bit error detection interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ec1Ecp_SPEC;
    pub type Ec1Ecp = crate::EnumBitfieldStruct<u8, Ec1Ecp_SPEC>;
    impl Ec1Ecp {
        #[doc = "At 1-bit error detection, the error correction is executed"]
        pub const _0: Self = Self::new(0);

        #[doc = "At 1-bit error detection, the error correction is not executed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecervf_SPEC;
    pub type Ecervf = crate::EnumBitfieldStruct<u8, Ecervf_SPEC>;
    impl Ecervf {
        #[doc = "Error judgment disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Error judgment enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecer1C_SPEC;
    pub type Ecer1C = crate::EnumBitfieldStruct<u8, Ecer1C_SPEC>;
    impl Ecer1C {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear accumulating ECC error detection and correction flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecer2C_SPEC;
    pub type Ecer2C = crate::EnumBitfieldStruct<u8, Ecer2C_SPEC>;
    impl Ecer2C {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear 2-bit ECC error detection flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecovff_SPEC;
    pub type Ecovff = crate::EnumBitfieldStruct<u8, Ecovff_SPEC>;
    impl Ecovff {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "ECC overflow detection flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecsedf0_SPEC;
    pub type Ecsedf0 = crate::EnumBitfieldStruct<u8, Ecsedf0_SPEC>;
    impl Ecsedf0 {
        #[doc = "There is no bit error in EC710EAD0 after reset or clearing ECER1F bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Address captured in EC710EAD0 shows that 1-bit error occurred and captured"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecdedf0_SPEC;
    pub type Ecdedf0 = crate::EnumBitfieldStruct<u8, Ecdedf0_SPEC>;
    impl Ecdedf0 {
        #[doc = "There is no bit error in EC710EAD0 after reset or clearing ECER2F bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Address captured in EC710EAD0 shows that 2-bit error occurred and captured"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ec710Tmc_SPEC;
impl crate::sealed::RegSpec for Ec710Tmc_SPEC {
    type DataType = u16;
}

#[doc = "ECC Test Mode Control Register"]
pub type Ec710Tmc = crate::RegValueT<Ec710Tmc_SPEC>;

impl Ec710Tmc {
    #[doc = "ECC Decode Input Select"]
    #[inline(always)]
    pub fn ecdcs(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ec710tmc::Ecdcs,
        ec710tmc::Ecdcs,
        Ec710Tmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ec710tmc::Ecdcs,
            ec710tmc::Ecdcs,
            Ec710Tmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC Test Mode Control Enable"]
    #[inline(always)]
    pub fn ectmce(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ec710tmc::Ectmce,
        ec710tmc::Ectmce,
        Ec710Tmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ec710tmc::Ectmce,
            ec710tmc::Ectmce,
            Ec710Tmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC Test Mode Bit Access Control"]
    #[inline(always)]
    pub fn etma(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, Ec710Tmc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,Ec710Tmc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ec710Tmc {
    #[inline(always)]
    fn default() -> Ec710Tmc {
        <crate::RegValueT<Ec710Tmc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ec710tmc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecdcs_SPEC;
    pub type Ecdcs = crate::EnumBitfieldStruct<u8, Ecdcs_SPEC>;
    impl Ecdcs {
        #[doc = "Input lower 32 bits of RAM output data to data area of decode circuit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Input ECEDB31-0 in EC710TED register to data area of decode circuit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ectmce_SPEC;
    pub type Ectmce = crate::EnumBitfieldStruct<u8, Ectmce_SPEC>;
    impl Ectmce {
        #[doc = "The access to test mode register and bit is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "The access to test mode register and bit is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ec710Ted_SPEC;
impl crate::sealed::RegSpec for Ec710Ted_SPEC {
    type DataType = u32;
}

#[doc = "ECC Test Substitute Data Register"]
pub type Ec710Ted = crate::RegValueT<Ec710Ted_SPEC>;

impl Ec710Ted {
    #[doc = "ECC Test Substitute Data"]
    #[inline(always)]
    pub fn ecedb(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ec710Ted_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Ec710Ted_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ec710Ted {
    #[inline(always)]
    fn default() -> Ec710Ted {
        <crate::RegValueT<Ec710Ted_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ec710Ead0_SPEC;
impl crate::sealed::RegSpec for Ec710Ead0_SPEC {
    type DataType = u32;
}

#[doc = "ECC Error Address Register"]
pub type Ec710Ead0 = crate::RegValueT<Ec710Ead0_SPEC>;

impl Ec710Ead0 {
    #[doc = "ECC Error Address"]
    #[inline(always)]
    pub fn ecead(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Ec710Ead0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Ec710Ead0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ec710Ead0 {
    #[inline(always)]
    fn default() -> Ec710Ead0 {
        <crate::RegValueT<Ec710Ead0_SPEC> as RegisterValue<_>>::new(0)
    }
}
