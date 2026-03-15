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
// Generated from SVD 0.90.02, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:05:50 +0000

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
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x48usize))
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

    #[doc = "Event Link Controller Security Attribution Register A"]
    #[inline(always)]
    pub const fn elcsara(
        &self,
    ) -> &'static crate::common::Reg<self::Elcsara_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcsara_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "Event Link Controller Security Attribution Register B"]
    #[inline(always)]
    pub const fn elcsarb(
        &self,
    ) -> &'static crate::common::Reg<self::Elcsarb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcsarb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "Event Link Controller Security Attribution Register C"]
    #[inline(always)]
    pub const fn elcsarc(
        &self,
    ) -> &'static crate::common::Reg<self::Elcsarc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcsarc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
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
        #[doc = "ELC function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "ELC function is enabled"]
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
}
impl ::core::default::Default for Elsegr {
    #[inline(always)]
    fn default() -> Elsegr {
        <crate::RegValueT<Elsegr_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod elsegr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Seg_SPEC;
    pub type Seg = crate::EnumBitfieldStruct<u8, Seg_SPEC>;
    impl Seg {
        #[doc = "Normal operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software event is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct We_SPEC;
    pub type We = crate::EnumBitfieldStruct<u8, We_SPEC>;
    impl We {
        #[doc = "Write to SEG bit disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write to SEG bit enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wi_SPEC;
    pub type Wi = crate::EnumBitfieldStruct<u8, Wi_SPEC>;
    impl Wi {
        #[doc = "Write to ELSEGR register enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write to ELSEGR register disabled"]
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
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Elsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Elsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Elsr {
    #[inline(always)]
    fn default() -> Elsr {
        <crate::RegValueT<Elsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcsara_SPEC;
impl crate::sealed::RegSpec for Elcsara_SPEC {
    type DataType = u16;
}

#[doc = "Event Link Controller Security Attribution Register A"]
pub type Elcsara = crate::RegValueT<Elcsara_SPEC>;

impl Elcsara {
    #[doc = "Event Link Controller Register Security Attribution"]
    #[inline(always)]
    pub fn elcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        elcsara::Elcr,
        elcsara::Elcr,
        Elcsara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            elcsara::Elcr,
            elcsara::Elcr,
            Elcsara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Software Event Generation Register 0 Security Attribution"]
    #[inline(always)]
    pub fn elsegr0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        elcsara::Elsegr0,
        elcsara::Elsegr0,
        Elcsara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            elcsara::Elsegr0,
            elcsara::Elsegr0,
            Elcsara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Software Event Generation Register 1 Security Attribution"]
    #[inline(always)]
    pub fn elsegr1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        elcsara::Elsegr1,
        elcsara::Elsegr1,
        Elcsara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            elcsara::Elsegr1,
            elcsara::Elsegr1,
            Elcsara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elcsara {
    #[inline(always)]
    fn default() -> Elcsara {
        <crate::RegValueT<Elcsara_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod elcsara {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elcr_SPEC;
    pub type Elcr = crate::EnumBitfieldStruct<u8, Elcr_SPEC>;
    impl Elcr {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsegr0_SPEC;
    pub type Elsegr0 = crate::EnumBitfieldStruct<u8, Elsegr0_SPEC>;
    impl Elsegr0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsegr1_SPEC;
    pub type Elsegr1 = crate::EnumBitfieldStruct<u8, Elsegr1_SPEC>;
    impl Elsegr1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcsarb_SPEC;
impl crate::sealed::RegSpec for Elcsarb_SPEC {
    type DataType = u16;
}

#[doc = "Event Link Controller Security Attribution Register B"]
pub type Elcsarb = crate::RegValueT<Elcsarb_SPEC>;

impl Elcsarb {
    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        elcsarb::Elsr,
        elcsarb::Elsr,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            elcsarb::Elsr,
            elcsarb::Elsr,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elcsarb {
    #[inline(always)]
    fn default() -> Elcsarb {
        <crate::RegValueT<Elcsarb_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod elcsarb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr_SPEC;
    pub type Elsr = crate::EnumBitfieldStruct<u8, Elsr_SPEC>;
    impl Elsr {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcsarc_SPEC;
impl crate::sealed::RegSpec for Elcsarc_SPEC {
    type DataType = u16;
}

#[doc = "Event Link Controller Security Attribution Register C"]
pub type Elcsarc = crate::RegValueT<Elcsarc_SPEC>;

impl Elcsarc {
    #[doc = "Event Link Setting Register n Security Attribution (n = 16 to 18)"]
    #[inline(always)]
    pub fn elsr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        elcsarc::Elsr,
        elcsarc::Elsr,
        Elcsarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            elcsarc::Elsr,
            elcsarc::Elsr,
            Elcsarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elcsarc {
    #[inline(always)]
    fn default() -> Elcsarc {
        <crate::RegValueT<Elcsarc_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod elcsarc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr_SPEC;
    pub type Elsr = crate::EnumBitfieldStruct<u8, Elsr_SPEC>;
    impl Elsr {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
