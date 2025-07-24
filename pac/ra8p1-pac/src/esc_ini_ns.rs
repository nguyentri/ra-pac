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
#[doc = r"Initial Configuration 1 for EtherCAT Slave Controller"]
unsafe impl ::core::marker::Send for super::EscIniNs {}
unsafe impl ::core::marker::Sync for super::EscIniNs {}
impl super::EscIniNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "ESC Reset Control Register"]
    #[inline(always)]
    pub const fn escrst(
        &self,
    ) -> &'static crate::common::Reg<self::Escrst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Escrst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "PHY LINK Polarity Setting Register"]
    #[inline(always)]
    pub const fn phylink(
        &self,
    ) -> &'static crate::common::Reg<self::Phylink_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Phylink_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "ESC Interrupt Control Register"]
    #[inline(always)]
    pub const fn escicr(
        &self,
    ) -> &'static crate::common::Reg<self::Escicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Escicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "EtherCAT PHY Offset Address Register"]
    #[inline(always)]
    pub const fn ecatoffadr(
        &self,
    ) -> &'static crate::common::Reg<self::Ecatoffadr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecatoffadr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "EtherCAT Operation Mode Register"]
    #[inline(always)]
    pub const fn ecatopmod(
        &self,
    ) -> &'static crate::common::Reg<self::Ecatopmod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecatopmod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "EtherCAT Debug Control Register"]
    #[inline(always)]
    pub const fn ecatdbgc(
        &self,
    ) -> &'static crate::common::Reg<self::Ecatdbgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecatdbgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escrst_SPEC;
impl crate::sealed::RegSpec for Escrst_SPEC {
    type DataType = u32;
}

#[doc = "ESC Reset Control Register"]
pub type Escrst = crate::RegValueT<Escrst_SPEC>;

