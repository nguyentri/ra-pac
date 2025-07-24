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
#[doc = r"IPC_NS"]
unsafe impl ::core::marker::Send for super::IpcNs {}
unsafe impl ::core::marker::Sync for super::IpcNs {}
impl super::IpcNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Semaphore Register %s (n = 0 to 15)"]
    #[inline(always)]
    pub const fn ipcsem(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub const fn ipcsem0(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem1(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem2(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem3(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem4(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem5(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem6(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem7(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem8(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem9(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem10(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem11(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem12(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem13(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem14(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ipcsem15(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsem_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsem_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3cusize),
            )
        }
    }

    #[doc = "Inter-Processor0 NMI Request Status Register"]
    #[inline(always)]
    pub const fn ipc0nmista(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc0Nmista_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ipc0Nmista_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Inter-Processor0 NMI Request Set Register"]
    #[inline(always)]
    pub const fn ipc0nmiset(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc0Nmiset_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc0Nmiset_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "Inter-Processor0 NMI Request Clear Register"]
    #[inline(always)]
    pub const fn ipc0nmiclr(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc0Nmiclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc0Nmiclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "Inter-Processor1 NMI Request Status Register"]
    #[inline(always)]
    pub const fn ipc1nmista(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc1Nmista_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ipc1Nmista_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Inter-Processor1 NMI Request Set Register"]
    #[inline(always)]
    pub const fn ipc1nmiset(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc1Nmiset_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc1Nmiset_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "Inter-Processor1 NMI Request Clear Register"]
    #[inline(always)]
    pub const fn ipc1nmiclr(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc1Nmiclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc1Nmiclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "Inter-Processor0 Status Register0"]
    #[inline(always)]
    pub const fn ipc0sta0(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc0Sta0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ipc0Sta0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "Inter-Processor0 IRQ Request Set Register0"]
    #[inline(always)]
    pub const fn ipc0iset0(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc0Iset0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc0Iset0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[doc = "Inter-Processor0 FIFO Transfer Data Register0"]
    #[inline(always)]
    pub const fn ipc0txd0(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc0Txd0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc0Txd0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[doc = "Inter-Processor0 FIFO Receive Data Register0"]
    #[inline(always)]
    pub const fn ipc0rxd0(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc0Rxd0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ipc0Rxd0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[doc = "Inter-Processor0 Clear Register0"]
    #[inline(always)]
    pub const fn ipc0clr0(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc0Clr0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc0Clr0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "Inter-Processor0 Status Register1"]
    #[inline(always)]
    pub const fn ipc0sta1(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc0Sta1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ipc0Sta1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[doc = "Inter-Processor0 IRQ Request Set Register1"]
    #[inline(always)]
    pub const fn ipc0iset1(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc0Iset1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc0Iset1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[doc = "Inter-Processor0 FIFO Transfer Data Register1"]
    #[inline(always)]
    pub const fn ipc0txd1(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc0Txd1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc0Txd1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[doc = "Inter-Processor0 FIFO Receive Data Register1"]
    #[inline(always)]
    pub const fn ipc0rxd1(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc0Rxd1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ipc0Rxd1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(236usize),
            )
        }
    }

    #[doc = "Inter-Processor0 Clear Register1"]
    #[inline(always)]
    pub const fn ipc0clr1(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc0Clr1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc0Clr1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[doc = "Inter-Processor1 Status Register0"]
    #[inline(always)]
    pub const fn ipc1sta0(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc1Sta0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ipc1Sta0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Inter-Processor1 IRQ Request Set Register0"]
    #[inline(always)]
    pub const fn ipc1iset0(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc1Iset0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc1Iset0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "Inter-Processor1 FIFO Transfer Data Register0"]
    #[inline(always)]
    pub const fn ipc1txd0(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc1Txd0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc1Txd0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "Inter-Processor1 FIFO Receive Data Register0"]
    #[inline(always)]
    pub const fn ipc1rxd0(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc1Rxd0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ipc1Rxd0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[doc = "Inter-Processor1 Request Clear Register0"]
    #[inline(always)]
    pub const fn ipc1clr0(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc1Clr0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc1Clr0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "Inter-Processor1 Request Status Register1"]
    #[inline(always)]
    pub const fn ipc1sta1(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc1Sta1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ipc1Sta1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "Inter-Processor1 IRQ Request Set Register1"]
    #[inline(always)]
    pub const fn ipc1iset1(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc1Iset1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc1Iset1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[doc = "Inter-Processor1 FIFO Transfer Data Register1"]
    #[inline(always)]
    pub const fn ipc1txd1(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc1Txd1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc1Txd1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(296usize),
            )
        }
    }

    #[doc = "Inter-Processor1 FIFO Receive Data Register1"]
    #[inline(always)]
    pub const fn ipc1rxd1(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc1Rxd1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ipc1Rxd1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(300usize),
            )
        }
    }

    #[doc = "Inter-Processor1 Clear Register1"]
    #[inline(always)]
    pub const fn ipc1clr1(
        &self,
    ) -> &'static crate::common::Reg<self::Ipc1Clr1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ipc1Clr1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcsem_SPEC;
impl crate::sealed::RegSpec for Ipcsem_SPEC {
    type DataType = u32;
}

#[doc = "Semaphore Register %s (n = 0 to 15)"]
pub type Ipcsem = crate::RegValueT<Ipcsem_SPEC>;

impl Ipcsem {
    #[doc = "This bit indicates the shared resource is locked."]
    #[inline(always)]
    pub fn lock(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ipcsem::Lock,
        ipcsem::Lock,
        Ipcsem_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ipcsem::Lock,
            ipcsem::Lock,
            Ipcsem_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ipcsem {
    #[inline(always)]
    fn default() -> Ipcsem {
        <crate::RegValueT<Ipcsem_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ipcsem {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lock_SPEC;
    pub type Lock = crate::EnumBitfieldStruct<u8, Lock_SPEC>;
    impl Lock {
        #[doc = "Unlocked"]
        pub const _0: Self = Self::new(0);

        #[doc = "Locked"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc0Nmista_SPEC;
impl crate::sealed::RegSpec for Ipc0Nmista_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor0 NMI Request Status Register"]
pub type Ipc0Nmista = crate::RegValueT<Ipc0Nmista_SPEC>;

impl Ipc0Nmista {
    #[doc = "This bit indicates the status of interrupt request."]
    #[inline(always)]
    pub fn nmi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ipc0nmista::Nmi,
        ipc0nmista::Nmi,
        Ipc0Nmista_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ipc0nmista::Nmi,
            ipc0nmista::Nmi,
            Ipc0Nmista_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ipc0Nmista {
    #[inline(always)]
    fn default() -> Ipc0Nmista {
        <crate::RegValueT<Ipc0Nmista_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ipc0nmista {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmi_SPEC;
    pub type Nmi = crate::EnumBitfieldStruct<u8, Nmi_SPEC>;
    impl Nmi {
        #[doc = "Not requesting interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Requesting interrupt"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc0Nmiset_SPEC;
impl crate::sealed::RegSpec for Ipc0Nmiset_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor0 NMI Request Set Register"]
pub type Ipc0Nmiset = crate::RegValueT<Ipc0Nmiset_SPEC>;

impl Ipc0Nmiset {
    #[doc = "Writing 1 to the bit sets IPC0NMISTA.NMI."]
    #[inline(always)]
    pub fn set(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ipc0Nmiset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Ipc0Nmiset_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipc0Nmiset {
    #[inline(always)]
    fn default() -> Ipc0Nmiset {
        <crate::RegValueT<Ipc0Nmiset_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc0Nmiclr_SPEC;
impl crate::sealed::RegSpec for Ipc0Nmiclr_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor0 NMI Request Clear Register"]
pub type Ipc0Nmiclr = crate::RegValueT<Ipc0Nmiclr_SPEC>;

impl Ipc0Nmiclr {
    #[doc = "Writing 1 to this bit clears IPC0NMISTA.NMI."]
    #[inline(always)]
    pub fn clr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ipc0Nmiclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Ipc0Nmiclr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipc0Nmiclr {
    #[inline(always)]
    fn default() -> Ipc0Nmiclr {
        <crate::RegValueT<Ipc0Nmiclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc1Nmista_SPEC;
impl crate::sealed::RegSpec for Ipc1Nmista_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor1 NMI Request Status Register"]
pub type Ipc1Nmista = crate::RegValueT<Ipc1Nmista_SPEC>;

impl Ipc1Nmista {
    #[doc = "This bit indicates the status of interrupt request."]
    #[inline(always)]
    pub fn nmi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ipc1nmista::Nmi,
        ipc1nmista::Nmi,
        Ipc1Nmista_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ipc1nmista::Nmi,
            ipc1nmista::Nmi,
            Ipc1Nmista_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ipc1Nmista {
    #[inline(always)]
    fn default() -> Ipc1Nmista {
        <crate::RegValueT<Ipc1Nmista_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ipc1nmista {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmi_SPEC;
    pub type Nmi = crate::EnumBitfieldStruct<u8, Nmi_SPEC>;
    impl Nmi {
        #[doc = "Not requesting interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Requesting interrupt"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc1Nmiset_SPEC;
impl crate::sealed::RegSpec for Ipc1Nmiset_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor1 NMI Request Set Register"]
pub type Ipc1Nmiset = crate::RegValueT<Ipc1Nmiset_SPEC>;

impl Ipc1Nmiset {
    #[doc = "Writing 1 to this bit sets IPC1NMISTA.NMI."]
    #[inline(always)]
    pub fn set(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ipc1Nmiset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Ipc1Nmiset_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipc1Nmiset {
    #[inline(always)]
    fn default() -> Ipc1Nmiset {
        <crate::RegValueT<Ipc1Nmiset_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc1Nmiclr_SPEC;
impl crate::sealed::RegSpec for Ipc1Nmiclr_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor1 NMI Request Clear Register"]
pub type Ipc1Nmiclr = crate::RegValueT<Ipc1Nmiclr_SPEC>;

impl Ipc1Nmiclr {
    #[doc = "Writing 1 to this bit clears IPC1NMISTA.NMI."]
    #[inline(always)]
    pub fn clr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ipc1Nmiclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Ipc1Nmiclr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipc1Nmiclr {
    #[inline(always)]
    fn default() -> Ipc1Nmiclr {
        <crate::RegValueT<Ipc1Nmiclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc0Sta0_SPEC;
impl crate::sealed::RegSpec for Ipc0Sta0_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor0 Status Register0"]
pub type Ipc0Sta0 = crate::RegValueT<Ipc0Sta0_SPEC>;

impl Ipc0Sta0 {
    #[doc = "This bit indicates the status of interrupt request."]
    #[inline(always)]
    pub fn irq0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ipc0sta0::Irq0,
        ipc0sta0::Irq0,
        Ipc0Sta0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ipc0sta0::Irq0,
            ipc0sta0::Irq0,
            Ipc0Sta0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "This bit indicates the status of interrupt request."]
    #[inline(always)]
    pub fn irq1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ipc0sta0::Irq1,
        ipc0sta0::Irq1,
        Ipc0Sta0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ipc0sta0::Irq1,
            ipc0sta0::Irq1,
            Ipc0Sta0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "This bit indicates the status of interrupt request."]
    #[inline(always)]
    pub fn irq2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ipc0sta0::Irq2,
        ipc0sta0::Irq2,
        Ipc0Sta0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ipc0sta0::Irq2,
            ipc0sta0::Irq2,
            Ipc0Sta0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "This bit indicates the status of interrupt request."]
    #[inline(always)]
    pub fn irq3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ipc0sta0::Irq3,
        ipc0sta0::Irq3,
        Ipc0Sta0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ipc0sta0::Irq3,
            ipc0sta0::Irq3,
            Ipc0Sta0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "This bit indicates the status of interrupt request."]
    #[inline(always)]
    pub fn irq4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ipc0sta0::Irq4,
        ipc0sta0::Irq4,
        Ipc0Sta0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ipc0sta0::Irq4,
            ipc0sta0::Irq4,
            Ipc0Sta0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "This bit indicates the status of interrupt request."]
    #[inline(always)]
    pub fn irq5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ipc0sta0::Irq5,
        ipc0sta0::Irq5,
        Ipc0Sta0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ipc0sta0::Irq5,
            ipc0sta0::Irq5,
            Ipc0Sta0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "This bit indicates the status of interrupt request."]
    #[inline(always)]
    pub fn irq6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ipc0sta0::Irq6,
        ipc0sta0::Irq6,
        Ipc0Sta0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ipc0sta0::Irq6,
            ipc0sta0::Irq6,
            Ipc0Sta0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "This bit indicates the status of interrupt request."]
    #[inline(always)]
    pub fn irq7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ipc0sta0::Irq7,
        ipc0sta0::Irq7,
        Ipc0Sta0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ipc0sta0::Irq7,
            ipc0sta0::Irq7,
            Ipc0Sta0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "This bit is set when FIFO is not empty."]
    #[inline(always)]
    pub fn rdy(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ipc0sta0::Rdy,
        ipc0sta0::Rdy,
        Ipc0Sta0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ipc0sta0::Rdy,
            ipc0sta0::Rdy,
            Ipc0Sta0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn full(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ipc0sta0::Full,
        ipc0sta0::Full,
        Ipc0Sta0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ipc0sta0::Full,
            ipc0sta0::Full,
            Ipc0Sta0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Indicates that the message FIFO 00 tried to read data despite being empty."]
    #[inline(always)]
    pub fn rerr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        ipc0sta0::Rerr,
        ipc0sta0::Rerr,
        Ipc0Sta0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            ipc0sta0::Rerr,
            ipc0sta0::Rerr,
            Ipc0Sta0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Indicates that the message FIFO 00 tried to send more data even though it was full."]
    #[inline(always)]
    pub fn ferr(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ipc0sta0::Ferr,
        ipc0sta0::Ferr,
        Ipc0Sta0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ipc0sta0::Ferr,
            ipc0sta0::Ferr,
            Ipc0Sta0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ipc0Sta0 {
    #[inline(always)]
    fn default() -> Ipc0Sta0 {
        <crate::RegValueT<Ipc0Sta0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ipc0sta0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq0_SPEC;
    pub type Irq0 = crate::EnumBitfieldStruct<u8, Irq0_SPEC>;
    impl Irq0 {
        #[doc = "Not requesting interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Requesting interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq1_SPEC;
    pub type Irq1 = crate::EnumBitfieldStruct<u8, Irq1_SPEC>;
    impl Irq1 {
        #[doc = "Not requesting interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Requesting interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq2_SPEC;
    pub type Irq2 = crate::EnumBitfieldStruct<u8, Irq2_SPEC>;
    impl Irq2 {
        #[doc = "Not requesting interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Requesting interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq3_SPEC;
    pub type Irq3 = crate::EnumBitfieldStruct<u8, Irq3_SPEC>;
    impl Irq3 {
        #[doc = "Not requesting interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Requesting interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq4_SPEC;
    pub type Irq4 = crate::EnumBitfieldStruct<u8, Irq4_SPEC>;
    impl Irq4 {
        #[doc = "Not requesting interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Requesting interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq5_SPEC;
    pub type Irq5 = crate::EnumBitfieldStruct<u8, Irq5_SPEC>;
    impl Irq5 {
        #[doc = "Not requesting interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Requesting interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq6_SPEC;
    pub type Irq6 = crate::EnumBitfieldStruct<u8, Irq6_SPEC>;
    impl Irq6 {
        #[doc = "Not requesting interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Requesting interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq7_SPEC;
    pub type Irq7 = crate::EnumBitfieldStruct<u8, Irq7_SPEC>;
    impl Irq7 {
        #[doc = "Not requesting interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Requesting interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdy_SPEC;
    pub type Rdy = crate::EnumBitfieldStruct<u8, Rdy_SPEC>;
    impl Rdy {
        #[doc = "FIFO 00 data is not ready"]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO 00 data is ready"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Full_SPEC;
    pub type Full = crate::EnumBitfieldStruct<u8, Full_SPEC>;
    impl Full {
        #[doc = "FIFO 00 is full"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rerr_SPEC;
    pub type Rerr = crate::EnumBitfieldStruct<u8, Rerr_SPEC>;
    impl Rerr {
        #[doc = "Not Error. Not requesting interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Error. Requesting interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ferr_SPEC;
    pub type Ferr = crate::EnumBitfieldStruct<u8, Ferr_SPEC>;
    impl Ferr {
        #[doc = "Not Error. Not requesting interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Error. Requesting interrupt."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc0Iset0_SPEC;
impl crate::sealed::RegSpec for Ipc0Iset0_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor0 IRQ Request Set Register0"]
pub type Ipc0Iset0 = crate::RegValueT<Ipc0Iset0_SPEC>;

impl NoBitfieldReg<Ipc0Iset0_SPEC> for Ipc0Iset0 {}
impl ::core::default::Default for Ipc0Iset0 {
    #[inline(always)]
    fn default() -> Ipc0Iset0 {
        <crate::RegValueT<Ipc0Iset0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc0Txd0_SPEC;
impl crate::sealed::RegSpec for Ipc0Txd0_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor0 FIFO Transfer Data Register0"]
pub type Ipc0Txd0 = crate::RegValueT<Ipc0Txd0_SPEC>;

impl Ipc0Txd0 {
    #[doc = "Transfer data"]
    #[inline(always)]
    pub fn txd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ipc0Txd0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ipc0Txd0_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipc0Txd0 {
    #[inline(always)]
    fn default() -> Ipc0Txd0 {
        <crate::RegValueT<Ipc0Txd0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc0Rxd0_SPEC;
impl crate::sealed::RegSpec for Ipc0Rxd0_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor0 FIFO Receive Data Register0"]
pub type Ipc0Rxd0 = crate::RegValueT<Ipc0Rxd0_SPEC>;

impl Ipc0Rxd0 {
    #[doc = "Receive data."]
    #[inline(always)]
    pub fn rxd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ipc0Rxd0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ipc0Rxd0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipc0Rxd0 {
    #[inline(always)]
    fn default() -> Ipc0Rxd0 {
        <crate::RegValueT<Ipc0Rxd0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc0Clr0_SPEC;
impl crate::sealed::RegSpec for Ipc0Clr0_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor0 Clear Register0"]
pub type Ipc0Clr0 = crate::RegValueT<Ipc0Clr0_SPEC>;

impl Ipc0Clr0 {
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ipc0Clr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ipc0Clr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Writing 1 resets IPC0STA0.RERR."]
    #[inline(always)]
    pub fn rclr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ipc0Clr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ipc0Clr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Writing 1 resets IPC0STA0. FERR."]
    #[inline(always)]
    pub fn fclr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ipc0Clr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ipc0Clr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ipc0Clr0 {
    #[inline(always)]
    fn default() -> Ipc0Clr0 {
        <crate::RegValueT<Ipc0Clr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc0Sta1_SPEC;
impl crate::sealed::RegSpec for Ipc0Sta1_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor0 Status Register1"]
pub type Ipc0Sta1 = crate::RegValueT<Ipc0Sta1_SPEC>;

impl Ipc0Sta1 {
    #[doc = "This bit is set when FIFO is not empty."]
    #[inline(always)]
    pub fn rdy(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ipc0sta1::Rdy,
        ipc0sta1::Rdy,
        Ipc0Sta1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ipc0sta1::Rdy,
            ipc0sta1::Rdy,
            Ipc0Sta1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn full(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ipc0sta1::Full,
        ipc0sta1::Full,
        Ipc0Sta1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ipc0sta1::Full,
            ipc0sta1::Full,
            Ipc0Sta1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Indicates that the message FIFO 01 tried to read data despite being empty."]
    #[inline(always)]
    pub fn rerr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        ipc0sta1::Rerr,
        ipc0sta1::Rerr,
        Ipc0Sta1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            ipc0sta1::Rerr,
            ipc0sta1::Rerr,
            Ipc0Sta1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Indicates that the message FIFO 01 tried to send more data even though it was full."]
    #[inline(always)]
    pub fn ferr(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ipc0sta1::Ferr,
        ipc0sta1::Ferr,
        Ipc0Sta1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ipc0sta1::Ferr,
            ipc0sta1::Ferr,
            Ipc0Sta1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ipc0Sta1 {
    #[inline(always)]
    fn default() -> Ipc0Sta1 {
        <crate::RegValueT<Ipc0Sta1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ipc0sta1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdy_SPEC;
    pub type Rdy = crate::EnumBitfieldStruct<u8, Rdy_SPEC>;
    impl Rdy {
        #[doc = "FIFO 01 data is not ready"]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO 01 data is ready"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Full_SPEC;
    pub type Full = crate::EnumBitfieldStruct<u8, Full_SPEC>;
    impl Full {
        #[doc = "FIFO 01 is full"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rerr_SPEC;
    pub type Rerr = crate::EnumBitfieldStruct<u8, Rerr_SPEC>;
    impl Rerr {
        #[doc = "Not Error. Not requesting interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Error. Requesting interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ferr_SPEC;
    pub type Ferr = crate::EnumBitfieldStruct<u8, Ferr_SPEC>;
    impl Ferr {
        #[doc = "Not Error. Not requesting interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Error. Requesting interrupt."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc0Iset1_SPEC;
impl crate::sealed::RegSpec for Ipc0Iset1_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor0 IRQ Request Set Register1"]
pub type Ipc0Iset1 = crate::RegValueT<Ipc0Iset1_SPEC>;

impl NoBitfieldReg<Ipc0Iset1_SPEC> for Ipc0Iset1 {}
impl ::core::default::Default for Ipc0Iset1 {
    #[inline(always)]
    fn default() -> Ipc0Iset1 {
        <crate::RegValueT<Ipc0Iset1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc0Txd1_SPEC;
impl crate::sealed::RegSpec for Ipc0Txd1_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor0 FIFO Transfer Data Register1"]
pub type Ipc0Txd1 = crate::RegValueT<Ipc0Txd1_SPEC>;

impl Ipc0Txd1 {
    #[doc = "Transfer data"]
    #[inline(always)]
    pub fn txd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ipc0Txd1_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ipc0Txd1_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipc0Txd1 {
    #[inline(always)]
    fn default() -> Ipc0Txd1 {
        <crate::RegValueT<Ipc0Txd1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc0Rxd1_SPEC;
impl crate::sealed::RegSpec for Ipc0Rxd1_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor0 FIFO Receive Data Register1"]
pub type Ipc0Rxd1 = crate::RegValueT<Ipc0Rxd1_SPEC>;

impl Ipc0Rxd1 {
    #[doc = "Receive data"]
    #[inline(always)]
    pub fn rxd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ipc0Rxd1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ipc0Rxd1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipc0Rxd1 {
    #[inline(always)]
    fn default() -> Ipc0Rxd1 {
        <crate::RegValueT<Ipc0Rxd1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc0Clr1_SPEC;
impl crate::sealed::RegSpec for Ipc0Clr1_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor0 Clear Register1"]
pub type Ipc0Clr1 = crate::RegValueT<Ipc0Clr1_SPEC>;

impl Ipc0Clr1 {
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ipc0Clr1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ipc0Clr1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn rclr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ipc0Clr1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ipc0Clr1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn fclr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ipc0Clr1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ipc0Clr1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ipc0Clr1 {
    #[inline(always)]
    fn default() -> Ipc0Clr1 {
        <crate::RegValueT<Ipc0Clr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc1Sta0_SPEC;
impl crate::sealed::RegSpec for Ipc1Sta0_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor1 Status Register0"]
pub type Ipc1Sta0 = crate::RegValueT<Ipc1Sta0_SPEC>;

impl Ipc1Sta0 {
    #[doc = "This bit is set when FIFO is not empty."]
    #[inline(always)]
    pub fn rdy(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ipc1sta0::Rdy,
        ipc1sta0::Rdy,
        Ipc1Sta0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ipc1sta0::Rdy,
            ipc1sta0::Rdy,
            Ipc1Sta0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn full(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ipc1sta0::Full,
        ipc1sta0::Full,
        Ipc1Sta0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ipc1sta0::Full,
            ipc1sta0::Full,
            Ipc1Sta0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit indicates that the message FIFO 10 tried to read data despite being empty."]
    #[inline(always)]
    pub fn rerr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        ipc1sta0::Rerr,
        ipc1sta0::Rerr,
        Ipc1Sta0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            ipc1sta0::Rerr,
            ipc1sta0::Rerr,
            Ipc1Sta0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "This bit indicates that the message FIFO 10 tried to send more data even though it was full."]
    #[inline(always)]
    pub fn ferr(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ipc1sta0::Ferr,
        ipc1sta0::Ferr,
        Ipc1Sta0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ipc1sta0::Ferr,
            ipc1sta0::Ferr,
            Ipc1Sta0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ipc1Sta0 {
    #[inline(always)]
    fn default() -> Ipc1Sta0 {
        <crate::RegValueT<Ipc1Sta0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ipc1sta0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdy_SPEC;
    pub type Rdy = crate::EnumBitfieldStruct<u8, Rdy_SPEC>;
    impl Rdy {
        #[doc = "FIFO 10 data is not ready"]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO 10 data is ready"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Full_SPEC;
    pub type Full = crate::EnumBitfieldStruct<u8, Full_SPEC>;
    impl Full {
        #[doc = "FIFO 10 is full"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rerr_SPEC;
    pub type Rerr = crate::EnumBitfieldStruct<u8, Rerr_SPEC>;
    impl Rerr {
        #[doc = "Not Error. Not requesting interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Error. Requesting interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ferr_SPEC;
    pub type Ferr = crate::EnumBitfieldStruct<u8, Ferr_SPEC>;
    impl Ferr {
        #[doc = "Not Error. Not requesting interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Error. Requesting interrupt."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc1Iset0_SPEC;
impl crate::sealed::RegSpec for Ipc1Iset0_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor1 IRQ Request Set Register0"]
pub type Ipc1Iset0 = crate::RegValueT<Ipc1Iset0_SPEC>;

impl NoBitfieldReg<Ipc1Iset0_SPEC> for Ipc1Iset0 {}
impl ::core::default::Default for Ipc1Iset0 {
    #[inline(always)]
    fn default() -> Ipc1Iset0 {
        <crate::RegValueT<Ipc1Iset0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc1Txd0_SPEC;
impl crate::sealed::RegSpec for Ipc1Txd0_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor1 FIFO Transfer Data Register0"]
pub type Ipc1Txd0 = crate::RegValueT<Ipc1Txd0_SPEC>;

impl Ipc1Txd0 {
    #[doc = "Transfer data"]
    #[inline(always)]
    pub fn txd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ipc1Txd0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ipc1Txd0_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipc1Txd0 {
    #[inline(always)]
    fn default() -> Ipc1Txd0 {
        <crate::RegValueT<Ipc1Txd0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc1Rxd0_SPEC;
impl crate::sealed::RegSpec for Ipc1Rxd0_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor1 FIFO Receive Data Register0"]
pub type Ipc1Rxd0 = crate::RegValueT<Ipc1Rxd0_SPEC>;

impl Ipc1Rxd0 {
    #[doc = "Receive data"]
    #[inline(always)]
    pub fn rxd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ipc1Rxd0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ipc1Rxd0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipc1Rxd0 {
    #[inline(always)]
    fn default() -> Ipc1Rxd0 {
        <crate::RegValueT<Ipc1Rxd0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc1Clr0_SPEC;
impl crate::sealed::RegSpec for Ipc1Clr0_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor1 Request Clear Register0"]
pub type Ipc1Clr0 = crate::RegValueT<Ipc1Clr0_SPEC>;

impl Ipc1Clr0 {
    #[doc = "Writing 1 resets Message FIFO 10."]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ipc1Clr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ipc1Clr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn rclr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ipc1Clr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ipc1Clr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Writing 1 resets IPC1STA0. FERR."]
    #[inline(always)]
    pub fn fclr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ipc1Clr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ipc1Clr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ipc1Clr0 {
    #[inline(always)]
    fn default() -> Ipc1Clr0 {
        <crate::RegValueT<Ipc1Clr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc1Sta1_SPEC;
impl crate::sealed::RegSpec for Ipc1Sta1_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor1 Request Status Register1"]
pub type Ipc1Sta1 = crate::RegValueT<Ipc1Sta1_SPEC>;

impl Ipc1Sta1 {
    #[doc = "This bit is set when FIFO is not empty."]
    #[inline(always)]
    pub fn rdy(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ipc1sta1::Rdy,
        ipc1sta1::Rdy,
        Ipc1Sta1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ipc1sta1::Rdy,
            ipc1sta1::Rdy,
            Ipc1Sta1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn full(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ipc1sta1::Full,
        ipc1sta1::Full,
        Ipc1Sta1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ipc1sta1::Full,
            ipc1sta1::Full,
            Ipc1Sta1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Indicates that the message FIFO 11 tried to read data despite being empty."]
    #[inline(always)]
    pub fn rerr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        ipc1sta1::Rerr,
        ipc1sta1::Rerr,
        Ipc1Sta1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            ipc1sta1::Rerr,
            ipc1sta1::Rerr,
            Ipc1Sta1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Indicates that the message FIFO 11 tried to send more data even though it was full."]
    #[inline(always)]
    pub fn ferr(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ipc1sta1::Ferr,
        ipc1sta1::Ferr,
        Ipc1Sta1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ipc1sta1::Ferr,
            ipc1sta1::Ferr,
            Ipc1Sta1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ipc1Sta1 {
    #[inline(always)]
    fn default() -> Ipc1Sta1 {
        <crate::RegValueT<Ipc1Sta1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ipc1sta1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdy_SPEC;
    pub type Rdy = crate::EnumBitfieldStruct<u8, Rdy_SPEC>;
    impl Rdy {
        #[doc = "FIFO 11 data is not ready"]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO 11 data is ready"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Full_SPEC;
    pub type Full = crate::EnumBitfieldStruct<u8, Full_SPEC>;
    impl Full {
        #[doc = "FIFO 11 is full"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rerr_SPEC;
    pub type Rerr = crate::EnumBitfieldStruct<u8, Rerr_SPEC>;
    impl Rerr {
        #[doc = "Not Error. Not requesting interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Error. Requesting interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ferr_SPEC;
    pub type Ferr = crate::EnumBitfieldStruct<u8, Ferr_SPEC>;
    impl Ferr {
        #[doc = "Not Error. Not requesting interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Error. Requesting interrupt."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc1Iset1_SPEC;
impl crate::sealed::RegSpec for Ipc1Iset1_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor1 IRQ Request Set Register1"]
pub type Ipc1Iset1 = crate::RegValueT<Ipc1Iset1_SPEC>;

impl NoBitfieldReg<Ipc1Iset1_SPEC> for Ipc1Iset1 {}
impl ::core::default::Default for Ipc1Iset1 {
    #[inline(always)]
    fn default() -> Ipc1Iset1 {
        <crate::RegValueT<Ipc1Iset1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc1Txd1_SPEC;
impl crate::sealed::RegSpec for Ipc1Txd1_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor1 FIFO Transfer Data Register1"]
pub type Ipc1Txd1 = crate::RegValueT<Ipc1Txd1_SPEC>;

impl Ipc1Txd1 {
    #[doc = "Transfer data"]
    #[inline(always)]
    pub fn txd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ipc1Txd1_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ipc1Txd1_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipc1Txd1 {
    #[inline(always)]
    fn default() -> Ipc1Txd1 {
        <crate::RegValueT<Ipc1Txd1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc1Rxd1_SPEC;
impl crate::sealed::RegSpec for Ipc1Rxd1_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor1 FIFO Receive Data Register1"]
pub type Ipc1Rxd1 = crate::RegValueT<Ipc1Rxd1_SPEC>;

impl Ipc1Rxd1 {
    #[doc = "Receive data"]
    #[inline(always)]
    pub fn rxd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ipc1Rxd1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ipc1Rxd1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipc1Rxd1 {
    #[inline(always)]
    fn default() -> Ipc1Rxd1 {
        <crate::RegValueT<Ipc1Rxd1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipc1Clr1_SPEC;
impl crate::sealed::RegSpec for Ipc1Clr1_SPEC {
    type DataType = u32;
}

#[doc = "Inter-Processor1 Clear Register1"]
pub type Ipc1Clr1 = crate::RegValueT<Ipc1Clr1_SPEC>;

impl Ipc1Clr1 {
    #[doc = "Writing 1 resets message FIFO 11."]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ipc1Clr1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ipc1Clr1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Writing 1 resets IPC1STA1.RERR."]
    #[inline(always)]
    pub fn rclr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ipc1Clr1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ipc1Clr1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Writing 1resets IPC1STA1. FERR."]
    #[inline(always)]
    pub fn fclr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ipc1Clr1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ipc1Clr1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ipc1Clr1 {
    #[inline(always)]
    fn default() -> Ipc1Clr1 {
        <crate::RegValueT<Ipc1Clr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
