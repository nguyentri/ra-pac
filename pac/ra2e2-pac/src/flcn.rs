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
// Generated from SVD 1.40.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:02:12 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Flash I/O Registers"]
unsafe impl ::core::marker::Send for super::Flcn {}
unsafe impl ::core::marker::Sync for super::Flcn {}
impl super::Flcn {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Data Flash Control Register"]
    #[inline(always)]
    pub const fn dflctl(
        &self,
    ) -> &'static crate::common::Reg<self::Dflctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dflctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Flash P/E Mode Control Register"]
    #[inline(always)]
    pub const fn fpmcr(&self) -> &'static crate::common::Reg<self::Fpmcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fpmcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Flash Area Select Register"]
    #[inline(always)]
    pub const fn fasr(&self) -> &'static crate::common::Reg<self::Fasr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fasr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "Flash Processing Start Address Register L"]
    #[inline(always)]
    pub const fn fsarl(&self) -> &'static crate::common::Reg<self::Fsarl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fsarl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "Flash Processing Start Address Register H"]
    #[inline(always)]
    pub const fn fsarh(&self) -> &'static crate::common::Reg<self::Fsarh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fsarh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "Flash Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &'static crate::common::Reg<self::Fcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(276usize),
            )
        }
    }

    #[doc = "Flash Processing End Address Register L"]
    #[inline(always)]
    pub const fn fearl(&self) -> &'static crate::common::Reg<self::Fearl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fearl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(280usize),
            )
        }
    }

    #[doc = "Flash Processing End Address Register H"]
    #[inline(always)]
    pub const fn fearh(&self) -> &'static crate::common::Reg<self::Fearh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fearh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "Flash Reset Register"]
    #[inline(always)]
    pub const fn fresetr(
        &self,
    ) -> &'static crate::common::Reg<self::Fresetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fresetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[doc = "Flash Status Register 1"]
    #[inline(always)]
    pub const fn fstatr1(
        &self,
    ) -> &'static crate::common::Reg<self::Fstatr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fstatr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(300usize),
            )
        }
    }

    #[doc = "Flash Write Buffer Register L0"]
    #[inline(always)]
    pub const fn fwbl0(&self) -> &'static crate::common::Reg<self::Fwbl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwbl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[doc = "Flash Write Buffer Register H0"]
    #[inline(always)]
    pub const fn fwbh0(&self) -> &'static crate::common::Reg<self::Fwbh0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwbh0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(312usize),
            )
        }
    }

    #[doc = "Protection Unlock Register"]
    #[inline(always)]
    pub const fn fpr(&self) -> &'static crate::common::Reg<self::Fpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[doc = "Protection Unlock Status Register"]
    #[inline(always)]
    pub const fn fpsr(&self) -> &'static crate::common::Reg<self::Fpsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fpsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(388usize),
            )
        }
    }

    #[doc = "Flash Read Buffer Register L0"]
    #[inline(always)]
    pub const fn frbl0(&self) -> &'static crate::common::Reg<self::Frbl0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frbl0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(392usize),
            )
        }
    }

    #[doc = "Flash Read Buffer Register H0"]
    #[inline(always)]
    pub const fn frbh0(&self) -> &'static crate::common::Reg<self::Frbh0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frbh0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(400usize),
            )
        }
    }

    #[doc = "Flash Start-Up Setting Monitor Register"]
    #[inline(always)]
    pub const fn fscmr(&self) -> &'static crate::common::Reg<self::Fscmr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fscmr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(448usize),
            )
        }
    }

    #[doc = "Flash Access Window Start Address Monitor Register"]
    #[inline(always)]
    pub const fn fawsmr(&self) -> &'static crate::common::Reg<self::Fawsmr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fawsmr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(456usize),
            )
        }
    }

    #[doc = "Flash Access Window End Address Monitor Register"]
    #[inline(always)]
    pub const fn fawemr(&self) -> &'static crate::common::Reg<self::Fawemr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fawemr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(464usize),
            )
        }
    }

    #[doc = "Flash Initial Setting Register"]
    #[inline(always)]
    pub const fn fisr(&self) -> &'static crate::common::Reg<self::Fisr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fisr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(472usize),
            )
        }
    }

    #[doc = "Flash Extra Area Control Register"]
    #[inline(always)]
    pub const fn fexcr(&self) -> &'static crate::common::Reg<self::Fexcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fexcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(476usize),
            )
        }
    }

    #[doc = "Flash Error Address Monitor Register L"]
    #[inline(always)]
    pub const fn feaml(&self) -> &'static crate::common::Reg<self::Feaml_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Feaml_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(480usize),
            )
        }
    }

    #[doc = "Flash Error Address Monitor Register H"]
    #[inline(always)]
    pub const fn feamh(&self) -> &'static crate::common::Reg<self::Feamh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Feamh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(488usize),
            )
        }
    }

    #[doc = "Flash Status Register 2"]
    #[inline(always)]
    pub const fn fstatr2(
        &self,
    ) -> &'static crate::common::Reg<self::Fstatr2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fstatr2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(496usize),
            )
        }
    }

    #[doc = "Temperature Sensor Calibration Data Register"]
    #[inline(always)]
    pub const fn tscdr(&self) -> &'static crate::common::Reg<self::Tscdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tscdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(552usize),
            )
        }
    }

    #[doc = "Flash P/E Mode Entry Register"]
    #[inline(always)]
    pub const fn fentryr(
        &self,
    ) -> &'static crate::common::Reg<self::Fentryr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fentryr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16304usize),
            )
        }
    }

    #[doc = "Memory Wait Cycle Control Register for Data Flash"]
    #[inline(always)]
    pub const fn fldwaitr(
        &self,
    ) -> &'static crate::common::Reg<self::Fldwaitr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fldwaitr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16324usize),
            )
        }
    }

    #[doc = "Prefetch Buffer Enable Register"]
    #[inline(always)]
    pub const fn pfber(&self) -> &'static crate::common::Reg<self::Pfber_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pfber_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16328usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dflctl_SPEC;
