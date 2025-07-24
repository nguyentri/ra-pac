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
// Generated from SVD 1.00.01, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:53:56 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CPU Control Registers"]
unsafe impl ::core::marker::Send for super::CpuCtrl {}
unsafe impl ::core::marker::Sync for super::CpuCtrl {}
impl super::CpuCtrl {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "CPU Lockup Control Register"]
    #[inline(always)]
    pub const fn cpulckupcr(
        &self,
    ) -> &'static crate::common::Reg<self::Cpulckupcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpulckupcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "CPU Function Lock Control Register"]
    #[inline(always)]
    pub const fn cpulockcr(
        &self,
    ) -> &'static crate::common::Reg<self::Cpulockcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpulockcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[doc = "CPU Control Register Protection Register"]
    #[inline(always)]
    pub const fn cpucrpt(
        &self,
    ) -> &'static crate::common::Reg<self::Cpucrpt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpucrpt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2112usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpulckupcr_SPEC;
impl crate::sealed::RegSpec for Cpulckupcr_SPEC {
    type DataType = u8;
}

#[doc = "CPU Lockup Control Register"]
pub type Cpulckupcr = crate::RegValueT<Cpulckupcr_SPEC>;

impl Cpulckupcr {
    #[doc = "Operation after detection of CPU lockup"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cpulckupcr::Oad,
        cpulckupcr::Oad,
        Cpulckupcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cpulckupcr::Oad,
            cpulckupcr::Oad,
            Cpulckupcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cpulckupcr {
    #[inline(always)]
    fn default() -> Cpulckupcr {
        <crate::RegValueT<Cpulckupcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpulckupcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non-maskable Interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU Lockup reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpulockcr_SPEC;
impl crate::sealed::RegSpec for Cpulockcr_SPEC {
    type DataType = u8;
}

#[doc = "CPU Function Lock Control Register"]
pub type Cpulockcr = crate::RegValueT<Cpulockcr_SPEC>;

impl Cpulockcr {
    #[doc = "Disable writes to the following secure registers from software or from a debug agent that is connected to the processor:"]
    #[inline(always)]
    pub fn lcksvtair(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Cpulockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Cpulockcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disable writes to registers that are associated with the Secure MPU region from software or from a debug agent connected to the processor:"]
    #[inline(always)]
    pub fn lcksmpu(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Cpulockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Cpulockcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disable writes to registers that are associated with the SAU region from software or from a debug agent connected to the processor:"]
    #[inline(always)]
    pub fn lcksau(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Cpulockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Cpulockcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disable writes to registers that are associated with the ITCM interface security gating from software or from a debug agent connected to the processor:"]
    #[inline(always)]
    pub fn lckitgu(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Cpulockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Cpulockcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disable writes to registers that are associated with the DTCM interface security gating from software or from a debug agent connected to the processor:"]
    #[inline(always)]
    pub fn lckdtgu(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Cpulockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Cpulockcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lckdcaic(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cpulockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Cpulockcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cpulockcr {
    #[inline(always)]
    fn default() -> Cpulockcr {
        <crate::RegValueT<Cpulockcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpucrpt_SPEC;
impl crate::sealed::RegSpec for Cpucrpt_SPEC {
    type DataType = u16;
}

#[doc = "CPU Control Register Protection Register"]
pub type Cpucrpt = crate::RegValueT<Cpucrpt_SPEC>;

impl Cpucrpt {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cpucrpt::Protect,
        cpucrpt::Protect,
        Cpucrpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cpucrpt::Protect,
            cpucrpt::Protect,
            Cpucrpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The KEY\\[7:0\\] bits enable or disable writing to the PROTECT bit."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cpucrpt_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cpucrpt_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cpucrpt {
    #[inline(always)]
    fn default() -> Cpucrpt {
        <crate::RegValueT<Cpucrpt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpucrpt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Writing to CPULCKUPCR register is permitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing to CPULCKUPCR register is not permitted. Read access is permitted."]
        pub const _1: Self = Self::new(1);
    }
}
