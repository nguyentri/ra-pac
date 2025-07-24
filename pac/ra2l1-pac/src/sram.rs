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
// Generated from SVD 1.50.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:47:04 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"SRAM Control"]
unsafe impl ::core::marker::Send for super::Sram {}
unsafe impl ::core::marker::Sync for super::Sram {}
impl super::Sram {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "SRAM Parity Error Operation After Detection Register"]
    #[inline(always)]
    pub const fn parioad(
        &self,
    ) -> &'static crate::common::Reg<self::Parioad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Parioad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "SRAM Protection Register"]
    #[inline(always)]
    pub const fn sramprcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sramprcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramprcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "ECC Operating Mode Control Register"]
    #[inline(always)]
    pub const fn eccmode(
        &self,
    ) -> &'static crate::common::Reg<self::Eccmode_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eccmode_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "ECC 2-Bit Error Status Register"]
    #[inline(always)]
    pub const fn ecc2sts(
        &self,
    ) -> &'static crate::common::Reg<self::Ecc2Sts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecc2Sts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(193usize),
            )
        }
    }

    #[doc = "ECC 1-Bit Error Information Update Enable Register"]
    #[inline(always)]
    pub const fn ecc1stsen(
        &self,
    ) -> &'static crate::common::Reg<self::Ecc1Stsen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecc1Stsen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(194usize),
            )
        }
    }

    #[doc = "ECC 1-Bit Error Status Register"]
    #[inline(always)]
    pub const fn ecc1sts(
        &self,
    ) -> &'static crate::common::Reg<self::Ecc1Sts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecc1Sts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(195usize),
            )
        }
    }

    #[doc = "ECC Protection Register"]
    #[inline(always)]
    pub const fn eccprcr(
        &self,
    ) -> &'static crate::common::Reg<self::Eccprcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eccprcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[doc = "ECC Protection Register 2"]
    #[inline(always)]
    pub const fn eccprcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Eccprcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eccprcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "ECC Test Control Register"]
    #[inline(always)]
    pub const fn eccetst(
        &self,
    ) -> &'static crate::common::Reg<self::Eccetst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eccetst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[doc = "SRAM ECC Error Operation After Detection Register"]
    #[inline(always)]
    pub const fn eccoad(
        &self,
    ) -> &'static crate::common::Reg<self::Eccoad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eccoad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Parioad_SPEC;
impl crate::sealed::RegSpec for Parioad_SPEC {
    type DataType = u8;
}

#[doc = "SRAM Parity Error Operation After Detection Register"]
pub type Parioad = crate::RegValueT<Parioad_SPEC>;

