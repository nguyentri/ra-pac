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
// Generated from SVD 1.0, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:52:24 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Event Link Controller"]
unsafe impl ::core::marker::Send for super::Elc {}
unsafe impl ::core::marker::Sync for super::Elc {}
impl super::Elc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Event Link Controller Register"]
    #[inline(always)]
    pub const fn elcr(&self) -> &'static crate::common::Reg<self::Elcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Event Link Software Event Generation Register %s"]
    #[inline(always)]
    pub const fn elsegr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Elsegr_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2usize))
        }
    }
    #[inline(always)]
    pub const fn elsegr0(
        &self,
    ) -> &'static crate::common::Reg<self::Elsegr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsegr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsegr1(
        &self,
    ) -> &'static crate::common::Reg<self::Elsegr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsegr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4usize),
            )
        }
    }

    #[doc = "Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn elsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Elsr_SPEC, crate::common::RW>,
        19,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10usize))
        }
    }
    #[inline(always)]
    pub const fn elsr0(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr1(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr2(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr3(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr4(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr5(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr6(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr7(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr8(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr9(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr10(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr11(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr12(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr13(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr14(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr15(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr16(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr17(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr18(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcr_SPEC;
impl crate::sealed::RegSpec for Elcr_SPEC {
    type DataType = u8;
}

#[doc = "Event Link Controller Register"]
pub type Elcr = crate::RegValueT<Elcr_SPEC>;

impl Elcr {
    #[doc = "All Event Link Enable"]
    #[inline(always)]
    pub fn elcon(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        elcr::Elcon,
        elcr::Elcon,
        Elcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            elcr::Elcon,
            elcr::Elcon,
            Elcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elcr {
    #[inline(always)]
    fn default() -> Elcr {
        <crate::RegValueT<Elcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod elcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elcon_SPEC;
    pub type Elcon = crate::EnumBitfieldStruct<u8, Elcon_SPEC>;
    impl Elcon {
        #[doc = "Disable ELC function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable ELC function."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsegr_SPEC;
impl crate::sealed::RegSpec for Elsegr_SPEC {
    type DataType = u8;
}

#[doc = "Event Link Software Event Generation Register %s"]
pub type Elsegr = crate::RegValueT<Elsegr_SPEC>;

impl Elsegr {
    #[doc = "ELSEGR Register Write Disable"]
    #[inline(always)]
    pub fn wi(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        elsegr::Wi,
        elsegr::Wi,
        Elsegr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            elsegr::Wi,
            elsegr::Wi,
            Elsegr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "SEG Bit Write Enable"]
    #[inline(always)]
    pub fn we(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        elsegr::We,
        elsegr::We,
        Elsegr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            elsegr::We,
            elsegr::We,
            Elsegr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software Event Generation"]
    #[inline(always)]
    pub fn seg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        elsegr::Seg,
        elsegr::Seg,
        Elsegr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            elsegr::Seg,
            elsegr::Seg,
            Elsegr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elsegr {
    #[inline(always)]
    fn default() -> Elsegr {
        <crate::RegValueT<Elsegr_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod elsegr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wi_SPEC;
    pub type Wi = crate::EnumBitfieldStruct<u8, Wi_SPEC>;
    impl Wi {
        #[doc = "Enable writes to ELSEGR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable writes to ELSEGR register."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct We_SPEC;
    pub type We = crate::EnumBitfieldStruct<u8, We_SPEC>;
    impl We {
        #[doc = "Disable writes to SEG bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes to SEG bit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Seg_SPEC;
    pub type Seg = crate::EnumBitfieldStruct<u8, Seg_SPEC>;
    impl Seg {
        #[doc = "Normal operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generate a software event"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsr_SPEC;
impl crate::sealed::RegSpec for Elsr_SPEC {
    type DataType = u16;
}

#[doc = "Event Link Setting Register %s"]
pub type Elsr = crate::RegValueT<Elsr_SPEC>;

impl Elsr {
    #[doc = "Event Link Select"]
    #[inline(always)]
    pub fn els(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        elsr::Els,
        elsr::Els,
        Elsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            elsr::Els,
            elsr::Els,
            Elsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elsr {
    #[inline(always)]
    fn default() -> Elsr {
        <crate::RegValueT<Elsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod elsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Els_SPEC;
    pub type Els = crate::EnumBitfieldStruct<u8, Els_SPEC>;
    impl Els {
        #[doc = "Event output to the corresponding peripheral module is disabled."]
        pub const _0_X_000: Self = Self::new(0);

        #[doc = "Set the number for the event signal to be linked."]
        pub const OTHERS: Self = Self::new(0);
    }
}
