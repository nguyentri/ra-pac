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
// Generated from SVD 1.10.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:47 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Serial Interface IICA"]
unsafe impl ::core::marker::Send for super::Iica {}
unsafe impl ::core::marker::Sync for super::Iica {}
impl super::Iica {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn iica0(&self) -> &'static crate::common::Reg<self::Iica0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iica0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iics0(&self) -> &'static crate::common::Reg<self::Iics0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iics0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iicf0(&self) -> &'static crate::common::Reg<self::Iicf0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iicf0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iicctl00(
        &self,
    ) -> &'static crate::common::Reg<self::Iicctl00_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iicctl00_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iicctl01(
        &self,
    ) -> &'static crate::common::Reg<self::Iicctl01_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iicctl01_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(257usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iicwl0(
        &self,
    ) -> &'static crate::common::Reg<self::Iicwl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iicwl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(258usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iicwh0(
        &self,
    ) -> &'static crate::common::Reg<self::Iicwh0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iicwh0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(259usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sva0(&self) -> &'static crate::common::Reg<self::Sva0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sva0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iica0_SPEC;
impl crate::sealed::RegSpec for Iica0_SPEC {
    type DataType = u8;
}

pub type Iica0 = crate::RegValueT<Iica0_SPEC>;

impl NoBitfieldReg<Iica0_SPEC> for Iica0 {}
impl ::core::default::Default for Iica0 {
    #[inline(always)]
    fn default() -> Iica0 {
        <crate::RegValueT<Iica0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iics0_SPEC;
impl crate::sealed::RegSpec for Iics0_SPEC {
    type DataType = u8;
}

pub type Iics0 = crate::RegValueT<Iics0_SPEC>;

impl Iics0 {
    #[inline(always)]
    pub fn spd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iics0::Spd,
        iics0::Spd,
        Iics0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iics0::Spd,
            iics0::Spd,
            Iics0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn std(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iics0::Std,
        iics0::Std,
        Iics0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iics0::Std,
            iics0::Std,
            Iics0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ackd(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        iics0::Ackd,
        iics0::Ackd,
        Iics0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            iics0::Ackd,
            iics0::Ackd,
            Iics0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trc(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iics0::Trc,
        iics0::Trc,
        Iics0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iics0::Trc,
            iics0::Trc,
            Iics0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn coi(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        iics0::Coi,
        iics0::Coi,
        Iics0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            iics0::Coi,
            iics0::Coi,
            Iics0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn exc(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        iics0::Exc,
        iics0::Exc,
        Iics0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            iics0::Exc,
            iics0::Exc,
            Iics0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ald(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        iics0::Ald,
        iics0::Ald,
        Iics0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            iics0::Ald,
            iics0::Ald,
            Iics0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn msts(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        iics0::Msts,
        iics0::Msts,
        Iics0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            iics0::Msts,
            iics0::Msts,
            Iics0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iics0 {
    #[inline(always)]
    fn default() -> Iics0 {
        <crate::RegValueT<Iics0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iics0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spd_SPEC;
    pub type Spd = crate::EnumBitfieldStruct<u8, Spd_SPEC>;
    impl Spd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Std_SPEC;
    pub type Std = crate::EnumBitfieldStruct<u8, Std_SPEC>;
    impl Std {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackd_SPEC;
    pub type Ackd = crate::EnumBitfieldStruct<u8, Ackd_SPEC>;
    impl Ackd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trc_SPEC;
    pub type Trc = crate::EnumBitfieldStruct<u8, Trc_SPEC>;
    impl Trc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Coi_SPEC;
    pub type Coi = crate::EnumBitfieldStruct<u8, Coi_SPEC>;
    impl Coi {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exc_SPEC;
    pub type Exc = crate::EnumBitfieldStruct<u8, Exc_SPEC>;
    impl Exc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ald_SPEC;
    pub type Ald = crate::EnumBitfieldStruct<u8, Ald_SPEC>;
    impl Ald {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Msts_SPEC;
    pub type Msts = crate::EnumBitfieldStruct<u8, Msts_SPEC>;
    impl Msts {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iicf0_SPEC;
impl crate::sealed::RegSpec for Iicf0_SPEC {
    type DataType = u8;
}

pub type Iicf0 = crate::RegValueT<Iicf0_SPEC>;

impl Iicf0 {
    #[inline(always)]
    pub fn iicrsv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iicf0::Iicrsv,
        iicf0::Iicrsv,
        Iicf0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iicf0::Iicrsv,
            iicf0::Iicrsv,
            Iicf0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn stcen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iicf0::Stcen,
        iicf0::Stcen,
        Iicf0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iicf0::Stcen,
            iicf0::Stcen,
            Iicf0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicbsy(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        iicf0::Iicbsy,
        iicf0::Iicbsy,
        Iicf0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            iicf0::Iicbsy,
            iicf0::Iicbsy,
            Iicf0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn stcf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        iicf0::Stcf,
        iicf0::Stcf,
        Iicf0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            iicf0::Stcf,
            iicf0::Stcf,
            Iicf0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iicf0 {
    #[inline(always)]
    fn default() -> Iicf0 {
        <crate::RegValueT<Iicf0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iicf0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicrsv_SPEC;
    pub type Iicrsv = crate::EnumBitfieldStruct<u8, Iicrsv_SPEC>;
    impl Iicrsv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stcen_SPEC;
    pub type Stcen = crate::EnumBitfieldStruct<u8, Stcen_SPEC>;
    impl Stcen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicbsy_SPEC;
    pub type Iicbsy = crate::EnumBitfieldStruct<u8, Iicbsy_SPEC>;
    impl Iicbsy {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stcf_SPEC;
    pub type Stcf = crate::EnumBitfieldStruct<u8, Stcf_SPEC>;
    impl Stcf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iicctl00_SPEC;
impl crate::sealed::RegSpec for Iicctl00_SPEC {
    type DataType = u8;
}

pub type Iicctl00 = crate::RegValueT<Iicctl00_SPEC>;

impl Iicctl00 {
    #[inline(always)]
    pub fn spt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iicctl00::Spt,
        iicctl00::Spt,
        Iicctl00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iicctl00::Spt,
            iicctl00::Spt,
            Iicctl00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn stt(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iicctl00::Stt,
        iicctl00::Stt,
        Iicctl00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iicctl00::Stt,
            iicctl00::Stt,
            Iicctl00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn acke(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        iicctl00::Acke,
        iicctl00::Acke,
        Iicctl00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            iicctl00::Acke,
            iicctl00::Acke,
            Iicctl00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wtim(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iicctl00::Wtim,
        iicctl00::Wtim,
        Iicctl00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iicctl00::Wtim,
            iicctl00::Wtim,
            Iicctl00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        iicctl00::Spie,
        iicctl00::Spie,
        Iicctl00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            iicctl00::Spie,
            iicctl00::Spie,
            Iicctl00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wrel(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        iicctl00::Wrel,
        iicctl00::Wrel,
        Iicctl00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            iicctl00::Wrel,
            iicctl00::Wrel,
            Iicctl00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lrel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        iicctl00::Lrel,
        iicctl00::Lrel,
        Iicctl00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            iicctl00::Lrel,
            iicctl00::Lrel,
            Iicctl00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iice(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        iicctl00::Iice,
        iicctl00::Iice,
        Iicctl00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            iicctl00::Iice,
            iicctl00::Iice,
            Iicctl00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iicctl00 {
    #[inline(always)]
    fn default() -> Iicctl00 {
        <crate::RegValueT<Iicctl00_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iicctl00 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spt_SPEC;
    pub type Spt = crate::EnumBitfieldStruct<u8, Spt_SPEC>;
    impl Spt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stt_SPEC;
    pub type Stt = crate::EnumBitfieldStruct<u8, Stt_SPEC>;
    impl Stt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acke_SPEC;
    pub type Acke = crate::EnumBitfieldStruct<u8, Acke_SPEC>;
    impl Acke {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wtim_SPEC;
    pub type Wtim = crate::EnumBitfieldStruct<u8, Wtim_SPEC>;
    impl Wtim {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spie_SPEC;
    pub type Spie = crate::EnumBitfieldStruct<u8, Spie_SPEC>;
    impl Spie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrel_SPEC;
    pub type Wrel = crate::EnumBitfieldStruct<u8, Wrel_SPEC>;
    impl Wrel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lrel_SPEC;
    pub type Lrel = crate::EnumBitfieldStruct<u8, Lrel_SPEC>;
    impl Lrel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iice_SPEC;
    pub type Iice = crate::EnumBitfieldStruct<u8, Iice_SPEC>;
    impl Iice {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iicctl01_SPEC;
impl crate::sealed::RegSpec for Iicctl01_SPEC {
    type DataType = u8;
}

pub type Iicctl01 = crate::RegValueT<Iicctl01_SPEC>;

impl Iicctl01 {
    #[inline(always)]
    pub fn prs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iicctl01::Prs,
        iicctl01::Prs,
        Iicctl01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iicctl01::Prs,
            iicctl01::Prs,
            Iicctl01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dfc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        iicctl01::Dfc,
        iicctl01::Dfc,
        Iicctl01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            iicctl01::Dfc,
            iicctl01::Dfc,
            Iicctl01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn smc(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iicctl01::Smc,
        iicctl01::Smc,
        Iicctl01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iicctl01::Smc,
            iicctl01::Smc,
            Iicctl01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dad(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        iicctl01::Dad,
        iicctl01::Dad,
        Iicctl01_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            iicctl01::Dad,
            iicctl01::Dad,
            Iicctl01_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cld(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        iicctl01::Cld,
        iicctl01::Cld,
        Iicctl01_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            iicctl01::Cld,
            iicctl01::Cld,
            Iicctl01_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn svadis(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        iicctl01::Svadis,
        iicctl01::Svadis,
        Iicctl01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            iicctl01::Svadis,
            iicctl01::Svadis,
            Iicctl01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wup(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        iicctl01::Wup,
        iicctl01::Wup,
        Iicctl01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            iicctl01::Wup,
            iicctl01::Wup,
            Iicctl01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iicctl01 {
    #[inline(always)]
    fn default() -> Iicctl01 {
        <crate::RegValueT<Iicctl01_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iicctl01 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prs_SPEC;
    pub type Prs = crate::EnumBitfieldStruct<u8, Prs_SPEC>;
    impl Prs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfc_SPEC;
    pub type Dfc = crate::EnumBitfieldStruct<u8, Dfc_SPEC>;
    impl Dfc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smc_SPEC;
    pub type Smc = crate::EnumBitfieldStruct<u8, Smc_SPEC>;
    impl Smc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dad_SPEC;
    pub type Dad = crate::EnumBitfieldStruct<u8, Dad_SPEC>;
    impl Dad {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cld_SPEC;
    pub type Cld = crate::EnumBitfieldStruct<u8, Cld_SPEC>;
    impl Cld {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svadis_SPEC;
    pub type Svadis = crate::EnumBitfieldStruct<u8, Svadis_SPEC>;
    impl Svadis {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wup_SPEC;
    pub type Wup = crate::EnumBitfieldStruct<u8, Wup_SPEC>;
    impl Wup {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iicwl0_SPEC;
impl crate::sealed::RegSpec for Iicwl0_SPEC {
    type DataType = u8;
}

pub type Iicwl0 = crate::RegValueT<Iicwl0_SPEC>;

impl NoBitfieldReg<Iicwl0_SPEC> for Iicwl0 {}
impl ::core::default::Default for Iicwl0 {
    #[inline(always)]
    fn default() -> Iicwl0 {
        <crate::RegValueT<Iicwl0_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iicwh0_SPEC;
impl crate::sealed::RegSpec for Iicwh0_SPEC {
    type DataType = u8;
}

pub type Iicwh0 = crate::RegValueT<Iicwh0_SPEC>;

impl NoBitfieldReg<Iicwh0_SPEC> for Iicwh0 {}
impl ::core::default::Default for Iicwh0 {
    #[inline(always)]
    fn default() -> Iicwh0 {
        <crate::RegValueT<Iicwh0_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sva0_SPEC;
impl crate::sealed::RegSpec for Sva0_SPEC {
    type DataType = u8;
}

pub type Sva0 = crate::RegValueT<Sva0_SPEC>;

impl Sva0 {
    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sva0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sva0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sva0 {
    #[inline(always)]
    fn default() -> Sva0 {
        <crate::RegValueT<Sva0_SPEC> as RegisterValue<_>>::new(0)
    }
}
