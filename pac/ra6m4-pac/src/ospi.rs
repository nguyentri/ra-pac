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
// Generated from SVD 1.50.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:51:51 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Octa Serial Peripheral Interface"]
unsafe impl ::core::marker::Send for super::Ospi {}
unsafe impl ::core::marker::Sync for super::Ospi {}
impl super::Ospi {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Device Command Register"]
    #[inline(always)]
    pub const fn dcr(&self) -> &'static crate::common::Reg<self::Dcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Device Address Register"]
    #[inline(always)]
    pub const fn dar(&self) -> &'static crate::common::Reg<self::Dar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Device Command Setting Register"]
    #[inline(always)]
    pub const fn dcsr(&self) -> &'static crate::common::Reg<self::Dcsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dcsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Device Size Register 0"]
    #[inline(always)]
    pub const fn dsr0(&self) -> &'static crate::common::Reg<self::Dsr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Device Size Register 1"]
    #[inline(always)]
    pub const fn dsr1(&self) -> &'static crate::common::Reg<self::Dsr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Memory Delay Trim Register"]
    #[inline(always)]
    pub const fn mdtr(&self) -> &'static crate::common::Reg<self::Mdtr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mdtr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Auto-Calibration Timer Register"]
    #[inline(always)]
    pub const fn actr(&self) -> &'static crate::common::Reg<self::Actr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Actr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Auto-Calibration Address Register 0"]
    #[inline(always)]
    pub const fn acar0(&self) -> &'static crate::common::Reg<self::Acar0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Acar0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Auto-Calibration Address Register 1"]
    #[inline(always)]
    pub const fn acar1(&self) -> &'static crate::common::Reg<self::Acar1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Acar1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Device Memory Map Read Chip Select Timing Setting Register"]
    #[inline(always)]
    pub const fn drcstr(
        &self,
    ) -> &'static crate::common::Reg<self::Drcstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Drcstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Device Memory Map Write Chip Select Timing Setting Register"]
    #[inline(always)]
    pub const fn dwcstr(
        &self,
    ) -> &'static crate::common::Reg<self::Dwcstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dwcstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Device Chip Select Timing Setting Register"]
    #[inline(always)]
    pub const fn dcstr(&self) -> &'static crate::common::Reg<self::Dcstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dcstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Controller and Device Setting Register"]
    #[inline(always)]
    pub const fn cdsr(&self) -> &'static crate::common::Reg<self::Cdsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Memory Map Dummy Length Register"]
    #[inline(always)]
    pub const fn mdlr(&self) -> &'static crate::common::Reg<self::Mdlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mdlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Memory Map Read/Write Command Register 0"]
    #[inline(always)]
    pub const fn mrwcr0(
        &self,
    ) -> &'static crate::common::Reg<self::Mrwcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrwcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "Memory Map Read/Write Command Register 1"]
    #[inline(always)]
    pub const fn mrwcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Mrwcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrwcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "Memory Map Read/Write Setting Register"]
    #[inline(always)]
    pub const fn mrwcsr(
        &self,
    ) -> &'static crate::common::Reg<self::Mrwcsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrwcsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Error Status Register"]
    #[inline(always)]
    pub const fn esr(&self) -> &'static crate::common::Reg<self::Esr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Esr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Configure Write without Data Register"]
    #[inline(always)]
    pub const fn cwndr(&self) -> &'static crate::common::Reg<self::Cwndr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Cwndr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "Configure Write Data Register"]
    #[inline(always)]
    pub const fn cwdr(&self) -> &'static crate::common::Reg<self::Cwdr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Cwdr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "Configure Read Register"]
    #[inline(always)]
    pub const fn crr(&self) -> &'static crate::common::Reg<self::Crr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Crr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Auto-Calibration Status Register"]
    #[inline(always)]
    pub const fn acsr(&self) -> &'static crate::common::Reg<self::Acsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Acsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "Device Chip Select Maximum Period Register"]
    #[inline(always)]
    pub const fn dcsmxr(
        &self,
    ) -> &'static crate::common::Reg<self::Dcsmxr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dcsmxr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = "Device Memory Map Write single continuous translating size Register"]
    #[inline(always)]
    pub const fn dwsctsr(
        &self,
    ) -> &'static crate::common::Reg<self::Dwsctsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dwsctsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcr_SPEC;
impl crate::sealed::RegSpec for Dcr_SPEC {
    type DataType = u32;
}

#[doc = "Device Command Register"]
pub type Dcr = crate::RegValueT<Dcr_SPEC>;

