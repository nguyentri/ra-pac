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
// Generated from SVD 1.40.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:52:39 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Inter-Integrated Circuit 0"]
unsafe impl ::core::marker::Send for super::IicB0 {}
unsafe impl ::core::marker::Sync for super::IicB0 {}
impl super::IicB0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Bus Control Register"]
    #[inline(always)]
    pub const fn bctl(&self) -> &'static crate::common::Reg<self::Bctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Reset Control Register"]
    #[inline(always)]
    pub const fn rstctl(
        &self,
    ) -> &'static crate::common::Reg<self::Rstctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Present State Register"]
    #[inline(always)]
    pub const fn prsst(&self) -> &'static crate::common::Reg<self::Prsst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Prsst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Bus Function Control Register"]
    #[inline(always)]
    pub const fn bfctl(&self) -> &'static crate::common::Reg<self::Bfctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bfctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Slave Control Register"]
    #[inline(always)]
    pub const fn svctl(&self) -> &'static crate::common::Reg<self::Svctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Svctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "Reference Clock Control Register"]
    #[inline(always)]
    pub const fn refckctl(
        &self,
    ) -> &'static crate::common::Reg<self::Refckctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Refckctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "Standard Bit Rate Register"]
    #[inline(always)]
    pub const fn stdbr(&self) -> &'static crate::common::Reg<self::Stdbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stdbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "Extended Bit Rate Register"]
    #[inline(always)]
    pub const fn extbr(&self) -> &'static crate::common::Reg<self::Extbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Extbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "Bus Free Condition Detection Time Register"]
    #[inline(always)]
    pub const fn bfrecdt(
        &self,
    ) -> &'static crate::common::Reg<self::Bfrecdt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bfrecdt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = "Output Control Register"]
    #[inline(always)]
    pub const fn outctl(
        &self,
    ) -> &'static crate::common::Reg<self::Outctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Outctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "Input Control Register"]
    #[inline(always)]
    pub const fn inctl(&self) -> &'static crate::common::Reg<self::Inctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Inctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "Timeout Control Register"]
    #[inline(always)]
    pub const fn tmoctl(
        &self,
    ) -> &'static crate::common::Reg<self::Tmoctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmoctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Acknowledge Control Register"]
    #[inline(always)]
    pub const fn ackctl(
        &self,
    ) -> &'static crate::common::Reg<self::Ackctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ackctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "SCL Stretch Control Register"]
    #[inline(always)]
    pub const fn scstrctl(
        &self,
    ) -> &'static crate::common::Reg<self::Scstrctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scstrctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "Condition Control Register"]
    #[inline(always)]
    pub const fn cndctl(
        &self,
    ) -> &'static crate::common::Reg<self::Cndctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cndctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[doc = "Normal Transfer Data Buffer Port Register 0"]
    #[inline(always)]
    pub const fn ntdtbp0(
        &self,
    ) -> &'static crate::common::Reg<self::Ntdtbp0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntdtbp0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(344usize),
            )
        }
    }

    #[doc = "Normal Transfer Data Buffer Port Register 0"]
    #[inline(always)]
    pub const fn ntdtbp0_by(
        &self,
    ) -> &'static crate::common::Reg<self::Ntdtbp0By_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntdtbp0By_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(344usize),
            )
        }
    }

    #[doc = "Bus Status Register"]
    #[inline(always)]
    pub const fn bst(&self) -> &'static crate::common::Reg<self::Bst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(464usize),
            )
        }
    }

    #[doc = "Bus Status Enable Register"]
    #[inline(always)]
    pub const fn bste(&self) -> &'static crate::common::Reg<self::Bste_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bste_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(468usize),
            )
        }
    }

    #[doc = "Bus Interrupt Enable Register"]
    #[inline(always)]
    pub const fn bie(&self) -> &'static crate::common::Reg<self::Bie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(472usize),
            )
        }
    }

    #[doc = "Bus Status Force Register"]
    #[inline(always)]
    pub const fn bstfc(&self) -> &'static crate::common::Reg<self::Bstfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bstfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(476usize),
            )
        }
    }

    #[doc = "Normal Transfer Status Register"]
    #[inline(always)]
    pub const fn ntst(&self) -> &'static crate::common::Reg<self::Ntst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(480usize),
            )
        }
    }

    #[doc = "Normal Transfer Status Enable Register"]
    #[inline(always)]
    pub const fn ntste(&self) -> &'static crate::common::Reg<self::Ntste_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntste_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(484usize),
            )
        }
    }

    #[doc = "Normal Transfer Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ntie(&self) -> &'static crate::common::Reg<self::Ntie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(488usize),
            )
        }
    }

    #[doc = "Normal Transfer Status Force Register"]
    #[inline(always)]
    pub const fn ntstfc(&self) -> &'static crate::common::Reg<self::Ntstfc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ntstfc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(492usize),
            )
        }
    }

    #[doc = "Bus Condition Status Register"]
    #[inline(always)]
    pub const fn bcst(&self) -> &'static crate::common::Reg<self::Bcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(528usize),
            )
        }
    }

    #[doc = "Slave Status Register"]
    #[inline(always)]
    pub const fn svst(&self) -> &'static crate::common::Reg<self::Svst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Svst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(532usize),
            )
        }
    }

    #[doc = "Slave Device Address Table Basic Register %s"]
    #[inline(always)]
    pub const fn sdatbas(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sdatbas_SPEC, crate::common::RW>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2b0usize))
        }
    }
    #[inline(always)]
    pub const fn sdatbas0(
        &self,
    ) -> &'static crate::common::Reg<self::Sdatbas_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdatbas_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdatbas1(
        &self,
    ) -> &'static crate::common::Reg<self::Sdatbas_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdatbas_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdatbas2(
        &self,
    ) -> &'static crate::common::Reg<self::Sdatbas_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdatbas_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b8usize),
            )
        }
    }

    #[doc = "Slave Device Address Register %s"]
    #[inline(always)]
    pub const fn svdvad(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Svdvad_SPEC, crate::common::R>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x330usize))
        }
    }
    #[inline(always)]
    pub const fn svdvad0(
        &self,
    ) -> &'static crate::common::Reg<self::Svdvad_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Svdvad_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x330usize),
            )
        }
    }
    #[inline(always)]
    pub const fn svdvad1(
        &self,
    ) -> &'static crate::common::Reg<self::Svdvad_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Svdvad_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x334usize),
            )
        }
    }
    #[inline(always)]
    pub const fn svdvad2(
        &self,
    ) -> &'static crate::common::Reg<self::Svdvad_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Svdvad_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x338usize),
            )
        }
    }

    #[doc = "Bit Count Register"]
    #[inline(always)]
    pub const fn bitcnt(&self) -> &'static crate::common::Reg<self::Bitcnt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bitcnt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(896usize),
            )
        }
    }

    #[doc = "Present State Debug Register"]
    #[inline(always)]
    pub const fn prstdbg(
        &self,
    ) -> &'static crate::common::Reg<self::Prstdbg_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Prstdbg_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(972usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bctl_SPEC;
impl crate::sealed::RegSpec for Bctl_SPEC {
    type DataType = u32;
}

#[doc = "Bus Control Register"]
pub type Bctl = crate::RegValueT<Bctl_SPEC>;

