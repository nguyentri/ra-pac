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
#[doc = r"Data Operation Circuit"]
unsafe impl ::core::marker::Send for super::DocB {}
unsafe impl ::core::marker::Sync for super::DocB {}
impl super::DocB {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "DOC Control Register"]
    #[inline(always)]
    pub const fn docr(&self) -> &'static crate::common::Reg<self::Docr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Docr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "DOC Flag Status Register"]
    #[inline(always)]
    pub const fn dosr(&self) -> &'static crate::common::Reg<self::Dosr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dosr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "DOC Flag Status Clear Register"]
    #[inline(always)]
    pub const fn doscr(&self) -> &'static crate::common::Reg<self::Doscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Doscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "DOC Data Input Register"]
    #[inline(always)]
    pub const fn dodir(&self) -> &'static crate::common::Reg<self::Dodir_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dodir_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "DOC Data Setting Register 0"]
    #[inline(always)]
    pub const fn dodsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Dodsr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dodsr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "DOC Data Setting Register 1"]
    #[inline(always)]
    pub const fn dodsr1(
        &self,
    ) -> &'static crate::common::Reg<self::Dodsr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dodsr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Docr_SPEC;
impl crate::sealed::RegSpec for Docr_SPEC {
    type DataType = u8;
}

#[doc = "DOC Control Register"]
pub type Docr = crate::RegValueT<Docr_SPEC>;

impl Docr {
    #[doc = "Operating Mode Select"]
    #[inline(always)]
    pub fn oms(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        docr::Oms,
        docr::Oms,
        Docr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            docr::Oms,
            docr::Oms,
            Docr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Operation Bit Width Select"]
    #[inline(always)]
    pub fn dobw(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        docr::Dobw,
        docr::Dobw,
        Docr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            docr::Dobw,
            docr::Dobw,
            Docr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Detection Condition Select"]
    #[inline(always)]
    pub fn dcsel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        docr::Dcsel,
        docr::Dcsel,
        Docr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            docr::Dcsel,
            docr::Dcsel,
            Docr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Docr {
    #[inline(always)]
    fn default() -> Docr {
        <crate::RegValueT<Docr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod docr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oms_SPEC;
    pub type Oms = crate::EnumBitfieldStruct<u8, Oms_SPEC>;
    impl Oms {
        #[doc = "Data comparison mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "Data addition mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "Data subtraction mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dobw_SPEC;
    pub type Dobw = crate::EnumBitfieldStruct<u8, Dobw_SPEC>;
    impl Dobw {
        #[doc = "16-bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "32-bit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcsel_SPEC;
    pub type Dcsel = crate::EnumBitfieldStruct<u8, Dcsel_SPEC>;
    impl Dcsel {
        #[doc = "Mismatch (DODSR0 ≠ DODIR)"]
        pub const _000: Self = Self::new(0);

        #[doc = "Match (DODSR0 = DODIR)"]
        pub const _001: Self = Self::new(1);

        #[doc = "Lower (DODSR0 > DODIR)"]
        pub const _010: Self = Self::new(2);

        #[doc = "Upper (DODSR0 < DODIR)"]
        pub const _011: Self = Self::new(3);

        #[doc = "Inside window (DODSR0 < DODIR < DODSR1)"]
        pub const _100: Self = Self::new(4);

        #[doc = "Outside window (DODIR < DODSR0, DODSR1 < DODIR)"]
        pub const _101: Self = Self::new(5);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dosr_SPEC;
impl crate::sealed::RegSpec for Dosr_SPEC {
    type DataType = u8;
}

#[doc = "DOC Flag Status Register"]
pub type Dosr = crate::RegValueT<Dosr_SPEC>;

impl Dosr {
    #[doc = "Data Operation Circuit Flag"]
    #[inline(always)]
    pub fn dopcf(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dosr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dosr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Dosr {
    #[inline(always)]
    fn default() -> Dosr {
        <crate::RegValueT<Dosr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doscr_SPEC;
impl crate::sealed::RegSpec for Doscr_SPEC {
    type DataType = u8;
}

#[doc = "DOC Flag Status Clear Register"]
pub type Doscr = crate::RegValueT<Doscr_SPEC>;

impl Doscr {
    #[doc = "DOPCF Clear"]
    #[inline(always)]
    pub fn dopcfcl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        doscr::Dopcfcl,
        doscr::Dopcfcl,
        Doscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            doscr::Dopcfcl,
            doscr::Dopcfcl,
            Doscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Doscr {
    #[inline(always)]
    fn default() -> Doscr {
        <crate::RegValueT<Doscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod doscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dopcfcl_SPEC;
    pub type Dopcfcl = crate::EnumBitfieldStruct<u8, Dopcfcl_SPEC>;
    impl Dopcfcl {
        #[doc = "Maintains the DOPCF flag state."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the DOPCF flag."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dodir_SPEC;
impl crate::sealed::RegSpec for Dodir_SPEC {
    type DataType = u32;
}

#[doc = "DOC Data Input Register"]
pub type Dodir = crate::RegValueT<Dodir_SPEC>;

impl NoBitfieldReg<Dodir_SPEC> for Dodir {}
impl ::core::default::Default for Dodir {
    #[inline(always)]
    fn default() -> Dodir {
        <crate::RegValueT<Dodir_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dodsr0_SPEC;
impl crate::sealed::RegSpec for Dodsr0_SPEC {
    type DataType = u32;
}

#[doc = "DOC Data Setting Register 0"]
pub type Dodsr0 = crate::RegValueT<Dodsr0_SPEC>;

impl NoBitfieldReg<Dodsr0_SPEC> for Dodsr0 {}
impl ::core::default::Default for Dodsr0 {
    #[inline(always)]
    fn default() -> Dodsr0 {
        <crate::RegValueT<Dodsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dodsr1_SPEC;
impl crate::sealed::RegSpec for Dodsr1_SPEC {
    type DataType = u32;
}

#[doc = "DOC Data Setting Register 1"]
pub type Dodsr1 = crate::RegValueT<Dodsr1_SPEC>;

impl NoBitfieldReg<Dodsr1_SPEC> for Dodsr1 {}
impl ::core::default::Default for Dodsr1 {
    #[inline(always)]
    fn default() -> Dodsr1 {
        <crate::RegValueT<Dodsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
