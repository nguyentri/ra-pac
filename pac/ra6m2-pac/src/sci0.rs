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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:11:44 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Serial Communication Interface 0"]
unsafe impl ::core::marker::Send for super::Sci0 {}
unsafe impl ::core::marker::Sync for super::Sci0 {}
impl super::Sci0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Serial Mode Register (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn smr(&self) -> &'static crate::common::Reg<self::Smr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Serial mode register (SCMR.SMIF = 1)"]
    #[inline(always)]
    pub const fn smr_smci(
        &self,
    ) -> &'static crate::common::Reg<self::SmrSmci_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmrSmci_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Bit Rate Register"]
    #[inline(always)]
    pub const fn brr(&self) -> &'static crate::common::Reg<self::Brr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Brr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "Serial Control Register (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn scr(&self) -> &'static crate::common::Reg<self::Scr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Serial Control Register (SCMR.SMIF =1)"]
    #[inline(always)]
    pub const fn scr_smci(
        &self,
    ) -> &'static crate::common::Reg<self::ScrSmci_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ScrSmci_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Transmit Data Register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &'static crate::common::Reg<self::Tdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)"]
    #[inline(always)]
    pub const fn ssr(&self) -> &'static crate::common::Reg<self::Ssr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=1)"]
    #[inline(always)]
    pub const fn ssr_fifo(
        &self,
    ) -> &'static crate::common::Reg<self::SsrFifo_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SsrFifo_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Serial Status Register(SCMR.SMIF = 1)"]
    #[inline(always)]
    pub const fn ssr_smci(
        &self,
    ) -> &'static crate::common::Reg<self::SsrSmci_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SsrSmci_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Receive Data Register"]
    #[inline(always)]
    pub const fn rdr(&self) -> &'static crate::common::Reg<self::Rdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[doc = "Smart Card Mode Register"]
    #[inline(always)]
    pub const fn scmr(&self) -> &'static crate::common::Reg<self::Scmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Serial Extended Mode Register"]
    #[inline(always)]
    pub const fn semr(&self) -> &'static crate::common::Reg<self::Semr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Semr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[doc = "Noise Filter Setting Register"]
    #[inline(always)]
    pub const fn snfr(&self) -> &'static crate::common::Reg<self::Snfr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snfr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "I2C Mode Register 1"]
    #[inline(always)]
    pub const fn simr1(&self) -> &'static crate::common::Reg<self::Simr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Simr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }

    #[doc = "I2C Mode Register 2"]
    #[inline(always)]
    pub const fn simr2(&self) -> &'static crate::common::Reg<self::Simr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Simr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "I2C Mode Register 3"]
    #[inline(always)]
    pub const fn simr3(&self) -> &'static crate::common::Reg<self::Simr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Simr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[doc = "I2C Status Register"]
    #[inline(always)]
    pub const fn sisr(&self) -> &'static crate::common::Reg<self::Sisr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sisr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "SPI Mode Register"]
    #[inline(always)]
    pub const fn spmr(&self) -> &'static crate::common::Reg<self::Spmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[doc = "Transmit 9-bit Data Register"]
    #[inline(always)]
    pub const fn tdrhl(&self) -> &'static crate::common::Reg<self::Tdrhl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdrhl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Transmit FIFO Data Register HL"]
    #[inline(always)]
    pub const fn ftdrhl(&self) -> &'static crate::common::Reg<self::Ftdrhl_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ftdrhl_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Transmit FIFO Data Register H"]
    #[inline(always)]
    pub const fn ftdrh(&self) -> &'static crate::common::Reg<self::Ftdrh_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ftdrh_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Transmit FIFO Data Register L"]
    #[inline(always)]
    pub const fn ftdrl(&self) -> &'static crate::common::Reg<self::Ftdrl_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ftdrl_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(15usize),
            )
        }
    }

    #[doc = "Receive 9-bit Data Register"]
    #[inline(always)]
    pub const fn rdrhl(&self) -> &'static crate::common::Reg<self::Rdrhl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdrhl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Receive FIFO Data Register HL"]
    #[inline(always)]
    pub const fn frdrhl(&self) -> &'static crate::common::Reg<self::Frdrhl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frdrhl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Receive FIFO Data Register H"]
    #[inline(always)]
    pub const fn frdrh(&self) -> &'static crate::common::Reg<self::Frdrh_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frdrh_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Receive FIFO Data Register L"]
    #[inline(always)]
    pub const fn frdrl(&self) -> &'static crate::common::Reg<self::Frdrl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frdrl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(17usize),
            )
        }
    }

    #[doc = "Modulation Duty Register"]
    #[inline(always)]
    pub const fn mddr(&self) -> &'static crate::common::Reg<self::Mddr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mddr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "Data Compare Match Control Register"]
    #[inline(always)]
    pub const fn dccr(&self) -> &'static crate::common::Reg<self::Dccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(19usize),
            )
        }
    }

    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &'static crate::common::Reg<self::Fcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "FIFO Data Count Register"]
    #[inline(always)]
    pub const fn fdr(&self) -> &'static crate::common::Reg<self::Fdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "Line Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &'static crate::common::Reg<self::Lsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Lsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Compare Match Data Register"]
    #[inline(always)]
    pub const fn cdr(&self) -> &'static crate::common::Reg<self::Cdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[doc = "Serial Port Register"]
    #[inline(always)]
    pub const fn sptr(&self) -> &'static crate::common::Reg<self::Sptr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sptr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smr_SPEC;
impl crate::sealed::RegSpec for Smr_SPEC {
    type DataType = u8;
}

#[doc = "Serial Mode Register (SCMR.SMIF = 0)"]
pub type Smr = crate::RegValueT<Smr_SPEC>;

