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
#[doc = r"SYSTEM/FLASH"]
unsafe impl ::core::marker::Send for super::Fcache {}
unsafe impl ::core::marker::Sync for super::Fcache {}
impl super::Fcache {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Flash Security Attribution Register"]
    #[inline(always)]
    pub const fn fsar(&self) -> &'static crate::common::Reg<self::Fsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsar_SPEC;
impl crate::sealed::RegSpec for Fsar_SPEC {
    type DataType = u16;
}

#[doc = "Flash Security Attribution Register"]
pub type Fsar = crate::RegValueT<Fsar_SPEC>;

impl Fsar {
    #[doc = "FLWT Security Attribution"]
    #[inline(always)]
    pub fn flwtsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fsar::Flwtsa,
        fsar::Flwtsa,
        Fsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fsar::Flwtsa,
            fsar::Flwtsa,
            Fsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFBER Security Attribution"]
    #[inline(always)]
    pub fn pfbersa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fsar::Pfbersa,
        fsar::Pfbersa,
        Fsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fsar::Pfbersa,
            fsar::Pfbersa,
            Fsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FACI Command Issuing Security Attribution"]
    #[inline(always)]
    pub fn facicomisa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        fsar::Facicomisa,
        fsar::Facicomisa,
        Fsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            fsar::Facicomisa,
            fsar::Facicomisa,
            Fsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FACI Command Registers Security Attribution"]
    #[inline(always)]
    pub fn facicomrsa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        fsar::Facicomrsa,
        fsar::Facicomrsa,
        Fsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            fsar::Facicomrsa,
            fsar::Facicomrsa,
            Fsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DFLCTL Security Attribution"]
    #[inline(always)]
    pub fn dflctlsa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fsar::Dflctlsa,
        fsar::Dflctlsa,
        Fsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fsar::Dflctlsa,
            fsar::Dflctlsa,
            Fsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fsar {
    #[inline(always)]
    fn default() -> Fsar {
        <crate::RegValueT<Fsar_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod fsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flwtsa_SPEC;
    pub type Flwtsa = crate::EnumBitfieldStruct<u8, Flwtsa_SPEC>;
    impl Flwtsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfbersa_SPEC;
    pub type Pfbersa = crate::EnumBitfieldStruct<u8, Pfbersa_SPEC>;
    impl Pfbersa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Facicomisa_SPEC;
    pub type Facicomisa = crate::EnumBitfieldStruct<u8, Facicomisa_SPEC>;
    impl Facicomisa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Facicomrsa_SPEC;
    pub type Facicomrsa = crate::EnumBitfieldStruct<u8, Facicomrsa_SPEC>;
    impl Facicomrsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dflctlsa_SPEC;
    pub type Dflctlsa = crate::EnumBitfieldStruct<u8, Dflctlsa_SPEC>;
    impl Dflctlsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
