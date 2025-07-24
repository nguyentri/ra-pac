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
// Generated from SVD 1.10.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:59 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Interrupt Controller"]
unsafe impl ::core::marker::Send for super::Icu {}
unsafe impl ::core::marker::Sync for super::Icu {}
impl super::Icu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "IRQ Control Register %s"]
    #[inline(always)]
    pub const fn irqcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Irqcr_SPEC, crate::common::RW>,
        6,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub const fn irqcr0(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr1(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr2(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr3(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr4(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr5(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5usize),
            )
        }
    }

    #[doc = "NMI Pin Interrupt Control Register"]
    #[inline(always)]
    pub const fn nmicr(&self) -> &'static crate::common::Reg<self::Nmicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nmicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Non-maskable Interrupt Enable Register"]
    #[inline(always)]
    pub const fn nmier(&self) -> &'static crate::common::Reg<self::Nmier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nmier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "Non-maskable Interrupt Status Clear Register"]
    #[inline(always)]
    pub const fn nmiclr(
        &self,
    ) -> &'static crate::common::Reg<self::Nmiclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nmiclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[doc = "Non-maskable Interrupt Status Register"]
    #[inline(always)]
    pub const fn nmisr(&self) -> &'static crate::common::Reg<self::Nmisr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Nmisr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[doc = "DTC Enable Status Register 0"]
    #[inline(always)]
    pub const fn dtcenst0(
        &self,
    ) -> &'static crate::common::Reg<self::Dtcenst0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dtcenst0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[doc = "DTC Enable Status Register 1"]
    #[inline(always)]
    pub const fn dtcenst1(
        &self,
    ) -> &'static crate::common::Reg<self::Dtcenst1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dtcenst1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(772usize),
            )
        }
    }

    #[doc = "DTC Enable Set Register 0"]
    #[inline(always)]
    pub const fn dtcenset0(
        &self,
    ) -> &'static crate::common::Reg<self::Dtcenset0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dtcenset0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(784usize),
            )
        }
    }

    #[doc = "DTC Enable Set Register 1"]
    #[inline(always)]
    pub const fn dtcenset1(
        &self,
    ) -> &'static crate::common::Reg<self::Dtcenset1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dtcenset1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(788usize),
            )
        }
    }

    #[doc = "DTC Enable Clear Register 0"]
    #[inline(always)]
    pub const fn dtcenclr0(
        &self,
    ) -> &'static crate::common::Reg<self::Dtcenclr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dtcenclr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(800usize),
            )
        }
    }

    #[doc = "DTC Enable Clear Register 1"]
    #[inline(always)]
    pub const fn dtcenclr1(
        &self,
    ) -> &'static crate::common::Reg<self::Dtcenclr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dtcenclr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(804usize),
            )
        }
    }

    #[doc = "Interrupt Request Flag Monitor Register 0"]
    #[inline(always)]
    pub const fn intflag0(
        &self,
    ) -> &'static crate::common::Reg<self::Intflag0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Intflag0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(816usize),
            )
        }
    }

    #[doc = "Interrupt Request Flag Monitor Register 1"]
    #[inline(always)]
    pub const fn intflag1(
        &self,
    ) -> &'static crate::common::Reg<self::Intflag1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Intflag1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(820usize),
            )
        }
    }

    #[doc = "Software Standby/Snooze End Control Register 0"]
    #[inline(always)]
    pub const fn sbyedcr0(
        &self,
    ) -> &'static crate::common::Reg<self::Sbyedcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sbyedcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(832usize),
            )
        }
    }

    #[doc = "Software Standby/Snooze End Control Register 1"]
    #[inline(always)]
    pub const fn sbyedcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Sbyedcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sbyedcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(836usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqcr_SPEC;
impl crate::sealed::RegSpec for Irqcr_SPEC {
    type DataType = u8;
}

#[doc = "IRQ Control Register %s"]
pub type Irqcr = crate::RegValueT<Irqcr_SPEC>;

impl Irqcr {
    #[doc = "IRQi Detection Sense Select"]
    #[inline(always)]
    pub fn irqmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        irqcr::Irqmd,
        irqcr::Irqmd,
        Irqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            irqcr::Irqmd,
            irqcr::Irqmd,
            Irqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Irqcr {
    #[inline(always)]
    fn default() -> Irqcr {
        <crate::RegValueT<Irqcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod irqcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqmd_SPEC;
    pub type Irqmd = crate::EnumBitfieldStruct<u8, Irqmd_SPEC>;
    impl Irqmd {
        #[doc = "Falling edge"]
        pub const _00: Self = Self::new(0);

        #[doc = "Rising edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Rising and falling edges"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmicr_SPEC;
impl crate::sealed::RegSpec for Nmicr_SPEC {
    type DataType = u8;
}

#[doc = "NMI Pin Interrupt Control Register"]
pub type Nmicr = crate::RegValueT<Nmicr_SPEC>;

impl Nmicr {
    #[doc = "NMI Detection Set"]
    #[inline(always)]
    pub fn nmimd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        nmicr::Nmimd,
        nmicr::Nmimd,
        Nmicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            nmicr::Nmimd,
            nmicr::Nmimd,
            Nmicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nmicr {
    #[inline(always)]
    fn default() -> Nmicr {
        <crate::RegValueT<Nmicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmimd_SPEC;
    pub type Nmimd = crate::EnumBitfieldStruct<u8, Nmimd_SPEC>;
    impl Nmimd {
        #[doc = "Falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edge"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmier_SPEC;
impl crate::sealed::RegSpec for Nmier_SPEC {
    type DataType = u16;
}

#[doc = "Non-maskable Interrupt Enable Register"]
pub type Nmier = crate::RegValueT<Nmier_SPEC>;

impl Nmier {
    #[doc = "IWDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    pub fn iwdten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        nmier::Iwdten,
        nmier::Iwdten,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            nmier::Iwdten,
            nmier::Iwdten,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Interrupt Enable"]
    #[inline(always)]
    pub fn lvd1en(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        nmier::Lvd1En,
        nmier::Lvd1En,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            nmier::Lvd1En,
            nmier::Lvd1En,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NMI Pin Interrupt Enable"]
    #[inline(always)]
    pub fn nmien(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        nmier::Nmien,
        nmier::Nmien,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            nmier::Nmien,
            nmier::Nmien,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SRAM Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn rpeen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        nmier::Rpeen,
        nmier::Rpeen,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            nmier::Rpeen,
            nmier::Rpeen,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nmier {
    #[inline(always)]
    fn default() -> Nmier {
        <crate::RegValueT<Nmier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdten_SPEC;
    pub type Iwdten = crate::EnumBitfieldStruct<u8, Iwdten_SPEC>;
    impl Iwdten {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1En_SPEC;
    pub type Lvd1En = crate::EnumBitfieldStruct<u8, Lvd1En_SPEC>;
    impl Lvd1En {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmien_SPEC;
    pub type Nmien = crate::EnumBitfieldStruct<u8, Nmien_SPEC>;
    impl Nmien {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpeen_SPEC;
    pub type Rpeen = crate::EnumBitfieldStruct<u8, Rpeen_SPEC>;
    impl Rpeen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmiclr_SPEC;
impl crate::sealed::RegSpec for Nmiclr_SPEC {
    type DataType = u16;
}

#[doc = "Non-maskable Interrupt Status Clear Register"]
pub type Nmiclr = crate::RegValueT<Nmiclr_SPEC>;

impl Nmiclr {
    #[doc = "IWDT Underflow/Refresh Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn iwdtclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        nmiclr::Iwdtclr,
        nmiclr::Iwdtclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            nmiclr::Iwdtclr,
            nmiclr::Iwdtclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn lvd1clr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        nmiclr::Lvd1Clr,
        nmiclr::Lvd1Clr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            nmiclr::Lvd1Clr,
            nmiclr::Lvd1Clr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NMI Pin Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn nmiclr(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        nmiclr::Nmiclr,
        nmiclr::Nmiclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            nmiclr::Nmiclr,
            nmiclr::Nmiclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SRAM Parity Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn rpeclr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        nmiclr::Rpeclr,
        nmiclr::Rpeclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            nmiclr::Rpeclr,
            nmiclr::Rpeclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nmiclr {
    #[inline(always)]
    fn default() -> Nmiclr {
        <crate::RegValueT<Nmiclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmiclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtclr_SPEC;
    pub type Iwdtclr = crate::EnumBitfieldStruct<u8, Iwdtclr_SPEC>;
    impl Iwdtclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.IWDTST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Clr_SPEC;
    pub type Lvd1Clr = crate::EnumBitfieldStruct<u8, Lvd1Clr_SPEC>;
    impl Lvd1Clr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.LVD1ST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmiclr_SPEC;
    pub type Nmiclr = crate::EnumBitfieldStruct<u8, Nmiclr_SPEC>;
    impl Nmiclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.NMIST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpeclr_SPEC;
    pub type Rpeclr = crate::EnumBitfieldStruct<u8, Rpeclr_SPEC>;
    impl Rpeclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.RPEST flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmisr_SPEC;
impl crate::sealed::RegSpec for Nmisr_SPEC {
    type DataType = u16;
}

#[doc = "Non-maskable Interrupt Status Register"]
pub type Nmisr = crate::RegValueT<Nmisr_SPEC>;

impl Nmisr {
    #[doc = "IWDT Underflow/Refresh Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn iwdtst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        nmisr::Iwdtst,
        nmisr::Iwdtst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            nmisr::Iwdtst,
            nmisr::Iwdtst,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Interrupt Status Flag"]
    #[inline(always)]
    pub fn lvd1st(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        nmisr::Lvd1St,
        nmisr::Lvd1St,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            nmisr::Lvd1St,
            nmisr::Lvd1St,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "NMI Pin Interrupt Status Flag"]
    #[inline(always)]
    pub fn nmist(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        nmisr::Nmist,
        nmisr::Nmist,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            nmisr::Nmist,
            nmisr::Nmist,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SRAM Parity Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn rpest(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        nmisr::Rpest,
        nmisr::Rpest,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            nmisr::Rpest,
            nmisr::Rpest,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nmisr {
    #[inline(always)]
    fn default() -> Nmisr {
        <crate::RegValueT<Nmisr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtst_SPEC;
    pub type Iwdtst = crate::EnumBitfieldStruct<u8, Iwdtst_SPEC>;
    impl Iwdtst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1St_SPEC;
    pub type Lvd1St = crate::EnumBitfieldStruct<u8, Lvd1St_SPEC>;
    impl Lvd1St {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmist_SPEC;
    pub type Nmist = crate::EnumBitfieldStruct<u8, Nmist_SPEC>;
    impl Nmist {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpest_SPEC;
    pub type Rpest = crate::EnumBitfieldStruct<u8, Rpest_SPEC>;
    impl Rpest {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcenst0_SPEC;
impl crate::sealed::RegSpec for Dtcenst0_SPEC {
    type DataType = u32;
}

#[doc = "DTC Enable Status Register 0"]
pub type Dtcenst0 = crate::RegValueT<Dtcenst0_SPEC>;

impl Dtcenst0 {
    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dtcenst0::St1,
        dtcenst0::St1,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dtcenst0::St1,
            dtcenst0::St1,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dtcenst0::St2,
        dtcenst0::St2,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dtcenst0::St2,
            dtcenst0::St2,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dtcenst0::St3,
        dtcenst0::St3,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dtcenst0::St3,
            dtcenst0::St3,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dtcenst0::St4,
        dtcenst0::St4,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dtcenst0::St4,
            dtcenst0::St4,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dtcenst0::St5,
        dtcenst0::St5,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dtcenst0::St5,
            dtcenst0::St5,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dtcenst0::St6,
        dtcenst0::St6,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dtcenst0::St6,
            dtcenst0::St6,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dtcenst0::St7,
        dtcenst0::St7,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dtcenst0::St7,
            dtcenst0::St7,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        dtcenst0::St12,
        dtcenst0::St12,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            dtcenst0::St12,
            dtcenst0::St12,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        dtcenst0::St13,
        dtcenst0::St13,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            dtcenst0::St13,
            dtcenst0::St13,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        dtcenst0::St15,
        dtcenst0::St15,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            dtcenst0::St15,
            dtcenst0::St15,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dtcenst0::St16,
        dtcenst0::St16,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dtcenst0::St16,
            dtcenst0::St16,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        dtcenst0::St18,
        dtcenst0::St18,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            dtcenst0::St18,
            dtcenst0::St18,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        dtcenst0::St19,
        dtcenst0::St19,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            dtcenst0::St19,
            dtcenst0::St19,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        dtcenst0::St22,
        dtcenst0::St22,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            dtcenst0::St22,
            dtcenst0::St22,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        dtcenst0::St23,
        dtcenst0::St23,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            dtcenst0::St23,
            dtcenst0::St23,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        dtcenst0::St27,
        dtcenst0::St27,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            dtcenst0::St27,
            dtcenst0::St27,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        dtcenst0::St28,
        dtcenst0::St28,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            dtcenst0::St28,
            dtcenst0::St28,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        dtcenst0::St29,
        dtcenst0::St29,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            dtcenst0::St29,
            dtcenst0::St29,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        dtcenst0::St30,
        dtcenst0::St30,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            dtcenst0::St30,
            dtcenst0::St30,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        dtcenst0::St31,
        dtcenst0::St31,
        Dtcenst0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            dtcenst0::St31,
            dtcenst0::St31,
            Dtcenst0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dtcenst0 {
    #[inline(always)]
    fn default() -> Dtcenst0 {
        <crate::RegValueT<Dtcenst0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dtcenst0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St1_SPEC;
    pub type St1 = crate::EnumBitfieldStruct<u8, St1_SPEC>;
    impl St1 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St2_SPEC;
    pub type St2 = crate::EnumBitfieldStruct<u8, St2_SPEC>;
    impl St2 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St3_SPEC;
    pub type St3 = crate::EnumBitfieldStruct<u8, St3_SPEC>;
    impl St3 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St4_SPEC;
    pub type St4 = crate::EnumBitfieldStruct<u8, St4_SPEC>;
    impl St4 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St5_SPEC;
    pub type St5 = crate::EnumBitfieldStruct<u8, St5_SPEC>;
    impl St5 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St6_SPEC;
    pub type St6 = crate::EnumBitfieldStruct<u8, St6_SPEC>;
    impl St6 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St7_SPEC;
    pub type St7 = crate::EnumBitfieldStruct<u8, St7_SPEC>;
    impl St7 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St12_SPEC;
    pub type St12 = crate::EnumBitfieldStruct<u8, St12_SPEC>;
    impl St12 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St13_SPEC;
    pub type St13 = crate::EnumBitfieldStruct<u8, St13_SPEC>;
    impl St13 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St15_SPEC;
    pub type St15 = crate::EnumBitfieldStruct<u8, St15_SPEC>;
    impl St15 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St16_SPEC;
    pub type St16 = crate::EnumBitfieldStruct<u8, St16_SPEC>;
    impl St16 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St18_SPEC;
    pub type St18 = crate::EnumBitfieldStruct<u8, St18_SPEC>;
    impl St18 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St19_SPEC;
    pub type St19 = crate::EnumBitfieldStruct<u8, St19_SPEC>;
    impl St19 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St22_SPEC;
    pub type St22 = crate::EnumBitfieldStruct<u8, St22_SPEC>;
    impl St22 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St23_SPEC;
    pub type St23 = crate::EnumBitfieldStruct<u8, St23_SPEC>;
    impl St23 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St27_SPEC;
    pub type St27 = crate::EnumBitfieldStruct<u8, St27_SPEC>;
    impl St27 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St28_SPEC;
    pub type St28 = crate::EnumBitfieldStruct<u8, St28_SPEC>;
    impl St28 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St29_SPEC;
    pub type St29 = crate::EnumBitfieldStruct<u8, St29_SPEC>;
    impl St29 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St30_SPEC;
    pub type St30 = crate::EnumBitfieldStruct<u8, St30_SPEC>;
    impl St30 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St31_SPEC;
    pub type St31 = crate::EnumBitfieldStruct<u8, St31_SPEC>;
    impl St31 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcenst1_SPEC;
impl crate::sealed::RegSpec for Dtcenst1_SPEC {
    type DataType = u32;
}

#[doc = "DTC Enable Status Register 1"]
pub type Dtcenst1 = crate::RegValueT<Dtcenst1_SPEC>;

impl Dtcenst1 {
    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st32(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dtcenst1::St32,
        dtcenst1::St32,
        Dtcenst1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dtcenst1::St32,
            dtcenst1::St32,
            Dtcenst1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st33(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dtcenst1::St33,
        dtcenst1::St33,
        Dtcenst1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dtcenst1::St33,
            dtcenst1::St33,
            Dtcenst1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st34(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dtcenst1::St34,
        dtcenst1::St34,
        Dtcenst1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dtcenst1::St34,
            dtcenst1::St34,
            Dtcenst1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st35(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dtcenst1::St35,
        dtcenst1::St35,
        Dtcenst1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dtcenst1::St35,
            dtcenst1::St35,
            Dtcenst1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st36(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dtcenst1::St36,
        dtcenst1::St36,
        Dtcenst1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dtcenst1::St36,
            dtcenst1::St36,
            Dtcenst1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st37(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dtcenst1::St37,
        dtcenst1::St37,
        Dtcenst1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dtcenst1::St37,
            dtcenst1::St37,
            Dtcenst1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st38(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dtcenst1::St38,
        dtcenst1::St38,
        Dtcenst1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dtcenst1::St38,
            dtcenst1::St38,
            Dtcenst1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st40(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dtcenst1::St40,
        dtcenst1::St40,
        Dtcenst1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dtcenst1::St40,
            dtcenst1::St40,
            Dtcenst1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Status by Event Number i"]
    #[inline(always)]
    pub fn st41(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        dtcenst1::St41,
        dtcenst1::St41,
        Dtcenst1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            dtcenst1::St41,
            dtcenst1::St41,
            Dtcenst1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dtcenst1 {
    #[inline(always)]
    fn default() -> Dtcenst1 {
        <crate::RegValueT<Dtcenst1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dtcenst1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St32_SPEC;
    pub type St32 = crate::EnumBitfieldStruct<u8, St32_SPEC>;
    impl St32 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St33_SPEC;
    pub type St33 = crate::EnumBitfieldStruct<u8, St33_SPEC>;
    impl St33 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St34_SPEC;
    pub type St34 = crate::EnumBitfieldStruct<u8, St34_SPEC>;
    impl St34 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St35_SPEC;
    pub type St35 = crate::EnumBitfieldStruct<u8, St35_SPEC>;
    impl St35 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St36_SPEC;
    pub type St36 = crate::EnumBitfieldStruct<u8, St36_SPEC>;
    impl St36 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St37_SPEC;
    pub type St37 = crate::EnumBitfieldStruct<u8, St37_SPEC>;
    impl St37 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St38_SPEC;
    pub type St38 = crate::EnumBitfieldStruct<u8, St38_SPEC>;
    impl St38 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St40_SPEC;
    pub type St40 = crate::EnumBitfieldStruct<u8, St40_SPEC>;
    impl St40 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St41_SPEC;
    pub type St41 = crate::EnumBitfieldStruct<u8, St41_SPEC>;
    impl St41 {
        #[doc = "DTC Disable by Event number i"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcenset0_SPEC;
impl crate::sealed::RegSpec for Dtcenset0_SPEC {
    type DataType = u32;
}

#[doc = "DTC Enable Set Register 0"]
pub type Dtcenset0 = crate::RegValueT<Dtcenset0_SPEC>;

impl Dtcenset0 {
    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dtcenset0::Set1,
        dtcenset0::Set1,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dtcenset0::Set1,
            dtcenset0::Set1,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dtcenset0::Set2,
        dtcenset0::Set2,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dtcenset0::Set2,
            dtcenset0::Set2,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dtcenset0::Set3,
        dtcenset0::Set3,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dtcenset0::Set3,
            dtcenset0::Set3,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dtcenset0::Set4,
        dtcenset0::Set4,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dtcenset0::Set4,
            dtcenset0::Set4,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dtcenset0::Set5,
        dtcenset0::Set5,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dtcenset0::Set5,
            dtcenset0::Set5,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dtcenset0::Set6,
        dtcenset0::Set6,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dtcenset0::Set6,
            dtcenset0::Set6,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dtcenset0::Set7,
        dtcenset0::Set7,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dtcenset0::Set7,
            dtcenset0::Set7,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        dtcenset0::Set12,
        dtcenset0::Set12,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            dtcenset0::Set12,
            dtcenset0::Set12,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        dtcenset0::Set13,
        dtcenset0::Set13,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            dtcenset0::Set13,
            dtcenset0::Set13,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        dtcenset0::Set15,
        dtcenset0::Set15,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            dtcenset0::Set15,
            dtcenset0::Set15,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dtcenset0::Set16,
        dtcenset0::Set16,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dtcenset0::Set16,
            dtcenset0::Set16,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        dtcenset0::Set18,
        dtcenset0::Set18,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            dtcenset0::Set18,
            dtcenset0::Set18,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        dtcenset0::Set19,
        dtcenset0::Set19,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            dtcenset0::Set19,
            dtcenset0::Set19,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        dtcenset0::Set22,
        dtcenset0::Set22,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            dtcenset0::Set22,
            dtcenset0::Set22,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        dtcenset0::Set23,
        dtcenset0::Set23,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            dtcenset0::Set23,
            dtcenset0::Set23,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        dtcenset0::Set27,
        dtcenset0::Set27,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            dtcenset0::Set27,
            dtcenset0::Set27,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        dtcenset0::Set28,
        dtcenset0::Set28,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            dtcenset0::Set28,
            dtcenset0::Set28,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        dtcenset0::Set29,
        dtcenset0::Set29,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            dtcenset0::Set29,
            dtcenset0::Set29,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        dtcenset0::Set30,
        dtcenset0::Set30,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            dtcenset0::Set30,
            dtcenset0::Set30,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        dtcenset0::Set31,
        dtcenset0::Set31,
        Dtcenset0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            dtcenset0::Set31,
            dtcenset0::Set31,
            Dtcenset0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dtcenset0 {
    #[inline(always)]
    fn default() -> Dtcenset0 {
        <crate::RegValueT<Dtcenset0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dtcenset0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set1_SPEC;
    pub type Set1 = crate::EnumBitfieldStruct<u8, Set1_SPEC>;
    impl Set1 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set2_SPEC;
    pub type Set2 = crate::EnumBitfieldStruct<u8, Set2_SPEC>;
    impl Set2 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set3_SPEC;
    pub type Set3 = crate::EnumBitfieldStruct<u8, Set3_SPEC>;
    impl Set3 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set4_SPEC;
    pub type Set4 = crate::EnumBitfieldStruct<u8, Set4_SPEC>;
    impl Set4 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set5_SPEC;
    pub type Set5 = crate::EnumBitfieldStruct<u8, Set5_SPEC>;
    impl Set5 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set6_SPEC;
    pub type Set6 = crate::EnumBitfieldStruct<u8, Set6_SPEC>;
    impl Set6 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set7_SPEC;
    pub type Set7 = crate::EnumBitfieldStruct<u8, Set7_SPEC>;
    impl Set7 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set12_SPEC;
    pub type Set12 = crate::EnumBitfieldStruct<u8, Set12_SPEC>;
    impl Set12 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set13_SPEC;
    pub type Set13 = crate::EnumBitfieldStruct<u8, Set13_SPEC>;
    impl Set13 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set15_SPEC;
    pub type Set15 = crate::EnumBitfieldStruct<u8, Set15_SPEC>;
    impl Set15 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set16_SPEC;
    pub type Set16 = crate::EnumBitfieldStruct<u8, Set16_SPEC>;
    impl Set16 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set18_SPEC;
    pub type Set18 = crate::EnumBitfieldStruct<u8, Set18_SPEC>;
    impl Set18 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set19_SPEC;
    pub type Set19 = crate::EnumBitfieldStruct<u8, Set19_SPEC>;
    impl Set19 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set22_SPEC;
    pub type Set22 = crate::EnumBitfieldStruct<u8, Set22_SPEC>;
    impl Set22 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set23_SPEC;
    pub type Set23 = crate::EnumBitfieldStruct<u8, Set23_SPEC>;
    impl Set23 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set27_SPEC;
    pub type Set27 = crate::EnumBitfieldStruct<u8, Set27_SPEC>;
    impl Set27 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set28_SPEC;
    pub type Set28 = crate::EnumBitfieldStruct<u8, Set28_SPEC>;
    impl Set28 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set29_SPEC;
    pub type Set29 = crate::EnumBitfieldStruct<u8, Set29_SPEC>;
    impl Set29 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set30_SPEC;
    pub type Set30 = crate::EnumBitfieldStruct<u8, Set30_SPEC>;
    impl Set30 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set31_SPEC;
    pub type Set31 = crate::EnumBitfieldStruct<u8, Set31_SPEC>;
    impl Set31 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcenset1_SPEC;
impl crate::sealed::RegSpec for Dtcenset1_SPEC {
    type DataType = u32;
}

#[doc = "DTC Enable Set Register 1"]
pub type Dtcenset1 = crate::RegValueT<Dtcenset1_SPEC>;

impl Dtcenset1 {
    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set32(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dtcenset1::Set32,
        dtcenset1::Set32,
        Dtcenset1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dtcenset1::Set32,
            dtcenset1::Set32,
            Dtcenset1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set33(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dtcenset1::Set33,
        dtcenset1::Set33,
        Dtcenset1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dtcenset1::Set33,
            dtcenset1::Set33,
            Dtcenset1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set34(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dtcenset1::Set34,
        dtcenset1::Set34,
        Dtcenset1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dtcenset1::Set34,
            dtcenset1::Set34,
            Dtcenset1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set35(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dtcenset1::Set35,
        dtcenset1::Set35,
        Dtcenset1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dtcenset1::Set35,
            dtcenset1::Set35,
            Dtcenset1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set36(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dtcenset1::Set36,
        dtcenset1::Set36,
        Dtcenset1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dtcenset1::Set36,
            dtcenset1::Set36,
            Dtcenset1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set37(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dtcenset1::Set37,
        dtcenset1::Set37,
        Dtcenset1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dtcenset1::Set37,
            dtcenset1::Set37,
            Dtcenset1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set38(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dtcenset1::Set38,
        dtcenset1::Set38,
        Dtcenset1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dtcenset1::Set38,
            dtcenset1::Set38,
            Dtcenset1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set40(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dtcenset1::Set40,
        dtcenset1::Set40,
        Dtcenset1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dtcenset1::Set40,
            dtcenset1::Set40,
            Dtcenset1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Set by Event Number i"]
    #[inline(always)]
    pub fn set41(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        dtcenset1::Set41,
        dtcenset1::Set41,
        Dtcenset1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            dtcenset1::Set41,
            dtcenset1::Set41,
            Dtcenset1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dtcenset1 {
    #[inline(always)]
    fn default() -> Dtcenset1 {
        <crate::RegValueT<Dtcenset1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dtcenset1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set32_SPEC;
    pub type Set32 = crate::EnumBitfieldStruct<u8, Set32_SPEC>;
    impl Set32 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set33_SPEC;
    pub type Set33 = crate::EnumBitfieldStruct<u8, Set33_SPEC>;
    impl Set33 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set34_SPEC;
    pub type Set34 = crate::EnumBitfieldStruct<u8, Set34_SPEC>;
    impl Set34 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set35_SPEC;
    pub type Set35 = crate::EnumBitfieldStruct<u8, Set35_SPEC>;
    impl Set35 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set36_SPEC;
    pub type Set36 = crate::EnumBitfieldStruct<u8, Set36_SPEC>;
    impl Set36 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set37_SPEC;
    pub type Set37 = crate::EnumBitfieldStruct<u8, Set37_SPEC>;
    impl Set37 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set38_SPEC;
    pub type Set38 = crate::EnumBitfieldStruct<u8, Set38_SPEC>;
    impl Set38 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set40_SPEC;
    pub type Set40 = crate::EnumBitfieldStruct<u8, Set40_SPEC>;
    impl Set40 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Set41_SPEC;
    pub type Set41 = crate::EnumBitfieldStruct<u8, Set41_SPEC>;
    impl Set41 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Enable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcenclr0_SPEC;
impl crate::sealed::RegSpec for Dtcenclr0_SPEC {
    type DataType = u32;
}

#[doc = "DTC Enable Clear Register 0"]
pub type Dtcenclr0 = crate::RegValueT<Dtcenclr0_SPEC>;

impl Dtcenclr0 {
    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dtcenclr0::Clr1,
        dtcenclr0::Clr1,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dtcenclr0::Clr1,
            dtcenclr0::Clr1,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dtcenclr0::Clr2,
        dtcenclr0::Clr2,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dtcenclr0::Clr2,
            dtcenclr0::Clr2,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dtcenclr0::Clr3,
        dtcenclr0::Clr3,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dtcenclr0::Clr3,
            dtcenclr0::Clr3,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dtcenclr0::Clr4,
        dtcenclr0::Clr4,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dtcenclr0::Clr4,
            dtcenclr0::Clr4,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dtcenclr0::Clr5,
        dtcenclr0::Clr5,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dtcenclr0::Clr5,
            dtcenclr0::Clr5,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dtcenclr0::Clr6,
        dtcenclr0::Clr6,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dtcenclr0::Clr6,
            dtcenclr0::Clr6,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dtcenclr0::Clr7,
        dtcenclr0::Clr7,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dtcenclr0::Clr7,
            dtcenclr0::Clr7,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        dtcenclr0::Clr12,
        dtcenclr0::Clr12,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            dtcenclr0::Clr12,
            dtcenclr0::Clr12,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        dtcenclr0::Clr13,
        dtcenclr0::Clr13,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            dtcenclr0::Clr13,
            dtcenclr0::Clr13,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        dtcenclr0::Clr15,
        dtcenclr0::Clr15,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            dtcenclr0::Clr15,
            dtcenclr0::Clr15,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dtcenclr0::Clr16,
        dtcenclr0::Clr16,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dtcenclr0::Clr16,
            dtcenclr0::Clr16,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        dtcenclr0::Clr18,
        dtcenclr0::Clr18,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            dtcenclr0::Clr18,
            dtcenclr0::Clr18,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        dtcenclr0::Clr19,
        dtcenclr0::Clr19,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            dtcenclr0::Clr19,
            dtcenclr0::Clr19,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        dtcenclr0::Clr22,
        dtcenclr0::Clr22,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            dtcenclr0::Clr22,
            dtcenclr0::Clr22,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        dtcenclr0::Clr23,
        dtcenclr0::Clr23,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            dtcenclr0::Clr23,
            dtcenclr0::Clr23,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        dtcenclr0::Clr27,
        dtcenclr0::Clr27,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            dtcenclr0::Clr27,
            dtcenclr0::Clr27,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        dtcenclr0::Clr28,
        dtcenclr0::Clr28,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            dtcenclr0::Clr28,
            dtcenclr0::Clr28,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        dtcenclr0::Clr29,
        dtcenclr0::Clr29,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            dtcenclr0::Clr29,
            dtcenclr0::Clr29,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        dtcenclr0::Clr30,
        dtcenclr0::Clr30,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            dtcenclr0::Clr30,
            dtcenclr0::Clr30,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        dtcenclr0::Clr31,
        dtcenclr0::Clr31,
        Dtcenclr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            dtcenclr0::Clr31,
            dtcenclr0::Clr31,
            Dtcenclr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dtcenclr0 {
    #[inline(always)]
    fn default() -> Dtcenclr0 {
        <crate::RegValueT<Dtcenclr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dtcenclr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr1_SPEC;
    pub type Clr1 = crate::EnumBitfieldStruct<u8, Clr1_SPEC>;
    impl Clr1 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr2_SPEC;
    pub type Clr2 = crate::EnumBitfieldStruct<u8, Clr2_SPEC>;
    impl Clr2 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr3_SPEC;
    pub type Clr3 = crate::EnumBitfieldStruct<u8, Clr3_SPEC>;
    impl Clr3 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr4_SPEC;
    pub type Clr4 = crate::EnumBitfieldStruct<u8, Clr4_SPEC>;
    impl Clr4 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr5_SPEC;
    pub type Clr5 = crate::EnumBitfieldStruct<u8, Clr5_SPEC>;
    impl Clr5 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr6_SPEC;
    pub type Clr6 = crate::EnumBitfieldStruct<u8, Clr6_SPEC>;
    impl Clr6 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr7_SPEC;
    pub type Clr7 = crate::EnumBitfieldStruct<u8, Clr7_SPEC>;
    impl Clr7 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr12_SPEC;
    pub type Clr12 = crate::EnumBitfieldStruct<u8, Clr12_SPEC>;
    impl Clr12 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr13_SPEC;
    pub type Clr13 = crate::EnumBitfieldStruct<u8, Clr13_SPEC>;
    impl Clr13 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr15_SPEC;
    pub type Clr15 = crate::EnumBitfieldStruct<u8, Clr15_SPEC>;
    impl Clr15 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr16_SPEC;
    pub type Clr16 = crate::EnumBitfieldStruct<u8, Clr16_SPEC>;
    impl Clr16 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr18_SPEC;
    pub type Clr18 = crate::EnumBitfieldStruct<u8, Clr18_SPEC>;
    impl Clr18 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr19_SPEC;
    pub type Clr19 = crate::EnumBitfieldStruct<u8, Clr19_SPEC>;
    impl Clr19 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr22_SPEC;
    pub type Clr22 = crate::EnumBitfieldStruct<u8, Clr22_SPEC>;
    impl Clr22 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr23_SPEC;
    pub type Clr23 = crate::EnumBitfieldStruct<u8, Clr23_SPEC>;
    impl Clr23 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr27_SPEC;
    pub type Clr27 = crate::EnumBitfieldStruct<u8, Clr27_SPEC>;
    impl Clr27 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr28_SPEC;
    pub type Clr28 = crate::EnumBitfieldStruct<u8, Clr28_SPEC>;
    impl Clr28 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr29_SPEC;
    pub type Clr29 = crate::EnumBitfieldStruct<u8, Clr29_SPEC>;
    impl Clr29 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr30_SPEC;
    pub type Clr30 = crate::EnumBitfieldStruct<u8, Clr30_SPEC>;
    impl Clr30 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr31_SPEC;
    pub type Clr31 = crate::EnumBitfieldStruct<u8, Clr31_SPEC>;
    impl Clr31 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcenclr1_SPEC;
impl crate::sealed::RegSpec for Dtcenclr1_SPEC {
    type DataType = u32;
}

#[doc = "DTC Enable Clear Register 1"]
pub type Dtcenclr1 = crate::RegValueT<Dtcenclr1_SPEC>;

impl Dtcenclr1 {
    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr32(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dtcenclr1::Clr32,
        dtcenclr1::Clr32,
        Dtcenclr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dtcenclr1::Clr32,
            dtcenclr1::Clr32,
            Dtcenclr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr33(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dtcenclr1::Clr33,
        dtcenclr1::Clr33,
        Dtcenclr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dtcenclr1::Clr33,
            dtcenclr1::Clr33,
            Dtcenclr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr34(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dtcenclr1::Clr34,
        dtcenclr1::Clr34,
        Dtcenclr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dtcenclr1::Clr34,
            dtcenclr1::Clr34,
            Dtcenclr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr35(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dtcenclr1::Clr35,
        dtcenclr1::Clr35,
        Dtcenclr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dtcenclr1::Clr35,
            dtcenclr1::Clr35,
            Dtcenclr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr36(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dtcenclr1::Clr36,
        dtcenclr1::Clr36,
        Dtcenclr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dtcenclr1::Clr36,
            dtcenclr1::Clr36,
            Dtcenclr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr37(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dtcenclr1::Clr37,
        dtcenclr1::Clr37,
        Dtcenclr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dtcenclr1::Clr37,
            dtcenclr1::Clr37,
            Dtcenclr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr38(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dtcenclr1::Clr38,
        dtcenclr1::Clr38,
        Dtcenclr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dtcenclr1::Clr38,
            dtcenclr1::Clr38,
            Dtcenclr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr40(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dtcenclr1::Clr40,
        dtcenclr1::Clr40,
        Dtcenclr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dtcenclr1::Clr40,
            dtcenclr1::Clr40,
            Dtcenclr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable Clear by Event Number i"]
    #[inline(always)]
    pub fn clr41(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        dtcenclr1::Clr41,
        dtcenclr1::Clr41,
        Dtcenclr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            dtcenclr1::Clr41,
            dtcenclr1::Clr41,
            Dtcenclr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dtcenclr1 {
    #[inline(always)]
    fn default() -> Dtcenclr1 {
        <crate::RegValueT<Dtcenclr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dtcenclr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr32_SPEC;
    pub type Clr32 = crate::EnumBitfieldStruct<u8, Clr32_SPEC>;
    impl Clr32 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr33_SPEC;
    pub type Clr33 = crate::EnumBitfieldStruct<u8, Clr33_SPEC>;
    impl Clr33 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr34_SPEC;
    pub type Clr34 = crate::EnumBitfieldStruct<u8, Clr34_SPEC>;
    impl Clr34 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr35_SPEC;
    pub type Clr35 = crate::EnumBitfieldStruct<u8, Clr35_SPEC>;
    impl Clr35 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr36_SPEC;
    pub type Clr36 = crate::EnumBitfieldStruct<u8, Clr36_SPEC>;
    impl Clr36 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr37_SPEC;
    pub type Clr37 = crate::EnumBitfieldStruct<u8, Clr37_SPEC>;
    impl Clr37 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr38_SPEC;
    pub type Clr38 = crate::EnumBitfieldStruct<u8, Clr38_SPEC>;
    impl Clr38 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr40_SPEC;
    pub type Clr40 = crate::EnumBitfieldStruct<u8, Clr40_SPEC>;
    impl Clr40 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr41_SPEC;
    pub type Clr41 = crate::EnumBitfieldStruct<u8, Clr41_SPEC>;
    impl Clr41 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC Disable by Event number i"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intflag0_SPEC;
impl crate::sealed::RegSpec for Intflag0_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Request Flag Monitor Register 0"]
pub type Intflag0 = crate::RegValueT<Intflag0_SPEC>;

impl Intflag0 {
    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        intflag0::If0,
        intflag0::If0,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            intflag0::If0,
            intflag0::If0,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        intflag0::If1,
        intflag0::If1,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            intflag0::If1,
            intflag0::If1,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        intflag0::If2,
        intflag0::If2,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            intflag0::If2,
            intflag0::If2,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        intflag0::If3,
        intflag0::If3,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            intflag0::If3,
            intflag0::If3,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        intflag0::If4,
        intflag0::If4,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            intflag0::If4,
            intflag0::If4,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        intflag0::If5,
        intflag0::If5,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            intflag0::If5,
            intflag0::If5,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        intflag0::If6,
        intflag0::If6,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            intflag0::If6,
            intflag0::If6,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        intflag0::If7,
        intflag0::If7,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            intflag0::If7,
            intflag0::If7,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        intflag0::If10,
        intflag0::If10,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            intflag0::If10,
            intflag0::If10,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        intflag0::If11,
        intflag0::If11,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            intflag0::If11,
            intflag0::If11,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        intflag0::If12,
        intflag0::If12,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            intflag0::If12,
            intflag0::If12,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        intflag0::If13,
        intflag0::If13,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            intflag0::If13,
            intflag0::If13,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        intflag0::If14,
        intflag0::If14,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            intflag0::If14,
            intflag0::If14,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        intflag0::If15,
        intflag0::If15,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            intflag0::If15,
            intflag0::If15,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        intflag0::If16,
        intflag0::If16,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            intflag0::If16,
            intflag0::If16,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        intflag0::If17,
        intflag0::If17,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            intflag0::If17,
            intflag0::If17,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        intflag0::If18,
        intflag0::If18,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            intflag0::If18,
            intflag0::If18,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        intflag0::If19,
        intflag0::If19,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            intflag0::If19,
            intflag0::If19,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        intflag0::If20,
        intflag0::If20,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            intflag0::If20,
            intflag0::If20,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        intflag0::If21,
        intflag0::If21,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            intflag0::If21,
            intflag0::If21,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        intflag0::If22,
        intflag0::If22,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            intflag0::If22,
            intflag0::If22,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        intflag0::If23,
        intflag0::If23,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            intflag0::If23,
            intflag0::If23,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        intflag0::If24,
        intflag0::If24,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            intflag0::If24,
            intflag0::If24,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        intflag0::If25,
        intflag0::If25,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            intflag0::If25,
            intflag0::If25,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        intflag0::If26,
        intflag0::If26,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            intflag0::If26,
            intflag0::If26,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        intflag0::If27,
        intflag0::If27,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            intflag0::If27,
            intflag0::If27,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        intflag0::If28,
        intflag0::If28,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            intflag0::If28,
            intflag0::If28,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        intflag0::If29,
        intflag0::If29,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            intflag0::If29,
            intflag0::If29,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        intflag0::If30,
        intflag0::If30,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            intflag0::If30,
            intflag0::If30,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        intflag0::If31,
        intflag0::If31,
        Intflag0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            intflag0::If31,
            intflag0::If31,
            Intflag0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Intflag0 {
    #[inline(always)]
    fn default() -> Intflag0 {
        <crate::RegValueT<Intflag0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod intflag0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If0_SPEC;
    pub type If0 = crate::EnumBitfieldStruct<u8, If0_SPEC>;
    impl If0 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If1_SPEC;
    pub type If1 = crate::EnumBitfieldStruct<u8, If1_SPEC>;
    impl If1 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If2_SPEC;
    pub type If2 = crate::EnumBitfieldStruct<u8, If2_SPEC>;
    impl If2 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If3_SPEC;
    pub type If3 = crate::EnumBitfieldStruct<u8, If3_SPEC>;
    impl If3 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If4_SPEC;
    pub type If4 = crate::EnumBitfieldStruct<u8, If4_SPEC>;
    impl If4 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If5_SPEC;
    pub type If5 = crate::EnumBitfieldStruct<u8, If5_SPEC>;
    impl If5 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If6_SPEC;
    pub type If6 = crate::EnumBitfieldStruct<u8, If6_SPEC>;
    impl If6 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If7_SPEC;
    pub type If7 = crate::EnumBitfieldStruct<u8, If7_SPEC>;
    impl If7 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If10_SPEC;
    pub type If10 = crate::EnumBitfieldStruct<u8, If10_SPEC>;
    impl If10 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If11_SPEC;
    pub type If11 = crate::EnumBitfieldStruct<u8, If11_SPEC>;
    impl If11 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If12_SPEC;
    pub type If12 = crate::EnumBitfieldStruct<u8, If12_SPEC>;
    impl If12 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If13_SPEC;
    pub type If13 = crate::EnumBitfieldStruct<u8, If13_SPEC>;
    impl If13 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If14_SPEC;
    pub type If14 = crate::EnumBitfieldStruct<u8, If14_SPEC>;
    impl If14 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If15_SPEC;
    pub type If15 = crate::EnumBitfieldStruct<u8, If15_SPEC>;
    impl If15 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If16_SPEC;
    pub type If16 = crate::EnumBitfieldStruct<u8, If16_SPEC>;
    impl If16 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If17_SPEC;
    pub type If17 = crate::EnumBitfieldStruct<u8, If17_SPEC>;
    impl If17 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If18_SPEC;
    pub type If18 = crate::EnumBitfieldStruct<u8, If18_SPEC>;
    impl If18 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If19_SPEC;
    pub type If19 = crate::EnumBitfieldStruct<u8, If19_SPEC>;
    impl If19 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If20_SPEC;
    pub type If20 = crate::EnumBitfieldStruct<u8, If20_SPEC>;
    impl If20 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If21_SPEC;
    pub type If21 = crate::EnumBitfieldStruct<u8, If21_SPEC>;
    impl If21 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If22_SPEC;
    pub type If22 = crate::EnumBitfieldStruct<u8, If22_SPEC>;
    impl If22 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If23_SPEC;
    pub type If23 = crate::EnumBitfieldStruct<u8, If23_SPEC>;
    impl If23 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If24_SPEC;
    pub type If24 = crate::EnumBitfieldStruct<u8, If24_SPEC>;
    impl If24 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If25_SPEC;
    pub type If25 = crate::EnumBitfieldStruct<u8, If25_SPEC>;
    impl If25 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If26_SPEC;
    pub type If26 = crate::EnumBitfieldStruct<u8, If26_SPEC>;
    impl If26 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If27_SPEC;
    pub type If27 = crate::EnumBitfieldStruct<u8, If27_SPEC>;
    impl If27 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If28_SPEC;
    pub type If28 = crate::EnumBitfieldStruct<u8, If28_SPEC>;
    impl If28 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If29_SPEC;
    pub type If29 = crate::EnumBitfieldStruct<u8, If29_SPEC>;
    impl If29 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If30_SPEC;
    pub type If30 = crate::EnumBitfieldStruct<u8, If30_SPEC>;
    impl If30 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If31_SPEC;
    pub type If31 = crate::EnumBitfieldStruct<u8, If31_SPEC>;
    impl If31 {
        #[doc = "Interrupt request of event number i is not being accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt request of event number i is being accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intflag1_SPEC;
impl crate::sealed::RegSpec for Intflag1_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Request Flag Monitor Register 1"]
pub type Intflag1 = crate::RegValueT<Intflag1_SPEC>;

impl Intflag1 {
    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if32(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        intflag1::If32,
        intflag1::If32,
        Intflag1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            intflag1::If32,
            intflag1::If32,
            Intflag1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if33(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        intflag1::If33,
        intflag1::If33,
        Intflag1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            intflag1::If33,
            intflag1::If33,
            Intflag1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if34(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        intflag1::If34,
        intflag1::If34,
        Intflag1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            intflag1::If34,
            intflag1::If34,
            Intflag1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if35(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        intflag1::If35,
        intflag1::If35,
        Intflag1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            intflag1::If35,
            intflag1::If35,
            Intflag1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if36(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        intflag1::If36,
        intflag1::If36,
        Intflag1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            intflag1::If36,
            intflag1::If36,
            Intflag1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if37(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        intflag1::If37,
        intflag1::If37,
        Intflag1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            intflag1::If37,
            intflag1::If37,
            Intflag1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if38(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        intflag1::If38,
        intflag1::If38,
        Intflag1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            intflag1::If38,
            intflag1::If38,
            Intflag1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if39(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        intflag1::If39,
        intflag1::If39,
        Intflag1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            intflag1::If39,
            intflag1::If39,
            Intflag1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if40(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        intflag1::If40,
        intflag1::If40,
        Intflag1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            intflag1::If40,
            intflag1::If40,
            Intflag1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Flag Monitor"]
    #[inline(always)]
    pub fn if41(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        intflag1::If41,
        intflag1::If41,
        Intflag1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            intflag1::If41,
            intflag1::If41,
            Intflag1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Intflag1 {
    #[inline(always)]
    fn default() -> Intflag1 {
        <crate::RegValueT<Intflag1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod intflag1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If32_SPEC;
    pub type If32 = crate::EnumBitfieldStruct<u8, If32_SPEC>;
    impl If32 {
        #[doc = "Interrupt source of event number i is not accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt source of event number i is accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If33_SPEC;
    pub type If33 = crate::EnumBitfieldStruct<u8, If33_SPEC>;
    impl If33 {
        #[doc = "Interrupt source of event number i is not accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt source of event number i is accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If34_SPEC;
    pub type If34 = crate::EnumBitfieldStruct<u8, If34_SPEC>;
    impl If34 {
        #[doc = "Interrupt source of event number i is not accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt source of event number i is accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If35_SPEC;
    pub type If35 = crate::EnumBitfieldStruct<u8, If35_SPEC>;
    impl If35 {
        #[doc = "Interrupt source of event number i is not accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt source of event number i is accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If36_SPEC;
    pub type If36 = crate::EnumBitfieldStruct<u8, If36_SPEC>;
    impl If36 {
        #[doc = "Interrupt source of event number i is not accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt source of event number i is accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If37_SPEC;
    pub type If37 = crate::EnumBitfieldStruct<u8, If37_SPEC>;
    impl If37 {
        #[doc = "Interrupt source of event number i is not accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt source of event number i is accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If38_SPEC;
    pub type If38 = crate::EnumBitfieldStruct<u8, If38_SPEC>;
    impl If38 {
        #[doc = "Interrupt source of event number i is not accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt source of event number i is accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If39_SPEC;
    pub type If39 = crate::EnumBitfieldStruct<u8, If39_SPEC>;
    impl If39 {
        #[doc = "Interrupt source of event number i is not accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt source of event number i is accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If40_SPEC;
    pub type If40 = crate::EnumBitfieldStruct<u8, If40_SPEC>;
    impl If40 {
        #[doc = "Interrupt source of event number i is not accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt source of event number i is accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct If41_SPEC;
    pub type If41 = crate::EnumBitfieldStruct<u8, If41_SPEC>;
    impl If41 {
        #[doc = "Interrupt source of event number i is not accepted by the ICU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt source of event number i is accepted by the ICU"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbyedcr0_SPEC;
impl crate::sealed::RegSpec for Sbyedcr0_SPEC {
    type DataType = u32;
}

#[doc = "Software Standby/Snooze End Control Register 0"]
pub type Sbyedcr0 = crate::RegValueT<Sbyedcr0_SPEC>;

impl Sbyedcr0 {
    #[doc = "IWDT Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn iwdted(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sbyedcr0::Iwdted,
        sbyedcr0::Iwdted,
        Sbyedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sbyedcr0::Iwdted,
            sbyedcr0::Iwdted,
            Sbyedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVD1 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn lvd1ed(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sbyedcr0::Lvd1Ed,
        sbyedcr0::Lvd1Ed,
        Sbyedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sbyedcr0::Lvd1Ed,
            sbyedcr0::Lvd1Ed,
            Sbyedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irq0ed(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sbyedcr0::Irq0Ed,
        sbyedcr0::Irq0Ed,
        Sbyedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sbyedcr0::Irq0Ed,
            sbyedcr0::Irq0Ed,
            Sbyedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irq1ed(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sbyedcr0::Irq1Ed,
        sbyedcr0::Irq1Ed,
        Sbyedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sbyedcr0::Irq1Ed,
            sbyedcr0::Irq1Ed,
            Sbyedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irq2ed(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sbyedcr0::Irq2Ed,
        sbyedcr0::Irq2Ed,
        Sbyedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sbyedcr0::Irq2Ed,
            sbyedcr0::Irq2Ed,
            Sbyedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irq3ed(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sbyedcr0::Irq3Ed,
        sbyedcr0::Irq3Ed,
        Sbyedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sbyedcr0::Irq3Ed,
            sbyedcr0::Irq3Ed,
            Sbyedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irq4ed(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sbyedcr0::Irq4Ed,
        sbyedcr0::Irq4Ed,
        Sbyedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sbyedcr0::Irq4Ed,
            sbyedcr0::Irq4Ed,
            Sbyedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irq5ed(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sbyedcr0::Irq5Ed,
        sbyedcr0::Irq5Ed,
        Sbyedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sbyedcr0::Irq5Ed,
            sbyedcr0::Irq5Ed,
            Sbyedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Transfer Complete Interrupt Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn dtced(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        sbyedcr0::Dtced,
        sbyedcr0::Dtced,
        Sbyedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            sbyedcr0::Dtced,
            sbyedcr0::Dtced,
            Sbyedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI00 Transfer End or Buffer Empty Interrupt Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn spi00rxed(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        sbyedcr0::Spi00Rxed,
        sbyedcr0::Spi00Rxed,
        Sbyedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            sbyedcr0::Spi00Rxed,
            sbyedcr0::Spi00Rxed,
            Sbyedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "UART0 Reception Communication Error Occurrence Interrupt Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn uart0erred(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        sbyedcr0::Uart0Erred,
        sbyedcr0::Uart0Erred,
        Sbyedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            sbyedcr0::Uart0Erred,
            sbyedcr0::Uart0Erred,
            Sbyedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IICA0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn iica0ed(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        sbyedcr0::Iica0Ed,
        sbyedcr0::Iica0Ed,
        Sbyedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            sbyedcr0::Iica0Ed,
            sbyedcr0::Iica0Ed,
            Sbyedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "UART0 Reception Transfer End Interrupt Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn uart0rxed(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        sbyedcr0::Uart0Rxed,
        sbyedcr0::Uart0Rxed,
        Sbyedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            sbyedcr0::Uart0Rxed,
            sbyedcr0::Uart0Rxed,
            Sbyedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "End of A/D Conversion Interrupt Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn adc12ed(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        sbyedcr0::Adc12Ed,
        sbyedcr0::Adc12Ed,
        Sbyedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            sbyedcr0::Adc12Ed,
            sbyedcr0::Adc12Ed,
            Sbyedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sbyedcr0 {
    #[inline(always)]
    fn default() -> Sbyedcr0 {
        <crate::RegValueT<Sbyedcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sbyedcr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdted_SPEC;
    pub type Iwdted = crate::EnumBitfieldStruct<u8, Iwdted_SPEC>;
    impl Iwdted {
        #[doc = "Software Standby/Snooze Mode returns by IWDT interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by IWDT interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Ed_SPEC;
    pub type Lvd1Ed = crate::EnumBitfieldStruct<u8, Lvd1Ed_SPEC>;
    impl Lvd1Ed {
        #[doc = "Software Standby/Snooze Mode returns by LVD1 interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by LVD1 interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq0Ed_SPEC;
    pub type Irq0Ed = crate::EnumBitfieldStruct<u8, Irq0Ed_SPEC>;
    impl Irq0Ed {
        #[doc = "Software Standby/Snooze mode returns by IRQ0 interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by IRQ0 interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq1Ed_SPEC;
    pub type Irq1Ed = crate::EnumBitfieldStruct<u8, Irq1Ed_SPEC>;
    impl Irq1Ed {
        #[doc = "Software Standby/Snooze mode returns by IRQ1 interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by IRQ1 interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq2Ed_SPEC;
    pub type Irq2Ed = crate::EnumBitfieldStruct<u8, Irq2Ed_SPEC>;
    impl Irq2Ed {
        #[doc = "Software Standby/Snooze mode returns by IRQ2 interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by IRQ2 interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq3Ed_SPEC;
    pub type Irq3Ed = crate::EnumBitfieldStruct<u8, Irq3Ed_SPEC>;
    impl Irq3Ed {
        #[doc = "Software Standby/Snooze mode returns by IRQ3 interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by IRQ3 interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq4Ed_SPEC;
    pub type Irq4Ed = crate::EnumBitfieldStruct<u8, Irq4Ed_SPEC>;
    impl Irq4Ed {
        #[doc = "Software Standby/Snooze mode returns by IRQ4 interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by IRQ4 interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq5Ed_SPEC;
    pub type Irq5Ed = crate::EnumBitfieldStruct<u8, Irq5Ed_SPEC>;
    impl Irq5Ed {
        #[doc = "Software Standby/Snooze mode returns by IRQ5 interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by IRQ5 interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtced_SPEC;
    pub type Dtced = crate::EnumBitfieldStruct<u8, Dtced_SPEC>;
    impl Dtced {
        #[doc = "Snooze Mode returns by DTC transfer complete interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Snooze Mode returns by DTC transfer complete interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spi00Rxed_SPEC;
    pub type Spi00Rxed = crate::EnumBitfieldStruct<u8, Spi00Rxed_SPEC>;
    impl Spi00Rxed {
        #[doc = "Snooze Mode returns by SPI00 transfer end or buffer empty interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Snooze Mode returns by SPI00 transfer end or buffer empty interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uart0Erred_SPEC;
    pub type Uart0Erred = crate::EnumBitfieldStruct<u8, Uart0Erred_SPEC>;
    impl Uart0Erred {
        #[doc = "Snooze Mode returns by UART0 reception communication error occurrence interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Snooze Mode returns by UART0 reception communication error occurrence interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iica0Ed_SPEC;
    pub type Iica0Ed = crate::EnumBitfieldStruct<u8, Iica0Ed_SPEC>;
    impl Iica0Ed {
        #[doc = "Software Standby/Snooze Mode returns by IICA0 address match interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by IICA0 address match interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uart0Rxed_SPEC;
    pub type Uart0Rxed = crate::EnumBitfieldStruct<u8, Uart0Rxed_SPEC>;
    impl Uart0Rxed {
        #[doc = "Snooze Mode returns by UART0 reception transfer end interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Snooze Mode returns by UART0 reception transfer end interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adc12Ed_SPEC;
    pub type Adc12Ed = crate::EnumBitfieldStruct<u8, Adc12Ed_SPEC>;
    impl Adc12Ed {
        #[doc = "Snooze Mode returns by End of A/D conversion interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Snooze Mode returns by End of A/D conversion interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbyedcr1_SPEC;
impl crate::sealed::RegSpec for Sbyedcr1_SPEC {
    type DataType = u32;
}

#[doc = "Software Standby/Snooze End Control Register 1"]
pub type Sbyedcr1 = crate::RegValueT<Sbyedcr1_SPEC>;

impl Sbyedcr1 {
    #[doc = "RTC Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn rtced(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sbyedcr1::Rtced,
        sbyedcr1::Rtced,
        Sbyedcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sbyedcr1::Rtced,
            sbyedcr1::Rtced,
            Sbyedcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interval Signal of 32-bit Interval Timer Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn itled(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sbyedcr1::Itled,
        sbyedcr1::Itled,
        Sbyedcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sbyedcr1::Itled,
            sbyedcr1::Itled,
            Sbyedcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "UARTA0 Reception Communication Error Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn ure0ed(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sbyedcr1::Ure0Ed,
        sbyedcr1::Ure0Ed,
        Sbyedcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sbyedcr1::Ure0Ed,
            sbyedcr1::Ure0Ed,
            Sbyedcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "UARTA0 Transmission Transfer End or Buffer Empty Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn ut0ed(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sbyedcr1::Ut0Ed,
        sbyedcr1::Ut0Ed,
        Sbyedcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sbyedcr1::Ut0Ed,
            sbyedcr1::Ut0Ed,
            Sbyedcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "UARTA0 Reception Transfer End Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn ur0ed(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        sbyedcr1::Ur0Ed,
        sbyedcr1::Ur0Ed,
        Sbyedcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            sbyedcr1::Ur0Ed,
            sbyedcr1::Ur0Ed,
            Sbyedcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sbyedcr1 {
    #[inline(always)]
    fn default() -> Sbyedcr1 {
        <crate::RegValueT<Sbyedcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sbyedcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtced_SPEC;
    pub type Rtced = crate::EnumBitfieldStruct<u8, Rtced_SPEC>;
    impl Rtced {
        #[doc = "Software Standby/Snooze Mode returns by RTC interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by RTC interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Itled_SPEC;
    pub type Itled = crate::EnumBitfieldStruct<u8, Itled_SPEC>;
    impl Itled {
        #[doc = "Software Standby/Snooze Mode returns by Interval signal of 32-bit interval timer interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by Interval signal of 32-bit interval timer interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ure0Ed_SPEC;
    pub type Ure0Ed = crate::EnumBitfieldStruct<u8, Ure0Ed_SPEC>;
    impl Ure0Ed {
        #[doc = "Software Standby/Snooze Mode returns by UARTA0 reception communication error interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by UARTA0 reception communication error interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ut0Ed_SPEC;
    pub type Ut0Ed = crate::EnumBitfieldStruct<u8, Ut0Ed_SPEC>;
    impl Ut0Ed {
        #[doc = "Software Standby/Snooze Mode returns by UARTA0 transmission transfer end or buffer empty interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by UARTA0 transmission transfer end or buffer empty interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ur0Ed_SPEC;
    pub type Ur0Ed = crate::EnumBitfieldStruct<u8, Ur0Ed_SPEC>;
    impl Ur0Ed {
        #[doc = "Software Standby/Snooze Mode returns by UARTA0 reception transfer end interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by UARTA0 reception transfer end interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
}
