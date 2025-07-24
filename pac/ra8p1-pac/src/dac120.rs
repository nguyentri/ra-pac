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
#[doc = r"12-bit D/A converter"]
unsafe impl ::core::marker::Send for super::Dac120 {}
unsafe impl ::core::marker::Sync for super::Dac120 {}
impl super::Dac120 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "D/A Data Register"]
    #[inline(always)]
    pub const fn dadr(&self) -> &'static crate::common::Reg<self::Dadr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dadr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "D/A Control 0 Register"]
    #[inline(always)]
    pub const fn dacr0(&self) -> &'static crate::common::Reg<self::Dacr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dacr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "D/A Control 1 Register"]
    #[inline(always)]
    pub const fn dacr1(&self) -> &'static crate::common::Reg<self::Dacr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dacr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "D/A Control 2 Register"]
    #[inline(always)]
    pub const fn dacr2(&self) -> &'static crate::common::Reg<self::Dacr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dacr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dadr_SPEC;
impl crate::sealed::RegSpec for Dadr_SPEC {
    type DataType = u16;
}

#[doc = "D/A Data Register"]
pub type Dadr = crate::RegValueT<Dadr_SPEC>;

impl NoBitfieldReg<Dadr_SPEC> for Dadr {}
impl ::core::default::Default for Dadr {
    #[inline(always)]
    fn default() -> Dadr {
        <crate::RegValueT<Dadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacr0_SPEC;
impl crate::sealed::RegSpec for Dacr0_SPEC {
    type DataType = u32;
}

#[doc = "D/A Control 0 Register"]
pub type Dacr0 = crate::RegValueT<Dacr0_SPEC>;

impl Dacr0 {
    #[doc = "D/A Output Enable"]
    #[inline(always)]
    pub fn dacen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dacr0::Dacen,
        dacr0::Dacen,
        Dacr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dacr0::Dacen,
            dacr0::Dacen,
            Dacr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "D/A Enable R/W"]
    #[inline(always)]
    pub fn dae(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        dacr0::Dae,
        dacr0::Dae,
        Dacr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            dacr0::Dae,
            dacr0::Dae,
            Dacr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Output Disables"]
    #[inline(always)]
    pub fn daoutdis(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        dacr0::Daoutdis,
        dacr0::Daoutdis,
        Dacr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            dacr0::Daoutdis,
            dacr0::Daoutdis,
            Dacr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dacr0 {
    #[inline(always)]
    fn default() -> Dacr0 {
        <crate::RegValueT<Dacr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dacr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dacen_SPEC;
    pub type Dacen = crate::EnumBitfieldStruct<u8, Dacen_SPEC>;
    impl Dacen {
        #[doc = "Disable analog output of channel n (DAn)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable D/A conversion of channel n (DAn)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dae_SPEC;
    pub type Dae = crate::EnumBitfieldStruct<u8, Dae_SPEC>;
    impl Dae {
        #[doc = "Control D/A conversion of channels 0 and 1 individually"]
        pub const _0: Self = Self::new(0);

        #[doc = "Control D/A conversion of channels 0 and 1 collectively"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Daoutdis_SPEC;
    pub type Daoutdis = crate::EnumBitfieldStruct<u8, Daoutdis_SPEC>;
    impl Daoutdis {
        #[doc = "The output to pin is enabled and the output to comparator is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "The output to pin is disabled and the output to comparator is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacr1_SPEC;
impl crate::sealed::RegSpec for Dacr1_SPEC {
    type DataType = u32;
}

#[doc = "D/A Control 1 Register"]
pub type Dacr1 = crate::RegValueT<Dacr1_SPEC>;

impl Dacr1 {
    #[doc = "DADR Format Select R/W"]
    #[inline(always)]
    pub fn dpsel(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dacr1::Dpsel,
        dacr1::Dpsel,
        Dacr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dacr1::Dpsel,
            dacr1::Dpsel,
            Dacr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dacr1 {
    #[inline(always)]
    fn default() -> Dacr1 {
        <crate::RegValueT<Dacr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dacr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpsel_SPEC;
    pub type Dpsel = crate::EnumBitfieldStruct<u8, Dpsel_SPEC>;
    impl Dpsel {
        #[doc = "Right-justified format"]
        pub const _0: Self = Self::new(0);

        #[doc = "Left-justified format"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacr2_SPEC;
impl crate::sealed::RegSpec for Dacr2_SPEC {
    type DataType = u32;
}

#[doc = "D/A Control 2 Register"]
pub type Dacr2 = crate::RegValueT<Dacr2_SPEC>;

impl Dacr2 {
    #[doc = "DAC Operating Voltage Mode Selection"]
    #[inline(always)]
    pub fn ofssel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dacr2::Ofssel,
        dacr2::Ofssel,
        Dacr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dacr2::Ofssel,
            dacr2::Ofssel,
            Dacr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dacr2 {
    #[inline(always)]
    fn default() -> Dacr2 {
        <crate::RegValueT<Dacr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dacr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ofssel_SPEC;
    pub type Ofssel = crate::EnumBitfieldStruct<u8, Ofssel_SPEC>;
    impl Ofssel {
        #[doc = "Normal voltage mode (VREFH ≥ 2.7 V)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low voltage mode (VREFH < 2.7 V)"]
        pub const _1: Self = Self::new(1);
    }
}