impl Dcr {
    #[doc = "Device Command data"]
    #[inline(always)]
    pub fn dvcmd0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device Command data"]
    #[inline(always)]
    pub fn dvcmd1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Dcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dcr {
    #[inline(always)]
    fn default() -> Dcr {
        <crate::RegValueT<Dcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dar_SPEC;
impl crate::sealed::RegSpec for Dar_SPEC {
    type DataType = u32;
}

#[doc = "Device Address Register"]
pub type Dar = crate::RegValueT<Dar_SPEC>;

impl Dar {
    #[doc = "Device Address data 0"]
    #[inline(always)]
    pub fn dvad0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device Address data 1"]
    #[inline(always)]
    pub fn dvad1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Dar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device Address data 2"]
    #[inline(always)]
    pub fn dvad2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Dar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Dar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device Address data 3"]
    #[inline(always)]
    pub fn dvad3(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Dar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Dar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dar {
    #[inline(always)]
    fn default() -> Dar {
        <crate::RegValueT<Dar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcsr_SPEC;
impl crate::sealed::RegSpec for Dcsr_SPEC {
    type DataType = u32;
}

#[doc = "Device Command Setting Register"]
pub type Dcsr = crate::RegValueT<Dcsr_SPEC>;

impl Dcsr {
    #[doc = "Transfer data length setting"]
    #[inline(always)]
    pub fn dalen(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dcsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Dummy cycle setting"]
    #[inline(always)]
    pub fn dmlen(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Dcsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Access Device setting"]
    #[inline(always)]
    pub fn acdv(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        dcsr::Acdv,
        dcsr::Acdv,
        Dcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            dcsr::Acdv,
            dcsr::Acdv,
            Dcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transfer command length setting"]
    #[inline(always)]
    pub fn cmdlen(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, Dcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,Dcsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data order setting"]
    #[inline(always)]
    pub fn daor(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        dcsr::Daor,
        dcsr::Daor,
        Dcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            dcsr::Daor,
            dcsr::Daor,
            Dcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transfer address length setting"]
    #[inline(always)]
    pub fn adlen(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, u8, Dcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8,u8,Dcsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DOPI single byte setting"]
    #[inline(always)]
    pub fn dopi(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        dcsr::Dopi,
        dcsr::Dopi,
        Dcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            dcsr::Dopi,
            dcsr::Dopi,
            Dcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Access Control"]
    #[inline(always)]
    pub fn acda(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        dcsr::Acda,
        dcsr::Acda,
        Dcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            dcsr::Acda,
            dcsr::Acda,
            Dcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Preamble bit enable for OctaRAM"]
    #[inline(always)]
    pub fn pren(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        dcsr::Pren,
        dcsr::Pren,
        Dcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            dcsr::Pren,
            dcsr::Pren,
            Dcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dcsr {
    #[inline(always)]
    fn default() -> Dcsr {
        <crate::RegValueT<Dcsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dcsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acdv_SPEC;
    pub type Acdv = crate::EnumBitfieldStruct<u8, Acdv_SPEC>;
    impl Acdv {
        #[doc = "Send commands to device 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Send commands to device 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Daor_SPEC;
    pub type Daor = crate::EnumBitfieldStruct<u8, Daor_SPEC>;
    impl Daor {
        #[doc = "byte0, byte1, byte2, byte3"]
        pub const _0: Self = Self::new(0);

        #[doc = "byte1, byte0, byte3, byte2"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dopi_SPEC;
    pub type Dopi = crate::EnumBitfieldStruct<u8, Dopi_SPEC>;
    impl Dopi {
        #[doc = "Each cycle has two bytes data. (normal DOPI mode)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Each cycle has one byte data. (The byte data changes at the rising edge of the clock and does not change at the falling edge of the clock.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acda_SPEC;
    pub type Acda = crate::EnumBitfieldStruct<u8, Acda_SPEC>;
    impl Acda {
        #[doc = "Register access Do not arrange the transfer data."]
        pub const _0: Self = Self::new(0);

        #[doc = "Data access"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pren_SPEC;
    pub type Pren = crate::EnumBitfieldStruct<u8, Pren_SPEC>;
    impl Pren {
        #[doc = "No check preamble bit from OctaRAM"]
        pub const _0: Self = Self::new(0);

        #[doc = "Check preamble bit from OctaRAM"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsr0_SPEC;
impl crate::sealed::RegSpec for Dsr0_SPEC {
    type DataType = u32;
}

#[doc = "Device Size Register 0"]
pub type Dsr0 = crate::RegValueT<Dsr0_SPEC>;

impl Dsr0 {
    #[doc = "Device 0 size setting"]
    #[inline(always)]
    pub fn dv0sz(
        self,
    ) -> crate::common::RegisterField<0, 0x3fffffff, 1, 0, u32, u32, Dsr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3fffffff,1,0,u32,u32,Dsr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 0 type setting"]
    #[inline(always)]
    pub fn dv0typ(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        dsr0::Dv0Typ,
        dsr0::Dv0Typ,
        Dsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            dsr0::Dv0Typ,
            dsr0::Dv0Typ,
            Dsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsr0 {
    #[inline(always)]
    fn default() -> Dsr0 {
        <crate::RegValueT<Dsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dv0Typ_SPEC;
    pub type Dv0Typ = crate::EnumBitfieldStruct<u8, Dv0Typ_SPEC>;
    impl Dv0Typ {
        #[doc = "Flash on device 0"]
        pub const _00: Self = Self::new(0);

        #[doc = "RAM on device 0"]
        pub const _01: Self = Self::new(1);

        #[doc = "No connection on device 0"]
        pub const _10: Self = Self::new(2);

        #[doc = "Forbidden"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsr1_SPEC;
impl crate::sealed::RegSpec for Dsr1_SPEC {
    type DataType = u32;
}

#[doc = "Device Size Register 1"]
pub type Dsr1 = crate::RegValueT<Dsr1_SPEC>;

impl Dsr1 {
    #[doc = "Device 1 size setting"]
    #[inline(always)]
    pub fn dv1sz(
        self,
    ) -> crate::common::RegisterField<0, 0x3fffffff, 1, 0, u32, u32, Dsr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3fffffff,1,0,u32,u32,Dsr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 1 type setting"]
    #[inline(always)]
    pub fn dv1typ(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        dsr1::Dv1Typ,
        dsr1::Dv1Typ,
        Dsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            dsr1::Dv1Typ,
            dsr1::Dv1Typ,
            Dsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsr1 {
    #[inline(always)]
    fn default() -> Dsr1 {
        <crate::RegValueT<Dsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dsr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dv1Typ_SPEC;
    pub type Dv1Typ = crate::EnumBitfieldStruct<u8, Dv1Typ_SPEC>;
    impl Dv1Typ {
        #[doc = "Flash on device 1"]
        pub const _00: Self = Self::new(0);

        #[doc = "RAM on device 1"]
        pub const _01: Self = Self::new(1);

        #[doc = "No connection on device 1"]
        pub const _10: Self = Self::new(2);

        #[doc = "Forbidden"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdtr_SPEC;
impl crate::sealed::RegSpec for Mdtr_SPEC {
    type DataType = u32;
}

#[doc = "Memory Delay Trim Register"]
pub type Mdtr = crate::RegValueT<Mdtr_SPEC>;

impl Mdtr {
    #[doc = "Device 0 delay setting"]
    #[inline(always)]
    pub fn dv0del(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Mdtr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Mdtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "OM_DQS enable counter"]
    #[inline(always)]
    pub fn dqseram(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Mdtr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Mdtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "OM_DQS enable counter"]
    #[inline(always)]
    pub fn dqsesopi(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Mdtr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Mdtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 1 delay setting"]
    #[inline(always)]
    pub fn dv1del(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Mdtr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Mdtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "OM_DQS enable counter"]
    #[inline(always)]
    pub fn dqsedopi(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Mdtr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Mdtr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mdtr {
    #[inline(always)]
    fn default() -> Mdtr {
        <crate::RegValueT<Mdtr_SPEC> as RegisterValue<_>>::new(100701184)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Actr_SPEC;
impl crate::sealed::RegSpec for Actr_SPEC {
    type DataType = u32;
}

#[doc = "Auto-Calibration Timer Register"]
pub type Actr = crate::RegValueT<Actr_SPEC>;

impl Actr {
    #[doc = "Automatic calibration cycle time setting"]
    #[inline(always)]
    pub fn ctp(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Actr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Actr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Actr {
    #[inline(always)]
    fn default() -> Actr {
        <crate::RegValueT<Actr_SPEC> as RegisterValue<_>>::new(268435456)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acar0_SPEC;
impl crate::sealed::RegSpec for Acar0_SPEC {
    type DataType = u32;
}

#[doc = "Auto-Calibration Address Register 0"]
pub type Acar0 = crate::RegValueT<Acar0_SPEC>;

impl Acar0 {
    #[doc = "Automatic calibration address"]
    #[inline(always)]
    pub fn cad0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Acar0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Acar0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Acar0 {
    #[inline(always)]
    fn default() -> Acar0 {
        <crate::RegValueT<Acar0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acar1_SPEC;
impl crate::sealed::RegSpec for Acar1_SPEC {
    type DataType = u32;
}

#[doc = "Auto-Calibration Address Register 1"]
pub type Acar1 = crate::RegValueT<Acar1_SPEC>;

impl Acar1 {
    #[doc = "Automatic calibration address"]
    #[inline(always)]
    pub fn cad1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Acar1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Acar1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Acar1 {
    #[inline(always)]
    fn default() -> Acar1 {
        <crate::RegValueT<Acar1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Drcstr_SPEC;
impl crate::sealed::RegSpec for Drcstr_SPEC {
    type DataType = u32;
}

#[doc = "Device Memory Map Read Chip Select Timing Setting Register"]
pub type Drcstr = crate::RegValueT<Drcstr_SPEC>;

impl Drcstr {
    #[doc = "Device 0 single continuous read waiting cycle setting in PCLKA units"]
    #[inline(always)]
    pub fn ctrw0(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Drcstr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Drcstr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 0 single continuous read mode setting"]
    #[inline(always)]
    pub fn ctr0(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        drcstr::Ctr0,
        drcstr::Ctr0,
        Drcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            drcstr::Ctr0,
            drcstr::Ctr0,
            Drcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 0 Command execution interval setting"]
    #[inline(always)]
    pub fn dvrdcmd0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        drcstr::Dvrdcmd0,
        drcstr::Dvrdcmd0,
        Drcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            drcstr::Dvrdcmd0,
            drcstr::Dvrdcmd0,
            Drcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 0 select signal pull-up timing setting"]
    #[inline(always)]
    pub fn dvrdhi0(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x7,
        1,
        0,
        drcstr::Dvrdhi0,
        drcstr::Dvrdhi0,
        Drcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x7,
            1,
            0,
            drcstr::Dvrdhi0,
            drcstr::Dvrdhi0,
            Drcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 0 select signal pull-down timing setting"]
    #[inline(always)]
    pub fn dvrdlo0(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        drcstr::Dvrdlo0,
        drcstr::Dvrdlo0,
        Drcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            drcstr::Dvrdlo0,
            drcstr::Dvrdlo0,
            Drcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 1 single continuous read waiting cycle setting in PCLKA units"]
    #[inline(always)]
    pub fn ctrw1(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, Drcstr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,Drcstr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 1 single continuous read mode setting"]
    #[inline(always)]
    pub fn ctr1(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        drcstr::Ctr1,
        drcstr::Ctr1,
        Drcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            drcstr::Ctr1,
            drcstr::Ctr1,
            Drcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 1 Command execution interval"]
    #[inline(always)]
    pub fn dvrdcmd1(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        drcstr::Dvrdcmd1,
        drcstr::Dvrdcmd1,
        Drcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            drcstr::Dvrdcmd1,
            drcstr::Dvrdcmd1,
            Drcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 1 select signal High timing setting"]
    #[inline(always)]
    pub fn dvrdhi1(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x7,
        1,
        0,
        drcstr::Dvrdhi1,
        drcstr::Dvrdhi1,
        Drcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x7,
            1,
            0,
            drcstr::Dvrdhi1,
            drcstr::Dvrdhi1,
            Drcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 1 select signal pull-down timing setting"]
    #[inline(always)]
    pub fn dvrdlo1(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        drcstr::Dvrdlo1,
        drcstr::Dvrdlo1,
        Drcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            drcstr::Dvrdlo1,
            drcstr::Dvrdlo1,
            Drcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Drcstr {
    #[inline(always)]
    fn default() -> Drcstr {
        <crate::RegValueT<Drcstr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod drcstr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctr0_SPEC;
    pub type Ctr0 = crate::EnumBitfieldStruct<u8, Ctr0_SPEC>;
    impl Ctr0 {
        #[doc = "Single continuous read mode is disabled for device 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Single continuous read mode is enabled for device 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvrdcmd0_SPEC;
    pub type Dvrdcmd0 = crate::EnumBitfieldStruct<u8, Dvrdcmd0_SPEC>;
    impl Dvrdcmd0 {
        #[doc = "2 clock cycles"]
        pub const _000: Self = Self::new(0);

        #[doc = "5 clock cycles"]
        pub const _001: Self = Self::new(1);

        #[doc = "7 clock cycles"]
        pub const _010: Self = Self::new(2);

        #[doc = "9 clock cycles"]
        pub const _011: Self = Self::new(3);

        #[doc = "11 clock cycles"]
        pub const _100: Self = Self::new(4);

        #[doc = "13 clock cycles"]
        pub const _101: Self = Self::new(5);

        #[doc = "15 clock cycles"]
        pub const _110: Self = Self::new(6);

        #[doc = "17 clock cycles"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvrdhi0_SPEC;
    pub type Dvrdhi0 = crate::EnumBitfieldStruct<u8, Dvrdhi0_SPEC>;
    impl Dvrdhi0 {
        #[doc = "Setting prohibit"]
        pub const _000: Self = Self::new(0);

        #[doc = "Setting prohibit"]
        pub const _001: Self = Self::new(1);

        #[doc = "Setting prohibit"]
        pub const _010: Self = Self::new(2);

        #[doc = "Setting prohibit (DOPI mode) 5 clock cycles (Other mode)"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibit (DOPI mode) 6 clock cycles (Other mode)"]
        pub const _100: Self = Self::new(4);

        #[doc = "6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
        pub const _101: Self = Self::new(5);

        #[doc = "7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
        pub const _110: Self = Self::new(6);

        #[doc = "8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvrdlo0_SPEC;
    pub type Dvrdlo0 = crate::EnumBitfieldStruct<u8, Dvrdlo0_SPEC>;
    impl Dvrdlo0 {
        #[doc = "Setting prohibit"]
        pub const _00: Self = Self::new(0);

        #[doc = "2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
        pub const _01: Self = Self::new(1);

        #[doc = "3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
        pub const _10: Self = Self::new(2);

        #[doc = "4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctr1_SPEC;
    pub type Ctr1 = crate::EnumBitfieldStruct<u8, Ctr1_SPEC>;
    impl Ctr1 {
        #[doc = "Single continuous read mode is disabled for device 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "Single continuous read mode is enabled for device 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvrdcmd1_SPEC;
    pub type Dvrdcmd1 = crate::EnumBitfieldStruct<u8, Dvrdcmd1_SPEC>;
    impl Dvrdcmd1 {
        #[doc = "2 clock cycles"]
        pub const _000: Self = Self::new(0);

        #[doc = "5 clock cycles"]
        pub const _001: Self = Self::new(1);

        #[doc = "7 clock cycles"]
        pub const _010: Self = Self::new(2);

        #[doc = "9 clock cycles"]
        pub const _011: Self = Self::new(3);

        #[doc = "11 clock cycles"]
        pub const _100: Self = Self::new(4);

        #[doc = "13 clock cycles"]
        pub const _101: Self = Self::new(5);

        #[doc = "15 clock cycles"]
        pub const _110: Self = Self::new(6);

        #[doc = "17 clock cycles"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvrdhi1_SPEC;
    pub type Dvrdhi1 = crate::EnumBitfieldStruct<u8, Dvrdhi1_SPEC>;
    impl Dvrdhi1 {
        #[doc = "Setting prohibit"]
        pub const _000: Self = Self::new(0);

        #[doc = "Setting prohibit"]
        pub const _001: Self = Self::new(1);

        #[doc = "Setting prohibit"]
        pub const _010: Self = Self::new(2);

        #[doc = "Setting prohibit (DOPI mode) 5 clock cycles (Other mode)"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibit (DOPI mode) 6 clock cycles (Other mode)"]
        pub const _100: Self = Self::new(4);

        #[doc = "6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
        pub const _101: Self = Self::new(5);

        #[doc = "7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
        pub const _110: Self = Self::new(6);

        #[doc = "8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvrdlo1_SPEC;
    pub type Dvrdlo1 = crate::EnumBitfieldStruct<u8, Dvrdlo1_SPEC>;
    impl Dvrdlo1 {
        #[doc = "Setting prohibited"]
        pub const _00: Self = Self::new(0);

        #[doc = "2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
        pub const _01: Self = Self::new(1);

        #[doc = "3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
        pub const _10: Self = Self::new(2);

        #[doc = "4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dwcstr_SPEC;
impl crate::sealed::RegSpec for Dwcstr_SPEC {
    type DataType = u32;
}

#[doc = "Device Memory Map Write Chip Select Timing Setting Register"]
pub type Dwcstr = crate::RegValueT<Dwcstr_SPEC>;

impl Dwcstr {
    #[doc = "Device 0 single continuous write waiting cycle setting in PCLKA units"]
    #[inline(always)]
    pub fn ctww0(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Dwcstr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Dwcstr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 0 single continuous write mode setting"]
    #[inline(always)]
    pub fn ctw0(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dwcstr::Ctw0,
        dwcstr::Ctw0,
        Dwcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dwcstr::Ctw0,
            dwcstr::Ctw0,
            Dwcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 0 Command execution interval setting"]
    #[inline(always)]
    pub fn dvwcmd0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        dwcstr::Dvwcmd0,
        dwcstr::Dvwcmd0,
        Dwcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            dwcstr::Dvwcmd0,
            dwcstr::Dvwcmd0,
            Dwcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 0 select signal pull-up timing setting"]
    #[inline(always)]
    pub fn dvwhi0(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x7,
        1,
        0,
        dwcstr::Dvwhi0,
        dwcstr::Dvwhi0,
        Dwcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x7,
            1,
            0,
            dwcstr::Dvwhi0,
            dwcstr::Dvwhi0,
            Dwcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 0 select signal pull-down timing setting"]
    #[inline(always)]
    pub fn dvwlo0(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        dwcstr::Dvwlo0,
        dwcstr::Dvwlo0,
        Dwcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            dwcstr::Dvwlo0,
            dwcstr::Dvwlo0,
            Dwcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 1 single continuous write waiting cycle setting in PCLKA units"]
    #[inline(always)]
    pub fn ctww1(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, Dwcstr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,Dwcstr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 1 single continuous write mode setting"]
    #[inline(always)]
    pub fn ctw1(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        dwcstr::Ctw1,
        dwcstr::Ctw1,
        Dwcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            dwcstr::Ctw1,
            dwcstr::Ctw1,
            Dwcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 1 Command execution interval setting"]
    #[inline(always)]
    pub fn dvwcmd1(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        dwcstr::Dvwcmd1,
        dwcstr::Dvwcmd1,
        Dwcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            dwcstr::Dvwcmd1,
            dwcstr::Dvwcmd1,
            Dwcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 1 select signal pull-up timing setting"]
    #[inline(always)]
    pub fn dvwhi1(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x7,
        1,
        0,
        dwcstr::Dvwhi1,
        dwcstr::Dvwhi1,
        Dwcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x7,
            1,
            0,
            dwcstr::Dvwhi1,
            dwcstr::Dvwhi1,
            Dwcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 1 select signal pull-down timing setting"]
    #[inline(always)]
    pub fn dvwlo1(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        dwcstr::Dvwlo1,
        dwcstr::Dvwlo1,
        Dwcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            dwcstr::Dvwlo1,
            dwcstr::Dvwlo1,
            Dwcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dwcstr {
    #[inline(always)]
    fn default() -> Dwcstr {
        <crate::RegValueT<Dwcstr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dwcstr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctw0_SPEC;
    pub type Ctw0 = crate::EnumBitfieldStruct<u8, Ctw0_SPEC>;
    impl Ctw0 {
        #[doc = "Single continuous write mode is disabled for device 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Single continuous write mode is enabled for device 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvwcmd0_SPEC;
    pub type Dvwcmd0 = crate::EnumBitfieldStruct<u8, Dvwcmd0_SPEC>;
    impl Dvwcmd0 {
        #[doc = "2 clock cycles"]
        pub const _000: Self = Self::new(0);

        #[doc = "5 clock cycles"]
        pub const _001: Self = Self::new(1);

        #[doc = "7 clock cycles"]
        pub const _010: Self = Self::new(2);

        #[doc = "9 clock cycles"]
        pub const _011: Self = Self::new(3);

        #[doc = "11 clock cycles"]
        pub const _100: Self = Self::new(4);

        #[doc = "13 clock cycles"]
        pub const _101: Self = Self::new(5);

        #[doc = "15 clock cycles"]
        pub const _110: Self = Self::new(6);

        #[doc = "17 clock cycles"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvwhi0_SPEC;
    pub type Dvwhi0 = crate::EnumBitfieldStruct<u8, Dvwhi0_SPEC>;
    impl Dvwhi0 {
        #[doc = "1.5 clock cycles (DOPI mode) 2 clock cycles (Other mode)"]
        pub const _000: Self = Self::new(0);

        #[doc = "2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
        pub const _001: Self = Self::new(1);

        #[doc = "3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
        pub const _010: Self = Self::new(2);

        #[doc = "4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
        pub const _011: Self = Self::new(3);

        #[doc = "5.5 clock cycles (DOPI mode) 6 clock cycles (Other mode)"]
        pub const _100: Self = Self::new(4);

        #[doc = "6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
        pub const _101: Self = Self::new(5);

        #[doc = "7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
        pub const _110: Self = Self::new(6);

        #[doc = "8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvwlo0_SPEC;
    pub type Dvwlo0 = crate::EnumBitfieldStruct<u8, Dvwlo0_SPEC>;
    impl Dvwlo0 {
        #[doc = "Setting prohibit"]
        pub const _00: Self = Self::new(0);

        #[doc = "2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
        pub const _01: Self = Self::new(1);

        #[doc = "3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
        pub const _10: Self = Self::new(2);

        #[doc = "4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctw1_SPEC;
    pub type Ctw1 = crate::EnumBitfieldStruct<u8, Ctw1_SPEC>;
    impl Ctw1 {
        #[doc = "Single continuous write mode is disabled for device 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Single continuous write mode is enabled for device 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvwcmd1_SPEC;
    pub type Dvwcmd1 = crate::EnumBitfieldStruct<u8, Dvwcmd1_SPEC>;
    impl Dvwcmd1 {
        #[doc = "Setting prohibited"]
        pub const _000: Self = Self::new(0);

        #[doc = "5 clock cycles"]
        pub const _001: Self = Self::new(1);

        #[doc = "7 clock cycles"]
        pub const _010: Self = Self::new(2);

        #[doc = "9 clock cycles"]
        pub const _011: Self = Self::new(3);

        #[doc = "11 clock cycles"]
        pub const _100: Self = Self::new(4);

        #[doc = "13 clock cycles"]
        pub const _101: Self = Self::new(5);

        #[doc = "15 clock cycles"]
        pub const _110: Self = Self::new(6);

        #[doc = "17 clock cycles"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvwhi1_SPEC;
    pub type Dvwhi1 = crate::EnumBitfieldStruct<u8, Dvwhi1_SPEC>;
    impl Dvwhi1 {
        #[doc = "1.5 clock cycles (DOPI mode) 2 clock cycles (Other mode)"]
        pub const _000: Self = Self::new(0);

        #[doc = "2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
        pub const _001: Self = Self::new(1);

        #[doc = "3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
        pub const _010: Self = Self::new(2);

        #[doc = "4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
        pub const _011: Self = Self::new(3);

        #[doc = "5.5 clock cycles (DOPI mode) 6 clock cycles (Other mode)"]
        pub const _100: Self = Self::new(4);

        #[doc = "6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
        pub const _101: Self = Self::new(5);

        #[doc = "7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
        pub const _110: Self = Self::new(6);

        #[doc = "8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvwlo1_SPEC;
    pub type Dvwlo1 = crate::EnumBitfieldStruct<u8, Dvwlo1_SPEC>;
    impl Dvwlo1 {
        #[doc = "Setting prohibit"]
        pub const _00: Self = Self::new(0);

        #[doc = "2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
        pub const _01: Self = Self::new(1);

        #[doc = "3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
        pub const _10: Self = Self::new(2);

        #[doc = "4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcstr_SPEC;
impl crate::sealed::RegSpec for Dcstr_SPEC {
    type DataType = u32;
}

#[doc = "Device Chip Select Timing Setting Register"]
pub type Dcstr = crate::RegValueT<Dcstr_SPEC>;

impl Dcstr {
    #[doc = "Device Command execution interval setting"]
    #[inline(always)]
    pub fn dvselcmd(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        dcstr::Dvselcmd,
        dcstr::Dvselcmd,
        Dcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            dcstr::Dvselcmd,
            dcstr::Dvselcmd,
            Dcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device select signal pull-up timing setting"]
    #[inline(always)]
    pub fn dvselhi(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x7,
        1,
        0,
        dcstr::Dvselhi,
        dcstr::Dvselhi,
        Dcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x7,
            1,
            0,
            dcstr::Dvselhi,
            dcstr::Dvselhi,
            Dcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device select signal pull-down timing setting"]
    #[inline(always)]
    pub fn dvsello(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        dcstr::Dvsello,
        dcstr::Dvsello,
        Dcstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            dcstr::Dvsello,
            dcstr::Dvsello,
            Dcstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dcstr {
    #[inline(always)]
    fn default() -> Dcstr {
        <crate::RegValueT<Dcstr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dcstr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvselcmd_SPEC;
    pub type Dvselcmd = crate::EnumBitfieldStruct<u8, Dvselcmd_SPEC>;
    impl Dvselcmd {
        #[doc = "2 clock cycles"]
        pub const _000: Self = Self::new(0);

        #[doc = "5 clock cycles"]
        pub const _001: Self = Self::new(1);

        #[doc = "7 clock cycles"]
        pub const _010: Self = Self::new(2);

        #[doc = "9 clock cycles"]
        pub const _011: Self = Self::new(3);

        #[doc = "11 clock cycles"]
        pub const _100: Self = Self::new(4);

        #[doc = "13 clock cycles"]
        pub const _101: Self = Self::new(5);

        #[doc = "15 clock cycles"]
        pub const _110: Self = Self::new(6);

        #[doc = "17 clock cycles"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvselhi_SPEC;
    pub type Dvselhi = crate::EnumBitfieldStruct<u8, Dvselhi_SPEC>;
    impl Dvselhi {
        #[doc = "Setting prohibited"]
        pub const _000: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _001: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _010: Self = Self::new(2);

        #[doc = "Setting prohibited (DOPI mode) 5 clock cycles (Other mode)"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibited (DOPI mode) 6 clock cycles (Other mode)"]
        pub const _100: Self = Self::new(4);

        #[doc = "6.5 clock cycles (DOPI mode) 7 clock cycles (Other mode)"]
        pub const _101: Self = Self::new(5);

        #[doc = "7.5 clock cycles (DOPI mode) 8 clock cycles (Other mode)"]
        pub const _110: Self = Self::new(6);

        #[doc = "8.5 clock cycles (DOPI mode) 9 clock cycles (Other mode)"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvsello_SPEC;
    pub type Dvsello = crate::EnumBitfieldStruct<u8, Dvsello_SPEC>;
    impl Dvsello {
        #[doc = "Setting prohibit"]
        pub const _00: Self = Self::new(0);

        #[doc = "2.5 clock cycles (DOPI mode) 3 clock cycles (Other mode)"]
        pub const _01: Self = Self::new(1);

        #[doc = "3.5 clock cycles (DOPI mode) 4 clock cycles (Other mode)"]
        pub const _10: Self = Self::new(2);

        #[doc = "4.5 clock cycles (DOPI mode) 5 clock cycles (Other mode)"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdsr_SPEC;
impl crate::sealed::RegSpec for Cdsr_SPEC {
    type DataType = u32;
}

#[doc = "Controller and Device Setting Register"]
pub type Cdsr = crate::RegValueT<Cdsr_SPEC>;

impl Cdsr {
    #[doc = "Device0_transfer_type setting"]
    #[inline(always)]
    pub fn dv0ttyp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        cdsr::Dv0Ttyp,
        cdsr::Dv0Ttyp,
        Cdsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cdsr::Dv0Ttyp,
            cdsr::Dv0Ttyp,
            Cdsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device1_transfer_type setting"]
    #[inline(always)]
    pub fn dv1ttyp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        cdsr::Dv1Ttyp,
        cdsr::Dv1Ttyp,
        Cdsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            cdsr::Dv1Ttyp,
            cdsr::Dv1Ttyp,
            Cdsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device0_memory pre-cycle setting"]
    #[inline(always)]
    pub fn dv0pc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cdsr::Dv0Pc,
        cdsr::Dv0Pc,
        Cdsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cdsr::Dv0Pc,
            cdsr::Dv0Pc,
            Cdsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device1_memory pre-cycle setting"]
    #[inline(always)]
    pub fn dv1pc(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cdsr::Dv1Pc,
        cdsr::Dv1Pc,
        Cdsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cdsr::Dv1Pc,
            cdsr::Dv1Pc,
            Cdsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Automatic calibration memory enable setting for device 0"]
    #[inline(always)]
    pub fn acmeme0(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        cdsr::Acmeme0,
        cdsr::Acmeme0,
        Cdsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cdsr::Acmeme0,
            cdsr::Acmeme0,
            Cdsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Automatic calibration memory enable setting for device 1"]
    #[inline(always)]
    pub fn acmeme1(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        cdsr::Acmeme1,
        cdsr::Acmeme1,
        Cdsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            cdsr::Acmeme1,
            cdsr::Acmeme1,
            Cdsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Automatic calibration mode"]
    #[inline(always)]
    pub fn acmode(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        cdsr::Acmode,
        cdsr::Acmode,
        Cdsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            cdsr::Acmode,
            cdsr::Acmode,
            Cdsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deadlock Free Timer Enable"]
    #[inline(always)]
    pub fn dlft(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cdsr::Dlft,
        cdsr::Dlft,
        Cdsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cdsr::Dlft,
            cdsr::Dlft,
            Cdsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cdsr {
    #[inline(always)]
    fn default() -> Cdsr {
        <crate::RegValueT<Cdsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cdsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dv0Ttyp_SPEC;
    pub type Dv0Ttyp = crate::EnumBitfieldStruct<u8, Dv0Ttyp_SPEC>;
    impl Dv0Ttyp {
        #[doc = "SPI mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "SOPI mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "DOPI mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dv1Ttyp_SPEC;
    pub type Dv1Ttyp = crate::EnumBitfieldStruct<u8, Dv1Ttyp_SPEC>;
    impl Dv1Ttyp {
        #[doc = "SPI mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "SOPI mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "DOPI mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dv0Pc_SPEC;
    pub type Dv0Pc = crate::EnumBitfieldStruct<u8, Dv0Pc_SPEC>;
    impl Dv0Pc {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dv1Pc_SPEC;
    pub type Dv1Pc = crate::EnumBitfieldStruct<u8, Dv1Pc_SPEC>;
    impl Dv1Pc {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acmeme0_SPEC;
    pub type Acmeme0 = crate::EnumBitfieldStruct<u8, Acmeme0_SPEC>;
    impl Acmeme0 {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acmeme1_SPEC;
    pub type Acmeme1 = crate::EnumBitfieldStruct<u8, Acmeme1_SPEC>;
    impl Acmeme1 {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acmode_SPEC;
    pub type Acmode = crate::EnumBitfieldStruct<u8, Acmode_SPEC>;
    impl Acmode {
        #[doc = "Automatic calibration is disabled"]
        pub const _00: Self = Self::new(0);

        #[doc = "Automatic calibration is enabled and modify MDTR"]
        pub const _01: Self = Self::new(1);

        #[doc = "Automatic calibration immediately is executed for all trim code, but it will not modify MDTR"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlft_SPEC;
    pub type Dlft = crate::EnumBitfieldStruct<u8, Dlft_SPEC>;
    impl Dlft {
        #[doc = "Enable timer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable timer"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdlr_SPEC;
impl crate::sealed::RegSpec for Mdlr_SPEC {
    type DataType = u32;
}

#[doc = "Memory Map Dummy Length Register"]
pub type Mdlr = crate::RegValueT<Mdlr_SPEC>;

impl Mdlr {
    #[doc = "Device 0 Read dummy length setting"]
    #[inline(always)]
    pub fn dv0rdl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Mdlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Mdlr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 0 Write dummy length setting"]
    #[inline(always)]
    pub fn dv0wdl(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mdlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mdlr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 1 Read dummy length setting"]
    #[inline(always)]
    pub fn dv1rdl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Mdlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Mdlr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 1 Write dummy length setting"]
    #[inline(always)]
    pub fn dv1wdl(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Mdlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Mdlr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mdlr {
    #[inline(always)]
    fn default() -> Mdlr {
        <crate::RegValueT<Mdlr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrwcr0_SPEC;
impl crate::sealed::RegSpec for Mrwcr0_SPEC {
    type DataType = u32;
}

#[doc = "Memory Map Read/Write Command Register 0"]
pub type Mrwcr0 = crate::RegValueT<Mrwcr0_SPEC>;

impl Mrwcr0 {
    #[doc = "Memory map read command 0 setting"]
    #[inline(always)]
    pub fn d0mrcmd0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Mrwcr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Mrwcr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Memory map read command 1 setting"]
    #[inline(always)]
    pub fn d0mrcmd1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mrwcr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mrwcr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Memory map write command 0 setting"]
    #[inline(always)]
    pub fn d0mwcmd0(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Mrwcr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Mrwcr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Memory map write command 1 setting"]
    #[inline(always)]
    pub fn d0mwcmd1(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Mrwcr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Mrwcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrwcr0 {
    #[inline(always)]
    fn default() -> Mrwcr0 {
        <crate::RegValueT<Mrwcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrwcr1_SPEC;
impl crate::sealed::RegSpec for Mrwcr1_SPEC {
    type DataType = u32;
}

#[doc = "Memory Map Read/Write Command Register 1"]
pub type Mrwcr1 = crate::RegValueT<Mrwcr1_SPEC>;

impl Mrwcr1 {
    #[doc = "Memory map read command 0 setting"]
    #[inline(always)]
    pub fn d1mrcmd0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Mrwcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Mrwcr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Memory map read command 1 setting"]
    #[inline(always)]
    pub fn d1mrcmd1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mrwcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mrwcr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Memory map write command 0 setting"]
    #[inline(always)]
    pub fn d1mwcmd0(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Mrwcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Mrwcr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Memory map write command 1 setting"]
    #[inline(always)]
    pub fn d1mwcmd1(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Mrwcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Mrwcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrwcr1 {
    #[inline(always)]
    fn default() -> Mrwcr1 {
        <crate::RegValueT<Mrwcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrwcsr_SPEC;
impl crate::sealed::RegSpec for Mrwcsr_SPEC {
    type DataType = u32;
}

#[doc = "Memory Map Read/Write Setting Register"]
pub type Mrwcsr = crate::RegValueT<Mrwcsr_SPEC>;

impl Mrwcsr {
    #[doc = "Device 0 read address length setting"]
    #[inline(always)]
    pub fn mral0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Mrwcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Mrwcsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 0 read command length setting"]
    #[inline(always)]
    pub fn mrcl0(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, Mrwcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,Mrwcsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 0 read order setting"]
    #[inline(always)]
    pub fn mro0(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mrwcsr::Mro0,
        mrwcsr::Mro0,
        Mrwcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mrwcsr::Mro0,
            mrwcsr::Mro0,
            Mrwcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Preamble bit enable for mem0 memory-map read"]
    #[inline(always)]
    pub fn pren0(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mrwcsr::Pren0,
        mrwcsr::Pren0,
        Mrwcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mrwcsr::Pren0,
            mrwcsr::Pren0,
            Mrwcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 0 write address length setting"]
    #[inline(always)]
    pub fn mwal0(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Mrwcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Mrwcsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 0 write command length setting"]
    #[inline(always)]
    pub fn mwcl0(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, Mrwcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,Mrwcsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 0 write order setting"]
    #[inline(always)]
    pub fn mwo0(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        mrwcsr::Mwo0,
        mrwcsr::Mwo0,
        Mrwcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            mrwcsr::Mwo0,
            mrwcsr::Mwo0,
            Mrwcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 1 read address length setting"]
    #[inline(always)]
    pub fn mral1(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Mrwcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Mrwcsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 1 read command length setting"]
    #[inline(always)]
    pub fn mrcl1(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, u8, u8, Mrwcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x7,1,0,u8,u8,Mrwcsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 1 read order setting"]
    #[inline(always)]
    pub fn mro1(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mrwcsr::Mro1,
        mrwcsr::Mro1,
        Mrwcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mrwcsr::Mro1,
            mrwcsr::Mro1,
            Mrwcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Preamble bit enable for mem1 memory-map read"]
    #[inline(always)]
    pub fn pren1(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mrwcsr::Pren1,
        mrwcsr::Pren1,
        Mrwcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mrwcsr::Pren1,
            mrwcsr::Pren1,
            Mrwcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device 1 write address length setting"]
    #[inline(always)]
    pub fn mwal1(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, u8, Mrwcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8,u8,Mrwcsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 1 write command length setting"]
    #[inline(always)]
    pub fn mwcl1(
        self,
    ) -> crate::common::RegisterField<27, 0x7, 1, 0, u8, u8, Mrwcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x7,1,0,u8,u8,Mrwcsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device 1 write order setting"]
    #[inline(always)]
    pub fn mwo1(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        mrwcsr::Mwo1,
        mrwcsr::Mwo1,
        Mrwcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            mrwcsr::Mwo1,
            mrwcsr::Mwo1,
            Mrwcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mrwcsr {
    #[inline(always)]
    fn default() -> Mrwcsr {
        <crate::RegValueT<Mrwcsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrwcsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mro0_SPEC;
    pub type Mro0 = crate::EnumBitfieldStruct<u8, Mro0_SPEC>;
    impl Mro0 {
        #[doc = "Read order is byte0, byte1, byte2, byte3."]
        pub const _0: Self = Self::new(0);

        #[doc = "Read order is byte1, byte0, byte3, byte2."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pren0_SPEC;
    pub type Pren0 = crate::EnumBitfieldStruct<u8, Pren0_SPEC>;
    impl Pren0 {
        #[doc = "No check preamble bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Check preamble bit from OctaFlash (if OctaFlash is connected to device 0)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mwo0_SPEC;
    pub type Mwo0 = crate::EnumBitfieldStruct<u8, Mwo0_SPEC>;
    impl Mwo0 {
        #[doc = "Write order is byte0, byte1, byte2, byte3."]
        pub const _0: Self = Self::new(0);

        #[doc = "Write order is byte1, byte0, byte3, byte2."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mro1_SPEC;
    pub type Mro1 = crate::EnumBitfieldStruct<u8, Mro1_SPEC>;
    impl Mro1 {
        #[doc = "Read order is byte0, byte1, byte2, byte3."]
        pub const _0: Self = Self::new(0);

        #[doc = "Read order is byte1, byte0, byte3, byte2."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pren1_SPEC;
    pub type Pren1 = crate::EnumBitfieldStruct<u8, Pren1_SPEC>;
    impl Pren1 {
        #[doc = "No check preamble bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Check preamble bit from OctaFlash (if OctaFlash is connected to device 1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mwo1_SPEC;
    pub type Mwo1 = crate::EnumBitfieldStruct<u8, Mwo1_SPEC>;
    impl Mwo1 {
        #[doc = "Write order is byte0, byte1, byte2, byte3."]
        pub const _0: Self = Self::new(0);

        #[doc = "Write order is byte1, byte0, byte3, byte2."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esr_SPEC;
impl crate::sealed::RegSpec for Esr_SPEC {
    type DataType = u32;
}

#[doc = "Error Status Register"]
pub type Esr = crate::RegValueT<Esr_SPEC>;

impl Esr {
    #[doc = "Memory map read error status"]
    #[inline(always)]
    pub fn mresr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        esr::Mresr,
        esr::Mresr,
        Esr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            esr::Mresr,
            esr::Mresr,
            Esr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Memory map write error status"]
    #[inline(always)]
    pub fn mwesr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        esr::Mwesr,
        esr::Mwesr,
        Esr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            esr::Mwesr,
            esr::Mwesr,
            Esr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Esr {
    #[inline(always)]
    fn default() -> Esr {
        <crate::RegValueT<Esr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod esr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mresr_SPEC;
    pub type Mresr = crate::EnumBitfieldStruct<u8, Mresr_SPEC>;
    impl Mresr {
        #[doc = "ECC error"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "Preamble error"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "Wait OM_DQS timeout"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "Invalid command"]
        pub const _0_X_80: Self = Self::new(128);

        #[doc = "Reserved"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mwesr_SPEC;
    pub type Mwesr = crate::EnumBitfieldStruct<u8, Mwesr_SPEC>;
    impl Mwesr {
        #[doc = "Invalid command"]
        pub const _0_X_80: Self = Self::new(128);

        #[doc = "Reserved"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cwndr_SPEC;
impl crate::sealed::RegSpec for Cwndr_SPEC {
    type DataType = u32;
}

#[doc = "Configure Write without Data Register"]
pub type Cwndr = crate::RegValueT<Cwndr_SPEC>;

impl Cwndr {
    #[doc = "The write value should be 0."]
    #[inline(always)]
    pub fn wnd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cwndr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cwndr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cwndr {
    #[inline(always)]
    fn default() -> Cwndr {
        <crate::RegValueT<Cwndr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cwdr_SPEC;
impl crate::sealed::RegSpec for Cwdr_SPEC {
    type DataType = u32;
}

#[doc = "Configure Write Data Register"]
pub type Cwdr = crate::RegValueT<Cwdr_SPEC>;

impl Cwdr {
    #[doc = "Write data 0"]
    #[inline(always)]
    pub fn wd0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cwdr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cwdr_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Write data 1"]
    #[inline(always)]
    pub fn wd1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cwdr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cwdr_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Write data 2"]
    #[inline(always)]
    pub fn wd2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cwdr_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cwdr_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Write data 3"]
    #[inline(always)]
    pub fn wd3(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cwdr_SPEC, crate::common::W> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cwdr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cwdr {
    #[inline(always)]
    fn default() -> Cwdr {
        <crate::RegValueT<Cwdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crr_SPEC;
impl crate::sealed::RegSpec for Crr_SPEC {
    type DataType = u32;
}

#[doc = "Configure Read Register"]
pub type Crr = crate::RegValueT<Crr_SPEC>;

impl Crr {
    #[doc = "Read data 0"]
    #[inline(always)]
    pub fn rd0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Crr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Crr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Read data 1"]
    #[inline(always)]
    pub fn rd1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Crr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Crr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Read data 2"]
    #[inline(always)]
    pub fn rd2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Crr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Crr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Read data 3"]
    #[inline(always)]
    pub fn rd3(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Crr_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Crr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Crr {
    #[inline(always)]
    fn default() -> Crr {
        <crate::RegValueT<Crr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acsr_SPEC;
impl crate::sealed::RegSpec for Acsr_SPEC {
    type DataType = u32;
}

#[doc = "Auto-Calibration Status Register"]
pub type Acsr = crate::RegValueT<Acsr_SPEC>;

impl Acsr {
    #[doc = "Auto-calibration status of device 0"]
    #[inline(always)]
    pub fn acsr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        acsr::Acsr0,
        acsr::Acsr0,
        Acsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            acsr::Acsr0,
            acsr::Acsr0,
            Acsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Auto-calibration status of device 1"]
    #[inline(always)]
    pub fn acsr1(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x7,
        1,
        0,
        acsr::Acsr1,
        acsr::Acsr1,
        Acsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            acsr::Acsr1,
            acsr::Acsr1,
            Acsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Acsr {
    #[inline(always)]
    fn default() -> Acsr {
        <crate::RegValueT<Acsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod acsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acsr0_SPEC;
    pub type Acsr0 = crate::EnumBitfieldStruct<u8, Acsr0_SPEC>;
    impl Acsr0 {
        #[doc = "Initial state"]
        pub const _000: Self = Self::new(0);

        #[doc = "Reserved"]
        pub const _001: Self = Self::new(1);

        #[doc = "Reserved"]
        pub const _010: Self = Self::new(2);

        #[doc = "Normal end"]
        pub const _011: Self = Self::new(3);

        #[doc = "Error end"]
        pub const _100: Self = Self::new(4);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acsr1_SPEC;
    pub type Acsr1 = crate::EnumBitfieldStruct<u8, Acsr1_SPEC>;
    impl Acsr1 {
        #[doc = "Initial state"]
        pub const _000: Self = Self::new(0);

        #[doc = "Reserved"]
        pub const _001: Self = Self::new(1);

        #[doc = "Reserved"]
        pub const _010: Self = Self::new(2);

        #[doc = "Normal end"]
        pub const _011: Self = Self::new(3);

        #[doc = "Error end"]
        pub const _100: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcsmxr_SPEC;
impl crate::sealed::RegSpec for Dcsmxr_SPEC {
    type DataType = u32;
}

#[doc = "Device Chip Select Maximum Period Register"]
pub type Dcsmxr = crate::RegValueT<Dcsmxr_SPEC>;

impl Dcsmxr {
    #[doc = "Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous write of OctaRAM."]
    #[inline(always)]
    pub fn ctwmx0(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Dcsmxr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Dcsmxr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Indicates the maximum period that OM_CS0 and OM_CS1 are Low in single continuous read of OctaRAM."]
    #[inline(always)]
    pub fn ctwmx1(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Dcsmxr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Dcsmxr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dcsmxr {
    #[inline(always)]
    fn default() -> Dcsmxr {
        <crate::RegValueT<Dcsmxr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dwsctsr_SPEC;
impl crate::sealed::RegSpec for Dwsctsr_SPEC {
    type DataType = u32;
}

#[doc = "Device Memory Map Write single continuous translating size Register"]
pub type Dwsctsr = crate::RegValueT<Dwsctsr_SPEC>;

impl Dwsctsr {
    #[doc = "Indicates the number of bytes to translate in single continuous write of device 0."]
    #[inline(always)]
    pub fn ctsn0(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Dwsctsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Dwsctsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Indicates the number of bytes to translate in single continuous write of device 1."]
    #[inline(always)]
    pub fn ctsn1(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, Dwsctsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,Dwsctsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dwsctsr {
    #[inline(always)]
    fn default() -> Dwsctsr {
        <crate::RegValueT<Dwsctsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