impl Bctl {
    #[doc = "Bus Enable"]
    #[inline(always)]
    pub fn buse(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        bctl::Buse,
        bctl::Buse,
        Bctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            bctl::Buse,
            bctl::Buse,
            Bctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bctl {
    #[inline(always)]
    fn default() -> Bctl {
        <crate::RegValueT<Bctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Buse_SPEC;
    pub type Buse = crate::EnumBitfieldStruct<u8, Buse_SPEC>;
    impl Buse {
        #[doc = "IIC bus operation is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "IIC bus operation is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstctl_SPEC;
impl crate::sealed::RegSpec for Rstctl_SPEC {
    type DataType = u32;
}

#[doc = "Reset Control Register"]
pub type Rstctl = crate::RegValueT<Rstctl_SPEC>;

impl Rstctl {
    #[doc = "IIC Software Reset"]
    #[inline(always)]
    pub fn ri2crst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rstctl::Ri2Crst,
        rstctl::Ri2Crst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rstctl::Ri2Crst,
            rstctl::Ri2Crst,
            Rstctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Internal Software Reset"]
    #[inline(always)]
    pub fn intlrst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        rstctl::Intlrst,
        rstctl::Intlrst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            rstctl::Intlrst,
            rstctl::Intlrst,
            Rstctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rstctl {
    #[inline(always)]
    fn default() -> Rstctl {
        <crate::RegValueT<Rstctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ri2Crst_SPEC;
    pub type Ri2Crst = crate::EnumBitfieldStruct<u8, Ri2Crst_SPEC>;
    impl Ri2Crst {
        #[doc = "Reset of all registers and internal state."]
        pub const _0: Self = Self::new(0);

        #[doc = "Releases of all registers and internal state."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intlrst_SPEC;
    pub type Intlrst = crate::EnumBitfieldStruct<u8, Intlrst_SPEC>;
    impl Intlrst {
        #[doc = "Releases of some registers and internal state."]
        pub const _0: Self = Self::new(0);

        #[doc = "Resets of some registers and internal state."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prsst_SPEC;
impl crate::sealed::RegSpec for Prsst_SPEC {
    type DataType = u32;
}

#[doc = "Present State Register"]
pub type Prsst = crate::RegValueT<Prsst_SPEC>;

impl Prsst {
    #[doc = "Current Master"]
    #[inline(always)]
    pub fn crms(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        prsst::Crms,
        prsst::Crms,
        Prsst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            prsst::Crms,
            prsst::Crms,
            Prsst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit/Receive Mode"]
    #[inline(always)]
    pub fn trmd(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        prsst::Trmd,
        prsst::Trmd,
        Prsst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            prsst::Trmd,
            prsst::Trmd,
            Prsst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Present State Write Protect"]
    #[inline(always)]
    pub fn prsstwp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        prsst::Prsstwp,
        prsst::Prsstwp,
        Prsst_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            prsst::Prsstwp,
            prsst::Prsstwp,
            Prsst_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Prsst {
    #[inline(always)]
    fn default() -> Prsst {
        <crate::RegValueT<Prsst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod prsst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crms_SPEC;
    pub type Crms = crate::EnumBitfieldStruct<u8, Crms_SPEC>;
    impl Crms {
        #[doc = "The Master is not the Current Master, and must request and acquire bus ownership before initiating any transfer."]
        pub const _0: Self = Self::new(0);

        #[doc = "The Master is the Current Master, and as a result can initiate transfers."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trmd_SPEC;
    pub type Trmd = crate::EnumBitfieldStruct<u8, Trmd_SPEC>;
    impl Trmd {
        #[doc = "Receive mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prsstwp_SPEC;
    pub type Prsstwp = crate::EnumBitfieldStruct<u8, Prsstwp_SPEC>;
    impl Prsstwp {
        #[doc = "CRMS bit is protected."]
        pub const _0: Self = Self::new(0);

        #[doc = "CRMS bit can be written when writing simultaneously with the value of the target bit."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfctl_SPEC;
impl crate::sealed::RegSpec for Bfctl_SPEC {
    type DataType = u32;
}

#[doc = "Bus Function Control Register"]
pub type Bfctl = crate::RegValueT<Bfctl_SPEC>;

impl Bfctl {
    #[doc = "Master Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn male(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bfctl::Male,
        bfctl::Male,
        Bfctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bfctl::Male,
            bfctl::Male,
            Bfctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NACK Transmission Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn nale(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bfctl::Nale,
        bfctl::Nale,
        Bfctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bfctl::Nale,
            bfctl::Nale,
            Bfctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Slave Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn sale(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        bfctl::Sale,
        bfctl::Sale,
        Bfctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            bfctl::Sale,
            bfctl::Sale,
            Bfctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCL Synchronous Circuit Enable"]
    #[inline(always)]
    pub fn scsyne(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bfctl::Scsyne,
        bfctl::Scsyne,
        Bfctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bfctl::Scsyne,
            bfctl::Scsyne,
            Bfctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SMBus/I2C Bus Selection"]
    #[inline(always)]
    pub fn smbs(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        bfctl::Smbs,
        bfctl::Smbs,
        Bfctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            bfctl::Smbs,
            bfctl::Smbs,
            Bfctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Fast-mode Plus Enable"]
    #[inline(always)]
    pub fn fmpe(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        bfctl::Fmpe,
        bfctl::Fmpe,
        Bfctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            bfctl::Fmpe,
            bfctl::Fmpe,
            Bfctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "High Speed Mode Enable"]
    #[inline(always)]
    pub fn hsme(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        bfctl::Hsme,
        bfctl::Hsme,
        Bfctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            bfctl::Hsme,
            bfctl::Hsme,
            Bfctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bfctl {
    #[inline(always)]
    fn default() -> Bfctl {
        <crate::RegValueT<Bfctl_SPEC> as RegisterValue<_>>::new(257)
    }
}
pub mod bfctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Male_SPEC;
    pub type Male = crate::EnumBitfieldStruct<u8, Male_SPEC>;
    impl Male {
        #[doc = "Master arbitration-lost detection disables. Disables the arbitration-lost detection function and does not clear the CRMS and TRMD bits in PRSST automatically when arbitration is lost."]
        pub const _0: Self = Self::new(0);

        #[doc = "Master arbitration-lost detection enables. Enables the arbitration-lost detection function and clears the CRMS and TRMD bits in PRSST automatically when arbitration is lost."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nale_SPEC;
    pub type Nale = crate::EnumBitfieldStruct<u8, Nale_SPEC>;
    impl Nale {
        #[doc = "NACK transmission arbitration-lost detection disables."]
        pub const _0: Self = Self::new(0);

        #[doc = "NACK transmission arbitration-lost detection enables."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sale_SPEC;
    pub type Sale = crate::EnumBitfieldStruct<u8, Sale_SPEC>;
    impl Sale {
        #[doc = "Slave arbitration-lost detection disables."]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave arbitration-lost detection enables."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scsyne_SPEC;
    pub type Scsyne = crate::EnumBitfieldStruct<u8, Scsyne_SPEC>;
    impl Scsyne {
        #[doc = "No SCL synchronous circuit uses."]
        pub const _0: Self = Self::new(0);

        #[doc = "An SCL synchronous circuit uses."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smbs_SPEC;
    pub type Smbs = crate::EnumBitfieldStruct<u8, Smbs_SPEC>;
    impl Smbs {
        #[doc = "The I2C bus select."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SMBus select."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fmpe_SPEC;
    pub type Fmpe = crate::EnumBitfieldStruct<u8, Fmpe_SPEC>;
    impl Fmpe {
        #[doc = "No Fm+ slope control circuit uses for the SCLn pin and SDAn pin. (n = 0, 1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "An Fm+ slope control circuit uses for the SCLn pin and SDAn pin. (n = 0, 1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsme_SPEC;
    pub type Hsme = crate::EnumBitfieldStruct<u8, Hsme_SPEC>;
    impl Hsme {
        #[doc = "Disable High Speed Mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable High Speed Mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svctl_SPEC;
impl crate::sealed::RegSpec for Svctl_SPEC {
    type DataType = u32;
}

#[doc = "Slave Control Register"]
pub type Svctl = crate::RegValueT<Svctl_SPEC>;

impl Svctl {
    #[doc = "General Call Address Enable"]
    #[inline(always)]
    pub fn gcae(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        svctl::Gcae,
        svctl::Gcae,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            svctl::Gcae,
            svctl::Gcae,
            Svctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Hs-mode Master Code Enable"]
    #[inline(always)]
    pub fn hsmce(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        svctl::Hsmce,
        svctl::Hsmce,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            svctl::Hsmce,
            svctl::Hsmce,
            Svctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device-ID Address Enable"]
    #[inline(always)]
    pub fn dvide(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        svctl::Dvide,
        svctl::Dvide,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            svctl::Dvide,
            svctl::Dvide,
            Svctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Host Address Enable"]
    #[inline(always)]
    pub fn hoae(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        svctl::Hoae,
        svctl::Hoae,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            svctl::Hoae,
            svctl::Hoae,
            Svctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Slave Address Enable 0"]
    #[inline(always)]
    pub fn svae0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        svctl::Svae0,
        svctl::Svae0,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            svctl::Svae0,
            svctl::Svae0,
            Svctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Slave Address Enable 1"]
    #[inline(always)]
    pub fn svae1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        svctl::Svae1,
        svctl::Svae1,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            svctl::Svae1,
            svctl::Svae1,
            Svctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Slave Address Enable 2"]
    #[inline(always)]
    pub fn svae2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        svctl::Svae2,
        svctl::Svae2,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            svctl::Svae2,
            svctl::Svae2,
            Svctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Svctl {
    #[inline(always)]
    fn default() -> Svctl {
        <crate::RegValueT<Svctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod svctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gcae_SPEC;
    pub type Gcae = crate::EnumBitfieldStruct<u8, Gcae_SPEC>;
    impl Gcae {
        #[doc = "General call address detection disables."]
        pub const _0: Self = Self::new(0);

        #[doc = "General call address detection enables."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsmce_SPEC;
    pub type Hsmce = crate::EnumBitfieldStruct<u8, Hsmce_SPEC>;
    impl Hsmce {
        #[doc = "Hs-mode Master Code Detection disables."]
        pub const _0: Self = Self::new(0);

        #[doc = "Hs-mode Master Code Detection enables."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvide_SPEC;
    pub type Dvide = crate::EnumBitfieldStruct<u8, Dvide_SPEC>;
    impl Dvide {
        #[doc = "Device-ID address detection disables."]
        pub const _0: Self = Self::new(0);

        #[doc = "Device-ID address detection enables."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hoae_SPEC;
    pub type Hoae = crate::EnumBitfieldStruct<u8, Hoae_SPEC>;
    impl Hoae {
        #[doc = "Host address detection disables."]
        pub const _0: Self = Self::new(0);

        #[doc = "Host address detection enables."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svae0_SPEC;
    pub type Svae0 = crate::EnumBitfieldStruct<u8, Svae0_SPEC>;
    impl Svae0 {
        #[doc = "Slave 0 disables"]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave 0 enables"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svae1_SPEC;
    pub type Svae1 = crate::EnumBitfieldStruct<u8, Svae1_SPEC>;
    impl Svae1 {
        #[doc = "Slave 1 disables"]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave 1 enables"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svae2_SPEC;
    pub type Svae2 = crate::EnumBitfieldStruct<u8, Svae2_SPEC>;
    impl Svae2 {
        #[doc = "Slave 2 disables"]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave 2 enables"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Refckctl_SPEC;
impl crate::sealed::RegSpec for Refckctl_SPEC {
    type DataType = u32;
}

#[doc = "Reference Clock Control Register"]
pub type Refckctl = crate::RegValueT<Refckctl_SPEC>;

impl Refckctl {
    #[doc = "Internal Reference Clock Selection"]
    #[inline(always)]
    pub fn irefcks(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        refckctl::Irefcks,
        refckctl::Irefcks,
        Refckctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            refckctl::Irefcks,
            refckctl::Irefcks,
            Refckctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Refckctl {
    #[inline(always)]
    fn default() -> Refckctl {
        <crate::RegValueT<Refckctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod refckctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irefcks_SPEC;
    pub type Irefcks = crate::EnumBitfieldStruct<u8, Irefcks_SPEC>;
    impl Irefcks {
        #[doc = "IICCLK/1 clock"]
        pub const _000: Self = Self::new(0);

        #[doc = "IICCLK/2 clock"]
        pub const _001: Self = Self::new(1);

        #[doc = "IICCLK/4 clock"]
        pub const _010: Self = Self::new(2);

        #[doc = "IICCLK/8 clock"]
        pub const _011: Self = Self::new(3);

        #[doc = "IICCLK/16 clock"]
        pub const _100: Self = Self::new(4);

        #[doc = "IICCLK/32 clock"]
        pub const _101: Self = Self::new(5);

        #[doc = "IICCLK/64 clock"]
        pub const _110: Self = Self::new(6);

        #[doc = "IICCLK/128 clock"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stdbr_SPEC;
impl crate::sealed::RegSpec for Stdbr_SPEC {
    type DataType = u32;
}

#[doc = "Standard Bit Rate Register"]
pub type Stdbr = crate::RegValueT<Stdbr_SPEC>;

impl Stdbr {
    #[doc = "Count value of the Low-level period of SCL clock"]
    #[inline(always)]
    pub fn sbrlo(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Stdbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Stdbr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Count value of the High-level period of SCL clock"]
    #[inline(always)]
    pub fn sbrho(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Stdbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Stdbr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Double the Standard Bit Rate Period for Open-Drain"]
    #[inline(always)]
    pub fn dsbrpo(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        stdbr::Dsbrpo,
        stdbr::Dsbrpo,
        Stdbr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            stdbr::Dsbrpo,
            stdbr::Dsbrpo,
            Stdbr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Stdbr {
    #[inline(always)]
    fn default() -> Stdbr {
        <crate::RegValueT<Stdbr_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod stdbr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsbrpo_SPEC;
    pub type Dsbrpo = crate::EnumBitfieldStruct<u8, Dsbrpo_SPEC>;
    impl Dsbrpo {
        #[doc = "The time period set for SBRHO\\[7:0\\] and SBRLO\\[7:0\\] is not doubled."]
        pub const _0: Self = Self::new(0);

        #[doc = "The time period set for SBRHO\\[7:0\\] and SBRLO\\[7:0\\] is doubled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extbr_SPEC;
impl crate::sealed::RegSpec for Extbr_SPEC {
    type DataType = u32;
}

#[doc = "Extended Bit Rate Register"]
pub type Extbr = crate::RegValueT<Extbr_SPEC>;

impl Extbr {
    #[doc = "Extended Bit Rate Low-Level Period Open-Drain"]
    #[inline(always)]
    pub fn ebrlo(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Extbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Extbr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Extended Bit Rate High-Level Period Open-Drain"]
    #[inline(always)]
    pub fn ebrho(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Extbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Extbr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Extbr {
    #[inline(always)]
    fn default() -> Extbr {
        <crate::RegValueT<Extbr_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfrecdt_SPEC;
impl crate::sealed::RegSpec for Bfrecdt_SPEC {
    type DataType = u32;
}

#[doc = "Bus Free Condition Detection Time Register"]
pub type Bfrecdt = crate::RegValueT<Bfrecdt_SPEC>;

impl Bfrecdt {
    #[doc = "Bus Free Condition Detection Cycle"]
    #[inline(always)]
    pub fn frecyc(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Bfrecdt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Bfrecdt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bfrecdt {
    #[inline(always)]
    fn default() -> Bfrecdt {
        <crate::RegValueT<Bfrecdt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outctl_SPEC;
impl crate::sealed::RegSpec for Outctl_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Register"]
pub type Outctl = crate::RegValueT<Outctl_SPEC>;

impl Outctl {
    #[doc = "SDA Output Control"]
    #[inline(always)]
    pub fn sdoc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        outctl::Sdoc,
        outctl::Sdoc,
        Outctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            outctl::Sdoc,
            outctl::Sdoc,
            Outctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCL Output Control"]
    #[inline(always)]
    pub fn scoc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        outctl::Scoc,
        outctl::Scoc,
        Outctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            outctl::Scoc,
            outctl::Scoc,
            Outctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCL/SDA Output Control Write Protect"]
    #[inline(always)]
    pub fn socwp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        outctl::Socwp,
        outctl::Socwp,
        Outctl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            outctl::Socwp,
            outctl::Socwp,
            Outctl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Extra SCL Clock Cycle Output"]
    #[inline(always)]
    pub fn excyc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        outctl::Excyc,
        outctl::Excyc,
        Outctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            outctl::Excyc,
            outctl::Excyc,
            Outctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDA Output Delay"]
    #[inline(always)]
    pub fn sdod(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        outctl::Sdod,
        outctl::Sdod,
        Outctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            outctl::Sdod,
            outctl::Sdod,
            Outctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDA Output Delay Clock Source Selection"]
    #[inline(always)]
    pub fn sdodcs(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        outctl::Sdodcs,
        outctl::Sdodcs,
        Outctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            outctl::Sdodcs,
            outctl::Sdodcs,
            Outctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Outctl {
    #[inline(always)]
    fn default() -> Outctl {
        <crate::RegValueT<Outctl_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod outctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdoc_SPEC;
    pub type Sdoc = crate::EnumBitfieldStruct<u8, Sdoc_SPEC>;
    impl Sdoc {
        #[doc = "IIC drives the SDAn pin low."]
        pub const _0: Self = Self::new(0);

        #[doc = "IIC releases the SDAn pin."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scoc_SPEC;
    pub type Scoc = crate::EnumBitfieldStruct<u8, Scoc_SPEC>;
    impl Scoc {
        #[doc = "IIC drives the SCLn pin low."]
        pub const _0: Self = Self::new(0);

        #[doc = "IIC releases the SCLn pin."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Socwp_SPEC;
    pub type Socwp = crate::EnumBitfieldStruct<u8, Socwp_SPEC>;
    impl Socwp {
        #[doc = "Bits SCOC and SDOC are protected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bits SCOC and SDOC can be written (When writing simultaneously with the value of the target bit). This bit is read as 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Excyc_SPEC;
    pub type Excyc = crate::EnumBitfieldStruct<u8, Excyc_SPEC>;
    impl Excyc {
        #[doc = "Does not output an extra SCL clock cycle (default)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Outputs an extra SCL clock cycle."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdod_SPEC;
    pub type Sdod = crate::EnumBitfieldStruct<u8, Sdod_SPEC>;
    impl Sdod {
        #[doc = "No output delay"]
        pub const _000: Self = Self::new(0);

        #[doc = "1 IICφ cycle (When OUTCTL.SDODCS = 0 (IICφ)) 1 or 2 IICφ cycles (When OUTCTL.SDODCS = 1 (IICφ/2))"]
        pub const _001: Self = Self::new(1);

        #[doc = "2 IICφ cycles (When OUTCTL.SDODCS = 0 (IICφ)) 3 or 4 IICφ cycles (When OUTCTL.SDODCS = 1 (IICφ/2))"]
        pub const _010: Self = Self::new(2);

        #[doc = "3 IICφ cycles (When OUTCTL.SDODCS = 0 (IICφ)) 5 or 6 IICφ cycles (When OUTCTL.SDODCS = 1 (IICφ/2))"]
        pub const _011: Self = Self::new(3);

        #[doc = "4 IICφ cycles (When OUTCTL.SDODCS = 0 (IICφ)) 7 or 8 IICφ cycles (When OUTCTL.SDODCS = 1 (IICφ/2))"]
        pub const _100: Self = Self::new(4);

        #[doc = "5 IICφ cycles (When OUTCTL.SDODCS = 0 (IICφ)) 9 or 10 IICφ cycles (When OUTCTL.SDODCS = 1 (IICφ/2))"]
        pub const _101: Self = Self::new(5);

        #[doc = "6 IICφ cycles (When OUTCTL.SDODCS = 0 (IICφ)) 11 or 12 IICφ cycles (When OUTCTL.SDODCS = 1 (IICφ/2))"]
        pub const _110: Self = Self::new(6);

        #[doc = "7 IICφ cycles (When OUTCTL.SDODCS = 0 (IICφ)) 13 or 14 IICφ cycles (When OUTCTL.SDODCS = 1 (IICφ/2))"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdodcs_SPEC;
    pub type Sdodcs = crate::EnumBitfieldStruct<u8, Sdodcs_SPEC>;
    impl Sdodcs {
        #[doc = "The internal reference clock (IICφ) is selected as the clock source of the SDA output delay counter."]
        pub const _0: Self = Self::new(0);

        #[doc = "The internal reference clock divided by 2 (IICφ/2) is selected as the clock source of the SDA output delay counter."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inctl_SPEC;
impl crate::sealed::RegSpec for Inctl_SPEC {
    type DataType = u32;
}

#[doc = "Input Control Register"]
pub type Inctl = crate::RegValueT<Inctl_SPEC>;

impl Inctl {
    #[doc = "Digital Noise Filter Stage Selection"]
    #[inline(always)]
    pub fn dnfs(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Inctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Inctl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Digital Noise Filter Circuit Enable"]
    #[inline(always)]
    pub fn dnfe(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        inctl::Dnfe,
        inctl::Dnfe,
        Inctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            inctl::Dnfe,
            inctl::Dnfe,
            Inctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Inctl {
    #[inline(always)]
    fn default() -> Inctl {
        <crate::RegValueT<Inctl_SPEC> as RegisterValue<_>>::new(208)
    }
}
pub mod inctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dnfe_SPEC;
    pub type Dnfe = crate::EnumBitfieldStruct<u8, Dnfe_SPEC>;
    impl Dnfe {
        #[doc = "No digital noise filter circuit is used."]
        pub const _0: Self = Self::new(0);

        #[doc = "A digital noise filter circuit is used."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmoctl_SPEC;
impl crate::sealed::RegSpec for Tmoctl_SPEC {
    type DataType = u32;
}

#[doc = "Timeout Control Register"]
pub type Tmoctl = crate::RegValueT<Tmoctl_SPEC>;

impl Tmoctl {
    #[doc = "Timeout Detection Time Selection"]
    #[inline(always)]
    pub fn todts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        tmoctl::Todts,
        tmoctl::Todts,
        Tmoctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            tmoctl::Todts,
            tmoctl::Todts,
            Tmoctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timeout L Count Control"]
    #[inline(always)]
    pub fn tolctl(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tmoctl::Tolctl,
        tmoctl::Tolctl,
        Tmoctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tmoctl::Tolctl,
            tmoctl::Tolctl,
            Tmoctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timeout H Count Control"]
    #[inline(always)]
    pub fn tohctl(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        tmoctl::Tohctl,
        tmoctl::Tohctl,
        Tmoctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            tmoctl::Tohctl,
            tmoctl::Tohctl,
            Tmoctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timeout Operation Mode Selection"]
    #[inline(always)]
    pub fn tomds(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        tmoctl::Tomds,
        tmoctl::Tomds,
        Tmoctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            tmoctl::Tomds,
            tmoctl::Tomds,
            Tmoctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tmoctl {
    #[inline(always)]
    fn default() -> Tmoctl {
        <crate::RegValueT<Tmoctl_SPEC> as RegisterValue<_>>::new(48)
    }
}
pub mod tmoctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Todts_SPEC;
    pub type Todts = crate::EnumBitfieldStruct<u8, Todts_SPEC>;
    impl Todts {
        #[doc = "16bit-timeout"]
        pub const _00: Self = Self::new(0);

        #[doc = "14bit-timeout"]
        pub const _01: Self = Self::new(1);

        #[doc = "8bit-timeout"]
        pub const _10: Self = Self::new(2);

        #[doc = "6bit-timeout"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tolctl_SPEC;
    pub type Tolctl = crate::EnumBitfieldStruct<u8, Tolctl_SPEC>;
    impl Tolctl {
        #[doc = "Count is disabled while the SCLn line is at a low level."]
        pub const _0: Self = Self::new(0);

        #[doc = "Count is enabled while the SCLn line is at a low level."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tohctl_SPEC;
    pub type Tohctl = crate::EnumBitfieldStruct<u8, Tohctl_SPEC>;
    impl Tohctl {
        #[doc = "Count is disabled while the SCLn line is at a high level."]
        pub const _0: Self = Self::new(0);

        #[doc = "Count is enabled while the SCLn line is at a high level."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tomds_SPEC;
    pub type Tomds = crate::EnumBitfieldStruct<u8, Tomds_SPEC>;
    impl Tomds {
        #[doc = "Timeout is detected during the following conditions: The bus is busy (BCST.BFREF = 0) in master mode.IIC’s own slave address is detected and the bus is busy in slave mode.The bus is free (BCST.BFREF = 1) while generation of a START condition is requested (CNDCTL.STCND = 1)."]
        pub const _00: Self = Self::new(0);

        #[doc = "Timeout is detected while the bus is busy."]
        pub const _01: Self = Self::new(1);

        #[doc = "Timeout is detected while the bus is free."]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ackctl_SPEC;
impl crate::sealed::RegSpec for Ackctl_SPEC {
    type DataType = u32;
}

#[doc = "Acknowledge Control Register"]
pub type Ackctl = crate::RegValueT<Ackctl_SPEC>;

impl Ackctl {
    #[doc = "Acknowledge Reception"]
    #[inline(always)]
    pub fn ackr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ackctl::Ackr,
        ackctl::Ackr,
        Ackctl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ackctl::Ackr,
            ackctl::Ackr,
            Ackctl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Acknowledge Transmission"]
    #[inline(always)]
    pub fn ackt(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ackctl::Ackt,
        ackctl::Ackt,
        Ackctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ackctl::Ackt,
            ackctl::Ackt,
            Ackctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACKT Write Protect"]
    #[inline(always)]
    pub fn acktwp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ackctl::Acktwp,
        ackctl::Acktwp,
        Ackctl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ackctl::Acktwp,
            ackctl::Acktwp,
            Ackctl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ackctl {
    #[inline(always)]
    fn default() -> Ackctl {
        <crate::RegValueT<Ackctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ackctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackr_SPEC;
    pub type Ackr = crate::EnumBitfieldStruct<u8, Ackr_SPEC>;
    impl Ackr {
        #[doc = "A 0 is received as the acknowledge bit (ACK reception)."]
        pub const _0: Self = Self::new(0);

        #[doc = "A 1 is received as the acknowledge bit (NACK reception)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackt_SPEC;
    pub type Ackt = crate::EnumBitfieldStruct<u8, Ackt_SPEC>;
    impl Ackt {
        #[doc = "A 0 is sent as the acknowledge bit (ACK transmission)."]
        pub const _0: Self = Self::new(0);

        #[doc = "A 1 is sent as the acknowledge bit (NACK transmission)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acktwp_SPEC;
    pub type Acktwp = crate::EnumBitfieldStruct<u8, Acktwp_SPEC>;
    impl Acktwp {
        #[doc = "The ACKT bit are protected."]
        pub const _0: Self = Self::new(0);

        #[doc = "The ACKT bit can be written (when writing simultaneously with the value of the target bit). This bit is read as 0."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scstrctl_SPEC;
impl crate::sealed::RegSpec for Scstrctl_SPEC {
    type DataType = u32;
}

#[doc = "SCL Stretch Control Register"]
pub type Scstrctl = crate::RegValueT<Scstrctl_SPEC>;

impl Scstrctl {
    #[doc = "Acknowledge Transmission Wait Enable"]
    #[inline(always)]
    pub fn acktwe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        scstrctl::Acktwe,
        scstrctl::Acktwe,
        Scstrctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            scstrctl::Acktwe,
            scstrctl::Acktwe,
            Scstrctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Wait Enable"]
    #[inline(always)]
    pub fn rwe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        scstrctl::Rwe,
        scstrctl::Rwe,
        Scstrctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            scstrctl::Rwe,
            scstrctl::Rwe,
            Scstrctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scstrctl {
    #[inline(always)]
    fn default() -> Scstrctl {
        <crate::RegValueT<Scstrctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scstrctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acktwe_SPEC;
    pub type Acktwe = crate::EnumBitfieldStruct<u8, Acktwe_SPEC>;
    impl Acktwe {
        #[doc = "NTST.RDBFF0 is set at the rising edge of the ninth SCL clock cycle. (The SCLn line is not held low at the falling edge of the eighth clock cycle.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "NTST.RDBFF0 is set at the rising edge of the eighth SCL clock cycle. (The SCLn line is held low at the falling edge of the eighth clock cycle.) Low-hold is released by writing a value to the ACKCTL.ACKT bit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rwe_SPEC;
    pub type Rwe = crate::EnumBitfieldStruct<u8, Rwe_SPEC>;
    impl Rwe {
        #[doc = "No WAIT (The period between ninth clock cycle and first clock cycle is not held low.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "WAIT (The period between ninth clock cycle and first clock cycle is held low.) Low-hold is released by reading NTDTBP0."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndctl_SPEC;
impl crate::sealed::RegSpec for Cndctl_SPEC {
    type DataType = u32;
}

#[doc = "Condition Control Register"]
pub type Cndctl = crate::RegValueT<Cndctl_SPEC>;

impl Cndctl {
    #[doc = "START (S) Condition Issuance"]
    #[inline(always)]
    pub fn stcnd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cndctl::Stcnd,
        cndctl::Stcnd,
        Cndctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cndctl::Stcnd,
            cndctl::Stcnd,
            Cndctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Repeated START (Sr) Condition Issuance"]
    #[inline(always)]
    pub fn srcnd(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cndctl::Srcnd,
        cndctl::Srcnd,
        Cndctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cndctl::Srcnd,
            cndctl::Srcnd,
            Cndctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "STOP (P) Condition Issuance"]
    #[inline(always)]
    pub fn spcnd(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cndctl::Spcnd,
        cndctl::Spcnd,
        Cndctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cndctl::Spcnd,
            cndctl::Spcnd,
            Cndctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cndctl {
    #[inline(always)]
    fn default() -> Cndctl {
        <crate::RegValueT<Cndctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cndctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stcnd_SPEC;
    pub type Stcnd = crate::EnumBitfieldStruct<u8, Stcnd_SPEC>;
    impl Stcnd {
        #[doc = "Does not request to issue a START condition."]
        pub const _0: Self = Self::new(0);

        #[doc = "Requests to issue a START condition."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Srcnd_SPEC;
    pub type Srcnd = crate::EnumBitfieldStruct<u8, Srcnd_SPEC>;
    impl Srcnd {
        #[doc = "Does not request to issue a Repeated START condition."]
        pub const _0: Self = Self::new(0);

        #[doc = "Requests to issue a Repeated START condition."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcnd_SPEC;
    pub type Spcnd = crate::EnumBitfieldStruct<u8, Spcnd_SPEC>;
    impl Spcnd {
        #[doc = "Does not request to issue a STOP condition."]
        pub const _0: Self = Self::new(0);

        #[doc = "Requests to issue a STOP condition."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntdtbp0_SPEC;
impl crate::sealed::RegSpec for Ntdtbp0_SPEC {
    type DataType = u32;
}

#[doc = "Normal Transfer Data Buffer Port Register 0"]
pub type Ntdtbp0 = crate::RegValueT<Ntdtbp0_SPEC>;

impl NoBitfieldReg<Ntdtbp0_SPEC> for Ntdtbp0 {}
impl ::core::default::Default for Ntdtbp0 {
    #[inline(always)]
    fn default() -> Ntdtbp0 {
        <crate::RegValueT<Ntdtbp0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntdtbp0By_SPEC;
impl crate::sealed::RegSpec for Ntdtbp0By_SPEC {
    type DataType = u8;
}

#[doc = "Normal Transfer Data Buffer Port Register 0"]
pub type Ntdtbp0By = crate::RegValueT<Ntdtbp0By_SPEC>;

impl NoBitfieldReg<Ntdtbp0By_SPEC> for Ntdtbp0By {}
impl ::core::default::Default for Ntdtbp0By {
    #[inline(always)]
    fn default() -> Ntdtbp0By {
        <crate::RegValueT<Ntdtbp0By_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bst_SPEC;
impl crate::sealed::RegSpec for Bst_SPEC {
    type DataType = u32;
}

#[doc = "Bus Status Register"]
pub type Bst = crate::RegValueT<Bst_SPEC>;

impl Bst {
    #[doc = "START Condition Detection Flag"]
    #[inline(always)]
    pub fn stcnddf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bst::Stcnddf,
        bst::Stcnddf,
        Bst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bst::Stcnddf,
            bst::Stcnddf,
            Bst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "STOP Condition Detection Flag"]
    #[inline(always)]
    pub fn spcnddf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bst::Spcnddf,
        bst::Spcnddf,
        Bst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bst::Spcnddf,
            bst::Spcnddf,
            Bst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NACK Detection Flag"]
    #[inline(always)]
    pub fn nackdf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        bst::Nackdf,
        bst::Nackdf,
        Bst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            bst::Nackdf,
            bst::Nackdf,
            Bst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub fn tendf(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bst::Tendf,
        bst::Tendf,
        Bst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bst::Tendf,
            bst::Tendf,
            Bst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Arbitration Lost Flag"]
    #[inline(always)]
    pub fn alf(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, bst::Alf, bst::Alf, Bst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,bst::Alf,bst::Alf,Bst_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Timeout Detection Flag"]
    #[inline(always)]
    pub fn todf(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        bst::Todf,
        bst::Todf,
        Bst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            bst::Todf,
            bst::Todf,
            Bst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Wake-Up Condition Detection Flag"]
    #[inline(always)]
    pub fn wucnddf(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        bst::Wucnddf,
        bst::Wucnddf,
        Bst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            bst::Wucnddf,
            bst::Wucnddf,
            Bst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bst {
    #[inline(always)]
    fn default() -> Bst {
        <crate::RegValueT<Bst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stcnddf_SPEC;
    pub type Stcnddf = crate::EnumBitfieldStruct<u8, Stcnddf_SPEC>;
    impl Stcnddf {
        #[doc = "START condition is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "START condition is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcnddf_SPEC;
    pub type Spcnddf = crate::EnumBitfieldStruct<u8, Spcnddf_SPEC>;
    impl Spcnddf {
        #[doc = "STOP condition is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "STOP condition is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackdf_SPEC;
    pub type Nackdf = crate::EnumBitfieldStruct<u8, Nackdf_SPEC>;
    impl Nackdf {
        #[doc = "NACK is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "NACK is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tendf_SPEC;
    pub type Tendf = crate::EnumBitfieldStruct<u8, Tendf_SPEC>;
    impl Tendf {
        #[doc = "Data is being transmitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Data has been transmitted."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alf_SPEC;
    pub type Alf = crate::EnumBitfieldStruct<u8, Alf_SPEC>;
    impl Alf {
        #[doc = "Arbitration is not lost"]
        pub const _0: Self = Self::new(0);

        #[doc = "Arbitration is lost."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Todf_SPEC;
    pub type Todf = crate::EnumBitfieldStruct<u8, Todf_SPEC>;
    impl Todf {
        #[doc = "Timeout is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Timeout is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wucnddf_SPEC;
    pub type Wucnddf = crate::EnumBitfieldStruct<u8, Wucnddf_SPEC>;
    impl Wucnddf {
        #[doc = "Wake-Up is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Wake-Up is detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bste_SPEC;
impl crate::sealed::RegSpec for Bste_SPEC {
    type DataType = u32;
}

#[doc = "Bus Status Enable Register"]
pub type Bste = crate::RegValueT<Bste_SPEC>;

impl Bste {
    #[doc = "START Condition Detection Enable"]
    #[inline(always)]
    pub fn stcndde(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bste::Stcndde,
        bste::Stcndde,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bste::Stcndde,
            bste::Stcndde,
            Bste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "STOP Condition Detection Enable"]
    #[inline(always)]
    pub fn spcndde(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bste::Spcndde,
        bste::Spcndde,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bste::Spcndde,
            bste::Spcndde,
            Bste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NACK Detection Enable"]
    #[inline(always)]
    pub fn nackde(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        bste::Nackde,
        bste::Nackde,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            bste::Nackde,
            bste::Nackde,
            Bste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Enable"]
    #[inline(always)]
    pub fn tende(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bste::Tende,
        bste::Tende,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bste::Tende,
            bste::Tende,
            Bste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Arbitration Lost Enable"]
    #[inline(always)]
    pub fn ale(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        bste::Ale,
        bste::Ale,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            bste::Ale,
            bste::Ale,
            Bste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timeout Detection Enable"]
    #[inline(always)]
    pub fn tode(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        bste::Tode,
        bste::Tode,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            bste::Tode,
            bste::Tode,
            Bste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Wake-up Condition Detection Enable"]
    #[inline(always)]
    pub fn wucndde(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        bste::Wucndde,
        bste::Wucndde,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            bste::Wucndde,
            bste::Wucndde,
            Bste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bste {
    #[inline(always)]
    fn default() -> Bste {
        <crate::RegValueT<Bste_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bste {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stcndde_SPEC;
    pub type Stcndde = crate::EnumBitfieldStruct<u8, Stcndde_SPEC>;
    impl Stcndde {
        #[doc = "Disables START condition Detection Interrupt Status logging."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables START condition Detection Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcndde_SPEC;
    pub type Spcndde = crate::EnumBitfieldStruct<u8, Spcndde_SPEC>;
    impl Spcndde {
        #[doc = "Disables STOP condition Detection Interrupt Status logging."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables STOP condition Detection Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackde_SPEC;
    pub type Nackde = crate::EnumBitfieldStruct<u8, Nackde_SPEC>;
    impl Nackde {
        #[doc = "Disables NACK Detection Interrupt Status logging."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables NACK Detection Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tende_SPEC;
    pub type Tende = crate::EnumBitfieldStruct<u8, Tende_SPEC>;
    impl Tende {
        #[doc = "Disables Transmit End Interrupt Status logging."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables Transmit End Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ale_SPEC;
    pub type Ale = crate::EnumBitfieldStruct<u8, Ale_SPEC>;
    impl Ale {
        #[doc = "Disables Arbitration Lost Interrupt Status logging."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables Arbitration Lost Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tode_SPEC;
    pub type Tode = crate::EnumBitfieldStruct<u8, Tode_SPEC>;
    impl Tode {
        #[doc = "Disables Timeout Detection Interrupt Status logging."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables Timeout Detection Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wucndde_SPEC;
    pub type Wucndde = crate::EnumBitfieldStruct<u8, Wucndde_SPEC>;
    impl Wucndde {
        #[doc = "Disables Wake-up Condition Detection Status logging."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables Wake-up Condition Detection Status logging."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bie_SPEC;
impl crate::sealed::RegSpec for Bie_SPEC {
    type DataType = u32;
}

#[doc = "Bus Interrupt Enable Register"]
pub type Bie = crate::RegValueT<Bie_SPEC>;

impl Bie {
    #[doc = "START Condition Detection Interrupt Enable"]
    #[inline(always)]
    pub fn stcnddie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bie::Stcnddie,
        bie::Stcnddie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bie::Stcnddie,
            bie::Stcnddie,
            Bie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "STOP Condition Detection Interrupt Enable"]
    #[inline(always)]
    pub fn spcnddie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bie::Spcnddie,
        bie::Spcnddie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bie::Spcnddie,
            bie::Spcnddie,
            Bie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NACK Detection Interrupt Enable"]
    #[inline(always)]
    pub fn nackdie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        bie::Nackdie,
        bie::Nackdie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            bie::Nackdie,
            bie::Nackdie,
            Bie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn tendie(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bie::Tendie,
        bie::Tendie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bie::Tendie,
            bie::Tendie,
            Bie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn alie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        bie::Alie,
        bie::Alie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            bie::Alie,
            bie::Alie,
            Bie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timeout Detection Interrupt Enable"]
    #[inline(always)]
    pub fn todie(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        bie::Todie,
        bie::Todie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            bie::Todie,
            bie::Todie,
            Bie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Wake-Up Condition Detection Interrupt Enable"]
    #[inline(always)]
    pub fn wucnddie(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        bie::Wucnddie,
        bie::Wucnddie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            bie::Wucnddie,
            bie::Wucnddie,
            Bie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bie {
    #[inline(always)]
    fn default() -> Bie {
        <crate::RegValueT<Bie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stcnddie_SPEC;
    pub type Stcnddie = crate::EnumBitfieldStruct<u8, Stcnddie_SPEC>;
    impl Stcnddie {
        #[doc = "Disables START condition Detection Interrupt Signal."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables START condition Detection Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcnddie_SPEC;
    pub type Spcnddie = crate::EnumBitfieldStruct<u8, Spcnddie_SPEC>;
    impl Spcnddie {
        #[doc = "Disables STOP condition Detection Interrupt Signal."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables STOP condition Detection Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackdie_SPEC;
    pub type Nackdie = crate::EnumBitfieldStruct<u8, Nackdie_SPEC>;
    impl Nackdie {
        #[doc = "Disables NACK Detection Interrupt Signal."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables NACK Detection Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tendie_SPEC;
    pub type Tendie = crate::EnumBitfieldStruct<u8, Tendie_SPEC>;
    impl Tendie {
        #[doc = "Disables Transmit End Interrupt Signal."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables Transmit End Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alie_SPEC;
    pub type Alie = crate::EnumBitfieldStruct<u8, Alie_SPEC>;
    impl Alie {
        #[doc = "Disables Arbitration Lost Interrupt Signal."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables Arbitration Lost Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Todie_SPEC;
    pub type Todie = crate::EnumBitfieldStruct<u8, Todie_SPEC>;
    impl Todie {
        #[doc = "Disables Timeout Detection Interrupt Signal."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables Timeout Detection Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wucnddie_SPEC;
    pub type Wucnddie = crate::EnumBitfieldStruct<u8, Wucnddie_SPEC>;
    impl Wucnddie {
        #[doc = "Disables Wake-Up Condition Detection Interrupt Signal."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables Wake-Up Condition Detection Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bstfc_SPEC;
impl crate::sealed::RegSpec for Bstfc_SPEC {
    type DataType = u32;
}

#[doc = "Bus Status Force Register"]
pub type Bstfc = crate::RegValueT<Bstfc_SPEC>;

impl Bstfc {
    #[doc = "START condition Detection Force"]
    #[inline(always)]
    pub fn stcnddfc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bstfc::Stcnddfc,
        bstfc::Stcnddfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bstfc::Stcnddfc,
            bstfc::Stcnddfc,
            Bstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "STOP condition Detection Force"]
    #[inline(always)]
    pub fn spcnddfc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bstfc::Spcnddfc,
        bstfc::Spcnddfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bstfc::Spcnddfc,
            bstfc::Spcnddfc,
            Bstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "NACK Detection Force"]
    #[inline(always)]
    pub fn nackdfc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        bstfc::Nackdfc,
        bstfc::Nackdfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            bstfc::Nackdfc,
            bstfc::Nackdfc,
            Bstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Force"]
    #[inline(always)]
    pub fn tendfc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bstfc::Tendfc,
        bstfc::Tendfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bstfc::Tendfc,
            bstfc::Tendfc,
            Bstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Arbitration Lost Force"]
    #[inline(always)]
    pub fn alfc(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        bstfc::Alfc,
        bstfc::Alfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            bstfc::Alfc,
            bstfc::Alfc,
            Bstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Timeout Detection Force"]
    #[inline(always)]
    pub fn todfc(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        bstfc::Todfc,
        bstfc::Todfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            bstfc::Todfc,
            bstfc::Todfc,
            Bstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Wake-Up Condition Detection Force"]
    #[inline(always)]
    pub fn wucnddfc(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        bstfc::Wucnddfc,
        bstfc::Wucnddfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            bstfc::Wucnddfc,
            bstfc::Wucnddfc,
            Bstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bstfc {
    #[inline(always)]
    fn default() -> Bstfc {
        <crate::RegValueT<Bstfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bstfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stcnddfc_SPEC;
    pub type Stcnddfc = crate::EnumBitfieldStruct<u8, Stcnddfc_SPEC>;
    impl Stcnddfc {
        #[doc = "Not Force START condition Detection Interrupt for software testing."]
        pub const _0: Self = Self::new(0);

        #[doc = "Force START condition Detection Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcnddfc_SPEC;
    pub type Spcnddfc = crate::EnumBitfieldStruct<u8, Spcnddfc_SPEC>;
    impl Spcnddfc {
        #[doc = "Not Force STOP condition Detection Interrupt for software testing."]
        pub const _0: Self = Self::new(0);

        #[doc = "Force STOP condition Detection Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackdfc_SPEC;
    pub type Nackdfc = crate::EnumBitfieldStruct<u8, Nackdfc_SPEC>;
    impl Nackdfc {
        #[doc = "Not Force NACK Detection Interrupt for software testing."]
        pub const _0: Self = Self::new(0);

        #[doc = "Force NACK Detection Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tendfc_SPEC;
    pub type Tendfc = crate::EnumBitfieldStruct<u8, Tendfc_SPEC>;
    impl Tendfc {
        #[doc = "Not Force Transmit End Interrupt for software testing."]
        pub const _0: Self = Self::new(0);

        #[doc = "Force Transmit End Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alfc_SPEC;
    pub type Alfc = crate::EnumBitfieldStruct<u8, Alfc_SPEC>;
    impl Alfc {
        #[doc = "Not Force Arbitration Lost Interrupt for software testing."]
        pub const _0: Self = Self::new(0);

        #[doc = "Force Arbitration Lost Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Todfc_SPEC;
    pub type Todfc = crate::EnumBitfieldStruct<u8, Todfc_SPEC>;
    impl Todfc {
        #[doc = "Not Force Timeout Detection Interrupt for software testing."]
        pub const _0: Self = Self::new(0);

        #[doc = "Force Timeout Detection Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wucnddfc_SPEC;
    pub type Wucnddfc = crate::EnumBitfieldStruct<u8, Wucnddfc_SPEC>;
    impl Wucnddfc {
        #[doc = "Not Force Wake-Up Condition Detection Interrupt for software testing."]
        pub const _0: Self = Self::new(0);

        #[doc = "Force Wake-Up Condition Detection Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntst_SPEC;
impl crate::sealed::RegSpec for Ntst_SPEC {
    type DataType = u32;
}

#[doc = "Normal Transfer Status Register"]
pub type Ntst = crate::RegValueT<Ntst_SPEC>;

impl Ntst {
    #[doc = "Normal Transmit Data Buffer Empty Flag 0"]
    #[inline(always)]
    pub fn tdbef0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ntst::Tdbef0,
        ntst::Tdbef0,
        Ntst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ntst::Tdbef0,
            ntst::Tdbef0,
            Ntst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Normal Receive Data Buffer Full Flag 0"]
    #[inline(always)]
    pub fn rdbff0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ntst::Rdbff0,
        ntst::Rdbff0,
        Ntst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ntst::Rdbff0,
            ntst::Rdbff0,
            Ntst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ntst {
    #[inline(always)]
    fn default() -> Ntst {
        <crate::RegValueT<Ntst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ntst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbef0_SPEC;
    pub type Tdbef0 = crate::EnumBitfieldStruct<u8, Tdbef0_SPEC>;
    impl Tdbef0 {
        #[doc = "Normal Transmit Data Buffer 0 contains transmit data."]
        pub const _0: Self = Self::new(0);

        #[doc = "Normal Transmit Data Buffer 0 contains no transmit data."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbff0_SPEC;
    pub type Rdbff0 = crate::EnumBitfieldStruct<u8, Rdbff0_SPEC>;
    impl Rdbff0 {
        #[doc = "Normal Receive Data Buffer0 contains no receive data."]
        pub const _0: Self = Self::new(0);

        #[doc = "Normal Receive Data Buffer0 contains receive data."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntste_SPEC;
impl crate::sealed::RegSpec for Ntste_SPEC {
    type DataType = u32;
}

#[doc = "Normal Transfer Status Enable Register"]
pub type Ntste = crate::RegValueT<Ntste_SPEC>;

impl Ntste {
    #[doc = "Normal Transmit Data Buffer Empty Enable 0"]
    #[inline(always)]
    pub fn tdbee0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ntste::Tdbee0,
        ntste::Tdbee0,
        Ntste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ntste::Tdbee0,
            ntste::Tdbee0,
            Ntste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Normal Receive Data Buffer Full Enable 0"]
    #[inline(always)]
    pub fn rdbfe0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ntste::Rdbfe0,
        ntste::Rdbfe0,
        Ntste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ntste::Rdbfe0,
            ntste::Rdbfe0,
            Ntste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ntste {
    #[inline(always)]
    fn default() -> Ntste {
        <crate::RegValueT<Ntste_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ntste {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbee0_SPEC;
    pub type Tdbee0 = crate::EnumBitfieldStruct<u8, Tdbee0_SPEC>;
    impl Tdbee0 {
        #[doc = "Disables Tx0 Data Buffer Empty Interrupt Status logging."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables Tx0 Data Buffer Empty Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbfe0_SPEC;
    pub type Rdbfe0 = crate::EnumBitfieldStruct<u8, Rdbfe0_SPEC>;
    impl Rdbfe0 {
        #[doc = "Disables Rx0 Data Buffer Full Interrupt Status logging."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables Rx0 Data Buffer Full Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntie_SPEC;
impl crate::sealed::RegSpec for Ntie_SPEC {
    type DataType = u32;
}

#[doc = "Normal Transfer Interrupt Enable Register"]
pub type Ntie = crate::RegValueT<Ntie_SPEC>;

impl Ntie {
    #[doc = "Normal Transmit Data Buffer Empty Interrupt Enable 0"]
    #[inline(always)]
    pub fn tdbeie0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ntie::Tdbeie0,
        ntie::Tdbeie0,
        Ntie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ntie::Tdbeie0,
            ntie::Tdbeie0,
            Ntie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Normal Receive Data Buffer Full Interrupt Enable 0"]
    #[inline(always)]
    pub fn rdbfie0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ntie::Rdbfie0,
        ntie::Rdbfie0,
        Ntie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ntie::Rdbfie0,
            ntie::Rdbfie0,
            Ntie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ntie {
    #[inline(always)]
    fn default() -> Ntie {
        <crate::RegValueT<Ntie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ntie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbeie0_SPEC;
    pub type Tdbeie0 = crate::EnumBitfieldStruct<u8, Tdbeie0_SPEC>;
    impl Tdbeie0 {
        #[doc = "Disables Tx0 Data Buffer Empty Interrupt Signal."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables Tx0 Data Buffer Empty Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbfie0_SPEC;
    pub type Rdbfie0 = crate::EnumBitfieldStruct<u8, Rdbfie0_SPEC>;
    impl Rdbfie0 {
        #[doc = "Disables Rx0 Data Buffer Full Interrupt Signal."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables Rx0 Data Buffer Full Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntstfc_SPEC;
impl crate::sealed::RegSpec for Ntstfc_SPEC {
    type DataType = u32;
}

#[doc = "Normal Transfer Status Force Register"]
pub type Ntstfc = crate::RegValueT<Ntstfc_SPEC>;

impl Ntstfc {
    #[doc = "Normal Transmit Data Buffer Empty Force 0"]
    #[inline(always)]
    pub fn tdbefc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ntstfc::Tdbefc0,
        ntstfc::Tdbefc0,
        Ntstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ntstfc::Tdbefc0,
            ntstfc::Tdbefc0,
            Ntstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Normal Receive Data Buffer Full Force 0"]
    #[inline(always)]
    pub fn rdbffc0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ntstfc::Rdbffc0,
        ntstfc::Rdbffc0,
        Ntstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ntstfc::Rdbffc0,
            ntstfc::Rdbffc0,
            Ntstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ntstfc {
    #[inline(always)]
    fn default() -> Ntstfc {
        <crate::RegValueT<Ntstfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ntstfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbefc0_SPEC;
    pub type Tdbefc0 = crate::EnumBitfieldStruct<u8, Tdbefc0_SPEC>;
    impl Tdbefc0 {
        #[doc = "Not Force Tx0 Data Buffer Empty Interrupt for software testing."]
        pub const _0: Self = Self::new(0);

        #[doc = "Force Tx0 Data Buffer Empty Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbffc0_SPEC;
    pub type Rdbffc0 = crate::EnumBitfieldStruct<u8, Rdbffc0_SPEC>;
    impl Rdbffc0 {
        #[doc = "Not Force Rx0 Data Buffer Full Interrupt for software testing."]
        pub const _0: Self = Self::new(0);

        #[doc = "Force Rx0 Data Buffer Full Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcst_SPEC;
impl crate::sealed::RegSpec for Bcst_SPEC {
    type DataType = u32;
}

#[doc = "Bus Condition Status Register"]
pub type Bcst = crate::RegValueT<Bcst_SPEC>;

impl Bcst {
    #[doc = "Bus Free Detection Flag"]
    #[inline(always)]
    pub fn bfref(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bcst::Bfref,
        bcst::Bfref,
        Bcst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bcst::Bfref,
            bcst::Bfref,
            Bcst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bcst {
    #[inline(always)]
    fn default() -> Bcst {
        <crate::RegValueT<Bcst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bcst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfref_SPEC;
    pub type Bfref = crate::EnumBitfieldStruct<u8, Bfref_SPEC>;
    impl Bfref {
        #[doc = "Have not Detected Bus Free"]
        pub const _0: Self = Self::new(0);

        #[doc = "Have Detected Bus Free"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svst_SPEC;
impl crate::sealed::RegSpec for Svst_SPEC {
    type DataType = u32;
}

#[doc = "Slave Status Register"]
pub type Svst = crate::RegValueT<Svst_SPEC>;

impl Svst {
    #[doc = "General Call Address Detection Flag"]
    #[inline(always)]
    pub fn gcaf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        svst::Gcaf,
        svst::Gcaf,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            svst::Gcaf,
            svst::Gcaf,
            Svst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Hs-mode Master Code Detection Flag"]
    #[inline(always)]
    pub fn hsmcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        svst::Hsmcf,
        svst::Hsmcf,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            svst::Hsmcf,
            svst::Hsmcf,
            Svst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device-ID Address Detection Flag"]
    #[inline(always)]
    pub fn dvidf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        svst::Dvidf,
        svst::Dvidf,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            svst::Dvidf,
            svst::Dvidf,
            Svst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Host Address Detection Flag"]
    #[inline(always)]
    pub fn hoaf(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        svst::Hoaf,
        svst::Hoaf,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            svst::Hoaf,
            svst::Hoaf,
            Svst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Slave Address Detection Flag 0"]
    #[inline(always)]
    pub fn svaf0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        svst::Svaf0,
        svst::Svaf0,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            svst::Svaf0,
            svst::Svaf0,
            Svst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Slave Address Detection Flag 1"]
    #[inline(always)]
    pub fn svaf1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        svst::Svaf1,
        svst::Svaf1,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            svst::Svaf1,
            svst::Svaf1,
            Svst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Slave Address Detection Flag 2"]
    #[inline(always)]
    pub fn svaf2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        svst::Svaf2,
        svst::Svaf2,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            svst::Svaf2,
            svst::Svaf2,
            Svst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Svst {
    #[inline(always)]
    fn default() -> Svst {
        <crate::RegValueT<Svst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod svst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gcaf_SPEC;
    pub type Gcaf = crate::EnumBitfieldStruct<u8, Gcaf_SPEC>;
    impl Gcaf {
        #[doc = "General call address does not detect."]
        pub const _0: Self = Self::new(0);

        #[doc = "General call address detects."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsmcf_SPEC;
    pub type Hsmcf = crate::EnumBitfieldStruct<u8, Hsmcf_SPEC>;
    impl Hsmcf {
        #[doc = "Hs-mode Master Code does not detect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Hs-mode Master Code detects."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvidf_SPEC;
    pub type Dvidf = crate::EnumBitfieldStruct<u8, Dvidf_SPEC>;
    impl Dvidf {
        #[doc = "Device-ID command does not detect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Device-ID command detects. This bit set to 1 when the first frame received immediately after a START condition is detected matches a value of (device ID (1111 100) + 0\\[W\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hoaf_SPEC;
    pub type Hoaf = crate::EnumBitfieldStruct<u8, Hoaf_SPEC>;
    impl Hoaf {
        #[doc = "Host address does not detect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Host address detects. This bit set to 1 when the received slave address matches the host address (0001 000)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svaf0_SPEC;
    pub type Svaf0 = crate::EnumBitfieldStruct<u8, Svaf0_SPEC>;
    impl Svaf0 {
        #[doc = "Slave 0 does not detect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave 0 detect"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svaf1_SPEC;
    pub type Svaf1 = crate::EnumBitfieldStruct<u8, Svaf1_SPEC>;
    impl Svaf1 {
        #[doc = "Slave 1 does not detect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave 1 detect"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svaf2_SPEC;
    pub type Svaf2 = crate::EnumBitfieldStruct<u8, Svaf2_SPEC>;
    impl Svaf2 {
        #[doc = "Slave 2 does not detect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave 2 detect"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdatbas_SPEC;
impl crate::sealed::RegSpec for Sdatbas_SPEC {
    type DataType = u32;
}

#[doc = "Slave Device Address Table Basic Register %s"]
pub type Sdatbas = crate::RegValueT<Sdatbas_SPEC>;

impl Sdatbas {
    #[doc = "Slave Device Static Address"]
    #[inline(always)]
    pub fn sdstad(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Sdatbas_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Sdatbas_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave Device Address Length Selection"]
    #[inline(always)]
    pub fn sdadls(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        sdatbas::Sdadls,
        sdatbas::Sdadls,
        Sdatbas_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            sdatbas::Sdadls,
            sdatbas::Sdadls,
            Sdatbas_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdatbas {
    #[inline(always)]
    fn default() -> Sdatbas {
        <crate::RegValueT<Sdatbas_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdatbas {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadls_SPEC;
    pub type Sdadls = crate::EnumBitfieldStruct<u8, Sdadls_SPEC>;
    impl Sdadls {
        #[doc = "Slave Device address length 7 bits selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave Device address length 10 bits selected. (I2C device only)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svdvad_SPEC;
impl crate::sealed::RegSpec for Svdvad_SPEC {
    type DataType = u32;
}

#[doc = "Slave Device Address Register %s"]
pub type Svdvad = crate::RegValueT<Svdvad_SPEC>;

impl Svdvad {
    #[doc = "Slave Address"]
    #[inline(always)]
    pub fn svad(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Svdvad_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Svdvad_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Slave Address Length"]
    #[inline(always)]
    pub fn sadlg(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        svdvad::Sadlg,
        svdvad::Sadlg,
        Svdvad_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            svdvad::Sadlg,
            svdvad::Sadlg,
            Svdvad_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slave Static Address Valid"]
    #[inline(always)]
    pub fn sstadv(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        svdvad::Sstadv,
        svdvad::Sstadv,
        Svdvad_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            svdvad::Sstadv,
            svdvad::Sstadv,
            Svdvad_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Svdvad {
    #[inline(always)]
    fn default() -> Svdvad {
        <crate::RegValueT<Svdvad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod svdvad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadlg_SPEC;
    pub type Sadlg = crate::EnumBitfieldStruct<u8, Sadlg_SPEC>;
    impl Sadlg {
        #[doc = "The 7-bit address format is selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "The 10-bit address format is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sstadv_SPEC;
    pub type Sstadv = crate::EnumBitfieldStruct<u8, Sstadv_SPEC>;
    impl Sstadv {
        #[doc = "Slave address is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave address is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bitcnt_SPEC;
impl crate::sealed::RegSpec for Bitcnt_SPEC {
    type DataType = u32;
}

#[doc = "Bit Count Register"]
pub type Bitcnt = crate::RegValueT<Bitcnt_SPEC>;

impl Bitcnt {
    #[doc = "Bit Counter"]
    #[inline(always)]
    pub fn bcnt(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Bitcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Bitcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Bitcnt {
    #[inline(always)]
    fn default() -> Bitcnt {
        <crate::RegValueT<Bitcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstdbg_SPEC;
impl crate::sealed::RegSpec for Prstdbg_SPEC {
    type DataType = u32;
}

#[doc = "Present State Debug Register"]
pub type Prstdbg = crate::RegValueT<Prstdbg_SPEC>;

impl Prstdbg {
    #[doc = "SCL Line Signal Level"]
    #[inline(always)]
    pub fn scilv(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Prstdbg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Prstdbg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "SDA Line Signal Level"]
    #[inline(always)]
    pub fn sdilv(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Prstdbg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Prstdbg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "SCL Output Level"]
    #[inline(always)]
    pub fn scolv(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        prstdbg::Scolv,
        prstdbg::Scolv,
        Prstdbg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            prstdbg::Scolv,
            prstdbg::Scolv,
            Prstdbg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SDA Output Level"]
    #[inline(always)]
    pub fn sdolv(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        prstdbg::Sdolv,
        prstdbg::Sdolv,
        Prstdbg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            prstdbg::Sdolv,
            prstdbg::Sdolv,
            Prstdbg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Prstdbg {
    #[inline(always)]
    fn default() -> Prstdbg {
        <crate::RegValueT<Prstdbg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod prstdbg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scolv_SPEC;
    pub type Scolv = crate::EnumBitfieldStruct<u8, Scolv_SPEC>;
    impl Scolv {
        #[doc = "IIC has driven the SCL pin low."]
        pub const _0: Self = Self::new(0);

        #[doc = "IIC has released the SCL pin."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdolv_SPEC;
    pub type Sdolv = crate::EnumBitfieldStruct<u8, Sdolv_SPEC>;
    impl Sdolv {
        #[doc = "IIC has driven the SDA pin low."]
        pub const _0: Self = Self::new(0);

        #[doc = "IIC has released the SDA pin."]
        pub const _1: Self = Self::new(1);
    }
}
