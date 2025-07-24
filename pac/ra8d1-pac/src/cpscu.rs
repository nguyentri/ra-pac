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
// Generated from SVD 1.20.02, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:53:12 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CPU System Security Control Unit"]
unsafe impl ::core::marker::Send for super::Cpscu {}
unsafe impl ::core::marker::Sync for super::Cpscu {}
impl super::Cpscu {
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

    #[doc = "Debug Security Attribution Register"]
    #[inline(always)]
    pub const fn debugsar(
        &self,
    ) -> &'static crate::common::Reg<self::Debugsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debugsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[doc = "DMA channel Security Attribution Register"]
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

    #[doc = "DMA channel Privilege Attribution Register"]
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

    #[doc = "SRAM Security Attribute Boundary Address Register (n = 0, 1)"]
    #[inline(always)]
    pub const fn sramsabar(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sramsabar_SPEC, crate::common::RW>,
        2,
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

    #[doc = "Standby SRAM Security Attribute Boundary Address Register"]
    #[inline(always)]
    pub const fn stbramsabar(
        &self,
    ) -> &'static crate::common::Reg<self::Stbramsabar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stbramsabar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1056usize),
            )
        }
    }

    #[doc = "Standby SRAM Privilege Attribute Boundary Address Register for Secure"]
    #[inline(always)]
    pub const fn stbrampabar_s(
        &self,
    ) -> &'static crate::common::Reg<self::StbrampabarS_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::StbrampabarS_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1172usize),
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

    #[doc = "Standby SRAM Register Security Attribution"]
    #[inline(always)]
    pub fn stbramsa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sramsar::Stbramsa,
        sramsar::Stbramsa,
        Sramsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sramsar::Stbramsa,
            sramsar::Stbramsa,
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

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramsa1_SPEC;
    pub type Sramsa1 = crate::EnumBitfieldStruct<u8, Sramsa1_SPEC>;
    impl Sramsa1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stbramsa_SPEC;
    pub type Stbramsa = crate::EnumBitfieldStruct<u8, Stbramsa_SPEC>;
    impl Stbramsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramwtsa_SPEC;
    pub type Sramwtsa = crate::EnumBitfieldStruct<u8, Sramwtsa_SPEC>;
    impl Sramwtsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
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
    #[doc = "DTC Security Attribution"]
    #[inline(always)]
    pub fn dtcstsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dtcsar::Dtcstsa,
        dtcsar::Dtcstsa,
        Dtcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dtcsar::Dtcstsa,
            dtcsar::Dtcstsa,
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
    pub struct Dtcstsa_SPEC;
    pub type Dtcstsa = crate::EnumBitfieldStruct<u8, Dtcstsa_SPEC>;
    impl Dtcstsa {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure."]
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
    #[doc = "DMAST Security Attribution"]
    #[inline(always)]
    pub fn dmastsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmacsar::Dmastsa,
        dmacsar::Dmastsa,
        Dmacsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmacsar::Dmastsa,
            dmacsar::Dmastsa,
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
    pub struct Dmastsa_SPEC;
    pub type Dmastsa = crate::EnumBitfieldStruct<u8, Dmastsa_SPEC>;
    impl Dmastsa {
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
    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

    #[doc = "Security attributes of registers for the IRQCRn register"]
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

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr01_SPEC;
    pub type Sairqcr01 = crate::EnumBitfieldStruct<u8, Sairqcr01_SPEC>;
    impl Sairqcr01 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr02_SPEC;
    pub type Sairqcr02 = crate::EnumBitfieldStruct<u8, Sairqcr02_SPEC>;
    impl Sairqcr02 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr03_SPEC;
    pub type Sairqcr03 = crate::EnumBitfieldStruct<u8, Sairqcr03_SPEC>;
    impl Sairqcr03 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr04_SPEC;
    pub type Sairqcr04 = crate::EnumBitfieldStruct<u8, Sairqcr04_SPEC>;
    impl Sairqcr04 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr05_SPEC;
    pub type Sairqcr05 = crate::EnumBitfieldStruct<u8, Sairqcr05_SPEC>;
    impl Sairqcr05 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr06_SPEC;
    pub type Sairqcr06 = crate::EnumBitfieldStruct<u8, Sairqcr06_SPEC>;
    impl Sairqcr06 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr07_SPEC;
    pub type Sairqcr07 = crate::EnumBitfieldStruct<u8, Sairqcr07_SPEC>;
    impl Sairqcr07 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr08_SPEC;
    pub type Sairqcr08 = crate::EnumBitfieldStruct<u8, Sairqcr08_SPEC>;
    impl Sairqcr08 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr09_SPEC;
    pub type Sairqcr09 = crate::EnumBitfieldStruct<u8, Sairqcr09_SPEC>;
    impl Sairqcr09 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr10_SPEC;
    pub type Sairqcr10 = crate::EnumBitfieldStruct<u8, Sairqcr10_SPEC>;
    impl Sairqcr10 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr11_SPEC;
    pub type Sairqcr11 = crate::EnumBitfieldStruct<u8, Sairqcr11_SPEC>;
    impl Sairqcr11 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr12_SPEC;
    pub type Sairqcr12 = crate::EnumBitfieldStruct<u8, Sairqcr12_SPEC>;
    impl Sairqcr12 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr13_SPEC;
    pub type Sairqcr13 = crate::EnumBitfieldStruct<u8, Sairqcr13_SPEC>;
    impl Sairqcr13 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr14_SPEC;
    pub type Sairqcr14 = crate::EnumBitfieldStruct<u8, Sairqcr14_SPEC>;
    impl Sairqcr14 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr15_SPEC;
    pub type Sairqcr15 = crate::EnumBitfieldStruct<u8, Sairqcr15_SPEC>;
    impl Sairqcr15 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
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
    #[doc = "Security attributes of registers for nonmaskable interrupt"]
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

        #[doc = "Non-secure"]
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

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sapvd1Wup_SPEC;
    pub type Sapvd1Wup = crate::EnumBitfieldStruct<u8, Sapvd1Wup_SPEC>;
    impl Sapvd1Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sapvd2Wup_SPEC;
    pub type Sapvd2Wup = crate::EnumBitfieldStruct<u8, Sapvd2Wup_SPEC>;
    impl Sapvd2Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Savbattwup_SPEC;
    pub type Savbattwup = crate::EnumBitfieldStruct<u8, Savbattwup_SPEC>;
    impl Savbattwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sartcalmwup_SPEC;
    pub type Sartcalmwup = crate::EnumBitfieldStruct<u8, Sartcalmwup_SPEC>;
    impl Sartcalmwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sartcprdwup_SPEC;
    pub type Sartcprdwup = crate::EnumBitfieldStruct<u8, Sartcprdwup_SPEC>;
    impl Sartcprdwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sausbhswup_SPEC;
    pub type Sausbhswup = crate::EnumBitfieldStruct<u8, Sausbhswup_SPEC>;
    impl Sausbhswup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sausbfs0Wup_SPEC;
    pub type Sausbfs0Wup = crate::EnumBitfieldStruct<u8, Sausbfs0Wup_SPEC>;
    impl Sausbfs0Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saagt1Udwup_SPEC;
    pub type Saagt1Udwup = crate::EnumBitfieldStruct<u8, Saagt1Udwup_SPEC>;
    impl Saagt1Udwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saagt1Cawup_SPEC;
    pub type Saagt1Cawup = crate::EnumBitfieldStruct<u8, Saagt1Cawup_SPEC>;
    impl Saagt1Cawup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saagt1Cbwup_SPEC;
    pub type Saagt1Cbwup = crate::EnumBitfieldStruct<u8, Saagt1Cbwup_SPEC>;
    impl Saagt1Cbwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saiic0Wup_SPEC;
    pub type Saiic0Wup = crate::EnumBitfieldStruct<u8, Saiic0Wup_SPEC>;
    impl Saiic0Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
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

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp0Uwup_SPEC;
    pub type Saulp0Uwup = crate::EnumBitfieldStruct<u8, Saulp0Uwup_SPEC>;
    impl Saulp0Uwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp0Awup_SPEC;
    pub type Saulp0Awup = crate::EnumBitfieldStruct<u8, Saulp0Awup_SPEC>;
    impl Saulp0Awup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp0Bwup_SPEC;
    pub type Saulp0Bwup = crate::EnumBitfieldStruct<u8, Saulp0Bwup_SPEC>;
    impl Saulp0Bwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sai3Cwup_SPEC;
    pub type Sai3Cwup = crate::EnumBitfieldStruct<u8, Sai3Cwup_SPEC>;
    impl Sai3Cwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp1Uwup_SPEC;
    pub type Saulp1Uwup = crate::EnumBitfieldStruct<u8, Saulp1Uwup_SPEC>;
    impl Saulp1Uwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp1Awup_SPEC;
    pub type Saulp1Awup = crate::EnumBitfieldStruct<u8, Saulp1Awup_SPEC>;
    impl Saulp1Awup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp1Bwup_SPEC;
    pub type Saulp1Bwup = crate::EnumBitfieldStruct<u8, Saulp1Bwup_SPEC>;
    impl Saulp1Bwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
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
    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

    #[doc = "Security attributes of registers for IELSR31 to IELSR0"]
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

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr01_SPEC;
    pub type Saielsr01 = crate::EnumBitfieldStruct<u8, Saielsr01_SPEC>;
    impl Saielsr01 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr02_SPEC;
    pub type Saielsr02 = crate::EnumBitfieldStruct<u8, Saielsr02_SPEC>;
    impl Saielsr02 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr03_SPEC;
    pub type Saielsr03 = crate::EnumBitfieldStruct<u8, Saielsr03_SPEC>;
    impl Saielsr03 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr04_SPEC;
    pub type Saielsr04 = crate::EnumBitfieldStruct<u8, Saielsr04_SPEC>;
    impl Saielsr04 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr05_SPEC;
    pub type Saielsr05 = crate::EnumBitfieldStruct<u8, Saielsr05_SPEC>;
    impl Saielsr05 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr06_SPEC;
    pub type Saielsr06 = crate::EnumBitfieldStruct<u8, Saielsr06_SPEC>;
    impl Saielsr06 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr07_SPEC;
    pub type Saielsr07 = crate::EnumBitfieldStruct<u8, Saielsr07_SPEC>;
    impl Saielsr07 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr08_SPEC;
    pub type Saielsr08 = crate::EnumBitfieldStruct<u8, Saielsr08_SPEC>;
    impl Saielsr08 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr09_SPEC;
    pub type Saielsr09 = crate::EnumBitfieldStruct<u8, Saielsr09_SPEC>;
    impl Saielsr09 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr10_SPEC;
    pub type Saielsr10 = crate::EnumBitfieldStruct<u8, Saielsr10_SPEC>;
    impl Saielsr10 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr11_SPEC;
    pub type Saielsr11 = crate::EnumBitfieldStruct<u8, Saielsr11_SPEC>;
    impl Saielsr11 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr12_SPEC;
    pub type Saielsr12 = crate::EnumBitfieldStruct<u8, Saielsr12_SPEC>;
    impl Saielsr12 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr13_SPEC;
    pub type Saielsr13 = crate::EnumBitfieldStruct<u8, Saielsr13_SPEC>;
    impl Saielsr13 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr14_SPEC;
    pub type Saielsr14 = crate::EnumBitfieldStruct<u8, Saielsr14_SPEC>;
    impl Saielsr14 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr15_SPEC;
    pub type Saielsr15 = crate::EnumBitfieldStruct<u8, Saielsr15_SPEC>;
    impl Saielsr15 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr16_SPEC;
    pub type Saielsr16 = crate::EnumBitfieldStruct<u8, Saielsr16_SPEC>;
    impl Saielsr16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr17_SPEC;
    pub type Saielsr17 = crate::EnumBitfieldStruct<u8, Saielsr17_SPEC>;
    impl Saielsr17 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr18_SPEC;
    pub type Saielsr18 = crate::EnumBitfieldStruct<u8, Saielsr18_SPEC>;
    impl Saielsr18 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr19_SPEC;
    pub type Saielsr19 = crate::EnumBitfieldStruct<u8, Saielsr19_SPEC>;
    impl Saielsr19 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr20_SPEC;
    pub type Saielsr20 = crate::EnumBitfieldStruct<u8, Saielsr20_SPEC>;
    impl Saielsr20 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr21_SPEC;
    pub type Saielsr21 = crate::EnumBitfieldStruct<u8, Saielsr21_SPEC>;
    impl Saielsr21 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr22_SPEC;
    pub type Saielsr22 = crate::EnumBitfieldStruct<u8, Saielsr22_SPEC>;
    impl Saielsr22 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr23_SPEC;
    pub type Saielsr23 = crate::EnumBitfieldStruct<u8, Saielsr23_SPEC>;
    impl Saielsr23 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr24_SPEC;
    pub type Saielsr24 = crate::EnumBitfieldStruct<u8, Saielsr24_SPEC>;
    impl Saielsr24 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr25_SPEC;
    pub type Saielsr25 = crate::EnumBitfieldStruct<u8, Saielsr25_SPEC>;
    impl Saielsr25 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr26_SPEC;
    pub type Saielsr26 = crate::EnumBitfieldStruct<u8, Saielsr26_SPEC>;
    impl Saielsr26 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr27_SPEC;
    pub type Saielsr27 = crate::EnumBitfieldStruct<u8, Saielsr27_SPEC>;
    impl Saielsr27 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr28_SPEC;
    pub type Saielsr28 = crate::EnumBitfieldStruct<u8, Saielsr28_SPEC>;
    impl Saielsr28 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr29_SPEC;
    pub type Saielsr29 = crate::EnumBitfieldStruct<u8, Saielsr29_SPEC>;
    impl Saielsr29 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr30_SPEC;
    pub type Saielsr30 = crate::EnumBitfieldStruct<u8, Saielsr30_SPEC>;
    impl Saielsr30 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr31_SPEC;
    pub type Saielsr31 = crate::EnumBitfieldStruct<u8, Saielsr31_SPEC>;
    impl Saielsr31 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
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
    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

    #[doc = "Security attributes of registers for IELSR63 to IELSR32"]
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

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr33_SPEC;
    pub type Saielsr33 = crate::EnumBitfieldStruct<u8, Saielsr33_SPEC>;
    impl Saielsr33 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr34_SPEC;
    pub type Saielsr34 = crate::EnumBitfieldStruct<u8, Saielsr34_SPEC>;
    impl Saielsr34 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr35_SPEC;
    pub type Saielsr35 = crate::EnumBitfieldStruct<u8, Saielsr35_SPEC>;
    impl Saielsr35 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr36_SPEC;
    pub type Saielsr36 = crate::EnumBitfieldStruct<u8, Saielsr36_SPEC>;
    impl Saielsr36 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr37_SPEC;
    pub type Saielsr37 = crate::EnumBitfieldStruct<u8, Saielsr37_SPEC>;
    impl Saielsr37 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr38_SPEC;
    pub type Saielsr38 = crate::EnumBitfieldStruct<u8, Saielsr38_SPEC>;
    impl Saielsr38 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr39_SPEC;
    pub type Saielsr39 = crate::EnumBitfieldStruct<u8, Saielsr39_SPEC>;
    impl Saielsr39 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr40_SPEC;
    pub type Saielsr40 = crate::EnumBitfieldStruct<u8, Saielsr40_SPEC>;
    impl Saielsr40 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr41_SPEC;
    pub type Saielsr41 = crate::EnumBitfieldStruct<u8, Saielsr41_SPEC>;
    impl Saielsr41 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr42_SPEC;
    pub type Saielsr42 = crate::EnumBitfieldStruct<u8, Saielsr42_SPEC>;
    impl Saielsr42 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr43_SPEC;
    pub type Saielsr43 = crate::EnumBitfieldStruct<u8, Saielsr43_SPEC>;
    impl Saielsr43 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr44_SPEC;
    pub type Saielsr44 = crate::EnumBitfieldStruct<u8, Saielsr44_SPEC>;
    impl Saielsr44 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr45_SPEC;
    pub type Saielsr45 = crate::EnumBitfieldStruct<u8, Saielsr45_SPEC>;
    impl Saielsr45 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr46_SPEC;
    pub type Saielsr46 = crate::EnumBitfieldStruct<u8, Saielsr46_SPEC>;
    impl Saielsr46 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr47_SPEC;
    pub type Saielsr47 = crate::EnumBitfieldStruct<u8, Saielsr47_SPEC>;
    impl Saielsr47 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr48_SPEC;
    pub type Saielsr48 = crate::EnumBitfieldStruct<u8, Saielsr48_SPEC>;
    impl Saielsr48 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr49_SPEC;
    pub type Saielsr49 = crate::EnumBitfieldStruct<u8, Saielsr49_SPEC>;
    impl Saielsr49 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr50_SPEC;
    pub type Saielsr50 = crate::EnumBitfieldStruct<u8, Saielsr50_SPEC>;
    impl Saielsr50 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr51_SPEC;
    pub type Saielsr51 = crate::EnumBitfieldStruct<u8, Saielsr51_SPEC>;
    impl Saielsr51 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr52_SPEC;
    pub type Saielsr52 = crate::EnumBitfieldStruct<u8, Saielsr52_SPEC>;
    impl Saielsr52 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr53_SPEC;
    pub type Saielsr53 = crate::EnumBitfieldStruct<u8, Saielsr53_SPEC>;
    impl Saielsr53 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr54_SPEC;
    pub type Saielsr54 = crate::EnumBitfieldStruct<u8, Saielsr54_SPEC>;
    impl Saielsr54 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr55_SPEC;
    pub type Saielsr55 = crate::EnumBitfieldStruct<u8, Saielsr55_SPEC>;
    impl Saielsr55 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr56_SPEC;
    pub type Saielsr56 = crate::EnumBitfieldStruct<u8, Saielsr56_SPEC>;
    impl Saielsr56 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr57_SPEC;
    pub type Saielsr57 = crate::EnumBitfieldStruct<u8, Saielsr57_SPEC>;
    impl Saielsr57 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr58_SPEC;
    pub type Saielsr58 = crate::EnumBitfieldStruct<u8, Saielsr58_SPEC>;
    impl Saielsr58 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr59_SPEC;
    pub type Saielsr59 = crate::EnumBitfieldStruct<u8, Saielsr59_SPEC>;
    impl Saielsr59 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr60_SPEC;
    pub type Saielsr60 = crate::EnumBitfieldStruct<u8, Saielsr60_SPEC>;
    impl Saielsr60 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr61_SPEC;
    pub type Saielsr61 = crate::EnumBitfieldStruct<u8, Saielsr61_SPEC>;
    impl Saielsr61 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr62_SPEC;
    pub type Saielsr62 = crate::EnumBitfieldStruct<u8, Saielsr62_SPEC>;
    impl Saielsr62 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr63_SPEC;
    pub type Saielsr63 = crate::EnumBitfieldStruct<u8, Saielsr63_SPEC>;
    impl Saielsr63 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
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
    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

    #[doc = "Security attributes of registers for IELSR95 to IELSR64"]
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

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr65_SPEC;
    pub type Saielsr65 = crate::EnumBitfieldStruct<u8, Saielsr65_SPEC>;
    impl Saielsr65 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr66_SPEC;
    pub type Saielsr66 = crate::EnumBitfieldStruct<u8, Saielsr66_SPEC>;
    impl Saielsr66 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr67_SPEC;
    pub type Saielsr67 = crate::EnumBitfieldStruct<u8, Saielsr67_SPEC>;
    impl Saielsr67 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr68_SPEC;
    pub type Saielsr68 = crate::EnumBitfieldStruct<u8, Saielsr68_SPEC>;
    impl Saielsr68 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr69_SPEC;
    pub type Saielsr69 = crate::EnumBitfieldStruct<u8, Saielsr69_SPEC>;
    impl Saielsr69 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr70_SPEC;
    pub type Saielsr70 = crate::EnumBitfieldStruct<u8, Saielsr70_SPEC>;
    impl Saielsr70 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr71_SPEC;
    pub type Saielsr71 = crate::EnumBitfieldStruct<u8, Saielsr71_SPEC>;
    impl Saielsr71 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr72_SPEC;
    pub type Saielsr72 = crate::EnumBitfieldStruct<u8, Saielsr72_SPEC>;
    impl Saielsr72 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr73_SPEC;
    pub type Saielsr73 = crate::EnumBitfieldStruct<u8, Saielsr73_SPEC>;
    impl Saielsr73 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr74_SPEC;
    pub type Saielsr74 = crate::EnumBitfieldStruct<u8, Saielsr74_SPEC>;
    impl Saielsr74 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr75_SPEC;
    pub type Saielsr75 = crate::EnumBitfieldStruct<u8, Saielsr75_SPEC>;
    impl Saielsr75 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr76_SPEC;
    pub type Saielsr76 = crate::EnumBitfieldStruct<u8, Saielsr76_SPEC>;
    impl Saielsr76 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr77_SPEC;
    pub type Saielsr77 = crate::EnumBitfieldStruct<u8, Saielsr77_SPEC>;
    impl Saielsr77 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr78_SPEC;
    pub type Saielsr78 = crate::EnumBitfieldStruct<u8, Saielsr78_SPEC>;
    impl Saielsr78 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr79_SPEC;
    pub type Saielsr79 = crate::EnumBitfieldStruct<u8, Saielsr79_SPEC>;
    impl Saielsr79 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr80_SPEC;
    pub type Saielsr80 = crate::EnumBitfieldStruct<u8, Saielsr80_SPEC>;
    impl Saielsr80 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr81_SPEC;
    pub type Saielsr81 = crate::EnumBitfieldStruct<u8, Saielsr81_SPEC>;
    impl Saielsr81 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr82_SPEC;
    pub type Saielsr82 = crate::EnumBitfieldStruct<u8, Saielsr82_SPEC>;
    impl Saielsr82 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr83_SPEC;
    pub type Saielsr83 = crate::EnumBitfieldStruct<u8, Saielsr83_SPEC>;
    impl Saielsr83 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr84_SPEC;
    pub type Saielsr84 = crate::EnumBitfieldStruct<u8, Saielsr84_SPEC>;
    impl Saielsr84 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr85_SPEC;
    pub type Saielsr85 = crate::EnumBitfieldStruct<u8, Saielsr85_SPEC>;
    impl Saielsr85 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr86_SPEC;
    pub type Saielsr86 = crate::EnumBitfieldStruct<u8, Saielsr86_SPEC>;
    impl Saielsr86 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr87_SPEC;
    pub type Saielsr87 = crate::EnumBitfieldStruct<u8, Saielsr87_SPEC>;
    impl Saielsr87 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr88_SPEC;
    pub type Saielsr88 = crate::EnumBitfieldStruct<u8, Saielsr88_SPEC>;
    impl Saielsr88 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr89_SPEC;
    pub type Saielsr89 = crate::EnumBitfieldStruct<u8, Saielsr89_SPEC>;
    impl Saielsr89 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr90_SPEC;
    pub type Saielsr90 = crate::EnumBitfieldStruct<u8, Saielsr90_SPEC>;
    impl Saielsr90 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr91_SPEC;
    pub type Saielsr91 = crate::EnumBitfieldStruct<u8, Saielsr91_SPEC>;
    impl Saielsr91 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr92_SPEC;
    pub type Saielsr92 = crate::EnumBitfieldStruct<u8, Saielsr92_SPEC>;
    impl Saielsr92 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr93_SPEC;
    pub type Saielsr93 = crate::EnumBitfieldStruct<u8, Saielsr93_SPEC>;
    impl Saielsr93 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr94_SPEC;
    pub type Saielsr94 = crate::EnumBitfieldStruct<u8, Saielsr94_SPEC>;
    impl Saielsr94 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr95_SPEC;
    pub type Saielsr95 = crate::EnumBitfieldStruct<u8, Saielsr95_SPEC>;
    impl Saielsr95 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
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
    #[doc = "MMPUA Security Attribution n (n = 0 to 7)"]
    #[inline(always)]
    pub fn mmpuasan(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        mmpusara::MmpuasAn,
        mmpusara::MmpuasAn,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            mmpusara::MmpuasAn,
            mmpusara::MmpuasAn,
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
    pub struct MmpuasAn_SPEC;
    pub type MmpuasAn = crate::EnumBitfieldStruct<u8, MmpuasAn_SPEC>;
    impl MmpuasAn {
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
    #[doc = "MMPUB Security Attribution 0"]
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

    #[doc = "MMPUB Security Attribution 8"]
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
    #[doc = "CPU Control Registers Security Attribution"]
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debugsar_SPEC;
impl crate::sealed::RegSpec for Debugsar_SPEC {
    type DataType = u32;
}

#[doc = "Debug Security Attribution Register"]
pub type Debugsar = crate::RegValueT<Debugsar_SPEC>;

impl Debugsar {
    #[doc = "Debug Resources Security Attribution 0"]
    #[inline(always)]
    pub fn dbgsa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        debugsar::Dbgsa0,
        debugsar::Dbgsa0,
        Debugsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            debugsar::Dbgsa0,
            debugsar::Dbgsa0,
            Debugsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Debugsar {
    #[inline(always)]
    fn default() -> Debugsar {
        <crate::RegValueT<Debugsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod debugsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbgsa0_SPEC;
    pub type Dbgsa0 = crate::EnumBitfieldStruct<u8, Dbgsa0_SPEC>;
    impl Dbgsa0 {
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

#[doc = "DMA channel Security Attribution Register"]
pub type Dmacchsar = crate::RegValueT<Dmacchsar_SPEC>;

impl Dmacchsar {
    #[doc = "Security attributes of output and registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmacchsar::Sadmac0,
        dmacchsar::Sadmac0,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmacchsar::Sadmac0,
            dmacchsar::Sadmac0,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dmacchsar::Sadmac1,
        dmacchsar::Sadmac1,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dmacchsar::Sadmac1,
            dmacchsar::Sadmac1,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dmacchsar::Sadmac2,
        dmacchsar::Sadmac2,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dmacchsar::Sadmac2,
            dmacchsar::Sadmac2,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dmacchsar::Sadmac3,
        dmacchsar::Sadmac3,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dmacchsar::Sadmac3,
            dmacchsar::Sadmac3,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dmacchsar::Sadmac4,
        dmacchsar::Sadmac4,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dmacchsar::Sadmac4,
            dmacchsar::Sadmac4,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dmacchsar::Sadmac5,
        dmacchsar::Sadmac5,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dmacchsar::Sadmac5,
            dmacchsar::Sadmac5,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dmacchsar::Sadmac6,
        dmacchsar::Sadmac6,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dmacchsar::Sadmac6,
            dmacchsar::Sadmac6,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security attributes of output and registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dmacchsar::Sadmac7,
        dmacchsar::Sadmac7,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dmacchsar::Sadmac7,
            dmacchsar::Sadmac7,
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
    pub struct Sadmac0_SPEC;
    pub type Sadmac0 = crate::EnumBitfieldStruct<u8, Sadmac0_SPEC>;
    impl Sadmac0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac1_SPEC;
    pub type Sadmac1 = crate::EnumBitfieldStruct<u8, Sadmac1_SPEC>;
    impl Sadmac1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac2_SPEC;
    pub type Sadmac2 = crate::EnumBitfieldStruct<u8, Sadmac2_SPEC>;
    impl Sadmac2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac3_SPEC;
    pub type Sadmac3 = crate::EnumBitfieldStruct<u8, Sadmac3_SPEC>;
    impl Sadmac3 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac4_SPEC;
    pub type Sadmac4 = crate::EnumBitfieldStruct<u8, Sadmac4_SPEC>;
    impl Sadmac4 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac5_SPEC;
    pub type Sadmac5 = crate::EnumBitfieldStruct<u8, Sadmac5_SPEC>;
    impl Sadmac5 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac6_SPEC;
    pub type Sadmac6 = crate::EnumBitfieldStruct<u8, Sadmac6_SPEC>;
    impl Sadmac6 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac7_SPEC;
    pub type Sadmac7 = crate::EnumBitfieldStruct<u8, Sadmac7_SPEC>;
    impl Sadmac7 {
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

#[doc = "DMA channel Privilege Attribution Register"]
pub type Dmacchpar = crate::RegValueT<Dmacchpar_SPEC>;

impl Dmacchpar {
    #[doc = "Privilege attributes of outputs and registers for DMAC channel"]
    #[inline(always)]
    pub fn padmac0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmacchpar::Padmac0,
        dmacchpar::Padmac0,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmacchpar::Padmac0,
            dmacchpar::Padmac0,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC channel"]
    #[inline(always)]
    pub fn padmac1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dmacchpar::Padmac1,
        dmacchpar::Padmac1,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dmacchpar::Padmac1,
            dmacchpar::Padmac1,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC channel"]
    #[inline(always)]
    pub fn padmac2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dmacchpar::Padmac2,
        dmacchpar::Padmac2,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dmacchpar::Padmac2,
            dmacchpar::Padmac2,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC channel"]
    #[inline(always)]
    pub fn padmac3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dmacchpar::Padmac3,
        dmacchpar::Padmac3,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dmacchpar::Padmac3,
            dmacchpar::Padmac3,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC channel"]
    #[inline(always)]
    pub fn padmac4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dmacchpar::Padmac4,
        dmacchpar::Padmac4,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dmacchpar::Padmac4,
            dmacchpar::Padmac4,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC channel"]
    #[inline(always)]
    pub fn padmac5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dmacchpar::Padmac5,
        dmacchpar::Padmac5,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dmacchpar::Padmac5,
            dmacchpar::Padmac5,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC channel"]
    #[inline(always)]
    pub fn padmac6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dmacchpar::Padmac6,
        dmacchpar::Padmac6,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dmacchpar::Padmac6,
            dmacchpar::Padmac6,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege attributes of outputs and registers for DMAC channel"]
    #[inline(always)]
    pub fn padmac7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dmacchpar::Padmac7,
        dmacchpar::Padmac7,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dmacchpar::Padmac7,
            dmacchpar::Padmac7,
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
    pub struct Padmac0_SPEC;
    pub type Padmac0 = crate::EnumBitfieldStruct<u8, Padmac0_SPEC>;
    impl Padmac0 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac1_SPEC;
    pub type Padmac1 = crate::EnumBitfieldStruct<u8, Padmac1_SPEC>;
    impl Padmac1 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac2_SPEC;
    pub type Padmac2 = crate::EnumBitfieldStruct<u8, Padmac2_SPEC>;
    impl Padmac2 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac3_SPEC;
    pub type Padmac3 = crate::EnumBitfieldStruct<u8, Padmac3_SPEC>;
    impl Padmac3 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac4_SPEC;
    pub type Padmac4 = crate::EnumBitfieldStruct<u8, Padmac4_SPEC>;
    impl Padmac4 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac5_SPEC;
    pub type Padmac5 = crate::EnumBitfieldStruct<u8, Padmac5_SPEC>;
    impl Padmac5 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac6_SPEC;
    pub type Padmac6 = crate::EnumBitfieldStruct<u8, Padmac6_SPEC>;
    impl Padmac6 {
        #[doc = "Privileged."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac7_SPEC;
    pub type Padmac7 = crate::EnumBitfieldStruct<u8, Padmac7_SPEC>;
    impl Padmac7 {
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

#[doc = "SRAM Security Attribute Boundary Address Register (n = 0, 1)"]
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
pub struct Stbramsabar_SPEC;
impl crate::sealed::RegSpec for Stbramsabar_SPEC {
    type DataType = u32;
}

#[doc = "Standby SRAM Security Attribute Boundary Address Register"]
pub type Stbramsabar = crate::RegValueT<Stbramsabar_SPEC>;

impl NoBitfieldReg<Stbramsabar_SPEC> for Stbramsabar {}
impl ::core::default::Default for Stbramsabar {
    #[inline(always)]
    fn default() -> Stbramsabar {
        <crate::RegValueT<Stbramsabar_SPEC> as RegisterValue<_>>::new(32640)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StbrampabarS_SPEC;
impl crate::sealed::RegSpec for StbrampabarS_SPEC {
    type DataType = u32;
}

#[doc = "Standby SRAM Privilege Attribute Boundary Address Register for Secure"]
pub type StbrampabarS = crate::RegValueT<StbrampabarS_SPEC>;

impl NoBitfieldReg<StbrampabarS_SPEC> for StbrampabarS {}
impl ::core::default::Default for StbrampabarS {
    #[inline(always)]
    fn default() -> StbrampabarS {
        <crate::RegValueT<StbrampabarS_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Trusted Event Route Control Register for IELSRn, DELSRn and ELCSRn"]
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
}
