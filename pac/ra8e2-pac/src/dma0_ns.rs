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
// Generated from SVD 1.00.01, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:40:06 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"DMAC Module Activation 0"]
unsafe impl ::core::marker::Send for super::Dma0Ns {}
unsafe impl ::core::marker::Sync for super::Dma0Ns {}
impl super::Dma0Ns {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "DMA Module Activation Register"]
    #[inline(always)]
    pub const fn dmast(&self) -> &'static crate::common::Reg<self::Dmast_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmast_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "DMAC Error Channel Register"]
    #[inline(always)]
    pub const fn dmechr(
        &self,
    ) -> &'static crate::common::Reg<self::Dmechr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmechr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "DMAC Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn delsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Delsr_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x80usize))
        }
    }
    #[inline(always)]
    pub const fn delsr0(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn delsr1(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn delsr2(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn delsr3(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn delsr4(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn delsr5(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn delsr6(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn delsr7(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9cusize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmast_SPEC;
impl crate::sealed::RegSpec for Dmast_SPEC {
    type DataType = u8;
}

#[doc = "DMA Module Activation Register"]
pub type Dmast = crate::RegValueT<Dmast_SPEC>;

impl Dmast {
    #[doc = "DMAC Operation Enable"]
    #[inline(always)]
    pub fn dmst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmast::Dmst,
        dmast::Dmst,
        Dmast_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmast::Dmst,
            dmast::Dmst,
            Dmast_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmast {
    #[inline(always)]
    fn default() -> Dmast {
        <crate::RegValueT<Dmast_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmast {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dmst_SPEC;
    pub type Dmst = crate::EnumBitfieldStruct<u8, Dmst_SPEC>;
    impl Dmst {
        #[doc = "DMAC activation is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "DMAC activation is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmechr_SPEC;
impl crate::sealed::RegSpec for Dmechr_SPEC {
    type DataType = u32;
}

#[doc = "DMAC Error Channel Register"]
pub type Dmechr = crate::RegValueT<Dmechr_SPEC>;

impl Dmechr {
    #[doc = "DMAC Error channel"]
    #[inline(always)]
    pub fn dmech(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Dmechr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Dmechr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMAC Error channel Security Attribution Monitor"]
    #[inline(always)]
    pub fn dmechsam(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dmechr::Dmechsam,
        dmechr::Dmechsam,
        Dmechr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dmechr::Dmechsam,
            dmechr::Dmechsam,
            Dmechr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DMAC Error Status"]
    #[inline(always)]
    pub fn dmesta(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dmechr::Dmesta,
        dmechr::Dmesta,
        Dmechr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dmechr::Dmesta,
            dmechr::Dmesta,
            Dmechr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmechr {
    #[inline(always)]
    fn default() -> Dmechr {
        <crate::RegValueT<Dmechr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmechr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dmechsam_SPEC;
    pub type Dmechsam = crate::EnumBitfieldStruct<u8, Dmechsam_SPEC>;
    impl Dmechsam {
        #[doc = "secure channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "non-secure channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dmesta_SPEC;
    pub type Dmesta = crate::EnumBitfieldStruct<u8, Dmesta_SPEC>;
    impl Dmesta {
        #[doc = "No DMA transfer error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "DMA transfer error occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Delsr_SPEC;
impl crate::sealed::RegSpec for Delsr_SPEC {
    type DataType = u32;
}

#[doc = "DMAC Event Link Setting Register %s"]
pub type Delsr = crate::RegValueT<Delsr_SPEC>;

impl Delsr {
    #[doc = "DMAC Event Link Select"]
    #[inline(always)]
    pub fn dels(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        delsr::Dels,
        delsr::Dels,
        Delsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            delsr::Dels,
            delsr::Dels,
            Delsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DMAC Activation Request Status flag"]
    #[inline(always)]
    pub fn ir(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        delsr::Ir,
        delsr::Ir,
        Delsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            delsr::Ir,
            delsr::Ir,
            Delsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Delsr {
    #[inline(always)]
    fn default() -> Delsr {
        <crate::RegValueT<Delsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod delsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dels_SPEC;
    pub type Dels = crate::EnumBitfieldStruct<u8, Dels_SPEC>;
    impl Dels {
        #[doc = "Disable interrupts to the associated DMAC module"]
        pub const _0_X_00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ir_SPEC;
    pub type Ir = crate::EnumBitfieldStruct<u8, Ir_SPEC>;
    impl Ir {
        #[doc = "No DMAC activation request occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "DMAC activation request occurred."]
        pub const _1: Self = Self::new(1);
    }
}
