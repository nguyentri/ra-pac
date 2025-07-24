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
#[doc = r"OCD Function"]
unsafe impl ::core::marker::Send for super::CpuOcd {}
unsafe impl ::core::marker::Sync for super::CpuOcd {}
impl super::CpuOcd {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "MCU Status Register"]
    #[inline(always)]
    pub const fn mcustat(
        &self,
    ) -> &'static crate::common::Reg<self::Mcustat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mcustat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[doc = "MCU Control Register"]
    #[inline(always)]
    pub const fn mcuctrl(
        &self,
    ) -> &'static crate::common::Reg<self::Mcuctrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mcuctrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1040usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcustat_SPEC;
impl crate::sealed::RegSpec for Mcustat_SPEC {
    type DataType = u32;
}

#[doc = "MCU Status Register"]
pub type Mcustat = crate::RegValueT<Mcustat_SPEC>;

impl Mcustat {
    #[doc = "Sleep mode status"]
    #[inline(always)]
    pub fn cpusleep(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mcustat::Cpusleep,
        mcustat::Cpusleep,
        Mcustat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mcustat::Cpusleep,
            mcustat::Cpusleep,
            Mcustat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CPU clock status"]
    #[inline(always)]
    pub fn cpustopclk(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mcustat::Cpustopclk,
        mcustat::Cpustopclk,
        Mcustat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mcustat::Cpustopclk,
            mcustat::Cpustopclk,
            Mcustat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Debugger status"]
    #[inline(always)]
    pub fn dbgfuncen(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        mcustat::Dbgfuncen,
        mcustat::Dbgfuncen,
        Mcustat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            mcustat::Dbgfuncen,
            mcustat::Dbgfuncen,
            Mcustat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Secure Debug status"]
    #[inline(always)]
    pub fn secdbg(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        mcustat::Secdbg,
        mcustat::Secdbg,
        Mcustat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            mcustat::Secdbg,
            mcustat::Secdbg,
            Mcustat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mcustat {
    #[inline(always)]
    fn default() -> Mcustat {
        <crate::RegValueT<Mcustat_SPEC> as RegisterValue<_>>::new(256)
    }
}
pub mod mcustat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpusleep_SPEC;
    pub type Cpusleep = crate::EnumBitfieldStruct<u8, Cpusleep_SPEC>;
    impl Cpusleep {
        #[doc = "CPU is not in Sleep mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU in Sleep mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpustopclk_SPEC;
    pub type Cpustopclk = crate::EnumBitfieldStruct<u8, Cpustopclk_SPEC>;
    impl Cpustopclk {
        #[doc = "CPU clock is not stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU clock is stopped"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbgfuncen_SPEC;
    pub type Dbgfuncen = crate::EnumBitfieldStruct<u8, Dbgfuncen_SPEC>;
    impl Dbgfuncen {
        #[doc = "Debugger connection is not available"]
        pub const _0: Self = Self::new(0);

        #[doc = "Debugger function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secdbg_SPEC;
    pub type Secdbg = crate::EnumBitfieldStruct<u8, Secdbg_SPEC>;
    impl Secdbg {
        #[doc = "Secure Debug is not available"]
        pub const _0: Self = Self::new(0);

        #[doc = "Secure Debug is available"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcuctrl_SPEC;
impl crate::sealed::RegSpec for Mcuctrl_SPEC {
    type DataType = u32;
}

#[doc = "MCU Control Register"]
pub type Mcuctrl = crate::RegValueT<Mcuctrl_SPEC>;

impl Mcuctrl {
    #[doc = "Debug Interrupt Request"]
    #[inline(always)]
    pub fn dbirq(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mcuctrl::Dbirq,
        mcuctrl::Dbirq,
        Mcuctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mcuctrl::Dbirq,
            mcuctrl::Dbirq,
            Mcuctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU Wait Setting"]
    #[inline(always)]
    pub fn cpuwait(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mcuctrl::Cpuwait,
        mcuctrl::Cpuwait,
        Mcuctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mcuctrl::Cpuwait,
            mcuctrl::Cpuwait,
            Mcuctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mcuctrl {
    #[inline(always)]
    fn default() -> Mcuctrl {
        <crate::RegValueT<Mcuctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcuctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbirq_SPEC;
    pub type Dbirq = crate::EnumBitfieldStruct<u8, Dbirq_SPEC>;
    impl Dbirq {
        #[doc = "Debug interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Debug interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpuwait_SPEC;
    pub type Cpuwait = crate::EnumBitfieldStruct<u8, Cpuwait_SPEC>;
    impl Cpuwait {
        #[doc = "Clear CPUWAIT to low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set CPUWAIT to high"]
        pub const _1: Self = Self::new(1);
    }
}
