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
// Generated from SVD 1.0, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:52:24 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CRC Calculator"]
unsafe impl ::core::marker::Send for super::Crc {}
unsafe impl ::core::marker::Sync for super::Crc {}
impl super::Crc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "CRC Control Register0"]
    #[inline(always)]
    pub const fn crccr0(
        &self,
    ) -> &'static crate::common::Reg<self::Crccr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crccr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "CRC Control Register1"]
    #[inline(always)]
    pub const fn crccr1(
        &self,
    ) -> &'static crate::common::Reg<self::Crccr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crccr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "CRC Data Input Register"]
    #[inline(always)]
    pub const fn crcdir(
        &self,
    ) -> &'static crate::common::Reg<self::Crcdir_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crcdir_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "CRC Data Input Register (byte access)"]
    #[inline(always)]
    pub const fn crcdir_by(
        &self,
    ) -> &'static crate::common::Reg<self::CrcdirBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CrcdirBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "CRC Data Output Register"]
    #[inline(always)]
    pub const fn crcdor(
        &self,
    ) -> &'static crate::common::Reg<self::Crcdor_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crcdor_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "CRC Data Output Register (halfword access)"]
    #[inline(always)]
    pub const fn crcdor_ha(
        &self,
    ) -> &'static crate::common::Reg<self::CrcdorHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CrcdorHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "CRC Data Output Register(byte access)"]
    #[inline(always)]
    pub const fn crcdor_by(
        &self,
    ) -> &'static crate::common::Reg<self::CrcdorBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CrcdorBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Snoop Address Register"]
    #[inline(always)]
    pub const fn crcsar(
        &self,
    ) -> &'static crate::common::Reg<self::Crcsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crcsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crccr0_SPEC;
impl crate::sealed::RegSpec for Crccr0_SPEC {
    type DataType = u8;
}

#[doc = "CRC Control Register0"]
pub type Crccr0 = crate::RegValueT<Crccr0_SPEC>;

