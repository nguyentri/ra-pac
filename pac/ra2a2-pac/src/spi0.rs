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
// Generated from SVD 1.20.02, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:46:19 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Serial Peripheral Interface"]
unsafe impl ::core::marker::Send for super::Spi0 {}
unsafe impl ::core::marker::Sync for super::Spi0 {}
impl super::Spi0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "SPI Control Register"]
    #[inline(always)]
    pub const fn spcr(&self) -> &'static crate::common::Reg<self::Spcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "SPI Slave Select Polarity Register"]
    #[inline(always)]
    pub const fn sslp(&self) -> &'static crate::common::Reg<self::Sslp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sslp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "SPI Pin Control Register"]
    #[inline(always)]
    pub const fn sppcr(&self) -> &'static crate::common::Reg<self::Sppcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sppcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "SPI Status Register"]
    #[inline(always)]
    pub const fn spsr(&self) -> &'static crate::common::Reg<self::Spsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "SPI Data Register"]
    #[inline(always)]
    pub const fn spdr(&self) -> &'static crate::common::Reg<self::Spdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "SPI Data Register"]
    #[inline(always)]
    pub const fn spdr_ha(
        &self,
    ) -> &'static crate::common::Reg<self::SpdrHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpdrHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "SPI Data Register"]
    #[inline(always)]
    pub const fn spdr_by(
        &self,
    ) -> &'static crate::common::Reg<self::SpdrBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpdrBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "SPI Bit Rate Register"]
    #[inline(always)]
    pub const fn spbr(&self) -> &'static crate::common::Reg<self::Spbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "SPI Data Control Register"]
    #[inline(always)]
    pub const fn spdcr(&self) -> &'static crate::common::Reg<self::Spdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[doc = "SPI Clock Delay Register"]
    #[inline(always)]
    pub const fn spckd(&self) -> &'static crate::common::Reg<self::Spckd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spckd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "SPI Slave Select Negation Delay Register"]
    #[inline(always)]
    pub const fn sslnd(&self) -> &'static crate::common::Reg<self::Sslnd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sslnd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[doc = "SPI Next-Access Delay Register"]
    #[inline(always)]
    pub const fn spnd(&self) -> &'static crate::common::Reg<self::Spnd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spnd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "SPI Control Register 2"]
    #[inline(always)]
    pub const fn spcr2(&self) -> &'static crate::common::Reg<self::Spcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(15usize),
            )
        }
    }

    #[doc = "SPI Command Register 0"]
    #[inline(always)]
    pub const fn spcmd0(
        &self,
    ) -> &'static crate::common::Reg<self::Spcmd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spcmd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcr_SPEC;
impl crate::sealed::RegSpec for Spcr_SPEC {
    type DataType = u8;
}

#[doc = "SPI Control Register"]
pub type Spcr = crate::RegValueT<Spcr_SPEC>;

