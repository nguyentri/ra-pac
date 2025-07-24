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
#[doc = r"Serial Communication Interface"]
unsafe impl ::core::marker::Send for super::Sci0BNs {}
unsafe impl ::core::marker::Sync for super::Sci0BNs {}
impl super::Sci0BNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Receive Data Register"]
    #[inline(always)]
    pub const fn rdr(&self) -> &'static crate::common::Reg<self::Rdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Transmit Data Register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &'static crate::common::Reg<self::Tdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Common Control Register 0"]
    #[inline(always)]
    pub const fn ccr0(&self) -> &'static crate::common::Reg<self::Ccr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Common Control Register 1"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &'static crate::common::Reg<self::Ccr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Common Control Register 2"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &'static crate::common::Reg<self::Ccr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Common Control Register 3"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &'static crate::common::Reg<self::Ccr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Common Control Register 4"]
    #[inline(always)]
    pub const fn ccr4(&self) -> &'static crate::common::Reg<self::Ccr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Communication Enable Status Register"]
    #[inline(always)]
    pub const fn cesr(&self) -> &'static crate::common::Reg<self::Cesr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cesr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Simple IIC Control Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &'static crate::common::Reg<self::Icr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &'static crate::common::Reg<self::Fcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Manchester Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &'static crate::common::Reg<self::Mcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Driver Control Register"]
    #[inline(always)]
    pub const fn dcr(&self) -> &'static crate::common::Reg<self::Dcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Simple LIN Control Register 0"]
    #[inline(always)]
    pub const fn xcr0(&self) -> &'static crate::common::Reg<self::Xcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Simple LIN Control Register 1"]
    #[inline(always)]
    pub const fn xcr1(&self) -> &'static crate::common::Reg<self::Xcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Simple LIN Control Register 2"]
    #[inline(always)]
    pub const fn xcr2(&self) -> &'static crate::common::Reg<self::Xcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Common Status Register"]
    #[inline(always)]
    pub const fn csr(&self) -> &'static crate::common::Reg<self::Csr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Csr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "Simple IIC Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &'static crate::common::Reg<self::Isr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Isr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "FIFO Receive Status Register"]
    #[inline(always)]
    pub const fn frsr(&self) -> &'static crate::common::Reg<self::Frsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "FIFO Transmit Status Register"]
    #[inline(always)]
    pub const fn ftsr(&self) -> &'static crate::common::Reg<self::Ftsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ftsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Manchester Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &'static crate::common::Reg<self::Msr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Msr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "Simple LIN Status Register 0"]
    #[inline(always)]
    pub const fn xsr0(&self) -> &'static crate::common::Reg<self::Xsr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Xsr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "Simple LIN Status Register 1"]
    #[inline(always)]
    pub const fn xsr1(&self) -> &'static crate::common::Reg<self::Xsr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Xsr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Common Flag Clear Register"]
    #[inline(always)]
    pub const fn cfclr(&self) -> &'static crate::common::Reg<self::Cfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Cfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "Simple IIC Flag Clear Register"]
    #[inline(always)]
    pub const fn icfclr(&self) -> &'static crate::common::Reg<self::Icfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Icfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "FIFO Flag Clear Register"]
    #[inline(always)]
    pub const fn ffclr(&self) -> &'static crate::common::Reg<self::Ffclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ffclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "Manchester Flag Clear Register"]
    #[inline(always)]
    pub const fn mfclr(&self) -> &'static crate::common::Reg<self::Mfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Mfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "Simple LIN Flag Clear Register"]
    #[inline(always)]
    pub const fn xfclr(&self) -> &'static crate::common::Reg<self::Xfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Xfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdr_SPEC;
impl crate::sealed::RegSpec for Rdr_SPEC {
    type DataType = u32;
}

#[doc = "Receive Data Register"]
pub type Rdr = crate::RegValueT<Rdr_SPEC>;

