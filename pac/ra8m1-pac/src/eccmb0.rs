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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:57:20 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Error correction circuit for MBRAM0"]
unsafe impl ::core::marker::Send for super::Eccmb0 {}
unsafe impl ::core::marker::Sync for super::Eccmb0 {}
impl super::Eccmb0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "ECC control register"]
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

    #[doc = "ECC test mode control register"]
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

    #[doc = "ECC encode or decode data test register"]
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

    #[doc = "ECC error address register0"]
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

#[doc = "ECC control register"]
pub type Ec710Ctl = crate::RegValueT<Ec710Ctl_SPEC>;

impl Ec710Ctl {
    #[doc = "Flag bit of ECC error message"]
    #[inline(always)]
    pub fn ecemf(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ec710Ctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ec710Ctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Flag bit of ECC error detection and correction"]
    #[inline(always)]
    pub fn ecer1f(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ec710Ctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ec710Ctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Flag bit of 2bits ECC error detection"]
    #[inline(always)]
    pub fn ecer2f(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ec710Ctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ec710Ctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Control bit of interrupt at ECC 1bit error detection"]
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

    #[doc = "Control bit of interrupt at ECC 2bits error detection"]
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

    #[doc = "Permission bit of ECC 1bit error correction"]
    #[inline(always)]
    pub fn ec1ecp(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ec710Ctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ec710Ctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Flag bit to enable ECC error judgment"]
    #[inline(always)]
    pub fn ecervf(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ec710Ctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ec710Ctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Select bit of ECC function through mode"]
    #[inline(always)]
    pub fn ecthm(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ec710ctl::Ecthm,
        ec710ctl::Ecthm,
        Ec710Ctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ec710ctl::Ecthm,
            ec710ctl::Ecthm,
            Ec710Ctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Flag clear bit of accumulating ECC error detection and correction"]
    #[inline(always)]
    pub fn ecer1c(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ec710Ctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ec710Ctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Flag clear bit of 2bits ECC error detection"]
    #[inline(always)]
    pub fn ecer2c(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ec710Ctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Ec710Ctl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Flag bit of ECC overflow detection"]
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

    #[doc = "Access control bit0 to ECC mode select bit"]
    #[inline(always)]
    pub fn emca0(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ec710Ctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Ec710Ctl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Access control bit1 to ECC mode select bit"]
    #[inline(always)]
    pub fn emca1(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ec710Ctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Ec710Ctl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Flag bit of ECC Single bit Error Detection"]
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

    #[doc = "Flag bit of ECC Dual bit Error Detection"]
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

    #[doc = "These bits are read as 00000000000000. The write value should be 00000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<18, 0x3fff, 1, 0, u16, u16, Ec710Ctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3fff,1,0,u16,u16,Ec710Ctl_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Ec1Edic_SPEC;
    pub type Ec1Edic = crate::EnumBitfieldStruct<u8, Ec1Edic_SPEC>;
    impl Ec1Edic {
        #[doc = "Interrupt is not occurred at detecting 2bit error."]
        pub const _0: Self = Self::new(0);

        #[doc = "EC7TIE2 is outputted at detecting 1bit error."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ec2Edic_SPEC;
    pub type Ec2Edic = crate::EnumBitfieldStruct<u8, Ec2Edic_SPEC>;
    impl Ec2Edic {
        #[doc = "Interrupt is not occurred at detecting 2bits error."]
        pub const _0: Self = Self::new(0);

        #[doc = "EC7TIE2 is outputted at detecting 2bits error. (initial value)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecthm_SPEC;
    pub type Ecthm = crate::EnumBitfieldStruct<u8, Ecthm_SPEC>;
    impl Ecthm {
        #[doc = "Through mode disable (normal operation mode). Initial value."]
        pub const _0: Self = Self::new(0);

        #[doc = "Through mode enable."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecovff_SPEC;
    pub type Ecovff = crate::EnumBitfieldStruct<u8, Ecovff_SPEC>;
    impl Ecovff {
        #[doc = "Overflow is not occurred after reset or clearing ECER2F and ECER1F."]
        pub const _0: Self = Self::new(0);

        #[doc = "Error address register overflowed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecsedf0_SPEC;
    pub type Ecsedf0 = crate::EnumBitfieldStruct<u8, Ecsedf0_SPEC>;
    impl Ecsedf0 {
        #[doc = "no bit error"]
        pub const _0: Self = Self::new(0);

        #[doc = "1bits error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecdedf0_SPEC;
    pub type Ecdedf0 = crate::EnumBitfieldStruct<u8, Ecdedf0_SPEC>;
    impl Ecdedf0 {
        #[doc = "no bit error"]
        pub const _0: Self = Self::new(0);

        #[doc = "2bits error occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ec710Tmc_SPEC;
impl crate::sealed::RegSpec for Ec710Tmc_SPEC {
    type DataType = u16;
}

#[doc = "ECC test mode control register"]
pub type Ec710Tmc = crate::RegValueT<Ec710Tmc_SPEC>;

impl Ec710Tmc {
    #[doc = "Select bit of ECC decode input"]
    #[inline(always)]
    pub fn ecdcs(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ec710Tmc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ec710Tmc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable bit of ECC test mode control"]
    #[inline(always)]
    pub fn ectmce(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ec710Tmc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ec710Tmc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Ec710Tmc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Ec710Tmc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Access control bit1 and bit0 to ECC test mode bit"]
    #[inline(always)]
    pub fn etma0(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ec710Tmc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Ec710Tmc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Access control bit1 and bit0 to ECC test mode bit"]
    #[inline(always)]
    pub fn etma1(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ec710Tmc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Ec710Tmc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ec710Tmc {
    #[inline(always)]
    fn default() -> Ec710Tmc {
        <crate::RegValueT<Ec710Tmc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ec710Ted_SPEC;
impl crate::sealed::RegSpec for Ec710Ted_SPEC {
    type DataType = u32;
}

#[doc = "ECC encode or decode data test register"]
pub type Ec710Ted = crate::RegValueT<Ec710Ted_SPEC>;

impl Ec710Ted {
    #[doc = "At test mode, this register data is used as input data to encode circuit or decode circuit"]
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

#[doc = "ECC error address register0"]
pub type Ec710Ead0 = crate::RegValueT<Ec710Ead0_SPEC>;

impl Ec710Ead0 {
    #[doc = "This is read only register to hold the ECC error occurred address."]
    #[inline(always)]
    pub fn ecead(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, u32, Ec710Ead0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x7fffffff,
            1,
            0,
            u32,
            u32,
            Ec710Ead0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Ec710Ead0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Ec710Ead0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ec710Ead0 {
    #[inline(always)]
    fn default() -> Ec710Ead0 {
        <crate::RegValueT<Ec710Ead0_SPEC> as RegisterValue<_>>::new(0)
    }
}