impl Smr {
    #[doc = "Communications Mode"]
    #[inline(always)]
    pub fn cm(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, smr::Cm, smr::Cm, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,smr::Cm,smr::Cm,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Character Length(Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn chr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, smr::Chr, smr::Chr, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,smr::Chr,smr::Chr,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Parity Enable(Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn pe(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, smr::Pe, smr::Pe, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,smr::Pe,smr::Pe,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Parity Mode (Valid only when the PE bit is 1)"]
    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, smr::Pm, smr::Pm, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,smr::Pm,smr::Pm,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Stop Bit Length(Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn stop(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, smr::Stop, smr::Stop, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smr::Stop,
            smr::Stop,
            Smr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multi-Processor Mode(Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn mp(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, smr::Mp, smr::Mp, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,smr::Mp,smr::Mp,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock Select"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, smr::Cks, smr::Cks, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,smr::Cks,smr::Cks,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Smr {
    #[inline(always)]
    fn default() -> Smr {
        <crate::RegValueT<Smr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cm_SPEC;
    pub type Cm = crate::EnumBitfieldStruct<u8, Cm_SPEC>;
    impl Cm {
        #[doc = "Asynchronous mode or simple I2C mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clock synchronous mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chr_SPEC;
    pub type Chr = crate::EnumBitfieldStruct<u8, Chr_SPEC>;
    impl Chr {
        #[doc = "Transmit/receive in 9-bit data length(SCMR.CHR1=0) /  in 8bit data length(SCMR.CHR1=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit/receive in 9-bit data length(SCMR.CHR1=0) /  in 7bit data length(SCMR.CHR1=1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe_SPEC;
    pub type Pe = crate::EnumBitfieldStruct<u8, Pe_SPEC>;
    impl Pe {
        #[doc = "Parity bit addition is not performed (transmitting) / Parity bit checking is not performed ( receiving )"]
        pub const _0: Self = Self::new(0);

        #[doc = "The parity bit is added (transmitting) / The parity bit is checked (receiving)"]
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
    pub struct Stop_SPEC;
    pub type Stop = crate::EnumBitfieldStruct<u8, Stop_SPEC>;
    impl Stop {
        #[doc = "1 stop bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "2 stop bits"]
        pub const _1: Self = Self::new(1);
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
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "PCLK clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "PCLK/4 clock"]
        pub const _01: Self = Self::new(1);

        #[doc = "PCLK/16 clock"]
        pub const _10: Self = Self::new(2);

        #[doc = "PCLK/64 clock"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmrSmci_SPEC;
impl crate::sealed::RegSpec for SmrSmci_SPEC {
    type DataType = u8;
}

#[doc = "Serial mode register (SCMR.SMIF = 1)"]
pub type SmrSmci = crate::RegValueT<SmrSmci_SPEC>;

impl SmrSmci {
    #[doc = "GSM Mode"]
    #[inline(always)]
    pub fn gm(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        smr_smci::Gm,
        smr_smci::Gm,
        SmrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            smr_smci::Gm,
            smr_smci::Gm,
            SmrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Block Transfer Mode"]
    #[inline(always)]
    pub fn blk(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        smr_smci::Blk,
        smr_smci::Blk,
        SmrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            smr_smci::Blk,
            smr_smci::Blk,
            SmrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parity Enable(Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn pe(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        smr_smci::Pe,
        smr_smci::Pe,
        SmrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            smr_smci::Pe,
            smr_smci::Pe,
            SmrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parity Mode (Valid only when the PE bit is 1)"]
    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        smr_smci::Pm,
        smr_smci::Pm,
        SmrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            smr_smci::Pm,
            smr_smci::Pm,
            SmrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Stop Bit Length(Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn bcp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        smr_smci::Bcp,
        smr_smci::Bcp,
        SmrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            smr_smci::Bcp,
            smr_smci::Bcp,
            SmrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Select"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        smr_smci::Cks,
        smr_smci::Cks,
        SmrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            smr_smci::Cks,
            smr_smci::Cks,
            SmrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SmrSmci {
    #[inline(always)]
    fn default() -> SmrSmci {
        <crate::RegValueT<SmrSmci_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smr_smci {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gm_SPEC;
    pub type Gm = crate::EnumBitfieldStruct<u8, Gm_SPEC>;
    impl Gm {
        #[doc = "Normal mode operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "GSM mode operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blk_SPEC;
    pub type Blk = crate::EnumBitfieldStruct<u8, Blk_SPEC>;
    impl Blk {
        #[doc = "Normal mode operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Block transfer mode operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe_SPEC;
    pub type Pe = crate::EnumBitfieldStruct<u8, Pe_SPEC>;
    impl Pe {
        #[doc = "Setting Prohibited"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set this bit to 1 in smart card interface mode."]
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
    pub struct Bcp_SPEC;
    pub type Bcp = crate::EnumBitfieldStruct<u8, Bcp_SPEC>;
    impl Bcp {
        #[doc = "93 clock cycles(S=93) (SCMR.BCP2=0) / 32 clock cycles(S=32)  (SCMR.BCP2=1)"]
        pub const _00: Self = Self::new(0);

        #[doc = "128 clock cycles(S=128) (SCMR.BCP2=0) / 64 clock cycles(S=64) (SCMR.BCP2=1)"]
        pub const _01: Self = Self::new(1);

        #[doc = "186 clock cycles(S=186) (SCMR.BCP2=0) / 372 clock cycles(S=372) (SCMR.BCP2=1)"]
        pub const _10: Self = Self::new(2);

        #[doc = "512 clock cycles(S=512) (SCMR.BCP2=0) / 256 clock cycles(S=256) (SCMR.BCP2=1)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "PCLK clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "PCLK/4 clock"]
        pub const _01: Self = Self::new(1);

        #[doc = "PCLK/16 clock"]
        pub const _10: Self = Self::new(2);

        #[doc = "PCLK/64 clock"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brr_SPEC;
impl crate::sealed::RegSpec for Brr_SPEC {
    type DataType = u8;
}

#[doc = "Bit Rate Register"]
pub type Brr = crate::RegValueT<Brr_SPEC>;

impl Brr {
    #[doc = "BRR is an 8-bit register that adjusts the bit rate."]
    #[inline(always)]
    pub fn brr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Brr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Brr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Brr {
    #[inline(always)]
    fn default() -> Brr {
        <crate::RegValueT<Brr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr_SPEC;
impl crate::sealed::RegSpec for Scr_SPEC {
    type DataType = u8;
}

#[doc = "Serial Control Register (SCMR.SMIF = 0)"]
pub type Scr = crate::RegValueT<Scr_SPEC>;

impl Scr {
    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, scr::Tie, scr::Tie, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,scr::Tie,scr::Tie,Scr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, scr::Rie, scr::Rie, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,scr::Rie,scr::Rie,Scr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit Enable"]
    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, scr::Te, scr::Te, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,scr::Te,scr::Te,Scr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive Enable"]
    #[inline(always)]
    pub fn re(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, scr::Re, scr::Re, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,scr::Re,scr::Re,Scr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Multi-Processor Interrupt Enable(Valid in asynchronous mode when SMR.MP = 1)"]
    #[inline(always)]
    pub fn mpie(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, scr::Mpie, scr::Mpie, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            scr::Mpie,
            scr::Mpie,
            Scr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, scr::Teie, scr::Teie, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            scr::Teie,
            scr::Teie,
            Scr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Enable"]
    #[inline(always)]
    pub fn cke(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, scr::Cke, scr::Cke, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,scr::Cke,scr::Cke,Scr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        <crate::RegValueT<Scr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        #[doc = "TXI interrupt request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "TXI interrupt request is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "RXI and ERI interrupt requests are disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "RXI and ERI interrupt requests are enabled"]
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
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        #[doc = "Serial reception is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Serial reception is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpie_SPEC;
    pub type Mpie = crate::EnumBitfieldStruct<u8, Mpie_SPEC>;
    impl Mpie {
        #[doc = "Normal reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "When the data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF,ORER and FER in SSR to 1 is disabled. When the data with the multiprocessor bit set to 1 is received, the MPIE bit is automatically cleared to 0, and normal reception is resumed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teie_SPEC;
    pub type Teie = crate::EnumBitfieldStruct<u8, Teie_SPEC>;
    impl Teie {
        #[doc = "TEI interrupt request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "TEI interrupt request is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cke_SPEC;
    pub type Cke = crate::EnumBitfieldStruct<u8, Cke_SPEC>;
    impl Cke {
        #[doc = "The SCKn pin is available for use as an I/O port in accord with the I/O port settings.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
        pub const _00: Self = Self::new(0);

        #[doc = "The clock with the same frequency as the bit rate is output from the SCKn pin.(Asynchronous mode) / The SCKn pin functions as the clock output pin(Clock synchronous mode)"]
        pub const _01: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScrSmci_SPEC;
impl crate::sealed::RegSpec for ScrSmci_SPEC {
    type DataType = u8;
}

#[doc = "Serial Control Register (SCMR.SMIF =1)"]
pub type ScrSmci = crate::RegValueT<ScrSmci_SPEC>;

impl ScrSmci {
    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        scr_smci::Tie,
        scr_smci::Tie,
        ScrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            scr_smci::Tie,
            scr_smci::Tie,
            ScrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        scr_smci::Rie,
        scr_smci::Rie,
        ScrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            scr_smci::Rie,
            scr_smci::Rie,
            ScrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Enable"]
    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        scr_smci::Te,
        scr_smci::Te,
        ScrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            scr_smci::Te,
            scr_smci::Te,
            ScrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Enable"]
    #[inline(always)]
    pub fn re(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        scr_smci::Re,
        scr_smci::Re,
        ScrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            scr_smci::Re,
            scr_smci::Re,
            ScrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multi-Processor Interrupt EnableSet this bit to 0 in smart card interface mode."]
    #[inline(always)]
    pub fn mpie(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ScrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, ScrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Transmit End Interrupt EnableSet this bit to 0 in smart card interface mode."]
    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ScrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, ScrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Clock Enable"]
    #[inline(always)]
    pub fn cke(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        scr_smci::Cke,
        scr_smci::Cke,
        ScrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            scr_smci::Cke,
            scr_smci::Cke,
            ScrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ScrSmci {
    #[inline(always)]
    fn default() -> ScrSmci {
        <crate::RegValueT<ScrSmci_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scr_smci {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        #[doc = "A TXI interrupt request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "A TXI interrupt request is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "RXI and ERI interrupt requests are disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "RXI and ERI interrupt requests are enabled"]
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
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        #[doc = "Serial reception is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Serial reception is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cke_SPEC;
    pub type Cke = crate::EnumBitfieldStruct<u8, Cke_SPEC>;
    impl Cke {
        #[doc = "Output disabled(SMR_SMCI.GM=0) / Output fixed low(SMR_SMCI.GM=1)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Clock Output"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited(SMR_SMCI.GM=0) / Output fixed High(SMR_SMCI.GM=1)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited(SMR_SMCI.GM=0) / Clock Output(SMR_SMCI.GM=1)"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr_SPEC;
impl crate::sealed::RegSpec for Tdr_SPEC {
    type DataType = u8;
}

#[doc = "Transmit Data Register"]
pub type Tdr = crate::RegValueT<Tdr_SPEC>;

impl Tdr {
    #[doc = "TDR is an 8-bit register that stores transmit data."]
    #[inline(always)]
    pub fn tdr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Tdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Tdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tdr {
    #[inline(always)]
    fn default() -> Tdr {
        <crate::RegValueT<Tdr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssr_SPEC;
impl crate::sealed::RegSpec for Ssr_SPEC {
    type DataType = u8;
}

#[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)"]
pub type Ssr = crate::RegValueT<Ssr_SPEC>;

impl Ssr {
    #[doc = "Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ssr::Tdre, ssr::Tdre, Ssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ssr::Tdre,
            ssr::Tdre,
            Ssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ssr::Rdrf, ssr::Rdrf, Ssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ssr::Rdrf,
            ssr::Rdrf,
            Ssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ssr::Orer, ssr::Orer, Ssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ssr::Orer,
            ssr::Orer,
            Ssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Framing Error Flag"]
    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ssr::Fer, ssr::Fer, Ssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ssr::Fer,ssr::Fer,Ssr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ssr::Per, ssr::Per, Ssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,ssr::Per,ssr::Per,Ssr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ssr::Tend, ssr::Tend, Ssr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,ssr::Tend,ssr::Tend,Ssr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Multi-Processor Bit. Value of the multi-processor bit in the reception frame"]
    #[inline(always)]
    pub fn mpb(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ssr::Mpb, ssr::Mpb, Ssr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,ssr::Mpb,ssr::Mpb,Ssr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Multi-Processor Bit Transfer. Sets the multi-processor bit for adding to the transmission frame"]
    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ssr::Mpbt, ssr::Mpbt, Ssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssr::Mpbt,
            ssr::Mpbt,
            Ssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssr {
    #[inline(always)]
    fn default() -> Ssr {
        <crate::RegValueT<Ssr_SPEC> as RegisterValue<_>>::new(132)
    }
}
pub mod ssr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        #[doc = "Transmit data is in TDR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "No transmit data is in TDR register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrf_SPEC;
    pub type Rdrf = crate::EnumBitfieldStruct<u8, Rdrf_SPEC>;
    impl Rdrf {
        #[doc = "No received data is in RDR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received data is in RDR register"]
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
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        #[doc = "No framing error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A framing error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "No parity error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A parity error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        #[doc = "A character is being transmitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Character transfer has been completed."]
        pub const _1: Self = Self::new(1);
    }
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
    pub struct Mpbt_SPEC;
    pub type Mpbt = crate::EnumBitfieldStruct<u8, Mpbt_SPEC>;
    impl Mpbt {
        #[doc = "Data transmission cycles"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycles"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsrFifo_SPEC;
impl crate::sealed::RegSpec for SsrFifo_SPEC {
    type DataType = u8;
}

#[doc = "Serial Status Register(SCMR.SMIF = 0 and FCR.FM=1)"]
pub type SsrFifo = crate::RegValueT<SsrFifo_SPEC>;

impl SsrFifo {
    #[doc = "Transmit FIFO data empty flag"]
    #[inline(always)]
    pub fn tdfe(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ssr_fifo::Tdfe,
        ssr_fifo::Tdfe,
        SsrFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ssr_fifo::Tdfe,
            ssr_fifo::Tdfe,
            SsrFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive FIFO data full flag"]
    #[inline(always)]
    pub fn rdf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ssr_fifo::Rdf,
        ssr_fifo::Rdf,
        SsrFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ssr_fifo::Rdf,
            ssr_fifo::Rdf,
            SsrFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ssr_fifo::Orer,
        ssr_fifo::Orer,
        SsrFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ssr_fifo::Orer,
            ssr_fifo::Orer,
            SsrFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Framing Error Flag"]
    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ssr_fifo::Fer,
        ssr_fifo::Fer,
        SsrFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ssr_fifo::Fer,
            ssr_fifo::Fer,
            SsrFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ssr_fifo::Per,
        ssr_fifo::Per,
        SsrFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ssr_fifo::Per,
            ssr_fifo::Per,
            SsrFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ssr_fifo::Tend,
        ssr_fifo::Tend,
        SsrFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ssr_fifo::Tend,
            ssr_fifo::Tend,
            SsrFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Data Ready flag(Valid only in asynchronous mode(including multi-processor) and FIFO selected)"]
    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssr_fifo::Dr,
        ssr_fifo::Dr,
        SsrFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssr_fifo::Dr,
            ssr_fifo::Dr,
            SsrFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SsrFifo {
    #[inline(always)]
    fn default() -> SsrFifo {
        <crate::RegValueT<SsrFifo_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod ssr_fifo {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdfe_SPEC;
    pub type Tdfe = crate::EnumBitfieldStruct<u8, Tdfe_SPEC>;
    impl Tdfe {
        #[doc = "The quantity of transmit data written in FTDR exceeds the specified transmit triggering number."]
        pub const _0: Self = Self::new(0);

        #[doc = "The quantity of transmit data written in FTDR is equal to or less than the specified transmit triggering number"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdf_SPEC;
    pub type Rdf = crate::EnumBitfieldStruct<u8, Rdf_SPEC>;
    impl Rdf {
        #[doc = "The quantity of receive data written in FRDR falls below the specified receive triggering number."]
        pub const _0: Self = Self::new(0);

        #[doc = "The quantity of receive data written in FRDR is equal to or greater than the specified receive triggering number."]
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
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        #[doc = "No framing error occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "A framing error has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "No parity error occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "A parity error has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        #[doc = "A character is being transmitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Character transfer has been completed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
        #[doc = "Receiving is in progress, or no received data has remained in FRDR after normally completed receiving.(receive FIFO is empty)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Next receive data has not been received for a period after normal completed receiving, , when data is stored in FIFO to equal or less than receive triggering number."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsrSmci_SPEC;
impl crate::sealed::RegSpec for SsrSmci_SPEC {
    type DataType = u8;
}

#[doc = "Serial Status Register(SCMR.SMIF = 1)"]
pub type SsrSmci = crate::RegValueT<SsrSmci_SPEC>;

impl SsrSmci {
    #[doc = "Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ssr_smci::Tdre,
        ssr_smci::Tdre,
        SsrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ssr_smci::Tdre,
            ssr_smci::Tdre,
            SsrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ssr_smci::Rdrf,
        ssr_smci::Rdrf,
        SsrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ssr_smci::Rdrf,
            ssr_smci::Rdrf,
            SsrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ssr_smci::Orer,
        ssr_smci::Orer,
        SsrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ssr_smci::Orer,
            ssr_smci::Orer,
            SsrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Error Signal Status Flag"]
    #[inline(always)]
    pub fn ers(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ssr_smci::Ers,
        ssr_smci::Ers,
        SsrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ssr_smci::Ers,
            ssr_smci::Ers,
            SsrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ssr_smci::Per,
        ssr_smci::Per,
        SsrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ssr_smci::Per,
            ssr_smci::Per,
            SsrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ssr_smci::Tend,
        ssr_smci::Tend,
        SsrSmci_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ssr_smci::Tend,
            ssr_smci::Tend,
            SsrSmci_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "This bit should be 0 in smart card interface mode."]
    #[inline(always)]
    pub fn mpb(self) -> crate::common::RegisterFieldBool<1, 1, 0, SsrSmci_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, SsrSmci_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "This bit should be 0 in smart card interface mode."]
    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SsrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, SsrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for SsrSmci {
    #[inline(always)]
    fn default() -> SsrSmci {
        <crate::RegValueT<SsrSmci_SPEC> as RegisterValue<_>>::new(132)
    }
}
pub mod ssr_smci {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        #[doc = "Transmit data is in TDR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "No transmit data is in TDR register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrf_SPEC;
    pub type Rdrf = crate::EnumBitfieldStruct<u8, Rdrf_SPEC>;
    impl Rdrf {
        #[doc = "No received data is in RDR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received data is in RDR register"]
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
    pub struct Ers_SPEC;
    pub type Ers = crate::EnumBitfieldStruct<u8, Ers_SPEC>;
    impl Ers {
        #[doc = "Low error signal not responded"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low error signal responded"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "No parity error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A parity error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        #[doc = "A character is being transmitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Character transfer has been completed."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdr_SPEC;
impl crate::sealed::RegSpec for Rdr_SPEC {
    type DataType = u8;
}

#[doc = "Receive Data Register"]
pub type Rdr = crate::RegValueT<Rdr_SPEC>;

impl Rdr {
    #[doc = "RDR is an 8-bit register that stores receive data."]
    #[inline(always)]
    pub fn rdr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Rdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rdr {
    #[inline(always)]
    fn default() -> Rdr {
        <crate::RegValueT<Rdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scmr_SPEC;
impl crate::sealed::RegSpec for Scmr_SPEC {
    type DataType = u8;
}

#[doc = "Smart Card Mode Register"]
pub type Scmr = crate::RegValueT<Scmr_SPEC>;

impl Scmr {
    #[doc = "Base Clock Pulse 2Selects the number of base clock cycles in combination with the SMR.BCP\\[1:0\\] bits"]
    #[inline(always)]
    pub fn bcp2(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        scmr::Bcp2,
        scmr::Bcp2,
        Scmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            scmr::Bcp2,
            scmr::Bcp2,
            Scmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Character Length 1(Only valid in asynchronous mode)"]
    #[inline(always)]
    pub fn chr1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        scmr::Chr1,
        scmr::Chr1,
        Scmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            scmr::Chr1,
            scmr::Chr1,
            Scmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmitted/Received Data Transfer DirectionNOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode.Set this bit to 1 if operation is to be in simple I2C mode."]
    #[inline(always)]
    pub fn sdir(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        scmr::Sdir,
        scmr::Sdir,
        Scmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            scmr::Sdir,
            scmr::Sdir,
            Scmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmitted/Received Data InvertSet this bit to 0 if operation is to be in simple I2C mode."]
    #[inline(always)]
    pub fn sinv(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        scmr::Sinv,
        scmr::Sinv,
        Scmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            scmr::Sinv,
            scmr::Sinv,
            Scmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Smart Card Interface Mode Select"]
    #[inline(always)]
    pub fn smif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        scmr::Smif,
        scmr::Smif,
        Scmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            scmr::Smif,
            scmr::Smif,
            Scmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scmr {
    #[inline(always)]
    fn default() -> Scmr {
        <crate::RegValueT<Scmr_SPEC> as RegisterValue<_>>::new(242)
    }
}
pub mod scmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcp2_SPEC;
    pub type Bcp2 = crate::EnumBitfieldStruct<u8, Bcp2_SPEC>;
    impl Bcp2 {
        #[doc = "S=93(SMR.BCP\\[1:0\\]=00), 128(SMR.BCP\\[1:0\\]=01), 186(SMR.BCP\\[1:0\\]=10), 512(SMR.BCP\\[1:0\\]=11)"]
        pub const _0: Self = Self::new(0);

        #[doc = "S=32(SMR.BCP\\[1:0\\]=00), 64(SMR.BCP\\[1:0\\]=01), 372(SMR.BCP\\[1:0\\]=10), 256(SMR.BCP\\[1:0\\]=11)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chr1_SPEC;
    pub type Chr1 = crate::EnumBitfieldStruct<u8, Chr1_SPEC>;
    impl Chr1 {
        #[doc = "Transmit/receive in 9-bit data length"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit/receive in 8-bit data length(SMR.CHR=0) /  in 7bit data length(SMR.CHR=1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdir_SPEC;
    pub type Sdir = crate::EnumBitfieldStruct<u8, Sdir_SPEC>;
    impl Sdir {
        #[doc = "Transfer with LSB first"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transfer with MSB first"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sinv_SPEC;
    pub type Sinv = crate::EnumBitfieldStruct<u8, Sinv_SPEC>;
    impl Sinv {
        #[doc = "TDR contents are transmitted as they are. Receive data is stored as it is in RDR."]
        pub const _0: Self = Self::new(0);

        #[doc = "TDR contents are inverted before being transmitted. Receive data is stored in inverted form in RDR."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smif_SPEC;
    pub type Smif = crate::EnumBitfieldStruct<u8, Smif_SPEC>;
    impl Smif {
        #[doc = "Non-smart card interface mode(Asynchronous mode, clock synchronous mode, simple SPI mode, or simple I2C mode)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Smart card interface mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Semr_SPEC;
impl crate::sealed::RegSpec for Semr_SPEC {
    type DataType = u8;
}

#[doc = "Serial Extended Mode Register"]
pub type Semr = crate::RegValueT<Semr_SPEC>;

impl Semr {
    #[doc = "Asynchronous Start Bit Edge Detection Select(Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn rxdesel(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        semr::Rxdesel,
        semr::Rxdesel,
        Semr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            semr::Rxdesel,
            semr::Rxdesel,
            Semr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Baud Rate Generator Double-Speed Mode Select(Only valid the CKE\\[1\\] bit in SCR is 0 in asynchronous mode)."]
    #[inline(always)]
    pub fn bgdm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        semr::Bgdm,
        semr::Bgdm,
        Semr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            semr::Bgdm,
            semr::Bgdm,
            Semr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Digital Noise Filter Function Enable(The NFEN bit should be 0 without simple I2C mode and asynchronous mode.)In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input."]
    #[inline(always)]
    pub fn nfen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        semr::Nfen,
        semr::Nfen,
        Semr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            semr::Nfen,
            semr::Nfen,
            Semr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Base Clock Select(Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn abcs(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        semr::Abcs,
        semr::Abcs,
        Semr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            semr::Abcs,
            semr::Abcs,
            Semr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Extended Base Clock Select1(Valid only in asynchronous mode and SCR.CKE\\[1\\]=0)"]
    #[inline(always)]
    pub fn abcse(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        semr::Abcse,
        semr::Abcse,
        Semr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            semr::Abcse,
            semr::Abcse,
            Semr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bit Modulation Enable"]
    #[inline(always)]
    pub fn brme(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        semr::Brme,
        semr::Brme,
        Semr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            semr::Brme,
            semr::Brme,
            Semr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Semr {
    #[inline(always)]
    fn default() -> Semr {
        <crate::RegValueT<Semr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod semr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdesel_SPEC;
    pub type Rxdesel = crate::EnumBitfieldStruct<u8, Rxdesel_SPEC>;
    impl Rxdesel {
        #[doc = "The low level on the RXDn pin is detected as the start bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "A falling edge on the RXDn pin is detected as the start bit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bgdm_SPEC;
    pub type Bgdm = crate::EnumBitfieldStruct<u8, Bgdm_SPEC>;
    impl Bgdm {
        #[doc = "Baud rate generator outputs the clock with normal frequency."]
        pub const _0: Self = Self::new(0);

        #[doc = "Baud rate generator outputs the clock with doubled frequency."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Noise cancellation function for the RXDn/TXDn input signal is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Noise cancellation function for the RXDn/TXDn input signal is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcs_SPEC;
    pub type Abcs = crate::EnumBitfieldStruct<u8, Abcs_SPEC>;
    impl Abcs {
        #[doc = "Selects 16 base clock cycles for 1-bit period."]
        pub const _0: Self = Self::new(0);

        #[doc = "Selects 8 base clock cycles for 1-bit period."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcse_SPEC;
    pub type Abcse = crate::EnumBitfieldStruct<u8, Abcse_SPEC>;
    impl Abcse {
        #[doc = "Clock cycles for 1-bit period is decided with combination between BGDM and ABCS in SEMR."]
        pub const _0: Self = Self::new(0);

        #[doc = "Baud rate is 6 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brme_SPEC;
    pub type Brme = crate::EnumBitfieldStruct<u8, Brme_SPEC>;
    impl Brme {
        #[doc = "Bit rate modulation function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit rate modulation function is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snfr_SPEC;
impl crate::sealed::RegSpec for Snfr_SPEC {
    type DataType = u8;
}

#[doc = "Noise Filter Setting Register"]
pub type Snfr = crate::RegValueT<Snfr_SPEC>;

impl Snfr {
    #[doc = "Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        snfr::Nfcs,
        snfr::Nfcs,
        Snfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            snfr::Nfcs,
            snfr::Nfcs,
            Snfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Snfr {
    #[inline(always)]
    fn default() -> Snfr {
        <crate::RegValueT<Snfr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snfr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "The clock signal divided by 1 is used with the noise filter.(In asynchronous mode)"]
        pub const _000: Self = Self::new(0);

        #[doc = "The clock signal divided by 1 is used with the noise filter.(In simple I2C mode)"]
        pub const _001: Self = Self::new(1);

        #[doc = "The clock signal divided by 2 is used with the noise filter.(In simple I2C mode)"]
        pub const _010: Self = Self::new(2);

        #[doc = "The clock signal divided by 4 is used with the noise filter.(In simple I2C mode)"]
        pub const _011: Self = Self::new(3);

        #[doc = "The clock signal divided by 8 is used with the noise filter.(In simple I2C mode)"]
        pub const _100: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr1_SPEC;
impl crate::sealed::RegSpec for Simr1_SPEC {
    type DataType = u8;
}

#[doc = "I2C Mode Register 1"]
pub type Simr1 = crate::RegValueT<Simr1_SPEC>;

impl Simr1 {
    #[doc = "SSDA Delay Output SelectCycles below are of the clock signal from the on-chip baud rate generator."]
    #[inline(always)]
    pub fn iicdl(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1f,
        1,
        0,
        simr1::Iicdl,
        simr1::Iicdl,
        Simr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1f,
            1,
            0,
            simr1::Iicdl,
            simr1::Iicdl,
            Simr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Simple I2C Mode Select"]
    #[inline(always)]
    pub fn iicm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        simr1::Iicm,
        simr1::Iicm,
        Simr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            simr1::Iicm,
            simr1::Iicm,
            Simr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Simr1 {
    #[inline(always)]
    fn default() -> Simr1 {
        <crate::RegValueT<Simr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod simr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicdl_SPEC;
    pub type Iicdl = crate::EnumBitfieldStruct<u8, Iicdl_SPEC>;
    impl Iicdl {
        #[doc = "No output delay"]
        pub const _00000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicm_SPEC;
    pub type Iicm = crate::EnumBitfieldStruct<u8, Iicm_SPEC>;
    impl Iicm {
        #[doc = "Asynchronous mode, Multi-processor mode, Clock synchronous mode(SCMR.SMIF=0) /Smart card interface mode(SCMR.SMIF=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Simple I2C mode(SCMR.SMIF=0) / Setting prohibited.(SCMR.SMIF=1)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr2_SPEC;
impl crate::sealed::RegSpec for Simr2_SPEC {
    type DataType = u8;
}

#[doc = "I2C Mode Register 2"]
pub type Simr2 = crate::RegValueT<Simr2_SPEC>;

impl Simr2 {
    #[doc = "ACK Transmission Data"]
    #[inline(always)]
    pub fn iicackt(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        simr2::Iicackt,
        simr2::Iicackt,
        Simr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            simr2::Iicackt,
            simr2::Iicackt,
            Simr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Synchronization"]
    #[inline(always)]
    pub fn iiccsc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        simr2::Iiccsc,
        simr2::Iiccsc,
        Simr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            simr2::Iiccsc,
            simr2::Iiccsc,
            Simr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I2C Interrupt Mode Select"]
    #[inline(always)]
    pub fn iicintm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        simr2::Iicintm,
        simr2::Iicintm,
        Simr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            simr2::Iicintm,
            simr2::Iicintm,
            Simr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Simr2 {
    #[inline(always)]
    fn default() -> Simr2 {
        <crate::RegValueT<Simr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod simr2 {

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
    pub struct Iiccsc_SPEC;
    pub type Iiccsc = crate::EnumBitfieldStruct<u8, Iiccsc_SPEC>;
    impl Iiccsc {
        #[doc = "No synchronization with the clock signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronization with the clock signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicintm_SPEC;
    pub type Iicintm = crate::EnumBitfieldStruct<u8, Iicintm_SPEC>;
    impl Iicintm {
        #[doc = "Use ACK/NACK interrupts."]
        pub const _0: Self = Self::new(0);

        #[doc = "Use reception and transmission interrupts"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr3_SPEC;
impl crate::sealed::RegSpec for Simr3_SPEC {
    type DataType = u8;
}

#[doc = "I2C Mode Register 3"]
pub type Simr3 = crate::RegValueT<Simr3_SPEC>;

impl Simr3 {
    #[doc = "SCL Output Select"]
    #[inline(always)]
    pub fn iicscls(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        simr3::Iicscls,
        simr3::Iicscls,
        Simr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            simr3::Iicscls,
            simr3::Iicscls,
            Simr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDA Output Select"]
    #[inline(always)]
    pub fn iicsdas(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        simr3::Iicsdas,
        simr3::Iicsdas,
        Simr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            simr3::Iicsdas,
            simr3::Iicsdas,
            Simr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Issuing of Start, Restart, or Stop Condition Completed Flag(When 0 is written to IICSTIF, it is cleared to 0.)"]
    #[inline(always)]
    pub fn iicstif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        simr3::Iicstif,
        simr3::Iicstif,
        Simr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            simr3::Iicstif,
            simr3::Iicstif,
            Simr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Stop Condition Generation"]
    #[inline(always)]
    pub fn iicstpreq(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        simr3::Iicstpreq,
        simr3::Iicstpreq,
        Simr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            simr3::Iicstpreq,
            simr3::Iicstpreq,
            Simr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Restart Condition Generation"]
    #[inline(always)]
    pub fn iicrstareq(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        simr3::Iicrstareq,
        simr3::Iicrstareq,
        Simr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            simr3::Iicrstareq,
            simr3::Iicrstareq,
            Simr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start Condition Generation"]
    #[inline(always)]
    pub fn iicstareq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        simr3::Iicstareq,
        simr3::Iicstareq,
        Simr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            simr3::Iicstareq,
            simr3::Iicstareq,
            Simr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Simr3 {
    #[inline(always)]
    fn default() -> Simr3 {
        <crate::RegValueT<Simr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod simr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicscls_SPEC;
    pub type Iicscls = crate::EnumBitfieldStruct<u8, Iicscls_SPEC>;
    impl Iicscls {
        #[doc = "Serial clock output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Generate a start, restart, or stop condition."]
        pub const _01: Self = Self::new(1);

        #[doc = "Output the low level on the SSCLn pin."]
        pub const _10: Self = Self::new(2);

        #[doc = "Place the SSCLn pin in the high-impedance state."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicsdas_SPEC;
    pub type Iicsdas = crate::EnumBitfieldStruct<u8, Iicsdas_SPEC>;
    impl Iicsdas {
        #[doc = "Serial data output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Generate a start, restart, or stop condition."]
        pub const _01: Self = Self::new(1);

        #[doc = "Output the low level on the SSDAn pin."]
        pub const _10: Self = Self::new(2);

        #[doc = "Place the SSDAn pin in the high-impedance state."]
        pub const _11: Self = Self::new(3);
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstpreq_SPEC;
    pub type Iicstpreq = crate::EnumBitfieldStruct<u8, Iicstpreq_SPEC>;
    impl Iicstpreq {
        #[doc = "A stop condition is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A stop condition is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicrstareq_SPEC;
    pub type Iicrstareq = crate::EnumBitfieldStruct<u8, Iicrstareq_SPEC>;
    impl Iicrstareq {
        #[doc = "A restart condition is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A restart condition is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstareq_SPEC;
    pub type Iicstareq = crate::EnumBitfieldStruct<u8, Iicstareq_SPEC>;
    impl Iicstareq {
        #[doc = "A start condition is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A start condition is generated."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sisr_SPEC;
impl crate::sealed::RegSpec for Sisr_SPEC {
    type DataType = u8;
}

#[doc = "I2C Status Register"]
pub type Sisr = crate::RegValueT<Sisr_SPEC>;

impl Sisr {
    #[doc = "ACK Reception Data Flag"]
    #[inline(always)]
    pub fn iicackr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sisr::Iicackr,
        sisr::Iicackr,
        Sisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sisr::Iicackr,
            sisr::Iicackr,
            Sisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sisr {
    #[inline(always)]
    fn default() -> Sisr {
        <crate::RegValueT<Sisr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicackr_SPEC;
    pub type Iicackr = crate::EnumBitfieldStruct<u8, Iicackr_SPEC>;
    impl Iicackr {
        #[doc = "ACK received"]
        pub const _0: Self = Self::new(0);

        #[doc = "NACK received"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spmr_SPEC;
impl crate::sealed::RegSpec for Spmr_SPEC {
    type DataType = u8;
}

#[doc = "SPI Mode Register"]
pub type Spmr = crate::RegValueT<Spmr_SPEC>;

impl Spmr {
    #[doc = "Clock Phase Select"]
    #[inline(always)]
    pub fn ckph(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spmr::Ckph,
        spmr::Ckph,
        Spmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spmr::Ckph,
            spmr::Ckph,
            Spmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Polarity Select"]
    #[inline(always)]
    pub fn ckpol(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        spmr::Ckpol,
        spmr::Ckpol,
        Spmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            spmr::Ckpol,
            spmr::Ckpol,
            Spmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mode Fault Flag"]
    #[inline(always)]
    pub fn mff(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        spmr::Mff,
        spmr::Mff,
        Spmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            spmr::Mff,
            spmr::Mff,
            Spmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Master or slave mode selection"]
    #[inline(always)]
    pub fn mss(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        spmr::Mss,
        spmr::Mss,
        Spmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            spmr::Mss,
            spmr::Mss,
            Spmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTS Enable"]
    #[inline(always)]
    pub fn ctse(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spmr::Ctse,
        spmr::Ctse,
        Spmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spmr::Ctse,
            spmr::Ctse,
            Spmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SSn# Pin Function Enable"]
    #[inline(always)]
    pub fn sse(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spmr::Sse,
        spmr::Sse,
        Spmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spmr::Sse,
            spmr::Sse,
            Spmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Spmr {
    #[inline(always)]
    fn default() -> Spmr {
        <crate::RegValueT<Spmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckph_SPEC;
    pub type Ckph = crate::EnumBitfieldStruct<u8, Ckph_SPEC>;
    impl Ckph {
        #[doc = "Clock is not delayed."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clock is delayed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckpol_SPEC;
    pub type Ckpol = crate::EnumBitfieldStruct<u8, Ckpol_SPEC>;
    impl Ckpol {
        #[doc = "Clock polarity is not inverted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clock polarity is inverted"]
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
    pub struct Mss_SPEC;
    pub type Mss = crate::EnumBitfieldStruct<u8, Mss_SPEC>;
    impl Mss {
        #[doc = "Transmission is through the TXDn pin and reception is through the RXDn pin (master mode)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reception is through the TXDn pin and transmission is through the RXDn pin (slave mode)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctse_SPEC;
    pub type Ctse = crate::EnumBitfieldStruct<u8, Ctse_SPEC>;
    impl Ctse {
        #[doc = "CTS function is disabled (RTS output function is enabled)."]
        pub const _0: Self = Self::new(0);

        #[doc = "CTS function is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sse_SPEC;
    pub type Sse = crate::EnumBitfieldStruct<u8, Sse_SPEC>;
    impl Sse {
        #[doc = "SSn# pin function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "SSn# pin function is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdrhl_SPEC;
impl crate::sealed::RegSpec for Tdrhl_SPEC {
    type DataType = u16;
}

#[doc = "Transmit 9-bit Data Register"]
pub type Tdrhl = crate::RegValueT<Tdrhl_SPEC>;

impl Tdrhl {
    #[doc = "TDRHL is a 16-bit register that stores transmit data."]
    #[inline(always)]
    pub fn tdrhl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Tdrhl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Tdrhl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tdrhl {
    #[inline(always)]
    fn default() -> Tdrhl {
        <crate::RegValueT<Tdrhl_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftdrhl_SPEC;
impl crate::sealed::RegSpec for Ftdrhl_SPEC {
    type DataType = u16;
}

#[doc = "Transmit FIFO Data Register HL"]
pub type Ftdrhl = crate::RegValueT<Ftdrhl_SPEC>;

impl Ftdrhl {
    #[doc = "Multi-processor transfer bit flag(Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ftdrhl::Mpbt,
        ftdrhl::Mpbt,
        Ftdrhl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ftdrhl::Mpbt,
            ftdrhl::Mpbt,
            Ftdrhl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Serial transmit data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Ftdrhl_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Ftdrhl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Ftdrhl {
    #[inline(always)]
    fn default() -> Ftdrhl {
        <crate::RegValueT<Ftdrhl_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod ftdrhl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpbt_SPEC;
    pub type Mpbt = crate::EnumBitfieldStruct<u8, Mpbt_SPEC>;
    impl Mpbt {
        #[doc = "Data transmission cycles"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycles"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftdrh_SPEC;
impl crate::sealed::RegSpec for Ftdrh_SPEC {
    type DataType = u8;
}

#[doc = "Transmit FIFO Data Register H"]
pub type Ftdrh = crate::RegValueT<Ftdrh_SPEC>;

impl Ftdrh {
    #[doc = "Multi-processor transfer bit flag(Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ftdrh::Mpbt,
        ftdrh::Mpbt,
        Ftdrh_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ftdrh::Mpbt,
            ftdrh::Mpbt,
            Ftdrh_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Serial transmit data (b8) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub fn tdath(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ftdrh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ftdrh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ftdrh {
    #[inline(always)]
    fn default() -> Ftdrh {
        <crate::RegValueT<Ftdrh_SPEC> as RegisterValue<_>>::new(255)
    }
}
pub mod ftdrh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpbt_SPEC;
    pub type Mpbt = crate::EnumBitfieldStruct<u8, Mpbt_SPEC>;
    impl Mpbt {
        #[doc = "Data transmission cycles"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycles"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftdrl_SPEC;
impl crate::sealed::RegSpec for Ftdrl_SPEC {
    type DataType = u8;
}

#[doc = "Transmit FIFO Data Register L"]
pub type Ftdrl = crate::RegValueT<Ftdrl_SPEC>;

impl Ftdrl {
    #[doc = "Serial transmit data(b7-b0) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub fn tdatl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ftdrl_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ftdrl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Ftdrl {
    #[inline(always)]
    fn default() -> Ftdrl {
        <crate::RegValueT<Ftdrl_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdrhl_SPEC;
impl crate::sealed::RegSpec for Rdrhl_SPEC {
    type DataType = u16;
}

#[doc = "Receive 9-bit Data Register"]
pub type Rdrhl = crate::RegValueT<Rdrhl_SPEC>;

impl Rdrhl {
    #[doc = "RDRHL is an 16-bit register that stores receive data."]
    #[inline(always)]
    pub fn rdrhl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Rdrhl_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Rdrhl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rdrhl {
    #[inline(always)]
    fn default() -> Rdrhl {
        <crate::RegValueT<Rdrhl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frdrhl_SPEC;
impl crate::sealed::RegSpec for Frdrhl_SPEC {
    type DataType = u16;
}

#[doc = "Receive FIFO Data Register HL"]
pub type Frdrhl = crate::RegValueT<Frdrhl_SPEC>;

impl Frdrhl {
    #[doc = "Receive FIFO data full flag(It is same as SSR.RDF)"]
    #[inline(always)]
    pub fn rdf(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        frdrhl::Rdf,
        frdrhl::Rdf,
        Frdrhl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            frdrhl::Rdf,
            frdrhl::Rdf,
            Frdrhl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Overrun error flag(It is same as SSR.ORER)"]
    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        frdrhl::Orer,
        frdrhl::Orer,
        Frdrhl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            frdrhl::Orer,
            frdrhl::Orer,
            Frdrhl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Framing error flag"]
    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        frdrhl::Fer,
        frdrhl::Fer,
        Frdrhl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            frdrhl::Fer,
            frdrhl::Fer,
            Frdrhl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Parity error flag"]
    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        frdrhl::Per,
        frdrhl::Per,
        Frdrhl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            frdrhl::Per,
            frdrhl::Per,
            Frdrhl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive data ready flag(It is same as SSR.DR)"]
    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        frdrhl::Dr,
        frdrhl::Dr,
        Frdrhl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            frdrhl::Dr,
            frdrhl::Dr,
            Frdrhl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Multi-processor bit flag(Valid only in asynchronous mode with SMR.MP=1 and FIFO selected) It can read multi-processor bit corresponded to serial receive data(RDATA\\[8:0\\])"]
    #[inline(always)]
    pub fn mpb(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        frdrhl::Mpb,
        frdrhl::Mpb,
        Frdrhl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            frdrhl::Mpb,
            frdrhl::Mpb,
            Frdrhl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Serial receive data(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub fn rdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Frdrhl_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Frdrhl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Frdrhl {
    #[inline(always)]
    fn default() -> Frdrhl {
        <crate::RegValueT<Frdrhl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod frdrhl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdf_SPEC;
    pub type Rdf = crate::EnumBitfieldStruct<u8, Rdf_SPEC>;
    impl Rdf {
        #[doc = "The quantity of receive data written in FRDRH and FRDRL falls below the specified receive triggering number."]
        pub const _0: Self = Self::new(0);

        #[doc = "The quantity of receive data written in FRDRH and FRDRL is equal to or greater than the specified receive triggering number."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        #[doc = "No overrun error occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "An overrun error has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        #[doc = "No framing error occurred at the first data of FRDRH and FRDRL."]
        pub const _0: Self = Self::new(0);

        #[doc = "A framing error has occurred at the first data of FRDRH and FRDRL."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "No parity error occurred at the first data of FRDRH and FRDRL."]
        pub const _0: Self = Self::new(0);

        #[doc = "A parity error has occurred at the first data of FRDRH and FRDRL."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
        #[doc = "Receiving is in progress, or no received data has remained in FRDRH and FRDRL after normally completed receiving."]
        pub const _0: Self = Self::new(0);

        #[doc = "Next receive data has not been received for a period after normal completed receiving."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpb_SPEC;
    pub type Mpb = crate::EnumBitfieldStruct<u8, Mpb_SPEC>;
    impl Mpb {
        #[doc = "Data transmission cycles"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycles"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frdrh_SPEC;
impl crate::sealed::RegSpec for Frdrh_SPEC {
    type DataType = u8;
}

#[doc = "Receive FIFO Data Register H"]
pub type Frdrh = crate::RegValueT<Frdrh_SPEC>;

impl Frdrh {
    #[doc = "Receive FIFO data full flag(It is same as SSR.RDF)"]
    #[inline(always)]
    pub fn rdf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        frdrh::Rdf,
        frdrh::Rdf,
        Frdrh_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            frdrh::Rdf,
            frdrh::Rdf,
            Frdrh_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Overrun error flag(It is same as SSR.ORER)"]
    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        frdrh::Orer,
        frdrh::Orer,
        Frdrh_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            frdrh::Orer,
            frdrh::Orer,
            Frdrh_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Framing error flag"]
    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        frdrh::Fer,
        frdrh::Fer,
        Frdrh_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            frdrh::Fer,
            frdrh::Fer,
            Frdrh_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Parity error flag"]
    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        frdrh::Per,
        frdrh::Per,
        Frdrh_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            frdrh::Per,
            frdrh::Per,
            Frdrh_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive data ready flag(It is same as SSR.DR)"]
    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        frdrh::Dr,
        frdrh::Dr,
        Frdrh_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            frdrh::Dr,
            frdrh::Dr,
            Frdrh_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Multi-processor bit flag(Valid only in asynchronous mode with SMR.MP=1 and FIFO selected) It can read multi-processor bit corresponded to serial receive data(RDATA\\[8:0\\])"]
    #[inline(always)]
    pub fn mpb(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        frdrh::Mpb,
        frdrh::Mpb,
        Frdrh_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            frdrh::Mpb,
            frdrh::Mpb,
            Frdrh_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Serial receive data(b8)(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub fn rdath(self) -> crate::common::RegisterFieldBool<0, 1, 0, Frdrh_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Frdrh_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Frdrh {
    #[inline(always)]
    fn default() -> Frdrh {
        <crate::RegValueT<Frdrh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod frdrh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdf_SPEC;
    pub type Rdf = crate::EnumBitfieldStruct<u8, Rdf_SPEC>;
    impl Rdf {
        #[doc = "The quantity of receive data written in FRDRH and FRDRL falls below the specified receive triggering number."]
        pub const _0: Self = Self::new(0);

        #[doc = "The quantity of receive data written in FRDRH and FRDRL is equal to or greater than the specified receive triggering number."]
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
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        #[doc = "No framing error occurred at the first data of FRDRH and FRDRL"]
        pub const _0: Self = Self::new(0);

        #[doc = "A framing error has occurred at the first data of FRDRH and FRDRL"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "No parity error occurred at the first data of FRDRH and FRDRL"]
        pub const _0: Self = Self::new(0);

        #[doc = "A parity error has occurred at the first data of FRDRH and FRDRL"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
        #[doc = "Receiving is in progress, or no received data has remained in FRDRH and FRDRL after normally completed receiving."]
        pub const _0: Self = Self::new(0);

        #[doc = "Next receive data has not been received for a period after normal completed receiving."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpb_SPEC;
    pub type Mpb = crate::EnumBitfieldStruct<u8, Mpb_SPEC>;
    impl Mpb {
        #[doc = "Data transmission cycles"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycles"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frdrl_SPEC;
impl crate::sealed::RegSpec for Frdrl_SPEC {
    type DataType = u8;
}

#[doc = "Receive FIFO Data Register L"]
pub type Frdrl = crate::RegValueT<Frdrl_SPEC>;

impl Frdrl {
    #[doc = "Serial receive data(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)NOTE: When reading both of FRDRH register and FRDRL register, please read by an order of the FRDRH register and the FRDRL register."]
    #[inline(always)]
    pub fn rdatl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Frdrl_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Frdrl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Frdrl {
    #[inline(always)]
    fn default() -> Frdrl {
        <crate::RegValueT<Frdrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mddr_SPEC;
impl crate::sealed::RegSpec for Mddr_SPEC {
    type DataType = u8;
}

#[doc = "Modulation Duty Register"]
pub type Mddr = crate::RegValueT<Mddr_SPEC>;

impl Mddr {
    #[doc = "MDDR corrects the bit rate adjusted by the BRR register."]
    #[inline(always)]
    pub fn mddr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Mddr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Mddr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mddr {
    #[inline(always)]
    fn default() -> Mddr {
        <crate::RegValueT<Mddr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dccr_SPEC;
impl crate::sealed::RegSpec for Dccr_SPEC {
    type DataType = u8;
}

#[doc = "Data Compare Match Control Register"]
pub type Dccr = crate::RegValueT<Dccr_SPEC>;

impl Dccr {
    #[doc = "Data Compare Match Enable(Valid only in asynchronous mode(including multi-processor)"]
    #[inline(always)]
    pub fn dcme(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dccr::Dcme,
        dccr::Dcme,
        Dccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dccr::Dcme,
            dccr::Dcme,
            Dccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ID frame select Bit(Valid only in asynchronous mode(including multi-processor)"]
    #[inline(always)]
    pub fn idsel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dccr::Idsel,
        dccr::Idsel,
        Dccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dccr::Idsel,
            dccr::Idsel,
            Dccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Compare Match Framing Error Flag"]
    #[inline(always)]
    pub fn dfer(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dccr::Dfer,
        dccr::Dfer,
        Dccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dccr::Dfer,
            dccr::Dfer,
            Dccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Compare Match Parity Error Flag"]
    #[inline(always)]
    pub fn dper(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dccr::Dper,
        dccr::Dper,
        Dccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dccr::Dper,
            dccr::Dper,
            Dccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Compare Match Flag"]
    #[inline(always)]
    pub fn dcmf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dccr::Dcmf,
        dccr::Dcmf,
        Dccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dccr::Dcmf,
            dccr::Dcmf,
            Dccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dccr {
    #[inline(always)]
    fn default() -> Dccr {
        <crate::RegValueT<Dccr_SPEC> as RegisterValue<_>>::new(64)
    }
}
pub mod dccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcme_SPEC;
    pub type Dcme = crate::EnumBitfieldStruct<u8, Dcme_SPEC>;
    impl Dcme {
        #[doc = "Address match function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Address match function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idsel_SPEC;
    pub type Idsel = crate::EnumBitfieldStruct<u8, Idsel_SPEC>;
    impl Idsel {
        #[doc = "It\'s always compared data in spite of the value of the MPB bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "It\'s compared data when the MPB bit is 1 ( ID frame ) only."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfer_SPEC;
    pub type Dfer = crate::EnumBitfieldStruct<u8, Dfer_SPEC>;
    impl Dfer {
        #[doc = "No framing error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A framing error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dper_SPEC;
    pub type Dper = crate::EnumBitfieldStruct<u8, Dper_SPEC>;
    impl Dper {
        #[doc = "No parity error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A parity error has occurred"]
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcr_SPEC;
impl crate::sealed::RegSpec for Fcr_SPEC {
    type DataType = u16;
}

#[doc = "FIFO Control Register"]
pub type Fcr = crate::RegValueT<Fcr_SPEC>;

impl Fcr {
    #[doc = "RTS# Output Active Trigger Number Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn rstrg(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        fcr::Rstrg,
        fcr::Rstrg,
        Fcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            fcr::Rstrg,
            fcr::Rstrg,
            Fcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive FIFO data trigger number"]
    #[inline(always)]
    pub fn rtrg(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, fcr::Rtrg, fcr::Rtrg, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            fcr::Rtrg,
            fcr::Rtrg,
            Fcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit FIFO data trigger number(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn ttrg(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, fcr::Ttrg, fcr::Ttrg, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            fcr::Ttrg,
            fcr::Ttrg,
            Fcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive data ready error select bit(When detecting a reception data ready, the interrupt request is selected.)"]
    #[inline(always)]
    pub fn dres(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, fcr::Dres, fcr::Dres, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fcr::Dres,
            fcr::Dres,
            Fcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit FIFO Data Register Reset(Valid only in FCR.FM=1)"]
    #[inline(always)]
    pub fn tfrst(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fcr::Tfrst,
        fcr::Tfrst,
        Fcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fcr::Tfrst,
            fcr::Tfrst,
            Fcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive FIFO Data Register Reset(Valid only in FCR.FM=1)"]
    #[inline(always)]
    pub fn rfrst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fcr::Rfrst,
        fcr::Rfrst,
        Fcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fcr::Rfrst,
            fcr::Rfrst,
            Fcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Mode Select(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn fm(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, fcr::Fm, fcr::Fm, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,fcr::Fm,fcr::Fm,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fcr {
    #[inline(always)]
    fn default() -> Fcr {
        <crate::RegValueT<Fcr_SPEC> as RegisterValue<_>>::new(63488)
    }
}
pub mod fcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstrg_SPEC;
    pub type Rstrg = crate::EnumBitfieldStruct<u8, Rstrg_SPEC>;
    impl Rstrg {
        #[doc = "Trigger number 0"]
        pub const _0000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtrg_SPEC;
    pub type Rtrg = crate::EnumBitfieldStruct<u8, Rtrg_SPEC>;
    impl Rtrg {
        #[doc = "Trigger number 0"]
        pub const _0000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ttrg_SPEC;
    pub type Ttrg = crate::EnumBitfieldStruct<u8, Ttrg_SPEC>;
    impl Ttrg {
        #[doc = "Trigger number 0"]
        pub const _0000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dres_SPEC;
    pub type Dres = crate::EnumBitfieldStruct<u8, Dres_SPEC>;
    impl Dres {
        #[doc = "reception data full interrupt (RXI)"]
        pub const _0: Self = Self::new(0);

        #[doc = "receive error interrupt (ERI)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfrst_SPEC;
    pub type Tfrst = crate::EnumBitfieldStruct<u8, Tfrst_SPEC>;
    impl Tfrst {
        #[doc = "The number of data stored in FTDRH and FTDRL register are NOT made 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "The number of data stored in FTDRH and FTDRL register are made 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfrst_SPEC;
    pub type Rfrst = crate::EnumBitfieldStruct<u8, Rfrst_SPEC>;
    impl Rfrst {
        #[doc = "The number of data stored in FRDRH and FRDRL register are NOT made 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "The number of data stored in FRDRH and FRDRL register are made 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fm_SPEC;
    pub type Fm = crate::EnumBitfieldStruct<u8, Fm_SPEC>;
    impl Fm {
        #[doc = "Non-FIFO mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdr_SPEC;
impl crate::sealed::RegSpec for Fdr_SPEC {
    type DataType = u16;
}

#[doc = "FIFO Data Count Register"]
pub type Fdr = crate::RegValueT<Fdr_SPEC>;

impl Fdr {
    #[doc = "Transmit FIFO Data CountIndicate the quantity of non-transmit data stored in FTDRH and FTDRL(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)"]
    #[inline(always)]
    pub fn t(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Fdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Fdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive FIFO Data CountIndicate the quantity of receive data stored in FRDRH and FRDRL(Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, while FCR.FM=1)"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Fdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Fdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fdr {
    #[inline(always)]
    fn default() -> Fdr {
        <crate::RegValueT<Fdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lsr_SPEC;
impl crate::sealed::RegSpec for Lsr_SPEC {
    type DataType = u16;
}

#[doc = "Line Status Register"]
pub type Lsr = crate::RegValueT<Lsr_SPEC>;

impl Lsr {
    #[doc = "Parity Error CountIndicates the quantity of data with a parity error among the receive data stored in the receive FIFO data register (FRDRH and FRDRL)."]
    #[inline(always)]
    pub fn pnum(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Lsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Lsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Framing Error CountIndicates the quantity of data with a framing error among the receive data stored in the receive FIFO data register (FRDRH and FRDRL)."]
    #[inline(always)]
    pub fn fnum(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, Lsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,Lsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Overrun Error Flag (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, lsr::Orer, lsr::Orer, Lsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,lsr::Orer,lsr::Orer,Lsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Lsr {
    #[inline(always)]
    fn default() -> Lsr {
        <crate::RegValueT<Lsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        #[doc = "No overrun error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "An overrun error has occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdr_SPEC;
impl crate::sealed::RegSpec for Cdr_SPEC {
    type DataType = u16;
}

#[doc = "Compare Match Data Register"]
pub type Cdr = crate::RegValueT<Cdr_SPEC>;

impl Cdr {
    #[doc = "Compare Match DataCompare data pattern for address match wake-up function"]
    #[inline(always)]
    pub fn cmpd(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Cdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Cdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdr {
    #[inline(always)]
    fn default() -> Cdr {
        <crate::RegValueT<Cdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sptr_SPEC;
impl crate::sealed::RegSpec for Sptr_SPEC {
    type DataType = u8;
}

#[doc = "Serial Port Register"]
pub type Sptr = crate::RegValueT<Sptr_SPEC>;

impl Sptr {
    #[doc = "Serial port break I/O bit(It\'s selected whether the value of SPB2DT is output to TxD terminal.)"]
    #[inline(always)]
    pub fn spb2io(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sptr::Spb2Io,
        sptr::Spb2Io,
        Sptr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sptr::Spb2Io,
            sptr::Spb2Io,
            Sptr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial port break data select bit(The output level of TxD terminal is selected when SCR.TE = 0.)"]
    #[inline(always)]
    pub fn spb2dt(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sptr::Spb2Dt,
        sptr::Spb2Dt,
        Sptr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sptr::Spb2Dt,
            sptr::Spb2Dt,
            Sptr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial input data monitor bit(The state of the RXD terminal is shown.)"]
    #[inline(always)]
    pub fn rxdmon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sptr::Rxdmon,
        sptr::Rxdmon,
        Sptr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sptr::Rxdmon,
            sptr::Rxdmon,
            Sptr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sptr {
    #[inline(always)]
    fn default() -> Sptr {
        <crate::RegValueT<Sptr_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod sptr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb2Io_SPEC;
    pub type Spb2Io = crate::EnumBitfieldStruct<u8, Spb2Io_SPEC>;
    impl Spb2Io {
        #[doc = "The value of SPB2DT bit isn\'t output in TxD terminal."]
        pub const _0: Self = Self::new(0);

        #[doc = "The value of SPB2DT bit is output in TxD terminal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb2Dt_SPEC;
    pub type Spb2Dt = crate::EnumBitfieldStruct<u8, Spb2Dt_SPEC>;
    impl Spb2Dt {
        #[doc = "Low level is output in TxD terminal."]
        pub const _0: Self = Self::new(0);

        #[doc = "High level is output in TxD terminal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdmon_SPEC;
    pub type Rxdmon = crate::EnumBitfieldStruct<u8, Rxdmon_SPEC>;
    impl Rxdmon {
        #[doc = "RXD terminal is the Low level."]
        pub const _0: Self = Self::new(0);

        #[doc = "RXD terminal is the High level."]
        pub const _1: Self = Self::new(1);
    }
}
