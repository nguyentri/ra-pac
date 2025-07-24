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
#[doc = r"TCM"]
unsafe impl ::core::marker::Send for super::Tcm {}
unsafe impl ::core::marker::Sync for super::Tcm {}
impl super::Tcm {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "TCM Protection Control Register for Secure"]
    #[inline(always)]
    pub const fn tcmprcr_s(
        &self,
    ) -> &'static crate::common::Reg<self::TcmprcrS_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TcmprcrS_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "TCM Control Register"]
    #[inline(always)]
    pub const fn tcmcrc(
        &self,
    ) -> &'static crate::common::Reg<self::Tcmcrc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tcmcrc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "TCM Control Register"]
    #[inline(always)]
    pub const fn tcmcrs(
        &self,
    ) -> &'static crate::common::Reg<self::Tcmcrs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tcmcrs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "TCM Error Status Register"]
    #[inline(always)]
    pub const fn tcmesr(&self) -> &'static crate::common::Reg<self::Tcmesr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tcmesr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "TCM Error Status Clear Register"]
    #[inline(always)]
    pub const fn tcmesclr(
        &self,
    ) -> &'static crate::common::Reg<self::Tcmesclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tcmesclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "TCM Error Address Register"]
    #[inline(always)]
    pub const fn tcmearc0(
        &self,
    ) -> &'static crate::common::Reg<self::Tcmearc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tcmearc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "TCM Error Address Register"]
    #[inline(always)]
    pub const fn tcmearc1(
        &self,
    ) -> &'static crate::common::Reg<self::Tcmearc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tcmearc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "TCM Error Address Register"]
    #[inline(always)]
    pub const fn tcmears0(
        &self,
    ) -> &'static crate::common::Reg<self::Tcmears0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tcmears0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "TCM Error Address Register"]
    #[inline(always)]
    pub const fn tcmears1(
        &self,
    ) -> &'static crate::common::Reg<self::Tcmears1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tcmears1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcmprcrS_SPEC;
impl crate::sealed::RegSpec for TcmprcrS_SPEC {
    type DataType = u16;
}

#[doc = "TCM Protection Control Register for Secure"]
pub type TcmprcrS = crate::RegValueT<TcmprcrS_SPEC>;