impl Escrst {
    #[doc = "ESC Reset"]
    #[inline(always)]
    pub fn escrst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        escrst::Escrst,
        escrst::Escrst,
        Escrst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            escrst::Escrst,
            escrst::Escrst,
            Escrst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PHY Reset"]
    #[inline(always)]
    pub fn phyrst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        escrst::Phyrst,
        escrst::Phyrst,
        Escrst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            escrst::Phyrst,
            escrst::Phyrst,
            Escrst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Escrst {
    #[inline(always)]
    fn default() -> Escrst {
        <crate::RegValueT<Escrst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod escrst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Escrst_SPEC;
    pub type Escrst = crate::EnumBitfieldStruct<u8, Escrst_SPEC>;
    impl Escrst {
        #[doc = "Reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset is released"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Phyrst_SPEC;
    pub type Phyrst = crate::EnumBitfieldStruct<u8, Phyrst_SPEC>;
    impl Phyrst {
        #[doc = "Reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset is released"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Phylink_SPEC;
impl crate::sealed::RegSpec for Phylink_SPEC {
    type DataType = u32;
}

#[doc = "PHY LINK Polarity Setting Register"]
pub type Phylink = crate::RegValueT<Phylink_SPEC>;

impl Phylink {
    #[doc = "CAT0_LINKSTA Pin Polarity"]
    #[inline(always)]
    pub fn linkpol0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        phylink::Linkpol0,
        phylink::Linkpol0,
        Phylink_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            phylink::Linkpol0,
            phylink::Linkpol0,
            Phylink_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CAT1_LINKSTA Pin Polarity"]
    #[inline(always)]
    pub fn linkpol1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        phylink::Linkpol1,
        phylink::Linkpol1,
        Phylink_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            phylink::Linkpol1,
            phylink::Linkpol1,
            Phylink_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Phylink {
    #[inline(always)]
    fn default() -> Phylink {
        <crate::RegValueT<Phylink_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod phylink {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Linkpol0_SPEC;
    pub type Linkpol0 = crate::EnumBitfieldStruct<u8, Linkpol0_SPEC>;
    impl Linkpol0 {
        #[doc = "Active high"]
        pub const _0: Self = Self::new(0);

        #[doc = "Active Low"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Linkpol1_SPEC;
    pub type Linkpol1 = crate::EnumBitfieldStruct<u8, Linkpol1_SPEC>;
    impl Linkpol1 {
        #[doc = "Active high"]
        pub const _0: Self = Self::new(0);

        #[doc = "Active Low"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Escicr_SPEC;
impl crate::sealed::RegSpec for Escicr_SPEC {
    type DataType = u32;
}

#[doc = "ESC Interrupt Control Register"]
pub type Escicr = crate::RegValueT<Escicr_SPEC>;

impl Escicr {
    #[inline(always)]
    pub fn sync0c(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        escicr::Sync0C,
        escicr::Sync0C,
        Escicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            escicr::Sync0C,
            escicr::Sync0C,
            Escicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sync1c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        escicr::Sync1C,
        escicr::Sync1C,
        Escicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            escicr::Sync1C,
            escicr::Sync1C,
            Escicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn escic(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        escicr::Escic,
        escicr::Escic,
        Escicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            escicr::Escic,
            escicr::Escic,
            Escicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Select DC Latch Trigger 0 for ESC"]
    #[inline(always)]
    pub fn lat0s(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        escicr::Lat0S,
        escicr::Lat0S,
        Escicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            escicr::Lat0S,
            escicr::Lat0S,
            Escicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Select DC Latch Trigger 1 for ESC"]
    #[inline(always)]
    pub fn lat1s(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        escicr::Lat1S,
        escicr::Lat1S,
        Escicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            escicr::Lat1S,
            escicr::Lat1S,
            Escicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Escicr {
    #[inline(always)]
    fn default() -> Escicr {
        <crate::RegValueT<Escicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod escicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sync0C_SPEC;
    pub type Sync0C = crate::EnumBitfieldStruct<u8, Sync0C_SPEC>;
    impl Sync0C {
        #[doc = "An interrupt is generated at the rising edge of the SYNC0 signal."]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is generated at the falling edge of the SYNC0 signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sync1C_SPEC;
    pub type Sync1C = crate::EnumBitfieldStruct<u8, Sync1C_SPEC>;
    impl Sync1C {
        #[doc = "An interrupt is generated at the rising edge of the SYNC1 signal."]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is generated at the falling edge of the SYNC1 signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Escic_SPEC;
    pub type Escic = crate::EnumBitfieldStruct<u8, Escic_SPEC>;
    impl Escic {
        #[doc = "An interrupt is generated when the PDI_IRQ is 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is generated when the PDI_IRQ is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lat0S_SPEC;
    pub type Lat0S = crate::EnumBitfieldStruct<u8, Lat0S_SPEC>;
    impl Lat0S {
        #[doc = "Select LATCH0 input from external"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select LATCH0 input from ELC"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lat1S_SPEC;
    pub type Lat1S = crate::EnumBitfieldStruct<u8, Lat1S_SPEC>;
    impl Lat1S {
        #[doc = "Select LATCH1 input from external"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select LATCH1 input from ELC"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecatoffadr_SPEC;
impl crate::sealed::RegSpec for Ecatoffadr_SPEC {
    type DataType = u32;
}

#[doc = "EtherCAT PHY Offset Address Register"]
pub type Ecatoffadr = crate::RegValueT<Ecatoffadr_SPEC>;

impl Ecatoffadr {
    #[inline(always)]
    pub fn oadd(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Ecatoffadr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Ecatoffadr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ecatoffadr {
    #[inline(always)]
    fn default() -> Ecatoffadr {
        <crate::RegValueT<Ecatoffadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecatopmod_SPEC;
impl crate::sealed::RegSpec for Ecatopmod_SPEC {
    type DataType = u32;
}

#[doc = "EtherCAT Operation Mode Register"]
pub type Ecatopmod = crate::RegValueT<Ecatopmod_SPEC>;

impl Ecatopmod {
    #[inline(always)]
    pub fn eepromsize(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ecatopmod::Eepromsize,
        ecatopmod::Eepromsize,
        Ecatopmod_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecatopmod::Eepromsize,
            ecatopmod::Eepromsize,
            Ecatopmod_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ecatopmod {
    #[inline(always)]
    fn default() -> Ecatopmod {
        <crate::RegValueT<Ecatopmod_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecatopmod {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eepromsize_SPEC;
    pub type Eepromsize = crate::EnumBitfieldStruct<u8, Eepromsize_SPEC>;
    impl Eepromsize {
        #[doc = "16 Kbits or less"]
        pub const _0: Self = Self::new(0);

        #[doc = "32 Kbits to 4 Mbits"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecatdbgc_SPEC;
impl crate::sealed::RegSpec for Ecatdbgc_SPEC {
    type DataType = u32;
}

#[doc = "EtherCAT Debug Control Register"]
pub type Ecatdbgc = crate::RegValueT<Ecatdbgc_SPEC>;

impl Ecatdbgc {
    #[doc = "Port 0 TX Signal Delay Setting"]
    #[inline(always)]
    pub fn txsft0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        ecatdbgc::Txsft0,
        ecatdbgc::Txsft0,
        Ecatdbgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            ecatdbgc::Txsft0,
            ecatdbgc::Txsft0,
            Ecatdbgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port 1 TX Signal Delay Setting"]
    #[inline(always)]
    pub fn txsft1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        ecatdbgc::Txsft1,
        ecatdbgc::Txsft1,
        Ecatdbgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            ecatdbgc::Txsft1,
            ecatdbgc::Txsft1,
            Ecatdbgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ecatdbgc {
    #[inline(always)]
    fn default() -> Ecatdbgc {
        <crate::RegValueT<Ecatdbgc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecatdbgc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txsft0_SPEC;
    pub type Txsft0 = crate::EnumBitfieldStruct<u8, Txsft0_SPEC>;
    impl Txsft0 {
        #[doc = "0 ns"]
        pub const _00: Self = Self::new(0);

        #[doc = "10 ns"]
        pub const _01: Self = Self::new(1);

        #[doc = "20 ns"]
        pub const _10: Self = Self::new(2);

        #[doc = "30 ns"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txsft1_SPEC;
    pub type Txsft1 = crate::EnumBitfieldStruct<u8, Txsft1_SPEC>;
    impl Txsft1 {
        #[doc = "0 ns"]
        pub const _00: Self = Self::new(0);

        #[doc = "10 ns"]
        pub const _01: Self = Self::new(1);

        #[doc = "20 ns"]
        pub const _10: Self = Self::new(2);

        #[doc = "30 ns"]
        pub const _11: Self = Self::new(3);
    }
}