impl Crccr0 {
    #[doc = "CRCDOR Register Clear"]
    #[inline(always)]
    pub fn dorclr(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        crccr0::Dorclr,
        crccr0::Dorclr,
        Crccr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            crccr0::Dorclr,
            crccr0::Dorclr,
            Crccr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "CRC Calculation Switching"]
    #[inline(always)]
    pub fn lms(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        crccr0::Lms,
        crccr0::Lms,
        Crccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            crccr0::Lms,
            crccr0::Lms,
            Crccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CRC Generating Polynomial Switching"]
    #[inline(always)]
    pub fn gps(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        crccr0::Gps,
        crccr0::Gps,
        Crccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            crccr0::Gps,
            crccr0::Gps,
            Crccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Crccr0 {
    #[inline(always)]
    fn default() -> Crccr0 {
        <crate::RegValueT<Crccr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod crccr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dorclr_SPEC;
    pub type Dorclr = crate::EnumBitfieldStruct<u8, Dorclr_SPEC>;
    impl Dorclr {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the CRCDOR register."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lms_SPEC;
    pub type Lms = crate::EnumBitfieldStruct<u8, Lms_SPEC>;
    impl Lms {
        #[doc = "Generates CRC for LSB first communication."]
        pub const _0: Self = Self::new(0);

        #[doc = "Generates CRC for MSB first communication."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gps_SPEC;
    pub type Gps = crate::EnumBitfieldStruct<u8, Gps_SPEC>;
    impl Gps {
        #[doc = "No calculation is executed."]
        pub const _000: Self = Self::new(0);

        #[doc = "8-bit CRC-8 (X8 + X2 + X + 1)"]
        pub const _001: Self = Self::new(1);

        #[doc = "16-bit CRC-16 (X16 + X15 + X2 + 1)"]
        pub const _010: Self = Self::new(2);

        #[doc = "16-bit CRC-CCITT (X16 + X12 + X5 + 1)"]
        pub const _011: Self = Self::new(3);

        #[doc = "32-bit CRC-32 (X32+X26+X23+X22+X16+X12+X11+X10+X8+X7+X5+X4+X2+X+1)"]
        pub const _100: Self = Self::new(4);

        #[doc = "32-bit CRC-32C (X32+X28+X27+X26+ X25+X23+X22+X20+X19+X18+X14+X13+X11+X10+X9+X8+X6+1)"]
        pub const _101: Self = Self::new(5);

        #[doc = "No calculation is executed."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crccr1_SPEC;
impl crate::sealed::RegSpec for Crccr1_SPEC {
    type DataType = u8;
}

#[doc = "CRC Control Register1"]
pub type Crccr1 = crate::RegValueT<Crccr1_SPEC>;

impl Crccr1 {
    #[doc = "Snoop enable bit"]
    #[inline(always)]
    pub fn crcsen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        crccr1::Crcsen,
        crccr1::Crcsen,
        Crccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            crccr1::Crcsen,
            crccr1::Crcsen,
            Crccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Snoop-on-write/read switch bit"]
    #[inline(always)]
    pub fn crcswr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        crccr1::Crcswr,
        crccr1::Crcswr,
        Crccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            crccr1::Crcswr,
            crccr1::Crcswr,
            Crccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Crccr1 {
    #[inline(always)]
    fn default() -> Crccr1 {
        <crate::RegValueT<Crccr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod crccr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crcsen_SPEC;
    pub type Crcsen = crate::EnumBitfieldStruct<u8, Crcsen_SPEC>;
    impl Crcsen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crcswr_SPEC;
    pub type Crcswr = crate::EnumBitfieldStruct<u8, Crcswr_SPEC>;
    impl Crcswr {
        #[doc = "Snoop-on-read"]
        pub const _0: Self = Self::new(0);

        #[doc = "Snoop-on-write"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcdir_SPEC;
impl crate::sealed::RegSpec for Crcdir_SPEC {
    type DataType = u32;
}

#[doc = "CRC Data Input Register"]
pub type Crcdir = crate::RegValueT<Crcdir_SPEC>;

impl Crcdir {
    #[doc = "Calculation input Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    pub fn crcdir(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Crcdir_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Crcdir_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Crcdir {
    #[inline(always)]
    fn default() -> Crcdir {
        <crate::RegValueT<Crcdir_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrcdirBy_SPEC;
impl crate::sealed::RegSpec for CrcdirBy_SPEC {
    type DataType = u8;
}

#[doc = "CRC Data Input Register (byte access)"]
pub type CrcdirBy = crate::RegValueT<CrcdirBy_SPEC>;

impl CrcdirBy {
    #[doc = "Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    pub fn crcdir_by(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, CrcdirBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,CrcdirBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CrcdirBy {
    #[inline(always)]
    fn default() -> CrcdirBy {
        <crate::RegValueT<CrcdirBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcdor_SPEC;
impl crate::sealed::RegSpec for Crcdor_SPEC {
    type DataType = u32;
}

#[doc = "CRC Data Output Register"]
pub type Crcdor = crate::RegValueT<Crcdor_SPEC>;

impl Crcdor {
    #[doc = "Calculation output Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    pub fn crcdor(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Crcdor_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Crcdor_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Crcdor {
    #[inline(always)]
    fn default() -> Crcdor {
        <crate::RegValueT<Crcdor_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrcdorHa_SPEC;
impl crate::sealed::RegSpec for CrcdorHa_SPEC {
    type DataType = u16;
}

#[doc = "CRC Data Output Register (halfword access)"]
pub type CrcdorHa = crate::RegValueT<CrcdorHa_SPEC>;

impl CrcdorHa {
    #[doc = "Calculation output Data (Case of CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    pub fn crcdor_ha(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, CrcdorHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,CrcdorHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CrcdorHa {
    #[inline(always)]
    fn default() -> CrcdorHa {
        <crate::RegValueT<CrcdorHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrcdorBy_SPEC;
impl crate::sealed::RegSpec for CrcdorBy_SPEC {
    type DataType = u8;
}

#[doc = "CRC Data Output Register(byte access)"]
pub type CrcdorBy = crate::RegValueT<CrcdorBy_SPEC>;

impl CrcdorBy {
    #[doc = "Calculation output Data (Case of CRC-8 )"]
    #[inline(always)]
    pub fn crcdor_by(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, CrcdorBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,CrcdorBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CrcdorBy {
    #[inline(always)]
    fn default() -> CrcdorBy {
        <crate::RegValueT<CrcdorBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcsar_SPEC;
impl crate::sealed::RegSpec for Crcsar_SPEC {
    type DataType = u16;
}

#[doc = "Snoop Address Register"]
pub type Crcsar = crate::RegValueT<Crcsar_SPEC>;

impl Crcsar {
    #[doc = "snoop address bitSet the I/O register address to snoop"]
    #[inline(always)]
    pub fn crcsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        crcsar::Crcsa,
        crcsar::Crcsa,
        Crcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            crcsar::Crcsa,
            crcsar::Crcsa,
            Crcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Crcsar {
    #[inline(always)]
    fn default() -> Crcsar {
        <crate::RegValueT<Crcsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod crcsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crcsa_SPEC;
    pub type Crcsa = crate::EnumBitfieldStruct<u16, Crcsa_SPEC>;
    impl Crcsa {
        #[doc = "SCI0.TDR"]
        pub const _0_X_0003: Self = Self::new(3);

        #[doc = "SCI0.RDR"]
        pub const _0_X_0005: Self = Self::new(5);

        #[doc = "SCI1.TDR"]
        pub const _0_X_0023: Self = Self::new(35);

        #[doc = "SCI1.RDR"]
        pub const _0_X_0025: Self = Self::new(37);

        #[doc = "SCI2.TDR"]
        pub const _0_X_0043: Self = Self::new(67);

        #[doc = "SCI2.RDR"]
        pub const _0_X_0045: Self = Self::new(69);

        #[doc = "SCI3.TDR"]
        pub const _0_X_0063: Self = Self::new(99);

        #[doc = "SCI3.RDR"]
        pub const _0_X_0065: Self = Self::new(101);

        #[doc = "SCI4.TDR"]
        pub const _0_X_0083: Self = Self::new(131);

        #[doc = "SCI4.RDR"]
        pub const _0_X_0085: Self = Self::new(133);

        #[doc = "SCI5.TDR"]
        pub const _0_X_00_A_3: Self = Self::new(163);

        #[doc = "SCI5.RDR"]
        pub const _0_X_00_A_5: Self = Self::new(165);

        #[doc = "SCI6.TDR"]
        pub const _0_X_00_C_3: Self = Self::new(195);

        #[doc = "SCI6.RDR"]
        pub const _0_X_00_C_5: Self = Self::new(197);

        #[doc = "SCI7.TDR"]
        pub const _0_X_00_E_3: Self = Self::new(227);

        #[doc = "SCI7.RDR"]
        pub const _0_X_00_E_5: Self = Self::new(229);

        #[doc = "SCI8.TDR"]
        pub const _0_X_0103: Self = Self::new(259);

        #[doc = "SCI8.RDR"]
        pub const _0_X_0105: Self = Self::new(261);

        #[doc = "SCI9.TDR"]
        pub const _0_X_0123: Self = Self::new(291);

        #[doc = "SCI9.RDR"]
        pub const _0_X_0125: Self = Self::new(293);

        #[doc = "Settings other than above are prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
}
