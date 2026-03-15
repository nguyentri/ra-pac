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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:57:20 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Data Transfer Controller"]
unsafe impl ::core::marker::Send for super::Dtc {}
unsafe impl ::core::marker::Sync for super::Dtc {}
impl super::Dtc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "DTC Control Register"]
    #[inline(always)]
    pub const fn dtccr(&self) -> &'static crate::common::Reg<self::Dtccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dtccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "DTC Control Register for Non Secure Region"]
    #[inline(always)]
    pub const fn dtccr_ns(
        &self,
    ) -> &'static crate::common::Reg<self::DtccrNs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DtccrNs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "DTC Vector Base Register"]
    #[inline(always)]
    pub const fn dtcvbr(
        &self,
    ) -> &'static crate::common::Reg<self::Dtcvbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dtcvbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "DTC Vector Base Register for Non Secure Region"]
    #[inline(always)]
    pub const fn dtcvbr_ns(
        &self,
    ) -> &'static crate::common::Reg<self::DtcvbrNs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DtcvbrNs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "DTC Module Start Register"]
    #[inline(always)]
    pub const fn dtcst(&self) -> &'static crate::common::Reg<self::Dtcst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dtcst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "DTC Status Register"]
    #[inline(always)]
    pub const fn dtcsts(&self) -> &'static crate::common::Reg<self::Dtcsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dtcsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "DTC Control Register for Secure Region"]
    #[inline(always)]
    pub const fn dtccr_s(
        &self,
    ) -> &'static crate::common::Reg<self::DtccrS_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DtccrS_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "DTC Control Register for Secure Region"]
    #[inline(always)]
    pub const fn dtccr_sec(
        &self,
    ) -> &'static crate::common::Reg<self::DtccrSec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DtccrSec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "DTC Vector Base Register for secure Region"]
    #[inline(always)]
    pub const fn dtcvbr_s(
        &self,
    ) -> &'static crate::common::Reg<self::DtcvbrS_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DtcvbrS_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "DTC Vector Base Register for secure Region"]
    #[inline(always)]
    pub const fn dtcvbr_sec(
        &self,
    ) -> &'static crate::common::Reg<self::DtcvbrSec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DtcvbrSec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "DTC Error Vector Register"]
    #[inline(always)]
    pub const fn dtevr(&self) -> &'static crate::common::Reg<self::Dtevr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dtevr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtccr_SPEC;
impl crate::sealed::RegSpec for Dtccr_SPEC {
    type DataType = u8;
}

#[doc = "DTC Control Register"]
pub type Dtccr = crate::RegValueT<Dtccr_SPEC>;