impl crate::sealed::RegSpec for Dflctl_SPEC {
    type DataType = u8;
}

#[doc = "Data Flash Control Register"]
pub type Dflctl = crate::RegValueT<Dflctl_SPEC>;

impl Dflctl {
    #[doc = "Data Flash Access Enable"]
    #[inline(always)]
    pub fn dflen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dflctl::Dflen,
        dflctl::Dflen,
        Dflctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dflctl::Dflen,
            dflctl::Dflen,
            Dflctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dflctl {
    #[inline(always)]
    fn default() -> Dflctl {
        <crate::RegValueT<Dflctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dflctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dflen_SPEC;
    pub type Dflen = crate::EnumBitfieldStruct<u8, Dflen_SPEC>;
    impl Dflen {
        #[doc = "Access to the data flash is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Access to the data flash is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpmcr_SPEC;
impl crate::sealed::RegSpec for Fpmcr_SPEC {
    type DataType = u8;
}

#[doc = "Flash P/E Mode Control Register"]
pub type Fpmcr = crate::RegValueT<Fpmcr_SPEC>;

impl Fpmcr {
    #[doc = "Flash Operating Mode Select 0"]
    #[inline(always)]
    pub fn fms0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fpmcr::Fms0,
        fpmcr::Fms0,
        Fpmcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fpmcr::Fms0,
            fpmcr::Fms0,
            Fpmcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Code Flash P/E Disable"]
    #[inline(always)]
    pub fn rpdis(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fpmcr::Rpdis,
        fpmcr::Rpdis,
        Fpmcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fpmcr::Rpdis,
            fpmcr::Rpdis,
            Fpmcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Flash Operating Mode Select 1"]
    #[inline(always)]
    pub fn fms1(self) -> crate::common::RegisterFieldBool<4, 1, 0, Fpmcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Fpmcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fpmcr {
    #[inline(always)]
    fn default() -> Fpmcr {
        <crate::RegValueT<Fpmcr_SPEC> as RegisterValue<_>>::new(8)
    }
}
pub mod fpmcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fms0_SPEC;
    pub type Fms0 = crate::EnumBitfieldStruct<u8, Fms0_SPEC>;
    impl Fms0 {
        #[doc = "FMS1 = 0: Read mode FMS1 = 1: Data flash P/E mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "FMS1 = 0: Code flash P/E mode FMS1 = 1: Setting prohibited."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpdis_SPEC;
    pub type Rpdis = crate::EnumBitfieldStruct<u8, Rpdis_SPEC>;
    impl Rpdis {
        #[doc = "Programming of the code flash is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Programming of the code flash is disabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fasr_SPEC;
impl crate::sealed::RegSpec for Fasr_SPEC {
    type DataType = u8;
}

#[doc = "Flash Area Select Register"]
pub type Fasr = crate::RegValueT<Fasr_SPEC>;

impl Fasr {
    #[doc = "Extra Area Select"]
    #[inline(always)]
    pub fn exs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fasr::Exs,
        fasr::Exs,
        Fasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fasr::Exs,
            fasr::Exs,
            Fasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fasr {
    #[inline(always)]
    fn default() -> Fasr {
        <crate::RegValueT<Fasr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fasr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exs_SPEC;
    pub type Exs = crate::EnumBitfieldStruct<u8, Exs_SPEC>;
    impl Exs {
        #[doc = "User area or data area"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extra area."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsarl_SPEC;
impl crate::sealed::RegSpec for Fsarl_SPEC {
    type DataType = u16;
}

#[doc = "Flash Processing Start Address Register L"]
pub type Fsarl = crate::RegValueT<Fsarl_SPEC>;

impl Fsarl {
    #[doc = "Flash Processing Start Address L"]
    #[inline(always)]
    pub fn fsarl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fsarl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fsarl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fsarl {
    #[inline(always)]
    fn default() -> Fsarl {
        <crate::RegValueT<Fsarl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsarh_SPEC;
impl crate::sealed::RegSpec for Fsarh_SPEC {
    type DataType = u16;
}

#[doc = "Flash Processing Start Address Register H"]
pub type Fsarh = crate::RegValueT<Fsarh_SPEC>;

impl Fsarh {
    #[doc = "Flash Processing Start Address H"]
    #[inline(always)]
    pub fn fsarh(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fsarh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fsarh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fsarh {
    #[inline(always)]
    fn default() -> Fsarh {
        <crate::RegValueT<Fsarh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcr_SPEC;
impl crate::sealed::RegSpec for Fcr_SPEC {
    type DataType = u8;
}

#[doc = "Flash Control Register"]
pub type Fcr = crate::RegValueT<Fcr_SPEC>;

impl Fcr {
    #[doc = "Software Command Setting"]
    #[inline(always)]
    pub fn cmd(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, fcr::Cmd, fcr::Cmd, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,fcr::Cmd,fcr::Cmd,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data Read Completion"]
    #[inline(always)]
    pub fn drc(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, fcr::Drc, fcr::Drc, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,fcr::Drc,fcr::Drc,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Forced Processing Stop"]
    #[inline(always)]
    pub fn stop(self) -> crate::common::RegisterFieldBool<6, 1, 0, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Fcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Processing Start"]
    #[inline(always)]
    pub fn opst(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, fcr::Opst, fcr::Opst, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fcr::Opst,
            fcr::Opst,
            Fcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fcr {
    #[inline(always)]
    fn default() -> Fcr {
        <crate::RegValueT<Fcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmd_SPEC;
    pub type Cmd = crate::EnumBitfieldStruct<u8, Cmd_SPEC>;
    impl Cmd {
        #[doc = "Program"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "Blank check (code flash)"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "Block erase"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "Consecutive read"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "Chip erase"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "Blank check (data flash)"]
        pub const _0_X_B: Self = Self::new(11);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drc_SPEC;
    pub type Drc = crate::EnumBitfieldStruct<u8, Drc_SPEC>;
    impl Drc {
        #[doc = "Data is not read or next data is requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data reading is complete."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opst_SPEC;
    pub type Opst = crate::EnumBitfieldStruct<u8, Opst_SPEC>;
    impl Opst {
        #[doc = "Processing stops"]
        pub const _0: Self = Self::new(0);

        #[doc = "Processing starts."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fearl_SPEC;
impl crate::sealed::RegSpec for Fearl_SPEC {
    type DataType = u16;
}

#[doc = "Flash Processing End Address Register L"]
pub type Fearl = crate::RegValueT<Fearl_SPEC>;

impl Fearl {
    #[doc = "Flash Processing End Address L"]
    #[inline(always)]
    pub fn fearl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fearl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fearl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fearl {
    #[inline(always)]
    fn default() -> Fearl {
        <crate::RegValueT<Fearl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fearh_SPEC;
impl crate::sealed::RegSpec for Fearh_SPEC {
    type DataType = u16;
}

#[doc = "Flash Processing End Address Register H"]
pub type Fearh = crate::RegValueT<Fearh_SPEC>;

impl Fearh {
    #[doc = "Flash Processing End Address H"]
    #[inline(always)]
    pub fn fearh(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fearh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fearh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fearh {
    #[inline(always)]
    fn default() -> Fearh {
        <crate::RegValueT<Fearh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fresetr_SPEC;
impl crate::sealed::RegSpec for Fresetr_SPEC {
    type DataType = u8;
}

#[doc = "Flash Reset Register"]
pub type Fresetr = crate::RegValueT<Fresetr_SPEC>;

impl Fresetr {
    #[doc = "Software reset of the registers"]
    #[inline(always)]
    pub fn freset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fresetr::Freset,
        fresetr::Freset,
        Fresetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fresetr::Freset,
            fresetr::Freset,
            Fresetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fresetr {
    #[inline(always)]
    fn default() -> Fresetr {
        <crate::RegValueT<Fresetr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fresetr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Freset_SPEC;
    pub type Freset = crate::EnumBitfieldStruct<u8, Freset_SPEC>;
    impl Freset {
        #[doc = "The registers related to the flash programming are not reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "The registers related to the flash programming are reset."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fstatr1_SPEC;
impl crate::sealed::RegSpec for Fstatr1_SPEC {
    type DataType = u8;
}

#[doc = "Flash Status Register 1"]
pub type Fstatr1 = crate::RegValueT<Fstatr1_SPEC>;

impl Fstatr1 {
    #[doc = "Data Read Ready Flag"]
    #[inline(always)]
    pub fn drrdy(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fstatr1::Drrdy,
        fstatr1::Drrdy,
        Fstatr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fstatr1::Drrdy,
            fstatr1::Drrdy,
            Fstatr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Flash Ready Flag"]
    #[inline(always)]
    pub fn frdy(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        fstatr1::Frdy,
        fstatr1::Frdy,
        Fstatr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            fstatr1::Frdy,
            fstatr1::Frdy,
            Fstatr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Extra Area Ready Flag"]
    #[inline(always)]
    pub fn exrdy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fstatr1::Exrdy,
        fstatr1::Exrdy,
        Fstatr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fstatr1::Exrdy,
            fstatr1::Exrdy,
            Fstatr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fstatr1 {
    #[inline(always)]
    fn default() -> Fstatr1 {
        <crate::RegValueT<Fstatr1_SPEC> as RegisterValue<_>>::new(4)
    }
}
pub mod fstatr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drrdy_SPEC;
    pub type Drrdy = crate::EnumBitfieldStruct<u8, Drrdy_SPEC>;
    impl Drrdy {
        #[doc = "The read processing of the consecutive read command at each address is not terminated."]
        pub const _0: Self = Self::new(0);

        #[doc = "The read processing of the consecutive read command at each address is terminated and read data is stored to the FRBH and FRBL registers."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frdy_SPEC;
    pub type Frdy = crate::EnumBitfieldStruct<u8, Frdy_SPEC>;
    impl Frdy {
        #[doc = "The software command of the FCR register is not terminated."]
        pub const _0: Self = Self::new(0);

        #[doc = "The software command of the FCR register is terminated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exrdy_SPEC;
    pub type Exrdy = crate::EnumBitfieldStruct<u8, Exrdy_SPEC>;
    impl Exrdy {
        #[doc = "The software command of the FEXCR register is not terminated."]
        pub const _0: Self = Self::new(0);

        #[doc = "The software command of the FEXCR register is terminated."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwbl0_SPEC;
impl crate::sealed::RegSpec for Fwbl0_SPEC {
    type DataType = u16;
}

#[doc = "Flash Write Buffer Register L0"]
pub type Fwbl0 = crate::RegValueT<Fwbl0_SPEC>;

impl Fwbl0 {
    #[doc = "Flash Write Buffer L0"]
    #[inline(always)]
    pub fn wdata(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwbl0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwbl0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwbl0 {
    #[inline(always)]
    fn default() -> Fwbl0 {
        <crate::RegValueT<Fwbl0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwbh0_SPEC;
impl crate::sealed::RegSpec for Fwbh0_SPEC {
    type DataType = u16;
}

#[doc = "Flash Write Buffer Register H0"]
pub type Fwbh0 = crate::RegValueT<Fwbh0_SPEC>;

impl Fwbh0 {
    #[doc = "Flash Write Buffer H0"]
    #[inline(always)]
    pub fn wdata(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwbh0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwbh0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwbh0 {
    #[inline(always)]
    fn default() -> Fwbh0 {
        <crate::RegValueT<Fwbh0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpr_SPEC;
impl crate::sealed::RegSpec for Fpr_SPEC {
    type DataType = u8;
}

#[doc = "Protection Unlock Register"]
pub type Fpr = crate::RegValueT<Fpr_SPEC>;

impl Fpr {
    #[doc = "Protection Unlock"]
    #[inline(always)]
    pub fn fpr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fpr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fpr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fpr {
    #[inline(always)]
    fn default() -> Fpr {
        <crate::RegValueT<Fpr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpsr_SPEC;
impl crate::sealed::RegSpec for Fpsr_SPEC {
    type DataType = u8;
}

#[doc = "Protection Unlock Status Register"]
pub type Fpsr = crate::RegValueT<Fpsr_SPEC>;

impl Fpsr {
    #[doc = "Protect Error Flag"]
    #[inline(always)]
    pub fn perr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fpsr::Perr,
        fpsr::Perr,
        Fpsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fpsr::Perr,
            fpsr::Perr,
            Fpsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fpsr {
    #[inline(always)]
    fn default() -> Fpsr {
        <crate::RegValueT<Fpsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fpsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Perr_SPEC;
    pub type Perr = crate::EnumBitfieldStruct<u8, Perr_SPEC>;
    impl Perr {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error occurs"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frbl0_SPEC;
impl crate::sealed::RegSpec for Frbl0_SPEC {
    type DataType = u16;
}

#[doc = "Flash Read Buffer Register L0"]
pub type Frbl0 = crate::RegValueT<Frbl0_SPEC>;

impl Frbl0 {
    #[doc = "Flash Read Buffer L0"]
    #[inline(always)]
    pub fn rdata(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Frbl0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Frbl0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Frbl0 {
    #[inline(always)]
    fn default() -> Frbl0 {
        <crate::RegValueT<Frbl0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frbh0_SPEC;
impl crate::sealed::RegSpec for Frbh0_SPEC {
    type DataType = u16;
}

#[doc = "Flash Read Buffer Register H0"]
pub type Frbh0 = crate::RegValueT<Frbh0_SPEC>;

impl Frbh0 {
    #[doc = "Flash Read Buffer H0"]
    #[inline(always)]
    pub fn rdata(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Frbh0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Frbh0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Frbh0 {
    #[inline(always)]
    fn default() -> Frbh0 {
        <crate::RegValueT<Frbh0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fscmr_SPEC;
impl crate::sealed::RegSpec for Fscmr_SPEC {
    type DataType = u16;
}

#[doc = "Flash Start-Up Setting Monitor Register"]
pub type Fscmr = crate::RegValueT<Fscmr_SPEC>;

impl Fscmr {
    #[doc = "Startup Area Setting Monitor Flag"]
    #[inline(always)]
    pub fn sasmf(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        fscmr::Sasmf,
        fscmr::Sasmf,
        Fscmr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            fscmr::Sasmf,
            fscmr::Sasmf,
            Fscmr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Access Window Protection Flag"]
    #[inline(always)]
    pub fn fspr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        fscmr::Fspr,
        fscmr::Fspr,
        Fscmr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            fscmr::Fspr,
            fscmr::Fspr,
            Fscmr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fscmr {
    #[inline(always)]
    fn default() -> Fscmr {
        <crate::RegValueT<Fscmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fscmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sasmf_SPEC;
    pub type Sasmf = crate::EnumBitfieldStruct<u8, Sasmf_SPEC>;
    impl Sasmf {
        #[doc = "Setting to start up using the alternative area"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting to start up using the default area"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fspr_SPEC;
    pub type Fspr = crate::EnumBitfieldStruct<u8, Fspr_SPEC>;
    impl Fspr {
        #[doc = "Access window setting disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Access window setting enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fawsmr_SPEC;
impl crate::sealed::RegSpec for Fawsmr_SPEC {
    type DataType = u16;
}

#[doc = "Flash Access Window Start Address Monitor Register"]
pub type Fawsmr = crate::RegValueT<Fawsmr_SPEC>;

impl Fawsmr {
    #[doc = "Access Window Start Address"]
    #[inline(always)]
    pub fn faws(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Fawsmr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Fawsmr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Access Window Protection Flag"]
    #[inline(always)]
    pub fn fspr(self) -> crate::common::RegisterFieldBool<15, 1, 0, Fawsmr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fawsmr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fawsmr {
    #[inline(always)]
    fn default() -> Fawsmr {
        <crate::RegValueT<Fawsmr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fawemr_SPEC;
impl crate::sealed::RegSpec for Fawemr_SPEC {
    type DataType = u16;
}

#[doc = "Flash Access Window End Address Monitor Register"]
pub type Fawemr = crate::RegValueT<Fawemr_SPEC>;

impl Fawemr {
    #[doc = "Access Window End Address"]
    #[inline(always)]
    pub fn fawe(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Fawemr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Fawemr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Startup Area Setting Monitor Flag"]
    #[inline(always)]
    pub fn sasmf(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fawemr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fawemr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fawemr {
    #[inline(always)]
    fn default() -> Fawemr {
        <crate::RegValueT<Fawemr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fisr_SPEC;
impl crate::sealed::RegSpec for Fisr_SPEC {
    type DataType = u8;
}

#[doc = "Flash Initial Setting Register"]
pub type Fisr = crate::RegValueT<Fisr_SPEC>;

impl Fisr {
    #[doc = "Flash-IF Clock Notification"]
    #[inline(always)]
    pub fn pcka(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fisr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fisr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Startup Area Select"]
    #[inline(always)]
    pub fn sas(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        fisr::Sas,
        fisr::Sas,
        Fisr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            fisr::Sas,
            fisr::Sas,
            Fisr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fisr {
    #[inline(always)]
    fn default() -> Fisr {
        <crate::RegValueT<Fisr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sas_SPEC;
    pub type Sas = crate::EnumBitfieldStruct<u8, Sas_SPEC>;
    impl Sas {
        #[doc = "The startup area is switched to the default area temporarily"]
        pub const _10: Self = Self::new(2);

        #[doc = "The startup area is switched to the alternate area temporarily."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fexcr_SPEC;
impl crate::sealed::RegSpec for Fexcr_SPEC {
    type DataType = u8;
}

#[doc = "Flash Extra Area Control Register"]
pub type Fexcr = crate::RegValueT<Fexcr_SPEC>;

impl Fexcr {
    #[doc = "Software Command Setting"]
    #[inline(always)]
    pub fn cmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        fexcr::Cmd,
        fexcr::Cmd,
        Fexcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            fexcr::Cmd,
            fexcr::Cmd,
            Fexcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Processing Start"]
    #[inline(always)]
    pub fn opst(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fexcr::Opst,
        fexcr::Opst,
        Fexcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fexcr::Opst,
            fexcr::Opst,
            Fexcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fexcr {
    #[inline(always)]
    fn default() -> Fexcr {
        <crate::RegValueT<Fexcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fexcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmd_SPEC;
    pub type Cmd = crate::EnumBitfieldStruct<u8, Cmd_SPEC>;
    impl Cmd {
        #[doc = "Access window information program Startup area selection and security setting"]
        pub const _010: Self = Self::new(2);

        #[doc = "OCDID1 program"]
        pub const _011: Self = Self::new(3);

        #[doc = "OCDID2 program"]
        pub const _100: Self = Self::new(4);

        #[doc = "OCDID3 program"]
        pub const _101: Self = Self::new(5);

        #[doc = "OCDID4 program"]
        pub const _110: Self = Self::new(6);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opst_SPEC;
    pub type Opst = crate::EnumBitfieldStruct<u8, Opst_SPEC>;
    impl Opst {
        #[doc = "Processing stops"]
        pub const _0: Self = Self::new(0);

        #[doc = "Processing starts."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Feaml_SPEC;
impl crate::sealed::RegSpec for Feaml_SPEC {
    type DataType = u16;
}

#[doc = "Flash Error Address Monitor Register L"]
pub type Feaml = crate::RegValueT<Feaml_SPEC>;

impl Feaml {
    #[doc = "Flash Error Address Monitor Register L"]
    #[inline(always)]
    pub fn feaml(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Feaml_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Feaml_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Feaml {
    #[inline(always)]
    fn default() -> Feaml {
        <crate::RegValueT<Feaml_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Feamh_SPEC;
impl crate::sealed::RegSpec for Feamh_SPEC {
    type DataType = u16;
}

#[doc = "Flash Error Address Monitor Register H"]
pub type Feamh = crate::RegValueT<Feamh_SPEC>;

impl Feamh {
    #[doc = "Flash Error Address Monitor Register H"]
    #[inline(always)]
    pub fn feamh(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Feamh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Feamh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Feamh {
    #[inline(always)]
    fn default() -> Feamh {
        <crate::RegValueT<Feamh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fstatr2_SPEC;
impl crate::sealed::RegSpec for Fstatr2_SPEC {
    type DataType = u16;
}

#[doc = "Flash Status Register 2"]
pub type Fstatr2 = crate::RegValueT<Fstatr2_SPEC>;

impl Fstatr2 {
    #[doc = "Erase Error Flag"]
    #[inline(always)]
    pub fn ererr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fstatr2::Ererr,
        fstatr2::Ererr,
        Fstatr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fstatr2::Ererr,
            fstatr2::Ererr,
            Fstatr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Program Error Flag"]
    #[inline(always)]
    pub fn prgerr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fstatr2::Prgerr,
        fstatr2::Prgerr,
        Fstatr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fstatr2::Prgerr,
            fstatr2::Prgerr,
            Fstatr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Program Error Flag 01"]
    #[inline(always)]
    pub fn prgerr01(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fstatr2::Prgerr01,
        fstatr2::Prgerr01,
        Fstatr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fstatr2::Prgerr01,
            fstatr2::Prgerr01,
            Fstatr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Blank Check Error Flag"]
    #[inline(always)]
    pub fn bcerr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fstatr2::Bcerr,
        fstatr2::Bcerr,
        Fstatr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fstatr2::Bcerr,
            fstatr2::Bcerr,
            Fstatr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Command Error Flag"]
    #[inline(always)]
    pub fn ilglerr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        fstatr2::Ilglerr,
        fstatr2::Ilglerr,
        Fstatr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            fstatr2::Ilglerr,
            fstatr2::Ilglerr,
            Fstatr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Extra Area Illegal Command Error Flag"]
    #[inline(always)]
    pub fn eilglerr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        fstatr2::Eilglerr,
        fstatr2::Eilglerr,
        Fstatr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            fstatr2::Eilglerr,
            fstatr2::Eilglerr,
            Fstatr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fstatr2 {
    #[inline(always)]
    fn default() -> Fstatr2 {
        <crate::RegValueT<Fstatr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fstatr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ererr_SPEC;
    pub type Ererr = crate::EnumBitfieldStruct<u8, Ererr_SPEC>;
    impl Ererr {
        #[doc = "Erasure terminates normally"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error occurs during erasure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prgerr_SPEC;
    pub type Prgerr = crate::EnumBitfieldStruct<u8, Prgerr_SPEC>;
    impl Prgerr {
        #[doc = "Programming terminates normally"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error occurs during programming."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prgerr01_SPEC;
    pub type Prgerr01 = crate::EnumBitfieldStruct<u8, Prgerr01_SPEC>;
    impl Prgerr01 {
        #[doc = "Programming by the FEXCR register terminates normally"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error occurs during programming."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcerr_SPEC;
    pub type Bcerr = crate::EnumBitfieldStruct<u8, Bcerr_SPEC>;
    impl Bcerr {
        #[doc = "Blank checking terminates normally"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error occurs during blank checking."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilglerr_SPEC;
    pub type Ilglerr = crate::EnumBitfieldStruct<u8, Ilglerr_SPEC>;
    impl Ilglerr {
        #[doc = "No illegal software command or illegal access is detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "An illegal command or illegal access is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eilglerr_SPEC;
    pub type Eilglerr = crate::EnumBitfieldStruct<u8, Eilglerr_SPEC>;
    impl Eilglerr {
        #[doc = "No illegal command or illegal access to the extra area is detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "An illegal command or illegal access to the extra area is detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscdr_SPEC;
impl crate::sealed::RegSpec for Tscdr_SPEC {
    type DataType = u16;
}

#[doc = "Temperature Sensor Calibration Data Register"]
pub type Tscdr = crate::RegValueT<Tscdr_SPEC>;

impl Tscdr {
    #[doc = "Temperature Sensor Calibration Data"]
    #[inline(always)]
    pub fn tscdr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Tscdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Tscdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tscdr {
    #[inline(always)]
    fn default() -> Tscdr {
        <crate::RegValueT<Tscdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fentryr_SPEC;
impl crate::sealed::RegSpec for Fentryr_SPEC {
    type DataType = u16;
}

#[doc = "Flash P/E Mode Entry Register"]
pub type Fentryr = crate::RegValueT<Fentryr_SPEC>;

impl Fentryr {
    #[doc = "Code Flash P/E Mode Entry 0"]
    #[inline(always)]
    pub fn fentry0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fentryr::Fentry0,
        fentryr::Fentry0,
        Fentryr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fentryr::Fentry0,
            fentryr::Fentry0,
            Fentryr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Flash P/E Mode Entry"]
    #[inline(always)]
    pub fn fentryd(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fentryr::Fentryd,
        fentryr::Fentryd,
        Fentryr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fentryr::Fentryd,
            fentryr::Fentryd,
            Fentryr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn fekey(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fentryr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fentryr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fentryr {
    #[inline(always)]
    fn default() -> Fentryr {
        <crate::RegValueT<Fentryr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fentryr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fentry0_SPEC;
    pub type Fentry0 = crate::EnumBitfieldStruct<u8, Fentry0_SPEC>;
    impl Fentry0 {
        #[doc = "The code flash is the read mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "The code flash is the P/E mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fentryd_SPEC;
    pub type Fentryd = crate::EnumBitfieldStruct<u8, Fentryd_SPEC>;
    impl Fentryd {
        #[doc = "The data flash is the read mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "The data flash is the P/E mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fldwaitr_SPEC;
impl crate::sealed::RegSpec for Fldwaitr_SPEC {
    type DataType = u8;
}

#[doc = "Memory Wait Cycle Control Register for Data Flash"]
pub type Fldwaitr = crate::RegValueT<Fldwaitr_SPEC>;

impl Fldwaitr {
    #[doc = "Memory Wait Cycle Select for Data Flash"]
    #[inline(always)]
    pub fn fldwait1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fldwaitr::Fldwait1,
        fldwaitr::Fldwait1,
        Fldwaitr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fldwaitr::Fldwait1,
            fldwaitr::Fldwait1,
            Fldwaitr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fldwaitr {
    #[inline(always)]
    fn default() -> Fldwaitr {
        <crate::RegValueT<Fldwaitr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fldwaitr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fldwait1_SPEC;
    pub type Fldwait1 = crate::EnumBitfieldStruct<u8, Fldwait1_SPEC>;
    impl Fldwait1 {
        #[doc = "1 wait access (Default)"]
        pub const _0: Self = Self::new(0);

        #[doc = "2 wait access"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfber_SPEC;
impl crate::sealed::RegSpec for Pfber_SPEC {
    type DataType = u8;
}

#[doc = "Prefetch Buffer Enable Register"]
pub type Pfber = crate::RegValueT<Pfber_SPEC>;

impl Pfber {
    #[doc = "Prefetch Buffer Enable bit"]
    #[inline(always)]
    pub fn pfbe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pfber::Pfbe,
        pfber::Pfbe,
        Pfber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pfber::Pfbe,
            pfber::Pfbe,
            Pfber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pfber {
    #[inline(always)]
    fn default() -> Pfber {
        <crate::RegValueT<Pfber_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pfber {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfbe_SPEC;
    pub type Pfbe = crate::EnumBitfieldStruct<u8, Pfbe_SPEC>;
    impl Pfbe {
        #[doc = "Prefetch buffer is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Prefetch buffer is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