impl Rdr {
    #[doc = "Serial receive data"]
    #[inline(always)]
    pub fn rdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Rdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Multi-processor flag"]
    #[inline(always)]
    pub fn mpb(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, rdr::Mpb, rdr::Mpb, Rdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x1,1,0,rdr::Mpb,rdr::Mpb,Rdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive data ready flag"]
    #[inline(always)]
    pub fn dr(self) -> crate::common::RegisterFieldBool<10, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "FIFO parity error flag"]
    #[inline(always)]
    pub fn fper(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, rdr::Fper, rdr::Fper, Rdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            rdr::Fper,
            rdr::Fper,
            Rdr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "FIFO framing error flag"]
    #[inline(always)]
    pub fn ffer(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, rdr::Ffer, rdr::Ffer, Rdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            rdr::Ffer,
            rdr::Ffer,
            Rdr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Overrun Error flag"]
    #[inline(always)]
    pub fn orer(self) -> crate::common::RegisterFieldBool<24, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Parity error flag"]
    #[inline(always)]
    pub fn per(self) -> crate::common::RegisterFieldBool<27, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Framing error flag"]
    #[inline(always)]
    pub fn fer(self) -> crate::common::RegisterFieldBool<28, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Rdr {
    #[inline(always)]
    fn default() -> Rdr {
        <crate::RegValueT<Rdr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rdr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpb_SPEC;
    pub type Mpb = crate::EnumBitfieldStruct<u8, Mpb_SPEC>;
    impl Mpb {
        #[doc = "Data transmission cycles"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycles"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fper_SPEC;
    pub type Fper = crate::EnumBitfieldStruct<u8, Fper_SPEC>;
    impl Fper {
        #[doc = "There is no parity error in the data read from the receive-FIFO"]
        pub const _0: Self = Self::new(0);

        #[doc = "There is parity error in the data read from the receive-FIFO"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ffer_SPEC;
    pub type Ffer = crate::EnumBitfieldStruct<u8, Ffer_SPEC>;
    impl Ffer {
        #[doc = "There is no framing error in the data read from the receive-FIFO"]
        pub const _0: Self = Self::new(0);

        #[doc = "There is framing error in the data read from the receive-FIFO"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr_SPEC;
impl crate::sealed::RegSpec for Tdr_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Data Register"]
pub type Tdr = crate::RegValueT<Tdr_SPEC>;

impl Tdr {
    #[doc = "Serial transmit data"]
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Tdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Tdr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Multi-processor transfer bit flag"]
    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, tdr::Mpbt, tdr::Mpbt, Tdr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            tdr::Mpbt,
            tdr::Mpbt,
            Tdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit SYNC data"]
    #[inline(always)]
    pub fn tsync(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        tdr::Tsync,
        tdr::Tsync,
        Tdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            tdr::Tsync,
            tdr::Tsync,
            Tdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tdr {
    #[inline(always)]
    fn default() -> Tdr {
        <crate::RegValueT<Tdr_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod tdr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpbt_SPEC;
    pub type Mpbt = crate::EnumBitfieldStruct<u8, Mpbt_SPEC>;
    impl Mpbt {
        #[doc = "Data transmission cycles"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycles"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsync_SPEC;
    pub type Tsync = crate::EnumBitfieldStruct<u8, Tsync_SPEC>;
    impl Tsync {
        #[doc = "The Start Bit is transmitted as DATA SYNC."]
        pub const _0: Self = Self::new(0);

        #[doc = "The Start Bit is transmitted as COMMAND SYNC."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr0_SPEC;
impl crate::sealed::RegSpec for Ccr0_SPEC {
    type DataType = u32;
}

#[doc = "Common Control Register 0"]
pub type Ccr0 = crate::RegValueT<Ccr0_SPEC>;

impl Ccr0 {
    #[doc = "Receive Enable"]
    #[inline(always)]
    pub fn re(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ccr0::Re, ccr0::Re, Ccr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ccr0::Re,ccr0::Re,Ccr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit Enable"]
    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ccr0::Te, ccr0::Te, Ccr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ccr0::Te,ccr0::Te,Ccr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Multi-Processor Interrupt Enable"]
    #[inline(always)]
    pub fn mpie(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ccr0::Mpie,
        ccr0::Mpie,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ccr0::Mpie,
            ccr0::Mpie,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Compare Match Enable"]
    #[inline(always)]
    pub fn dcme(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ccr0::Dcme,
        ccr0::Dcme,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ccr0::Dcme,
            ccr0::Dcme,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ID Frame Select"]
    #[inline(always)]
    pub fn idsel(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ccr0::Idsel,
        ccr0::Idsel,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ccr0::Idsel,
            ccr0::Idsel,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ccr0::Rie,
        ccr0::Rie,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ccr0::Rie,
            ccr0::Rie,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ccr0::Tie,
        ccr0::Tie,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ccr0::Tie,
            ccr0::Tie,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        ccr0::Teie,
        ccr0::Teie,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            ccr0::Teie,
            ccr0::Teie,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SSn Pin Function Enable"]
    #[inline(always)]
    pub fn sse(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        ccr0::Sse,
        ccr0::Sse,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            ccr0::Sse,
            ccr0::Sse,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccr0 {
    #[inline(always)]
    fn default() -> Ccr0 {
        <crate::RegValueT<Ccr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        #[doc = "Serial reception is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Serial reception is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te_SPEC;
    pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
    impl Te {
        #[doc = "Serial transmission is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Serial transmission is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpie_SPEC;
    pub type Mpie = crate::EnumBitfieldStruct<u8, Mpie_SPEC>;
    impl Mpie {
        #[doc = "Non-multi-processor reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multi-processor reception When the data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags to 1 is disabled. When the data with the multi-processor bit set to 1 is received, the MPIE bit is automatically cleared to 0, and non-multi-processor reception is resumed. If you want to continue receiving operation using the multiprocessor function, set this bit to 1 sufficiently earlier than receiving the STOP bit of the next received frame (consider the synchronization delay time)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcme_SPEC;
    pub type Dcme = crate::EnumBitfieldStruct<u8, Dcme_SPEC>;
    impl Dcme {
        #[doc = "Address match function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Address match function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idsel_SPEC;
    pub type Idsel = crate::EnumBitfieldStruct<u8, Idsel_SPEC>;
    impl Idsel {
        #[doc = "Compare data irrespective of the value of the MPB bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare data only when the MPB bit is 1 (ID frame)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "SCIn_RXI and SCIn_ERI interrupt requests are disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "SCIn_RXI and SCIn_ERI interrupt requests are enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        #[doc = "SCIn_TXI interrupt request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "SCIn_TXI interrupt request is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teie_SPEC;
    pub type Teie = crate::EnumBitfieldStruct<u8, Teie_SPEC>;
    impl Teie {
        #[doc = "SCIn_TEI interrupt request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "SCIn_TEI interrupt request is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sse_SPEC;
    pub type Sse = crate::EnumBitfieldStruct<u8, Sse_SPEC>;
    impl Sse {
        #[doc = "SSn pin function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "SSn pin function is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1_SPEC;
impl crate::sealed::RegSpec for Ccr1_SPEC {
    type DataType = u32;
}

#[doc = "Common Control Register 1"]
pub type Ccr1 = crate::RegValueT<Ccr1_SPEC>;

impl Ccr1 {
    #[doc = "CTS Enable"]
    #[inline(always)]
    pub fn ctse(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr1::Ctse,
        ccr1::Ctse,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr1::Ctse,
            ccr1::Ctse,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTS External Pin Enable"]
    #[inline(always)]
    pub fn ctspen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccr1::Ctspen,
        ccr1::Ctspen,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccr1::Ctspen,
            ccr1::Ctspen,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial Port Break Data Select"]
    #[inline(always)]
    pub fn spb2dt(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr1::Spb2Dt,
        ccr1::Spb2Dt,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr1::Spb2Dt,
            ccr1::Spb2Dt,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial Port Break I/O"]
    #[inline(always)]
    pub fn spb2io(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr1::Spb2Io,
        ccr1::Spb2Io,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr1::Spb2Io,
            ccr1::Spb2Io,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parity Enable"]
    #[inline(always)]
    pub fn pe(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ccr1::Pe, ccr1::Pe, Ccr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,ccr1::Pe,ccr1::Pe,Ccr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Parity Mode"]
    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ccr1::Pm, ccr1::Pm, Ccr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,ccr1::Pm,ccr1::Pm,Ccr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "TXD Invert"]
    #[inline(always)]
    pub fn tinv(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ccr1::Tinv,
        ccr1::Tinv,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ccr1::Tinv,
            ccr1::Tinv,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RXD Invert"]
    #[inline(always)]
    pub fn rinv(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ccr1::Rinv,
        ccr1::Rinv,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ccr1::Rinv,
            ccr1::Rinv,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Loopback Control"]
    #[inline(always)]
    pub fn splp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ccr1::Splp,
        ccr1::Splp,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ccr1::Splp,
            ccr1::Splp,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Half-Duplex Communication Select"]
    #[inline(always)]
    pub fn sharps(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ccr1::Sharps,
        ccr1::Sharps,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ccr1::Sharps,
            ccr1::Sharps,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        ccr1::Nfcs,
        ccr1::Nfcs,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            ccr1::Nfcs,
            ccr1::Nfcs,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Digital Noise Filter Function Enable"]
    #[inline(always)]
    pub fn nfen(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ccr1::Nfen,
        ccr1::Nfen,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ccr1::Nfen,
            ccr1::Nfen,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise Filter Mode"]
    #[inline(always)]
    pub fn nfm(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ccr1::Nfm,
        ccr1::Nfm,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ccr1::Nfm,
            ccr1::Nfm,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccr1 {
    #[inline(always)]
    fn default() -> Ccr1 {
        <crate::RegValueT<Ccr1_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod ccr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctse_SPEC;
    pub type Ctse = crate::EnumBitfieldStruct<u8, Ctse_SPEC>;
    impl Ctse {
        #[doc = "CTS function is disabled (RTS output function is enabled)"]
        pub const _0: Self = Self::new(0);

        #[doc = "CTS function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctspen_SPEC;
    pub type Ctspen = crate::EnumBitfieldStruct<u8, Ctspen_SPEC>;
    impl Ctspen {
        #[doc = "Alternate setting to use CTS and RTS functions as either 1 pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Dedicated setting for separately using CTS and RTS functions with 2 pins"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb2Dt_SPEC;
    pub type Spb2Dt = crate::EnumBitfieldStruct<u8, Spb2Dt_SPEC>;
    impl Spb2Dt {
        #[doc = "When TINV is 0, low level is output in TXDn pin. When TINV is 1, high level is output in TXDn pin."]
        pub const _0: Self = Self::new(0);

        #[doc = "When TINV is 0, high level is output in TXDn pin. When TINV is 1, low level is output in TXDn pin."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb2Io_SPEC;
    pub type Spb2Io = crate::EnumBitfieldStruct<u8, Spb2Io_SPEC>;
    impl Spb2Io {
        #[doc = "The value of SPB2DT bit is not output in TXDn pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "The value of SPB2DT bit is output in TXDn pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe_SPEC;
    pub type Pe = crate::EnumBitfieldStruct<u8, Pe_SPEC>;
    impl Pe {
        #[doc = "When transmitting: Do not add parity bit When receiving: Do not check parity bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "When transmitting: Add parity bit When receiving: Check parity bit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        #[doc = "Selects even parity"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selects odd parity"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tinv_SPEC;
    pub type Tinv = crate::EnumBitfieldStruct<u8, Tinv_SPEC>;
    impl Tinv {
        #[doc = "Transmit data is not inverted and output to TXDn"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit data is inverted and output to TXDn"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rinv_SPEC;
    pub type Rinv = crate::EnumBitfieldStruct<u8, Rinv_SPEC>;
    impl Rinv {
        #[doc = "Received data from RXDn is not inverted and input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received data from RXDn is inverted and input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Splp_SPEC;
    pub type Splp = crate::EnumBitfieldStruct<u8, Splp_SPEC>;
    impl Splp {
        #[doc = "Normal mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Loopback mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sharps_SPEC;
    pub type Sharps = crate::EnumBitfieldStruct<u8, Sharps_SPEC>;
    impl Sharps {
        #[doc = "TXDn pin, RXDn pin independent"]
        pub const _0: Self = Self::new(0);

        #[doc = "TXDn/RXDn pin combination use (half-duplex communication using TXDn pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "The base clock signal divided by 1"]
        pub const _000: Self = Self::new(0);

        #[doc = "The on-chip baud rate generator source clock divided by 1"]
        pub const _001: Self = Self::new(1);

        #[doc = "The on-chip baud rate generator source clock divided by 2"]
        pub const _010: Self = Self::new(2);

        #[doc = "The on-chip baud rate generator source clock divided by 4"]
        pub const _011: Self = Self::new(3);

        #[doc = "The on-chip baud rate generator source clock divided by 8"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "In Asynchronous, Manchester, Simple LIN modes: Disable noise cancellation function for RXDn input signal In Simple IIC mode: Disable noise cancellation function for SCLn and SDAn input signals"]
        pub const _0: Self = Self::new(0);

        #[doc = "In Asynchronous, Manchester, Simple LIN modes: Enable noise cancellation function for RXDn input signal In Simple IIC mode: Enable noise cancellation function for SCLn and SDAn input signals"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfm_SPEC;
    pub type Nfm = crate::EnumBitfieldStruct<u8, Nfm_SPEC>;
    impl Nfm {
        #[doc = "3-point matching mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Majority vote mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2_SPEC;
impl crate::sealed::RegSpec for Ccr2_SPEC {
    type DataType = u32;
}

#[doc = "Common Control Register 2"]
pub type Ccr2 = crate::RegValueT<Ccr2_SPEC>;

impl Ccr2 {
    #[doc = "Base Clock Pulse"]
    #[inline(always)]
    pub fn bcp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        ccr2::Bcp,
        ccr2::Bcp,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            ccr2::Bcp,
            ccr2::Bcp,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Baud Rate Generator Double-Speed Mode Select"]
    #[inline(always)]
    pub fn bgdm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr2::Bgdm,
        ccr2::Bgdm,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr2::Bgdm,
            ccr2::Bgdm,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Base Clock Select"]
    #[inline(always)]
    pub fn abcs(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr2::Abcs,
        ccr2::Abcs,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr2::Abcs,
            ccr2::Abcs,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Extended Base Clock Select"]
    #[inline(always)]
    pub fn abcse(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ccr2::Abcse,
        ccr2::Abcse,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ccr2::Abcse,
            ccr2::Abcse,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Extended Base Clock Select 2"]
    #[inline(always)]
    pub fn abcse2(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ccr2::Abcse2,
        ccr2::Abcse2,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ccr2::Abcse2,
            ccr2::Abcse2,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bit Rate Setting"]
    #[inline(always)]
    pub fn brr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ccr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ccr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bit Rate Modulation Enable"]
    #[inline(always)]
    pub fn brme(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ccr2::Brme,
        ccr2::Brme,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ccr2::Brme,
            ccr2::Brme,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Select"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        ccr2::Cks,
        ccr2::Cks,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            ccr2::Cks,
            ccr2::Cks,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Modulation Duty Setting"]
    #[inline(always)]
    pub fn mddr(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ccr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ccr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr2 {
    #[inline(always)]
    fn default() -> Ccr2 {
        <crate::RegValueT<Ccr2_SPEC> as RegisterValue<_>>::new(4278255364)
    }
}
pub mod ccr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcp_SPEC;
    pub type Bcp = crate::EnumBitfieldStruct<u8, Bcp_SPEC>;
    impl Bcp {
        #[doc = "93 clock cycles (S = 93)"]
        pub const _000: Self = Self::new(0);

        #[doc = "128 clock cycles (S = 128)"]
        pub const _001: Self = Self::new(1);

        #[doc = "186 clock cycles (S = 186)"]
        pub const _010: Self = Self::new(2);

        #[doc = "512 clock cycles (S = 512)"]
        pub const _011: Self = Self::new(3);

        #[doc = "32 clock cycles (S = 32) (Initial value)"]
        pub const _100: Self = Self::new(4);

        #[doc = "64 clock cycles (S = 64)"]
        pub const _101: Self = Self::new(5);

        #[doc = "372 clock cycles (S = 372)"]
        pub const _110: Self = Self::new(6);

        #[doc = "256 clock cycles (S = 256)"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bgdm_SPEC;
    pub type Bgdm = crate::EnumBitfieldStruct<u8, Bgdm_SPEC>;
    impl Bgdm {
        #[doc = "Baud rate generator outputs the clock with single frequency"]
        pub const _0: Self = Self::new(0);

        #[doc = "Baud rate generator outputs the clock with doubled frequency"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcs_SPEC;
    pub type Abcs = crate::EnumBitfieldStruct<u8, Abcs_SPEC>;
    impl Abcs {
        #[doc = "Selects 16 base clock cycles for 1-bit period"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selects 8 base clock cycles for 1-bit period"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcse_SPEC;
    pub type Abcse = crate::EnumBitfieldStruct<u8, Abcse_SPEC>;
    impl Abcse {
        #[doc = "Clock cycles for 1-bit period is determined by combination between CCR2.BGDM and CCR2.ABCS."]
        pub const _0: Self = Self::new(0);

        #[doc = "Baud rate is 6 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcse2_SPEC;
    pub type Abcse2 = crate::EnumBitfieldStruct<u8, Abcse2_SPEC>;
    impl Abcse2 {
        #[doc = "Clock cycles for 1-bit period is decided with combination between CCR2.BGDM and CCR2.ABCS."]
        pub const _0: Self = Self::new(0);

        #[doc = "Baud rate is 4 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brme_SPEC;
    pub type Brme = crate::EnumBitfieldStruct<u8, Brme_SPEC>;
    impl Brme {
        #[doc = "Bit rate modulation function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit rate modulation function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "TCLK clock (n = 0)"]
        pub const _00: Self = Self::new(0);

        #[doc = "TCLK/4 clock (n = 1)"]
        pub const _01: Self = Self::new(1);

        #[doc = "TCLK/16 clock (n = 2)"]
        pub const _10: Self = Self::new(2);

        #[doc = "TCLK/64 clock (n = 3)"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr3_SPEC;
impl crate::sealed::RegSpec for Ccr3_SPEC {
    type DataType = u32;
}

#[doc = "Common Control Register 3"]
pub type Ccr3 = crate::RegValueT<Ccr3_SPEC>;

impl Ccr3 {
    #[doc = "Clock Phase Select"]
    #[inline(always)]
    pub fn cpha(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr3::Cpha,
        ccr3::Cpha,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr3::Cpha,
            ccr3::Cpha,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Polarity Select"]
    #[inline(always)]
    pub fn cpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccr3::Cpol,
        ccr3::Cpol,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccr3::Cpol,
            ccr3::Cpol,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Synchronizer Bypass Enable"]
    #[inline(always)]
    pub fn bpen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ccr3::Bpen,
        ccr3::Bpen,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ccr3::Bpen,
            ccr3::Bpen,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Character Length"]
    #[inline(always)]
    pub fn chr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        ccr3::Chr,
        ccr3::Chr,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            ccr3::Chr,
            ccr3::Chr,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LSB First select"]
    #[inline(always)]
    pub fn lsbf(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ccr3::Lsbf,
        ccr3::Lsbf,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ccr3::Lsbf,
            ccr3::Lsbf,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmitted/Received Data Invert"]
    #[inline(always)]
    pub fn sinv(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ccr3::Sinv,
        ccr3::Sinv,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ccr3::Sinv,
            ccr3::Sinv,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Stop Bit Length"]
    #[inline(always)]
    pub fn stp(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ccr3::Stp,
        ccr3::Stp,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ccr3::Stp,
            ccr3::Stp,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Start Bit Edge Detection Select"]
    #[inline(always)]
    pub fn rxdesel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ccr3::Rxdesel,
        ccr3::Rxdesel,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ccr3::Rxdesel,
            ccr3::Rxdesel,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Communication Mode Select"]
    #[inline(always)]
    pub fn r#mod(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        ccr3::Mod,
        ccr3::Mod,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            ccr3::Mod,
            ccr3::Mod,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multi-Processor Mode"]
    #[inline(always)]
    pub fn mp(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, ccr3::Mp, ccr3::Mp, Ccr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            ccr3::Mp,
            ccr3::Mp,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Mode Select"]
    #[inline(always)]
    pub fn fm(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ccr3::Fm, ccr3::Fm, Ccr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ccr3::Fm,
            ccr3::Fm,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Driver Enable"]
    #[inline(always)]
    pub fn den(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        ccr3::Den,
        ccr3::Den,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            ccr3::Den,
            ccr3::Den,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Enable"]
    #[inline(always)]
    pub fn cke(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        ccr3::Cke,
        ccr3::Cke,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            ccr3::Cke,
            ccr3::Cke,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Clock Source Select"]
    #[inline(always)]
    pub fn acs0(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        ccr3::Acs0,
        ccr3::Acs0,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            ccr3::Acs0,
            ccr3::Acs0,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GSM Mode"]
    #[inline(always)]
    pub fn gm(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, ccr3::Gm, ccr3::Gm, Ccr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ccr3::Gm,
            ccr3::Gm,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Block Transfer Mode"]
    #[inline(always)]
    pub fn blk(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ccr3::Blk,
        ccr3::Blk,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ccr3::Blk,
            ccr3::Blk,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccr3 {
    #[inline(always)]
    fn default() -> Ccr3 {
        <crate::RegValueT<Ccr3_SPEC> as RegisterValue<_>>::new(4611)
    }
}
pub mod ccr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha_SPEC;
    pub type Cpha = crate::EnumBitfieldStruct<u8, Cpha_SPEC>;
    impl Cpha {
        #[doc = "Data is sampled at an odd edge and changes at an even edge (clock is delayed)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data changes at an odd edge and is sampled at an even edge (clock is not delayed)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol_SPEC;
    pub type Cpol = crate::EnumBitfieldStruct<u8, Cpol_SPEC>;
    impl Cpol {
        #[doc = "SCKn in idle state is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "SCKn in idle state is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpen_SPEC;
    pub type Bpen = crate::EnumBitfieldStruct<u8, Bpen_SPEC>;
    impl Bpen {
        #[doc = "Synchronizer circuit is not bypassed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronizer circuit is bypassed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chr_SPEC;
    pub type Chr = crate::EnumBitfieldStruct<u8, Chr_SPEC>;
    impl Chr {
        #[doc = "Transmit/receive in 9-bit data length"]
        pub const _00: Self = Self::new(0);

        #[doc = "Transmit/receive in 9-bit data length"]
        pub const _01: Self = Self::new(1);

        #[doc = "Transmit/receive in 8-bit data length (initial value)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Transmit/receive in 7-bit data length"]
        pub const _11: Self = Self::new(3);
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
    pub struct Sinv_SPEC;
    pub type Sinv = crate::EnumBitfieldStruct<u8, Sinv_SPEC>;
    impl Sinv {
        #[doc = "TDR contents are transmitted to TSR as is. RSR contents are stored to RDR as is."]
        pub const _0: Self = Self::new(0);

        #[doc = "TDR contents are inverted before being transmitted to TSR. RSR contents are inverted and stored to RDR."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stp_SPEC;
    pub type Stp = crate::EnumBitfieldStruct<u8, Stp_SPEC>;
    impl Stp {
        #[doc = "1 stop bit/break delimiter length is 1-bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "2 stop bits/break delimiter length is 2-bit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdesel_SPEC;
    pub type Rxdesel = crate::EnumBitfieldStruct<u8, Rxdesel_SPEC>;
    impl Rxdesel {
        #[doc = "The low level on the RXDn pin is detected as the start bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "A falling edge on the RXDn pin is detected as the start bit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mod_SPEC;
    pub type Mod = crate::EnumBitfieldStruct<u8, Mod_SPEC>;
    impl Mod {
        #[doc = "Asynchronous mode (multi-processor mode)"]
        pub const _000: Self = Self::new(0);

        #[doc = "Smart card interface mode"]
        pub const _001: Self = Self::new(1);

        #[doc = "Clock synchronous mode"]
        pub const _010: Self = Self::new(2);

        #[doc = "Simple SPI mode"]
        pub const _011: Self = Self::new(3);

        #[doc = "Simple IIC mode"]
        pub const _100: Self = Self::new(4);

        #[doc = "Manchester mode"]
        pub const _101: Self = Self::new(5);

        #[doc = "Simple LIN mode"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mp_SPEC;
    pub type Mp = crate::EnumBitfieldStruct<u8, Mp_SPEC>;
    impl Mp {
        #[doc = "Multi-processor communications function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multi-processor communications function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fm_SPEC;
    pub type Fm = crate::EnumBitfieldStruct<u8, Fm_SPEC>;
    impl Fm {
        #[doc = "TDR register and RDR register are non-FIFO buffer configuration"]
        pub const _0: Self = Self::new(0);

        #[doc = "TDR register and RDR register are FIFO buffer configuration"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Den_SPEC;
    pub type Den = crate::EnumBitfieldStruct<u8, Den_SPEC>;
    impl Den {
        #[doc = "RS-485 driver control function disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "RS-485 driver control function enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cke_SPEC;
    pub type Cke = crate::EnumBitfieldStruct<u8, Cke_SPEC>;
    impl Cke {
        #[doc = "In Asynchronous mode:On-chip baud rate generatorThe SCKn pin is available for use as an I/O port in accord with the I/O port settings. In Manchester mode and Simple LIN mode:On-chip baud rate generatorThe SCKn pin functions as I/O port. In Clock synchronous mode and Simple SPI mode:Internal clock (master operation)The SCKn pin functions as the clock output pin. In Smart card interface mode and when CCR3.GM = 0:Output disabled (the SCKn pin is available for use as an I/O port in accord with the I/O port settings). In Smart card interface mode and when CCR3.GM = 1:Output fixed low"]
        pub const _00: Self = Self::new(0);

        #[doc = "In Asynchronous mode:On-chip baud rate generatorThe clock with the same frequency as the bit rate is output from the SCKn pin. In Manchester mode and Simple LIN mode:Prohibited In Clock synchronous mode and Simple SPI mode:Internal clock (master operation)The SCKn pin functions as the clock output pin. In Smart card interface mode and when CCR3.GM = 0:Clock output In Smart card interface mode and when CCR3.GM = 1:Clock output"]
        pub const _01: Self = Self::new(1);

        #[doc = "In Asynchronous mode:External clock or GPT clock When using the external clock16 times the bit rate should be input from the SCKn pin when CCR2.ABCS bit is 0. Input a clock signal with a frequency 8 times the bit rate when the CCR2.ABCS bit is 1.When using the GPT clockThe SCKn pin is available for use as an I/O port in accord with the I/O port settings when the GPT clock is used. In Manchester mode and Simple LIN mode:Prohibited In Clock synchronous mode and Simple SPI mode:External clock (slave operation)The SCKn pin functions as the clock input pin. In Smart card interface mode and when CCR3.GM = 0:Prohibited In Smart card interface mode and when CCR3.GM = 1:Output fixed high"]
        pub const _10: Self = Self::new(2);

        #[doc = "In Asynchronous mode:External clock or GPT clock When using the external clock16 times the bit rate should be input from the SCKn pin when CCR2.ABCS bit is 0. Input a clock signal with a frequency 8 times the bit rate when the CCR2.ABCS bit is 1.When using the GPT clock The SCKn pin is available for use as an I/O port in accord with the I/O port settings when the GPT clock is used. In Manchester mode and Simple LIN mode:Prohibited In Clock synchronous mode and Simple SPI mode:External clock (slave operation)The SCKn pin functions as the clock input pin. In Smart card interface mode and when CCR3.GM = 0:Prohibited In Smart card interface mode and when CCR3.GM = 1:Clock output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acs0_SPEC;
    pub type Acs0 = crate::EnumBitfieldStruct<u8, Acs0_SPEC>;
    impl Acs0 {
        #[doc = "External clock input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Logical AND of compare matches output from the internal GPT"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gm_SPEC;
    pub type Gm = crate::EnumBitfieldStruct<u8, Gm_SPEC>;
    impl Gm {
        #[doc = "Non-GSM mode operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "GSM mode operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blk_SPEC;
    pub type Blk = crate::EnumBitfieldStruct<u8, Blk_SPEC>;
    impl Blk {
        #[doc = "Non-block transfer mode operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Block transfer mode operation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr4_SPEC;
impl crate::sealed::RegSpec for Ccr4_SPEC {
    type DataType = u32;
}

#[doc = "Common Control Register 4"]
pub type Ccr4 = crate::RegValueT<Ccr4_SPEC>;

impl Ccr4 {
    #[doc = "Compare Match Data"]
    #[inline(always)]
    pub fn cmpd(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Ccr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Ccr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Adjust Receive Sampling Timing Enable"]
    #[inline(always)]
    pub fn asen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ccr4::Asen,
        ccr4::Asen,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ccr4::Asen,
            ccr4::Asen,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjust Transmit Timing Enable"]
    #[inline(always)]
    pub fn aten(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ccr4::Aten,
        ccr4::Aten,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ccr4::Aten,
            ccr4::Aten,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustment Value for Receive Sampling Timing"]
    #[inline(always)]
    pub fn ast(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        ccr4::Ast,
        ccr4::Ast,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            ccr4::Ast,
            ccr4::Ast,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustment Direction for Receive Sampling Timing"]
    #[inline(always)]
    pub fn ajd(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        ccr4::Ajd,
        ccr4::Ajd,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            ccr4::Ajd,
            ccr4::Ajd,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustment Value for Transmit Timing"]
    #[inline(always)]
    pub fn att(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Ccr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Ccr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Adjustment Edge for Transmit Timing"]
    #[inline(always)]
    pub fn aet(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        ccr4::Aet,
        ccr4::Aet,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            ccr4::Aet,
            ccr4::Aet,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccr4 {
    #[inline(always)]
    fn default() -> Ccr4 {
        <crate::RegValueT<Ccr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asen_SPEC;
    pub type Asen = crate::EnumBitfieldStruct<u8, Asen_SPEC>;
    impl Asen {
        #[doc = "Adjust sampling timing disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Adjust sampling timing enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aten_SPEC;
    pub type Aten = crate::EnumBitfieldStruct<u8, Aten_SPEC>;
    impl Aten {
        #[doc = "Adjust transmit timing disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Adjust transmit timing enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ast_SPEC;
    pub type Ast = crate::EnumBitfieldStruct<u8, Ast_SPEC>;
    impl Ast {
        #[doc = "1-TCLK delay"]
        pub const _000: Self = Self::new(0);

        #[doc = "2-TCLK delay"]
        pub const _001: Self = Self::new(1);

        #[doc = "3-TCLK delay"]
        pub const _010: Self = Self::new(2);

        #[doc = "4-TCLK delay"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajd_SPEC;
    pub type Ajd = crate::EnumBitfieldStruct<u8, Ajd_SPEC>;
    impl Ajd {
        #[doc = "The sampling timing is adjusted backward to the middle of bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "The sampling timing is adjusted forward to the middle of bit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aet_SPEC;
    pub type Aet = crate::EnumBitfieldStruct<u8, Aet_SPEC>;
    impl Aet {
        #[doc = "When CCR1.TINV is 0, adjust the rising edge timing. When CCR1.TINV is 1, adjust the falling edge timing."]
        pub const _0: Self = Self::new(0);

        #[doc = "When CCR1.TINV is 0, adjust the falling edge timing. When CCR1.TINV is 1, adjust the rising edge timing."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cesr_SPEC;
impl crate::sealed::RegSpec for Cesr_SPEC {
    type DataType = u8;
}

#[doc = "Communication Enable Status Register"]
pub type Cesr = crate::RegValueT<Cesr_SPEC>;

impl Cesr {
    #[doc = "RE Internal Status"]
    #[inline(always)]
    pub fn rist(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cesr::Rist,
        cesr::Rist,
        Cesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cesr::Rist,
            cesr::Rist,
            Cesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "TE Internal Status"]
    #[inline(always)]
    pub fn tist(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cesr::Tist,
        cesr::Tist,
        Cesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cesr::Tist,
            cesr::Tist,
            Cesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cesr {
    #[inline(always)]
    fn default() -> Cesr {
        <crate::RegValueT<Cesr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cesr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rist_SPEC;
    pub type Rist = crate::EnumBitfieldStruct<u8, Rist_SPEC>;
    impl Rist {
        #[doc = "RE signal internal state value 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "RE signal internal state value 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tist_SPEC;
    pub type Tist = crate::EnumBitfieldStruct<u8, Tist_SPEC>;
    impl Tist {
        #[doc = "TE signal internal state value 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "TE signal internal state value 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr_SPEC;
impl crate::sealed::RegSpec for Icr_SPEC {
    type DataType = u32;
}

#[doc = "Simple IIC Control Register"]
pub type Icr = crate::RegValueT<Icr_SPEC>;

impl Icr {
    #[doc = "SDA Delay Output Select"]
    #[inline(always)]
    pub fn iicdl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Icr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IIC Interrupt Mode Select"]
    #[inline(always)]
    pub fn iicintm(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icr::Iicintm,
        icr::Iicintm,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icr::Iicintm,
            icr::Iicintm,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Synchronization"]
    #[inline(always)]
    pub fn iiccsc(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icr::Iiccsc,
        icr::Iiccsc,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icr::Iiccsc,
            icr::Iiccsc,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACK Transmission Data"]
    #[inline(always)]
    pub fn iicackt(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icr::Iicackt,
        icr::Iicackt,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icr::Iicackt,
            icr::Iicackt,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start Condition Generation"]
    #[inline(always)]
    pub fn iicstareq(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icr::Iicstareq,
        icr::Iicstareq,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icr::Iicstareq,
            icr::Iicstareq,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Restart Condition Generation"]
    #[inline(always)]
    pub fn iicrstareq(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        icr::Iicrstareq,
        icr::Iicrstareq,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            icr::Iicrstareq,
            icr::Iicrstareq,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Stop Condition Generation"]
    #[inline(always)]
    pub fn iicstpreq(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icr::Iicstpreq,
        icr::Iicstpreq,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icr::Iicstpreq,
            icr::Iicstpreq,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDA Output Select"]
    #[inline(always)]
    pub fn iicsdas(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        icr::Iicsdas,
        icr::Iicsdas,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            icr::Iicsdas,
            icr::Iicsdas,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCL Output Select"]
    #[inline(always)]
    pub fn iicscls(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x3,
        1,
        0,
        icr::Iicscls,
        icr::Iicscls,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x3,
            1,
            0,
            icr::Iicscls,
            icr::Iicscls,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        <crate::RegValueT<Icr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicintm_SPEC;
    pub type Iicintm = crate::EnumBitfieldStruct<u8, Iicintm_SPEC>;
    impl Iicintm {
        #[doc = "Use ACK/NACK interrupts"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use reception and transmission interrupts"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iiccsc_SPEC;
    pub type Iiccsc = crate::EnumBitfieldStruct<u8, Iiccsc_SPEC>;
    impl Iiccsc {
        #[doc = "No synchronization with the clock signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronization with the clock signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicackt_SPEC;
    pub type Iicackt = crate::EnumBitfieldStruct<u8, Iicackt_SPEC>;
    impl Iicackt {
        #[doc = "ACK transmission"]
        pub const _0: Self = Self::new(0);

        #[doc = "NACK transmission and reception of ACK/NACK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstareq_SPEC;
    pub type Iicstareq = crate::EnumBitfieldStruct<u8, Iicstareq_SPEC>;
    impl Iicstareq {
        #[doc = "A start condition is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "A start condition is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicrstareq_SPEC;
    pub type Iicrstareq = crate::EnumBitfieldStruct<u8, Iicrstareq_SPEC>;
    impl Iicrstareq {
        #[doc = "A restart condition is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "A restart condition is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstpreq_SPEC;
    pub type Iicstpreq = crate::EnumBitfieldStruct<u8, Iicstpreq_SPEC>;
    impl Iicstpreq {
        #[doc = "A stop condition is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A stop condition is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicsdas_SPEC;
    pub type Iicsdas = crate::EnumBitfieldStruct<u8, Iicsdas_SPEC>;
    impl Iicsdas {
        #[doc = "Serial data output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Generate a start, restart, or stop condition"]
        pub const _01: Self = Self::new(1);

        #[doc = "Output the low level on the SDAn pin"]
        pub const _10: Self = Self::new(2);

        #[doc = "Place the SDAn pin in the high-impedance state"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicscls_SPEC;
    pub type Iicscls = crate::EnumBitfieldStruct<u8, Iicscls_SPEC>;
    impl Iicscls {
        #[doc = "Serial clock output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Generate a start, restart, or stop condition"]
        pub const _01: Self = Self::new(1);

        #[doc = "Output the low level on the SCLn pin"]
        pub const _10: Self = Self::new(2);

        #[doc = "Place the SCLn pin in the high-impedance state"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcr_SPEC;
impl crate::sealed::RegSpec for Fcr_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Control Register"]
pub type Fcr = crate::RegValueT<Fcr_SPEC>;

impl Fcr {
    #[doc = "Receive Data Ready Error Select Bit"]
    #[inline(always)]
    pub fn dres(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, fcr::Dres, fcr::Dres, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fcr::Dres,
            fcr::Dres,
            Fcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit FIFO Data Trigger Number"]
    #[inline(always)]
    pub fn ttrg(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit FIFO Data Register Reset"]
    #[inline(always)]
    pub fn tfrst(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fcr::Tfrst,
        fcr::Tfrst,
        Fcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fcr::Tfrst,
            fcr::Tfrst,
            Fcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Receive FIFO Data Trigger Number"]
    #[inline(always)]
    pub fn rtrg(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive FIFO Data Register Reset"]
    #[inline(always)]
    pub fn rfrst(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        fcr::Rfrst,
        fcr::Rfrst,
        Fcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            fcr::Rfrst,
            fcr::Rfrst,
            Fcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "RTS Output Active Trigger Number Select"]
    #[inline(always)]
    pub fn rstrg(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fcr {
    #[inline(always)]
    fn default() -> Fcr {
        <crate::RegValueT<Fcr_SPEC> as RegisterValue<_>>::new(522125312)
    }
}
pub mod fcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dres_SPEC;
    pub type Dres = crate::EnumBitfieldStruct<u8, Dres_SPEC>;
    impl Dres {
        #[doc = "Reception data full interrupt (SCIn_RXI)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Receive error interrupt (SCIn_ERI)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfrst_SPEC;
    pub type Tfrst = crate::EnumBitfieldStruct<u8, Tfrst_SPEC>;
    impl Tfrst {
        #[doc = "When set, this bit is invalid and does not affect operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "The number of data stored in Transmit-FIFO (TDR register) is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfrst_SPEC;
    pub type Rfrst = crate::EnumBitfieldStruct<u8, Rfrst_SPEC>;
    impl Rfrst {
        #[doc = "When set, this bit is invalid and does not affect operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "The number of data stored in Receive-FIFO(RDR register) is 0"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr_SPEC;
impl crate::sealed::RegSpec for Mcr_SPEC {
    type DataType = u32;
}

#[doc = "Manchester Control Register"]
pub type Mcr = crate::RegValueT<Mcr_SPEC>;

impl Mcr {
    #[doc = "Polarity of Received Manchester Code"]
    #[inline(always)]
    pub fn rmpol(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mcr::Rmpol,
        mcr::Rmpol,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mcr::Rmpol,
            mcr::Rmpol,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Polarity of Transmit Manchester Code"]
    #[inline(always)]
    pub fn tmpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mcr::Tmpol,
        mcr::Tmpol,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mcr::Tmpol,
            mcr::Tmpol,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Manchester Edge Retiming Enable"]
    #[inline(always)]
    pub fn erten(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mcr::Erten,
        mcr::Erten,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mcr::Erten,
            mcr::Erten,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYNC value Setting"]
    #[inline(always)]
    pub fn synval(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SYNC Select"]
    #[inline(always)]
    pub fn synsel(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mcr::Synsel,
        mcr::Synsel,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mcr::Synsel,
            mcr::Synsel,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start Bit Select"]
    #[inline(always)]
    pub fn sbsel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mcr::Sbsel,
        mcr::Sbsel,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mcr::Sbsel,
            mcr::Sbsel,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Preface Length"]
    #[inline(always)]
    pub fn tplen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        mcr::Tplen,
        mcr::Tplen,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            mcr::Tplen,
            mcr::Tplen,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Preface Pattern"]
    #[inline(always)]
    pub fn tppat(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        mcr::Tppat,
        mcr::Tppat,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            mcr::Tppat,
            mcr::Tppat,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Preface Length"]
    #[inline(always)]
    pub fn rplen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        mcr::Rplen,
        mcr::Rplen,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            mcr::Rplen,
            mcr::Rplen,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Preface Pattern"]
    #[inline(always)]
    pub fn rppat(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        mcr::Rppat,
        mcr::Rppat,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            mcr::Rppat,
            mcr::Rppat,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Preface Error Enable"]
    #[inline(always)]
    pub fn pferen(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mcr::Pferen,
        mcr::Pferen,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mcr::Pferen,
            mcr::Pferen,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive SYNC Error Enable"]
    #[inline(always)]
    pub fn syeren(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mcr::Syeren,
        mcr::Syeren,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mcr::Syeren,
            mcr::Syeren,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start Bit Error Enable"]
    #[inline(always)]
    pub fn sberen(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        mcr::Sberen,
        mcr::Sberen,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            mcr::Sberen,
            mcr::Sberen,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        <crate::RegValueT<Mcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmpol_SPEC;
    pub type Rmpol = crate::EnumBitfieldStruct<u8, Rmpol_SPEC>;
    impl Rmpol {
        #[doc = "Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code"]
        pub const _0: Self = Self::new(0);

        #[doc = "Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmpol_SPEC;
    pub type Tmpol = crate::EnumBitfieldStruct<u8, Tmpol_SPEC>;
    impl Tmpol {
        #[doc = "Logic 0 is coded as a zero-to-one transition in Manchester code Logic 1 is coded as a one-to-zero transition in Manchester code"]
        pub const _0: Self = Self::new(0);

        #[doc = "Logic 0 is coded as a one-to-zero transition in Manchester code Logic 1 is coded as a zero-to-one transition in Manchester code"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Erten_SPEC;
    pub type Erten = crate::EnumBitfieldStruct<u8, Erten_SPEC>;
    impl Erten {
        #[doc = "Disables the receive retiming function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables the receive retiming function"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Synsel_SPEC;
    pub type Synsel = crate::EnumBitfieldStruct<u8, Synsel_SPEC>;
    impl Synsel {
        #[doc = "The start bit pattern is set with the SYNVAL bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "The start bit pattern is set with the TSYNC bit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbsel_SPEC;
    pub type Sbsel = crate::EnumBitfieldStruct<u8, Sbsel_SPEC>;
    impl Sbsel {
        #[doc = "The start bit area consists of one bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "The start bit area consists of three bits (COMMAND SYNC or DATA SYNC)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tplen_SPEC;
    pub type Tplen = crate::EnumBitfieldStruct<u8, Tplen_SPEC>;
    impl Tplen {
        #[doc = "Disables the transmit preface generation"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Transmit preface length (bit length)"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tppat_SPEC;
    pub type Tppat = crate::EnumBitfieldStruct<u8, Tppat_SPEC>;
    impl Tppat {
        #[doc = "ALL ZERO"]
        pub const _00: Self = Self::new(0);

        #[doc = "ZERO ONE"]
        pub const _01: Self = Self::new(1);

        #[doc = "ONE ZERO"]
        pub const _10: Self = Self::new(2);

        #[doc = "ALL ONE"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rplen_SPEC;
    pub type Rplen = crate::EnumBitfieldStruct<u8, Rplen_SPEC>;
    impl Rplen {
        #[doc = "Disables the receive preface generation"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Receive preface length (bit length)"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rppat_SPEC;
    pub type Rppat = crate::EnumBitfieldStruct<u8, Rppat_SPEC>;
    impl Rppat {
        #[doc = "ALL ZERO"]
        pub const _00: Self = Self::new(0);

        #[doc = "ZERO ONE"]
        pub const _01: Self = Self::new(1);

        #[doc = "ONE ZERO"]
        pub const _10: Self = Self::new(2);

        #[doc = "ALL ONE"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pferen_SPEC;
    pub type Pferen = crate::EnumBitfieldStruct<u8, Pferen_SPEC>;
    impl Pferen {
        #[doc = "Does not handle a preface error as an interrupt source"]
        pub const _0: Self = Self::new(0);

        #[doc = "Handles a preface error as an interrupt source"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syeren_SPEC;
    pub type Syeren = crate::EnumBitfieldStruct<u8, Syeren_SPEC>;
    impl Syeren {
        #[doc = "Does not handle a receive SYNC error as an interrupt source"]
        pub const _0: Self = Self::new(0);

        #[doc = "Handles a receive SYNC error as an interrupt source"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sberen_SPEC;
    pub type Sberen = crate::EnumBitfieldStruct<u8, Sberen_SPEC>;
    impl Sberen {
        #[doc = "Does not handle a start bit error as an interrupt source"]
        pub const _0: Self = Self::new(0);

        #[doc = "Handles a start bit error as an interrupt source"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcr_SPEC;
impl crate::sealed::RegSpec for Dcr_SPEC {
    type DataType = u32;
}

#[doc = "Driver Control Register"]
pub type Dcr = crate::RegValueT<Dcr_SPEC>;

impl Dcr {
    #[doc = "Driver Effective Polarity Select"]
    #[inline(always)]
    pub fn depol(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dcr::Depol,
        dcr::Depol,
        Dcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dcr::Depol,
            dcr::Depol,
            Dcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Driver Assertion Time"]
    #[inline(always)]
    pub fn deast(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Dcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Dcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Driver Negate Time"]
    #[inline(always)]
    pub fn dengt(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Dcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Dcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dcr {
    #[inline(always)]
    fn default() -> Dcr {
        <crate::RegValueT<Dcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Depol_SPEC;
    pub type Depol = crate::EnumBitfieldStruct<u8, Depol_SPEC>;
    impl Depol {
        #[doc = "The DEn signal is active-high"]
        pub const _0: Self = Self::new(0);

        #[doc = "The DEn signal is active-low"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr0_SPEC;
impl crate::sealed::RegSpec for Xcr0_SPEC {
    type DataType = u32;
}

#[doc = "Simple LIN Control Register 0"]
pub type Xcr0 = crate::RegValueT<Xcr0_SPEC>;

impl Xcr0 {
    #[doc = "Timer Count Clock Source Selection"]
    #[inline(always)]
    pub fn tcss(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        xcr0::Tcss,
        xcr0::Tcss,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            xcr0::Tcss,
            xcr0::Tcss,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Break Field Enable"]
    #[inline(always)]
    pub fn bfe(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        xcr0::Bfe,
        xcr0::Bfe,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            xcr0::Bfe,
            xcr0::Bfe,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control Field 0 Enable"]
    #[inline(always)]
    pub fn cf0re(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        xcr0::Cf0Re,
        xcr0::Cf0Re,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            xcr0::Cf0Re,
            xcr0::Cf0Re,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control Field1 Compare Data Select"]
    #[inline(always)]
    pub fn cf1ds(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        xcr0::Cf1Ds,
        xcr0::Cf1Ds,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            xcr0::Cf1Ds,
            xcr0::Cf1Ds,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Priority Interrupt Bit Enable"]
    #[inline(always)]
    pub fn pibe(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        xcr0::Pibe,
        xcr0::Pibe,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            xcr0::Pibe,
            xcr0::Pibe,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Priority Interrupt Bit Select"]
    #[inline(always)]
    pub fn pibs(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x7,
        1,
        0,
        xcr0::Pibs,
        xcr0::Pibs,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x7,
            1,
            0,
            xcr0::Pibs,
            xcr0::Pibs,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Break Field Output Completion Interrupt Enable"]
    #[inline(always)]
    pub fn bfoie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        xcr0::Bfoie,
        xcr0::Bfoie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            xcr0::Bfoie,
            xcr0::Bfoie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Conflict Detection Interrupt Enable"]
    #[inline(always)]
    pub fn bcdie(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        xcr0::Bcdie,
        xcr0::Bcdie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            xcr0::Bcdie,
            xcr0::Bcdie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Break Field Detection Interrupt Enable"]
    #[inline(always)]
    pub fn bfdie(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        xcr0::Bfdie,
        xcr0::Bfdie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            xcr0::Bfdie,
            xcr0::Bfdie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Counter Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn cofie(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        xcr0::Cofie,
        xcr0::Cofie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            xcr0::Cofie,
            xcr0::Cofie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Active Edge Detection Interrupt Enable"]
    #[inline(always)]
    pub fn aedie(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        xcr0::Aedie,
        xcr0::Aedie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            xcr0::Aedie,
            xcr0::Aedie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Conflict Detection Clock Selection"]
    #[inline(always)]
    pub fn bccs(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        xcr0::Bccs,
        xcr0::Bccs,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            xcr0::Bccs,
            xcr0::Bccs,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xcr0 {
    #[inline(always)]
    fn default() -> Xcr0 {
        <crate::RegValueT<Xcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcss_SPEC;
    pub type Tcss = crate::EnumBitfieldStruct<u8, Tcss_SPEC>;
    impl Tcss {
        #[doc = "TCLK/4"]
        pub const _01: Self = Self::new(1);

        #[doc = "TCLK/16"]
        pub const _10: Self = Self::new(2);

        #[doc = "TCLK/64"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfe_SPEC;
    pub type Bfe = crate::EnumBitfieldStruct<u8, Bfe_SPEC>;
    impl Bfe {
        #[doc = "No Break Field"]
        pub const _0: Self = Self::new(0);

        #[doc = "With Break Field"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Re_SPEC;
    pub type Cf0Re = crate::EnumBitfieldStruct<u8, Cf0Re_SPEC>;
    impl Cf0Re {
        #[doc = "No Control Field 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "With Control Field 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ds_SPEC;
    pub type Cf1Ds = crate::EnumBitfieldStruct<u8, Cf1Ds_SPEC>;
    impl Cf1Ds {
        #[doc = "Select XCR1.PCF1D\\[7:0\\] as the compare data"]
        pub const _00: Self = Self::new(0);

        #[doc = "Select XCR1.SCF1D\\[7:0\\] as the compare data"]
        pub const _01: Self = Self::new(1);

        #[doc = "Select both XCR1.PCF1D\\[7:0\\] and XCR1.SCF1D\\[7:0\\] as the compare data"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibe_SPEC;
    pub type Pibe = crate::EnumBitfieldStruct<u8, Pibe_SPEC>;
    impl Pibe {
        #[doc = "Priority interrupt bit disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Priority interrupt bit enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibs_SPEC;
    pub type Pibs = crate::EnumBitfieldStruct<u8, Pibs_SPEC>;
    impl Pibs {
        #[doc = "Bit 0 of Control Field 1"]
        pub const _000: Self = Self::new(0);

        #[doc = "Bit 1 of Control Field 1"]
        pub const _001: Self = Self::new(1);

        #[doc = "Bit 2 of Control Field 1"]
        pub const _010: Self = Self::new(2);

        #[doc = "Bit 3 of Control Field 1"]
        pub const _011: Self = Self::new(3);

        #[doc = "Bit 4 of Control Field 1"]
        pub const _100: Self = Self::new(4);

        #[doc = "Bit 5 of Control Field 1"]
        pub const _101: Self = Self::new(5);

        #[doc = "Bit 6 of Control Field 1"]
        pub const _110: Self = Self::new(6);

        #[doc = "Bit 7 of Control Field 1"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfoie_SPEC;
    pub type Bfoie = crate::EnumBitfieldStruct<u8, Bfoie_SPEC>;
    impl Bfoie {
        #[doc = "Break Field output completion is not included in SCIn_TXI interrupt factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Break Field output completion is included in SCIn_TXI interrupt factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcdie_SPEC;
    pub type Bcdie = crate::EnumBitfieldStruct<u8, Bcdie_SPEC>;
    impl Bcdie {
        #[doc = "Bus conflict detection is not included in SCIn_ERI interrupt factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus conflict detection is included in SCIn_ERI interrupt factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfdie_SPEC;
    pub type Bfdie = crate::EnumBitfieldStruct<u8, Bfdie_SPEC>;
    impl Bfdie {
        #[doc = "Break Field detection interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Break Field detection interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cofie_SPEC;
    pub type Cofie = crate::EnumBitfieldStruct<u8, Cofie_SPEC>;
    impl Cofie {
        #[doc = "Counter overflow is not included in SCIn_ERI interrupt factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter overflow is included in SCIn_ERI interrupt factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aedie_SPEC;
    pub type Aedie = crate::EnumBitfieldStruct<u8, Aedie_SPEC>;
    impl Aedie {
        #[doc = "Active edge detection interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Active edge detection interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bccs_SPEC;
    pub type Bccs = crate::EnumBitfieldStruct<u8, Bccs_SPEC>;
    impl Bccs {
        #[doc = "Base clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "Base clock/2"]
        pub const _01: Self = Self::new(1);

        #[doc = "Base clock/4"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr1_SPEC;
impl crate::sealed::RegSpec for Xcr1_SPEC {
    type DataType = u32;
}

#[doc = "Simple LIN Control Register 1"]
pub type Xcr1 = crate::RegValueT<Xcr1_SPEC>;

impl Xcr1 {
    #[doc = "Break Field Output Timer Count Start Trigger"]
    #[inline(always)]
    pub fn tcst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        xcr1::Tcst,
        xcr1::Tcst,
        Xcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            xcr1::Tcst,
            xcr1::Tcst,
            Xcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start Frame Detection Enable"]
    #[inline(always)]
    pub fn sdst(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        xcr1::Sdst,
        xcr1::Sdst,
        Xcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            xcr1::Sdst,
            xcr1::Sdst,
            Xcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bit Rate Measurement Enable"]
    #[inline(always)]
    pub fn bmen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        xcr1::Bmen,
        xcr1::Bmen,
        Xcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            xcr1::Bmen,
            xcr1::Bmen,
            Xcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Priority Compare Data for Control Field 1"]
    #[inline(always)]
    pub fn pcf1d(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Xcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Xcr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Secondary Compare Data for Control Field 1"]
    #[inline(always)]
    pub fn scf1d(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Xcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Xcr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Control Field 1 Compare Bit Enable"]
    #[inline(always)]
    pub fn cf1ce(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        xcr1::Cf1Ce,
        xcr1::Cf1Ce,
        Xcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            xcr1::Cf1Ce,
            xcr1::Cf1Ce,
            Xcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xcr1 {
    #[inline(always)]
    fn default() -> Xcr1 {
        <crate::RegValueT<Xcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcst_SPEC;
    pub type Tcst = crate::EnumBitfieldStruct<u8, Tcst_SPEC>;
    impl Tcst {
        #[doc = "Break Field output timer count stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Break Field output timer count start"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdst_SPEC;
    pub type Sdst = crate::EnumBitfieldStruct<u8, Sdst_SPEC>;
    impl Sdst {
        #[doc = "Start Frame/Break Field detection disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start Frame/Break Field detection enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bmen_SPEC;
    pub type Bmen = crate::EnumBitfieldStruct<u8, Bmen_SPEC>;
    impl Bmen {
        #[doc = "Bit rate measurement disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit rate measurement enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ce_SPEC;
    pub type Cf1Ce = crate::EnumBitfieldStruct<u8, Cf1Ce_SPEC>;
    impl Cf1Ce {
        #[doc = "Control Field 1 bit N compare disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Control Field 1 bit N compare enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr2_SPEC;
impl crate::sealed::RegSpec for Xcr2_SPEC {
    type DataType = u32;
}

#[doc = "Simple LIN Control Register 2"]
pub type Xcr2 = crate::RegValueT<Xcr2_SPEC>;

impl Xcr2 {
    #[doc = "Control Field 0 Compare Data"]
    #[inline(always)]
    pub fn cf0d(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Xcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Xcr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Control Field 0 Compare Bit Enable"]
    #[inline(always)]
    pub fn cf0ce(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        xcr2::Cf0Ce,
        xcr2::Cf0Ce,
        Xcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            xcr2::Cf0Ce,
            xcr2::Cf0Ce,
            Xcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Break Field Length Setting"]
    #[inline(always)]
    pub fn bflw(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Xcr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Xcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr2 {
    #[inline(always)]
    fn default() -> Xcr2 {
        <crate::RegValueT<Xcr2_SPEC> as RegisterValue<_>>::new(4294836224)
    }
}
pub mod xcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Ce_SPEC;
    pub type Cf0Ce = crate::EnumBitfieldStruct<u8, Cf0Ce_SPEC>;
    impl Cf0Ce {
        #[doc = "Control Field 0 bit N compare disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Control Field 0 bit N compare enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr_SPEC;
impl crate::sealed::RegSpec for Csr_SPEC {
    type DataType = u32;
}

#[doc = "Common Status Register"]
pub type Csr = crate::RegValueT<Csr_SPEC>;

impl Csr {
    #[doc = "Error Signal Status Flag"]
    #[inline(always)]
    pub fn ers(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, csr::Ers, csr::Ers, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,csr::Ers,csr::Ers,Csr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Serial Input Data Monitor Bit"]
    #[inline(always)]
    pub fn rxdmon(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        csr::Rxdmon,
        csr::Rxdmon,
        Csr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            csr::Rxdmon,
            csr::Rxdmon,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Compare Match Flag"]
    #[inline(always)]
    pub fn dcmf(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, csr::Dcmf, csr::Dcmf, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            csr::Dcmf,
            csr::Dcmf,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Compare Match Parity Error Flag"]
    #[inline(always)]
    pub fn dper(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, csr::Dper, csr::Dper, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            csr::Dper,
            csr::Dper,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Compare Match Framing Error Flag"]
    #[inline(always)]
    pub fn dfer(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, csr::Dfer, csr::Dfer, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            csr::Dfer,
            csr::Dfer,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, csr::Orer, csr::Orer, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            csr::Orer,
            csr::Orer,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Mode Fault Error Flag"]
    #[inline(always)]
    pub fn mff(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, csr::Mff, csr::Mff, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<26,0x1,1,0,csr::Mff,csr::Mff,Csr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, csr::Per, csr::Per, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<27,0x1,1,0,csr::Per,csr::Per,Csr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Framing Error Flag"]
    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, csr::Fer, csr::Fer, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<28,0x1,1,0,csr::Fer,csr::Fer,Csr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, csr::Tdre, csr::Tdre, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            csr::Tdre,
            csr::Tdre,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, csr::Tend, csr::Tend, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            csr::Tend,
            csr::Tend,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, csr::Rdrf, csr::Rdrf, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            csr::Rdrf,
            csr::Rdrf,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        <crate::RegValueT<Csr_SPEC> as RegisterValue<_>>::new(1610645504)
    }
}
pub mod csr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ers_SPEC;
    pub type Ers = crate::EnumBitfieldStruct<u8, Ers_SPEC>;
    impl Ers {
        #[doc = "Error signal low not responded"]
        pub const _0: Self = Self::new(0);

        #[doc = "Error signal low responded"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdmon_SPEC;
    pub type Rxdmon = crate::EnumBitfieldStruct<u8, Rxdmon_SPEC>;
    impl Rxdmon {
        #[doc = "When RINV is 0, RXDn pin is the low level. When RINV is 1, RXDn pin is the high level."]
        pub const _0: Self = Self::new(0);

        #[doc = "When RINV is 0, RXDn pin is the high level. When RINV is 1, RXDn pin is the low level."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcmf_SPEC;
    pub type Dcmf = crate::EnumBitfieldStruct<u8, Dcmf_SPEC>;
    impl Dcmf {
        #[doc = "No matched"]
        pub const _0: Self = Self::new(0);

        #[doc = "Matched"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dper_SPEC;
    pub type Dper = crate::EnumBitfieldStruct<u8, Dper_SPEC>;
    impl Dper {
        #[doc = "No parity error occurred at address match detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "A parity error has occurred at address match detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfer_SPEC;
    pub type Dfer = crate::EnumBitfieldStruct<u8, Dfer_SPEC>;
    impl Dfer {
        #[doc = "No framing error occurred at address match detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "A framing error has occurred at address match detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        #[doc = "No overrun error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "An overrun error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mff_SPEC;
    pub type Mff = crate::EnumBitfieldStruct<u8, Mff_SPEC>;
    impl Mff {
        #[doc = "No mode fault error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mode fault error"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "Non-FIFO selected (CCR3.FM = 0): No parity error occurred FIFO selected (CCR3.FM = 1): No parity error in all received data in receive-FIFO"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-FIFO selected (CCR3.FM = 0): A parity error has occurred FIFO selected (CCR3.FM = 1): One or more parity errors occurred in received data in receive-FIFO"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        #[doc = "Non-FIFO selected (CCR3.FM = 0): No framing error occurred FIFO selected (CCR3.FM = 1): No framing error in all received data in receive-FIFO"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-FIFO selected (CCR3.FM = 0): A framing error has occurred FIFO selected (CCR3.FM = 1): One or more framing errors occurred in received data in receive-FIFO"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        #[doc = "Non-FIFO selected (CCR3.FM = 0): Transmit data is in TDR register FIFO selected (CCR3.FM = 1): The quantity of transmit data written in transmit-FIFO exceeds the specified transmit triggering number."]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-FIFO selected (CCR3.FM = 0): No transmit data is in TDR register FIFO selected (CCR3.FM = 1): The quantity of transmit data written in transmit-FIFO is equal to or less than the specified transmit triggering number."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        #[doc = "A character is being transmitted or standing by for transmission."]
        pub const _0: Self = Self::new(0);

        #[doc = "Character transfer has been completed, or sending Break Field."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrf_SPEC;
    pub type Rdrf = crate::EnumBitfieldStruct<u8, Rdrf_SPEC>;
    impl Rdrf {
        #[doc = "Non-FIFO selected (CCR3.FM = 0): No received data is in RDR register FIFO selected (CCR3.FM = 1): The quantity of receive data written in receive-FIFO falls below the specified receive triggering number."]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-FIFO selected (CCR3.FM = 0): Received data is in RDR register FIFO selected (CCR3.FM = 1): The quantity of receive data written in receive-FIFO is equal to or greater than the specified receive triggering number."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr_SPEC;
impl crate::sealed::RegSpec for Isr_SPEC {
    type DataType = u32;
}

#[doc = "Simple IIC Status Register"]
pub type Isr = crate::RegValueT<Isr_SPEC>;

impl Isr {
    #[doc = "ACK Reception Data Flag"]
    #[inline(always)]
    pub fn iicackr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        isr::Iicackr,
        isr::Iicackr,
        Isr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            isr::Iicackr,
            isr::Iicackr,
            Isr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Issuing of Start, Restart, or Stop Condition Completed Flag"]
    #[inline(always)]
    pub fn iicstif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        isr::Iicstif,
        isr::Iicstif,
        Isr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            isr::Iicstif,
            isr::Iicstif,
            Isr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        <crate::RegValueT<Isr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod isr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicackr_SPEC;
    pub type Iicackr = crate::EnumBitfieldStruct<u8, Iicackr_SPEC>;
    impl Iicackr {
        #[doc = "ACK received"]
        pub const _0: Self = Self::new(0);

        #[doc = "NACK received"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstif_SPEC;
    pub type Iicstif = crate::EnumBitfieldStruct<u8, Iicstif_SPEC>;
    impl Iicstif {
        #[doc = "There are no requests for generating conditions or a condition is being generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A start, restart, or stop condition is completely generated."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frsr_SPEC;
impl crate::sealed::RegSpec for Frsr_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Receive Status Register"]
pub type Frsr = crate::RegValueT<Frsr_SPEC>;

impl Frsr {
    #[doc = "Receive Data Ready Flag"]
    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, frsr::Dr, frsr::Dr, Frsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,frsr::Dr,frsr::Dr,Frsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive-FIFO Data Count"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Frsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Frsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Parity Error Count"]
    #[inline(always)]
    pub fn pnum(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Frsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,Frsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Framing Error Count"]
    #[inline(always)]
    pub fn fnum(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, Frsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3f,1,0,u8,u8,Frsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Frsr {
    #[inline(always)]
    fn default() -> Frsr {
        <crate::RegValueT<Frsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod frsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
        #[doc = "Receiving is in progress, or no received data remains in the receive-FIFO after a successful reception (receive-FIFO is empty)"]
        pub const _0: Self = Self::new(0);

        #[doc = "The following receive data is not received for a fixed period after storing data under the threshold in the receive-FIFO"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftsr_SPEC;
impl crate::sealed::RegSpec for Ftsr_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Transmit Status Register"]
pub type Ftsr = crate::RegValueT<Ftsr_SPEC>;

impl Ftsr {
    #[doc = "Transmit-FIFO Data Count"]
    #[inline(always)]
    pub fn t(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Ftsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Ftsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ftsr {
    #[inline(always)]
    fn default() -> Ftsr {
        <crate::RegValueT<Ftsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msr_SPEC;
impl crate::sealed::RegSpec for Msr_SPEC {
    type DataType = u32;
}

#[doc = "Manchester Status Register"]
pub type Msr = crate::RegValueT<Msr_SPEC>;

impl Msr {
    #[doc = "Preface Error Flag"]
    #[inline(always)]
    pub fn pfer(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, msr::Pfer, msr::Pfer, Msr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,msr::Pfer,msr::Pfer,Msr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SYNC Error Flag"]
    #[inline(always)]
    pub fn syer(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, msr::Syer, msr::Syer, Msr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,msr::Syer,msr::Syer,Msr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Start Bit Error Flag"]
    #[inline(always)]
    pub fn sber(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, msr::Sber, msr::Sber, Msr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,msr::Sber,msr::Sber,Msr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Manchester Error Flag"]
    #[inline(always)]
    pub fn mer(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, msr::Mer, msr::Mer, Msr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,msr::Mer,msr::Mer,Msr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive SYNC Data Bit"]
    #[inline(always)]
    pub fn rsync(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        msr::Rsync,
        msr::Rsync,
        Msr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            msr::Rsync,
            msr::Rsync,
            Msr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Msr {
    #[inline(always)]
    fn default() -> Msr {
        <crate::RegValueT<Msr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfer_SPEC;
    pub type Pfer = crate::EnumBitfieldStruct<u8, Pfer_SPEC>;
    impl Pfer {
        #[doc = "No preface error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Preface error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syer_SPEC;
    pub type Syer = crate::EnumBitfieldStruct<u8, Syer_SPEC>;
    impl Syer {
        #[doc = "No receive SYNC error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Receive SYNC error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sber_SPEC;
    pub type Sber = crate::EnumBitfieldStruct<u8, Sber_SPEC>;
    impl Sber {
        #[doc = "No start bit error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start bit error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mer_SPEC;
    pub type Mer = crate::EnumBitfieldStruct<u8, Mer_SPEC>;
    impl Mer {
        #[doc = "No Manchester error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Manchester error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsync_SPEC;
    pub type Rsync = crate::EnumBitfieldStruct<u8, Rsync_SPEC>;
    impl Rsync {
        #[doc = "The received start bit is DATA SYNC"]
        pub const _0: Self = Self::new(0);

        #[doc = "The received start bit is COMMAND SYNC"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xsr0_SPEC;
impl crate::sealed::RegSpec for Xsr0_SPEC {
    type DataType = u32;
}

#[doc = "Simple LIN Status Register 0"]
pub type Xsr0 = crate::RegValueT<Xsr0_SPEC>;

impl Xsr0 {
    #[doc = "Start Frame Status Flag"]
    #[inline(always)]
    pub fn sfsf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        xsr0::Sfsf,
        xsr0::Sfsf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            xsr0::Sfsf,
            xsr0::Sfsf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "RXDn Input Status Flag"]
    #[inline(always)]
    pub fn rxdsf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        xsr0::Rxdsf,
        xsr0::Rxdsf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            xsr0::Rxdsf,
            xsr0::Rxdsf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Break Field Output Completion Flag"]
    #[inline(always)]
    pub fn bfof(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        xsr0::Bfof,
        xsr0::Bfof,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            xsr0::Bfof,
            xsr0::Bfof,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Bus Conflict Detection Flag"]
    #[inline(always)]
    pub fn bcdf(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        xsr0::Bcdf,
        xsr0::Bcdf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            xsr0::Bcdf,
            xsr0::Bcdf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Break Field Detection Flag"]
    #[inline(always)]
    pub fn bfdf(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        xsr0::Bfdf,
        xsr0::Bfdf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            xsr0::Bfdf,
            xsr0::Bfdf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Control Field 0 Compare Match Flag"]
    #[inline(always)]
    pub fn cf0mf(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        xsr0::Cf0Mf,
        xsr0::Cf0Mf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            xsr0::Cf0Mf,
            xsr0::Cf0Mf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Control Field 1 Compare Match Flag"]
    #[inline(always)]
    pub fn cf1mf(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        xsr0::Cf1Mf,
        xsr0::Cf1Mf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            xsr0::Cf1Mf,
            xsr0::Cf1Mf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Priority Interrupt Bit Detection Flag"]
    #[inline(always)]
    pub fn pibdf(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        xsr0::Pibdf,
        xsr0::Pibdf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            xsr0::Pibdf,
            xsr0::Pibdf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Counter Overflow Flag"]
    #[inline(always)]
    pub fn cof(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        xsr0::Cof,
        xsr0::Cof,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            xsr0::Cof,
            xsr0::Cof,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Active Edge Detection Flag"]
    #[inline(always)]
    pub fn aedf(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        xsr0::Aedf,
        xsr0::Aedf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            xsr0::Aedf,
            xsr0::Aedf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Control Field 0 received data"]
    #[inline(always)]
    pub fn cf0rd(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Xsr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Control Field 1 received data"]
    #[inline(always)]
    pub fn cf1rd(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Xsr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Xsr0 {
    #[inline(always)]
    fn default() -> Xsr0 {
        <crate::RegValueT<Xsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfsf_SPEC;
    pub type Sfsf = crate::EnumBitfieldStruct<u8, Sfsf_SPEC>;
    impl Sfsf {
        #[doc = "Start Frame detection disabled or Start Frame detection complete"]
        pub const _0: Self = Self::new(0);

        #[doc = "Before Start Frame detection or during detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdsf_SPEC;
    pub type Rxdsf = crate::EnumBitfieldStruct<u8, Rxdsf_SPEC>;
    impl Rxdsf {
        #[doc = "RXDn input to SCI is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "RXDn input to SCI is disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfof_SPEC;
    pub type Bfof = crate::EnumBitfieldStruct<u8, Bfof_SPEC>;
    impl Bfof {
        #[doc = "Break Field is being output or is not being output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Completed BF output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcdf_SPEC;
    pub type Bcdf = crate::EnumBitfieldStruct<u8, Bcdf_SPEC>;
    impl Bcdf {
        #[doc = "No bus collision detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detected a bus collision"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfdf_SPEC;
    pub type Bfdf = crate::EnumBitfieldStruct<u8, Bfdf_SPEC>;
    impl Bfdf {
        #[doc = "Break Field is not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Break Field is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Mf_SPEC;
    pub type Cf0Mf = crate::EnumBitfieldStruct<u8, Cf0Mf_SPEC>;
    impl Cf0Mf {
        #[doc = "Control Field 0 received data does not match the setting data"]
        pub const _0: Self = Self::new(0);

        #[doc = "Control Field 0 received data matches the setting data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Mf_SPEC;
    pub type Cf1Mf = crate::EnumBitfieldStruct<u8, Cf1Mf_SPEC>;
    impl Cf1Mf {
        #[doc = "Control Field 1 received data does not match the setting data"]
        pub const _0: Self = Self::new(0);

        #[doc = "Control Field 1 received data matches the setting data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibdf_SPEC;
    pub type Pibdf = crate::EnumBitfieldStruct<u8, Pibdf_SPEC>;
    impl Pibdf {
        #[doc = "Priority interrupt bit is not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Priority interrupt bit is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cof_SPEC;
    pub type Cof = crate::EnumBitfieldStruct<u8, Cof_SPEC>;
    impl Cof {
        #[doc = "Break Field detection counter has not overflowed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Break Field detection counter overflowed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aedf_SPEC;
    pub type Aedf = crate::EnumBitfieldStruct<u8, Aedf_SPEC>;
    impl Aedf {
        #[doc = "No valid edge detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detected a valid edge"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xsr1_SPEC;
impl crate::sealed::RegSpec for Xsr1_SPEC {
    type DataType = u32;
}

#[doc = "Simple LIN Status Register 1"]
pub type Xsr1 = crate::RegValueT<Xsr1_SPEC>;

impl Xsr1 {
    #[doc = "Timer Count Capture Value"]
    #[inline(always)]
    pub fn tcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Xsr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Xsr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Xsr1 {
    #[inline(always)]
    fn default() -> Xsr1 {
        <crate::RegValueT<Xsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfclr_SPEC;
impl crate::sealed::RegSpec for Cfclr_SPEC {
    type DataType = u32;
}

#[doc = "Common Flag Clear Register"]
pub type Cfclr = crate::RegValueT<Cfclr_SPEC>;

impl Cfclr {
    #[doc = "ERS Clear Bit"]
    #[inline(always)]
    pub fn ersc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "DCMF Clear Bit"]
    #[inline(always)]
    pub fn dcmfc(self) -> crate::common::RegisterFieldBool<16, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "DPER Clear Bit"]
    #[inline(always)]
    pub fn dperc(self) -> crate::common::RegisterFieldBool<17, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "DFER Clear Bit"]
    #[inline(always)]
    pub fn dferc(self) -> crate::common::RegisterFieldBool<18, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "ORER Clear Bit"]
    #[inline(always)]
    pub fn orerc(self) -> crate::common::RegisterFieldBool<24, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "MFF Clear Bit"]
    #[inline(always)]
    pub fn mffc(self) -> crate::common::RegisterFieldBool<26, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "PER Clear Bit"]
    #[inline(always)]
    pub fn perc(self) -> crate::common::RegisterFieldBool<27, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "FER Clear Bit"]
    #[inline(always)]
    pub fn ferc(self) -> crate::common::RegisterFieldBool<28, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "TDRE Clear Bit"]
    #[inline(always)]
    pub fn tdrec(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "RDRF Clear Bit"]
    #[inline(always)]
    pub fn rdrfc(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cfclr {
    #[inline(always)]
    fn default() -> Cfclr {
        <crate::RegValueT<Cfclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icfclr_SPEC;
impl crate::sealed::RegSpec for Icfclr_SPEC {
    type DataType = u32;
}

#[doc = "Simple IIC Flag Clear Register"]
pub type Icfclr = crate::RegValueT<Icfclr_SPEC>;

impl Icfclr {
    #[doc = "IICSTIF Clear Bit"]
    #[inline(always)]
    pub fn iicstifc(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Icfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Icfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Icfclr {
    #[inline(always)]
    fn default() -> Icfclr {
        <crate::RegValueT<Icfclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffclr_SPEC;
impl crate::sealed::RegSpec for Ffclr_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Flag Clear Register"]
pub type Ffclr = crate::RegValueT<Ffclr_SPEC>;

impl Ffclr {
    #[doc = "DR Clear Bit"]
    #[inline(always)]
    pub fn drc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ffclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ffclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ffclr {
    #[inline(always)]
    fn default() -> Ffclr {
        <crate::RegValueT<Ffclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mfclr_SPEC;
impl crate::sealed::RegSpec for Mfclr_SPEC {
    type DataType = u32;
}

#[doc = "Manchester Flag Clear Register"]
pub type Mfclr = crate::RegValueT<Mfclr_SPEC>;

impl Mfclr {
    #[doc = "PFER Clear Bit"]
    #[inline(always)]
    pub fn pferc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "SYER Clear Bit"]
    #[inline(always)]
    pub fn syerc(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "SBER Clear Bit"]
    #[inline(always)]
    pub fn sberc(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "MER Clear Bit"]
    #[inline(always)]
    pub fn merc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mfclr {
    #[inline(always)]
    fn default() -> Mfclr {
        <crate::RegValueT<Mfclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xfclr_SPEC;
impl crate::sealed::RegSpec for Xfclr_SPEC {
    type DataType = u32;
}

#[doc = "Simple LIN Flag Clear Register"]
pub type Xfclr = crate::RegValueT<Xfclr_SPEC>;

impl Xfclr {
    #[doc = "BFOF Clear Bit"]
    #[inline(always)]
    pub fn bfoc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "BCDF Clear Bit"]
    #[inline(always)]
    pub fn bcdc(self) -> crate::common::RegisterFieldBool<9, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "BFDF Clear Bit"]
    #[inline(always)]
    pub fn bfdc(self) -> crate::common::RegisterFieldBool<10, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "CF0MF Clear Bit"]
    #[inline(always)]
    pub fn cf0mc(self) -> crate::common::RegisterFieldBool<11, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "CF1MF Clear Bit"]
    #[inline(always)]
    pub fn cf1mc(self) -> crate::common::RegisterFieldBool<12, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "PIBDF Clear Bit"]
    #[inline(always)]
    pub fn pibdc(self) -> crate::common::RegisterFieldBool<13, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "COFF Clear Bit"]
    #[inline(always)]
    pub fn cofc(self) -> crate::common::RegisterFieldBool<14, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "AEDF Clear Bit"]
    #[inline(always)]
    pub fn aedc(self) -> crate::common::RegisterFieldBool<15, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Xfclr {
    #[inline(always)]
    fn default() -> Xfclr {
        <crate::RegValueT<Xfclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
