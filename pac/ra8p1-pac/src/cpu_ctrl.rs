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
#[doc = r"CPU(CTRL)"]
unsafe impl ::core::marker::Send for super::CpuCtrl {}
unsafe impl ::core::marker::Sync for super::CpuCtrl {}
impl super::CpuCtrl {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "CPU%s Lockup Control Register"]
    #[inline(always)]
    pub const fn cpulckupcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cpulckupcr_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x30usize))
        }
    }
    #[inline(always)]
    pub const fn cpu0lckupcr(
        &self,
    ) -> &'static crate::common::Reg<self::Cpulckupcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpulckupcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cpu1lckupcr(
        &self,
    ) -> &'static crate::common::Reg<self::Cpulckupcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpulckupcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }

    #[doc = "CPU Initial Vector Base Address Register"]
    #[inline(always)]
    pub const fn cpuinitvtor(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cpuinitvtor_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }
    #[inline(always)]
    pub const fn cpu0initvtor(
        &self,
    ) -> &'static crate::common::Reg<self::Cpuinitvtor_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpuinitvtor_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cpu1initvtor(
        &self,
    ) -> &'static crate::common::Reg<self::Cpuinitvtor_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpuinitvtor_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }

    #[doc = "CPU%s CPUWAIT Control Register"]
    #[inline(always)]
    pub const fn cpuwaitcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cpuwaitcr_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x50usize))
        }
    }
    #[inline(always)]
    pub const fn cpu0waitcr(
        &self,
    ) -> &'static crate::common::Reg<self::Cpuwaitcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpuwaitcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cpu1waitcr(
        &self,
    ) -> &'static crate::common::Reg<self::Cpuwaitcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpuwaitcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }

    #[doc = "CPU%s Activation Control and Status Register"]
    #[inline(always)]
    pub const fn cpuactcsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cpuactcsr_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x60usize))
        }
    }
    #[inline(always)]
    pub const fn cpu0actcsr(
        &self,
    ) -> &'static crate::common::Reg<self::Cpuactcsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpuactcsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cpu1actcsr(
        &self,
    ) -> &'static crate::common::Reg<self::Cpuactcsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpuactcsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64usize),
            )
        }
    }

    #[doc = "CPU0 Local Memory Error Control Register"]
    #[inline(always)]
    pub const fn cpu0lmecr(
        &self,
    ) -> &'static crate::common::Reg<self::Cpu0Lmecr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpu0Lmecr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "CPU Identification Register"]
    #[inline(always)]
    pub const fn cpuidr(
        &self,
    ) -> &'static crate::common::Reg<self::Cpuidr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpuidr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "CPU0 Status Monitor Register"]
    #[inline(always)]
    pub const fn cpu0statm(
        &self,
    ) -> &'static crate::common::Reg<self::Cpu0Statm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpu0Statm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "CPU1 Status Monitor Register"]
    #[inline(always)]
    pub const fn cpu1statm(
        &self,
    ) -> &'static crate::common::Reg<self::Cpu1Statm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpu1Statm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "CPU SECEXT Monitor Register"]
    #[inline(always)]
    pub const fn secextmon(
        &self,
    ) -> &'static crate::common::Reg<self::Secextmon_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Secextmon_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Non-secure CPU Control Register"]
    #[inline(always)]
    pub const fn nscpucr(
        &self,
    ) -> &'static crate::common::Reg<self::Nscpucr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nscpucr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "CPU0 Function Lock Control Register"]
    #[inline(always)]
    pub const fn cpu0lockcr(
        &self,
    ) -> &'static crate::common::Reg<self::Cpu0Lockcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpu0Lockcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[doc = "CPU1 Function Lock Control Register"]
    #[inline(always)]
    pub const fn cpu1lockcr(
        &self,
    ) -> &'static crate::common::Reg<self::Cpu1Lockcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpu1Lockcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1028usize),
            )
        }
    }

    #[doc = "CPU%s Control Register Protection Register"]
    #[inline(always)]
    pub const fn cpucrpt(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cpucrpt_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x840usize))
        }
    }
    #[inline(always)]
    pub const fn cpu0crpt(
        &self,
    ) -> &'static crate::common::Reg<self::Cpucrpt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpucrpt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x840usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cpu1crpt(
        &self,
    ) -> &'static crate::common::Reg<self::Cpucrpt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpucrpt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x844usize),
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

#[doc = "CPU%s Lockup Control Register"]
pub type Cpulckupcr = crate::RegValueT<Cpulckupcr_SPEC>;

