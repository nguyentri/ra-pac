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
// Generated from SVD 1.20.02, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:01:00 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"BUS Control"]
unsafe impl ::core::marker::Send for super::Bus {}
unsafe impl ::core::marker::Sync for super::Bus {}
impl super::Bus {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Master Bus Control Register SYS"]
    #[inline(always)]
    pub const fn busmcntsys(
        &self,
    ) -> &'static crate::common::Reg<self::Busmcntsys_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmcntsys_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4104usize),
            )
        }
    }

    #[doc = "Master Bus Control Register DMA"]
    #[inline(always)]
    pub const fn busmcntdma(
        &self,
    ) -> &'static crate::common::Reg<self::Busmcntdma_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmcntdma_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4108usize),
            )
        }
    }

    #[doc = "Bus Error Address Register 3"]
    #[inline(always)]
    pub const fn bus3erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Bus3Erradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bus3Erradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6176usize),
            )
        }
    }

    #[doc = "BUS Error Status Register 3"]
    #[inline(always)]
    pub const fn bus3errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Bus3Errstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bus3Errstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6180usize),
            )
        }
    }

    #[doc = "Bus Error Address Register 4"]
    #[inline(always)]
    pub const fn bus4erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Bus4Erradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bus4Erradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6192usize),
            )
        }
    }

    #[doc = "BUS Error Status Register 4"]
    #[inline(always)]
    pub const fn bus4errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Bus4Errstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bus4Errstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6196usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busmcntsys_SPEC;
impl crate::sealed::RegSpec for Busmcntsys_SPEC {
    type DataType = u16;
}

#[doc = "Master Bus Control Register SYS"]
pub type Busmcntsys = crate::RegValueT<Busmcntsys_SPEC>;

impl Busmcntsys {
    #[doc = "Ignore Error Responses"]
    #[inline(always)]
    pub fn ieres(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        busmcntsys::Ieres,
        busmcntsys::Ieres,
        Busmcntsys_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            busmcntsys::Ieres,
            busmcntsys::Ieres,
            Busmcntsys_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busmcntsys {
    #[inline(always)]
    fn default() -> Busmcntsys {
        <crate::RegValueT<Busmcntsys_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busmcntsys {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ieres_SPEC;
    pub type Ieres = crate::EnumBitfieldStruct<u8, Ieres_SPEC>;
    impl Ieres {
        #[doc = "A bus error is reported"]
        pub const _0: Self = Self::new(0);

        #[doc = "A bus error is not reported"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busmcntdma_SPEC;
impl crate::sealed::RegSpec for Busmcntdma_SPEC {
    type DataType = u16;
}

#[doc = "Master Bus Control Register DMA"]
pub type Busmcntdma = crate::RegValueT<Busmcntdma_SPEC>;

impl Busmcntdma {
    #[doc = "Ignore Error Responses"]
    #[inline(always)]
    pub fn ieres(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        busmcntdma::Ieres,
        busmcntdma::Ieres,
        Busmcntdma_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            busmcntdma::Ieres,
            busmcntdma::Ieres,
            Busmcntdma_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busmcntdma {
    #[inline(always)]
    fn default() -> Busmcntdma {
        <crate::RegValueT<Busmcntdma_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busmcntdma {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ieres_SPEC;
    pub type Ieres = crate::EnumBitfieldStruct<u8, Ieres_SPEC>;
    impl Ieres {
        #[doc = "A bus error is reported"]
        pub const _0: Self = Self::new(0);

        #[doc = "A bus error is not reported"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bus3Erradd_SPEC;
impl crate::sealed::RegSpec for Bus3Erradd_SPEC {
    type DataType = u32;
}

#[doc = "Bus Error Address Register 3"]
pub type Bus3Erradd = crate::RegValueT<Bus3Erradd_SPEC>;

impl Bus3Erradd {
    #[doc = "Bus Error Address"]
    #[inline(always)]
    pub fn berad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Bus3Erradd_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Bus3Erradd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bus3Erradd {
    #[inline(always)]
    fn default() -> Bus3Erradd {
        <crate::RegValueT<Bus3Erradd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bus3Errstat_SPEC;
impl crate::sealed::RegSpec for Bus3Errstat_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Status Register 3"]
pub type Bus3Errstat = crate::RegValueT<Bus3Errstat_SPEC>;

impl Bus3Errstat {
    #[doc = "Error Access Status flag"]
    #[inline(always)]
    pub fn accstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bus3errstat::Accstat,
        bus3errstat::Accstat,
        Bus3Errstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bus3errstat::Accstat,
            bus3errstat::Accstat,
            Bus3Errstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Bus Error Status flag"]
    #[inline(always)]
    pub fn errstat(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        bus3errstat::Errstat,
        bus3errstat::Errstat,
        Bus3Errstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            bus3errstat::Errstat,
            bus3errstat::Errstat,
            Bus3Errstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bus3Errstat {
    #[inline(always)]
    fn default() -> Bus3Errstat {
        <crate::RegValueT<Bus3Errstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bus3errstat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Accstat_SPEC;
    pub type Accstat = crate::EnumBitfieldStruct<u8, Accstat_SPEC>;
    impl Accstat {
        #[doc = "Read access"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write access"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Errstat_SPEC;
    pub type Errstat = crate::EnumBitfieldStruct<u8, Errstat_SPEC>;
    impl Errstat {
        #[doc = "No bus error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus error occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bus4Erradd_SPEC;
impl crate::sealed::RegSpec for Bus4Erradd_SPEC {
    type DataType = u32;
}

#[doc = "Bus Error Address Register 4"]
pub type Bus4Erradd = crate::RegValueT<Bus4Erradd_SPEC>;

impl Bus4Erradd {
    #[doc = "Bus Error Address"]
    #[inline(always)]
    pub fn berad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Bus4Erradd_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Bus4Erradd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bus4Erradd {
    #[inline(always)]
    fn default() -> Bus4Erradd {
        <crate::RegValueT<Bus4Erradd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bus4Errstat_SPEC;
impl crate::sealed::RegSpec for Bus4Errstat_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Status Register 4"]
pub type Bus4Errstat = crate::RegValueT<Bus4Errstat_SPEC>;

impl Bus4Errstat {
    #[doc = "Error Access Status flag"]
    #[inline(always)]
    pub fn accstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bus4errstat::Accstat,
        bus4errstat::Accstat,
        Bus4Errstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bus4errstat::Accstat,
            bus4errstat::Accstat,
            Bus4Errstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Bus Error Status flag"]
    #[inline(always)]
    pub fn errstat(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        bus4errstat::Errstat,
        bus4errstat::Errstat,
        Bus4Errstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            bus4errstat::Errstat,
            bus4errstat::Errstat,
            Bus4Errstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bus4Errstat {
    #[inline(always)]
    fn default() -> Bus4Errstat {
        <crate::RegValueT<Bus4Errstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bus4errstat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Accstat_SPEC;
    pub type Accstat = crate::EnumBitfieldStruct<u8, Accstat_SPEC>;
    impl Accstat {
        #[doc = "Read access"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write access"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Errstat_SPEC;
    pub type Errstat = crate::EnumBitfieldStruct<u8, Errstat_SPEC>;
    impl Errstat {
        #[doc = "No bus error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus error occurred"]
        pub const _1: Self = Self::new(1);
    }
}
