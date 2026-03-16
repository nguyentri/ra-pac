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
// Generated from SVD 1.10.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:00:13 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Serial Interface UARTA"]
unsafe impl ::core::marker::Send for super::Uarta {}
unsafe impl ::core::marker::Sync for super::Uarta {}
impl super::Uarta {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Transmit Buffer Register 0"]
    #[inline(always)]
    pub const fn txba0(&self) -> &'static crate::common::Reg<self::Txba0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Txba0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Receive Buffer Register 0"]
    #[inline(always)]
    pub const fn rxba0(&self) -> &'static crate::common::Reg<self::Rxba0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rxba0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "Operation Mode Setting Register 00"]
    #[inline(always)]
    pub const fn asima00(
        &self,
    ) -> &'static crate::common::Reg<self::Asima00_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Asima00_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Operation Mode Setting Register 01"]
    #[inline(always)]
    pub const fn asima01(
        &self,
    ) -> &'static crate::common::Reg<self::Asima01_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Asima01_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "Baud Rate Generator Control Register 0"]
    #[inline(always)]
    pub const fn brgca0(
        &self,
    ) -> &'static crate::common::Reg<self::Brgca0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Brgca0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Status Register 0"]
    #[inline(always)]
    pub const fn asisa0(&self) -> &'static crate::common::Reg<self::Asisa0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Asisa0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[doc = "Status Clear Trigger Register 0"]
    #[inline(always)]
    pub const fn ascta0(
        &self,
    ) -> &'static crate::common::Reg<self::Ascta0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ascta0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "UARTA Clock Select Register 0"]
    #[inline(always)]
    pub const fn uta0ck(
        &self,
    ) -> &'static crate::common::Reg<self::Uta0Ck_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uta0Ck_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txba0_SPEC;
impl crate::sealed::RegSpec for Txba0_SPEC {
    type DataType = u8;
}

#[doc = "Transmit Buffer Register 0"]
pub type Txba0 = crate::RegValueT<Txba0_SPEC>;

impl NoBitfieldReg<Txba0_SPEC> for Txba0 {}
impl ::core::default::Default for Txba0 {
    #[inline(always)]
    fn default() -> Txba0 {
        <crate::RegValueT<Txba0_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxba0_SPEC;
impl crate::sealed::RegSpec for Rxba0_SPEC {
    type DataType = u8;
}

#[doc = "Receive Buffer Register 0"]
pub type Rxba0 = crate::RegValueT<Rxba0_SPEC>;

impl NoBitfieldReg<Rxba0_SPEC> for Rxba0 {}
impl ::core::default::Default for Rxba0 {
    #[inline(always)]
    fn default() -> Rxba0 {
        <crate::RegValueT<Rxba0_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asima00_SPEC;
impl crate::sealed::RegSpec for Asima00_SPEC {
    type DataType = u8;
}

#[doc = "Operation Mode Setting Register 00"]
pub type Asima00 = crate::RegValueT<Asima00_SPEC>;

impl Asima00 {
    #[doc = "Receive Interrupt Mode Select"]
    #[inline(always)]
    pub fn isrma(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        asima00::Isrma,
        asima00::Isrma,
        Asima00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            asima00::Isrma,
            asima00::Isrma,
            Asima00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Interrupt Mode Select"]
    #[inline(always)]
    pub fn issma(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        asima00::Issma,
        asima00::Issma,
        Asima00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            asima00::Issma,
            asima00::Issma,
            Asima00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Reception Enable"]
    #[inline(always)]
    pub fn rxea(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        asima00::Rxea,
        asima00::Rxea,
        Asima00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            asima00::Rxea,
            asima00::Rxea,
            Asima00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission Enable"]
    #[inline(always)]
    pub fn txea(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        asima00::Txea,
        asima00::Txea,
        Asima00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            asima00::Txea,
            asima00::Txea,
            Asima00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "UART Operation Enable"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        asima00::En,
        asima00::En,
        Asima00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            asima00::En,
            asima00::En,
            Asima00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Asima00 {
    #[inline(always)]
    fn default() -> Asima00 {
        <crate::RegValueT<Asima00_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod asima00 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isrma_SPEC;
    pub type Isrma = crate::EnumBitfieldStruct<u8, Isrma_SPEC>;
    impl Isrma {
        #[doc = "The UARTA0_ERRI interrupt is generated when a reception error occurs (UARTA0_RXI is not generated)"]
        pub const _0: Self = Self::new(0);

        #[doc = "The UARTA0_RXI interrupt is generated when a reception error occurs (UARTA0_ERRI is not generated)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Issma_SPEC;
    pub type Issma = crate::EnumBitfieldStruct<u8, Issma_SPEC>;
    impl Issma {
        #[doc = "The UARTA0_TXI interrupt is generated on completion of transmission"]
        pub const _0: Self = Self::new(0);

        #[doc = "The UARTA0_TXI interrupt is generated when the transmit buffer becomes empty (for continuous transmission)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxea_SPEC;
    pub type Rxea = crate::EnumBitfieldStruct<u8, Rxea_SPEC>;
    impl Rxea {
        #[doc = "Disables reception (reset the reception circuit)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables reception"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txea_SPEC;
    pub type Txea = crate::EnumBitfieldStruct<u8, Txea_SPEC>;
    impl Txea {
        #[doc = "Disables transmission (resets the transmission circuit)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Disables the UART operation clock (resets the internal circuits)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables the UART operation clock"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asima01_SPEC;
impl crate::sealed::RegSpec for Asima01_SPEC {
    type DataType = u8;
}

#[doc = "Operation Mode Setting Register 01"]
pub type Asima01 = crate::RegValueT<Asima01_SPEC>;

impl Asima01 {
    #[doc = "Transmission and Reception Level Setting"]
    #[inline(always)]
    pub fn alv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        asima01::Alv,
        asima01::Alv,
        Asima01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            asima01::Alv,
            asima01::Alv,
            Asima01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission and Reception Order Setting"]
    #[inline(always)]
    pub fn dir(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        asima01::Dir,
        asima01::Dir,
        Asima01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            asima01::Dir,
            asima01::Dir,
            Asima01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission Stop Bit Length Setting"]
    #[inline(always)]
    pub fn sl(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        asima01::Sl,
        asima01::Sl,
        Asima01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            asima01::Sl,
            asima01::Sl,
            Asima01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission and Reception Character Length Setting"]
    #[inline(always)]
    pub fn cl(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x3,
        1,
        0,
        asima01::Cl,
        asima01::Cl,
        Asima01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x3,
            1,
            0,
            asima01::Cl,
            asima01::Cl,
            Asima01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission and Reception Parity Bit Setting"]
    #[inline(always)]
    pub fn ps(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x3,
        1,
        0,
        asima01::Ps,
        asima01::Ps,
        Asima01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x3,
            1,
            0,
            asima01::Ps,
            asima01::Ps,
            Asima01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Asima01 {
    #[inline(always)]
    fn default() -> Asima01 {
        <crate::RegValueT<Asima01_SPEC> as RegisterValue<_>>::new(26)
    }
}
pub mod asima01 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alv_SPEC;
    pub type Alv = crate::EnumBitfieldStruct<u8, Alv_SPEC>;
    impl Alv {
        #[doc = "Positive logic (wait state = high level, start bit = low level, stop bit = high level)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Negative logic (wait state = low level, start bit = high level, stop bit = low level)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dir_SPEC;
    pub type Dir = crate::EnumBitfieldStruct<u8, Dir_SPEC>;
    impl Dir {
        #[doc = "MSB first"]
        pub const _0: Self = Self::new(0);

        #[doc = "LSB first"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sl_SPEC;
    pub type Sl = crate::EnumBitfieldStruct<u8, Sl_SPEC>;
    impl Sl {
        #[doc = "Stop bit length = 1 bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop bit length = 2 bits"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cl_SPEC;
    pub type Cl = crate::EnumBitfieldStruct<u8, Cl_SPEC>;
    impl Cl {
        #[doc = "Character length of data = 5 bits"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Character length of data = 7 bits"]
        pub const _10: Self = Self::new(2);

        #[doc = "Character length of data = 8 bits"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ps_SPEC;
    pub type Ps = crate::EnumBitfieldStruct<u8, Ps_SPEC>;
    impl Ps {
        #[doc = "Transmission: No parity bit is output. Reception: Data is received without parity."]
        pub const _00: Self = Self::new(0);

        #[doc = "Transmission: 0 parity is output. Reception: Data is received with 0 parity."]
        pub const _01: Self = Self::new(1);

        #[doc = "Transmission: Odd parity is output. Reception: Check is made for odd parity."]
        pub const _10: Self = Self::new(2);

        #[doc = "Transmission: Even parity is output. Reception: Check is made for even parity."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brgca0_SPEC;
impl crate::sealed::RegSpec for Brgca0_SPEC {
    type DataType = u8;
}

#[doc = "Baud Rate Generator Control Register 0"]
pub type Brgca0 = crate::RegValueT<Brgca0_SPEC>;

impl NoBitfieldReg<Brgca0_SPEC> for Brgca0 {}
impl ::core::default::Default for Brgca0 {
    #[inline(always)]
    fn default() -> Brgca0 {
        <crate::RegValueT<Brgca0_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asisa0_SPEC;
impl crate::sealed::RegSpec for Asisa0_SPEC {
    type DataType = u8;
}

#[doc = "Status Register 0"]
pub type Asisa0 = crate::RegValueT<Asisa0_SPEC>;

impl Asisa0 {
    #[doc = "Overrun Error Flag"]
    #[inline(always)]
    pub fn ovea(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        asisa0::Ovea,
        asisa0::Ovea,
        Asisa0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            asisa0::Ovea,
            asisa0::Ovea,
            Asisa0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Framing Error Flag"]
    #[inline(always)]
    pub fn fea(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        asisa0::Fea,
        asisa0::Fea,
        Asisa0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            asisa0::Fea,
            asisa0::Fea,
            Asisa0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub fn pea(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        asisa0::Pea,
        asisa0::Pea,
        Asisa0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            asisa0::Pea,
            asisa0::Pea,
            Asisa0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Shift Register Data Flag"]
    #[inline(always)]
    pub fn txsfa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        asisa0::Txsfa,
        asisa0::Txsfa,
        Asisa0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            asisa0::Txsfa,
            asisa0::Txsfa,
            Asisa0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Buffer Data Flag"]
    #[inline(always)]
    pub fn txbfa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        asisa0::Txbfa,
        asisa0::Txbfa,
        Asisa0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            asisa0::Txbfa,
            asisa0::Txbfa,
            Asisa0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Asisa0 {
    #[inline(always)]
    fn default() -> Asisa0 {
        <crate::RegValueT<Asisa0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod asisa0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovea_SPEC;
    pub type Ovea = crate::EnumBitfieldStruct<u8, Ovea_SPEC>;
    impl Ovea {
        #[doc = "No error has occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fea_SPEC;
    pub type Fea = crate::EnumBitfieldStruct<u8, Fea_SPEC>;
    impl Fea {
        #[doc = "No error has occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pea_SPEC;
    pub type Pea = crate::EnumBitfieldStruct<u8, Pea_SPEC>;
    impl Pea {
        #[doc = "No error has occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txsfa_SPEC;
    pub type Txsfa = crate::EnumBitfieldStruct<u8, Txsfa_SPEC>;
    impl Txsfa {
        #[doc = "Data is not being transmitted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data is being transmitted"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txbfa_SPEC;
    pub type Txbfa = crate::EnumBitfieldStruct<u8, Txbfa_SPEC>;
    impl Txbfa {
        #[doc = "No valid data exists in the TXBA0 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Valid data exists in the TXBA0 register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ascta0_SPEC;
impl crate::sealed::RegSpec for Ascta0_SPEC {
    type DataType = u8;
}

#[doc = "Status Clear Trigger Register 0"]
pub type Ascta0 = crate::RegValueT<Ascta0_SPEC>;

impl Ascta0 {
    #[doc = "Overrun Error Flag Clear Trigger"]
    #[inline(always)]
    pub fn ovecta(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ascta0::Ovecta,
        ascta0::Ovecta,
        Ascta0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ascta0::Ovecta,
            ascta0::Ovecta,
            Ascta0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Framing Error Flag Clear Trigger"]
    #[inline(always)]
    pub fn fecta(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ascta0::Fecta,
        ascta0::Fecta,
        Ascta0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ascta0::Fecta,
            ascta0::Fecta,
            Ascta0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parity Error Flag Clear Trigger"]
    #[inline(always)]
    pub fn pecta(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ascta0::Pecta,
        ascta0::Pecta,
        Ascta0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ascta0::Pecta,
            ascta0::Pecta,
            Ascta0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ascta0 {
    #[inline(always)]
    fn default() -> Ascta0 {
        <crate::RegValueT<Ascta0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ascta0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovecta_SPEC;
    pub type Ovecta = crate::EnumBitfieldStruct<u8, Ovecta_SPEC>;
    impl Ovecta {
        #[doc = "Does not clear the ASISA0.OVEA flag (the flag is retained)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the ASISA0.OVEA flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fecta_SPEC;
    pub type Fecta = crate::EnumBitfieldStruct<u8, Fecta_SPEC>;
    impl Fecta {
        #[doc = "Does not clear the ASISA0.FEA flag (the flag is retained)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the ASISA0.FEA flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pecta_SPEC;
    pub type Pecta = crate::EnumBitfieldStruct<u8, Pecta_SPEC>;
    impl Pecta {
        #[doc = "Does not clear the ASISA0.PEA flag (the flag is retained)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the ASISA0.PEA flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uta0Ck_SPEC;
impl crate::sealed::RegSpec for Uta0Ck_SPEC {
    type DataType = u8;
}

#[doc = "UARTA Clock Select Register 0"]
pub type Uta0Ck = crate::RegValueT<Uta0Ck_SPEC>;

impl Uta0Ck {
    #[doc = "UARTA0 Operation Clock Select (fUTA0)"]
    #[inline(always)]
    pub fn ck(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        uta0ck::Ck,
        uta0ck::Ck,
        Uta0Ck_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            uta0ck::Ck,
            uta0ck::Ck,
            Uta0Ck_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "fSEL Clock Select"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        uta0ck::Sel,
        uta0ck::Sel,
        Uta0Ck_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            uta0ck::Sel,
            uta0ck::Sel,
            Uta0Ck_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uta0Ck {
    #[inline(always)]
    fn default() -> Uta0Ck {
        <crate::RegValueT<Uta0Ck_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod uta0ck {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ck_SPEC;
    pub type Ck = crate::EnumBitfieldStruct<u8, Ck_SPEC>;
    impl Ck {
        #[doc = "fSEL"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "fSEL/2"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "fSEL/4"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "fSEL/8"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "fSEL/16"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "fSEL/32"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "fSEL/64"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "FSXP"]
        pub const _0_X_8: Self = Self::new(8);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "Stop"]
        pub const _00: Self = Self::new(0);

        #[doc = "MOSC"]
        pub const _01: Self = Self::new(1);

        #[doc = "HOCO"]
        pub const _10: Self = Self::new(2);

        #[doc = "MOCO"]
        pub const _11: Self = Self::new(3);
    }
}
