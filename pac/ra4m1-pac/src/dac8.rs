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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:49:18 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"8-bit D/A converter"]
unsafe impl ::core::marker::Send for super::Dac8 {}
unsafe impl ::core::marker::Sync for super::Dac8 {}
impl super::Dac8 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "D/A Conversion Value Setting Register %s"]
    #[inline(always)]
    pub const fn dacs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Dacs_SPEC, crate::common::RW>,
        2,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub const fn dacs0(&self) -> &'static crate::common::Reg<self::Dacs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dacs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn dacs1(&self) -> &'static crate::common::Reg<self::Dacs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dacs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1usize),
            )
        }
    }

    #[doc = "D/A Converter Mode Register"]
    #[inline(always)]
    pub const fn dam(&self) -> &'static crate::common::Reg<self::Dam_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dam_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacs_SPEC;
impl crate::sealed::RegSpec for Dacs_SPEC {
    type DataType = u8;
}

#[doc = "D/A Conversion Value Setting Register %s"]
pub type Dacs = crate::RegValueT<Dacs_SPEC>;

impl Dacs {
    #[doc = "DACS D/A conversion store data \nnote: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\\[7:0\\] bits for the channel in use is\nprohibited."]
    #[inline(always)]
    pub fn dacs(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dacs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dacs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dacs {
    #[inline(always)]
    fn default() -> Dacs {
        <crate::RegValueT<Dacs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dam_SPEC;
impl crate::sealed::RegSpec for Dam_SPEC {
    type DataType = u8;
}

#[doc = "D/A Converter Mode Register"]
pub type Dam = crate::RegValueT<Dam_SPEC>;

impl Dam {
    #[doc = "D/A Operation Enable 1"]
    #[inline(always)]
    pub fn dace1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dam::Dace1,
        dam::Dace1,
        Dam_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dam::Dace1,
            dam::Dace1,
            Dam_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "D/A Operation Enable 0"]
    #[inline(always)]
    pub fn dace0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dam::Dace0,
        dam::Dace0,
        Dam_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dam::Dace0,
            dam::Dace0,
            Dam_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Dam_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Dam_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dam {
    #[inline(always)]
    fn default() -> Dam {
        <crate::RegValueT<Dam_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dam {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dace1_SPEC;
    pub type Dace1 = crate::EnumBitfieldStruct<u8, Dace1_SPEC>;
    impl Dace1 {
        #[doc = "D/A conversion disabled for channel 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "D/A conversion enabled for channel 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dace0_SPEC;
    pub type Dace0 = crate::EnumBitfieldStruct<u8, Dace0_SPEC>;
    impl Dace0 {
        #[doc = "D/A conversion disabled for channel 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "D/A conversion enabled for channel 0."]
        pub const _1: Self = Self::new(1);
    }
}