impl Spcr {
    #[doc = "SPI Mode Select"]
    #[inline(always)]
    pub fn spms(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spcr::Spms,
        spcr::Spms,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spcr::Spms,
            spcr::Spms,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Communications Operating Mode Select"]
    #[inline(always)]
    pub fn txmd(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spcr::Txmd,
        spcr::Txmd,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spcr::Txmd,
            spcr::Txmd,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mode Fault Error Detection Enable"]
    #[inline(always)]
    pub fn modfen(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        spcr::Modfen,
        spcr::Modfen,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            spcr::Modfen,
            spcr::Modfen,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI Master/Slave Mode Select"]
    #[inline(always)]
    pub fn mstr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        spcr::Mstr,
        spcr::Mstr,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            spcr::Mstr,
            spcr::Mstr,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI Error Interrupt Enable"]
    #[inline(always)]
    pub fn speie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        spcr::Speie,
        spcr::Speie,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            spcr::Speie,
            spcr::Speie,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn sptie(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        spcr::Sptie,
        spcr::Sptie,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            spcr::Sptie,
            spcr::Sptie,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI Function Enable"]
    #[inline(always)]
    pub fn spe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        spcr::Spe,
        spcr::Spe,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            spcr::Spe,
            spcr::Spe,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn sprie(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spcr::Sprie,
        spcr::Sprie,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spcr::Sprie,
            spcr::Sprie,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Spcr {
    #[inline(always)]
    fn default() -> Spcr {
        <crate::RegValueT<Spcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spms_SPEC;
    pub type Spms = crate::EnumBitfieldStruct<u8, Spms_SPEC>;
    impl Spms {
        #[doc = "Select SPI operation (4-wire method)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select clock synchronous operation (3-wire method)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txmd_SPEC;
    pub type Txmd = crate::EnumBitfieldStruct<u8, Txmd_SPEC>;
    impl Txmd {
        #[doc = "Select full-duplex synchronous serial communications"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select serial communications with transmit-only"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Modfen_SPEC;
    pub type Modfen = crate::EnumBitfieldStruct<u8, Modfen_SPEC>;
    impl Modfen {
        #[doc = "Disable detection of mode fault errors"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable detection of mode fault errors"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstr_SPEC;
    pub type Mstr = crate::EnumBitfieldStruct<u8, Mstr_SPEC>;
    impl Mstr {
        #[doc = "Select slave mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select master mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Speie_SPEC;
    pub type Speie = crate::EnumBitfieldStruct<u8, Speie_SPEC>;
    impl Speie {
        #[doc = "Disable SPI error interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable SPI error interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sptie_SPEC;
    pub type Sptie = crate::EnumBitfieldStruct<u8, Sptie_SPEC>;
    impl Sptie {
        #[doc = "Disable transmit buffer empty interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable transmit buffer empty interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spe_SPEC;
    pub type Spe = crate::EnumBitfieldStruct<u8, Spe_SPEC>;
    impl Spe {
        #[doc = "Disable SPI function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable SPI function"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sprie_SPEC;
    pub type Sprie = crate::EnumBitfieldStruct<u8, Sprie_SPEC>;
    impl Sprie {
        #[doc = "Disable SPI receive buffer full interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable SPI receive buffer full interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sslp_SPEC;
impl crate::sealed::RegSpec for Sslp_SPEC {
    type DataType = u8;
}

#[doc = "SPI Slave Select Polarity Register"]
pub type Sslp = crate::RegValueT<Sslp_SPEC>;

impl Sslp {
    #[doc = "SSLn0 Signal Polarity Setting"]
    #[inline(always)]
    pub fn ssl0p(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sslp::Ssl0P,
        sslp::Ssl0P,
        Sslp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sslp::Ssl0P,
            sslp::Ssl0P,
            Sslp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SSLn1 Signal Polarity Setting"]
    #[inline(always)]
    pub fn ssl1p(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sslp::Ssl1P,
        sslp::Ssl1P,
        Sslp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sslp::Ssl1P,
            sslp::Ssl1P,
            Sslp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SSLn2 Signal Polarity Setting"]
    #[inline(always)]
    pub fn ssl2p(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sslp::Ssl2P,
        sslp::Ssl2P,
        Sslp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sslp::Ssl2P,
            sslp::Ssl2P,
            Sslp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SSLn3 Signal Polarity Setting"]
    #[inline(always)]
    pub fn ssl3p(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sslp::Ssl3P,
        sslp::Ssl3P,
        Sslp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sslp::Ssl3P,
            sslp::Ssl3P,
            Sslp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sslp {
    #[inline(always)]
    fn default() -> Sslp {
        <crate::RegValueT<Sslp_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sslp {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssl0P_SPEC;
    pub type Ssl0P = crate::EnumBitfieldStruct<u8, Ssl0P_SPEC>;
    impl Ssl0P {
        #[doc = "Set SSLn0 signal to active-low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set SSLn0 signal to active-high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssl1P_SPEC;
    pub type Ssl1P = crate::EnumBitfieldStruct<u8, Ssl1P_SPEC>;
    impl Ssl1P {
        #[doc = "Set SSLn1 signal to active-low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set SSLn1 signal to active-high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssl2P_SPEC;
    pub type Ssl2P = crate::EnumBitfieldStruct<u8, Ssl2P_SPEC>;
    impl Ssl2P {
        #[doc = "Set SSLn2 signal to active-low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set SSLn2 signal to active-high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssl3P_SPEC;
    pub type Ssl3P = crate::EnumBitfieldStruct<u8, Ssl3P_SPEC>;
    impl Ssl3P {
        #[doc = "Set SSLn3 signal to active-low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set SSLn3 signal to active-high"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sppcr_SPEC;
impl crate::sealed::RegSpec for Sppcr_SPEC {
    type DataType = u8;
}

#[doc = "SPI Pin Control Register"]
pub type Sppcr = crate::RegValueT<Sppcr_SPEC>;

impl Sppcr {
    #[doc = "SPI Loopback"]
    #[inline(always)]
    pub fn splp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sppcr::Splp,
        sppcr::Splp,
        Sppcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sppcr::Splp,
            sppcr::Splp,
            Sppcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI Loopback 2"]
    #[inline(always)]
    pub fn splp2(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sppcr::Splp2,
        sppcr::Splp2,
        Sppcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sppcr::Splp2,
            sppcr::Splp2,
            Sppcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MOSI Idle Fixed Value"]
    #[inline(always)]
    pub fn moifv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sppcr::Moifv,
        sppcr::Moifv,
        Sppcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sppcr::Moifv,
            sppcr::Moifv,
            Sppcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MOSI Idle Value Fixing Enable"]
    #[inline(always)]
    pub fn moife(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sppcr::Moife,
        sppcr::Moife,
        Sppcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sppcr::Moife,
            sppcr::Moife,
            Sppcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sppcr {
    #[inline(always)]
    fn default() -> Sppcr {
        <crate::RegValueT<Sppcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sppcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Splp_SPEC;
    pub type Splp = crate::EnumBitfieldStruct<u8, Splp_SPEC>;
    impl Splp {
        #[doc = "Normal mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Loopback mode (receive data = inverted transmit data)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Splp2_SPEC;
    pub type Splp2 = crate::EnumBitfieldStruct<u8, Splp2_SPEC>;
    impl Splp2 {
        #[doc = "Normal mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Loopback mode (receive data = transmit data)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moifv_SPEC;
    pub type Moifv = crate::EnumBitfieldStruct<u8, Moifv_SPEC>;
    impl Moifv {
        #[doc = "Set level output on MOSIn pin during MOSI idling to low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set level output on MOSIn pin during MOSI idling to high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moife_SPEC;
    pub type Moife = crate::EnumBitfieldStruct<u8, Moife_SPEC>;
    impl Moife {
        #[doc = "Set MOSI output value to equal final data from previous transfer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set MOSI output value to equal value set in the MOIFV bit"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spsr_SPEC;
impl crate::sealed::RegSpec for Spsr_SPEC {
    type DataType = u8;
}

#[doc = "SPI Status Register"]
pub type Spsr = crate::RegValueT<Spsr_SPEC>;

impl Spsr {
    #[doc = "Overrun Error Flag"]
    #[inline(always)]
    pub fn ovrf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spsr::Ovrf,
        spsr::Ovrf,
        Spsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spsr::Ovrf,
            spsr::Ovrf,
            Spsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI Idle Flag"]
    #[inline(always)]
    pub fn idlnf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spsr::Idlnf,
        spsr::Idlnf,
        Spsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spsr::Idlnf,
            spsr::Idlnf,
            Spsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Mode Fault Error Flag"]
    #[inline(always)]
    pub fn modf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        spsr::Modf,
        spsr::Modf,
        Spsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            spsr::Modf,
            spsr::Modf,
            Spsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub fn perf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        spsr::Perf,
        spsr::Perf,
        Spsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            spsr::Perf,
            spsr::Perf,
            Spsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Underrun Error Flag"]
    #[inline(always)]
    pub fn udrf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        spsr::Udrf,
        spsr::Udrf,
        Spsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            spsr::Udrf,
            spsr::Udrf,
            Spsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI Transmit Buffer Empty Flag"]
    #[inline(always)]
    pub fn sptef(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        spsr::Sptef,
        spsr::Sptef,
        Spsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            spsr::Sptef,
            spsr::Sptef,
            Spsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI Receive Buffer Full Flag"]
    #[inline(always)]
    pub fn sprf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spsr::Sprf,
        spsr::Sprf,
        Spsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spsr::Sprf,
            spsr::Sprf,
            Spsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Spsr {
    #[inline(always)]
    fn default() -> Spsr {
        <crate::RegValueT<Spsr_SPEC> as RegisterValue<_>>::new(32)
    }
}
pub mod spsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovrf_SPEC;
    pub type Ovrf = crate::EnumBitfieldStruct<u8, Ovrf_SPEC>;
    impl Ovrf {
        #[doc = "No overrun error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Overrun error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idlnf_SPEC;
    pub type Idlnf = crate::EnumBitfieldStruct<u8, Idlnf_SPEC>;
    impl Idlnf {
        #[doc = "SPI is in the idle state"]
        pub const _0: Self = Self::new(0);

        #[doc = "SPI is in the transfer state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Modf_SPEC;
    pub type Modf = crate::EnumBitfieldStruct<u8, Modf_SPEC>;
    impl Modf {
        #[doc = "No mode fault or underrun error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mode fault error or underrun error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Perf_SPEC;
    pub type Perf = crate::EnumBitfieldStruct<u8, Perf_SPEC>;
    impl Perf {
        #[doc = "No parity error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Parity error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Udrf_SPEC;
    pub type Udrf = crate::EnumBitfieldStruct<u8, Udrf_SPEC>;
    impl Udrf {
        #[doc = "Mode fault error occurred (MODF = 1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Underrun error occurred (MODF = 1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sptef_SPEC;
    pub type Sptef = crate::EnumBitfieldStruct<u8, Sptef_SPEC>;
    impl Sptef {
        #[doc = "Data is in the transmit buffer"]
        pub const _0: Self = Self::new(0);

        #[doc = "No data is in the transmit buffer"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sprf_SPEC;
    pub type Sprf = crate::EnumBitfieldStruct<u8, Sprf_SPEC>;
    impl Sprf {
        #[doc = "No valid data is in SPDR/SPDR_HA"]
        pub const _0: Self = Self::new(0);

        #[doc = "Valid data is in SPDR/SPDR_HA"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spdr_SPEC;
impl crate::sealed::RegSpec for Spdr_SPEC {
    type DataType = u32;
}

#[doc = "SPI Data Register"]
pub type Spdr = crate::RegValueT<Spdr_SPEC>;

impl NoBitfieldReg<Spdr_SPEC> for Spdr {}
impl ::core::default::Default for Spdr {
    #[inline(always)]
    fn default() -> Spdr {
        <crate::RegValueT<Spdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpdrHa_SPEC;
impl crate::sealed::RegSpec for SpdrHa_SPEC {
    type DataType = u16;
}

#[doc = "SPI Data Register"]
pub type SpdrHa = crate::RegValueT<SpdrHa_SPEC>;

impl NoBitfieldReg<SpdrHa_SPEC> for SpdrHa {}
impl ::core::default::Default for SpdrHa {
    #[inline(always)]
    fn default() -> SpdrHa {
        <crate::RegValueT<SpdrHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpdrBy_SPEC;
impl crate::sealed::RegSpec for SpdrBy_SPEC {
    type DataType = u8;
}

#[doc = "SPI Data Register"]
pub type SpdrBy = crate::RegValueT<SpdrBy_SPEC>;

impl NoBitfieldReg<SpdrBy_SPEC> for SpdrBy {}
impl ::core::default::Default for SpdrBy {
    #[inline(always)]
    fn default() -> SpdrBy {
        <crate::RegValueT<SpdrBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spbr_SPEC;
impl crate::sealed::RegSpec for Spbr_SPEC {
    type DataType = u8;
}

#[doc = "SPI Bit Rate Register"]
pub type Spbr = crate::RegValueT<Spbr_SPEC>;

impl NoBitfieldReg<Spbr_SPEC> for Spbr {}
impl ::core::default::Default for Spbr {
    #[inline(always)]
    fn default() -> Spbr {
        <crate::RegValueT<Spbr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spdcr_SPEC;
impl crate::sealed::RegSpec for Spdcr_SPEC {
    type DataType = u8;
}

#[doc = "SPI Data Control Register"]
pub type Spdcr = crate::RegValueT<Spdcr_SPEC>;

impl Spdcr {
    #[doc = "SPI Receive/Transmit Data Select"]
    #[inline(always)]
    pub fn sprdtd(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        spdcr::Sprdtd,
        spdcr::Sprdtd,
        Spdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            spdcr::Sprdtd,
            spdcr::Sprdtd,
            Spdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI Word Access/Halfword Access Specification"]
    #[inline(always)]
    pub fn splw(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        spdcr::Splw,
        spdcr::Splw,
        Spdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            spdcr::Splw,
            spdcr::Splw,
            Spdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI Byte Access Specification"]
    #[inline(always)]
    pub fn spbyt(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        spdcr::Spbyt,
        spdcr::Spbyt,
        Spdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            spdcr::Spbyt,
            spdcr::Spbyt,
            Spdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Spdcr {
    #[inline(always)]
    fn default() -> Spdcr {
        <crate::RegValueT<Spdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sprdtd_SPEC;
    pub type Sprdtd = crate::EnumBitfieldStruct<u8, Sprdtd_SPEC>;
    impl Sprdtd {
        #[doc = "Read SPDR/SPDR_HA values from receive buffer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read SPDR/SPDR_HA values from transmit buffer, but only if the transmit buffer is empty"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Splw_SPEC;
    pub type Splw = crate::EnumBitfieldStruct<u8, Splw_SPEC>;
    impl Splw {
        #[doc = "Set SPDR_HA to valid for halfword access"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set SPDR to valid for word access"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spbyt_SPEC;
    pub type Spbyt = crate::EnumBitfieldStruct<u8, Spbyt_SPEC>;
    impl Spbyt {
        #[doc = "SPDR/SPDR_HA is accessed in halfword or word (SPLW is valid)"]
        pub const _0: Self = Self::new(0);

        #[doc = "SPDR_BY is accessed in byte (SPLW is invalid)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spckd_SPEC;
impl crate::sealed::RegSpec for Spckd_SPEC {
    type DataType = u8;
}

#[doc = "SPI Clock Delay Register"]
pub type Spckd = crate::RegValueT<Spckd_SPEC>;

impl Spckd {
    #[doc = "RSPCK Delay Setting"]
    #[inline(always)]
    pub fn sckdl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        spckd::Sckdl,
        spckd::Sckdl,
        Spckd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            spckd::Sckdl,
            spckd::Sckdl,
            Spckd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Spckd {
    #[inline(always)]
    fn default() -> Spckd {
        <crate::RegValueT<Spckd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spckd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckdl_SPEC;
    pub type Sckdl = crate::EnumBitfieldStruct<u8, Sckdl_SPEC>;
    impl Sckdl {
        #[doc = "1 RSPCK"]
        pub const _000: Self = Self::new(0);

        #[doc = "2 RSPCK"]
        pub const _001: Self = Self::new(1);

        #[doc = "3 RSPCK"]
        pub const _010: Self = Self::new(2);

        #[doc = "4 RSPCK"]
        pub const _011: Self = Self::new(3);

        #[doc = "5 RSPCK"]
        pub const _100: Self = Self::new(4);

        #[doc = "6 RSPCK"]
        pub const _101: Self = Self::new(5);

        #[doc = "7 RSPCK"]
        pub const _110: Self = Self::new(6);

        #[doc = "8 RSPCK"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sslnd_SPEC;
impl crate::sealed::RegSpec for Sslnd_SPEC {
    type DataType = u8;
}

#[doc = "SPI Slave Select Negation Delay Register"]
pub type Sslnd = crate::RegValueT<Sslnd_SPEC>;

impl Sslnd {
    #[doc = "SSL Negation Delay Setting"]
    #[inline(always)]
    pub fn slndl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        sslnd::Slndl,
        sslnd::Slndl,
        Sslnd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            sslnd::Slndl,
            sslnd::Slndl,
            Sslnd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sslnd {
    #[inline(always)]
    fn default() -> Sslnd {
        <crate::RegValueT<Sslnd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sslnd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slndl_SPEC;
    pub type Slndl = crate::EnumBitfieldStruct<u8, Slndl_SPEC>;
    impl Slndl {
        #[doc = "1 RSPCK"]
        pub const _000: Self = Self::new(0);

        #[doc = "2 RSPCK"]
        pub const _001: Self = Self::new(1);

        #[doc = "3 RSPCK"]
        pub const _010: Self = Self::new(2);

        #[doc = "4 RSPCK"]
        pub const _011: Self = Self::new(3);

        #[doc = "5 RSPCK"]
        pub const _100: Self = Self::new(4);

        #[doc = "6 RSPCK"]
        pub const _101: Self = Self::new(5);

        #[doc = "7 RSPCK"]
        pub const _110: Self = Self::new(6);

        #[doc = "8 RSPCK"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spnd_SPEC;
impl crate::sealed::RegSpec for Spnd_SPEC {
    type DataType = u8;
}

#[doc = "SPI Next-Access Delay Register"]
pub type Spnd = crate::RegValueT<Spnd_SPEC>;

impl Spnd {
    #[doc = "SPI Next-Access Delay Setting"]
    #[inline(always)]
    pub fn spndl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        spnd::Spndl,
        spnd::Spndl,
        Spnd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            spnd::Spndl,
            spnd::Spndl,
            Spnd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Spnd {
    #[inline(always)]
    fn default() -> Spnd {
        <crate::RegValueT<Spnd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spnd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spndl_SPEC;
    pub type Spndl = crate::EnumBitfieldStruct<u8, Spndl_SPEC>;
    impl Spndl {
        #[doc = "1 RSPCK + 2 PCLKB"]
        pub const _000: Self = Self::new(0);

        #[doc = "2 RSPCK + 2 PCLKB"]
        pub const _001: Self = Self::new(1);

        #[doc = "3 RSPCK + 2 PCLKB"]
        pub const _010: Self = Self::new(2);

        #[doc = "4 RSPCK + 2 PCLKB"]
        pub const _011: Self = Self::new(3);

        #[doc = "5 RSPCK + 2 PCLKB"]
        pub const _100: Self = Self::new(4);

        #[doc = "6 RSPCK + 2 PCLKB"]
        pub const _101: Self = Self::new(5);

        #[doc = "7 RSPCK + 2 PCLKB"]
        pub const _110: Self = Self::new(6);

        #[doc = "8 RSPCK + 2 PCLKB"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcr2_SPEC;
impl crate::sealed::RegSpec for Spcr2_SPEC {
    type DataType = u8;
}

#[doc = "SPI Control Register 2"]
pub type Spcr2 = crate::RegValueT<Spcr2_SPEC>;

impl Spcr2 {
    #[doc = "Parity Enable"]
    #[inline(always)]
    pub fn sppe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spcr2::Sppe,
        spcr2::Sppe,
        Spcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spcr2::Sppe,
            spcr2::Sppe,
            Spcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parity Mode"]
    #[inline(always)]
    pub fn spoe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spcr2::Spoe,
        spcr2::Spoe,
        Spcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spcr2::Spoe,
            spcr2::Spoe,
            Spcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI Idle Interrupt Enable"]
    #[inline(always)]
    pub fn spiie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        spcr2::Spiie,
        spcr2::Spiie,
        Spcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            spcr2::Spiie,
            spcr2::Spiie,
            Spcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parity Self-Testing"]
    #[inline(always)]
    pub fn pte(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        spcr2::Pte,
        spcr2::Pte,
        Spcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            spcr2::Pte,
            spcr2::Pte,
            Spcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RSPCK Auto-Stop Function Enable"]
    #[inline(always)]
    pub fn sckase(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        spcr2::Sckase,
        spcr2::Sckase,
        Spcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            spcr2::Sckase,
            spcr2::Sckase,
            Spcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Spcr2 {
    #[inline(always)]
    fn default() -> Spcr2 {
        <crate::RegValueT<Spcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sppe_SPEC;
    pub type Sppe = crate::EnumBitfieldStruct<u8, Sppe_SPEC>;
    impl Sppe {
        #[doc = "Do not add parity bit to transmit data and do not check parity bit of receive data"]
        pub const _0: Self = Self::new(0);

        #[doc = "When SPCR.TXMD = 0: Add parity bit to transmit data and check parity bit of receive data When SPCR.TXMD = 1: Add parity bit to transmit data but do not check parity bit of receive data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spoe_SPEC;
    pub type Spoe = crate::EnumBitfieldStruct<u8, Spoe_SPEC>;
    impl Spoe {
        #[doc = "Select even parity for transmission and reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select odd parity for transmission and reception"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spiie_SPEC;
    pub type Spiie = crate::EnumBitfieldStruct<u8, Spiie_SPEC>;
    impl Spiie {
        #[doc = "Disable idle interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable idle interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pte_SPEC;
    pub type Pte = crate::EnumBitfieldStruct<u8, Pte_SPEC>;
    impl Pte {
        #[doc = "Disable self-diagnosis function of the parity circuit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable self-diagnosis function of the parity circuit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckase_SPEC;
    pub type Sckase = crate::EnumBitfieldStruct<u8, Sckase_SPEC>;
    impl Sckase {
        #[doc = "Disable RSPCK auto-stop function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable RSPCK auto-stop function"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcmd0_SPEC;
impl crate::sealed::RegSpec for Spcmd0_SPEC {
    type DataType = u16;
}

#[doc = "SPI Command Register 0"]
pub type Spcmd0 = crate::RegValueT<Spcmd0_SPEC>;

impl Spcmd0 {
    #[doc = "RSPCK Phase Setting"]
    #[inline(always)]
    pub fn cpha(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spcmd0::Cpha,
        spcmd0::Cpha,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spcmd0::Cpha,
            spcmd0::Cpha,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RSPCK Polarity Setting"]
    #[inline(always)]
    pub fn cpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spcmd0::Cpol,
        spcmd0::Cpol,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spcmd0::Cpol,
            spcmd0::Cpol,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bit Rate Division Setting"]
    #[inline(always)]
    pub fn brdv(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        spcmd0::Brdv,
        spcmd0::Brdv,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            spcmd0::Brdv,
            spcmd0::Brdv,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SSL Signal Assertion Setting"]
    #[inline(always)]
    pub fn ssla(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        spcmd0::Ssla,
        spcmd0::Ssla,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            spcmd0::Ssla,
            spcmd0::Ssla,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI Data Length Setting"]
    #[inline(always)]
    pub fn spb(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        spcmd0::Spb,
        spcmd0::Spb,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            spcmd0::Spb,
            spcmd0::Spb,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI LSB First"]
    #[inline(always)]
    pub fn lsbf(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        spcmd0::Lsbf,
        spcmd0::Lsbf,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            spcmd0::Lsbf,
            spcmd0::Lsbf,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SPI Next-Access Delay Enable"]
    #[inline(always)]
    pub fn spnden(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        spcmd0::Spnden,
        spcmd0::Spnden,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            spcmd0::Spnden,
            spcmd0::Spnden,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SSL Negation Delay Setting Enable"]
    #[inline(always)]
    pub fn slnden(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        spcmd0::Slnden,
        spcmd0::Slnden,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            spcmd0::Slnden,
            spcmd0::Slnden,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RSPCK Delay Setting Enable"]
    #[inline(always)]
    pub fn sckden(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        spcmd0::Sckden,
        spcmd0::Sckden,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            spcmd0::Sckden,
            spcmd0::Sckden,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Spcmd0 {
    #[inline(always)]
    fn default() -> Spcmd0 {
        <crate::RegValueT<Spcmd0_SPEC> as RegisterValue<_>>::new(1805)
    }
}
pub mod spcmd0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha_SPEC;
    pub type Cpha = crate::EnumBitfieldStruct<u8, Cpha_SPEC>;
    impl Cpha {
        #[doc = "Select data sampling on leading edge, data change on trailing edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select data change on leading edge, data sampling on trailing edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol_SPEC;
    pub type Cpol = crate::EnumBitfieldStruct<u8, Cpol_SPEC>;
    impl Cpol {
        #[doc = "Set RSPCK low during idle"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set RSPCK high during idle"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdv_SPEC;
    pub type Brdv = crate::EnumBitfieldStruct<u8, Brdv_SPEC>;
    impl Brdv {
        #[doc = "Base bit rate"]
        pub const _00: Self = Self::new(0);

        #[doc = "Base bit rate divided by 2"]
        pub const _01: Self = Self::new(1);

        #[doc = "Base bit rate divided by 4"]
        pub const _10: Self = Self::new(2);

        #[doc = "Base bit rate divided by 8"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssla_SPEC;
    pub type Ssla = crate::EnumBitfieldStruct<u8, Ssla_SPEC>;
    impl Ssla {
        #[doc = "SSL0"]
        pub const _000: Self = Self::new(0);

        #[doc = "SSL1"]
        pub const _001: Self = Self::new(1);

        #[doc = "SSL2"]
        pub const _010: Self = Self::new(2);

        #[doc = "SSL3"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb_SPEC;
    pub type Spb = crate::EnumBitfieldStruct<u8, Spb_SPEC>;
    impl Spb {
        #[doc = "20 bits"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "24 bits"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "32 bits"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "32 bits"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "9 bits"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "10 bits"]
        pub const _0_X_9: Self = Self::new(9);

        #[doc = "11 bits"]
        pub const _0_X_A: Self = Self::new(10);

        #[doc = "12 bits"]
        pub const _0_X_B: Self = Self::new(11);

        #[doc = "13 bits"]
        pub const _0_X_C: Self = Self::new(12);

        #[doc = "14 bits"]
        pub const _0_X_D: Self = Self::new(13);

        #[doc = "15 bits"]
        pub const _0_X_E: Self = Self::new(14);

        #[doc = "16 bits"]
        pub const _0_X_F: Self = Self::new(15);

        #[doc = "8 bits"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsbf_SPEC;
    pub type Lsbf = crate::EnumBitfieldStruct<u8, Lsbf_SPEC>;
    impl Lsbf {
        #[doc = "MSB-first"]
        pub const _0: Self = Self::new(0);

        #[doc = "LSB-first"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spnden_SPEC;
    pub type Spnden = crate::EnumBitfieldStruct<u8, Spnden_SPEC>;
    impl Spnden {
        #[doc = "Select next-access delay of 1 RSPCK + 2 PCLKB"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select next-access delay equal to the setting in the SPI Next-Access Delay Register (SPND)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slnden_SPEC;
    pub type Slnden = crate::EnumBitfieldStruct<u8, Slnden_SPEC>;
    impl Slnden {
        #[doc = "Select SSL negation delay of 1 RSPCK"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select SSL negation delay equal to the setting in the SPI Slave Select Negation Delay Register (SSLND)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckden_SPEC;
    pub type Sckden = crate::EnumBitfieldStruct<u8, Sckden_SPEC>;
    impl Sckden {
        #[doc = "Select RSPCK delay of 1 RSPCK"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select RSPCK delay equal to the setting in the SPI Clock Delay Register (SPCKD)"]
        pub const _1: Self = Self::new(1);
    }
}