impl Parioad {
    #[doc = "Operation After Detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        parioad::Oad,
        parioad::Oad,
        Parioad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            parioad::Oad,
            parioad::Oad,
            Parioad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Parioad {
    #[inline(always)]
    fn default() -> Parioad {
        <crate::RegValueT<Parioad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod parioad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramprcr_SPEC;
impl crate::sealed::RegSpec for Sramprcr_SPEC {
    type DataType = u8;
}

#[doc = "SRAM Protection Register"]
pub type Sramprcr = crate::RegValueT<Sramprcr_SPEC>;

impl Sramprcr {
    #[doc = "Register Write Control"]
    #[inline(always)]
    pub fn sramprcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramprcr::Sramprcr,
        sramprcr::Sramprcr,
        Sramprcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramprcr::Sramprcr,
            sramprcr::Sramprcr,
            Sramprcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write Key Code"]
    #[inline(always)]
    pub fn kw(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sramprcr_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sramprcr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Sramprcr {
    #[inline(always)]
    fn default() -> Sramprcr {
        <crate::RegValueT<Sramprcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramprcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramprcr_SPEC;
    pub type Sramprcr = crate::EnumBitfieldStruct<u8, Sramprcr_SPEC>;
    impl Sramprcr {
        #[doc = "Disable writes to protected registers"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes to protected registers"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccmode_SPEC;
impl crate::sealed::RegSpec for Eccmode_SPEC {
    type DataType = u8;
}

#[doc = "ECC Operating Mode Control Register"]
pub type Eccmode = crate::RegValueT<Eccmode_SPEC>;

impl Eccmode {
    #[doc = "ECC Operating Mode Select"]
    #[inline(always)]
    pub fn eccmod(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        eccmode::Eccmod,
        eccmode::Eccmod,
        Eccmode_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            eccmode::Eccmod,
            eccmode::Eccmod,
            Eccmode_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eccmode {
    #[inline(always)]
    fn default() -> Eccmode {
        <crate::RegValueT<Eccmode_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eccmode {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccmod_SPEC;
    pub type Eccmod = crate::EnumBitfieldStruct<u8, Eccmod_SPEC>;
    impl Eccmod {
        #[doc = "Disable ECC function"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Enable ECC function without error checking"]
        pub const _10: Self = Self::new(2);

        #[doc = "Enable ECC function with error checking"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecc2Sts_SPEC;
impl crate::sealed::RegSpec for Ecc2Sts_SPEC {
    type DataType = u8;
}

#[doc = "ECC 2-Bit Error Status Register"]
pub type Ecc2Sts = crate::RegValueT<Ecc2Sts_SPEC>;

impl Ecc2Sts {
    #[doc = "ECC 2-Bit Error Status"]
    #[inline(always)]
    pub fn ecc2err(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ecc2sts::Ecc2Err,
        ecc2sts::Ecc2Err,
        Ecc2Sts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecc2sts::Ecc2Err,
            ecc2sts::Ecc2Err,
            Ecc2Sts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ecc2Sts {
    #[inline(always)]
    fn default() -> Ecc2Sts {
        <crate::RegValueT<Ecc2Sts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecc2sts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecc2Err_SPEC;
    pub type Ecc2Err = crate::EnumBitfieldStruct<u8, Ecc2Err_SPEC>;
    impl Ecc2Err {
        #[doc = "No 2-bit ECC error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "2-bit ECC error occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecc1Stsen_SPEC;
impl crate::sealed::RegSpec for Ecc1Stsen_SPEC {
    type DataType = u8;
}

#[doc = "ECC 1-Bit Error Information Update Enable Register"]
pub type Ecc1Stsen = crate::RegValueT<Ecc1Stsen_SPEC>;

impl Ecc1Stsen {
    #[doc = "ECC 1-Bit Error Information Update Enable"]
    #[inline(always)]
    pub fn e1stsen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ecc1stsen::E1Stsen,
        ecc1stsen::E1Stsen,
        Ecc1Stsen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecc1stsen::E1Stsen,
            ecc1stsen::E1Stsen,
            Ecc1Stsen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ecc1Stsen {
    #[inline(always)]
    fn default() -> Ecc1Stsen {
        <crate::RegValueT<Ecc1Stsen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecc1stsen {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct E1Stsen_SPEC;
    pub type E1Stsen = crate::EnumBitfieldStruct<u8, E1Stsen_SPEC>;
    impl E1Stsen {
        #[doc = "Disable updating of 1-bit ECC error information"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable updating of 1-bit ECC error information"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecc1Sts_SPEC;
impl crate::sealed::RegSpec for Ecc1Sts_SPEC {
    type DataType = u8;
}

#[doc = "ECC 1-Bit Error Status Register"]
pub type Ecc1Sts = crate::RegValueT<Ecc1Sts_SPEC>;

impl Ecc1Sts {
    #[doc = "ECC 1-Bit Error Status"]
    #[inline(always)]
    pub fn ecc1err(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ecc1sts::Ecc1Err,
        ecc1sts::Ecc1Err,
        Ecc1Sts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecc1sts::Ecc1Err,
            ecc1sts::Ecc1Err,
            Ecc1Sts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ecc1Sts {
    #[inline(always)]
    fn default() -> Ecc1Sts {
        <crate::RegValueT<Ecc1Sts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecc1sts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecc1Err_SPEC;
    pub type Ecc1Err = crate::EnumBitfieldStruct<u8, Ecc1Err_SPEC>;
    impl Ecc1Err {
        #[doc = "No 1-bit ECC error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "1-bit ECC error occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccprcr_SPEC;
impl crate::sealed::RegSpec for Eccprcr_SPEC {
    type DataType = u8;
}

#[doc = "ECC Protection Register"]
pub type Eccprcr = crate::RegValueT<Eccprcr_SPEC>;

impl Eccprcr {
    #[doc = "Register Write Control"]
    #[inline(always)]
    pub fn eccprcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eccprcr::Eccprcr,
        eccprcr::Eccprcr,
        Eccprcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eccprcr::Eccprcr,
            eccprcr::Eccprcr,
            Eccprcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write Key Code"]
    #[inline(always)]
    pub fn kw(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7f,
        1,
        0,
        eccprcr::Kw,
        eccprcr::Kw,
        Eccprcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x7f,
            1,
            0,
            eccprcr::Kw,
            eccprcr::Kw,
            Eccprcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eccprcr {
    #[inline(always)]
    fn default() -> Eccprcr {
        <crate::RegValueT<Eccprcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eccprcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccprcr_SPEC;
    pub type Eccprcr = crate::EnumBitfieldStruct<u8, Eccprcr_SPEC>;
    impl Eccprcr {
        #[doc = "Disable writes to the protected registers"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes to the protected registers"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Kw_SPEC;
    pub type Kw = crate::EnumBitfieldStruct<u8, Kw_SPEC>;
    impl Kw {
        #[doc = "Enable write to the ECCPRCR bit"]
        pub const _0_X_78: Self = Self::new(120);

        #[doc = "Disable write to the ECCPRCR bit"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccprcr2_SPEC;
impl crate::sealed::RegSpec for Eccprcr2_SPEC {
    type DataType = u8;
}

#[doc = "ECC Protection Register 2"]
pub type Eccprcr2 = crate::RegValueT<Eccprcr2_SPEC>;

impl Eccprcr2 {
    #[doc = "Register Write Control"]
    #[inline(always)]
    pub fn eccprcr2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eccprcr2::Eccprcr2,
        eccprcr2::Eccprcr2,
        Eccprcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eccprcr2::Eccprcr2,
            eccprcr2::Eccprcr2,
            Eccprcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write Key Code"]
    #[inline(always)]
    pub fn kw2(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7f,
        1,
        0,
        eccprcr2::Kw2,
        eccprcr2::Kw2,
        Eccprcr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x7f,
            1,
            0,
            eccprcr2::Kw2,
            eccprcr2::Kw2,
            Eccprcr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eccprcr2 {
    #[inline(always)]
    fn default() -> Eccprcr2 {
        <crate::RegValueT<Eccprcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eccprcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccprcr2_SPEC;
    pub type Eccprcr2 = crate::EnumBitfieldStruct<u8, Eccprcr2_SPEC>;
    impl Eccprcr2 {
        #[doc = "Disable writes to the protected registers"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes to the protected registers"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Kw2_SPEC;
    pub type Kw2 = crate::EnumBitfieldStruct<u8, Kw2_SPEC>;
    impl Kw2 {
        #[doc = "Enable write to the ECCPRCR2 bit"]
        pub const _0_X_78: Self = Self::new(120);

        #[doc = "Disable write to the ECCPRCR2 bit"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccetst_SPEC;
impl crate::sealed::RegSpec for Eccetst_SPEC {
    type DataType = u8;
}

#[doc = "ECC Test Control Register"]
pub type Eccetst = crate::RegValueT<Eccetst_SPEC>;

impl Eccetst {
    #[doc = "ECC Bypass Select"]
    #[inline(always)]
    pub fn tstbyp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eccetst::Tstbyp,
        eccetst::Tstbyp,
        Eccetst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eccetst::Tstbyp,
            eccetst::Tstbyp,
            Eccetst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eccetst {
    #[inline(always)]
    fn default() -> Eccetst {
        <crate::RegValueT<Eccetst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eccetst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tstbyp_SPEC;
    pub type Tstbyp = crate::EnumBitfieldStruct<u8, Tstbyp_SPEC>;
    impl Tstbyp {
        #[doc = "Disable ECC bypass"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable ECC bypass"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccoad_SPEC;
impl crate::sealed::RegSpec for Eccoad_SPEC {
    type DataType = u8;
}

#[doc = "SRAM ECC Error Operation After Detection Register"]
pub type Eccoad = crate::RegValueT<Eccoad_SPEC>;

impl Eccoad {
    #[doc = "Operation After Detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eccoad::Oad,
        eccoad::Oad,
        Eccoad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eccoad::Oad,
            eccoad::Oad,
            Eccoad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eccoad {
    #[inline(always)]
    fn default() -> Eccoad {
        <crate::RegValueT<Eccoad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eccoad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
}