impl NoBitfieldReg<Dtccr_SPEC> for Dtccr {}
impl ::core::default::Default for Dtccr {
    #[inline(always)]
    fn default() -> Dtccr {
        <crate::RegValueT<Dtccr_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DtccrNs_SPEC;
impl crate::sealed::RegSpec for DtccrNs_SPEC {
    type DataType = u8;
}

#[doc = "DTC Control Register for Non Secure Region"]
pub type DtccrNs = crate::RegValueT<DtccrNs_SPEC>;

impl DtccrNs {
    #[doc = "DTC Transfer Information Read Skip Enable."]
    #[inline(always)]
    pub fn rrs(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dtccr_ns::Rrs,
        dtccr_ns::Rrs,
        DtccrNs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dtccr_ns::Rrs,
            dtccr_ns::Rrs,
            DtccrNs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, DtccrNs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,DtccrNs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DtccrNs {
    #[inline(always)]
    fn default() -> DtccrNs {
        <crate::RegValueT<DtccrNs_SPEC> as RegisterValue<_>>::new(8)
    }
}
pub mod dtccr_ns {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rrs_SPEC;
    pub type Rrs = crate::EnumBitfieldStruct<u8, Rrs_SPEC>;
    impl Rrs {
        #[doc = "Do not skip transfer information read"]
        pub const _0: Self = Self::new(0);

        #[doc = "Skip transfer information read when vector numbers match"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcvbr_SPEC;
impl crate::sealed::RegSpec for Dtcvbr_SPEC {
    type DataType = u32;
}

#[doc = "DTC Vector Base Register"]
pub type Dtcvbr = crate::RegValueT<Dtcvbr_SPEC>;

impl NoBitfieldReg<Dtcvbr_SPEC> for Dtcvbr {}
impl ::core::default::Default for Dtcvbr {
    #[inline(always)]
    fn default() -> Dtcvbr {
        <crate::RegValueT<Dtcvbr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DtcvbrNs_SPEC;
impl crate::sealed::RegSpec for DtcvbrNs_SPEC {
    type DataType = u32;
}

#[doc = "DTC Vector Base Register for Non Secure Region"]
pub type DtcvbrNs = crate::RegValueT<DtcvbrNs_SPEC>;

impl DtcvbrNs {
    #[doc = "DTC Vector Base Address for non-secure region(Upper 22 bits)"]
    #[inline(always)]
    pub fn dtcvbr(
        self,
    ) -> crate::common::RegisterField<10, 0x3fffff, 1, 0, u32, u32, DtcvbrNs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3fffff,1,0,u32,u32,DtcvbrNs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DtcvbrNs {
    #[inline(always)]
    fn default() -> DtcvbrNs {
        <crate::RegValueT<DtcvbrNs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcst_SPEC;
impl crate::sealed::RegSpec for Dtcst_SPEC {
    type DataType = u8;
}

#[doc = "DTC Module Start Register"]
pub type Dtcst = crate::RegValueT<Dtcst_SPEC>;

impl Dtcst {
    #[doc = "DTC Module Start"]
    #[inline(always)]
    pub fn dtcst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dtcst::Dtcst,
        dtcst::Dtcst,
        Dtcst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dtcst::Dtcst,
            dtcst::Dtcst,
            Dtcst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Dtcst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Dtcst_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dtcst {
    #[inline(always)]
    fn default() -> Dtcst {
        <crate::RegValueT<Dtcst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dtcst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtcst_SPEC;
    pub type Dtcst = crate::EnumBitfieldStruct<u8, Dtcst_SPEC>;
    impl Dtcst {
        #[doc = "DTC module stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC module start"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcsts_SPEC;
impl crate::sealed::RegSpec for Dtcsts_SPEC {
    type DataType = u16;
}

#[doc = "DTC Status Register"]
pub type Dtcsts = crate::RegValueT<Dtcsts_SPEC>;

impl Dtcsts {
    #[doc = "DTC-Activating Vector Number MonitoringThese bits indicate the vector number for the activating source when DTC transfer is in progress.The value is only valid if DTC transfer is in progress (the value of the ACT flag is 1)"]
    #[inline(always)]
    pub fn vecn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dtcsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dtcsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, Dtcsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,Dtcsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DTC Active Flag"]
    #[inline(always)]
    pub fn act(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        dtcsts::Act,
        dtcsts::Act,
        Dtcsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            dtcsts::Act,
            dtcsts::Act,
            Dtcsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dtcsts {
    #[inline(always)]
    fn default() -> Dtcsts {
        <crate::RegValueT<Dtcsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dtcsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Act_SPEC;
    pub type Act = crate::EnumBitfieldStruct<u8, Act_SPEC>;
    impl Act {
        #[doc = "DTC transfer operation is not in progress."]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC transfer operation is in progress."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DtccrS_SPEC;
impl crate::sealed::RegSpec for DtccrS_SPEC {
    type DataType = u8;
}

#[doc = "DTC Control Register for Secure Region"]
pub type DtccrS = crate::RegValueT<DtccrS_SPEC>;

impl DtccrS {
    #[doc = "DTC Transfer Information Read Skip Enable for secure"]
    #[inline(always)]
    pub fn rrss(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dtccr_s::Rrss,
        dtccr_s::Rrss,
        DtccrS_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dtccr_s::Rrss,
            dtccr_s::Rrss,
            DtccrS_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, DtccrS_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,DtccrS_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DtccrS {
    #[inline(always)]
    fn default() -> DtccrS {
        <crate::RegValueT<DtccrS_SPEC> as RegisterValue<_>>::new(8)
    }
}
pub mod dtccr_s {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rrss_SPEC;
    pub type Rrss = crate::EnumBitfieldStruct<u8, Rrss_SPEC>;
    impl Rrss {
        #[doc = "Transfer information read is not skipped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transfer information read is skipped when vector numbers match."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DtccrSec_SPEC;
impl crate::sealed::RegSpec for DtccrSec_SPEC {
    type DataType = u8;
}

#[doc = "DTC Control Register for Secure Region"]
pub type DtccrSec = crate::RegValueT<DtccrSec_SPEC>;

impl NoBitfieldReg<DtccrSec_SPEC> for DtccrSec {}
impl ::core::default::Default for DtccrSec {
    #[inline(always)]
    fn default() -> DtccrSec {
        <crate::RegValueT<DtccrSec_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DtcvbrS_SPEC;
impl crate::sealed::RegSpec for DtcvbrS_SPEC {
    type DataType = u32;
}

#[doc = "DTC Vector Base Register for secure Region"]
pub type DtcvbrS = crate::RegValueT<DtcvbrS_SPEC>;

impl DtcvbrS {
    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, DtcvbrS_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,DtcvbrS_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DTC Vector Base Address for secure region(Upper 22 bits)"]
    #[inline(always)]
    pub fn dtcvbrs(
        self,
    ) -> crate::common::RegisterField<10, 0x3fffff, 1, 0, u32, u32, DtcvbrS_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3fffff,1,0,u32,u32,DtcvbrS_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DtcvbrS {
    #[inline(always)]
    fn default() -> DtcvbrS {
        <crate::RegValueT<DtcvbrS_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DtcvbrSec_SPEC;
impl crate::sealed::RegSpec for DtcvbrSec_SPEC {
    type DataType = u32;
}

#[doc = "DTC Vector Base Register for secure Region"]
pub type DtcvbrSec = crate::RegValueT<DtcvbrSec_SPEC>;

impl NoBitfieldReg<DtcvbrSec_SPEC> for DtcvbrSec {}
impl ::core::default::Default for DtcvbrSec {
    #[inline(always)]
    fn default() -> DtcvbrSec {
        <crate::RegValueT<DtcvbrSec_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtevr_SPEC;
impl crate::sealed::RegSpec for Dtevr_SPEC {
    type DataType = u32;
}

#[doc = "DTC Error Vector Register"]
pub type Dtevr = crate::RegValueT<Dtevr_SPEC>;

impl Dtevr {
    #[doc = "DTC Error Vector Number"]
    #[inline(always)]
    pub fn dtev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dtevr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dtevr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DTC Error Vector Number SA Monitor"]
    #[inline(always)]
    pub fn dtevsam(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dtevr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Dtevr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DTC Error Status Flag"]
    #[inline(always)]
    pub fn dtesta(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dtevr::Dtesta,
        dtevr::Dtesta,
        Dtevr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dtevr::Dtesta,
            dtevr::Dtesta,
            Dtevr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, u16, Dtevr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x7fff,1,0,u16,u16,Dtevr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dtevr {
    #[inline(always)]
    fn default() -> Dtevr {
        <crate::RegValueT<Dtevr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dtevr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtesta_SPEC;
    pub type Dtesta = crate::EnumBitfieldStruct<u8, Dtesta_SPEC>;
    impl Dtesta {
        #[doc = "Not DTC transfer error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "DMA transfer error occurred"]
        pub const _1: Self = Self::new(1);
    }
}
