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
#[doc = r"CPU System Security Control Unit"]
unsafe impl ::core::marker::Send for super::CpscuNs {}
unsafe impl ::core::marker::Sync for super::CpscuNs {}
impl super::CpscuNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "SRAM Security Attribution Register"]
    #[inline(always)]
    pub const fn sramsar(
        &self,
    ) -> &'static crate::common::Reg<self::Sramsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "DTC Controller Security Attribution Register"]
    #[inline(always)]
    pub const fn dtcsar(
        &self,
    ) -> &'static crate::common::Reg<self::Dtcsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dtcsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "DMAC Controller Security Attribution Register"]
    #[inline(always)]
    pub const fn dmacsar(
        &self,
    ) -> &'static crate::common::Reg<self::Dmacsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmacsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Interrupt Controller Unit Security Attribution Register A"]
    #[inline(always)]
    pub const fn icusara(
        &self,
    ) -> &'static crate::common::Reg<self::Icusara_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusara_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Interrupt Controller Unit Security Attribution Register B"]
    #[inline(always)]
    pub const fn icusarb(
        &self,
    ) -> &'static crate::common::Reg<self::Icusarb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusarb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Interrupt Controller Unit Security Attribution Register E"]
    #[inline(always)]
    pub const fn icusare(
        &self,
    ) -> &'static crate::common::Reg<self::Icusare_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusare_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Interrupt Controller Unit Security Attribution Register F"]
    #[inline(always)]
    pub const fn icusarf(
        &self,
    ) -> &'static crate::common::Reg<self::Icusarf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusarf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Interrupt Controller Unit Security Attribution Register G"]
    #[inline(always)]
    pub const fn icusarg(
        &self,
    ) -> &'static crate::common::Reg<self::Icusarg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusarg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "Interrupt Controller Unit Security Attribution Register H"]
    #[inline(always)]
    pub const fn icusarh(
        &self,
    ) -> &'static crate::common::Reg<self::Icusarh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusarh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "Interrupt Controller Unit Security Attribution Register I"]
    #[inline(always)]
    pub const fn icusari(
        &self,
    ) -> &'static crate::common::Reg<self::Icusari_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusari_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "Interrupt Controller Unit Security Attribution Register J"]
    #[inline(always)]
    pub const fn icusarj(
        &self,
    ) -> &'static crate::common::Reg<self::Icusarj_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusarj_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = "Interrupt Controller Unit Security Attribution Register K"]
    #[inline(always)]
    pub const fn icusark(
        &self,
    ) -> &'static crate::common::Reg<self::Icusark_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusark_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Interrupt Controller Unit Security Attribution Register L"]
    #[inline(always)]
    pub const fn icusarl(
        &self,
    ) -> &'static crate::common::Reg<self::Icusarl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusarl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "Bus Security Attribution Register A"]
    #[inline(always)]
    pub const fn bussara(
        &self,
    ) -> &'static crate::common::Reg<self::Bussara_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussara_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Bus Security Attribution Register B"]
    #[inline(always)]
    pub const fn bussarb(
        &self,
    ) -> &'static crate::common::Reg<self::Bussarb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussarb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "Bus Security Attribution Register C"]
    #[inline(always)]
    pub const fn bussarc(
        &self,
    ) -> &'static crate::common::Reg<self::Bussarc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussarc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "Bus Privileged Attribution Register C"]
    #[inline(always)]
    pub const fn busparc(
        &self,
    ) -> &'static crate::common::Reg<self::Busparc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busparc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(276usize),
            )
        }
    }

    #[doc = "Master Memory Protection Unit Security Attribution Register A"]
    #[inline(always)]
    pub const fn mmpusara(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusara_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusara_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[doc = "Master Memory Protection Unit Security Attribution Register B"]
    #[inline(always)]
    pub const fn mmpusarb(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusarb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusarb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(308usize),
            )
        }
    }

    #[doc = "CPU Security Attribution Register"]
    #[inline(always)]
    pub const fn cpusar(
        &self,
    ) -> &'static crate::common::Reg<self::Cpusar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cpusar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(368usize),
            )
        }
    }

    #[doc = "DMA Channel Security Attribution Register"]
    #[inline(always)]
    pub const fn dmacchsar(
        &self,
    ) -> &'static crate::common::Reg<self::Dmacchsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmacchsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(416usize),
            )
        }
    }

    #[doc = "DMA Channel Privilege Attribution Register"]
    #[inline(always)]
    pub const fn dmacchpar(
        &self,
    ) -> &'static crate::common::Reg<self::Dmacchpar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmacchpar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(496usize),
            )
        }
    }

    #[doc = "SRAM Security Attribute Boundary Address Register (n = 0 to 3)"]
    #[inline(always)]
    pub const fn sramsabar(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sramsabar_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x400usize))
        }
    }
    #[inline(always)]
    pub const fn sramsabar0(
        &self,
    ) -> &'static crate::common::Reg<self::Sramsabar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramsabar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x400usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sramsabar1(
        &self,
    ) -> &'static crate::common::Reg<self::Sramsabar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramsabar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x404usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sramsabar2(
        &self,
    ) -> &'static crate::common::Reg<self::Sramsabar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramsabar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x408usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sramsabar3(
        &self,
    ) -> &'static crate::common::Reg<self::Sramsabar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramsabar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40cusize),
            )
        }
    }

    #[doc = "Cache Security Attribution Register"]
    #[inline(always)]
    pub const fn cachesar(
        &self,
    ) -> &'static crate::common::Reg<self::Cachesar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cachesar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1280usize),
            )
        }
    }

    #[doc = "SRAM ECC region Security Attribute Register"]
    #[inline(always)]
    pub const fn sramesar(
        &self,
    ) -> &'static crate::common::Reg<self::Sramesar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramesar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1296usize),
            )
        }
    }

    #[doc = "Trusted Event Route Control Register"]
    #[inline(always)]
    pub const fn tevtrcr(
        &self,
    ) -> &'static crate::common::Reg<self::Tevtrcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tevtrcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1536usize),
            )
        }
    }

    #[doc = "IPC Security Attribution Register"]
    #[inline(always)]
    pub const fn ipcsar(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1552usize),
            )
        }
    }

    #[doc = "IPC Privileged Attribution Register"]
    #[inline(always)]
    pub const fn ipcpar(
        &self,
    ) -> &'static crate::common::Reg<self::Ipcpar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipcpar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1556usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramsar_SPEC;
impl crate::sealed::RegSpec for Sramsar_SPEC {
    type DataType = u32;
}

#[doc = "SRAM Security Attribution Register"]
pub type Sramsar = crate::RegValueT<Sramsar_SPEC>;