impl Cpulckupcr {
    #[doc = "Operation after detection of CPUn lockup"]
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
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "System reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpuinitvtor_SPEC;
impl crate::sealed::RegSpec for Cpuinitvtor_SPEC {
    type DataType = u32;
}

#[doc = "CPU Initial Vector Base Address Register"]
pub type Cpuinitvtor = crate::RegValueT<Cpuinitvtor_SPEC>;

impl Cpuinitvtor {
    #[doc = "CPUn Initial Vector Base Address"]
    #[inline(always)]
    pub fn cpuinitvtor(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Cpuinitvtor_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Cpuinitvtor_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cpuinitvtor {
    #[inline(always)]
    fn default() -> Cpuinitvtor {
        <crate::RegValueT<Cpuinitvtor_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpuwaitcr_SPEC;
impl crate::sealed::RegSpec for Cpuwaitcr_SPEC {
    type DataType = u8;
}

#[doc = "CPU%s CPUWAIT Control Register"]
pub type Cpuwaitcr = crate::RegValueT<Cpuwaitcr_SPEC>;

impl Cpuwaitcr {
    #[doc = "Writing 1 to stall the CPUn when it is out of reset."]
    #[inline(always)]
    pub fn cpuwait(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cpuwaitcr::Cpuwait,
        cpuwaitcr::Cpuwait,
        Cpuwaitcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cpuwaitcr::Cpuwait,
            cpuwaitcr::Cpuwait,
            Cpuwaitcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cpuwaitcr {
    #[inline(always)]
    fn default() -> Cpuwaitcr {
        <crate::RegValueT<Cpuwaitcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpuwaitcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpuwait_SPEC;
    pub type Cpuwait = crate::EnumBitfieldStruct<u8, Cpuwait_SPEC>;
    impl Cpuwait {
        #[doc = "CPUn starts instruction execution out of reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "CPUn is forced into a quiescent state out of reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpuactcsr_SPEC;
impl crate::sealed::RegSpec for Cpuactcsr_SPEC {
    type DataType = u16;
}

#[doc = "CPU%s Activation Control and Status Register"]
pub type Cpuactcsr = crate::RegValueT<Cpuactcsr_SPEC>;

impl Cpuactcsr {
    #[doc = "CPUn activation request"]
    #[inline(always)]
    pub fn actreq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cpuactcsr::Actreq,
        cpuactcsr::Actreq,
        Cpuactcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cpuactcsr::Actreq,
            cpuactcsr::Actreq,
            Cpuactcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPUn activation status"]
    #[inline(always)]
    pub fn act(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cpuactcsr::Act,
        cpuactcsr::Act,
        Cpuactcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cpuactcsr::Act,
            cpuactcsr::Act,
            Cpuactcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cpuactcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cpuactcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cpuactcsr {
    #[inline(always)]
    fn default() -> Cpuactcsr {
        <crate::RegValueT<Cpuactcsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpuactcsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actreq_SPEC;
    pub type Actreq = crate::EnumBitfieldStruct<u8, Actreq_SPEC>;
    impl Actreq {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request CPUn activation if ACT bit is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Act_SPEC;
    pub type Act = crate::EnumBitfieldStruct<u8, Act_SPEC>;
    impl Act {
        #[doc = "CPUn is in inactive state (power gating/reset state)"]
        pub const _0: Self = Self::new(0);

        #[doc = "CPUn is in active state (out of reset)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0Lmecr_SPEC;
impl crate::sealed::RegSpec for Cpu0Lmecr_SPEC {
    type DataType = u8;
}

#[doc = "CPU0 Local Memory Error Control Register"]
pub type Cpu0Lmecr = crate::RegValueT<Cpu0Lmecr_SPEC>;

impl Cpu0Lmecr {
    #[doc = "System Reset Request Enable"]
    #[inline(always)]
    pub fn syrsten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cpu0lmecr::Syrsten,
        cpu0lmecr::Syrsten,
        Cpu0Lmecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cpu0lmecr::Syrsten,
            cpu0lmecr::Syrsten,
            Cpu0Lmecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cpu0Lmecr {
    #[inline(always)]
    fn default() -> Cpu0Lmecr {
        <crate::RegValueT<Cpu0Lmecr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpu0lmecr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syrsten_SPEC;
    pub type Syrsten = crate::EnumBitfieldStruct<u8, Syrsten_SPEC>;
    impl Syrsten {
        #[doc = "Disables system reset request when multiple bits error occurs in data cache or TCM"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables system reset request when multiple bits error occurs in data cache or TCM"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpuidr_SPEC;
impl crate::sealed::RegSpec for Cpuidr_SPEC {
    type DataType = u8;
}

#[doc = "CPU Identification Register"]
pub type Cpuidr = crate::RegValueT<Cpuidr_SPEC>;

impl Cpuidr {
    #[doc = "CPU Identification"]
    #[inline(always)]
    pub fn cpuid(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cpuidr::Cpuid,
        cpuidr::Cpuid,
        Cpuidr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cpuidr::Cpuid,
            cpuidr::Cpuid,
            Cpuidr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cpuidr {
    #[inline(always)]
    fn default() -> Cpuidr {
        <crate::RegValueT<Cpuidr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpuidr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpuid_SPEC;
    pub type Cpuid = crate::EnumBitfieldStruct<u8, Cpuid_SPEC>;
    impl Cpuid {
        #[doc = "CPU0"]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0Statm_SPEC;
impl crate::sealed::RegSpec for Cpu0Statm_SPEC {
    type DataType = u8;
}

#[doc = "CPU0 Status Monitor Register"]
pub type Cpu0Statm = crate::RegValueT<Cpu0Statm_SPEC>;

impl Cpu0Statm {
    #[doc = "CPU0 Sleeping State"]
    #[inline(always)]
    pub fn sleeping(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cpu0statm::Sleeping,
        cpu0statm::Sleeping,
        Cpu0Statm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cpu0statm::Sleeping,
            cpu0statm::Sleeping,
            Cpu0Statm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CPU0 SLEEPDEEP state"]
    #[inline(always)]
    pub fn sleepdeep(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Cpu0Statm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cpu0Statm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn sahbstp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cpu0statm::Sahbstp,
        cpu0statm::Sahbstp,
        Cpu0Statm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cpu0statm::Sahbstp,
            cpu0statm::Sahbstp,
            Cpu0Statm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cpu0Statm {
    #[inline(always)]
    fn default() -> Cpu0Statm {
        <crate::RegValueT<Cpu0Statm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpu0statm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sleeping_SPEC;
    pub type Sleeping = crate::EnumBitfieldStruct<u8, Sleeping_SPEC>;
    impl Sleeping {
        #[doc = "CPU0 is running"]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU0 is at Sleep mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sahbstp_SPEC;
    pub type Sahbstp = crate::EnumBitfieldStruct<u8, Sahbstp_SPEC>;
    impl Sahbstp {
        #[doc = "CPU0 S-AHB is active"]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU0 S-AHB is stopped"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu1Statm_SPEC;
impl crate::sealed::RegSpec for Cpu1Statm_SPEC {
    type DataType = u8;
}

#[doc = "CPU1 Status Monitor Register"]
pub type Cpu1Statm = crate::RegValueT<Cpu1Statm_SPEC>;

impl Cpu1Statm {
    #[doc = "CPU1 Sleeping State"]
    #[inline(always)]
    pub fn sleeping(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cpu1statm::Sleeping,
        cpu1statm::Sleeping,
        Cpu1Statm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cpu1statm::Sleeping,
            cpu1statm::Sleeping,
            Cpu1Statm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CPU1 SLEEPDEEP state"]
    #[inline(always)]
    pub fn sleepdeep(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Cpu1Statm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cpu1Statm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cpu1Statm {
    #[inline(always)]
    fn default() -> Cpu1Statm {
        <crate::RegValueT<Cpu1Statm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpu1statm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sleeping_SPEC;
    pub type Sleeping = crate::EnumBitfieldStruct<u8, Sleeping_SPEC>;
    impl Sleeping {
        #[doc = "CPU1 is running"]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU1 is at Sleep mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Secextmon_SPEC;
impl crate::sealed::RegSpec for Secextmon_SPEC {
    type DataType = u8;
}

#[doc = "CPU SECEXT Monitor Register"]
pub type Secextmon = crate::RegValueT<Secextmon_SPEC>;

impl Secextmon {
    #[doc = "CPU0 Security Extension"]
    #[inline(always)]
    pub fn secext0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        secextmon::Secext0,
        secextmon::Secext0,
        Secextmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            secextmon::Secext0,
            secextmon::Secext0,
            Secextmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CPU1 Security Extension"]
    #[inline(always)]
    pub fn secext1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        secextmon::Secext1,
        secextmon::Secext1,
        Secextmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            secextmon::Secext1,
            secextmon::Secext1,
            Secextmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Secextmon {
    #[inline(always)]
    fn default() -> Secextmon {
        <crate::RegValueT<Secextmon_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod secextmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secext0_SPEC;
    pub type Secext0 = crate::EnumBitfieldStruct<u8, Secext0_SPEC>;
    impl Secext0 {
        #[doc = "CPU0 does not include Security Extension"]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU0 includes Security Extension"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secext1_SPEC;
    pub type Secext1 = crate::EnumBitfieldStruct<u8, Secext1_SPEC>;
    impl Secext1 {
        #[doc = "CPU1 does not include Security Extension"]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU1 includes Security Extension"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nscpucr_SPEC;
impl crate::sealed::RegSpec for Nscpucr_SPEC {
    type DataType = u32;
}

#[doc = "Non-secure CPU Control Register"]
pub type Nscpucr = crate::RegValueT<Nscpucr_SPEC>;

impl Nscpucr {
    #[doc = "System Reset Request Enable"]
    #[inline(always)]
    pub fn rstreqen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        nscpucr::Rstreqen,
        nscpucr::Rstreqen,
        Nscpucr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            nscpucr::Rstreqen,
            nscpucr::Rstreqen,
            Nscpucr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nscpucr {
    #[inline(always)]
    fn default() -> Nscpucr {
        <crate::RegValueT<Nscpucr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod nscpucr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstreqen_SPEC;
    pub type Rstreqen = crate::EnumBitfieldStruct<u8, Rstreqen_SPEC>;
    impl Rstreqen {
        #[doc = "System reset request caused by non-secure CPU without SECEXT is not allowed"]
        pub const _0: Self = Self::new(0);

        #[doc = "System reset request caused by non-secure CPU without SECEXT is allowed"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0Lockcr_SPEC;
impl crate::sealed::RegSpec for Cpu0Lockcr_SPEC {
    type DataType = u8;
}

#[doc = "CPU0 Function Lock Control Register"]
pub type Cpu0Lockcr = crate::RegValueT<Cpu0Lockcr_SPEC>;

impl Cpu0Lockcr {
    #[doc = "Disables writes to the following secure registers from software or from a debug agent that is connected to the processor."]
    #[inline(always)]
    pub fn lcksvtair(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Cpu0Lockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Cpu0Lockcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disables writes to registers that are associated with the Secure MPU region from software or from a debug agent connected to the processor."]
    #[inline(always)]
    pub fn lcksmpu(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Cpu0Lockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Cpu0Lockcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This signal disables writes to registers that are associated with the SAU region from software or from a debug agent connected to the processor."]
    #[inline(always)]
    pub fn lcksau(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Cpu0Lockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Cpu0Lockcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ITGUCTRL, ITGU_LUTn"]
    #[inline(always)]
    pub fn lckitgu(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Cpu0Lockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Cpu0Lockcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DTGUCTRL, DTGU_LUTn"]
    #[inline(always)]
    pub fn lckdtgu(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Cpu0Lockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Cpu0Lockcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lckdcaic(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cpu0Lockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Cpu0Lockcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cpu0Lockcr {
    #[inline(always)]
    fn default() -> Cpu0Lockcr {
        <crate::RegValueT<Cpu0Lockcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu1Lockcr_SPEC;
impl crate::sealed::RegSpec for Cpu1Lockcr_SPEC {
    type DataType = u8;
}

#[doc = "CPU1 Function Lock Control Register"]
pub type Cpu1Lockcr = crate::RegValueT<Cpu1Lockcr_SPEC>;

impl Cpu1Lockcr {
    #[doc = "Disables writes to the following secure registers from software or from a debug agent that is connected to the processor."]
    #[inline(always)]
    pub fn lcksvtair(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Cpu1Lockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Cpu1Lockcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disables writes to registers that are associated with the Secure MPU region from software or from a debug agent connected to the processor."]
    #[inline(always)]
    pub fn lcksmpu(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Cpu1Lockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Cpu1Lockcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This signal disables writes to registers that are associated with the SAU region from software or from a debug agent connected to the processor."]
    #[inline(always)]
    pub fn lcksau(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Cpu1Lockcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Cpu1Lockcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cpu1Lockcr {
    #[inline(always)]
    fn default() -> Cpu1Lockcr {
        <crate::RegValueT<Cpu1Lockcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpucrpt_SPEC;
impl crate::sealed::RegSpec for Cpucrpt_SPEC {
    type DataType = u16;
}

#[doc = "CPU%s Control Register Protection Register"]
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
        #[doc = "Writing to CPUnLCKUPCR register is available."]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing to CPUnLCKUPCR register is not available. Read access is available."]
        pub const _1: Self = Self::new(1);
    }
}
