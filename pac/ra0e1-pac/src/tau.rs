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
#[doc = r"Timer Array Unit"]
unsafe impl ::core::marker::Send for super::Tau {}
unsafe impl ::core::marker::Sync for super::Tau {}
impl super::Tau {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn tdr00(&self) -> &'static crate::common::Reg<self::Tdr00_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr00_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdr01(&self) -> &'static crate::common::Reg<self::Tdr01_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr01_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdr01l(
        &self,
    ) -> &'static crate::common::Reg<self::Tdr01L_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr01L_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdr01h(
        &self,
    ) -> &'static crate::common::Reg<self::Tdr01H_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr01H_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdr02(&self) -> &'static crate::common::Reg<self::Tdr02_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr02_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdr03(&self) -> &'static crate::common::Reg<self::Tdr03_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr03_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdr03l(
        &self,
    ) -> &'static crate::common::Reg<self::Tdr03L_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr03L_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdr03h(
        &self,
    ) -> &'static crate::common::Reg<self::Tdr03H_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr03H_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdr04(&self) -> &'static crate::common::Reg<self::Tdr04_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr04_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdr05(&self) -> &'static crate::common::Reg<self::Tdr05_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr05_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdr06(&self) -> &'static crate::common::Reg<self::Tdr06_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr06_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdr07(&self) -> &'static crate::common::Reg<self::Tdr07_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr07_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tcr0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Tcr0_SPEC, crate::common::R>,
        8,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }

    #[inline(always)]
    pub const fn tmr00(&self) -> &'static crate::common::Reg<self::Tmr00_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmr00_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tmr01(&self) -> &'static crate::common::Reg<self::Tmr01_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmr01_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(274usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tmr02(&self) -> &'static crate::common::Reg<self::Tmr02_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmr02_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(276usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tmr03(&self) -> &'static crate::common::Reg<self::Tmr03_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmr03_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(278usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tmr04(&self) -> &'static crate::common::Reg<self::Tmr04_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmr04_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(280usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tmr05(&self) -> &'static crate::common::Reg<self::Tmr05_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmr05_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(282usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tmr06(&self) -> &'static crate::common::Reg<self::Tmr06_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmr06_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(284usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tmr07(&self) -> &'static crate::common::Reg<self::Tmr07_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmr07_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(286usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tsr0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Tsr0_SPEC, crate::common::R>,
        8,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x120usize))
        }
    }

    #[inline(always)]
    pub const fn te0(&self) -> &'static crate::common::Reg<self::Te0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Te0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ts0(&self) -> &'static crate::common::Reg<self::Ts0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ts0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(306usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tt0(&self) -> &'static crate::common::Reg<self::Tt0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tt0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(308usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tps0(&self) -> &'static crate::common::Reg<self::Tps0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tps0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(310usize),
            )
        }
    }

    #[inline(always)]
    pub const fn to0(&self) -> &'static crate::common::Reg<self::To0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::To0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(312usize),
            )
        }
    }

    #[inline(always)]
    pub const fn toe0(&self) -> &'static crate::common::Reg<self::Toe0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Toe0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(314usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tol0(&self) -> &'static crate::common::Reg<self::Tol0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tol0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(316usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tom0(&self) -> &'static crate::common::Reg<self::Tom0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tom0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(318usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr00_SPEC;
impl crate::sealed::RegSpec for Tdr00_SPEC {
    type DataType = u16;
}

pub type Tdr00 = crate::RegValueT<Tdr00_SPEC>;

impl NoBitfieldReg<Tdr00_SPEC> for Tdr00 {}
impl ::core::default::Default for Tdr00 {
    #[inline(always)]
    fn default() -> Tdr00 {
        <crate::RegValueT<Tdr00_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr01_SPEC;
impl crate::sealed::RegSpec for Tdr01_SPEC {
    type DataType = u16;
}

pub type Tdr01 = crate::RegValueT<Tdr01_SPEC>;

impl NoBitfieldReg<Tdr01_SPEC> for Tdr01 {}
impl ::core::default::Default for Tdr01 {
    #[inline(always)]
    fn default() -> Tdr01 {
        <crate::RegValueT<Tdr01_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr01L_SPEC;
impl crate::sealed::RegSpec for Tdr01L_SPEC {
    type DataType = u8;
}

pub type Tdr01L = crate::RegValueT<Tdr01L_SPEC>;

impl NoBitfieldReg<Tdr01L_SPEC> for Tdr01L {}
impl ::core::default::Default for Tdr01L {
    #[inline(always)]
    fn default() -> Tdr01L {
        <crate::RegValueT<Tdr01L_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr01H_SPEC;
impl crate::sealed::RegSpec for Tdr01H_SPEC {
    type DataType = u8;
}

pub type Tdr01H = crate::RegValueT<Tdr01H_SPEC>;

impl NoBitfieldReg<Tdr01H_SPEC> for Tdr01H {}
impl ::core::default::Default for Tdr01H {
    #[inline(always)]
    fn default() -> Tdr01H {
        <crate::RegValueT<Tdr01H_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr02_SPEC;
impl crate::sealed::RegSpec for Tdr02_SPEC {
    type DataType = u16;
}

pub type Tdr02 = crate::RegValueT<Tdr02_SPEC>;

impl NoBitfieldReg<Tdr02_SPEC> for Tdr02 {}
impl ::core::default::Default for Tdr02 {
    #[inline(always)]
    fn default() -> Tdr02 {
        <crate::RegValueT<Tdr02_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr03_SPEC;
impl crate::sealed::RegSpec for Tdr03_SPEC {
    type DataType = u16;
}

pub type Tdr03 = crate::RegValueT<Tdr03_SPEC>;

impl NoBitfieldReg<Tdr03_SPEC> for Tdr03 {}
impl ::core::default::Default for Tdr03 {
    #[inline(always)]
    fn default() -> Tdr03 {
        <crate::RegValueT<Tdr03_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr03L_SPEC;
impl crate::sealed::RegSpec for Tdr03L_SPEC {
    type DataType = u8;
}

pub type Tdr03L = crate::RegValueT<Tdr03L_SPEC>;

impl NoBitfieldReg<Tdr03L_SPEC> for Tdr03L {}
impl ::core::default::Default for Tdr03L {
    #[inline(always)]
    fn default() -> Tdr03L {
        <crate::RegValueT<Tdr03L_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr03H_SPEC;
impl crate::sealed::RegSpec for Tdr03H_SPEC {
    type DataType = u8;
}

pub type Tdr03H = crate::RegValueT<Tdr03H_SPEC>;

impl NoBitfieldReg<Tdr03H_SPEC> for Tdr03H {}
impl ::core::default::Default for Tdr03H {
    #[inline(always)]
    fn default() -> Tdr03H {
        <crate::RegValueT<Tdr03H_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr04_SPEC;
impl crate::sealed::RegSpec for Tdr04_SPEC {
    type DataType = u16;
}

pub type Tdr04 = crate::RegValueT<Tdr04_SPEC>;

impl NoBitfieldReg<Tdr04_SPEC> for Tdr04 {}
impl ::core::default::Default for Tdr04 {
    #[inline(always)]
    fn default() -> Tdr04 {
        <crate::RegValueT<Tdr04_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr05_SPEC;
impl crate::sealed::RegSpec for Tdr05_SPEC {
    type DataType = u16;
}

pub type Tdr05 = crate::RegValueT<Tdr05_SPEC>;

impl NoBitfieldReg<Tdr05_SPEC> for Tdr05 {}
impl ::core::default::Default for Tdr05 {
    #[inline(always)]
    fn default() -> Tdr05 {
        <crate::RegValueT<Tdr05_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr06_SPEC;
impl crate::sealed::RegSpec for Tdr06_SPEC {
    type DataType = u16;
}

pub type Tdr06 = crate::RegValueT<Tdr06_SPEC>;

impl NoBitfieldReg<Tdr06_SPEC> for Tdr06 {}
impl ::core::default::Default for Tdr06 {
    #[inline(always)]
    fn default() -> Tdr06 {
        <crate::RegValueT<Tdr06_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr07_SPEC;
impl crate::sealed::RegSpec for Tdr07_SPEC {
    type DataType = u16;
}

pub type Tdr07 = crate::RegValueT<Tdr07_SPEC>;

impl NoBitfieldReg<Tdr07_SPEC> for Tdr07 {}
impl ::core::default::Default for Tdr07 {
    #[inline(always)]
    fn default() -> Tdr07 {
        <crate::RegValueT<Tdr07_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr0_SPEC;
impl crate::sealed::RegSpec for Tcr0_SPEC {
    type DataType = u16;
}

pub type Tcr0 = crate::RegValueT<Tcr0_SPEC>;

impl NoBitfieldReg<Tcr0_SPEC> for Tcr0 {}
impl ::core::default::Default for Tcr0 {
    #[inline(always)]
    fn default() -> Tcr0 {
        <crate::RegValueT<Tcr0_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr00_SPEC;
impl crate::sealed::RegSpec for Tmr00_SPEC {
    type DataType = u16;
}

pub type Tmr00 = crate::RegValueT<Tmr00_SPEC>;

impl Tmr00 {
    #[inline(always)]
    pub fn opirq(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tmr00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tmr00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn md(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7,
        1,
        0,
        tmr00::Md,
        tmr00::Md,
        Tmr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7,
            1,
            0,
            tmr00::Md,
            tmr00::Md,
            Tmr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cis(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        tmr00::Cis,
        tmr00::Cis,
        Tmr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            tmr00::Cis,
            tmr00::Cis,
            Tmr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        tmr00::Sts,
        tmr00::Sts,
        Tmr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            tmr00::Sts,
            tmr00::Sts,
            Tmr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        tmr00::Ccs,
        tmr00::Ccs,
        Tmr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            tmr00::Ccs,
            tmr00::Ccs,
            Tmr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        tmr00::Cks,
        tmr00::Cks,
        Tmr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            tmr00::Cks,
            tmr00::Cks,
            Tmr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tmr00 {
    #[inline(always)]
    fn default() -> Tmr00 {
        <crate::RegValueT<Tmr00_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tmr00 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md_SPEC;
    pub type Md = crate::EnumBitfieldStruct<u8, Md_SPEC>;
    impl Md {
        pub const _000: Self = Self::new(0);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _110: Self = Self::new(6);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cis_SPEC;
    pub type Cis = crate::EnumBitfieldStruct<u8, Cis_SPEC>;
    impl Cis {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sts_SPEC;
    pub type Sts = crate::EnumBitfieldStruct<u8, Sts_SPEC>;
    impl Sts {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _100: Self = Self::new(4);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr01_SPEC;
impl crate::sealed::RegSpec for Tmr01_SPEC {
    type DataType = u16;
}

pub type Tmr01 = crate::RegValueT<Tmr01_SPEC>;

impl Tmr01 {
    #[inline(always)]
    pub fn opirq(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tmr01_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tmr01_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn md(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7,
        1,
        0,
        tmr01::Md,
        tmr01::Md,
        Tmr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7,
            1,
            0,
            tmr01::Md,
            tmr01::Md,
            Tmr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cis(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        tmr01::Cis,
        tmr01::Cis,
        Tmr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            tmr01::Cis,
            tmr01::Cis,
            Tmr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        tmr01::Sts,
        tmr01::Sts,
        Tmr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            tmr01::Sts,
            tmr01::Sts,
            Tmr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn split(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        tmr01::Split,
        tmr01::Split,
        Tmr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            tmr01::Split,
            tmr01::Split,
            Tmr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        tmr01::Ccs,
        tmr01::Ccs,
        Tmr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            tmr01::Ccs,
            tmr01::Ccs,
            Tmr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        tmr01::Cks,
        tmr01::Cks,
        Tmr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            tmr01::Cks,
            tmr01::Cks,
            Tmr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tmr01 {
    #[inline(always)]
    fn default() -> Tmr01 {
        <crate::RegValueT<Tmr01_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tmr01 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md_SPEC;
    pub type Md = crate::EnumBitfieldStruct<u8, Md_SPEC>;
    impl Md {
        pub const _000: Self = Self::new(0);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _110: Self = Self::new(6);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cis_SPEC;
    pub type Cis = crate::EnumBitfieldStruct<u8, Cis_SPEC>;
    impl Cis {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sts_SPEC;
    pub type Sts = crate::EnumBitfieldStruct<u8, Sts_SPEC>;
    impl Sts {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _100: Self = Self::new(4);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Split_SPEC;
    pub type Split = crate::EnumBitfieldStruct<u8, Split_SPEC>;
    impl Split {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr02_SPEC;
impl crate::sealed::RegSpec for Tmr02_SPEC {
    type DataType = u16;
}

pub type Tmr02 = crate::RegValueT<Tmr02_SPEC>;

impl Tmr02 {
    #[inline(always)]
    pub fn opirq(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tmr02_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tmr02_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn md(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7,
        1,
        0,
        tmr02::Md,
        tmr02::Md,
        Tmr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7,
            1,
            0,
            tmr02::Md,
            tmr02::Md,
            Tmr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cis(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        tmr02::Cis,
        tmr02::Cis,
        Tmr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            tmr02::Cis,
            tmr02::Cis,
            Tmr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        tmr02::Sts,
        tmr02::Sts,
        Tmr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            tmr02::Sts,
            tmr02::Sts,
            Tmr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn master(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        tmr02::Master,
        tmr02::Master,
        Tmr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            tmr02::Master,
            tmr02::Master,
            Tmr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        tmr02::Ccs,
        tmr02::Ccs,
        Tmr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            tmr02::Ccs,
            tmr02::Ccs,
            Tmr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        tmr02::Cks,
        tmr02::Cks,
        Tmr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            tmr02::Cks,
            tmr02::Cks,
            Tmr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tmr02 {
    #[inline(always)]
    fn default() -> Tmr02 {
        <crate::RegValueT<Tmr02_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tmr02 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md_SPEC;
    pub type Md = crate::EnumBitfieldStruct<u8, Md_SPEC>;
    impl Md {
        pub const _000: Self = Self::new(0);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _110: Self = Self::new(6);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cis_SPEC;
    pub type Cis = crate::EnumBitfieldStruct<u8, Cis_SPEC>;
    impl Cis {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sts_SPEC;
    pub type Sts = crate::EnumBitfieldStruct<u8, Sts_SPEC>;
    impl Sts {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _100: Self = Self::new(4);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Master_SPEC;
    pub type Master = crate::EnumBitfieldStruct<u8, Master_SPEC>;
    impl Master {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr03_SPEC;
impl crate::sealed::RegSpec for Tmr03_SPEC {
    type DataType = u16;
}

pub type Tmr03 = crate::RegValueT<Tmr03_SPEC>;

impl Tmr03 {
    #[inline(always)]
    pub fn opirq(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tmr03_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tmr03_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn md(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7,
        1,
        0,
        tmr03::Md,
        tmr03::Md,
        Tmr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7,
            1,
            0,
            tmr03::Md,
            tmr03::Md,
            Tmr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cis(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        tmr03::Cis,
        tmr03::Cis,
        Tmr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            tmr03::Cis,
            tmr03::Cis,
            Tmr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        tmr03::Sts,
        tmr03::Sts,
        Tmr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            tmr03::Sts,
            tmr03::Sts,
            Tmr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn split(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        tmr03::Split,
        tmr03::Split,
        Tmr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            tmr03::Split,
            tmr03::Split,
            Tmr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        tmr03::Ccs,
        tmr03::Ccs,
        Tmr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            tmr03::Ccs,
            tmr03::Ccs,
            Tmr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        tmr03::Cks,
        tmr03::Cks,
        Tmr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            tmr03::Cks,
            tmr03::Cks,
            Tmr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tmr03 {
    #[inline(always)]
    fn default() -> Tmr03 {
        <crate::RegValueT<Tmr03_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tmr03 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md_SPEC;
    pub type Md = crate::EnumBitfieldStruct<u8, Md_SPEC>;
    impl Md {
        pub const _000: Self = Self::new(0);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _110: Self = Self::new(6);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cis_SPEC;
    pub type Cis = crate::EnumBitfieldStruct<u8, Cis_SPEC>;
    impl Cis {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sts_SPEC;
    pub type Sts = crate::EnumBitfieldStruct<u8, Sts_SPEC>;
    impl Sts {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _100: Self = Self::new(4);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Split_SPEC;
    pub type Split = crate::EnumBitfieldStruct<u8, Split_SPEC>;
    impl Split {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr04_SPEC;
impl crate::sealed::RegSpec for Tmr04_SPEC {
    type DataType = u16;
}

pub type Tmr04 = crate::RegValueT<Tmr04_SPEC>;

impl Tmr04 {
    #[inline(always)]
    pub fn opirq(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tmr04_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tmr04_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn md(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7,
        1,
        0,
        tmr04::Md,
        tmr04::Md,
        Tmr04_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7,
            1,
            0,
            tmr04::Md,
            tmr04::Md,
            Tmr04_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cis(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        tmr04::Cis,
        tmr04::Cis,
        Tmr04_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            tmr04::Cis,
            tmr04::Cis,
            Tmr04_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        tmr04::Sts,
        tmr04::Sts,
        Tmr04_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            tmr04::Sts,
            tmr04::Sts,
            Tmr04_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn master(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        tmr04::Master,
        tmr04::Master,
        Tmr04_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            tmr04::Master,
            tmr04::Master,
            Tmr04_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        tmr04::Ccs,
        tmr04::Ccs,
        Tmr04_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            tmr04::Ccs,
            tmr04::Ccs,
            Tmr04_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        tmr04::Cks,
        tmr04::Cks,
        Tmr04_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            tmr04::Cks,
            tmr04::Cks,
            Tmr04_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tmr04 {
    #[inline(always)]
    fn default() -> Tmr04 {
        <crate::RegValueT<Tmr04_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tmr04 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md_SPEC;
    pub type Md = crate::EnumBitfieldStruct<u8, Md_SPEC>;
    impl Md {
        pub const _000: Self = Self::new(0);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _110: Self = Self::new(6);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cis_SPEC;
    pub type Cis = crate::EnumBitfieldStruct<u8, Cis_SPEC>;
    impl Cis {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sts_SPEC;
    pub type Sts = crate::EnumBitfieldStruct<u8, Sts_SPEC>;
    impl Sts {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _100: Self = Self::new(4);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Master_SPEC;
    pub type Master = crate::EnumBitfieldStruct<u8, Master_SPEC>;
    impl Master {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr05_SPEC;
impl crate::sealed::RegSpec for Tmr05_SPEC {
    type DataType = u16;
}

pub type Tmr05 = crate::RegValueT<Tmr05_SPEC>;

impl Tmr05 {
    #[inline(always)]
    pub fn opirq(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tmr05_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tmr05_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn md(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7,
        1,
        0,
        tmr05::Md,
        tmr05::Md,
        Tmr05_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7,
            1,
            0,
            tmr05::Md,
            tmr05::Md,
            Tmr05_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cis(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        tmr05::Cis,
        tmr05::Cis,
        Tmr05_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            tmr05::Cis,
            tmr05::Cis,
            Tmr05_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        tmr05::Sts,
        tmr05::Sts,
        Tmr05_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            tmr05::Sts,
            tmr05::Sts,
            Tmr05_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        tmr05::Ccs,
        tmr05::Ccs,
        Tmr05_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            tmr05::Ccs,
            tmr05::Ccs,
            Tmr05_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        tmr05::Cks,
        tmr05::Cks,
        Tmr05_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            tmr05::Cks,
            tmr05::Cks,
            Tmr05_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tmr05 {
    #[inline(always)]
    fn default() -> Tmr05 {
        <crate::RegValueT<Tmr05_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tmr05 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md_SPEC;
    pub type Md = crate::EnumBitfieldStruct<u8, Md_SPEC>;
    impl Md {
        pub const _000: Self = Self::new(0);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _110: Self = Self::new(6);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cis_SPEC;
    pub type Cis = crate::EnumBitfieldStruct<u8, Cis_SPEC>;
    impl Cis {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sts_SPEC;
    pub type Sts = crate::EnumBitfieldStruct<u8, Sts_SPEC>;
    impl Sts {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _100: Self = Self::new(4);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr06_SPEC;
impl crate::sealed::RegSpec for Tmr06_SPEC {
    type DataType = u16;
}

pub type Tmr06 = crate::RegValueT<Tmr06_SPEC>;

impl Tmr06 {
    #[inline(always)]
    pub fn opirq(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tmr06_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tmr06_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn md(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7,
        1,
        0,
        tmr06::Md,
        tmr06::Md,
        Tmr06_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7,
            1,
            0,
            tmr06::Md,
            tmr06::Md,
            Tmr06_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cis(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        tmr06::Cis,
        tmr06::Cis,
        Tmr06_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            tmr06::Cis,
            tmr06::Cis,
            Tmr06_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        tmr06::Sts,
        tmr06::Sts,
        Tmr06_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            tmr06::Sts,
            tmr06::Sts,
            Tmr06_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn master(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        tmr06::Master,
        tmr06::Master,
        Tmr06_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            tmr06::Master,
            tmr06::Master,
            Tmr06_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        tmr06::Ccs,
        tmr06::Ccs,
        Tmr06_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            tmr06::Ccs,
            tmr06::Ccs,
            Tmr06_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        tmr06::Cks,
        tmr06::Cks,
        Tmr06_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            tmr06::Cks,
            tmr06::Cks,
            Tmr06_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tmr06 {
    #[inline(always)]
    fn default() -> Tmr06 {
        <crate::RegValueT<Tmr06_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tmr06 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md_SPEC;
    pub type Md = crate::EnumBitfieldStruct<u8, Md_SPEC>;
    impl Md {
        pub const _000: Self = Self::new(0);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _110: Self = Self::new(6);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cis_SPEC;
    pub type Cis = crate::EnumBitfieldStruct<u8, Cis_SPEC>;
    impl Cis {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sts_SPEC;
    pub type Sts = crate::EnumBitfieldStruct<u8, Sts_SPEC>;
    impl Sts {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _100: Self = Self::new(4);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Master_SPEC;
    pub type Master = crate::EnumBitfieldStruct<u8, Master_SPEC>;
    impl Master {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr07_SPEC;
impl crate::sealed::RegSpec for Tmr07_SPEC {
    type DataType = u16;
}

pub type Tmr07 = crate::RegValueT<Tmr07_SPEC>;

impl Tmr07 {
    #[inline(always)]
    pub fn opirq(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tmr07_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tmr07_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn md(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7,
        1,
        0,
        tmr07::Md,
        tmr07::Md,
        Tmr07_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7,
            1,
            0,
            tmr07::Md,
            tmr07::Md,
            Tmr07_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cis(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        tmr07::Cis,
        tmr07::Cis,
        Tmr07_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            tmr07::Cis,
            tmr07::Cis,
            Tmr07_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        tmr07::Sts,
        tmr07::Sts,
        Tmr07_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            tmr07::Sts,
            tmr07::Sts,
            Tmr07_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        tmr07::Ccs,
        tmr07::Ccs,
        Tmr07_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            tmr07::Ccs,
            tmr07::Ccs,
            Tmr07_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        tmr07::Cks,
        tmr07::Cks,
        Tmr07_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            tmr07::Cks,
            tmr07::Cks,
            Tmr07_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tmr07 {
    #[inline(always)]
    fn default() -> Tmr07 {
        <crate::RegValueT<Tmr07_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tmr07 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md_SPEC;
    pub type Md = crate::EnumBitfieldStruct<u8, Md_SPEC>;
    impl Md {
        pub const _000: Self = Self::new(0);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _110: Self = Self::new(6);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cis_SPEC;
    pub type Cis = crate::EnumBitfieldStruct<u8, Cis_SPEC>;
    impl Cis {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sts_SPEC;
    pub type Sts = crate::EnumBitfieldStruct<u8, Sts_SPEC>;
    impl Sts {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _100: Self = Self::new(4);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsr0_SPEC;
impl crate::sealed::RegSpec for Tsr0_SPEC {
    type DataType = u16;
}

pub type Tsr0 = crate::RegValueT<Tsr0_SPEC>;

impl Tsr0 {
    #[inline(always)]
    pub fn ovf(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tsr0::Ovf, tsr0::Ovf, Tsr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tsr0::Ovf,
            tsr0::Ovf,
            Tsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tsr0 {
    #[inline(always)]
    fn default() -> Tsr0 {
        <crate::RegValueT<Tsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovf_SPEC;
    pub type Ovf = crate::EnumBitfieldStruct<u8, Ovf_SPEC>;
    impl Ovf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Te0_SPEC;
impl crate::sealed::RegSpec for Te0_SPEC {
    type DataType = u16;
}

pub type Te0 = crate::RegValueT<Te0_SPEC>;

impl Te0 {
    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, te0::Te, te0::Te, Te0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,te0::Te,te0::Te,Te0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn teh1(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, te0::Teh1, te0::Teh1, Te0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x1,1,0,te0::Teh1,te0::Teh1,Te0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn teh3(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, te0::Teh3, te0::Teh3, Te0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            te0::Teh3,
            te0::Teh3,
            Te0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Te0 {
    #[inline(always)]
    fn default() -> Te0 {
        <crate::RegValueT<Te0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod te0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te_SPEC;
    pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
    impl Te {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teh1_SPEC;
    pub type Teh1 = crate::EnumBitfieldStruct<u8, Teh1_SPEC>;
    impl Teh1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teh3_SPEC;
    pub type Teh3 = crate::EnumBitfieldStruct<u8, Teh3_SPEC>;
    impl Teh3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ts0_SPEC;
impl crate::sealed::RegSpec for Ts0_SPEC {
    type DataType = u16;
}

pub type Ts0 = crate::RegValueT<Ts0_SPEC>;

impl Ts0 {
    #[inline(always)]
    pub fn ts(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, ts0::Ts, ts0::Ts, Ts0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,ts0::Ts,ts0::Ts,Ts0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tsh1(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ts0::Tsh1, ts0::Tsh1, Ts0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ts0::Tsh1,
            ts0::Tsh1,
            Ts0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tsh3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ts0::Tsh3,
        ts0::Tsh3,
        Ts0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ts0::Tsh3,
            ts0::Tsh3,
            Ts0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ts0 {
    #[inline(always)]
    fn default() -> Ts0 {
        <crate::RegValueT<Ts0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ts0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ts_SPEC;
    pub type Ts = crate::EnumBitfieldStruct<u8, Ts_SPEC>;
    impl Ts {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsh1_SPEC;
    pub type Tsh1 = crate::EnumBitfieldStruct<u8, Tsh1_SPEC>;
    impl Tsh1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsh3_SPEC;
    pub type Tsh3 = crate::EnumBitfieldStruct<u8, Tsh3_SPEC>;
    impl Tsh3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tt0_SPEC;
impl crate::sealed::RegSpec for Tt0_SPEC {
    type DataType = u16;
}

pub type Tt0 = crate::RegValueT<Tt0_SPEC>;

impl Tt0 {
    #[inline(always)]
    pub fn tt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, tt0::Tt, tt0::Tt, Tt0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,tt0::Tt,tt0::Tt,Tt0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tth1(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, tt0::Tth1, tt0::Tth1, Tt0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            tt0::Tth1,
            tt0::Tth1,
            Tt0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tth3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        tt0::Tth3,
        tt0::Tth3,
        Tt0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            tt0::Tth3,
            tt0::Tth3,
            Tt0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tt0 {
    #[inline(always)]
    fn default() -> Tt0 {
        <crate::RegValueT<Tt0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tt0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tt_SPEC;
    pub type Tt = crate::EnumBitfieldStruct<u8, Tt_SPEC>;
    impl Tt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tth1_SPEC;
    pub type Tth1 = crate::EnumBitfieldStruct<u8, Tth1_SPEC>;
    impl Tth1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tth3_SPEC;
    pub type Tth3 = crate::EnumBitfieldStruct<u8, Tth3_SPEC>;
    impl Tth3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tps0_SPEC;
impl crate::sealed::RegSpec for Tps0_SPEC {
    type DataType = u16;
}

pub type Tps0 = crate::RegValueT<Tps0_SPEC>;

impl Tps0 {
    #[inline(always)]
    pub fn prs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        tps0::Prs0,
        tps0::Prs0,
        Tps0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            tps0::Prs0,
            tps0::Prs0,
            Tps0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn prs1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        tps0::Prs1,
        tps0::Prs1,
        Tps0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            tps0::Prs1,
            tps0::Prs1,
            Tps0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn prs2(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        tps0::Prs2,
        tps0::Prs2,
        Tps0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            tps0::Prs2,
            tps0::Prs2,
            Tps0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn prs3(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        tps0::Prs3,
        tps0::Prs3,
        Tps0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            tps0::Prs3,
            tps0::Prs3,
            Tps0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tps0 {
    #[inline(always)]
    fn default() -> Tps0 {
        <crate::RegValueT<Tps0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tps0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prs0_SPEC;
    pub type Prs0 = crate::EnumBitfieldStruct<u8, Prs0_SPEC>;
    impl Prs0 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const _0_X_6: Self = Self::new(6);

        pub const _0_X_7: Self = Self::new(7);

        pub const _0_X_8: Self = Self::new(8);

        pub const _0_X_9: Self = Self::new(9);

        pub const _0_X_A: Self = Self::new(10);

        pub const _0_X_B: Self = Self::new(11);

        pub const _0_X_C: Self = Self::new(12);

        pub const _0_X_D: Self = Self::new(13);

        pub const _0_X_E: Self = Self::new(14);

        pub const _0_X_F: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prs1_SPEC;
    pub type Prs1 = crate::EnumBitfieldStruct<u8, Prs1_SPEC>;
    impl Prs1 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const _0_X_6: Self = Self::new(6);

        pub const _0_X_7: Self = Self::new(7);

        pub const _0_X_8: Self = Self::new(8);

        pub const _0_X_9: Self = Self::new(9);

        pub const _0_X_A: Self = Self::new(10);

        pub const _0_X_B: Self = Self::new(11);

        pub const _0_X_C: Self = Self::new(12);

        pub const _0_X_D: Self = Self::new(13);

        pub const _0_X_E: Self = Self::new(14);

        pub const _0_X_F: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prs2_SPEC;
    pub type Prs2 = crate::EnumBitfieldStruct<u8, Prs2_SPEC>;
    impl Prs2 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prs3_SPEC;
    pub type Prs3 = crate::EnumBitfieldStruct<u8, Prs3_SPEC>;
    impl Prs3 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct To0_SPEC;
impl crate::sealed::RegSpec for To0_SPEC {
    type DataType = u16;
}

pub type To0 = crate::RegValueT<To0_SPEC>;

impl To0 {
    #[inline(always)]
    pub fn to(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, to0::To, to0::To, To0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,to0::To,to0::To,To0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for To0 {
    #[inline(always)]
    fn default() -> To0 {
        <crate::RegValueT<To0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod to0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct To_SPEC;
    pub type To = crate::EnumBitfieldStruct<u8, To_SPEC>;
    impl To {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Toe0_SPEC;
impl crate::sealed::RegSpec for Toe0_SPEC {
    type DataType = u16;
}

pub type Toe0 = crate::RegValueT<Toe0_SPEC>;

impl Toe0 {
    #[inline(always)]
    pub fn toe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        toe0::Toe,
        toe0::Toe,
        Toe0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            toe0::Toe,
            toe0::Toe,
            Toe0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Toe0 {
    #[inline(always)]
    fn default() -> Toe0 {
        <crate::RegValueT<Toe0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod toe0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toe_SPEC;
    pub type Toe = crate::EnumBitfieldStruct<u8, Toe_SPEC>;
    impl Toe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tol0_SPEC;
impl crate::sealed::RegSpec for Tol0_SPEC {
    type DataType = u16;
}

pub type Tol0 = crate::RegValueT<Tol0_SPEC>;

impl Tol0 {
    #[inline(always)]
    pub fn tol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7f,
        1,
        0,
        tol0::Tol,
        tol0::Tol,
        Tol0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7f,
            1,
            0,
            tol0::Tol,
            tol0::Tol,
            Tol0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tol0 {
    #[inline(always)]
    fn default() -> Tol0 {
        <crate::RegValueT<Tol0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tol0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tol_SPEC;
    pub type Tol = crate::EnumBitfieldStruct<u8, Tol_SPEC>;
    impl Tol {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tom0_SPEC;
impl crate::sealed::RegSpec for Tom0_SPEC {
    type DataType = u16;
}

pub type Tom0 = crate::RegValueT<Tom0_SPEC>;

impl Tom0 {
    #[inline(always)]
    pub fn tom(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7f,
        1,
        0,
        tom0::Tom,
        tom0::Tom,
        Tom0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7f,
            1,
            0,
            tom0::Tom,
            tom0::Tom,
            Tom0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tom0 {
    #[inline(always)]
    fn default() -> Tom0 {
        <crate::RegValueT<Tom0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tom0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tom_SPEC;
    pub type Tom = crate::EnumBitfieldStruct<u8, Tom_SPEC>;
    impl Tom {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
