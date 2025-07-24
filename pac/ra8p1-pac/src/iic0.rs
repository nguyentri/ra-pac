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
#[doc = r"Inter-Integrated Circuit 0"]
unsafe impl ::core::marker::Send for super::Iic0 {}
unsafe impl ::core::marker::Sync for super::Iic0 {}
impl super::Iic0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "I2C Bus Control Register 1"]
    #[inline(always)]
    pub const fn iccr1(&self) -> &'static crate::common::Reg<self::Iccr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iccr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iccr1_SPEC;
impl crate::sealed::RegSpec for Iccr1_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Control Register 1"]
pub type Iccr1 = crate::RegValueT<Iccr1_SPEC>;

impl Iccr1 {
    #[doc = "SDA Line Monitor"]
    #[inline(always)]
    pub fn sdai(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iccr1::Sdai,
        iccr1::Sdai,
        Iccr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iccr1::Sdai,
            iccr1::Sdai,
            Iccr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SCL Line Monitor"]
    #[inline(always)]
    pub fn scli(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iccr1::Scli,
        iccr1::Scli,
        Iccr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iccr1::Scli,
            iccr1::Scli,
            Iccr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SDA Output Control/Monitor"]
    #[inline(always)]
    pub fn sdao(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        iccr1::Sdao,
        iccr1::Sdao,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            iccr1::Sdao,
            iccr1::Sdao,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCL Output Control/Monitor"]
    #[inline(always)]
    pub fn sclo(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iccr1::Sclo,
        iccr1::Sclo,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iccr1::Sclo,
            iccr1::Sclo,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCLO/SDAO Write Protect"]
    #[inline(always)]
    pub fn sowp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        iccr1::Sowp,
        iccr1::Sowp,
        Iccr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            iccr1::Sowp,
            iccr1::Sowp,
            Iccr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Extra SCL Clock Cycle Output"]
    #[inline(always)]
    pub fn clo(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        iccr1::Clo,
        iccr1::Clo,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            iccr1::Clo,
            iccr1::Clo,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I2C Bus Interface Internal Reset"]
    #[inline(always)]
    pub fn iicrst(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        iccr1::Iicrst,
        iccr1::Iicrst,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            iccr1::Iicrst,
            iccr1::Iicrst,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I2C Bus Interface Enable"]
    #[inline(always)]
    pub fn ice(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        iccr1::Ice,
        iccr1::Ice,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            iccr1::Ice,
            iccr1::Ice,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iccr1 {
    #[inline(always)]
    fn default() -> Iccr1 {
        <crate::RegValueT<Iccr1_SPEC> as RegisterValue<_>>::new(31)
    }
}
pub mod iccr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdai_SPEC;
    pub type Sdai = crate::EnumBitfieldStruct<u8, Sdai_SPEC>;
    impl Sdai {
        #[doc = "SDAn line is low"]
        pub const _0: Self = Self::new(0);

        #[doc = "SDAn line is high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scli_SPEC;
    pub type Scli = crate::EnumBitfieldStruct<u8, Scli_SPEC>;
    impl Scli {
        #[doc = "SCLn line is low"]
        pub const _0: Self = Self::new(0);

        #[doc = "SCLn line is high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdao_SPEC;
    pub type Sdao = crate::EnumBitfieldStruct<u8, Sdao_SPEC>;
    impl Sdao {
        #[doc = "Read: IIC drives SDAn pin low Write: IIC drives SDAn pin low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read: IIC releases SDAn pin Write: IIC releases SDAn pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sclo_SPEC;
    pub type Sclo = crate::EnumBitfieldStruct<u8, Sclo_SPEC>;
    impl Sclo {
        #[doc = "Read: IIC drives SCLn pin low Write: IIC drives SCLn pin low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read: IIC releases SCLn pin Write: IIC releases SCLn pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sowp_SPEC;
    pub type Sowp = crate::EnumBitfieldStruct<u8, Sowp_SPEC>;
    impl Sowp {
        #[doc = "Write enable SCLO and SDAO bits"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write protect SCLO and SDAO bits"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clo_SPEC;
    pub type Clo = crate::EnumBitfieldStruct<u8, Clo_SPEC>;
    impl Clo {
        #[doc = "Do not output extra SCL clock cycle (default)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output extra SCL clock cycle"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicrst_SPEC;
    pub type Iicrst = crate::EnumBitfieldStruct<u8, Iicrst_SPEC>;
    impl Iicrst {
        #[doc = "Release IIC reset or internal reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initiate IIC reset or internal reset"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ice_SPEC;
    pub type Ice = crate::EnumBitfieldStruct<u8, Ice_SPEC>;
    impl Ice {
        #[doc = "Disable (SCLn and SDAn pins in inactive state)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable (SCLn and SDAn pins in active state)"]
        pub const _1: Self = Self::new(1);
    }
}
