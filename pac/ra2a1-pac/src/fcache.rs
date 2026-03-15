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
#[doc = r"Flash Cache"]
unsafe impl ::core::marker::Send for super::Fcache {}
unsafe impl ::core::marker::Sync for super::Fcache {}
impl super::Fcache {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Flash Cache Enable Register"]
    #[inline(always)]
    pub const fn fcachee(
        &self,
    ) -> &'static crate::common::Reg<self::Fcachee_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fcachee_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Flash Cache Invalidate Register"]
    #[inline(always)]
    pub const fn fcacheiv(
        &self,
    ) -> &'static crate::common::Reg<self::Fcacheiv_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fcacheiv_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcachee_SPEC;
impl crate::sealed::RegSpec for Fcachee_SPEC {
    type DataType = u16;
}

#[doc = "Flash Cache Enable Register"]
pub type Fcachee = crate::RegValueT<Fcachee_SPEC>;

impl Fcachee {
    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fff, 1, 0, u16, u16, Fcachee_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fff,1,0,u16,u16,Fcachee_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "FCACHE Enable"]
    #[inline(always)]
    pub fn fcacheen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fcachee::Fcacheen,
        fcachee::Fcacheen,
        Fcachee_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fcachee::Fcacheen,
            fcachee::Fcacheen,
            Fcachee_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fcachee {
    #[inline(always)]
    fn default() -> Fcachee {
        <crate::RegValueT<Fcachee_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fcachee {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fcacheen_SPEC;
    pub type Fcacheen = crate::EnumBitfieldStruct<u8, Fcacheen_SPEC>;
    impl Fcacheen {
        #[doc = "FCACHE is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "FCACHE is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcacheiv_SPEC;
impl crate::sealed::RegSpec for Fcacheiv_SPEC {
    type DataType = u16;
}

#[doc = "Flash Cache Invalidate Register"]
pub type Fcacheiv = crate::RegValueT<Fcacheiv_SPEC>;

impl Fcacheiv {
    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fff, 1, 0, u16, u16, Fcacheiv_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fff,1,0,u16,u16,Fcacheiv_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "FCACHE Invalidation"]
    #[inline(always)]
    pub fn fcacheiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fcacheiv::Fcacheiv,
        fcacheiv::Fcacheiv,
        Fcacheiv_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fcacheiv::Fcacheiv,
            fcacheiv::Fcacheiv,
            Fcacheiv_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fcacheiv {
    #[inline(always)]
    fn default() -> Fcacheiv {
        <crate::RegValueT<Fcacheiv_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fcacheiv {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fcacheiv_SPEC;
    pub type Fcacheiv = crate::EnumBitfieldStruct<u8, Fcacheiv_SPEC>;
    impl Fcacheiv {
        #[doc = "(Read)not in progress / (Write) no effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "(Read)in progress /(Write)  Starting Cache Invalidation"]
        pub const _1: Self = Self::new(1);
    }
}