impl TcmprcrS {
    #[doc = "Register Write Control"]
    #[inline(always)]
    pub fn pr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tcmprcr_s::Pr,
        tcmprcr_s::Pr,
        TcmprcrS_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tcmprcr_s::Pr,
            tcmprcr_s::Pr,
            TcmprcrS_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write Key Code"]
    #[inline(always)]
    pub fn kw(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, TcmprcrS_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,TcmprcrS_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TcmprcrS {
    #[inline(always)]
    fn default() -> TcmprcrS {
        <crate::RegValueT<TcmprcrS_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcmprcr_s {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pr_SPEC;
    pub type Pr = crate::EnumBitfieldStruct<u8, Pr_SPEC>;
    impl Pr {
        #[doc = "Write registers are disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write registers are enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcmcrc_SPEC;
impl crate::sealed::RegSpec for Tcmcrc_SPEC {
    type DataType = u8;
}

#[doc = "TCM Control Register"]
pub type Tcmcrc = crate::RegValueT<Tcmcrc_SPEC>;

impl Tcmcrc {
    #[doc = "Operation after ECC error detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tcmcrc::Oad,
        tcmcrc::Oad,
        Tcmcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tcmcrc::Oad,
            tcmcrc::Oad,
            Tcmcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC Operating Mode Select"]
    #[inline(always)]
    pub fn eccmod(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        tcmcrc::Eccmod,
        tcmcrc::Eccmod,
        Tcmcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            tcmcrc::Eccmod,
            tcmcrc::Eccmod,
            Tcmcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC 1-Bit Error Information Update Enable"]
    #[inline(always)]
    pub fn e1stsen(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcmcrc::E1Stsen,
        tcmcrc::E1Stsen,
        Tcmcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcmcrc::E1Stsen,
            tcmcrc::E1Stsen,
            Tcmcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC Test Enable/ECC Bypass Select"]
    #[inline(always)]
    pub fn tstbyp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        tcmcrc::Tstbyp,
        tcmcrc::Tstbyp,
        Tcmcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            tcmcrc::Tstbyp,
            tcmcrc::Tstbyp,
            Tcmcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tcmcrc {
    #[inline(always)]
    fn default() -> Tcmcrc {
        <crate::RegValueT<Tcmcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcmcrc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct E1Stsen_SPEC;
    pub type E1Stsen = crate::EnumBitfieldStruct<u8, E1Stsen_SPEC>;
    impl E1Stsen {
        #[doc = "Disable updating of 1-bit ECC error information"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable updating of 1-bit ECC error information"]
        pub const _1: Self = Self::new(1);
    }
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
pub struct Tcmcrs_SPEC;
impl crate::sealed::RegSpec for Tcmcrs_SPEC {
    type DataType = u8;
}

#[doc = "TCM Control Register"]
pub type Tcmcrs = crate::RegValueT<Tcmcrs_SPEC>;

impl Tcmcrs {
    #[doc = "Operation after ECC error detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tcmcrs::Oad,
        tcmcrs::Oad,
        Tcmcrs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tcmcrs::Oad,
            tcmcrs::Oad,
            Tcmcrs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC Operating Mode Select"]
    #[inline(always)]
    pub fn eccmod(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        tcmcrs::Eccmod,
        tcmcrs::Eccmod,
        Tcmcrs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            tcmcrs::Eccmod,
            tcmcrs::Eccmod,
            Tcmcrs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC 1-Bit Error Information Update Enable"]
    #[inline(always)]
    pub fn e1stsen(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcmcrs::E1Stsen,
        tcmcrs::E1Stsen,
        Tcmcrs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcmcrs::E1Stsen,
            tcmcrs::E1Stsen,
            Tcmcrs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC Test Enable/ECC Bypass Select"]
    #[inline(always)]
    pub fn tstbyp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        tcmcrs::Tstbyp,
        tcmcrs::Tstbyp,
        Tcmcrs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            tcmcrs::Tstbyp,
            tcmcrs::Tstbyp,
            Tcmcrs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tcmcrs {
    #[inline(always)]
    fn default() -> Tcmcrs {
        <crate::RegValueT<Tcmcrs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcmcrs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct E1Stsen_SPEC;
    pub type E1Stsen = crate::EnumBitfieldStruct<u8, E1Stsen_SPEC>;
    impl E1Stsen {
        #[doc = "Disable updating of 1-bit ECC error information"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable updating of 1-bit ECC error information"]
        pub const _1: Self = Self::new(1);
    }
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
pub struct Tcmesr_SPEC;
impl crate::sealed::RegSpec for Tcmesr_SPEC {
    type DataType = u16;
}

#[doc = "TCM Error Status Register"]
pub type Tcmesr = crate::RegValueT<Tcmesr_SPEC>;

impl Tcmesr {
    #[doc = "C-TCM 1-bit ECC Error Status"]
    #[inline(always)]
    pub fn errc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tcmesr::Errc0,
        tcmesr::Errc0,
        Tcmesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tcmesr::Errc0,
            tcmesr::Errc0,
            Tcmesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "C-TCM 2-bit ECC Error Status"]
    #[inline(always)]
    pub fn errc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tcmesr::Errc1,
        tcmesr::Errc1,
        Tcmesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tcmesr::Errc1,
            tcmesr::Errc1,
            Tcmesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "S-TCM 1-bit ECC Error Status"]
    #[inline(always)]
    pub fn errs0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        tcmesr::Errs0,
        tcmesr::Errs0,
        Tcmesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            tcmesr::Errs0,
            tcmesr::Errs0,
            Tcmesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "S-TCM 2-bit ECC Error Status"]
    #[inline(always)]
    pub fn errs1(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        tcmesr::Errs1,
        tcmesr::Errs1,
        Tcmesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            tcmesr::Errs1,
            tcmesr::Errs1,
            Tcmesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tcmesr {
    #[inline(always)]
    fn default() -> Tcmesr {
        <crate::RegValueT<Tcmesr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcmesr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Errc0_SPEC;
    pub type Errc0 = crate::EnumBitfieldStruct<u8, Errc0_SPEC>;
    impl Errc0 {
        #[doc = "1-bit ECC error has not occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "1-bit ECC error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Errc1_SPEC;
    pub type Errc1 = crate::EnumBitfieldStruct<u8, Errc1_SPEC>;
    impl Errc1 {
        #[doc = "2-bit ECC error has not occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "2-bit ECC error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Errs0_SPEC;
    pub type Errs0 = crate::EnumBitfieldStruct<u8, Errs0_SPEC>;
    impl Errs0 {
        #[doc = "1-bit ECC error has not occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "1-bit ECC error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Errs1_SPEC;
    pub type Errs1 = crate::EnumBitfieldStruct<u8, Errs1_SPEC>;
    impl Errs1 {
        #[doc = "2-bit ECC error has not occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "2-bit ECC error has occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcmesclr_SPEC;
impl crate::sealed::RegSpec for Tcmesclr_SPEC {
    type DataType = u16;
}

#[doc = "TCM Error Status Clear Register"]
pub type Tcmesclr = crate::RegValueT<Tcmesclr_SPEC>;

impl Tcmesclr {
    #[doc = "C-TCM 1-bit ECC Error Status Clear"]
    #[inline(always)]
    pub fn clrc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tcmesclr::Clrc0,
        tcmesclr::Clrc0,
        Tcmesclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tcmesclr::Clrc0,
            tcmesclr::Clrc0,
            Tcmesclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "C-TCM 2-bit ECC Error Status Clear"]
    #[inline(always)]
    pub fn clrc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tcmesclr::Clrc1,
        tcmesclr::Clrc1,
        Tcmesclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tcmesclr::Clrc1,
            tcmesclr::Clrc1,
            Tcmesclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "S-TCM 1-bit ECC Error Status Clear"]
    #[inline(always)]
    pub fn clrs0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        tcmesclr::Clrs0,
        tcmesclr::Clrs0,
        Tcmesclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            tcmesclr::Clrs0,
            tcmesclr::Clrs0,
            Tcmesclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "S-TCM 2-bit ECC Error Status Clear"]
    #[inline(always)]
    pub fn clrs1(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        tcmesclr::Clrs1,
        tcmesclr::Clrs1,
        Tcmesclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            tcmesclr::Clrs1,
            tcmesclr::Clrs1,
            Tcmesclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tcmesclr {
    #[inline(always)]
    fn default() -> Tcmesclr {
        <crate::RegValueT<Tcmesclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcmesclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clrc0_SPEC;
    pub type Clrc0 = crate::EnumBitfieldStruct<u8, Clrc0_SPEC>;
    impl Clrc0 {
        #[doc = "Clear error"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clrc1_SPEC;
    pub type Clrc1 = crate::EnumBitfieldStruct<u8, Clrc1_SPEC>;
    impl Clrc1 {
        #[doc = "Clear error"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clrs0_SPEC;
    pub type Clrs0 = crate::EnumBitfieldStruct<u8, Clrs0_SPEC>;
    impl Clrs0 {
        #[doc = "Clear error"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clrs1_SPEC;
    pub type Clrs1 = crate::EnumBitfieldStruct<u8, Clrs1_SPEC>;
    impl Clrs1 {
        #[doc = "Clear error"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcmearc0_SPEC;
impl crate::sealed::RegSpec for Tcmearc0_SPEC {
    type DataType = u32;
}

#[doc = "TCM Error Address Register"]
pub type Tcmearc0 = crate::RegValueT<Tcmearc0_SPEC>;

impl Tcmearc0 {
    #[doc = "When an SRAM error occurs, an error address is stored."]
    #[inline(always)]
    pub fn ear(
        self,
    ) -> crate::common::RegisterField<2, 0xffff, 1, 0, u16, u16, Tcmearc0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0xffff,1,0,u16,u16,Tcmearc0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcmearc0 {
    #[inline(always)]
    fn default() -> Tcmearc0 {
        <crate::RegValueT<Tcmearc0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcmearc1_SPEC;
impl crate::sealed::RegSpec for Tcmearc1_SPEC {
    type DataType = u32;
}

#[doc = "TCM Error Address Register"]
pub type Tcmearc1 = crate::RegValueT<Tcmearc1_SPEC>;

impl Tcmearc1 {
    #[doc = "When an SRAM error occurs, an error address is stored."]
    #[inline(always)]
    pub fn ear(
        self,
    ) -> crate::common::RegisterField<2, 0xffff, 1, 0, u16, u16, Tcmearc1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0xffff,1,0,u16,u16,Tcmearc1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcmearc1 {
    #[inline(always)]
    fn default() -> Tcmearc1 {
        <crate::RegValueT<Tcmearc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcmears0_SPEC;
impl crate::sealed::RegSpec for Tcmears0_SPEC {
    type DataType = u32;
}

#[doc = "TCM Error Address Register"]
pub type Tcmears0 = crate::RegValueT<Tcmears0_SPEC>;

impl Tcmears0 {
    #[doc = "When an SRAM error occurs, an error address is stored."]
    #[inline(always)]
    pub fn ear(
        self,
    ) -> crate::common::RegisterField<2, 0xffff, 1, 0, u16, u16, Tcmears0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0xffff,1,0,u16,u16,Tcmears0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcmears0 {
    #[inline(always)]
    fn default() -> Tcmears0 {
        <crate::RegValueT<Tcmears0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcmears1_SPEC;
impl crate::sealed::RegSpec for Tcmears1_SPEC {
    type DataType = u32;
}

#[doc = "TCM Error Address Register"]
pub type Tcmears1 = crate::RegValueT<Tcmears1_SPEC>;

impl Tcmears1 {
    #[doc = "When an SRAM error occurs, an error address is stored."]
    #[inline(always)]
    pub fn ear(
        self,
    ) -> crate::common::RegisterField<2, 0xffff, 1, 0, u16, u16, Tcmears1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0xffff,1,0,u16,u16,Tcmears1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcmears1 {
    #[inline(always)]
    fn default() -> Tcmears1 {
        <crate::RegValueT<Tcmears1_SPEC> as RegisterValue<_>>::new(0)
    }
}
