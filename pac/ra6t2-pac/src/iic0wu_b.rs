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
// Generated from SVD 1.40.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:37:20 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Inter-Integrated Circuit 0 Wake-up Unit"]
unsafe impl ::core::marker::Send for super::Iic0WuB {}
unsafe impl ::core::marker::Sync for super::Iic0WuB {}
impl super::Iic0WuB {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Wake Up Unit Control Register"]
    #[inline(always)]
    pub const fn wuctl(&self) -> &'static crate::common::Reg<self::Wuctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wuctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Wake Up Unit Operating Status Register"]
    #[inline(always)]
    pub const fn wust(&self) -> &'static crate::common::Reg<self::Wust_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Wust_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wuctl_SPEC;
impl crate::sealed::RegSpec for Wuctl_SPEC {
    type DataType = u32;
}

#[doc = "Wake Up Unit Control Register"]
pub type Wuctl = crate::RegValueT<Wuctl_SPEC>;

impl Wuctl {
    #[doc = "Wake-Up Acknowledge Selection"]
    #[inline(always)]
    pub fn wuacks(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Wuctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Wuctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wake-Up Analog Noise Filter Selection"]
    #[inline(always)]
    pub fn wuanfs(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        wuctl::Wuanfs,
        wuctl::Wuanfs,
        Wuctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            wuctl::Wuanfs,
            wuctl::Wuanfs,
            Wuctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Wake-Up function PCLKA Synchronous Enable"]
    #[inline(always)]
    pub fn wufsyne(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        wuctl::Wufsyne,
        wuctl::Wufsyne,
        Wuctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            wuctl::Wufsyne,
            wuctl::Wufsyne,
            Wuctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Wake-Up function Enable"]
    #[inline(always)]
    pub fn wufe(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        wuctl::Wufe,
        wuctl::Wufe,
        Wuctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            wuctl::Wufe,
            wuctl::Wufe,
            Wuctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Wuctl {
    #[inline(always)]
    fn default() -> Wuctl {
        <crate::RegValueT<Wuctl_SPEC> as RegisterValue<_>>::new(65)
    }
}
pub mod wuctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wuanfs_SPEC;
    pub type Wuanfs = crate::EnumBitfieldStruct<u8, Wuanfs_SPEC>;
    impl Wuanfs {
        #[doc = "Do not add the Wake Up analog filter."]
        pub const _0: Self = Self::new(0);

        #[doc = "Add the Wake Up analog filter."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wufsyne_SPEC;
    pub type Wufsyne = crate::EnumBitfieldStruct<u8, Wufsyne_SPEC>;
    impl Wufsyne {
        #[doc = "IIC asynchronous circuit enable"]
        pub const _0: Self = Self::new(0);

        #[doc = "IIC synchronous circuit enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wufe_SPEC;
    pub type Wufe = crate::EnumBitfieldStruct<u8, Wufe_SPEC>;
    impl Wufe {
        #[doc = "Wake-up function disables"]
        pub const _0: Self = Self::new(0);

        #[doc = "Wake-up function enables"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wust_SPEC;
impl crate::sealed::RegSpec for Wust_SPEC {
    type DataType = u32;
}

#[doc = "Wake Up Unit Operating Status Register"]
pub type Wust = crate::RegValueT<Wust_SPEC>;

impl Wust {
    #[doc = "Wake-up function asynchronous operation status flag"]
    #[inline(always)]
    pub fn wuasynf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        wust::Wuasynf,
        wust::Wuasynf,
        Wust_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            wust::Wuasynf,
            wust::Wuasynf,
            Wust_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Wust {
    #[inline(always)]
    fn default() -> Wust {
        <crate::RegValueT<Wust_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod wust {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wuasynf_SPEC;
    pub type Wuasynf = crate::EnumBitfieldStruct<u8, Wuasynf_SPEC>;
    impl Wuasynf {
        #[doc = "IIC synchronous circuit enable condition."]
        pub const _0: Self = Self::new(0);

        #[doc = "IIC asynchronous circuit enable condition."]
        pub const _1: Self = Self::new(1);
    }
}
