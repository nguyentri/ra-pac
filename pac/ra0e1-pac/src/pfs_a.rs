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
// Generated from SVD 1.10.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:59 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Pmn Pin Function Select"]
unsafe impl ::core::marker::Send for super::PfsA {}
unsafe impl ::core::marker::Sync for super::PfsA {}
impl super::PfsA {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs_a(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsA_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10usize))
        }
    }
    #[inline(always)]
    pub const fn p008pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p009pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12usize),
            )
        }
    }

    #[doc = "Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p0pfs_a(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P0PfsA_SPEC, crate::common::RW>,
        5,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x14usize))
        }
    }
    #[inline(always)]
    pub const fn p010pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p011pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x16usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p012pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p013pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p014pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cusize),
            )
        }
    }

    #[doc = "Port 015 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p015pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P015PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P015PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[doc = "Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p10pfs_a(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P10PfsA_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x24usize))
        }
    }
    #[inline(always)]
    pub const fn p102pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p103pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x26usize),
            )
        }
    }

    #[doc = "Port 108 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p108pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P108PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P108PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Port 109 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p109pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P109PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P109PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "Port 110 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p110pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P110PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P110PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Port 112 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p112pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P112PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P112PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Port 200 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p200pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P200PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P200PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Port 201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P201PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P201PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[doc = "Port 206 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p206pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P206PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P206PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p20pfs_a(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20PfsA_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4eusize))
        }
    }
    #[inline(always)]
    pub const fn p207pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p208pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }

    #[doc = "Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p2pfs_a(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P2PfsA_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x5cusize))
        }
    }
    #[inline(always)]
    pub const fn p214pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p215pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5eusize),
            )
        }
    }

    #[doc = "Port 300 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p300pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P300PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P300PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Port 407 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p407pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P407PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P407PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(142usize),
            )
        }
    }

    #[doc = "Port 9%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p9pfs_a(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P9PfsA_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13ausize))
        }
    }
    #[inline(always)]
    pub const fn p913pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p914pfs_a(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13cusize),
            )
        }
    }

    #[doc = "Write-Protect Register"]
    #[inline(always)]
    pub const fn pwpr(&self) -> &'static crate::common::Reg<self::Pwpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00PfsA_SPEC;
impl crate::sealed::RegSpec for P00PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 00%s Pin Function Select Register"]
pub type P00PfsA = crate::RegValueT<P00PfsA_SPEC>;