impl Sramsar {
    #[doc = "SRAM0 Register Security Attribution"]
    #[inline(always)]
    pub fn sramsa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramsar::Sramsa0,
        sramsar::Sramsa0,
        Sramsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramsar::Sramsa0,
            sramsar::Sramsa0,
            Sramsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SRAM1 Register Security Attribution"]
    #[inline(always)]
    pub fn sramsa1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sramsar::Sramsa1,
        sramsar::Sramsa1,
        Sramsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sramsar::Sramsa1,
            sramsar::Sramsa1,
            Sramsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SRAM2 Register Security Attribution"]
    #[inline(always)]
    pub fn sramsa2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sramsar::Sramsa2,
        sramsar::Sramsa2,
        Sramsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sramsar::Sramsa2,
            sramsar::Sramsa2,
            Sramsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SRAM3 Register Security Attribution"]
    #[inline(always)]
    pub fn sramsa3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sramsar::Sramsa3,
        sramsar::Sramsa3,
        Sramsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sramsar::Sramsa3,
            sramsar::Sramsa3,
            Sramsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SRAMWTSC Security Attribution"]
    #[inline(always)]
    pub fn sramwtsa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sramsar::Sramwtsa,
        sramsar::Sramwtsa,
        Sramsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sramsar::Sramwtsa,
            sramsar::Sramwtsa,
            Sramsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sramsar {
    #[inline(always)]
    fn default() -> Sramsar {
        <crate::RegValueT<Sramsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramsa0_SPEC;
    pub type Sramsa0 = crate::EnumBitfieldStruct<u8, Sramsa0_SPEC>;
    impl Sramsa0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramsa1_SPEC;
    pub type Sramsa1 = crate::EnumBitfieldStruct<u8, Sramsa1_SPEC>;
    impl Sramsa1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramsa2_SPEC;
    pub type Sramsa2 = crate::EnumBitfieldStruct<u8, Sramsa2_SPEC>;
    impl Sramsa2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramsa3_SPEC;
    pub type Sramsa3 = crate::EnumBitfieldStruct<u8, Sramsa3_SPEC>;
    impl Sramsa3 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramwtsa_SPEC;
    pub type Sramwtsa = crate::EnumBitfieldStruct<u8, Sramwtsa_SPEC>;
    impl Sramwtsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcsar_SPEC;
impl crate::sealed::RegSpec for Dtcsar_SPEC {
    type DataType = u32;
}

#[doc = "DTC Controller Security Attribution Register"]
pub type Dtcsar = crate::RegValueT<Dtcsar_SPEC>;

impl Dtcsar {
    #[doc = "DTC0 Security Attribution"]
    #[inline(always)]
    pub fn dtcstsa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dtcsar::Dtcstsa0,
        dtcsar::Dtcstsa0,
        Dtcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dtcsar::Dtcstsa0,
            dtcsar::Dtcstsa0,
            Dtcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC1 Security Attribution"]
    #[inline(always)]
    pub fn dtcstsa1(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dtcsar::Dtcstsa1,
        dtcsar::Dtcstsa1,
        Dtcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dtcsar::Dtcstsa1,
            dtcsar::Dtcstsa1,
            Dtcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dtcsar {
    #[inline(always)]
    fn default() -> Dtcsar {
        <crate::RegValueT<Dtcsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dtcsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtcstsa0_SPEC;
    pub type Dtcstsa0 = crate::EnumBitfieldStruct<u8, Dtcstsa0_SPEC>;
    impl Dtcstsa0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtcstsa1_SPEC;
    pub type Dtcstsa1 = crate::EnumBitfieldStruct<u8, Dtcstsa1_SPEC>;
    impl Dtcstsa1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacsar_SPEC;
impl crate::sealed::RegSpec for Dmacsar_SPEC {
    type DataType = u32;
}

#[doc = "DMAC Controller Security Attribution Register"]
pub type Dmacsar = crate::RegValueT<Dmacsar_SPEC>;

impl Dmacsar {
    #[doc = "DMAC0 DMAST Security Attribution"]
    #[inline(always)]
    pub fn dmastsa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmacsar::Dmastsa0,
        dmacsar::Dmastsa0,
        Dmacsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmacsar::Dmastsa0,
            dmacsar::Dmastsa0,
            Dmacsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DMAC1 DMAST Security Attribution"]
    #[inline(always)]
    pub fn dmastsa1(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dmacsar::Dmastsa1,
        dmacsar::Dmastsa1,
        Dmacsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dmacsar::Dmastsa1,
            dmacsar::Dmastsa1,
            Dmacsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmacsar {
    #[inline(always)]
    fn default() -> Dmacsar {
        <crate::RegValueT<Dmacsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmacsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dmastsa0_SPEC;
    pub type Dmastsa0 = crate::EnumBitfieldStruct<u8, Dmastsa0_SPEC>;
    impl Dmastsa0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dmastsa1_SPEC;
    pub type Dmastsa1 = crate::EnumBitfieldStruct<u8, Dmastsa1_SPEC>;
    impl Dmastsa1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusara_SPEC;
impl crate::sealed::RegSpec for Icusara_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Controller Unit Security Attribution Register A"]
pub type Icusara = crate::RegValueT<Icusara_SPEC>;

impl Icusara {
    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icusara::Sairqcr00,
        icusara::Sairqcr00,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icusara::Sairqcr00,
            icusara::Sairqcr00,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icusara::Sairqcr01,
        icusara::Sairqcr01,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icusara::Sairqcr01,
            icusara::Sairqcr01,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icusara::Sairqcr02,
        icusara::Sairqcr02,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icusara::Sairqcr02,
            icusara::Sairqcr02,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icusara::Sairqcr03,
        icusara::Sairqcr03,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icusara::Sairqcr03,
            icusara::Sairqcr03,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icusara::Sairqcr04,
        icusara::Sairqcr04,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icusara::Sairqcr04,
            icusara::Sairqcr04,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icusara::Sairqcr05,
        icusara::Sairqcr05,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icusara::Sairqcr05,
            icusara::Sairqcr05,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icusara::Sairqcr06,
        icusara::Sairqcr06,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icusara::Sairqcr06,
            icusara::Sairqcr06,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icusara::Sairqcr07,
        icusara::Sairqcr07,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icusara::Sairqcr07,
            icusara::Sairqcr07,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icusara::Sairqcr08,
        icusara::Sairqcr08,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icusara::Sairqcr08,
            icusara::Sairqcr08,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icusara::Sairqcr09,
        icusara::Sairqcr09,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icusara::Sairqcr09,
            icusara::Sairqcr09,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        icusara::Sairqcr10,
        icusara::Sairqcr10,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            icusara::Sairqcr10,
            icusara::Sairqcr10,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        icusara::Sairqcr11,
        icusara::Sairqcr11,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            icusara::Sairqcr11,
            icusara::Sairqcr11,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        icusara::Sairqcr12,
        icusara::Sairqcr12,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            icusara::Sairqcr12,
            icusara::Sairqcr12,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icusara::Sairqcr13,
        icusara::Sairqcr13,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icusara::Sairqcr13,
            icusara::Sairqcr13,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        icusara::Sairqcr14,
        icusara::Sairqcr14,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            icusara::Sairqcr14,
            icusara::Sairqcr14,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        icusara::Sairqcr15,
        icusara::Sairqcr15,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            icusara::Sairqcr15,
            icusara::Sairqcr15,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icusara::Sairqcr16,
        icusara::Sairqcr16,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icusara::Sairqcr16,
            icusara::Sairqcr16,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        icusara::Sairqcr17,
        icusara::Sairqcr17,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            icusara::Sairqcr17,
            icusara::Sairqcr17,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icusara::Sairqcr18,
        icusara::Sairqcr18,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icusara::Sairqcr18,
            icusara::Sairqcr18,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        icusara::Sairqcr19,
        icusara::Sairqcr19,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            icusara::Sairqcr19,
            icusara::Sairqcr19,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        icusara::Sairqcr20,
        icusara::Sairqcr20,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            icusara::Sairqcr20,
            icusara::Sairqcr20,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        icusara::Sairqcr21,
        icusara::Sairqcr21,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            icusara::Sairqcr21,
            icusara::Sairqcr21,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        icusara::Sairqcr22,
        icusara::Sairqcr22,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            icusara::Sairqcr22,
            icusara::Sairqcr22,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        icusara::Sairqcr23,
        icusara::Sairqcr23,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            icusara::Sairqcr23,
            icusara::Sairqcr23,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        icusara::Sairqcr24,
        icusara::Sairqcr24,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            icusara::Sairqcr24,
            icusara::Sairqcr24,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        icusara::Sairqcr25,
        icusara::Sairqcr25,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            icusara::Sairqcr25,
            icusara::Sairqcr25,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        icusara::Sairqcr26,
        icusara::Sairqcr26,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            icusara::Sairqcr26,
            icusara::Sairqcr26,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        icusara::Sairqcr27,
        icusara::Sairqcr27,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            icusara::Sairqcr27,
            icusara::Sairqcr27,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        icusara::Sairqcr28,
        icusara::Sairqcr28,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            icusara::Sairqcr28,
            icusara::Sairqcr28,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        icusara::Sairqcr29,
        icusara::Sairqcr29,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            icusara::Sairqcr29,
            icusara::Sairqcr29,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        icusara::Sairqcr30,
        icusara::Sairqcr30,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            icusara::Sairqcr30,
            icusara::Sairqcr30,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for the IRQCR, WUPEN0, WUPEN1 registers"]
    #[inline(always)]
    pub fn sairqcr31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        icusara::Sairqcr31,
        icusara::Sairqcr31,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            icusara::Sairqcr31,
            icusara::Sairqcr31,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icusara {
    #[inline(always)]
    fn default() -> Icusara {
        <crate::RegValueT<Icusara_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusara {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr00_SPEC;
    pub type Sairqcr00 = crate::EnumBitfieldStruct<u8, Sairqcr00_SPEC>;
    impl Sairqcr00 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr01_SPEC;
    pub type Sairqcr01 = crate::EnumBitfieldStruct<u8, Sairqcr01_SPEC>;
    impl Sairqcr01 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr02_SPEC;
    pub type Sairqcr02 = crate::EnumBitfieldStruct<u8, Sairqcr02_SPEC>;
    impl Sairqcr02 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr03_SPEC;
    pub type Sairqcr03 = crate::EnumBitfieldStruct<u8, Sairqcr03_SPEC>;
    impl Sairqcr03 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr04_SPEC;
    pub type Sairqcr04 = crate::EnumBitfieldStruct<u8, Sairqcr04_SPEC>;
    impl Sairqcr04 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr05_SPEC;
    pub type Sairqcr05 = crate::EnumBitfieldStruct<u8, Sairqcr05_SPEC>;
    impl Sairqcr05 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr06_SPEC;
    pub type Sairqcr06 = crate::EnumBitfieldStruct<u8, Sairqcr06_SPEC>;
    impl Sairqcr06 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr07_SPEC;
    pub type Sairqcr07 = crate::EnumBitfieldStruct<u8, Sairqcr07_SPEC>;
    impl Sairqcr07 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr08_SPEC;
    pub type Sairqcr08 = crate::EnumBitfieldStruct<u8, Sairqcr08_SPEC>;
    impl Sairqcr08 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr09_SPEC;
    pub type Sairqcr09 = crate::EnumBitfieldStruct<u8, Sairqcr09_SPEC>;
    impl Sairqcr09 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr10_SPEC;
    pub type Sairqcr10 = crate::EnumBitfieldStruct<u8, Sairqcr10_SPEC>;
    impl Sairqcr10 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr11_SPEC;
    pub type Sairqcr11 = crate::EnumBitfieldStruct<u8, Sairqcr11_SPEC>;
    impl Sairqcr11 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr12_SPEC;
    pub type Sairqcr12 = crate::EnumBitfieldStruct<u8, Sairqcr12_SPEC>;
    impl Sairqcr12 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr13_SPEC;
    pub type Sairqcr13 = crate::EnumBitfieldStruct<u8, Sairqcr13_SPEC>;
    impl Sairqcr13 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr14_SPEC;
    pub type Sairqcr14 = crate::EnumBitfieldStruct<u8, Sairqcr14_SPEC>;
    impl Sairqcr14 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr15_SPEC;
    pub type Sairqcr15 = crate::EnumBitfieldStruct<u8, Sairqcr15_SPEC>;
    impl Sairqcr15 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr16_SPEC;
    pub type Sairqcr16 = crate::EnumBitfieldStruct<u8, Sairqcr16_SPEC>;
    impl Sairqcr16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr17_SPEC;
    pub type Sairqcr17 = crate::EnumBitfieldStruct<u8, Sairqcr17_SPEC>;
    impl Sairqcr17 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr18_SPEC;
    pub type Sairqcr18 = crate::EnumBitfieldStruct<u8, Sairqcr18_SPEC>;
    impl Sairqcr18 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr19_SPEC;
    pub type Sairqcr19 = crate::EnumBitfieldStruct<u8, Sairqcr19_SPEC>;
    impl Sairqcr19 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr20_SPEC;
    pub type Sairqcr20 = crate::EnumBitfieldStruct<u8, Sairqcr20_SPEC>;
    impl Sairqcr20 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr21_SPEC;
    pub type Sairqcr21 = crate::EnumBitfieldStruct<u8, Sairqcr21_SPEC>;
    impl Sairqcr21 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr22_SPEC;
    pub type Sairqcr22 = crate::EnumBitfieldStruct<u8, Sairqcr22_SPEC>;
    impl Sairqcr22 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr23_SPEC;
    pub type Sairqcr23 = crate::EnumBitfieldStruct<u8, Sairqcr23_SPEC>;
    impl Sairqcr23 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr24_SPEC;
    pub type Sairqcr24 = crate::EnumBitfieldStruct<u8, Sairqcr24_SPEC>;
    impl Sairqcr24 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr25_SPEC;
    pub type Sairqcr25 = crate::EnumBitfieldStruct<u8, Sairqcr25_SPEC>;
    impl Sairqcr25 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr26_SPEC;
    pub type Sairqcr26 = crate::EnumBitfieldStruct<u8, Sairqcr26_SPEC>;
    impl Sairqcr26 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr27_SPEC;
    pub type Sairqcr27 = crate::EnumBitfieldStruct<u8, Sairqcr27_SPEC>;
    impl Sairqcr27 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr28_SPEC;
    pub type Sairqcr28 = crate::EnumBitfieldStruct<u8, Sairqcr28_SPEC>;
    impl Sairqcr28 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr29_SPEC;
    pub type Sairqcr29 = crate::EnumBitfieldStruct<u8, Sairqcr29_SPEC>;
    impl Sairqcr29 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr30_SPEC;
    pub type Sairqcr30 = crate::EnumBitfieldStruct<u8, Sairqcr30_SPEC>;
    impl Sairqcr30 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr31_SPEC;
    pub type Sairqcr31 = crate::EnumBitfieldStruct<u8, Sairqcr31_SPEC>;
    impl Sairqcr31 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusarb_SPEC;
impl crate::sealed::RegSpec for Icusarb_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Controller Unit Security Attribution Register B"]
pub type Icusarb = crate::RegValueT<Icusarb_SPEC>;

impl Icusarb {
    #[doc = "Security attributes of the NMICR register"]
    #[inline(always)]
    pub fn sanmi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icusarb::Sanmi,
        icusarb::Sanmi,
        Icusarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icusarb::Sanmi,
            icusarb::Sanmi,
            Icusarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of the ICU0.NMISR, ICU0.NMIER, ICU0.NMICLR registers"]
    #[inline(always)]
    pub fn sanmi0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icusarb::Sanmi0,
        icusarb::Sanmi0,
        Icusarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icusarb::Sanmi0,
            icusarb::Sanmi0,
            Icusarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of the ICU1.NMISR, ICU1.NMIER, ICU1.NMICLR registers"]
    #[inline(always)]
    pub fn sanmi1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icusarb::Sanmi1,
        icusarb::Sanmi1,
        Icusarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icusarb::Sanmi1,
            icusarb::Sanmi1,
            Icusarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icusarb {
    #[inline(always)]
    fn default() -> Icusarb {
        <crate::RegValueT<Icusarb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusarb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sanmi_SPEC;
    pub type Sanmi = crate::EnumBitfieldStruct<u8, Sanmi_SPEC>;
    impl Sanmi {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sanmi0_SPEC;
    pub type Sanmi0 = crate::EnumBitfieldStruct<u8, Sanmi0_SPEC>;
    impl Sanmi0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sanmi1_SPEC;
    pub type Sanmi1 = crate::EnumBitfieldStruct<u8, Sanmi1_SPEC>;
    impl Sanmi1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusare_SPEC;
impl crate::sealed::RegSpec for Icusare_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Controller Unit Security Attribution Register E"]
pub type Icusare = crate::RegValueT<Icusare_SPEC>;

impl Icusare {
    #[doc = "Security attributes of registers for WUPEN0.b16"]
    #[inline(always)]
    pub fn saiwdtwup(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icusare::Saiwdtwup,
        icusare::Saiwdtwup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icusare::Saiwdtwup,
            icusare::Saiwdtwup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN0.b18"]
    #[inline(always)]
    pub fn sapvd1wup(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icusare::Sapvd1Wup,
        icusare::Sapvd1Wup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icusare::Sapvd1Wup,
            icusare::Sapvd1Wup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN0.b19"]
    #[inline(always)]
    pub fn sapvd2wup(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        icusare::Sapvd2Wup,
        icusare::Sapvd2Wup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            icusare::Sapvd2Wup,
            icusare::Sapvd2Wup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN0.b20"]
    #[inline(always)]
    pub fn savbattwup(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        icusare::Savbattwup,
        icusare::Savbattwup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            icusare::Savbattwup,
            icusare::Savbattwup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN0.b24"]
    #[inline(always)]
    pub fn sartcalmwup(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        icusare::Sartcalmwup,
        icusare::Sartcalmwup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            icusare::Sartcalmwup,
            icusare::Sartcalmwup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN0.b25"]
    #[inline(always)]
    pub fn sartcprdwup(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        icusare::Sartcprdwup,
        icusare::Sartcprdwup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            icusare::Sartcprdwup,
            icusare::Sartcprdwup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN0.b26"]
    #[inline(always)]
    pub fn sausbhswup(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        icusare::Sausbhswup,
        icusare::Sausbhswup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            icusare::Sausbhswup,
            icusare::Sausbhswup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN0.b27"]
    #[inline(always)]
    pub fn sausbfs0wup(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        icusare::Sausbfs0Wup,
        icusare::Sausbfs0Wup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            icusare::Sausbfs0Wup,
            icusare::Sausbfs0Wup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN0.b28"]
    #[inline(always)]
    pub fn saagt1udwup(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        icusare::Saagt1Udwup,
        icusare::Saagt1Udwup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            icusare::Saagt1Udwup,
            icusare::Saagt1Udwup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN0.b29"]
    #[inline(always)]
    pub fn saagt1cawup(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        icusare::Saagt1Cawup,
        icusare::Saagt1Cawup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            icusare::Saagt1Cawup,
            icusare::Saagt1Cawup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN0.b30"]
    #[inline(always)]
    pub fn saagt1cbwup(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        icusare::Saagt1Cbwup,
        icusare::Saagt1Cbwup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            icusare::Saagt1Cbwup,
            icusare::Saagt1Cbwup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN0.b31"]
    #[inline(always)]
    pub fn saiic0wup(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        icusare::Saiic0Wup,
        icusare::Saiic0Wup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            icusare::Saiic0Wup,
            icusare::Saiic0Wup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icusare {
    #[inline(always)]
    fn default() -> Icusare {
        <crate::RegValueT<Icusare_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusare {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saiwdtwup_SPEC;
    pub type Saiwdtwup = crate::EnumBitfieldStruct<u8, Saiwdtwup_SPEC>;
    impl Saiwdtwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sapvd1Wup_SPEC;
    pub type Sapvd1Wup = crate::EnumBitfieldStruct<u8, Sapvd1Wup_SPEC>;
    impl Sapvd1Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sapvd2Wup_SPEC;
    pub type Sapvd2Wup = crate::EnumBitfieldStruct<u8, Sapvd2Wup_SPEC>;
    impl Sapvd2Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Savbattwup_SPEC;
    pub type Savbattwup = crate::EnumBitfieldStruct<u8, Savbattwup_SPEC>;
    impl Savbattwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sartcalmwup_SPEC;
    pub type Sartcalmwup = crate::EnumBitfieldStruct<u8, Sartcalmwup_SPEC>;
    impl Sartcalmwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sartcprdwup_SPEC;
    pub type Sartcprdwup = crate::EnumBitfieldStruct<u8, Sartcprdwup_SPEC>;
    impl Sartcprdwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sausbhswup_SPEC;
    pub type Sausbhswup = crate::EnumBitfieldStruct<u8, Sausbhswup_SPEC>;
    impl Sausbhswup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sausbfs0Wup_SPEC;
    pub type Sausbfs0Wup = crate::EnumBitfieldStruct<u8, Sausbfs0Wup_SPEC>;
    impl Sausbfs0Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saagt1Udwup_SPEC;
    pub type Saagt1Udwup = crate::EnumBitfieldStruct<u8, Saagt1Udwup_SPEC>;
    impl Saagt1Udwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saagt1Cawup_SPEC;
    pub type Saagt1Cawup = crate::EnumBitfieldStruct<u8, Saagt1Cawup_SPEC>;
    impl Saagt1Cawup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saagt1Cbwup_SPEC;
    pub type Saagt1Cbwup = crate::EnumBitfieldStruct<u8, Saagt1Cbwup_SPEC>;
    impl Saagt1Cbwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saiic0Wup_SPEC;
    pub type Saiic0Wup = crate::EnumBitfieldStruct<u8, Saiic0Wup_SPEC>;
    impl Saiic0Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusarf_SPEC;
impl crate::sealed::RegSpec for Icusarf_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Controller Unit Security Attribution Register F"]
pub type Icusarf = crate::RegValueT<Icusarf_SPEC>;

impl Icusarf {
    #[doc = "Security attributes of registers for WUPEN1.b3"]
    #[inline(always)]
    pub fn sacomphs0wup(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icusarf::Sacomphs0Wup,
        icusarf::Sacomphs0Wup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icusarf::Sacomphs0Wup,
            icusarf::Sacomphs0Wup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN1.b7"]
    #[inline(always)]
    pub fn sasoscwup(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icusarf::Sasoscwup,
        icusarf::Sasoscwup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icusarf::Sasoscwup,
            icusarf::Sasoscwup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN1.b8"]
    #[inline(always)]
    pub fn saulp0uwup(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icusarf::Saulp0Uwup,
        icusarf::Saulp0Uwup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icusarf::Saulp0Uwup,
            icusarf::Saulp0Uwup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN1.b9"]
    #[inline(always)]
    pub fn saulp0awup(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icusarf::Saulp0Awup,
        icusarf::Saulp0Awup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icusarf::Saulp0Awup,
            icusarf::Saulp0Awup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN1.b10"]
    #[inline(always)]
    pub fn saulp0bwup(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        icusarf::Saulp0Bwup,
        icusarf::Saulp0Bwup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            icusarf::Saulp0Bwup,
            icusarf::Saulp0Bwup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN1.b11"]
    #[inline(always)]
    pub fn sai3cwup(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        icusarf::Sai3Cwup,
        icusarf::Sai3Cwup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            icusarf::Sai3Cwup,
            icusarf::Sai3Cwup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN1.b12"]
    #[inline(always)]
    pub fn saulp1uwup(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        icusarf::Saulp1Uwup,
        icusarf::Saulp1Uwup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            icusarf::Saulp1Uwup,
            icusarf::Saulp1Uwup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN1.b13"]
    #[inline(always)]
    pub fn saulp1awup(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icusarf::Saulp1Awup,
        icusarf::Saulp1Awup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icusarf::Saulp1Awup,
            icusarf::Saulp1Awup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN1.b14"]
    #[inline(always)]
    pub fn saulp1bwup(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        icusarf::Saulp1Bwup,
        icusarf::Saulp1Bwup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            icusarf::Saulp1Bwup,
            icusarf::Saulp1Bwup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for WUPEN1.b15"]
    #[inline(always)]
    pub fn sapdmwup(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        icusarf::Sapdmwup,
        icusarf::Sapdmwup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            icusarf::Sapdmwup,
            icusarf::Sapdmwup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icusarf {
    #[inline(always)]
    fn default() -> Icusarf {
        <crate::RegValueT<Icusarf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusarf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sacomphs0Wup_SPEC;
    pub type Sacomphs0Wup = crate::EnumBitfieldStruct<u8, Sacomphs0Wup_SPEC>;
    impl Sacomphs0Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sasoscwup_SPEC;
    pub type Sasoscwup = crate::EnumBitfieldStruct<u8, Sasoscwup_SPEC>;
    impl Sasoscwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp0Uwup_SPEC;
    pub type Saulp0Uwup = crate::EnumBitfieldStruct<u8, Saulp0Uwup_SPEC>;
    impl Saulp0Uwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp0Awup_SPEC;
    pub type Saulp0Awup = crate::EnumBitfieldStruct<u8, Saulp0Awup_SPEC>;
    impl Saulp0Awup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp0Bwup_SPEC;
    pub type Saulp0Bwup = crate::EnumBitfieldStruct<u8, Saulp0Bwup_SPEC>;
    impl Saulp0Bwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sai3Cwup_SPEC;
    pub type Sai3Cwup = crate::EnumBitfieldStruct<u8, Sai3Cwup_SPEC>;
    impl Sai3Cwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp1Uwup_SPEC;
    pub type Saulp1Uwup = crate::EnumBitfieldStruct<u8, Saulp1Uwup_SPEC>;
    impl Saulp1Uwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp1Awup_SPEC;
    pub type Saulp1Awup = crate::EnumBitfieldStruct<u8, Saulp1Awup_SPEC>;
    impl Saulp1Awup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp1Bwup_SPEC;
    pub type Saulp1Bwup = crate::EnumBitfieldStruct<u8, Saulp1Bwup_SPEC>;
    impl Saulp1Bwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sapdmwup_SPEC;
    pub type Sapdmwup = crate::EnumBitfieldStruct<u8, Sapdmwup_SPEC>;
    impl Sapdmwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusarg_SPEC;
impl crate::sealed::RegSpec for Icusarg_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Controller Unit Security Attribution Register G"]
pub type Icusarg = crate::RegValueT<Icusarg_SPEC>;

impl Icusarg {
    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icusarg::Saielsr00,
        icusarg::Saielsr00,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icusarg::Saielsr00,
            icusarg::Saielsr00,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icusarg::Saielsr01,
        icusarg::Saielsr01,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icusarg::Saielsr01,
            icusarg::Saielsr01,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icusarg::Saielsr02,
        icusarg::Saielsr02,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icusarg::Saielsr02,
            icusarg::Saielsr02,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icusarg::Saielsr03,
        icusarg::Saielsr03,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icusarg::Saielsr03,
            icusarg::Saielsr03,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icusarg::Saielsr04,
        icusarg::Saielsr04,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icusarg::Saielsr04,
            icusarg::Saielsr04,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icusarg::Saielsr05,
        icusarg::Saielsr05,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icusarg::Saielsr05,
            icusarg::Saielsr05,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icusarg::Saielsr06,
        icusarg::Saielsr06,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icusarg::Saielsr06,
            icusarg::Saielsr06,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icusarg::Saielsr07,
        icusarg::Saielsr07,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icusarg::Saielsr07,
            icusarg::Saielsr07,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icusarg::Saielsr08,
        icusarg::Saielsr08,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icusarg::Saielsr08,
            icusarg::Saielsr08,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icusarg::Saielsr09,
        icusarg::Saielsr09,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icusarg::Saielsr09,
            icusarg::Saielsr09,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        icusarg::Saielsr10,
        icusarg::Saielsr10,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            icusarg::Saielsr10,
            icusarg::Saielsr10,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        icusarg::Saielsr11,
        icusarg::Saielsr11,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            icusarg::Saielsr11,
            icusarg::Saielsr11,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        icusarg::Saielsr12,
        icusarg::Saielsr12,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            icusarg::Saielsr12,
            icusarg::Saielsr12,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icusarg::Saielsr13,
        icusarg::Saielsr13,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icusarg::Saielsr13,
            icusarg::Saielsr13,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        icusarg::Saielsr14,
        icusarg::Saielsr14,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            icusarg::Saielsr14,
            icusarg::Saielsr14,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        icusarg::Saielsr15,
        icusarg::Saielsr15,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            icusarg::Saielsr15,
            icusarg::Saielsr15,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icusarg::Saielsr16,
        icusarg::Saielsr16,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icusarg::Saielsr16,
            icusarg::Saielsr16,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        icusarg::Saielsr17,
        icusarg::Saielsr17,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            icusarg::Saielsr17,
            icusarg::Saielsr17,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icusarg::Saielsr18,
        icusarg::Saielsr18,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icusarg::Saielsr18,
            icusarg::Saielsr18,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        icusarg::Saielsr19,
        icusarg::Saielsr19,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            icusarg::Saielsr19,
            icusarg::Saielsr19,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        icusarg::Saielsr20,
        icusarg::Saielsr20,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            icusarg::Saielsr20,
            icusarg::Saielsr20,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        icusarg::Saielsr21,
        icusarg::Saielsr21,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            icusarg::Saielsr21,
            icusarg::Saielsr21,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        icusarg::Saielsr22,
        icusarg::Saielsr22,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            icusarg::Saielsr22,
            icusarg::Saielsr22,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        icusarg::Saielsr23,
        icusarg::Saielsr23,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            icusarg::Saielsr23,
            icusarg::Saielsr23,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        icusarg::Saielsr24,
        icusarg::Saielsr24,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            icusarg::Saielsr24,
            icusarg::Saielsr24,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        icusarg::Saielsr25,
        icusarg::Saielsr25,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            icusarg::Saielsr25,
            icusarg::Saielsr25,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        icusarg::Saielsr26,
        icusarg::Saielsr26,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            icusarg::Saielsr26,
            icusarg::Saielsr26,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        icusarg::Saielsr27,
        icusarg::Saielsr27,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            icusarg::Saielsr27,
            icusarg::Saielsr27,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        icusarg::Saielsr28,
        icusarg::Saielsr28,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            icusarg::Saielsr28,
            icusarg::Saielsr28,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        icusarg::Saielsr29,
        icusarg::Saielsr29,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            icusarg::Saielsr29,
            icusarg::Saielsr29,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        icusarg::Saielsr30,
        icusarg::Saielsr30,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            icusarg::Saielsr30,
            icusarg::Saielsr30,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting0"]
    #[inline(always)]
    pub fn saielsr31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        icusarg::Saielsr31,
        icusarg::Saielsr31,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            icusarg::Saielsr31,
            icusarg::Saielsr31,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icusarg {
    #[inline(always)]
    fn default() -> Icusarg {
        <crate::RegValueT<Icusarg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusarg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr00_SPEC;
    pub type Saielsr00 = crate::EnumBitfieldStruct<u8, Saielsr00_SPEC>;
    impl Saielsr00 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr01_SPEC;
    pub type Saielsr01 = crate::EnumBitfieldStruct<u8, Saielsr01_SPEC>;
    impl Saielsr01 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr02_SPEC;
    pub type Saielsr02 = crate::EnumBitfieldStruct<u8, Saielsr02_SPEC>;
    impl Saielsr02 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr03_SPEC;
    pub type Saielsr03 = crate::EnumBitfieldStruct<u8, Saielsr03_SPEC>;
    impl Saielsr03 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr04_SPEC;
    pub type Saielsr04 = crate::EnumBitfieldStruct<u8, Saielsr04_SPEC>;
    impl Saielsr04 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr05_SPEC;
    pub type Saielsr05 = crate::EnumBitfieldStruct<u8, Saielsr05_SPEC>;
    impl Saielsr05 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr06_SPEC;
    pub type Saielsr06 = crate::EnumBitfieldStruct<u8, Saielsr06_SPEC>;
    impl Saielsr06 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr07_SPEC;
    pub type Saielsr07 = crate::EnumBitfieldStruct<u8, Saielsr07_SPEC>;
    impl Saielsr07 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr08_SPEC;
    pub type Saielsr08 = crate::EnumBitfieldStruct<u8, Saielsr08_SPEC>;
    impl Saielsr08 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr09_SPEC;
    pub type Saielsr09 = crate::EnumBitfieldStruct<u8, Saielsr09_SPEC>;
    impl Saielsr09 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr10_SPEC;
    pub type Saielsr10 = crate::EnumBitfieldStruct<u8, Saielsr10_SPEC>;
    impl Saielsr10 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr11_SPEC;
    pub type Saielsr11 = crate::EnumBitfieldStruct<u8, Saielsr11_SPEC>;
    impl Saielsr11 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr12_SPEC;
    pub type Saielsr12 = crate::EnumBitfieldStruct<u8, Saielsr12_SPEC>;
    impl Saielsr12 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr13_SPEC;
    pub type Saielsr13 = crate::EnumBitfieldStruct<u8, Saielsr13_SPEC>;
    impl Saielsr13 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr14_SPEC;
    pub type Saielsr14 = crate::EnumBitfieldStruct<u8, Saielsr14_SPEC>;
    impl Saielsr14 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr15_SPEC;
    pub type Saielsr15 = crate::EnumBitfieldStruct<u8, Saielsr15_SPEC>;
    impl Saielsr15 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr16_SPEC;
    pub type Saielsr16 = crate::EnumBitfieldStruct<u8, Saielsr16_SPEC>;
    impl Saielsr16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr17_SPEC;
    pub type Saielsr17 = crate::EnumBitfieldStruct<u8, Saielsr17_SPEC>;
    impl Saielsr17 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr18_SPEC;
    pub type Saielsr18 = crate::EnumBitfieldStruct<u8, Saielsr18_SPEC>;
    impl Saielsr18 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr19_SPEC;
    pub type Saielsr19 = crate::EnumBitfieldStruct<u8, Saielsr19_SPEC>;
    impl Saielsr19 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr20_SPEC;
    pub type Saielsr20 = crate::EnumBitfieldStruct<u8, Saielsr20_SPEC>;
    impl Saielsr20 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr21_SPEC;
    pub type Saielsr21 = crate::EnumBitfieldStruct<u8, Saielsr21_SPEC>;
    impl Saielsr21 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr22_SPEC;
    pub type Saielsr22 = crate::EnumBitfieldStruct<u8, Saielsr22_SPEC>;
    impl Saielsr22 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr23_SPEC;
    pub type Saielsr23 = crate::EnumBitfieldStruct<u8, Saielsr23_SPEC>;
    impl Saielsr23 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr24_SPEC;
    pub type Saielsr24 = crate::EnumBitfieldStruct<u8, Saielsr24_SPEC>;
    impl Saielsr24 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr25_SPEC;
    pub type Saielsr25 = crate::EnumBitfieldStruct<u8, Saielsr25_SPEC>;
    impl Saielsr25 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr26_SPEC;
    pub type Saielsr26 = crate::EnumBitfieldStruct<u8, Saielsr26_SPEC>;
    impl Saielsr26 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr27_SPEC;
    pub type Saielsr27 = crate::EnumBitfieldStruct<u8, Saielsr27_SPEC>;
    impl Saielsr27 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr28_SPEC;
    pub type Saielsr28 = crate::EnumBitfieldStruct<u8, Saielsr28_SPEC>;
    impl Saielsr28 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr29_SPEC;
    pub type Saielsr29 = crate::EnumBitfieldStruct<u8, Saielsr29_SPEC>;
    impl Saielsr29 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr30_SPEC;
    pub type Saielsr30 = crate::EnumBitfieldStruct<u8, Saielsr30_SPEC>;
    impl Saielsr30 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr31_SPEC;
    pub type Saielsr31 = crate::EnumBitfieldStruct<u8, Saielsr31_SPEC>;
    impl Saielsr31 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusarh_SPEC;
impl crate::sealed::RegSpec for Icusarh_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Controller Unit Security Attribution Register H"]
pub type Icusarh = crate::RegValueT<Icusarh_SPEC>;

impl Icusarh {
    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr32(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icusarh::Saielsr32,
        icusarh::Saielsr32,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icusarh::Saielsr32,
            icusarh::Saielsr32,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr33(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icusarh::Saielsr33,
        icusarh::Saielsr33,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icusarh::Saielsr33,
            icusarh::Saielsr33,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr34(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icusarh::Saielsr34,
        icusarh::Saielsr34,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icusarh::Saielsr34,
            icusarh::Saielsr34,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr35(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icusarh::Saielsr35,
        icusarh::Saielsr35,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icusarh::Saielsr35,
            icusarh::Saielsr35,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr36(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icusarh::Saielsr36,
        icusarh::Saielsr36,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icusarh::Saielsr36,
            icusarh::Saielsr36,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr37(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icusarh::Saielsr37,
        icusarh::Saielsr37,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icusarh::Saielsr37,
            icusarh::Saielsr37,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr38(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icusarh::Saielsr38,
        icusarh::Saielsr38,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icusarh::Saielsr38,
            icusarh::Saielsr38,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr39(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icusarh::Saielsr39,
        icusarh::Saielsr39,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icusarh::Saielsr39,
            icusarh::Saielsr39,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr40(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icusarh::Saielsr40,
        icusarh::Saielsr40,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icusarh::Saielsr40,
            icusarh::Saielsr40,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr41(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icusarh::Saielsr41,
        icusarh::Saielsr41,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icusarh::Saielsr41,
            icusarh::Saielsr41,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr42(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        icusarh::Saielsr42,
        icusarh::Saielsr42,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            icusarh::Saielsr42,
            icusarh::Saielsr42,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr43(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        icusarh::Saielsr43,
        icusarh::Saielsr43,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            icusarh::Saielsr43,
            icusarh::Saielsr43,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr44(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        icusarh::Saielsr44,
        icusarh::Saielsr44,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            icusarh::Saielsr44,
            icusarh::Saielsr44,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr45(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icusarh::Saielsr45,
        icusarh::Saielsr45,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icusarh::Saielsr45,
            icusarh::Saielsr45,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr46(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        icusarh::Saielsr46,
        icusarh::Saielsr46,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            icusarh::Saielsr46,
            icusarh::Saielsr46,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr47(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        icusarh::Saielsr47,
        icusarh::Saielsr47,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            icusarh::Saielsr47,
            icusarh::Saielsr47,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr48(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icusarh::Saielsr48,
        icusarh::Saielsr48,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icusarh::Saielsr48,
            icusarh::Saielsr48,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr49(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        icusarh::Saielsr49,
        icusarh::Saielsr49,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            icusarh::Saielsr49,
            icusarh::Saielsr49,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr50(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icusarh::Saielsr50,
        icusarh::Saielsr50,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icusarh::Saielsr50,
            icusarh::Saielsr50,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr51(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        icusarh::Saielsr51,
        icusarh::Saielsr51,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            icusarh::Saielsr51,
            icusarh::Saielsr51,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr52(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        icusarh::Saielsr52,
        icusarh::Saielsr52,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            icusarh::Saielsr52,
            icusarh::Saielsr52,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr53(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        icusarh::Saielsr53,
        icusarh::Saielsr53,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            icusarh::Saielsr53,
            icusarh::Saielsr53,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr54(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        icusarh::Saielsr54,
        icusarh::Saielsr54,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            icusarh::Saielsr54,
            icusarh::Saielsr54,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr55(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        icusarh::Saielsr55,
        icusarh::Saielsr55,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            icusarh::Saielsr55,
            icusarh::Saielsr55,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr56(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        icusarh::Saielsr56,
        icusarh::Saielsr56,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            icusarh::Saielsr56,
            icusarh::Saielsr56,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr57(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        icusarh::Saielsr57,
        icusarh::Saielsr57,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            icusarh::Saielsr57,
            icusarh::Saielsr57,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr58(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        icusarh::Saielsr58,
        icusarh::Saielsr58,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            icusarh::Saielsr58,
            icusarh::Saielsr58,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr59(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        icusarh::Saielsr59,
        icusarh::Saielsr59,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            icusarh::Saielsr59,
            icusarh::Saielsr59,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr60(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        icusarh::Saielsr60,
        icusarh::Saielsr60,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            icusarh::Saielsr60,
            icusarh::Saielsr60,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr61(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        icusarh::Saielsr61,
        icusarh::Saielsr61,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            icusarh::Saielsr61,
            icusarh::Saielsr61,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr62(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        icusarh::Saielsr62,
        icusarh::Saielsr62,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            icusarh::Saielsr62,
            icusarh::Saielsr62,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0event link setting1."]
    #[inline(always)]
    pub fn saielsr63(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        icusarh::Saielsr63,
        icusarh::Saielsr63,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            icusarh::Saielsr63,
            icusarh::Saielsr63,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icusarh {
    #[inline(always)]
    fn default() -> Icusarh {
        <crate::RegValueT<Icusarh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusarh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr32_SPEC;
    pub type Saielsr32 = crate::EnumBitfieldStruct<u8, Saielsr32_SPEC>;
    impl Saielsr32 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr33_SPEC;
    pub type Saielsr33 = crate::EnumBitfieldStruct<u8, Saielsr33_SPEC>;
    impl Saielsr33 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr34_SPEC;
    pub type Saielsr34 = crate::EnumBitfieldStruct<u8, Saielsr34_SPEC>;
    impl Saielsr34 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr35_SPEC;
    pub type Saielsr35 = crate::EnumBitfieldStruct<u8, Saielsr35_SPEC>;
    impl Saielsr35 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr36_SPEC;
    pub type Saielsr36 = crate::EnumBitfieldStruct<u8, Saielsr36_SPEC>;
    impl Saielsr36 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr37_SPEC;
    pub type Saielsr37 = crate::EnumBitfieldStruct<u8, Saielsr37_SPEC>;
    impl Saielsr37 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr38_SPEC;
    pub type Saielsr38 = crate::EnumBitfieldStruct<u8, Saielsr38_SPEC>;
    impl Saielsr38 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr39_SPEC;
    pub type Saielsr39 = crate::EnumBitfieldStruct<u8, Saielsr39_SPEC>;
    impl Saielsr39 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr40_SPEC;
    pub type Saielsr40 = crate::EnumBitfieldStruct<u8, Saielsr40_SPEC>;
    impl Saielsr40 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr41_SPEC;
    pub type Saielsr41 = crate::EnumBitfieldStruct<u8, Saielsr41_SPEC>;
    impl Saielsr41 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr42_SPEC;
    pub type Saielsr42 = crate::EnumBitfieldStruct<u8, Saielsr42_SPEC>;
    impl Saielsr42 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr43_SPEC;
    pub type Saielsr43 = crate::EnumBitfieldStruct<u8, Saielsr43_SPEC>;
    impl Saielsr43 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr44_SPEC;
    pub type Saielsr44 = crate::EnumBitfieldStruct<u8, Saielsr44_SPEC>;
    impl Saielsr44 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr45_SPEC;
    pub type Saielsr45 = crate::EnumBitfieldStruct<u8, Saielsr45_SPEC>;
    impl Saielsr45 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr46_SPEC;
    pub type Saielsr46 = crate::EnumBitfieldStruct<u8, Saielsr46_SPEC>;
    impl Saielsr46 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr47_SPEC;
    pub type Saielsr47 = crate::EnumBitfieldStruct<u8, Saielsr47_SPEC>;
    impl Saielsr47 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr48_SPEC;
    pub type Saielsr48 = crate::EnumBitfieldStruct<u8, Saielsr48_SPEC>;
    impl Saielsr48 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr49_SPEC;
    pub type Saielsr49 = crate::EnumBitfieldStruct<u8, Saielsr49_SPEC>;
    impl Saielsr49 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr50_SPEC;
    pub type Saielsr50 = crate::EnumBitfieldStruct<u8, Saielsr50_SPEC>;
    impl Saielsr50 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr51_SPEC;
    pub type Saielsr51 = crate::EnumBitfieldStruct<u8, Saielsr51_SPEC>;
    impl Saielsr51 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr52_SPEC;
    pub type Saielsr52 = crate::EnumBitfieldStruct<u8, Saielsr52_SPEC>;
    impl Saielsr52 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr53_SPEC;
    pub type Saielsr53 = crate::EnumBitfieldStruct<u8, Saielsr53_SPEC>;
    impl Saielsr53 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr54_SPEC;
    pub type Saielsr54 = crate::EnumBitfieldStruct<u8, Saielsr54_SPEC>;
    impl Saielsr54 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr55_SPEC;
    pub type Saielsr55 = crate::EnumBitfieldStruct<u8, Saielsr55_SPEC>;
    impl Saielsr55 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr56_SPEC;
    pub type Saielsr56 = crate::EnumBitfieldStruct<u8, Saielsr56_SPEC>;
    impl Saielsr56 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr57_SPEC;
    pub type Saielsr57 = crate::EnumBitfieldStruct<u8, Saielsr57_SPEC>;
    impl Saielsr57 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr58_SPEC;
    pub type Saielsr58 = crate::EnumBitfieldStruct<u8, Saielsr58_SPEC>;
    impl Saielsr58 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr59_SPEC;
    pub type Saielsr59 = crate::EnumBitfieldStruct<u8, Saielsr59_SPEC>;
    impl Saielsr59 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr60_SPEC;
    pub type Saielsr60 = crate::EnumBitfieldStruct<u8, Saielsr60_SPEC>;
    impl Saielsr60 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr61_SPEC;
    pub type Saielsr61 = crate::EnumBitfieldStruct<u8, Saielsr61_SPEC>;
    impl Saielsr61 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr62_SPEC;
    pub type Saielsr62 = crate::EnumBitfieldStruct<u8, Saielsr62_SPEC>;
    impl Saielsr62 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr63_SPEC;
    pub type Saielsr63 = crate::EnumBitfieldStruct<u8, Saielsr63_SPEC>;
    impl Saielsr63 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusari_SPEC;
impl crate::sealed::RegSpec for Icusari_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Controller Unit Security Attribution Register I"]
pub type Icusari = crate::RegValueT<Icusari_SPEC>;

impl Icusari {
    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr64(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icusari::Saielsr64,
        icusari::Saielsr64,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icusari::Saielsr64,
            icusari::Saielsr64,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr65(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icusari::Saielsr65,
        icusari::Saielsr65,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icusari::Saielsr65,
            icusari::Saielsr65,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr66(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icusari::Saielsr66,
        icusari::Saielsr66,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icusari::Saielsr66,
            icusari::Saielsr66,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr67(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icusari::Saielsr67,
        icusari::Saielsr67,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icusari::Saielsr67,
            icusari::Saielsr67,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr68(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icusari::Saielsr68,
        icusari::Saielsr68,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icusari::Saielsr68,
            icusari::Saielsr68,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr69(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icusari::Saielsr69,
        icusari::Saielsr69,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icusari::Saielsr69,
            icusari::Saielsr69,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr70(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icusari::Saielsr70,
        icusari::Saielsr70,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icusari::Saielsr70,
            icusari::Saielsr70,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr71(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icusari::Saielsr71,
        icusari::Saielsr71,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icusari::Saielsr71,
            icusari::Saielsr71,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr72(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icusari::Saielsr72,
        icusari::Saielsr72,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icusari::Saielsr72,
            icusari::Saielsr72,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr73(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icusari::Saielsr73,
        icusari::Saielsr73,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icusari::Saielsr73,
            icusari::Saielsr73,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr74(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        icusari::Saielsr74,
        icusari::Saielsr74,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            icusari::Saielsr74,
            icusari::Saielsr74,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr75(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        icusari::Saielsr75,
        icusari::Saielsr75,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            icusari::Saielsr75,
            icusari::Saielsr75,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr76(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        icusari::Saielsr76,
        icusari::Saielsr76,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            icusari::Saielsr76,
            icusari::Saielsr76,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr77(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icusari::Saielsr77,
        icusari::Saielsr77,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icusari::Saielsr77,
            icusari::Saielsr77,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr78(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        icusari::Saielsr78,
        icusari::Saielsr78,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            icusari::Saielsr78,
            icusari::Saielsr78,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr79(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        icusari::Saielsr79,
        icusari::Saielsr79,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            icusari::Saielsr79,
            icusari::Saielsr79,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr80(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icusari::Saielsr80,
        icusari::Saielsr80,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icusari::Saielsr80,
            icusari::Saielsr80,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr81(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        icusari::Saielsr81,
        icusari::Saielsr81,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            icusari::Saielsr81,
            icusari::Saielsr81,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr82(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icusari::Saielsr82,
        icusari::Saielsr82,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icusari::Saielsr82,
            icusari::Saielsr82,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr83(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        icusari::Saielsr83,
        icusari::Saielsr83,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            icusari::Saielsr83,
            icusari::Saielsr83,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr84(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        icusari::Saielsr84,
        icusari::Saielsr84,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            icusari::Saielsr84,
            icusari::Saielsr84,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr85(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        icusari::Saielsr85,
        icusari::Saielsr85,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            icusari::Saielsr85,
            icusari::Saielsr85,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr86(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        icusari::Saielsr86,
        icusari::Saielsr86,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            icusari::Saielsr86,
            icusari::Saielsr86,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr87(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        icusari::Saielsr87,
        icusari::Saielsr87,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            icusari::Saielsr87,
            icusari::Saielsr87,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr88(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        icusari::Saielsr88,
        icusari::Saielsr88,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            icusari::Saielsr88,
            icusari::Saielsr88,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr89(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        icusari::Saielsr89,
        icusari::Saielsr89,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            icusari::Saielsr89,
            icusari::Saielsr89,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr90(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        icusari::Saielsr90,
        icusari::Saielsr90,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            icusari::Saielsr90,
            icusari::Saielsr90,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr91(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        icusari::Saielsr91,
        icusari::Saielsr91,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            icusari::Saielsr91,
            icusari::Saielsr91,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr92(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        icusari::Saielsr92,
        icusari::Saielsr92,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            icusari::Saielsr92,
            icusari::Saielsr92,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr93(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        icusari::Saielsr93,
        icusari::Saielsr93,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            icusari::Saielsr93,
            icusari::Saielsr93,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr94(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        icusari::Saielsr94,
        icusari::Saielsr94,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            icusari::Saielsr94,
            icusari::Saielsr94,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU0 event link setting2."]
    #[inline(always)]
    pub fn saielsr95(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        icusari::Saielsr95,
        icusari::Saielsr95,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            icusari::Saielsr95,
            icusari::Saielsr95,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icusari {
    #[inline(always)]
    fn default() -> Icusari {
        <crate::RegValueT<Icusari_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusari {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr64_SPEC;
    pub type Saielsr64 = crate::EnumBitfieldStruct<u8, Saielsr64_SPEC>;
    impl Saielsr64 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr65_SPEC;
    pub type Saielsr65 = crate::EnumBitfieldStruct<u8, Saielsr65_SPEC>;
    impl Saielsr65 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr66_SPEC;
    pub type Saielsr66 = crate::EnumBitfieldStruct<u8, Saielsr66_SPEC>;
    impl Saielsr66 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr67_SPEC;
    pub type Saielsr67 = crate::EnumBitfieldStruct<u8, Saielsr67_SPEC>;
    impl Saielsr67 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr68_SPEC;
    pub type Saielsr68 = crate::EnumBitfieldStruct<u8, Saielsr68_SPEC>;
    impl Saielsr68 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr69_SPEC;
    pub type Saielsr69 = crate::EnumBitfieldStruct<u8, Saielsr69_SPEC>;
    impl Saielsr69 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr70_SPEC;
    pub type Saielsr70 = crate::EnumBitfieldStruct<u8, Saielsr70_SPEC>;
    impl Saielsr70 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr71_SPEC;
    pub type Saielsr71 = crate::EnumBitfieldStruct<u8, Saielsr71_SPEC>;
    impl Saielsr71 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr72_SPEC;
    pub type Saielsr72 = crate::EnumBitfieldStruct<u8, Saielsr72_SPEC>;
    impl Saielsr72 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr73_SPEC;
    pub type Saielsr73 = crate::EnumBitfieldStruct<u8, Saielsr73_SPEC>;
    impl Saielsr73 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr74_SPEC;
    pub type Saielsr74 = crate::EnumBitfieldStruct<u8, Saielsr74_SPEC>;
    impl Saielsr74 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr75_SPEC;
    pub type Saielsr75 = crate::EnumBitfieldStruct<u8, Saielsr75_SPEC>;
    impl Saielsr75 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr76_SPEC;
    pub type Saielsr76 = crate::EnumBitfieldStruct<u8, Saielsr76_SPEC>;
    impl Saielsr76 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr77_SPEC;
    pub type Saielsr77 = crate::EnumBitfieldStruct<u8, Saielsr77_SPEC>;
    impl Saielsr77 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr78_SPEC;
    pub type Saielsr78 = crate::EnumBitfieldStruct<u8, Saielsr78_SPEC>;
    impl Saielsr78 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr79_SPEC;
    pub type Saielsr79 = crate::EnumBitfieldStruct<u8, Saielsr79_SPEC>;
    impl Saielsr79 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr80_SPEC;
    pub type Saielsr80 = crate::EnumBitfieldStruct<u8, Saielsr80_SPEC>;
    impl Saielsr80 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr81_SPEC;
    pub type Saielsr81 = crate::EnumBitfieldStruct<u8, Saielsr81_SPEC>;
    impl Saielsr81 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr82_SPEC;
    pub type Saielsr82 = crate::EnumBitfieldStruct<u8, Saielsr82_SPEC>;
    impl Saielsr82 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr83_SPEC;
    pub type Saielsr83 = crate::EnumBitfieldStruct<u8, Saielsr83_SPEC>;
    impl Saielsr83 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr84_SPEC;
    pub type Saielsr84 = crate::EnumBitfieldStruct<u8, Saielsr84_SPEC>;
    impl Saielsr84 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr85_SPEC;
    pub type Saielsr85 = crate::EnumBitfieldStruct<u8, Saielsr85_SPEC>;
    impl Saielsr85 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr86_SPEC;
    pub type Saielsr86 = crate::EnumBitfieldStruct<u8, Saielsr86_SPEC>;
    impl Saielsr86 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr87_SPEC;
    pub type Saielsr87 = crate::EnumBitfieldStruct<u8, Saielsr87_SPEC>;
    impl Saielsr87 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr88_SPEC;
    pub type Saielsr88 = crate::EnumBitfieldStruct<u8, Saielsr88_SPEC>;
    impl Saielsr88 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr89_SPEC;
    pub type Saielsr89 = crate::EnumBitfieldStruct<u8, Saielsr89_SPEC>;
    impl Saielsr89 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr90_SPEC;
    pub type Saielsr90 = crate::EnumBitfieldStruct<u8, Saielsr90_SPEC>;
    impl Saielsr90 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr91_SPEC;
    pub type Saielsr91 = crate::EnumBitfieldStruct<u8, Saielsr91_SPEC>;
    impl Saielsr91 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr92_SPEC;
    pub type Saielsr92 = crate::EnumBitfieldStruct<u8, Saielsr92_SPEC>;
    impl Saielsr92 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr93_SPEC;
    pub type Saielsr93 = crate::EnumBitfieldStruct<u8, Saielsr93_SPEC>;
    impl Saielsr93 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr94_SPEC;
    pub type Saielsr94 = crate::EnumBitfieldStruct<u8, Saielsr94_SPEC>;
    impl Saielsr94 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr95_SPEC;
    pub type Saielsr95 = crate::EnumBitfieldStruct<u8, Saielsr95_SPEC>;
    impl Saielsr95 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusarj_SPEC;
impl crate::sealed::RegSpec for Icusarj_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Controller Unit Security Attribution Register J"]
pub type Icusarj = crate::RegValueT<Icusarj_SPEC>;

impl Icusarj {
    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icusarj::Saielsr0,
        icusarj::Saielsr0,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icusarj::Saielsr0,
            icusarj::Saielsr0,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icusarj::Saielsr1,
        icusarj::Saielsr1,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icusarj::Saielsr1,
            icusarj::Saielsr1,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icusarj::Saielsr2,
        icusarj::Saielsr2,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icusarj::Saielsr2,
            icusarj::Saielsr2,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icusarj::Saielsr3,
        icusarj::Saielsr3,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icusarj::Saielsr3,
            icusarj::Saielsr3,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icusarj::Saielsr4,
        icusarj::Saielsr4,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icusarj::Saielsr4,
            icusarj::Saielsr4,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icusarj::Saielsr5,
        icusarj::Saielsr5,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icusarj::Saielsr5,
            icusarj::Saielsr5,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icusarj::Saielsr6,
        icusarj::Saielsr6,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icusarj::Saielsr6,
            icusarj::Saielsr6,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icusarj::Saielsr7,
        icusarj::Saielsr7,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icusarj::Saielsr7,
            icusarj::Saielsr7,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icusarj::Saielsr8,
        icusarj::Saielsr8,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icusarj::Saielsr8,
            icusarj::Saielsr8,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icusarj::Saielsr9,
        icusarj::Saielsr9,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icusarj::Saielsr9,
            icusarj::Saielsr9,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        icusarj::Saielsr10,
        icusarj::Saielsr10,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            icusarj::Saielsr10,
            icusarj::Saielsr10,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        icusarj::Saielsr11,
        icusarj::Saielsr11,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            icusarj::Saielsr11,
            icusarj::Saielsr11,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        icusarj::Saielsr12,
        icusarj::Saielsr12,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            icusarj::Saielsr12,
            icusarj::Saielsr12,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icusarj::Saielsr13,
        icusarj::Saielsr13,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icusarj::Saielsr13,
            icusarj::Saielsr13,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        icusarj::Saielsr14,
        icusarj::Saielsr14,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            icusarj::Saielsr14,
            icusarj::Saielsr14,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        icusarj::Saielsr15,
        icusarj::Saielsr15,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            icusarj::Saielsr15,
            icusarj::Saielsr15,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icusarj::Saielsr16,
        icusarj::Saielsr16,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icusarj::Saielsr16,
            icusarj::Saielsr16,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        icusarj::Saielsr17,
        icusarj::Saielsr17,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            icusarj::Saielsr17,
            icusarj::Saielsr17,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icusarj::Saielsr18,
        icusarj::Saielsr18,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icusarj::Saielsr18,
            icusarj::Saielsr18,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        icusarj::Saielsr19,
        icusarj::Saielsr19,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            icusarj::Saielsr19,
            icusarj::Saielsr19,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        icusarj::Saielsr20,
        icusarj::Saielsr20,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            icusarj::Saielsr20,
            icusarj::Saielsr20,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        icusarj::Saielsr21,
        icusarj::Saielsr21,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            icusarj::Saielsr21,
            icusarj::Saielsr21,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        icusarj::Saielsr22,
        icusarj::Saielsr22,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            icusarj::Saielsr22,
            icusarj::Saielsr22,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        icusarj::Saielsr23,
        icusarj::Saielsr23,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            icusarj::Saielsr23,
            icusarj::Saielsr23,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        icusarj::Saielsr24,
        icusarj::Saielsr24,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            icusarj::Saielsr24,
            icusarj::Saielsr24,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        icusarj::Saielsr25,
        icusarj::Saielsr25,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            icusarj::Saielsr25,
            icusarj::Saielsr25,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        icusarj::Saielsr26,
        icusarj::Saielsr26,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            icusarj::Saielsr26,
            icusarj::Saielsr26,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        icusarj::Saielsr27,
        icusarj::Saielsr27,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            icusarj::Saielsr27,
            icusarj::Saielsr27,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        icusarj::Saielsr28,
        icusarj::Saielsr28,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            icusarj::Saielsr28,
            icusarj::Saielsr28,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        icusarj::Saielsr29,
        icusarj::Saielsr29,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            icusarj::Saielsr29,
            icusarj::Saielsr29,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        icusarj::Saielsr30,
        icusarj::Saielsr30,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            icusarj::Saielsr30,
            icusarj::Saielsr30,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting0."]
    #[inline(always)]
    pub fn saielsr31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        icusarj::Saielsr31,
        icusarj::Saielsr31,
        Icusarj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            icusarj::Saielsr31,
            icusarj::Saielsr31,
            Icusarj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icusarj {
    #[inline(always)]
    fn default() -> Icusarj {
        <crate::RegValueT<Icusarj_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusarj {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr0_SPEC;
    pub type Saielsr0 = crate::EnumBitfieldStruct<u8, Saielsr0_SPEC>;
    impl Saielsr0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr1_SPEC;
    pub type Saielsr1 = crate::EnumBitfieldStruct<u8, Saielsr1_SPEC>;
    impl Saielsr1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr2_SPEC;
    pub type Saielsr2 = crate::EnumBitfieldStruct<u8, Saielsr2_SPEC>;
    impl Saielsr2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr3_SPEC;
    pub type Saielsr3 = crate::EnumBitfieldStruct<u8, Saielsr3_SPEC>;
    impl Saielsr3 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr4_SPEC;
    pub type Saielsr4 = crate::EnumBitfieldStruct<u8, Saielsr4_SPEC>;
    impl Saielsr4 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr5_SPEC;
    pub type Saielsr5 = crate::EnumBitfieldStruct<u8, Saielsr5_SPEC>;
    impl Saielsr5 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr6_SPEC;
    pub type Saielsr6 = crate::EnumBitfieldStruct<u8, Saielsr6_SPEC>;
    impl Saielsr6 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr7_SPEC;
    pub type Saielsr7 = crate::EnumBitfieldStruct<u8, Saielsr7_SPEC>;
    impl Saielsr7 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr8_SPEC;
    pub type Saielsr8 = crate::EnumBitfieldStruct<u8, Saielsr8_SPEC>;
    impl Saielsr8 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr9_SPEC;
    pub type Saielsr9 = crate::EnumBitfieldStruct<u8, Saielsr9_SPEC>;
    impl Saielsr9 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr10_SPEC;
    pub type Saielsr10 = crate::EnumBitfieldStruct<u8, Saielsr10_SPEC>;
    impl Saielsr10 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr11_SPEC;
    pub type Saielsr11 = crate::EnumBitfieldStruct<u8, Saielsr11_SPEC>;
    impl Saielsr11 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr12_SPEC;
    pub type Saielsr12 = crate::EnumBitfieldStruct<u8, Saielsr12_SPEC>;
    impl Saielsr12 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr13_SPEC;
    pub type Saielsr13 = crate::EnumBitfieldStruct<u8, Saielsr13_SPEC>;
    impl Saielsr13 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr14_SPEC;
    pub type Saielsr14 = crate::EnumBitfieldStruct<u8, Saielsr14_SPEC>;
    impl Saielsr14 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr15_SPEC;
    pub type Saielsr15 = crate::EnumBitfieldStruct<u8, Saielsr15_SPEC>;
    impl Saielsr15 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr16_SPEC;
    pub type Saielsr16 = crate::EnumBitfieldStruct<u8, Saielsr16_SPEC>;
    impl Saielsr16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr17_SPEC;
    pub type Saielsr17 = crate::EnumBitfieldStruct<u8, Saielsr17_SPEC>;
    impl Saielsr17 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr18_SPEC;
    pub type Saielsr18 = crate::EnumBitfieldStruct<u8, Saielsr18_SPEC>;
    impl Saielsr18 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr19_SPEC;
    pub type Saielsr19 = crate::EnumBitfieldStruct<u8, Saielsr19_SPEC>;
    impl Saielsr19 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr20_SPEC;
    pub type Saielsr20 = crate::EnumBitfieldStruct<u8, Saielsr20_SPEC>;
    impl Saielsr20 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr21_SPEC;
    pub type Saielsr21 = crate::EnumBitfieldStruct<u8, Saielsr21_SPEC>;
    impl Saielsr21 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr22_SPEC;
    pub type Saielsr22 = crate::EnumBitfieldStruct<u8, Saielsr22_SPEC>;
    impl Saielsr22 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr23_SPEC;
    pub type Saielsr23 = crate::EnumBitfieldStruct<u8, Saielsr23_SPEC>;
    impl Saielsr23 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr24_SPEC;
    pub type Saielsr24 = crate::EnumBitfieldStruct<u8, Saielsr24_SPEC>;
    impl Saielsr24 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr25_SPEC;
    pub type Saielsr25 = crate::EnumBitfieldStruct<u8, Saielsr25_SPEC>;
    impl Saielsr25 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr26_SPEC;
    pub type Saielsr26 = crate::EnumBitfieldStruct<u8, Saielsr26_SPEC>;
    impl Saielsr26 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr27_SPEC;
    pub type Saielsr27 = crate::EnumBitfieldStruct<u8, Saielsr27_SPEC>;
    impl Saielsr27 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr28_SPEC;
    pub type Saielsr28 = crate::EnumBitfieldStruct<u8, Saielsr28_SPEC>;
    impl Saielsr28 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr29_SPEC;
    pub type Saielsr29 = crate::EnumBitfieldStruct<u8, Saielsr29_SPEC>;
    impl Saielsr29 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr30_SPEC;
    pub type Saielsr30 = crate::EnumBitfieldStruct<u8, Saielsr30_SPEC>;
    impl Saielsr30 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr31_SPEC;
    pub type Saielsr31 = crate::EnumBitfieldStruct<u8, Saielsr31_SPEC>;
    impl Saielsr31 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusark_SPEC;
impl crate::sealed::RegSpec for Icusark_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Controller Unit Security Attribution Register K"]
pub type Icusark = crate::RegValueT<Icusark_SPEC>;

impl Icusark {
    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr32(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icusark::Saielsr32,
        icusark::Saielsr32,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icusark::Saielsr32,
            icusark::Saielsr32,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr33(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icusark::Saielsr33,
        icusark::Saielsr33,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icusark::Saielsr33,
            icusark::Saielsr33,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr34(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icusark::Saielsr34,
        icusark::Saielsr34,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icusark::Saielsr34,
            icusark::Saielsr34,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr35(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icusark::Saielsr35,
        icusark::Saielsr35,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icusark::Saielsr35,
            icusark::Saielsr35,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr36(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icusark::Saielsr36,
        icusark::Saielsr36,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icusark::Saielsr36,
            icusark::Saielsr36,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr37(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icusark::Saielsr37,
        icusark::Saielsr37,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icusark::Saielsr37,
            icusark::Saielsr37,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr38(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icusark::Saielsr38,
        icusark::Saielsr38,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icusark::Saielsr38,
            icusark::Saielsr38,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr39(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icusark::Saielsr39,
        icusark::Saielsr39,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icusark::Saielsr39,
            icusark::Saielsr39,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr40(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icusark::Saielsr40,
        icusark::Saielsr40,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icusark::Saielsr40,
            icusark::Saielsr40,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr41(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icusark::Saielsr41,
        icusark::Saielsr41,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icusark::Saielsr41,
            icusark::Saielsr41,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr42(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        icusark::Saielsr42,
        icusark::Saielsr42,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            icusark::Saielsr42,
            icusark::Saielsr42,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr43(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        icusark::Saielsr43,
        icusark::Saielsr43,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            icusark::Saielsr43,
            icusark::Saielsr43,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr44(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        icusark::Saielsr44,
        icusark::Saielsr44,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            icusark::Saielsr44,
            icusark::Saielsr44,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr45(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icusark::Saielsr45,
        icusark::Saielsr45,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icusark::Saielsr45,
            icusark::Saielsr45,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr46(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        icusark::Saielsr46,
        icusark::Saielsr46,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            icusark::Saielsr46,
            icusark::Saielsr46,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr47(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        icusark::Saielsr47,
        icusark::Saielsr47,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            icusark::Saielsr47,
            icusark::Saielsr47,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr48(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icusark::Saielsr48,
        icusark::Saielsr48,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icusark::Saielsr48,
            icusark::Saielsr48,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr49(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        icusark::Saielsr49,
        icusark::Saielsr49,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            icusark::Saielsr49,
            icusark::Saielsr49,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr50(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icusark::Saielsr50,
        icusark::Saielsr50,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icusark::Saielsr50,
            icusark::Saielsr50,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr51(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        icusark::Saielsr51,
        icusark::Saielsr51,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            icusark::Saielsr51,
            icusark::Saielsr51,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr52(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        icusark::Saielsr52,
        icusark::Saielsr52,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            icusark::Saielsr52,
            icusark::Saielsr52,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr53(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        icusark::Saielsr53,
        icusark::Saielsr53,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            icusark::Saielsr53,
            icusark::Saielsr53,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr54(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        icusark::Saielsr54,
        icusark::Saielsr54,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            icusark::Saielsr54,
            icusark::Saielsr54,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr55(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        icusark::Saielsr55,
        icusark::Saielsr55,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            icusark::Saielsr55,
            icusark::Saielsr55,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr56(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        icusark::Saielsr56,
        icusark::Saielsr56,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            icusark::Saielsr56,
            icusark::Saielsr56,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr57(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        icusark::Saielsr57,
        icusark::Saielsr57,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            icusark::Saielsr57,
            icusark::Saielsr57,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr58(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        icusark::Saielsr58,
        icusark::Saielsr58,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            icusark::Saielsr58,
            icusark::Saielsr58,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr59(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        icusark::Saielsr59,
        icusark::Saielsr59,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            icusark::Saielsr59,
            icusark::Saielsr59,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr60(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        icusark::Saielsr60,
        icusark::Saielsr60,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            icusark::Saielsr60,
            icusark::Saielsr60,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr61(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        icusark::Saielsr61,
        icusark::Saielsr61,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            icusark::Saielsr61,
            icusark::Saielsr61,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr62(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        icusark::Saielsr62,
        icusark::Saielsr62,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            icusark::Saielsr62,
            icusark::Saielsr62,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting1"]
    #[inline(always)]
    pub fn saielsr63(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        icusark::Saielsr63,
        icusark::Saielsr63,
        Icusark_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            icusark::Saielsr63,
            icusark::Saielsr63,
            Icusark_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icusark {
    #[inline(always)]
    fn default() -> Icusark {
        <crate::RegValueT<Icusark_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusark {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr32_SPEC;
    pub type Saielsr32 = crate::EnumBitfieldStruct<u8, Saielsr32_SPEC>;
    impl Saielsr32 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr33_SPEC;
    pub type Saielsr33 = crate::EnumBitfieldStruct<u8, Saielsr33_SPEC>;
    impl Saielsr33 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr34_SPEC;
    pub type Saielsr34 = crate::EnumBitfieldStruct<u8, Saielsr34_SPEC>;
    impl Saielsr34 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr35_SPEC;
    pub type Saielsr35 = crate::EnumBitfieldStruct<u8, Saielsr35_SPEC>;
    impl Saielsr35 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr36_SPEC;
    pub type Saielsr36 = crate::EnumBitfieldStruct<u8, Saielsr36_SPEC>;
    impl Saielsr36 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr37_SPEC;
    pub type Saielsr37 = crate::EnumBitfieldStruct<u8, Saielsr37_SPEC>;
    impl Saielsr37 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr38_SPEC;
    pub type Saielsr38 = crate::EnumBitfieldStruct<u8, Saielsr38_SPEC>;
    impl Saielsr38 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr39_SPEC;
    pub type Saielsr39 = crate::EnumBitfieldStruct<u8, Saielsr39_SPEC>;
    impl Saielsr39 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr40_SPEC;
    pub type Saielsr40 = crate::EnumBitfieldStruct<u8, Saielsr40_SPEC>;
    impl Saielsr40 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr41_SPEC;
    pub type Saielsr41 = crate::EnumBitfieldStruct<u8, Saielsr41_SPEC>;
    impl Saielsr41 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr42_SPEC;
    pub type Saielsr42 = crate::EnumBitfieldStruct<u8, Saielsr42_SPEC>;
    impl Saielsr42 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr43_SPEC;
    pub type Saielsr43 = crate::EnumBitfieldStruct<u8, Saielsr43_SPEC>;
    impl Saielsr43 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr44_SPEC;
    pub type Saielsr44 = crate::EnumBitfieldStruct<u8, Saielsr44_SPEC>;
    impl Saielsr44 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr45_SPEC;
    pub type Saielsr45 = crate::EnumBitfieldStruct<u8, Saielsr45_SPEC>;
    impl Saielsr45 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr46_SPEC;
    pub type Saielsr46 = crate::EnumBitfieldStruct<u8, Saielsr46_SPEC>;
    impl Saielsr46 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr47_SPEC;
    pub type Saielsr47 = crate::EnumBitfieldStruct<u8, Saielsr47_SPEC>;
    impl Saielsr47 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr48_SPEC;
    pub type Saielsr48 = crate::EnumBitfieldStruct<u8, Saielsr48_SPEC>;
    impl Saielsr48 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr49_SPEC;
    pub type Saielsr49 = crate::EnumBitfieldStruct<u8, Saielsr49_SPEC>;
    impl Saielsr49 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr50_SPEC;
    pub type Saielsr50 = crate::EnumBitfieldStruct<u8, Saielsr50_SPEC>;
    impl Saielsr50 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr51_SPEC;
    pub type Saielsr51 = crate::EnumBitfieldStruct<u8, Saielsr51_SPEC>;
    impl Saielsr51 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr52_SPEC;
    pub type Saielsr52 = crate::EnumBitfieldStruct<u8, Saielsr52_SPEC>;
    impl Saielsr52 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr53_SPEC;
    pub type Saielsr53 = crate::EnumBitfieldStruct<u8, Saielsr53_SPEC>;
    impl Saielsr53 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr54_SPEC;
    pub type Saielsr54 = crate::EnumBitfieldStruct<u8, Saielsr54_SPEC>;
    impl Saielsr54 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr55_SPEC;
    pub type Saielsr55 = crate::EnumBitfieldStruct<u8, Saielsr55_SPEC>;
    impl Saielsr55 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr56_SPEC;
    pub type Saielsr56 = crate::EnumBitfieldStruct<u8, Saielsr56_SPEC>;
    impl Saielsr56 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr57_SPEC;
    pub type Saielsr57 = crate::EnumBitfieldStruct<u8, Saielsr57_SPEC>;
    impl Saielsr57 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr58_SPEC;
    pub type Saielsr58 = crate::EnumBitfieldStruct<u8, Saielsr58_SPEC>;
    impl Saielsr58 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr59_SPEC;
    pub type Saielsr59 = crate::EnumBitfieldStruct<u8, Saielsr59_SPEC>;
    impl Saielsr59 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr60_SPEC;
    pub type Saielsr60 = crate::EnumBitfieldStruct<u8, Saielsr60_SPEC>;
    impl Saielsr60 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr61_SPEC;
    pub type Saielsr61 = crate::EnumBitfieldStruct<u8, Saielsr61_SPEC>;
    impl Saielsr61 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr62_SPEC;
    pub type Saielsr62 = crate::EnumBitfieldStruct<u8, Saielsr62_SPEC>;
    impl Saielsr62 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr63_SPEC;
    pub type Saielsr63 = crate::EnumBitfieldStruct<u8, Saielsr63_SPEC>;
    impl Saielsr63 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusarl_SPEC;
impl crate::sealed::RegSpec for Icusarl_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Controller Unit Security Attribution Register L"]
pub type Icusarl = crate::RegValueT<Icusarl_SPEC>;

impl Icusarl {
    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr64(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icusarl::Saielsr64,
        icusarl::Saielsr64,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icusarl::Saielsr64,
            icusarl::Saielsr64,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr65(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icusarl::Saielsr65,
        icusarl::Saielsr65,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icusarl::Saielsr65,
            icusarl::Saielsr65,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr66(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icusarl::Saielsr66,
        icusarl::Saielsr66,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icusarl::Saielsr66,
            icusarl::Saielsr66,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr67(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icusarl::Saielsr67,
        icusarl::Saielsr67,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icusarl::Saielsr67,
            icusarl::Saielsr67,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr68(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icusarl::Saielsr68,
        icusarl::Saielsr68,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icusarl::Saielsr68,
            icusarl::Saielsr68,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr69(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icusarl::Saielsr69,
        icusarl::Saielsr69,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icusarl::Saielsr69,
            icusarl::Saielsr69,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr70(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icusarl::Saielsr70,
        icusarl::Saielsr70,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icusarl::Saielsr70,
            icusarl::Saielsr70,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr71(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icusarl::Saielsr71,
        icusarl::Saielsr71,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icusarl::Saielsr71,
            icusarl::Saielsr71,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr72(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icusarl::Saielsr72,
        icusarl::Saielsr72,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icusarl::Saielsr72,
            icusarl::Saielsr72,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr73(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icusarl::Saielsr73,
        icusarl::Saielsr73,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icusarl::Saielsr73,
            icusarl::Saielsr73,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr74(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        icusarl::Saielsr74,
        icusarl::Saielsr74,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            icusarl::Saielsr74,
            icusarl::Saielsr74,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr75(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        icusarl::Saielsr75,
        icusarl::Saielsr75,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            icusarl::Saielsr75,
            icusarl::Saielsr75,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr76(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        icusarl::Saielsr76,
        icusarl::Saielsr76,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            icusarl::Saielsr76,
            icusarl::Saielsr76,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr77(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icusarl::Saielsr77,
        icusarl::Saielsr77,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icusarl::Saielsr77,
            icusarl::Saielsr77,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr78(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        icusarl::Saielsr78,
        icusarl::Saielsr78,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            icusarl::Saielsr78,
            icusarl::Saielsr78,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr79(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        icusarl::Saielsr79,
        icusarl::Saielsr79,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            icusarl::Saielsr79,
            icusarl::Saielsr79,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr80(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icusarl::Saielsr80,
        icusarl::Saielsr80,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icusarl::Saielsr80,
            icusarl::Saielsr80,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr81(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        icusarl::Saielsr81,
        icusarl::Saielsr81,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            icusarl::Saielsr81,
            icusarl::Saielsr81,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr82(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icusarl::Saielsr82,
        icusarl::Saielsr82,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icusarl::Saielsr82,
            icusarl::Saielsr82,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr83(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        icusarl::Saielsr83,
        icusarl::Saielsr83,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            icusarl::Saielsr83,
            icusarl::Saielsr83,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr84(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        icusarl::Saielsr84,
        icusarl::Saielsr84,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            icusarl::Saielsr84,
            icusarl::Saielsr84,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr85(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        icusarl::Saielsr85,
        icusarl::Saielsr85,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            icusarl::Saielsr85,
            icusarl::Saielsr85,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr86(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        icusarl::Saielsr86,
        icusarl::Saielsr86,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            icusarl::Saielsr86,
            icusarl::Saielsr86,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr87(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        icusarl::Saielsr87,
        icusarl::Saielsr87,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            icusarl::Saielsr87,
            icusarl::Saielsr87,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr88(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        icusarl::Saielsr88,
        icusarl::Saielsr88,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            icusarl::Saielsr88,
            icusarl::Saielsr88,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr89(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        icusarl::Saielsr89,
        icusarl::Saielsr89,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            icusarl::Saielsr89,
            icusarl::Saielsr89,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr90(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        icusarl::Saielsr90,
        icusarl::Saielsr90,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            icusarl::Saielsr90,
            icusarl::Saielsr90,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr91(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        icusarl::Saielsr91,
        icusarl::Saielsr91,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            icusarl::Saielsr91,
            icusarl::Saielsr91,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr92(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        icusarl::Saielsr92,
        icusarl::Saielsr92,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            icusarl::Saielsr92,
            icusarl::Saielsr92,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr93(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        icusarl::Saielsr93,
        icusarl::Saielsr93,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            icusarl::Saielsr93,
            icusarl::Saielsr93,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr94(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        icusarl::Saielsr94,
        icusarl::Saielsr94,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            icusarl::Saielsr94,
            icusarl::Saielsr94,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for ICU1 event link setting2."]
    #[inline(always)]
    pub fn saielsr95(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        icusarl::Saielsr95,
        icusarl::Saielsr95,
        Icusarl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            icusarl::Saielsr95,
            icusarl::Saielsr95,
            Icusarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icusarl {
    #[inline(always)]
    fn default() -> Icusarl {
        <crate::RegValueT<Icusarl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusarl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr64_SPEC;
    pub type Saielsr64 = crate::EnumBitfieldStruct<u8, Saielsr64_SPEC>;
    impl Saielsr64 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr65_SPEC;
    pub type Saielsr65 = crate::EnumBitfieldStruct<u8, Saielsr65_SPEC>;
    impl Saielsr65 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr66_SPEC;
    pub type Saielsr66 = crate::EnumBitfieldStruct<u8, Saielsr66_SPEC>;
    impl Saielsr66 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr67_SPEC;
    pub type Saielsr67 = crate::EnumBitfieldStruct<u8, Saielsr67_SPEC>;
    impl Saielsr67 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr68_SPEC;
    pub type Saielsr68 = crate::EnumBitfieldStruct<u8, Saielsr68_SPEC>;
    impl Saielsr68 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr69_SPEC;
    pub type Saielsr69 = crate::EnumBitfieldStruct<u8, Saielsr69_SPEC>;
    impl Saielsr69 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr70_SPEC;
    pub type Saielsr70 = crate::EnumBitfieldStruct<u8, Saielsr70_SPEC>;
    impl Saielsr70 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr71_SPEC;
    pub type Saielsr71 = crate::EnumBitfieldStruct<u8, Saielsr71_SPEC>;
    impl Saielsr71 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr72_SPEC;
    pub type Saielsr72 = crate::EnumBitfieldStruct<u8, Saielsr72_SPEC>;
    impl Saielsr72 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr73_SPEC;
    pub type Saielsr73 = crate::EnumBitfieldStruct<u8, Saielsr73_SPEC>;
    impl Saielsr73 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr74_SPEC;
    pub type Saielsr74 = crate::EnumBitfieldStruct<u8, Saielsr74_SPEC>;
    impl Saielsr74 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr75_SPEC;
    pub type Saielsr75 = crate::EnumBitfieldStruct<u8, Saielsr75_SPEC>;
    impl Saielsr75 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr76_SPEC;
    pub type Saielsr76 = crate::EnumBitfieldStruct<u8, Saielsr76_SPEC>;
    impl Saielsr76 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr77_SPEC;
    pub type Saielsr77 = crate::EnumBitfieldStruct<u8, Saielsr77_SPEC>;
    impl Saielsr77 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr78_SPEC;
    pub type Saielsr78 = crate::EnumBitfieldStruct<u8, Saielsr78_SPEC>;
    impl Saielsr78 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr79_SPEC;
    pub type Saielsr79 = crate::EnumBitfieldStruct<u8, Saielsr79_SPEC>;
    impl Saielsr79 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr80_SPEC;
    pub type Saielsr80 = crate::EnumBitfieldStruct<u8, Saielsr80_SPEC>;
    impl Saielsr80 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr81_SPEC;
    pub type Saielsr81 = crate::EnumBitfieldStruct<u8, Saielsr81_SPEC>;
    impl Saielsr81 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr82_SPEC;
    pub type Saielsr82 = crate::EnumBitfieldStruct<u8, Saielsr82_SPEC>;
    impl Saielsr82 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr83_SPEC;
    pub type Saielsr83 = crate::EnumBitfieldStruct<u8, Saielsr83_SPEC>;
    impl Saielsr83 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr84_SPEC;
    pub type Saielsr84 = crate::EnumBitfieldStruct<u8, Saielsr84_SPEC>;
    impl Saielsr84 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr85_SPEC;
    pub type Saielsr85 = crate::EnumBitfieldStruct<u8, Saielsr85_SPEC>;
    impl Saielsr85 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr86_SPEC;
    pub type Saielsr86 = crate::EnumBitfieldStruct<u8, Saielsr86_SPEC>;
    impl Saielsr86 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr87_SPEC;
    pub type Saielsr87 = crate::EnumBitfieldStruct<u8, Saielsr87_SPEC>;
    impl Saielsr87 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr88_SPEC;
    pub type Saielsr88 = crate::EnumBitfieldStruct<u8, Saielsr88_SPEC>;
    impl Saielsr88 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr89_SPEC;
    pub type Saielsr89 = crate::EnumBitfieldStruct<u8, Saielsr89_SPEC>;
    impl Saielsr89 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr90_SPEC;
    pub type Saielsr90 = crate::EnumBitfieldStruct<u8, Saielsr90_SPEC>;
    impl Saielsr90 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr91_SPEC;
    pub type Saielsr91 = crate::EnumBitfieldStruct<u8, Saielsr91_SPEC>;
    impl Saielsr91 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr92_SPEC;
    pub type Saielsr92 = crate::EnumBitfieldStruct<u8, Saielsr92_SPEC>;
    impl Saielsr92 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr93_SPEC;
    pub type Saielsr93 = crate::EnumBitfieldStruct<u8, Saielsr93_SPEC>;
    impl Saielsr93 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr94_SPEC;
    pub type Saielsr94 = crate::EnumBitfieldStruct<u8, Saielsr94_SPEC>;
    impl Saielsr94 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr95_SPEC;
    pub type Saielsr95 = crate::EnumBitfieldStruct<u8, Saielsr95_SPEC>;
    impl Saielsr95 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussara_SPEC;
impl crate::sealed::RegSpec for Bussara_SPEC {
    type DataType = u32;
}

#[doc = "Bus Security Attribution Register A"]
pub type Bussara = crate::RegValueT<Bussara_SPEC>;

impl Bussara {
    #[doc = "Bus Security Attribution A0"]
    #[inline(always)]
    pub fn bussa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussara::Bussa0,
        bussara::Bussa0,
        Bussara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussara::Bussa0,
            bussara::Bussa0,
            Bussara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussara {
    #[inline(always)]
    fn default() -> Bussara {
        <crate::RegValueT<Bussara_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussara {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bussa0_SPEC;
    pub type Bussa0 = crate::EnumBitfieldStruct<u8, Bussa0_SPEC>;
    impl Bussa0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussarb_SPEC;
impl crate::sealed::RegSpec for Bussarb_SPEC {
    type DataType = u32;
}

#[doc = "Bus Security Attribution Register B"]
pub type Bussarb = crate::RegValueT<Bussarb_SPEC>;

impl Bussarb {
    #[doc = "Bus Security Attribution B0"]
    #[inline(always)]
    pub fn bussb0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussarb::Bussb0,
        bussarb::Bussb0,
        Bussarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussarb::Bussb0,
            bussarb::Bussb0,
            Bussarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussarb {
    #[inline(always)]
    fn default() -> Bussarb {
        <crate::RegValueT<Bussarb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussarb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bussb0_SPEC;
    pub type Bussb0 = crate::EnumBitfieldStruct<u8, Bussb0_SPEC>;
    impl Bussb0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussarc_SPEC;
impl crate::sealed::RegSpec for Bussarc_SPEC {
    type DataType = u32;
}

#[doc = "Bus Security Attribution Register C"]
pub type Bussarc = crate::RegValueT<Bussarc_SPEC>;

impl Bussarc {
    #[doc = "Bus Security Attribution C0"]
    #[inline(always)]
    pub fn bussc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussarc::Bussc0,
        bussarc::Bussc0,
        Bussarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussarc::Bussc0,
            bussarc::Bussc0,
            Bussarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussarc {
    #[inline(always)]
    fn default() -> Bussarc {
        <crate::RegValueT<Bussarc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussarc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bussc0_SPEC;
    pub type Bussc0 = crate::EnumBitfieldStruct<u8, Bussc0_SPEC>;
    impl Bussc0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busparc_SPEC;
impl crate::sealed::RegSpec for Busparc_SPEC {
    type DataType = u32;
}

#[doc = "Bus Privileged Attribution Register C"]
pub type Busparc = crate::RegValueT<Busparc_SPEC>;

impl Busparc {
    #[doc = "External bus controller privilege attribution"]
    #[inline(always)]
    pub fn buspa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busparc::Buspa0,
        busparc::Buspa0,
        Busparc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busparc::Buspa0,
            busparc::Buspa0,
            Busparc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busparc {
    #[inline(always)]
    fn default() -> Busparc {
        <crate::RegValueT<Busparc_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod busparc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Buspa0_SPEC;
    pub type Buspa0 = crate::EnumBitfieldStruct<u8, Buspa0_SPEC>;
    impl Buspa0 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusara_SPEC;
impl crate::sealed::RegSpec for Mmpusara_SPEC {
    type DataType = u32;
}

#[doc = "Master Memory Protection Unit Security Attribution Register A"]
pub type Mmpusara = crate::RegValueT<Mmpusara_SPEC>;

impl Mmpusara {
    #[doc = "MMPUAn Security Attribution (n = 0 to 7)"]
    #[inline(always)]
    pub fn mmpuasa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa0,
        mmpusara::Mmpuasa0,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa0,
            mmpusara::Mmpuasa0,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 0 to 7)"]
    #[inline(always)]
    pub fn mmpuasa1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa1,
        mmpusara::Mmpuasa1,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa1,
            mmpusara::Mmpuasa1,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 0 to 7)"]
    #[inline(always)]
    pub fn mmpuasa2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa2,
        mmpusara::Mmpuasa2,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa2,
            mmpusara::Mmpuasa2,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 0 to 7)"]
    #[inline(always)]
    pub fn mmpuasa3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa3,
        mmpusara::Mmpuasa3,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa3,
            mmpusara::Mmpuasa3,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 0 to 7)"]
    #[inline(always)]
    pub fn mmpuasa4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa4,
        mmpusara::Mmpuasa4,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa4,
            mmpusara::Mmpuasa4,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 0 to 7)"]
    #[inline(always)]
    pub fn mmpuasa5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa5,
        mmpusara::Mmpuasa5,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa5,
            mmpusara::Mmpuasa5,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 0 to 7)"]
    #[inline(always)]
    pub fn mmpuasa6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa6,
        mmpusara::Mmpuasa6,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa6,
            mmpusara::Mmpuasa6,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 0 to 7)"]
    #[inline(always)]
    pub fn mmpuasa7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa7,
        mmpusara::Mmpuasa7,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa7,
            mmpusara::Mmpuasa7,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 16 to 23)"]
    #[inline(always)]
    pub fn mmpuasa16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa16,
        mmpusara::Mmpuasa16,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa16,
            mmpusara::Mmpuasa16,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 16 to 23)"]
    #[inline(always)]
    pub fn mmpuasa17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa17,
        mmpusara::Mmpuasa17,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa17,
            mmpusara::Mmpuasa17,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 16 to 23)"]
    #[inline(always)]
    pub fn mmpuasa18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa18,
        mmpusara::Mmpuasa18,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa18,
            mmpusara::Mmpuasa18,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 16 to 23)"]
    #[inline(always)]
    pub fn mmpuasa19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa19,
        mmpusara::Mmpuasa19,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa19,
            mmpusara::Mmpuasa19,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 16 to 23)"]
    #[inline(always)]
    pub fn mmpuasa20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa20,
        mmpusara::Mmpuasa20,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa20,
            mmpusara::Mmpuasa20,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 16 to 23)"]
    #[inline(always)]
    pub fn mmpuasa21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa21,
        mmpusara::Mmpuasa21,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa21,
            mmpusara::Mmpuasa21,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 16 to 23)"]
    #[inline(always)]
    pub fn mmpuasa22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa22,
        mmpusara::Mmpuasa22,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa22,
            mmpusara::Mmpuasa22,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUAn Security Attribution (n = 16 to 23)"]
    #[inline(always)]
    pub fn mmpuasa23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mmpusara::Mmpuasa23,
        mmpusara::Mmpuasa23,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mmpusara::Mmpuasa23,
            mmpusara::Mmpuasa23,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpusara {
    #[inline(always)]
    fn default() -> Mmpusara {
        <crate::RegValueT<Mmpusara_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpusara {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa0_SPEC;
    pub type Mmpuasa0 = crate::EnumBitfieldStruct<u8, Mmpuasa0_SPEC>;
    impl Mmpuasa0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa1_SPEC;
    pub type Mmpuasa1 = crate::EnumBitfieldStruct<u8, Mmpuasa1_SPEC>;
    impl Mmpuasa1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa2_SPEC;
    pub type Mmpuasa2 = crate::EnumBitfieldStruct<u8, Mmpuasa2_SPEC>;
    impl Mmpuasa2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa3_SPEC;
    pub type Mmpuasa3 = crate::EnumBitfieldStruct<u8, Mmpuasa3_SPEC>;
    impl Mmpuasa3 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa4_SPEC;
    pub type Mmpuasa4 = crate::EnumBitfieldStruct<u8, Mmpuasa4_SPEC>;
    impl Mmpuasa4 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa5_SPEC;
    pub type Mmpuasa5 = crate::EnumBitfieldStruct<u8, Mmpuasa5_SPEC>;
    impl Mmpuasa5 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa6_SPEC;
    pub type Mmpuasa6 = crate::EnumBitfieldStruct<u8, Mmpuasa6_SPEC>;
    impl Mmpuasa6 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa7_SPEC;
    pub type Mmpuasa7 = crate::EnumBitfieldStruct<u8, Mmpuasa7_SPEC>;
    impl Mmpuasa7 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa16_SPEC;
    pub type Mmpuasa16 = crate::EnumBitfieldStruct<u8, Mmpuasa16_SPEC>;
    impl Mmpuasa16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa17_SPEC;
    pub type Mmpuasa17 = crate::EnumBitfieldStruct<u8, Mmpuasa17_SPEC>;
    impl Mmpuasa17 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa18_SPEC;
    pub type Mmpuasa18 = crate::EnumBitfieldStruct<u8, Mmpuasa18_SPEC>;
    impl Mmpuasa18 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa19_SPEC;
    pub type Mmpuasa19 = crate::EnumBitfieldStruct<u8, Mmpuasa19_SPEC>;
    impl Mmpuasa19 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa20_SPEC;
    pub type Mmpuasa20 = crate::EnumBitfieldStruct<u8, Mmpuasa20_SPEC>;
    impl Mmpuasa20 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa21_SPEC;
    pub type Mmpuasa21 = crate::EnumBitfieldStruct<u8, Mmpuasa21_SPEC>;
    impl Mmpuasa21 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa22_SPEC;
    pub type Mmpuasa22 = crate::EnumBitfieldStruct<u8, Mmpuasa22_SPEC>;
    impl Mmpuasa22 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpuasa23_SPEC;
    pub type Mmpuasa23 = crate::EnumBitfieldStruct<u8, Mmpuasa23_SPEC>;
    impl Mmpuasa23 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusarb_SPEC;
impl crate::sealed::RegSpec for Mmpusarb_SPEC {
    type DataType = u32;
}

#[doc = "Master Memory Protection Unit Security Attribution Register B"]
pub type Mmpusarb = crate::RegValueT<Mmpusarb_SPEC>;

impl Mmpusarb {
    #[doc = "MMPUB0 Security Attribution"]
    #[inline(always)]
    pub fn mmpubsa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpusarb::Mmpubsa0,
        mmpusarb::Mmpubsa0,
        Mmpusarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpusarb::Mmpubsa0,
            mmpusarb::Mmpubsa0,
            Mmpusarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUB1 Security Attribution"]
    #[inline(always)]
    pub fn mmpubsa1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpusarb::Mmpubsa1,
        mmpusarb::Mmpubsa1,
        Mmpusarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpusarb::Mmpubsa1,
            mmpusarb::Mmpubsa1,
            Mmpusarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MMPUB8 Security Attribution"]
    #[inline(always)]
    pub fn mmpubsa8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mmpusarb::Mmpubsa8,
        mmpusarb::Mmpubsa8,
        Mmpusarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mmpusarb::Mmpubsa8,
            mmpusarb::Mmpubsa8,
            Mmpusarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpusarb {
    #[inline(always)]
    fn default() -> Mmpusarb {
        <crate::RegValueT<Mmpusarb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpusarb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpubsa0_SPEC;
    pub type Mmpubsa0 = crate::EnumBitfieldStruct<u8, Mmpubsa0_SPEC>;
    impl Mmpubsa0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpubsa1_SPEC;
    pub type Mmpubsa1 = crate::EnumBitfieldStruct<u8, Mmpubsa1_SPEC>;
    impl Mmpubsa1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpubsa8_SPEC;
    pub type Mmpubsa8 = crate::EnumBitfieldStruct<u8, Mmpubsa8_SPEC>;
    impl Mmpubsa8 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpusar_SPEC;
impl crate::sealed::RegSpec for Cpusar_SPEC {
    type DataType = u32;
}

#[doc = "CPU Security Attribution Register"]
pub type Cpusar = crate::RegValueT<Cpusar_SPEC>;

impl Cpusar {
    #[doc = "CPU Security Attribution 0 (CPU0)"]
    #[inline(always)]
    pub fn cpusa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cpusar::Cpusa0,
        cpusar::Cpusa0,
        Cpusar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cpusar::Cpusa0,
            cpusar::Cpusa0,
            Cpusar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU Security Attribution 1 (CPU1)"]
    #[inline(always)]
    pub fn cpusa1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cpusar::Cpusa1,
        cpusar::Cpusa1,
        Cpusar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cpusar::Cpusa1,
            cpusar::Cpusa1,
            Cpusar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cpusar {
    #[inline(always)]
    fn default() -> Cpusar {
        <crate::RegValueT<Cpusar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpusar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpusa0_SPEC;
    pub type Cpusa0 = crate::EnumBitfieldStruct<u8, Cpusa0_SPEC>;
    impl Cpusa0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpusa1_SPEC;
    pub type Cpusa1 = crate::EnumBitfieldStruct<u8, Cpusa1_SPEC>;
    impl Cpusa1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacchsar_SPEC;
impl crate::sealed::RegSpec for Dmacchsar_SPEC {
    type DataType = u32;
}

#[doc = "DMA Channel Security Attribution Register"]
pub type Dmacchsar = crate::RegValueT<Dmacchsar_SPEC>;

impl Dmacchsar {
    #[doc = "Security attributes of output and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn sadmac000(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmacchsar::Sadmac000,
        dmacchsar::Sadmac000,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmacchsar::Sadmac000,
            dmacchsar::Sadmac000,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn sadmac001(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dmacchsar::Sadmac001,
        dmacchsar::Sadmac001,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dmacchsar::Sadmac001,
            dmacchsar::Sadmac001,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn sadmac002(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dmacchsar::Sadmac002,
        dmacchsar::Sadmac002,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dmacchsar::Sadmac002,
            dmacchsar::Sadmac002,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn sadmac003(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dmacchsar::Sadmac003,
        dmacchsar::Sadmac003,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dmacchsar::Sadmac003,
            dmacchsar::Sadmac003,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn sadmac004(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dmacchsar::Sadmac004,
        dmacchsar::Sadmac004,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dmacchsar::Sadmac004,
            dmacchsar::Sadmac004,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn sadmac005(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dmacchsar::Sadmac005,
        dmacchsar::Sadmac005,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dmacchsar::Sadmac005,
            dmacchsar::Sadmac005,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn sadmac006(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dmacchsar::Sadmac006,
        dmacchsar::Sadmac006,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dmacchsar::Sadmac006,
            dmacchsar::Sadmac006,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn sadmac007(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dmacchsar::Sadmac007,
        dmacchsar::Sadmac007,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dmacchsar::Sadmac007,
            dmacchsar::Sadmac007,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn sadmac100(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dmacchsar::Sadmac100,
        dmacchsar::Sadmac100,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dmacchsar::Sadmac100,
            dmacchsar::Sadmac100,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn sadmac101(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        dmacchsar::Sadmac101,
        dmacchsar::Sadmac101,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            dmacchsar::Sadmac101,
            dmacchsar::Sadmac101,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn sadmac102(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        dmacchsar::Sadmac102,
        dmacchsar::Sadmac102,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            dmacchsar::Sadmac102,
            dmacchsar::Sadmac102,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn sadmac103(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        dmacchsar::Sadmac103,
        dmacchsar::Sadmac103,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            dmacchsar::Sadmac103,
            dmacchsar::Sadmac103,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn sadmac104(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        dmacchsar::Sadmac104,
        dmacchsar::Sadmac104,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            dmacchsar::Sadmac104,
            dmacchsar::Sadmac104,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn sadmac105(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        dmacchsar::Sadmac105,
        dmacchsar::Sadmac105,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            dmacchsar::Sadmac105,
            dmacchsar::Sadmac105,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn sadmac106(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        dmacchsar::Sadmac106,
        dmacchsar::Sadmac106,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            dmacchsar::Sadmac106,
            dmacchsar::Sadmac106,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn sadmac107(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        dmacchsar::Sadmac107,
        dmacchsar::Sadmac107,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            dmacchsar::Sadmac107,
            dmacchsar::Sadmac107,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmacchsar {
    #[inline(always)]
    fn default() -> Dmacchsar {
        <crate::RegValueT<Dmacchsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmacchsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac000_SPEC;
    pub type Sadmac000 = crate::EnumBitfieldStruct<u8, Sadmac000_SPEC>;
    impl Sadmac000 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac001_SPEC;
    pub type Sadmac001 = crate::EnumBitfieldStruct<u8, Sadmac001_SPEC>;
    impl Sadmac001 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac002_SPEC;
    pub type Sadmac002 = crate::EnumBitfieldStruct<u8, Sadmac002_SPEC>;
    impl Sadmac002 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac003_SPEC;
    pub type Sadmac003 = crate::EnumBitfieldStruct<u8, Sadmac003_SPEC>;
    impl Sadmac003 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac004_SPEC;
    pub type Sadmac004 = crate::EnumBitfieldStruct<u8, Sadmac004_SPEC>;
    impl Sadmac004 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac005_SPEC;
    pub type Sadmac005 = crate::EnumBitfieldStruct<u8, Sadmac005_SPEC>;
    impl Sadmac005 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac006_SPEC;
    pub type Sadmac006 = crate::EnumBitfieldStruct<u8, Sadmac006_SPEC>;
    impl Sadmac006 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac007_SPEC;
    pub type Sadmac007 = crate::EnumBitfieldStruct<u8, Sadmac007_SPEC>;
    impl Sadmac007 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac100_SPEC;
    pub type Sadmac100 = crate::EnumBitfieldStruct<u8, Sadmac100_SPEC>;
    impl Sadmac100 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac101_SPEC;
    pub type Sadmac101 = crate::EnumBitfieldStruct<u8, Sadmac101_SPEC>;
    impl Sadmac101 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac102_SPEC;
    pub type Sadmac102 = crate::EnumBitfieldStruct<u8, Sadmac102_SPEC>;
    impl Sadmac102 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac103_SPEC;
    pub type Sadmac103 = crate::EnumBitfieldStruct<u8, Sadmac103_SPEC>;
    impl Sadmac103 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac104_SPEC;
    pub type Sadmac104 = crate::EnumBitfieldStruct<u8, Sadmac104_SPEC>;
    impl Sadmac104 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac105_SPEC;
    pub type Sadmac105 = crate::EnumBitfieldStruct<u8, Sadmac105_SPEC>;
    impl Sadmac105 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac106_SPEC;
    pub type Sadmac106 = crate::EnumBitfieldStruct<u8, Sadmac106_SPEC>;
    impl Sadmac106 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac107_SPEC;
    pub type Sadmac107 = crate::EnumBitfieldStruct<u8, Sadmac107_SPEC>;
    impl Sadmac107 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacchpar_SPEC;
impl crate::sealed::RegSpec for Dmacchpar_SPEC {
    type DataType = u32;
}

#[doc = "DMA Channel Privilege Attribution Register"]
pub type Dmacchpar = crate::RegValueT<Dmacchpar_SPEC>;

impl Dmacchpar {
    #[doc = "Privilege attributes of outputs and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn padmac000(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmacchpar::Padmac000,
        dmacchpar::Padmac000,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmacchpar::Padmac000,
            dmacchpar::Padmac000,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn padmac001(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dmacchpar::Padmac001,
        dmacchpar::Padmac001,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dmacchpar::Padmac001,
            dmacchpar::Padmac001,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn padmac002(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dmacchpar::Padmac002,
        dmacchpar::Padmac002,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dmacchpar::Padmac002,
            dmacchpar::Padmac002,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn padmac003(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dmacchpar::Padmac003,
        dmacchpar::Padmac003,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dmacchpar::Padmac003,
            dmacchpar::Padmac003,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn padmac004(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dmacchpar::Padmac004,
        dmacchpar::Padmac004,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dmacchpar::Padmac004,
            dmacchpar::Padmac004,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn padmac005(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dmacchpar::Padmac005,
        dmacchpar::Padmac005,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dmacchpar::Padmac005,
            dmacchpar::Padmac005,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn padmac006(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dmacchpar::Padmac006,
        dmacchpar::Padmac006,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dmacchpar::Padmac006,
            dmacchpar::Padmac006,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC0 channel"]
    #[inline(always)]
    pub fn padmac007(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dmacchpar::Padmac007,
        dmacchpar::Padmac007,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dmacchpar::Padmac007,
            dmacchpar::Padmac007,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn padmac100(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dmacchpar::Padmac100,
        dmacchpar::Padmac100,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dmacchpar::Padmac100,
            dmacchpar::Padmac100,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn padmac101(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        dmacchpar::Padmac101,
        dmacchpar::Padmac101,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            dmacchpar::Padmac101,
            dmacchpar::Padmac101,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn padmac102(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        dmacchpar::Padmac102,
        dmacchpar::Padmac102,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            dmacchpar::Padmac102,
            dmacchpar::Padmac102,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn padmac103(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        dmacchpar::Padmac103,
        dmacchpar::Padmac103,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            dmacchpar::Padmac103,
            dmacchpar::Padmac103,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn padmac104(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        dmacchpar::Padmac104,
        dmacchpar::Padmac104,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            dmacchpar::Padmac104,
            dmacchpar::Padmac104,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn padmac105(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        dmacchpar::Padmac105,
        dmacchpar::Padmac105,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            dmacchpar::Padmac105,
            dmacchpar::Padmac105,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn padmac106(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        dmacchpar::Padmac106,
        dmacchpar::Padmac106,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            dmacchpar::Padmac106,
            dmacchpar::Padmac106,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC1 channel"]
    #[inline(always)]
    pub fn padmac107(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        dmacchpar::Padmac107,
        dmacchpar::Padmac107,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            dmacchpar::Padmac107,
            dmacchpar::Padmac107,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmacchpar {
    #[inline(always)]
    fn default() -> Dmacchpar {
        <crate::RegValueT<Dmacchpar_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dmacchpar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac000_SPEC;
    pub type Padmac000 = crate::EnumBitfieldStruct<u8, Padmac000_SPEC>;
    impl Padmac000 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac001_SPEC;
    pub type Padmac001 = crate::EnumBitfieldStruct<u8, Padmac001_SPEC>;
    impl Padmac001 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac002_SPEC;
    pub type Padmac002 = crate::EnumBitfieldStruct<u8, Padmac002_SPEC>;
    impl Padmac002 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac003_SPEC;
    pub type Padmac003 = crate::EnumBitfieldStruct<u8, Padmac003_SPEC>;
    impl Padmac003 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac004_SPEC;
    pub type Padmac004 = crate::EnumBitfieldStruct<u8, Padmac004_SPEC>;
    impl Padmac004 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac005_SPEC;
    pub type Padmac005 = crate::EnumBitfieldStruct<u8, Padmac005_SPEC>;
    impl Padmac005 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac006_SPEC;
    pub type Padmac006 = crate::EnumBitfieldStruct<u8, Padmac006_SPEC>;
    impl Padmac006 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac007_SPEC;
    pub type Padmac007 = crate::EnumBitfieldStruct<u8, Padmac007_SPEC>;
    impl Padmac007 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac100_SPEC;
    pub type Padmac100 = crate::EnumBitfieldStruct<u8, Padmac100_SPEC>;
    impl Padmac100 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac101_SPEC;
    pub type Padmac101 = crate::EnumBitfieldStruct<u8, Padmac101_SPEC>;
    impl Padmac101 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac102_SPEC;
    pub type Padmac102 = crate::EnumBitfieldStruct<u8, Padmac102_SPEC>;
    impl Padmac102 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac103_SPEC;
    pub type Padmac103 = crate::EnumBitfieldStruct<u8, Padmac103_SPEC>;
    impl Padmac103 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac104_SPEC;
    pub type Padmac104 = crate::EnumBitfieldStruct<u8, Padmac104_SPEC>;
    impl Padmac104 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac105_SPEC;
    pub type Padmac105 = crate::EnumBitfieldStruct<u8, Padmac105_SPEC>;
    impl Padmac105 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac106_SPEC;
    pub type Padmac106 = crate::EnumBitfieldStruct<u8, Padmac106_SPEC>;
    impl Padmac106 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac107_SPEC;
    pub type Padmac107 = crate::EnumBitfieldStruct<u8, Padmac107_SPEC>;
    impl Padmac107 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramsabar_SPEC;
impl crate::sealed::RegSpec for Sramsabar_SPEC {
    type DataType = u32;
}

#[doc = "SRAM Security Attribute Boundary Address Register (n = 0 to 3)"]
pub type Sramsabar = crate::RegValueT<Sramsabar_SPEC>;

impl NoBitfieldReg<Sramsabar_SPEC> for Sramsabar {}
impl ::core::default::Default for Sramsabar {
    #[inline(always)]
    fn default() -> Sramsabar {
        <crate::RegValueT<Sramsabar_SPEC> as RegisterValue<_>>::new(2088960)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cachesar_SPEC;
impl crate::sealed::RegSpec for Cachesar_SPEC {
    type DataType = u32;
}

#[doc = "Cache Security Attribution Register"]
pub type Cachesar = crate::RegValueT<Cachesar_SPEC>;

impl Cachesar {
    #[doc = "Security attributes of registers for cache control"]
    #[inline(always)]
    pub fn cachesa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cachesar::Cachesa,
        cachesar::Cachesa,
        Cachesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cachesar::Cachesa,
            cachesar::Cachesa,
            Cachesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for cache error"]
    #[inline(always)]
    pub fn cacheesa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cachesar::Cacheesa,
        cachesar::Cacheesa,
        Cachesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cachesar::Cacheesa,
            cachesar::Cacheesa,
            Cachesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cachesar {
    #[inline(always)]
    fn default() -> Cachesar {
        <crate::RegValueT<Cachesar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cachesar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cachesa_SPEC;
    pub type Cachesa = crate::EnumBitfieldStruct<u8, Cachesa_SPEC>;
    impl Cachesa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cacheesa_SPEC;
    pub type Cacheesa = crate::EnumBitfieldStruct<u8, Cacheesa_SPEC>;
    impl Cacheesa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramesar_SPEC;
impl crate::sealed::RegSpec for Sramesar_SPEC {
    type DataType = u32;
}

#[doc = "SRAM ECC region Security Attribute Register"]
pub type Sramesar = crate::RegValueT<Sramesar_SPEC>;

impl Sramesar {
    #[doc = "ECC region Security Attribution"]
    #[inline(always)]
    pub fn sramesa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramesar::Sramesa,
        sramesar::Sramesa,
        Sramesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramesar::Sramesa,
            sramesar::Sramesa,
            Sramesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sramesar {
    #[inline(always)]
    fn default() -> Sramesar {
        <crate::RegValueT<Sramesar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramesar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramesa_SPEC;
    pub type Sramesa = crate::EnumBitfieldStruct<u8, Sramesa_SPEC>;
    impl Sramesa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tevtrcr_SPEC;
impl crate::sealed::RegSpec for Tevtrcr_SPEC {
    type DataType = u32;
}

#[doc = "Trusted Event Route Control Register"]
pub type Tevtrcr = crate::RegValueT<Tevtrcr_SPEC>;

impl Tevtrcr {
    #[doc = "Trusted Event Route Control Register for ELC"]
    #[inline(always)]
    pub fn tevte(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tevtrcr::Tevte,
        tevtrcr::Tevte,
        Tevtrcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tevtrcr::Tevte,
            tevtrcr::Tevte,
            Tevtrcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Trusted Event Route Control Register for ICU0"]
    #[inline(always)]
    pub fn tevteicu0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tevtrcr::Tevteicu0,
        tevtrcr::Tevteicu0,
        Tevtrcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tevtrcr::Tevteicu0,
            tevtrcr::Tevteicu0,
            Tevtrcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Trusted Event Route Control Register for ICU1"]
    #[inline(always)]
    pub fn tevteicu1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        tevtrcr::Tevteicu1,
        tevtrcr::Tevteicu1,
        Tevtrcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            tevtrcr::Tevteicu1,
            tevtrcr::Tevteicu1,
            Tevtrcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tevtrcr {
    #[inline(always)]
    fn default() -> Tevtrcr {
        <crate::RegValueT<Tevtrcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tevtrcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tevte_SPEC;
    pub type Tevte = crate::EnumBitfieldStruct<u8, Tevte_SPEC>;
    impl Tevte {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tevteicu0_SPEC;
    pub type Tevteicu0 = crate::EnumBitfieldStruct<u8, Tevteicu0_SPEC>;
    impl Tevteicu0 {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tevteicu1_SPEC;
    pub type Tevteicu1 = crate::EnumBitfieldStruct<u8, Tevteicu1_SPEC>;
    impl Tevteicu1 {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcsar_SPEC;
impl crate::sealed::RegSpec for Ipcsar_SPEC {
    type DataType = u32;
}

#[doc = "IPC Security Attribution Register"]
pub type Ipcsar = crate::RegValueT<Ipcsar_SPEC>;

impl Ipcsar {
    #[doc = "Security attributes of registers for IPCSEMn (n = 0 to 7)"]
    #[inline(always)]
    pub fn saipcsem0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ipcsar::Saipcsem0,
        ipcsar::Saipcsem0,
        Ipcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ipcsar::Saipcsem0,
            ipcsar::Saipcsem0,
            Ipcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for IPCSEMn (n = 8 to 15)"]
    #[inline(always)]
    pub fn saipcsem1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ipcsar::Saipcsem1,
        ipcsar::Saipcsem1,
        Ipcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ipcsar::Saipcsem1,
            ipcsar::Saipcsem1,
            Ipcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of IPC0NMISTA, IPC0NMISET and IPC0NMICLR"]
    #[inline(always)]
    pub fn saipcnmi0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ipcsar::Saipcnmi0,
        ipcsar::Saipcnmi0,
        Ipcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ipcsar::Saipcnmi0,
            ipcsar::Saipcnmi0,
            Ipcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of IPC1NMISTA, IPC1NMISET and IPC1NMICLR"]
    #[inline(always)]
    pub fn saipcnmi1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ipcsar::Saipcnmi1,
        ipcsar::Saipcnmi1,
        Ipcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ipcsar::Saipcnmi1,
            ipcsar::Saipcnmi1,
            Ipcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for IPC0STA0, IPC0ISET0, IPC0TXD0, IPC0RXD0 and IPC0CLR0"]
    #[inline(always)]
    pub fn saipcir0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ipcsar::Saipcir0,
        ipcsar::Saipcir0,
        Ipcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ipcsar::Saipcir0,
            ipcsar::Saipcir0,
            Ipcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for IPC0STA1, IPC0ISET1, IPC0TXD1, IPC0RXD1 and IPC0CLR1"]
    #[inline(always)]
    pub fn saipcir1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ipcsar::Saipcir1,
        ipcsar::Saipcir1,
        Ipcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ipcsar::Saipcir1,
            ipcsar::Saipcir1,
            Ipcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for IPC1STA0, IPC1ISET0, IPC1TXD0, IPC1RXD0 and IPC1CLR0"]
    #[inline(always)]
    pub fn saipcir2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        ipcsar::Saipcir2,
        ipcsar::Saipcir2,
        Ipcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            ipcsar::Saipcir2,
            ipcsar::Saipcir2,
            Ipcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of registers for IPC1STA1, IPC1ISET1, IPC1TXD1, IPC1RXD1 and IPC1CLR1"]
    #[inline(always)]
    pub fn saipcir3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        ipcsar::Saipcir3,
        ipcsar::Saipcir3,
        Ipcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            ipcsar::Saipcir3,
            ipcsar::Saipcir3,
            Ipcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ipcsar {
    #[inline(always)]
    fn default() -> Ipcsar {
        <crate::RegValueT<Ipcsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ipcsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saipcsem0_SPEC;
    pub type Saipcsem0 = crate::EnumBitfieldStruct<u8, Saipcsem0_SPEC>;
    impl Saipcsem0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saipcsem1_SPEC;
    pub type Saipcsem1 = crate::EnumBitfieldStruct<u8, Saipcsem1_SPEC>;
    impl Saipcsem1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saipcnmi0_SPEC;
    pub type Saipcnmi0 = crate::EnumBitfieldStruct<u8, Saipcnmi0_SPEC>;
    impl Saipcnmi0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saipcnmi1_SPEC;
    pub type Saipcnmi1 = crate::EnumBitfieldStruct<u8, Saipcnmi1_SPEC>;
    impl Saipcnmi1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saipcir0_SPEC;
    pub type Saipcir0 = crate::EnumBitfieldStruct<u8, Saipcir0_SPEC>;
    impl Saipcir0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saipcir1_SPEC;
    pub type Saipcir1 = crate::EnumBitfieldStruct<u8, Saipcir1_SPEC>;
    impl Saipcir1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saipcir2_SPEC;
    pub type Saipcir2 = crate::EnumBitfieldStruct<u8, Saipcir2_SPEC>;
    impl Saipcir2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saipcir3_SPEC;
    pub type Saipcir3 = crate::EnumBitfieldStruct<u8, Saipcir3_SPEC>;
    impl Saipcir3 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcpar_SPEC;
impl crate::sealed::RegSpec for Ipcpar_SPEC {
    type DataType = u32;
}

#[doc = "IPC Privileged Attribution Register"]
pub type Ipcpar = crate::RegValueT<Ipcpar_SPEC>;

impl Ipcpar {
    #[doc = "Privileged attributes of registers for IPCSEMn (n = 0 to 7)"]
    #[inline(always)]
    pub fn paipcsem0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ipcpar::Paipcsem0,
        ipcpar::Paipcsem0,
        Ipcpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ipcpar::Paipcsem0,
            ipcpar::Paipcsem0,
            Ipcpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privileged attributes of registers for IPCSEMn (n = 8 to 15)"]
    #[inline(always)]
    pub fn paipcsem1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ipcpar::Paipcsem1,
        ipcpar::Paipcsem1,
        Ipcpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ipcpar::Paipcsem1,
            ipcpar::Paipcsem1,
            Ipcpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privileged attributes of registers for IPC0NMISTA, IPC0NMISET and IPC0NMICLR"]
    #[inline(always)]
    pub fn paipcnmi0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ipcpar::Paipcnmi0,
        ipcpar::Paipcnmi0,
        Ipcpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ipcpar::Paipcnmi0,
            ipcpar::Paipcnmi0,
            Ipcpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privileged attributes of registers for IPC1NMISTA, IPC1NMISET and IPC1NMICLR"]
    #[inline(always)]
    pub fn paipcnmi1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ipcpar::Paipcnmi1,
        ipcpar::Paipcnmi1,
        Ipcpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ipcpar::Paipcnmi1,
            ipcpar::Paipcnmi1,
            Ipcpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privileged attributes of registers for IPC0STA0, IPC0ISET0, IPC0TXD0, IPC0RXD0 and IPC0CLR0"]
    #[inline(always)]
    pub fn paipcir0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ipcpar::Paipcir0,
        ipcpar::Paipcir0,
        Ipcpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ipcpar::Paipcir0,
            ipcpar::Paipcir0,
            Ipcpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privileged attributes of registers for IPC0STA1, IPC0ISET1, IPC0TXD1, IPC0RXD1 and IPC0CLR1"]
    #[inline(always)]
    pub fn paipcir1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ipcpar::Paipcir1,
        ipcpar::Paipcir1,
        Ipcpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ipcpar::Paipcir1,
            ipcpar::Paipcir1,
            Ipcpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privileged attributes of registers for IPC1STA0, IPC1ISET0, IPC1TXD0, IPC1RXD0 and IPC1CLR0"]
    #[inline(always)]
    pub fn paipcir2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        ipcpar::Paipcir2,
        ipcpar::Paipcir2,
        Ipcpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            ipcpar::Paipcir2,
            ipcpar::Paipcir2,
            Ipcpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privileged attributes of registers for IPC1STA1, IPC1ISET1, IPC1TXD1, IPC1RXD1 and IPC1CLR1"]
    #[inline(always)]
    pub fn paipcir3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        ipcpar::Paipcir3,
        ipcpar::Paipcir3,
        Ipcpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            ipcpar::Paipcir3,
            ipcpar::Paipcir3,
            Ipcpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ipcpar {
    #[inline(always)]
    fn default() -> Ipcpar {
        <crate::RegValueT<Ipcpar_SPEC> as RegisterValue<_>>::new(983811)
    }
}
pub mod ipcpar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Paipcsem0_SPEC;
    pub type Paipcsem0 = crate::EnumBitfieldStruct<u8, Paipcsem0_SPEC>;
    impl Paipcsem0 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Paipcsem1_SPEC;
    pub type Paipcsem1 = crate::EnumBitfieldStruct<u8, Paipcsem1_SPEC>;
    impl Paipcsem1 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Paipcnmi0_SPEC;
    pub type Paipcnmi0 = crate::EnumBitfieldStruct<u8, Paipcnmi0_SPEC>;
    impl Paipcnmi0 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Paipcnmi1_SPEC;
    pub type Paipcnmi1 = crate::EnumBitfieldStruct<u8, Paipcnmi1_SPEC>;
    impl Paipcnmi1 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Paipcir0_SPEC;
    pub type Paipcir0 = crate::EnumBitfieldStruct<u8, Paipcir0_SPEC>;
    impl Paipcir0 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Paipcir1_SPEC;
    pub type Paipcir1 = crate::EnumBitfieldStruct<u8, Paipcir1_SPEC>;
    impl Paipcir1 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Paipcir2_SPEC;
    pub type Paipcir2 = crate::EnumBitfieldStruct<u8, Paipcir2_SPEC>;
    impl Paipcir2 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Paipcir3_SPEC;
    pub type Paipcir3 = crate::EnumBitfieldStruct<u8, Paipcir3_SPEC>;
    impl Paipcir3 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
}