impl P00PfsA {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p00pfs_a::Podr,
        p00pfs_a::Podr,
        P00PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p00pfs_a::Podr,
            p00pfs_a::Podr,
            P00PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p00pfs_a::Pidr,
        p00pfs_a::Pidr,
        P00PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p00pfs_a::Pidr,
            p00pfs_a::Pidr,
            P00PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p00pfs_a::Pdr,
        p00pfs_a::Pdr,
        P00PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p00pfs_a::Pdr,
            p00pfs_a::Pdr,
            P00PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits select the peripheral function. For individual pin functions, see the associated tables in this chapter."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, P00PfsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,P00PfsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pin mode control"]
    #[inline(always)]
    pub fn pmc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p00pfs_a::Pmc,
        p00pfs_a::Pmc,
        P00PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p00pfs_a::Pmc,
            p00pfs_a::Pmc,
            P00PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P00PfsA {
    #[inline(always)]
    fn default() -> P00PfsA {
        <crate::RegValueT<P00PfsA_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod p00pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmc_SPEC;
    pub type Pmc = crate::EnumBitfieldStruct<u8, Pmc_SPEC>;
    impl Pmc {
        #[doc = "Digital I/O"]
        pub const _0: Self = Self::new(0);

        #[doc = "Analog input function. Input to the input buffer is disable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0PfsA_SPEC;
impl crate::sealed::RegSpec for P0PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 0%s Pin Function Select Register"]
pub type P0PfsA = crate::RegValueT<P0PfsA_SPEC>;

impl P0PfsA {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p0pfs_a::Podr,
        p0pfs_a::Podr,
        P0PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p0pfs_a::Podr,
            p0pfs_a::Podr,
            P0PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p0pfs_a::Pidr,
        p0pfs_a::Pidr,
        P0PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p0pfs_a::Pidr,
            p0pfs_a::Pidr,
            P0PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p0pfs_a::Pdr,
        p0pfs_a::Pdr,
        P0PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p0pfs_a::Pdr,
            p0pfs_a::Pdr,
            P0PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits select the peripheral function. For individual pin functions, see the associated tables in this chapter."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, P0PfsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,P0PfsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pin mode control"]
    #[inline(always)]
    pub fn pmc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p0pfs_a::Pmc,
        p0pfs_a::Pmc,
        P0PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p0pfs_a::Pmc,
            p0pfs_a::Pmc,
            P0PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0PfsA {
    #[inline(always)]
    fn default() -> P0PfsA {
        <crate::RegValueT<P0PfsA_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod p0pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmc_SPEC;
    pub type Pmc = crate::EnumBitfieldStruct<u8, Pmc_SPEC>;
    impl Pmc {
        #[doc = "Digital I/O"]
        pub const _0: Self = Self::new(0);

        #[doc = "Analog input function. Input to the input buffer is disable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P015PfsA_SPEC;
impl crate::sealed::RegSpec for P015PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 015 Pin Function Select Register"]
pub type P015PfsA = crate::RegValueT<P015PfsA_SPEC>;

impl P015PfsA {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p015pfs_a::Podr,
        p015pfs_a::Podr,
        P015PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p015pfs_a::Podr,
            p015pfs_a::Podr,
            P015PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p015pfs_a::Pidr,
        p015pfs_a::Pidr,
        P015PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p015pfs_a::Pidr,
            p015pfs_a::Pidr,
            P015PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p015pfs_a::Pdr,
        p015pfs_a::Pdr,
        P015PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p015pfs_a::Pdr,
            p015pfs_a::Pdr,
            P015PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits select the peripheral function. For individual pin functions, see the associated tables in this chapter."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, P015PfsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,P015PfsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p015pfs_a::Isel,
        p015pfs_a::Isel,
        P015PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p015pfs_a::Isel,
            p015pfs_a::Isel,
            P015PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pin mode control"]
    #[inline(always)]
    pub fn pmc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p015pfs_a::Pmc,
        p015pfs_a::Pmc,
        P015PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p015pfs_a::Pmc,
            p015pfs_a::Pmc,
            P015PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P015PfsA {
    #[inline(always)]
    fn default() -> P015PfsA {
        <crate::RegValueT<P015PfsA_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod p015pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Notused as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmc_SPEC;
    pub type Pmc = crate::EnumBitfieldStruct<u8, Pmc_SPEC>;
    impl Pmc {
        #[doc = "Digital I/O"]
        pub const _0: Self = Self::new(0);

        #[doc = "Analog input function. Input to the input buffer is disable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10PfsA_SPEC;
impl crate::sealed::RegSpec for P10PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 10%s Pin Function Select Register"]
pub type P10PfsA = crate::RegValueT<P10PfsA_SPEC>;

impl P10PfsA {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p10pfs_a::Podr,
        p10pfs_a::Podr,
        P10PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p10pfs_a::Podr,
            p10pfs_a::Podr,
            P10PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p10pfs_a::Pidr,
        p10pfs_a::Pidr,
        P10PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p10pfs_a::Pidr,
            p10pfs_a::Pidr,
            P10PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p10pfs_a::Pdr,
        p10pfs_a::Pdr,
        P10PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p10pfs_a::Pdr,
            p10pfs_a::Pdr,
            P10PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p10pfs_a::Pcr,
        p10pfs_a::Pcr,
        P10PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p10pfs_a::Pcr,
            p10pfs_a::Pcr,
            P10PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pim(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p10pfs_a::Pim,
        p10pfs_a::Pim,
        P10PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p10pfs_a::Pim,
            p10pfs_a::Pim,
            P10PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p10pfs_a::Ncodr,
        p10pfs_a::Ncodr,
        P10PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p10pfs_a::Ncodr,
            p10pfs_a::Ncodr,
            P10PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits select the peripheral function. For individual pin functions, see the associated tables in this chapter."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, P10PfsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,P10PfsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p10pfs_a::Isel,
        p10pfs_a::Isel,
        P10PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p10pfs_a::Isel,
            p10pfs_a::Isel,
            P10PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pin mode control"]
    #[inline(always)]
    pub fn pmc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p10pfs_a::Pmc,
        p10pfs_a::Pmc,
        P10PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p10pfs_a::Pmc,
            p10pfs_a::Pmc,
            P10PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P10PfsA {
    #[inline(always)]
    fn default() -> P10PfsA {
        <crate::RegValueT<P10PfsA_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p10pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disableinput pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enableinput pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pim_SPEC;
    pub type Pim = crate::EnumBitfieldStruct<u8, Pim_SPEC>;
    impl Pim {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Notused as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmc_SPEC;
    pub type Pmc = crate::EnumBitfieldStruct<u8, Pmc_SPEC>;
    impl Pmc {
        #[doc = "Digital I/O"]
        pub const _0: Self = Self::new(0);

        #[doc = "Analog input function. Input to the input buffer is disable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P108PfsA_SPEC;
impl crate::sealed::RegSpec for P108PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 108 Pin Function Select Register"]
pub type P108PfsA = crate::RegValueT<P108PfsA_SPEC>;

impl P108PfsA {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p108pfs_a::Podr,
        p108pfs_a::Podr,
        P108PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p108pfs_a::Podr,
            p108pfs_a::Podr,
            P108PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p108pfs_a::Pidr,
        p108pfs_a::Pidr,
        P108PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p108pfs_a::Pidr,
            p108pfs_a::Pidr,
            P108PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p108pfs_a::Pdr,
        p108pfs_a::Pdr,
        P108PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p108pfs_a::Pdr,
            p108pfs_a::Pdr,
            P108PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p108pfs_a::Pcr,
        p108pfs_a::Pcr,
        P108PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p108pfs_a::Pcr,
            p108pfs_a::Pcr,
            P108PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pim(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p108pfs_a::Pim,
        p108pfs_a::Pim,
        P108PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p108pfs_a::Pim,
            p108pfs_a::Pim,
            P108PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits select the peripheral function. For individual pin functions, see the associated tables in this chapter."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, P108PfsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,P108PfsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pin mode control"]
    #[inline(always)]
    pub fn pmc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p108pfs_a::Pmc,
        p108pfs_a::Pmc,
        P108PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p108pfs_a::Pmc,
            p108pfs_a::Pmc,
            P108PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P108PfsA {
    #[inline(always)]
    fn default() -> P108PfsA {
        <crate::RegValueT<P108PfsA_SPEC> as RegisterValue<_>>::new(272)
    }
}
pub mod p108pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disableinput pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enableinput pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pim_SPEC;
    pub type Pim = crate::EnumBitfieldStruct<u8, Pim_SPEC>;
    impl Pim {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmc_SPEC;
    pub type Pmc = crate::EnumBitfieldStruct<u8, Pmc_SPEC>;
    impl Pmc {
        #[doc = "Digital I/O"]
        pub const _0: Self = Self::new(0);

        #[doc = "Analog input function. Input to the input buffer is disable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P109PfsA_SPEC;
impl crate::sealed::RegSpec for P109PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 109 Pin Function Select Register"]
pub type P109PfsA = crate::RegValueT<P109PfsA_SPEC>;

impl P109PfsA {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p109pfs_a::Podr,
        p109pfs_a::Podr,
        P109PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p109pfs_a::Podr,
            p109pfs_a::Podr,
            P109PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p109pfs_a::Pidr,
        p109pfs_a::Pidr,
        P109PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p109pfs_a::Pidr,
            p109pfs_a::Pidr,
            P109PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p109pfs_a::Pdr,
        p109pfs_a::Pdr,
        P109PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p109pfs_a::Pdr,
            p109pfs_a::Pdr,
            P109PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p109pfs_a::Pcr,
        p109pfs_a::Pcr,
        P109PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p109pfs_a::Pcr,
            p109pfs_a::Pcr,
            P109PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pim(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p109pfs_a::Pim,
        p109pfs_a::Pim,
        P109PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p109pfs_a::Pim,
            p109pfs_a::Pim,
            P109PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p109pfs_a::Ncodr,
        p109pfs_a::Ncodr,
        P109PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p109pfs_a::Ncodr,
            p109pfs_a::Ncodr,
            P109PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits select the peripheral function. For individual pin functions, see the associated tables in this chapter."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, P109PfsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,P109PfsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p109pfs_a::Isel,
        p109pfs_a::Isel,
        P109PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p109pfs_a::Isel,
            p109pfs_a::Isel,
            P109PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pin mode control"]
    #[inline(always)]
    pub fn pmc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p109pfs_a::Pmc,
        p109pfs_a::Pmc,
        P109PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p109pfs_a::Pmc,
            p109pfs_a::Pmc,
            P109PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P109PfsA {
    #[inline(always)]
    fn default() -> P109PfsA {
        <crate::RegValueT<P109PfsA_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p109pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disableinput pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enableinput pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pim_SPEC;
    pub type Pim = crate::EnumBitfieldStruct<u8, Pim_SPEC>;
    impl Pim {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Notused as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmc_SPEC;
    pub type Pmc = crate::EnumBitfieldStruct<u8, Pmc_SPEC>;
    impl Pmc {
        #[doc = "Digital I/O"]
        pub const _0: Self = Self::new(0);

        #[doc = "Analog input function. Input to the input buffer is disable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P110PfsA_SPEC;
impl crate::sealed::RegSpec for P110PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 110 Pin Function Select Register"]
pub type P110PfsA = crate::RegValueT<P110PfsA_SPEC>;

impl P110PfsA {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p110pfs_a::Podr,
        p110pfs_a::Podr,
        P110PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p110pfs_a::Podr,
            p110pfs_a::Podr,
            P110PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p110pfs_a::Pidr,
        p110pfs_a::Pidr,
        P110PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p110pfs_a::Pidr,
            p110pfs_a::Pidr,
            P110PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p110pfs_a::Pdr,
        p110pfs_a::Pdr,
        P110PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p110pfs_a::Pdr,
            p110pfs_a::Pdr,
            P110PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p110pfs_a::Pcr,
        p110pfs_a::Pcr,
        P110PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p110pfs_a::Pcr,
            p110pfs_a::Pcr,
            P110PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pim(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p110pfs_a::Pim,
        p110pfs_a::Pim,
        P110PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p110pfs_a::Pim,
            p110pfs_a::Pim,
            P110PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p110pfs_a::Ncodr,
        p110pfs_a::Ncodr,
        P110PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p110pfs_a::Ncodr,
            p110pfs_a::Ncodr,
            P110PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits select the peripheral function. For individual pin functions, see the associated tables in this chapter."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, P110PfsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,P110PfsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p110pfs_a::Isel,
        p110pfs_a::Isel,
        P110PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p110pfs_a::Isel,
            p110pfs_a::Isel,
            P110PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pin mode control"]
    #[inline(always)]
    pub fn pmc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p110pfs_a::Pmc,
        p110pfs_a::Pmc,
        P110PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p110pfs_a::Pmc,
            p110pfs_a::Pmc,
            P110PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P110PfsA {
    #[inline(always)]
    fn default() -> P110PfsA {
        <crate::RegValueT<P110PfsA_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p110pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disableinput pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enableinput pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pim_SPEC;
    pub type Pim = crate::EnumBitfieldStruct<u8, Pim_SPEC>;
    impl Pim {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Notused as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmc_SPEC;
    pub type Pmc = crate::EnumBitfieldStruct<u8, Pmc_SPEC>;
    impl Pmc {
        #[doc = "Digital I/O"]
        pub const _0: Self = Self::new(0);

        #[doc = "Analog input function. Input to the input buffer is disable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P112PfsA_SPEC;
impl crate::sealed::RegSpec for P112PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 112 Pin Function Select Register"]
pub type P112PfsA = crate::RegValueT<P112PfsA_SPEC>;

impl P112PfsA {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p112pfs_a::Podr,
        p112pfs_a::Podr,
        P112PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p112pfs_a::Podr,
            p112pfs_a::Podr,
            P112PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p112pfs_a::Pidr,
        p112pfs_a::Pidr,
        P112PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p112pfs_a::Pidr,
            p112pfs_a::Pidr,
            P112PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p112pfs_a::Pdr,
        p112pfs_a::Pdr,
        P112PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p112pfs_a::Pdr,
            p112pfs_a::Pdr,
            P112PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p112pfs_a::Pcr,
        p112pfs_a::Pcr,
        P112PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p112pfs_a::Pcr,
            p112pfs_a::Pcr,
            P112PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pim(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p112pfs_a::Pim,
        p112pfs_a::Pim,
        P112PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p112pfs_a::Pim,
            p112pfs_a::Pim,
            P112PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p112pfs_a::Ncodr,
        p112pfs_a::Ncodr,
        P112PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p112pfs_a::Ncodr,
            p112pfs_a::Ncodr,
            P112PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits select the peripheral function. For individual pin functions, see the associated tables in this chapter."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, P112PfsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,P112PfsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p112pfs_a::Isel,
        p112pfs_a::Isel,
        P112PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p112pfs_a::Isel,
            p112pfs_a::Isel,
            P112PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pin mode control"]
    #[inline(always)]
    pub fn pmc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p112pfs_a::Pmc,
        p112pfs_a::Pmc,
        P112PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p112pfs_a::Pmc,
            p112pfs_a::Pmc,
            P112PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P112PfsA {
    #[inline(always)]
    fn default() -> P112PfsA {
        <crate::RegValueT<P112PfsA_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p112pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disableinput pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enableinput pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pim_SPEC;
    pub type Pim = crate::EnumBitfieldStruct<u8, Pim_SPEC>;
    impl Pim {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Notused as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmc_SPEC;
    pub type Pmc = crate::EnumBitfieldStruct<u8, Pmc_SPEC>;
    impl Pmc {
        #[doc = "Digital I/O"]
        pub const _0: Self = Self::new(0);

        #[doc = "Analog input function. Input to the input buffer is disable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P200PfsA_SPEC;
impl crate::sealed::RegSpec for P200PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 200 Pin Function Select Register"]
pub type P200PfsA = crate::RegValueT<P200PfsA_SPEC>;

impl P200PfsA {
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p200pfs_a::Pidr,
        p200pfs_a::Pidr,
        P200PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p200pfs_a::Pidr,
            p200pfs_a::Pidr,
            P200PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p200pfs_a::Isel,
        p200pfs_a::Isel,
        P200PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p200pfs_a::Isel,
            p200pfs_a::Isel,
            P200PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pin mode control"]
    #[inline(always)]
    pub fn pmc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p200pfs_a::Pmc,
        p200pfs_a::Pmc,
        P200PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p200pfs_a::Pmc,
            p200pfs_a::Pmc,
            P200PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P200PfsA {
    #[inline(always)]
    fn default() -> P200PfsA {
        <crate::RegValueT<P200PfsA_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p200pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Notused as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmc_SPEC;
    pub type Pmc = crate::EnumBitfieldStruct<u8, Pmc_SPEC>;
    impl Pmc {
        #[doc = "Digital I/O"]
        pub const _0: Self = Self::new(0);

        #[doc = "Analog input function. Input to the input buffer is disable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201PfsA_SPEC;
impl crate::sealed::RegSpec for P201PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 201 Pin Function Select Register"]
pub type P201PfsA = crate::RegValueT<P201PfsA_SPEC>;

impl P201PfsA {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p201pfs_a::Podr,
        p201pfs_a::Podr,
        P201PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p201pfs_a::Podr,
            p201pfs_a::Podr,
            P201PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p201pfs_a::Pidr,
        p201pfs_a::Pidr,
        P201PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p201pfs_a::Pidr,
            p201pfs_a::Pidr,
            P201PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p201pfs_a::Pdr,
        p201pfs_a::Pdr,
        P201PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p201pfs_a::Pdr,
            p201pfs_a::Pdr,
            P201PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p201pfs_a::Pcr,
        p201pfs_a::Pcr,
        P201PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p201pfs_a::Pcr,
            p201pfs_a::Pcr,
            P201PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pim(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p201pfs_a::Pim,
        p201pfs_a::Pim,
        P201PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p201pfs_a::Pim,
            p201pfs_a::Pim,
            P201PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p201pfs_a::Ncodr,
        p201pfs_a::Ncodr,
        P201PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p201pfs_a::Ncodr,
            p201pfs_a::Ncodr,
            P201PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits select the peripheral function. For individual pin functions, see the associated tables in this chapter."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, P201PfsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,P201PfsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p201pfs_a::Isel,
        p201pfs_a::Isel,
        P201PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p201pfs_a::Isel,
            p201pfs_a::Isel,
            P201PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pin mode control"]
    #[inline(always)]
    pub fn pmc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p201pfs_a::Pmc,
        p201pfs_a::Pmc,
        P201PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p201pfs_a::Pmc,
            p201pfs_a::Pmc,
            P201PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P201PfsA {
    #[inline(always)]
    fn default() -> P201PfsA {
        <crate::RegValueT<P201PfsA_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p201pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disableinput pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enableinput pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pim_SPEC;
    pub type Pim = crate::EnumBitfieldStruct<u8, Pim_SPEC>;
    impl Pim {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Notused as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmc_SPEC;
    pub type Pmc = crate::EnumBitfieldStruct<u8, Pmc_SPEC>;
    impl Pmc {
        #[doc = "Digital I/O"]
        pub const _0: Self = Self::new(0);

        #[doc = "Analog input function. Input to the input buffer is disable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P206PfsA_SPEC;
impl crate::sealed::RegSpec for P206PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 206 Pin Function Select Register"]
pub type P206PfsA = crate::RegValueT<P206PfsA_SPEC>;

impl P206PfsA {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p206pfs_a::Podr,
        p206pfs_a::Podr,
        P206PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p206pfs_a::Podr,
            p206pfs_a::Podr,
            P206PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p206pfs_a::Pidr,
        p206pfs_a::Pidr,
        P206PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p206pfs_a::Pidr,
            p206pfs_a::Pidr,
            P206PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p206pfs_a::Pdr,
        p206pfs_a::Pdr,
        P206PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p206pfs_a::Pdr,
            p206pfs_a::Pdr,
            P206PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p206pfs_a::Pcr,
        p206pfs_a::Pcr,
        P206PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p206pfs_a::Pcr,
            p206pfs_a::Pcr,
            P206PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P206PfsA {
    #[inline(always)]
    fn default() -> P206PfsA {
        <crate::RegValueT<P206PfsA_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p206pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disableinput pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enableinput pull-up"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20PfsA_SPEC;
impl crate::sealed::RegSpec for P20PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 20%s Pin Function Select Register"]
pub type P20PfsA = crate::RegValueT<P20PfsA_SPEC>;

impl P20PfsA {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p20pfs_a::Podr,
        p20pfs_a::Podr,
        P20PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p20pfs_a::Podr,
            p20pfs_a::Podr,
            P20PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p20pfs_a::Pidr,
        p20pfs_a::Pidr,
        P20PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p20pfs_a::Pidr,
            p20pfs_a::Pidr,
            P20PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p20pfs_a::Pdr,
        p20pfs_a::Pdr,
        P20PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p20pfs_a::Pdr,
            p20pfs_a::Pdr,
            P20PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p20pfs_a::Pcr,
        p20pfs_a::Pcr,
        P20PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p20pfs_a::Pcr,
            p20pfs_a::Pcr,
            P20PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pim(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p20pfs_a::Pim,
        p20pfs_a::Pim,
        P20PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p20pfs_a::Pim,
            p20pfs_a::Pim,
            P20PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p20pfs_a::Ncodr,
        p20pfs_a::Ncodr,
        P20PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p20pfs_a::Ncodr,
            p20pfs_a::Ncodr,
            P20PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits select the peripheral function. For individual pin functions, see the associated tables in this chapter."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, P20PfsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,P20PfsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p20pfs_a::Isel,
        p20pfs_a::Isel,
        P20PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p20pfs_a::Isel,
            p20pfs_a::Isel,
            P20PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pin mode control"]
    #[inline(always)]
    pub fn pmc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p20pfs_a::Pmc,
        p20pfs_a::Pmc,
        P20PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p20pfs_a::Pmc,
            p20pfs_a::Pmc,
            P20PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P20PfsA {
    #[inline(always)]
    fn default() -> P20PfsA {
        <crate::RegValueT<P20PfsA_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p20pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disableinput pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enableinput pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pim_SPEC;
    pub type Pim = crate::EnumBitfieldStruct<u8, Pim_SPEC>;
    impl Pim {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Notused as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmc_SPEC;
    pub type Pmc = crate::EnumBitfieldStruct<u8, Pmc_SPEC>;
    impl Pmc {
        #[doc = "Digital I/O"]
        pub const _0: Self = Self::new(0);

        #[doc = "Analog input function. Input to the input buffer is disable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2PfsA_SPEC;
impl crate::sealed::RegSpec for P2PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 2%s Pin Function Select Register"]
pub type P2PfsA = crate::RegValueT<P2PfsA_SPEC>;

impl P2PfsA {
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p2pfs_a::Pidr,
        p2pfs_a::Pidr,
        P2PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p2pfs_a::Pidr,
            p2pfs_a::Pidr,
            P2PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2PfsA {
    #[inline(always)]
    fn default() -> P2PfsA {
        <crate::RegValueT<P2PfsA_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p2pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);

        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P300PfsA_SPEC;
impl crate::sealed::RegSpec for P300PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 300 Pin Function Select Register"]
pub type P300PfsA = crate::RegValueT<P300PfsA_SPEC>;

impl P300PfsA {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p300pfs_a::Podr,
        p300pfs_a::Podr,
        P300PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p300pfs_a::Podr,
            p300pfs_a::Podr,
            P300PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p300pfs_a::Pidr,
        p300pfs_a::Pidr,
        P300PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p300pfs_a::Pidr,
            p300pfs_a::Pidr,
            P300PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p300pfs_a::Pdr,
        p300pfs_a::Pdr,
        P300PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p300pfs_a::Pdr,
            p300pfs_a::Pdr,
            P300PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p300pfs_a::Pcr,
        p300pfs_a::Pcr,
        P300PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p300pfs_a::Pcr,
            p300pfs_a::Pcr,
            P300PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pim(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p300pfs_a::Pim,
        p300pfs_a::Pim,
        P300PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p300pfs_a::Pim,
            p300pfs_a::Pim,
            P300PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits select the peripheral function. For individual pin functions, see the associated tables in this chapter."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, P300PfsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,P300PfsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pin mode control"]
    #[inline(always)]
    pub fn pmc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p300pfs_a::Pmc,
        p300pfs_a::Pmc,
        P300PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p300pfs_a::Pmc,
            p300pfs_a::Pmc,
            P300PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P300PfsA {
    #[inline(always)]
    fn default() -> P300PfsA {
        <crate::RegValueT<P300PfsA_SPEC> as RegisterValue<_>>::new(272)
    }
}
pub mod p300pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disableinput pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enableinput pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pim_SPEC;
    pub type Pim = crate::EnumBitfieldStruct<u8, Pim_SPEC>;
    impl Pim {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmc_SPEC;
    pub type Pmc = crate::EnumBitfieldStruct<u8, Pmc_SPEC>;
    impl Pmc {
        #[doc = "Digital I/O"]
        pub const _0: Self = Self::new(0);

        #[doc = "Analog input function. Input to the input buffer is disable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P407PfsA_SPEC;
impl crate::sealed::RegSpec for P407PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 407 Pin Function Select Register"]
pub type P407PfsA = crate::RegValueT<P407PfsA_SPEC>;

impl P407PfsA {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p407pfs_a::Podr,
        p407pfs_a::Podr,
        P407PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p407pfs_a::Podr,
            p407pfs_a::Podr,
            P407PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p407pfs_a::Pidr,
        p407pfs_a::Pidr,
        P407PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p407pfs_a::Pidr,
            p407pfs_a::Pidr,
            P407PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p407pfs_a::Pdr,
        p407pfs_a::Pdr,
        P407PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p407pfs_a::Pdr,
            p407pfs_a::Pdr,
            P407PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p407pfs_a::Pcr,
        p407pfs_a::Pcr,
        P407PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p407pfs_a::Pcr,
            p407pfs_a::Pcr,
            P407PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pim(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p407pfs_a::Pim,
        p407pfs_a::Pim,
        P407PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p407pfs_a::Pim,
            p407pfs_a::Pim,
            P407PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p407pfs_a::Ncodr,
        p407pfs_a::Ncodr,
        P407PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p407pfs_a::Ncodr,
            p407pfs_a::Ncodr,
            P407PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits select the peripheral function. For individual pin functions, see the associated tables in this chapter."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, P407PfsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,P407PfsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p407pfs_a::Isel,
        p407pfs_a::Isel,
        P407PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p407pfs_a::Isel,
            p407pfs_a::Isel,
            P407PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pin mode control"]
    #[inline(always)]
    pub fn pmc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p407pfs_a::Pmc,
        p407pfs_a::Pmc,
        P407PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p407pfs_a::Pmc,
            p407pfs_a::Pmc,
            P407PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P407PfsA {
    #[inline(always)]
    fn default() -> P407PfsA {
        <crate::RegValueT<P407PfsA_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p407pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disableinput pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enableinput pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pim_SPEC;
    pub type Pim = crate::EnumBitfieldStruct<u8, Pim_SPEC>;
    impl Pim {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Notused as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmc_SPEC;
    pub type Pmc = crate::EnumBitfieldStruct<u8, Pmc_SPEC>;
    impl Pmc {
        #[doc = "Digital I/O"]
        pub const _0: Self = Self::new(0);

        #[doc = "Analog input function. Input to the input buffer is disable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P9PfsA_SPEC;
impl crate::sealed::RegSpec for P9PfsA_SPEC {
    type DataType = u16;
}

#[doc = "Port 9%s Pin Function Select Register"]
pub type P9PfsA = crate::RegValueT<P9PfsA_SPEC>;

impl P9PfsA {
    #[doc = "P9n Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p9pfs_a::Podr,
        p9pfs_a::Podr,
        P9PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p9pfs_a::Podr,
            p9pfs_a::Podr,
            P9PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P9n State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p9pfs_a::Pidr,
        p9pfs_a::Pidr,
        P9PfsA_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p9pfs_a::Pidr,
            p9pfs_a::Pidr,
            P9PfsA_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "P9n Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p9pfs_a::Pdr,
        p9pfs_a::Pdr,
        P9PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p9pfs_a::Pdr,
            p9pfs_a::Pdr,
            P9PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, P9PfsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,P9PfsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pin Mode Control"]
    #[inline(always)]
    pub fn pmc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p9pfs_a::Pmc,
        p9pfs_a::Pmc,
        P9PfsA_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p9pfs_a::Pmc,
            p9pfs_a::Pmc,
            P9PfsA_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P9PfsA {
    #[inline(always)]
    fn default() -> P9PfsA {
        <crate::RegValueT<P9PfsA_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p9pfs_a {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmc_SPEC;
    pub type Pmc = crate::EnumBitfieldStruct<u8, Pmc_SPEC>;
    impl Pmc {
        #[doc = "Digital I/O"]
        pub const _0: Self = Self::new(0);

        #[doc = "Analog input function. Input to the input buffer is disable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwpr_SPEC;
impl crate::sealed::RegSpec for Pwpr_SPEC {
    type DataType = u8;
}

#[doc = "Write-Protect Register"]
pub type Pwpr = crate::RegValueT<Pwpr_SPEC>;

impl Pwpr {
    #[doc = "PmnPFS_A Register Write Enable"]
    #[inline(always)]
    pub fn pfswe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pwpr::Pfswe,
        pwpr::Pfswe,
        Pwpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pwpr::Pfswe,
            pwpr::Pfswe,
            Pwpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFSWE Bit Write Disable"]
    #[inline(always)]
    pub fn b0wi(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pwpr::B0Wi,
        pwpr::B0Wi,
        Pwpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pwpr::B0Wi,
            pwpr::B0Wi,
            Pwpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwpr {
    #[inline(always)]
    fn default() -> Pwpr {
        <crate::RegValueT<Pwpr_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod pwpr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfswe_SPEC;
    pub type Pfswe = crate::EnumBitfieldStruct<u8, Pfswe_SPEC>;
    impl Pfswe {
        #[doc = "Writing to the PmnPFS_A register is disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing to the PmnPFS_A register is enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct B0Wi_SPEC;
    pub type B0Wi = crate::EnumBitfieldStruct<u8, B0Wi_SPEC>;
    impl B0Wi {
        #[doc = "Writing to the PFSWE bit is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing to the PFSWE bit is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
