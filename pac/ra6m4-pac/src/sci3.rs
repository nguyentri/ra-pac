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
// Generated from SVD 1.30.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:13:05 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Serial Communication Interface"]
unsafe impl ::core::marker::Send for super::Sci3 {}
unsafe impl ::core::marker::Sync for super::Sci3 {}
impl super::Sci3 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Serial Mode Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn smr(&self) -> &'static crate::common::Reg<self::Smr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Serial Mode Register for Smart Card Interface Mode (SCMR.SMIF = 1)"]
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

    #[doc = "Serial Control Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn scr(&self) -> &'static crate::common::Reg<self::Scr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Serial Control Register for Smart Card Interface Mode (SCMR.SMIF = 1)"]
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

    #[doc = "Serial Status Register for Non-Smart Card Interface and Non-FIFO Mode (SCMR.SMIF = 0, FCR.FM = 0, and MMR.MANEN = 0)"]
    #[inline(always)]
    pub const fn ssr(&self) -> &'static crate::common::Reg<self::Ssr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Serial Status Register for Non-Smart Card Interface and FIFO Mode (SCMR.SMIF = 0, FCR.FM = 1, and MMR.MANEN = 0)"]
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

    #[doc = "Serial Status Register for Manchester Mode (SCMR.SMIF = 0, and MMR.MANEN = 1)"]
    #[inline(always)]
    pub const fn ssr_manc(
        &self,
    ) -> &'static crate::common::Reg<self::SsrManc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SsrManc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Serial Status Register for Smart Card Interface Mode (SCMR.SMIF = 1, and MMR.MANEN = 0)"]
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

    #[doc = "IIC Mode Register 1"]
    #[inline(always)]
    pub const fn simr1(&self) -> &'static crate::common::Reg<self::Simr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Simr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }

    #[doc = "IIC Mode Register 2"]
    #[inline(always)]
    pub const fn simr2(&self) -> &'static crate::common::Reg<self::Simr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Simr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "IIC Mode Register 3"]
    #[inline(always)]
    pub const fn simr3(&self) -> &'static crate::common::Reg<self::Simr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Simr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[doc = "IIC Status Register"]
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

    #[doc = "Transmit FIFO Data Register"]
    #[inline(always)]
    pub const fn ftdrhl(&self) -> &'static crate::common::Reg<self::Ftdrhl_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ftdrhl_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Transmit Data Register for Non-Manchester mode (MMR.MANEN = 0)"]
    #[inline(always)]
    pub const fn tdrhl(&self) -> &'static crate::common::Reg<self::Tdrhl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdrhl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Transmit Data Register for Manchester mode (MMR.MANEN = 1)"]
    #[inline(always)]
    pub const fn tdrhl_man(
        &self,
    ) -> &'static crate::common::Reg<self::TdrhlMan_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TdrhlMan_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Transmit FIFO Data Register"]
    #[inline(always)]
    pub const fn ftdrh(&self) -> &'static crate::common::Reg<self::Ftdrh_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ftdrh_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Transmit FIFO Data Register"]
    #[inline(always)]
    pub const fn ftdrl(&self) -> &'static crate::common::Reg<self::Ftdrl_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ftdrl_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(15usize),
            )
        }
    }

    #[doc = "Receive FIFO Data Register"]
    #[inline(always)]
    pub const fn frdrhl(&self) -> &'static crate::common::Reg<self::Frdrhl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frdrhl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Receive Data Register for Non-Manchester mode (MMR.MANEN = 0)"]
    #[inline(always)]
    pub const fn rdrhl(&self) -> &'static crate::common::Reg<self::Rdrhl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdrhl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Receive Data Register for Manchester mode (MMR.MANEN = 1)"]
    #[inline(always)]
    pub const fn rdrhl_man(
        &self,
    ) -> &'static crate::common::Reg<self::RdrhlMan_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::RdrhlMan_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Receive FIFO Data Register"]
    #[inline(always)]
    pub const fn frdrh(&self) -> &'static crate::common::Reg<self::Frdrh_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frdrh_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Receive FIFO Data Register"]
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

    #[doc = "Adjustment Communication Timing Register"]
    #[inline(always)]
    pub const fn actr(&self) -> &'static crate::common::Reg<self::Actr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Actr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(29usize),
            )
        }
    }

    #[doc = "Manchester Mode Register"]
    #[inline(always)]
    pub const fn mmr(&self) -> &'static crate::common::Reg<self::Mmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Transmit Manchester Preface Setting Register"]
    #[inline(always)]
    pub const fn tmpr(&self) -> &'static crate::common::Reg<self::Tmpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[doc = "Receive Manchester Preface Setting Register"]
    #[inline(always)]
    pub const fn rmpr(&self) -> &'static crate::common::Reg<self::Rmpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rmpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(35usize),
            )
        }
    }

    #[doc = "Manchester Extended Error Status Register"]
    #[inline(always)]
    pub const fn mesr(&self) -> &'static crate::common::Reg<self::Mesr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mesr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Manchester Extended Error Control Register"]
    #[inline(always)]
    pub const fn mecr(&self) -> &'static crate::common::Reg<self::Mecr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mecr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(37usize),
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

#[doc = "Serial Mode Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)"]
pub type Smr = crate::RegValueT<Smr_SPEC>;

impl Smr {
    #[doc = "Clock Select"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, smr::Cks, smr::Cks, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,smr::Cks,smr::Cks,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Multi-Processor Mode"]
    #[inline(always)]
    pub fn mp(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, smr::Mp, smr::Mp, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,smr::Mp,smr::Mp,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Stop Bit Length"]
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

    #[doc = "Parity Mode"]
    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, smr::Pm, smr::Pm, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,smr::Pm,smr::Pm,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Parity Enable"]
    #[inline(always)]
    pub fn pe(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, smr::Pe, smr::Pe, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,smr::Pe,smr::Pe,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Character Length"]
    #[inline(always)]
    pub fn chr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, smr::Chr, smr::Chr, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,smr::Chr,smr::Chr,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Communication Mode"]
    #[inline(always)]
    pub fn cm(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, smr::Cm, smr::Cm, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,smr::Cm,smr::Cm,Smr_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "PCLK clock (n = 0)"]
        pub const _00: Self = Self::new(0);

        #[doc = "PCLK/4 clock (n = 1)"]
        pub const _01: Self = Self::new(1);

        #[doc = "PCLK/16 clock (n = 2)"]
        pub const _10: Self = Self::new(2);

        #[doc = "PCLK/64 clock (n = 3)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mp_SPEC;
    pub type Mp = crate::EnumBitfieldStruct<u8, Mp_SPEC>;
    impl Mp {
        #[doc = "Disable multi-processor communications function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable multi-processor communications function"]
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
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        #[doc = "Even parity"]
        pub const _0: Self = Self::new(0);

        #[doc = "Odd parity"]
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
    pub struct Chr_SPEC;
    pub type Chr = crate::EnumBitfieldStruct<u8, Chr_SPEC>;
    impl Chr {
        #[doc = "SCMR.CHR1 = 0: Transmit/receive in 9-bit data length SCMR.CHR1 = 1: Transmit/receive in 8-bit data length (initial value)"]
        pub const _0: Self = Self::new(0);

        #[doc = "SCMR.CHR1 = 0: Transmit/receive in 9-bit data length SCMR.CHR1 = 1: Transmit/receive in 7-bit data length"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cm_SPEC;
    pub type Cm = crate::EnumBitfieldStruct<u8, Cm_SPEC>;
    impl Cm {
        #[doc = "Asynchronous mode or simple IIC mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clock synchronous mode or simple SPI mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmrSmci_SPEC;
impl crate::sealed::RegSpec for SmrSmci_SPEC {
    type DataType = u8;
}

#[doc = "Serial Mode Register for Smart Card Interface Mode (SCMR.SMIF = 1)"]
pub type SmrSmci = crate::RegValueT<SmrSmci_SPEC>;

impl SmrSmci {
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

    #[doc = "Base Clock Pulse"]
    #[inline(always)]
    pub fn bcp(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, SmrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,SmrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Parity Mode"]
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

    #[doc = "Parity Enable"]
    #[inline(always)]
    pub fn pe(self) -> crate::common::RegisterFieldBool<5, 1, 0, SmrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, SmrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
}
impl ::core::default::Default for SmrSmci {
    #[inline(always)]
    fn default() -> SmrSmci {
        <crate::RegValueT<SmrSmci_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smr_smci {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "PCLK clock (n = 0)"]
        pub const _00: Self = Self::new(0);

        #[doc = "PCLK/4 clock (n = 1)"]
        pub const _01: Self = Self::new(1);

        #[doc = "PCLK/16 clock (n = 2)"]
        pub const _10: Self = Self::new(2);

        #[doc = "PCLK/64 clock (n = 3)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        #[doc = "Even parity"]
        pub const _0: Self = Self::new(0);

        #[doc = "Odd parity"]
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
    pub struct Gm_SPEC;
    pub type Gm = crate::EnumBitfieldStruct<u8, Gm_SPEC>;
    impl Gm {
        #[doc = "Normal mode operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "GSM mode operation"]
        pub const _1: Self = Self::new(1);
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

impl NoBitfieldReg<Brr_SPEC> for Brr {}
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

#[doc = "Serial Control Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)"]
pub type Scr = crate::RegValueT<Scr_SPEC>;

impl Scr {
    #[doc = "Clock Enable"]
    #[inline(always)]
    pub fn cke(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, scr::Cke, scr::Cke, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,scr::Cke,scr::Cke,Scr_SPEC,crate::common::RW>::from_register(self,0)
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

    #[doc = "Multi-Processor Interrupt Enable"]
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

    #[doc = "Receive Enable"]
    #[inline(always)]
    pub fn re(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, scr::Re, scr::Re, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,scr::Re,scr::Re,Scr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit Enable"]
    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, scr::Te, scr::Te, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,scr::Te,scr::Te,Scr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, scr::Rie, scr::Rie, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,scr::Rie,scr::Rie,Scr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, scr::Tie, scr::Tie, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,scr::Tie,scr::Tie,Scr_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Cke_SPEC;
    pub type Cke = crate::EnumBitfieldStruct<u8, Cke_SPEC>;
    impl Cke {
        #[doc = "In asynchronous mode, the SCKn pin is available for use as an I/O port based on the I/O port settings. In clock synchronous mode, the SCKn pin functions as the clock output pin."]
        pub const _00: Self = Self::new(0);

        #[doc = "In asynchronous mode, a clock with the same frequency as the bit rate is output from the SCKn pin. In clock synchronous mode, the SCKn pin functions as the clock output pin."]
        pub const _01: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teie_SPEC;
    pub type Teie = crate::EnumBitfieldStruct<u8, Teie_SPEC>;
    impl Teie {
        #[doc = "Disable SCIn_TEI interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable SCIn_TEI interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpie_SPEC;
    pub type Mpie = crate::EnumBitfieldStruct<u8, Mpie_SPEC>;
    impl Mpie {
        #[doc = "Normal reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "When data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF, ORER, and FER in SSR to 1 and the status flags SYER, PFER, and SBER in MESR are disabled. When data with the multi-processor bit set to 1 is received, the MPIE bit is automatically set to 0, and normal reception is resumed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        #[doc = "Disable serial reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable serial reception"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te_SPEC;
    pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
    impl Te {
        #[doc = "Disable serial transmission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable serial transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "Disable SCIn_RXI and SCIn_ERI interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable SCIn_RXI and SCIn_ERI interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        #[doc = "Disable SCIn_TXI interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable SCIn_TXI interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScrSmci_SPEC;
impl crate::sealed::RegSpec for ScrSmci_SPEC {
    type DataType = u8;
}

#[doc = "Serial Control Register for Smart Card Interface Mode (SCMR.SMIF = 1)"]
pub type ScrSmci = crate::RegValueT<ScrSmci_SPEC>;

impl ScrSmci {
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

    #[doc = "Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ScrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, ScrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Multi-Processor Interrupt Enable"]
    #[inline(always)]
    pub fn mpie(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ScrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, ScrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
}
impl ::core::default::Default for ScrSmci {
    #[inline(always)]
    fn default() -> ScrSmci {
        <crate::RegValueT<ScrSmci_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scr_smci {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cke_SPEC;
    pub type Cke = crate::EnumBitfieldStruct<u8, Cke_SPEC>;
    impl Cke {
        #[doc = "When SMR_SMCI.GM = 0: Disable output The SCKn pin is available for use as an I/O port if set up in the I/O port settings When SMR_SMCI.GM = 1: Fix output low"]
        pub const _00: Self = Self::new(0);

        #[doc = "When SMR_SMCI.GM = 0: Output clock When SMR_SMCI.GM = 1: Output clock"]
        pub const _01: Self = Self::new(1);

        #[doc = "When SMR_SMCI.GM = 0: Setting prohibited When SMR_SMCI.GM = 1: Fix output high"]
        pub const _10: Self = Self::new(2);

        #[doc = "When SMR_SMCI.GM = 0: Setting prohibited When SMR_SMCI.GM = 1: Output clock"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        #[doc = "Disable serial reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable serial reception"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te_SPEC;
    pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
    impl Te {
        #[doc = "Disable serial transmission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable serial transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "Disable SCIn_RXI and SCIn_ERI interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable SCIn_RXI and SCIn_ERI interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        #[doc = "Disable SCIn_TXI interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable SCIn_TXI interrupt requests"]
        pub const _1: Self = Self::new(1);
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

impl NoBitfieldReg<Tdr_SPEC> for Tdr {}
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

#[doc = "Serial Status Register for Non-Smart Card Interface and Non-FIFO Mode (SCMR.SMIF = 0, FCR.FM = 0, and MMR.MANEN = 0)"]
pub type Ssr = crate::RegValueT<Ssr_SPEC>;

impl Ssr {
    #[doc = "Multi-Processor Bit Transfer"]
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

    #[doc = "Multi-Processor"]
    #[inline(always)]
    pub fn mpb(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ssr::Mpb, ssr::Mpb, Ssr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,ssr::Mpb,ssr::Mpb,Ssr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ssr::Tend, ssr::Tend, Ssr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,ssr::Tend,ssr::Tend,Ssr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ssr::Per, ssr::Per, Ssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,ssr::Per,ssr::Per,Ssr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Framing Error Flag"]
    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ssr::Fer, ssr::Fer, Ssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ssr::Fer,ssr::Fer,Ssr_SPEC,crate::common::RW>::from_register(self,0)
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
}
impl ::core::default::Default for Ssr {
    #[inline(always)]
    fn default() -> Ssr {
        <crate::RegValueT<Ssr_SPEC> as RegisterValue<_>>::new(132)
    }
}
pub mod ssr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpbt_SPEC;
    pub type Mpbt = crate::EnumBitfieldStruct<u8, Mpbt_SPEC>;
    impl Mpbt {
        #[doc = "Data transmission cycle"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycle"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpb_SPEC;
    pub type Mpb = crate::EnumBitfieldStruct<u8, Mpb_SPEC>;
    impl Mpb {
        #[doc = "Data transmission cycle"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycle"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        #[doc = "A character is being transmitted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Character transfer is complete"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "No parity error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Parity error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        #[doc = "No framing error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Framing error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        #[doc = "No overrun error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Overrun error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrf_SPEC;
    pub type Rdrf = crate::EnumBitfieldStruct<u8, Rdrf_SPEC>;
    impl Rdrf {
        #[doc = "No received data in RDR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received data in RDR register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        #[doc = "Transmit data in TDR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "No transmit data in TDR register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsrFifo_SPEC;
impl crate::sealed::RegSpec for SsrFifo_SPEC {
    type DataType = u8;
}

#[doc = "Serial Status Register for Non-Smart Card Interface and FIFO Mode (SCMR.SMIF = 0, FCR.FM = 1, and MMR.MANEN = 0)"]
pub type SsrFifo = crate::RegValueT<SsrFifo_SPEC>;

impl SsrFifo {
    #[doc = "Receive Data Ready Flag"]
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

    #[doc = "Receive FIFO Data Full Flag"]
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

    #[doc = "Transmit FIFO Data Empty Flag"]
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
}
impl ::core::default::Default for SsrFifo {
    #[inline(always)]
    fn default() -> SsrFifo {
        <crate::RegValueT<SsrFifo_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod ssr_fifo {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
        #[doc = "Receiving is in progress, or no received data remains in FRDRHL after successfully completed reception (receive FIFO empty)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Next receive data is not received for a period after normal receiving is complete, when the amount of data stored in the FIFO is equal to or less than the receive triggering number"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        #[doc = "A character is being transmitted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Character transfer is complete"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "No parity error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Parity error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        #[doc = "No framing error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Framing error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        #[doc = "No overrun error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Overrun error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdf_SPEC;
    pub type Rdf = crate::EnumBitfieldStruct<u8, Rdf_SPEC>;
    impl Rdf {
        #[doc = "The amount of receive data written in FRDRHL is less than the specified receive triggering number"]
        pub const _0: Self = Self::new(0);

        #[doc = "The amount of receive data written in FRDRHL is equal to or greater than the specified receive triggering number"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdfe_SPEC;
    pub type Tdfe = crate::EnumBitfieldStruct<u8, Tdfe_SPEC>;
    impl Tdfe {
        #[doc = "The amount of transmit data written in FTDRHL exceeds the specified transmit triggering number"]
        pub const _0: Self = Self::new(0);

        #[doc = "The amount of transmit data written in FTDRHL is equal to or less than the specified transmit triggering number"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsrManc_SPEC;
impl crate::sealed::RegSpec for SsrManc_SPEC {
    type DataType = u8;
}

#[doc = "Serial Status Register for Manchester Mode (SCMR.SMIF = 0, and MMR.MANEN = 1)"]
pub type SsrManc = crate::RegValueT<SsrManc_SPEC>;

impl SsrManc {
    #[doc = "Manchester Error Flag"]
    #[inline(always)]
    pub fn mer(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssr_manc::Mer,
        ssr_manc::Mer,
        SsrManc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssr_manc::Mer,
            ssr_manc::Mer,
            SsrManc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multi-Processor"]
    #[inline(always)]
    pub fn mpb(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ssr_manc::Mpb,
        ssr_manc::Mpb,
        SsrManc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ssr_manc::Mpb,
            ssr_manc::Mpb,
            SsrManc_SPEC,
            crate::common::R,
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
        ssr_manc::Tend,
        ssr_manc::Tend,
        SsrManc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ssr_manc::Tend,
            ssr_manc::Tend,
            SsrManc_SPEC,
            crate::common::R,
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
        ssr_manc::Per,
        ssr_manc::Per,
        SsrManc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ssr_manc::Per,
            ssr_manc::Per,
            SsrManc_SPEC,
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
        ssr_manc::Fer,
        ssr_manc::Fer,
        SsrManc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ssr_manc::Fer,
            ssr_manc::Fer,
            SsrManc_SPEC,
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
        ssr_manc::Orer,
        ssr_manc::Orer,
        SsrManc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ssr_manc::Orer,
            ssr_manc::Orer,
            SsrManc_SPEC,
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
        ssr_manc::Rdrf,
        ssr_manc::Rdrf,
        SsrManc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ssr_manc::Rdrf,
            ssr_manc::Rdrf,
            SsrManc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ssr_manc::Tdre,
        ssr_manc::Tdre,
        SsrManc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ssr_manc::Tdre,
            ssr_manc::Tdre,
            SsrManc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SsrManc {
    #[inline(always)]
    fn default() -> SsrManc {
        <crate::RegValueT<SsrManc_SPEC> as RegisterValue<_>>::new(132)
    }
}
pub mod ssr_manc {

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
    pub struct Mpb_SPEC;
    pub type Mpb = crate::EnumBitfieldStruct<u8, Mpb_SPEC>;
    impl Mpb {
        #[doc = "Data transmission cycles"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycles"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        #[doc = "A character is being transmitted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Character transfer has been completed."]
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
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        #[doc = "No framing error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A framing error has occurred"]
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
    pub struct Rdrf_SPEC;
    pub type Rdrf = crate::EnumBitfieldStruct<u8, Rdrf_SPEC>;
    impl Rdrf {
        #[doc = "No received data is in RDR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received data is in RDR register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        #[doc = "Transmit data is in TDR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "No transmit data is in TDR register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsrSmci_SPEC;
impl crate::sealed::RegSpec for SsrSmci_SPEC {
    type DataType = u8;
}

#[doc = "Serial Status Register for Smart Card Interface Mode (SCMR.SMIF = 1, and MMR.MANEN = 0)"]
pub type SsrSmci = crate::RegValueT<SsrSmci_SPEC>;

impl SsrSmci {
    #[doc = "Multi-Processor Bit Transfer"]
    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SsrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, SsrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Multi-Processor"]
    #[inline(always)]
    pub fn mpb(self) -> crate::common::RegisterFieldBool<1, 1, 0, SsrSmci_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, SsrSmci_SPEC, crate::common::R>::from_register(
            self, 0,
        )
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
}
impl ::core::default::Default for SsrSmci {
    #[inline(always)]
    fn default() -> SsrSmci {
        <crate::RegValueT<SsrSmci_SPEC> as RegisterValue<_>>::new(132)
    }
}
pub mod ssr_smci {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        #[doc = "A character is being transmitted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Character transfer is complete"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "No parity error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Parity error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ers_SPEC;
    pub type Ers = crate::EnumBitfieldStruct<u8, Ers_SPEC>;
    impl Ers {
        #[doc = "No low error signal response"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low error signal response occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        #[doc = "No overrun error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Overrun error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrf_SPEC;
    pub type Rdrf = crate::EnumBitfieldStruct<u8, Rdrf_SPEC>;
    impl Rdrf {
        #[doc = "No received data in RDR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received data in RDR register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        #[doc = "Transmit data in TDR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "No transmit data in TDR register"]
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

impl NoBitfieldReg<Rdr_SPEC> for Rdr {}
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

    #[doc = "Transmitted/Received Data Invert"]
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

    #[doc = "Transmitted/Received Data Transfer Direction"]
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

    #[doc = "Character Length 1"]
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

    #[doc = "Base Clock Pulse 2"]
    #[inline(always)]
    pub fn bcp2(self) -> crate::common::RegisterFieldBool<7, 1, 0, Scmr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Scmr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
    pub struct Smif_SPEC;
    pub type Smif = crate::EnumBitfieldStruct<u8, Smif_SPEC>;
    impl Smif {
        #[doc = "Non-smart card interface mode (asynchronous mode, clock synchronous mode, simple SPI mode, or simple IIC mode)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Smart card interface mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sinv_SPEC;
    pub type Sinv = crate::EnumBitfieldStruct<u8, Sinv_SPEC>;
    impl Sinv {
        #[doc = "TDR contents are transmitted as they are. Received data is stored as received in the RDR register."]
        pub const _0: Self = Self::new(0);

        #[doc = "TDR register contents are inverted before transmission. Receive data is stored in inverted form in the RDR register."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdir_SPEC;
    pub type Sdir = crate::EnumBitfieldStruct<u8, Sdir_SPEC>;
    impl Sdir {
        #[doc = "Transfer LSB-first"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transfer MSB-first"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chr1_SPEC;
    pub type Chr1 = crate::EnumBitfieldStruct<u8, Chr1_SPEC>;
    impl Chr1 {
        #[doc = "SMR.CHR = 0: Transmit/receive in 9-bit data length SMR.CHR = 1: Transmit/receive in 9-bit data length"]
        pub const _0: Self = Self::new(0);

        #[doc = "SMR.CHR = 0: Transmit/receive in 8-bit data length (initial value) SMR.CHR = 1: Transmit/receive in 7-bit data length"]
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
    #[doc = "Asynchronous Mode Clock Source Select"]
    #[inline(always)]
    pub fn acs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        semr::Acs0,
        semr::Acs0,
        Semr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            semr::Acs0,
            semr::Acs0,
            Semr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Preamble function Disable"]
    #[inline(always)]
    pub fn padis(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        semr::Padis,
        semr::Padis,
        Semr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            semr::Padis,
            semr::Padis,
            Semr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bit Rate Modulation Enable"]
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

    #[doc = "Asynchronous Mode Extended Base Clock Select 1"]
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

    #[doc = "Asynchronous Mode Base Clock Select"]
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

    #[doc = "Digital Noise Filter Function Enable"]
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

    #[doc = "Baud Rate Generator Double-Speed Mode Select"]
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

    #[doc = "Asynchronous Start Bit Edge Detection Select"]
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
}
impl ::core::default::Default for Semr {
    #[inline(always)]
    fn default() -> Semr {
        <crate::RegValueT<Semr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod semr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acs0_SPEC;
    pub type Acs0 = crate::EnumBitfieldStruct<u8, Acs0_SPEC>;
    impl Acs0 {
        #[doc = "External clock input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Logical AND of compare matches output from the internal GPT. These bit for the other SCI channels than SCIn (n = 1, 2) are reserved."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padis_SPEC;
    pub type Padis = crate::EnumBitfieldStruct<u8, Padis_SPEC>;
    impl Padis {
        #[doc = "Preamble output function is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Preamble output function is disabled These bits for the other SCI channels than SCIn (n = 0, 3 to 9) are reserved."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brme_SPEC;
    pub type Brme = crate::EnumBitfieldStruct<u8, Brme_SPEC>;
    impl Brme {
        #[doc = "Disable bit rate modulation function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable bit rate modulation function"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcse_SPEC;
    pub type Abcse = crate::EnumBitfieldStruct<u8, Abcse_SPEC>;
    impl Abcse {
        #[doc = "Clock cycles for 1-bit period determined by combination of the BGDM and ABCS bits in the SEMR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Baud rate is 6 base clock cycles for 1-bit period These bits for the other SCI channels than SCIn (n = 0, 3 to 9) are reserved."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcs_SPEC;
    pub type Abcs = crate::EnumBitfieldStruct<u8, Abcs_SPEC>;
    impl Abcs {
        #[doc = "Select 16 base clock cycles for 1-bit period"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select 8 base clock cycles for 1-bit period"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "In asynchronous mode: Disable noise cancellation function for RXDn input signal In simple I2C mode: Disable noise cancellation function for SCLn and SDAn input signals"]
        pub const _0: Self = Self::new(0);

        #[doc = "In asynchronous mode: Enable noise cancellation function for RXDn input signal In simple I2C mode: Enable noise cancellation function for SCLn and SDAn input signals"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bgdm_SPEC;
    pub type Bgdm = crate::EnumBitfieldStruct<u8, Bgdm_SPEC>;
    impl Bgdm {
        #[doc = "Output clock from baud rate generator with normal frequency"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output clock from baud rate generator with doubled frequency"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdesel_SPEC;
    pub type Rxdesel = crate::EnumBitfieldStruct<u8, Rxdesel_SPEC>;
    impl Rxdesel {
        #[doc = "Detect low level on RXDn pin as start bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect falling edge of RXDn pin as start bit"]
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
        #[doc = "In asynchronous mode: Use clock signal divided by 1 with noise filter In simple I2C mode: Setting prohibited"]
        pub const _000: Self = Self::new(0);

        #[doc = "In asynchronous mode: Setting prohibited In simple I2C mode: Use clock signal divided by 1 with noise filter"]
        pub const _001: Self = Self::new(1);

        #[doc = "In asynchronous mode: Setting prohibited In simple I2C mode: Use clock signal divided by 2 with noise filter"]
        pub const _010: Self = Self::new(2);

        #[doc = "In asynchronous mode: Setting prohibited In simple I2C mode: Use clock signal divided by 4 with noise filter"]
        pub const _011: Self = Self::new(3);

        #[doc = "In asynchronous mode: Setting prohibited In simple I2C mode: Use clock signal divided by 8 with noise filter"]
        pub const _100: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr1_SPEC;
impl crate::sealed::RegSpec for Simr1_SPEC {
    type DataType = u8;
}

#[doc = "IIC Mode Register 1"]
pub type Simr1 = crate::RegValueT<Simr1_SPEC>;

impl Simr1 {
    #[doc = "Simple IIC Mode Select"]
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

    #[doc = "SDAn Delay Output Select"]
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
}
impl ::core::default::Default for Simr1 {
    #[inline(always)]
    fn default() -> Simr1 {
        <crate::RegValueT<Simr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod simr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicm_SPEC;
    pub type Iicm = crate::EnumBitfieldStruct<u8, Iicm_SPEC>;
    impl Iicm {
        #[doc = "SCMR.SMIF = 0: Asynchronous mode (including multi-processor mode), clock synchronous mode, or simple SPI mode SCMR.SMIF = 1: Smart card interface mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "SCMR.SMIF = 0: Simple IIC mode SCMR.SMIF = 1: Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicdl_SPEC;
    pub type Iicdl = crate::EnumBitfieldStruct<u8, Iicdl_SPEC>;
    impl Iicdl {
        #[doc = "No output delay"]
        pub const _0_X_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr2_SPEC;
impl crate::sealed::RegSpec for Simr2_SPEC {
    type DataType = u8;
}

#[doc = "IIC Mode Register 2"]
pub type Simr2 = crate::RegValueT<Simr2_SPEC>;

impl Simr2 {
    #[doc = "IIC Interrupt Mode Select"]
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
}
impl ::core::default::Default for Simr2 {
    #[inline(always)]
    fn default() -> Simr2 {
        <crate::RegValueT<Simr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod simr2 {

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
        #[doc = "Do not synchronize with clock signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronize with clock signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicackt_SPEC;
    pub type Iicackt = crate::EnumBitfieldStruct<u8, Iicackt_SPEC>;
    impl Iicackt {
        #[doc = "ACK transmission"]
        pub const _0: Self = Self::new(0);

        #[doc = "NACK transmission and ACK/NACK reception"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr3_SPEC;
impl crate::sealed::RegSpec for Simr3_SPEC {
    type DataType = u8;
}

#[doc = "IIC Mode Register 3"]
pub type Simr3 = crate::RegValueT<Simr3_SPEC>;

impl Simr3 {
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

    #[doc = "Issuing of Start, Restart, or Stop Condition Completed Flag"]
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

    #[doc = "SDAn Output Select"]
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

    #[doc = "SCLn Output Select"]
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
}
impl ::core::default::Default for Simr3 {
    #[inline(always)]
    fn default() -> Simr3 {
        <crate::RegValueT<Simr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod simr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstareq_SPEC;
    pub type Iicstareq = crate::EnumBitfieldStruct<u8, Iicstareq_SPEC>;
    impl Iicstareq {
        #[doc = "Do not generate start condition"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generate start condition"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicrstareq_SPEC;
    pub type Iicrstareq = crate::EnumBitfieldStruct<u8, Iicrstareq_SPEC>;
    impl Iicrstareq {
        #[doc = "Do not generate restart condition"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generate restart condition"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstpreq_SPEC;
    pub type Iicstpreq = crate::EnumBitfieldStruct<u8, Iicstpreq_SPEC>;
    impl Iicstpreq {
        #[doc = "Do not generate stop condition"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generate stop condition"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstif_SPEC;
    pub type Iicstif = crate::EnumBitfieldStruct<u8, Iicstif_SPEC>;
    impl Iicstif {
        #[doc = "No requests are being made for generating conditions, or a condition is being generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generation of start, restart, or stop condition is complete. When 0 is written to IICSTIF, it is set to 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicsdas_SPEC;
    pub type Iicsdas = crate::EnumBitfieldStruct<u8, Iicsdas_SPEC>;
    impl Iicsdas {
        #[doc = "Output serial data"]
        pub const _00: Self = Self::new(0);

        #[doc = "Generate start, restart, or stop condition"]
        pub const _01: Self = Self::new(1);

        #[doc = "Output low on SDAn pin"]
        pub const _10: Self = Self::new(2);

        #[doc = "Drive SDAn pin to high-impedance state"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicscls_SPEC;
    pub type Iicscls = crate::EnumBitfieldStruct<u8, Iicscls_SPEC>;
    impl Iicscls {
        #[doc = "Output serial clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "Generate start, restart, or stop condition"]
        pub const _01: Self = Self::new(1);

        #[doc = "Output low on SCLn pin"]
        pub const _10: Self = Self::new(2);

        #[doc = "Drive SCLn pin to high-impedance state"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sisr_SPEC;
impl crate::sealed::RegSpec for Sisr_SPEC {
    type DataType = u8;
}

#[doc = "IIC Status Register"]
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
    #[doc = "SSn Pin Function Enable"]
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

    #[doc = "Master Slave Select"]
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

    #[doc = "CTS external pin Enable"]
    #[inline(always)]
    pub fn ctspen(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        spmr::Ctspen,
        spmr::Ctspen,
        Spmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            spmr::Ctspen,
            spmr::Ctspen,
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
}
impl ::core::default::Default for Spmr {
    #[inline(always)]
    fn default() -> Spmr {
        <crate::RegValueT<Spmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sse_SPEC;
    pub type Sse = crate::EnumBitfieldStruct<u8, Sse_SPEC>;
    impl Sse {
        #[doc = "Disable SSn pin function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable SSn pin function"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctse_SPEC;
    pub type Ctse = crate::EnumBitfieldStruct<u8, Ctse_SPEC>;
    impl Ctse {
        #[doc = "Disable CTS function (enable RTS output function)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable CTS function"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mss_SPEC;
    pub type Mss = crate::EnumBitfieldStruct<u8, Mss_SPEC>;
    impl Mss {
        #[doc = "Transmit through TXDn pin and receive through RXDn pin (master mode)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Receive through TXDn pin and transmit through RXDn pin (slave mode)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctspen_SPEC;
    pub type Ctspen = crate::EnumBitfieldStruct<u8, Ctspen_SPEC>;
    impl Ctspen {
        #[doc = "Alternate setting to use CTS and RTS functions as either one terminal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Dedicated setting for separately using CTS and RTS functions with 2 terminals These bits for the other SCI channels than SCIn (n = 0, 3 to 9) are reserved."]
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
    pub struct Ckpol_SPEC;
    pub type Ckpol = crate::EnumBitfieldStruct<u8, Ckpol_SPEC>;
    impl Ckpol {
        #[doc = "Do not invert clock polarity"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invert clock polarity"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckph_SPEC;
    pub type Ckph = crate::EnumBitfieldStruct<u8, Ckph_SPEC>;
    impl Ckph {
        #[doc = "Do not delay clock"]
        pub const _0: Self = Self::new(0);

        #[doc = "Delay clock"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftdrhl_SPEC;
impl crate::sealed::RegSpec for Ftdrhl_SPEC {
    type DataType = u16;
}

#[doc = "Transmit FIFO Data Register"]
pub type Ftdrhl = crate::RegValueT<Ftdrhl_SPEC>;

impl Ftdrhl {
    #[doc = "Serial transmit data"]
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Ftdrhl_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Ftdrhl_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Multi-Processor Transfer Bit Flag"]
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
        #[doc = "Data transmission cycle"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycle"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdrhl_SPEC;
impl crate::sealed::RegSpec for Tdrhl_SPEC {
    type DataType = u16;
}

#[doc = "Transmit Data Register for Non-Manchester mode (MMR.MANEN = 0)"]
pub type Tdrhl = crate::RegValueT<Tdrhl_SPEC>;

impl Tdrhl {
    #[doc = "Serial Transmit Data"]
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Tdrhl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Tdrhl_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct TdrhlMan_SPEC;
impl crate::sealed::RegSpec for TdrhlMan_SPEC {
    type DataType = u16;
}

#[doc = "Transmit Data Register for Manchester mode (MMR.MANEN = 1)"]
pub type TdrhlMan = crate::RegValueT<TdrhlMan_SPEC>;

impl TdrhlMan {
    #[doc = "Serial transmit data"]
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, TdrhlMan_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,TdrhlMan_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Multi-processor transfer bit flag"]
    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        tdrhl_man::Mpbt,
        tdrhl_man::Mpbt,
        TdrhlMan_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            tdrhl_man::Mpbt,
            tdrhl_man::Mpbt,
            TdrhlMan_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit SYNC data bit"]
    #[inline(always)]
    pub fn tsync(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        tdrhl_man::Tsync,
        tdrhl_man::Tsync,
        TdrhlMan_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            tdrhl_man::Tsync,
            tdrhl_man::Tsync,
            TdrhlMan_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TdrhlMan {
    #[inline(always)]
    fn default() -> TdrhlMan {
        <crate::RegValueT<TdrhlMan_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod tdrhl_man {

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
pub struct Ftdrh_SPEC;
impl crate::sealed::RegSpec for Ftdrh_SPEC {
    type DataType = u8;
}

#[doc = "Transmit FIFO Data Register"]
pub type Ftdrh = crate::RegValueT<Ftdrh_SPEC>;

impl Ftdrh {
    #[doc = "Multi-Processor Transfer Bit Flag"]
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
        #[doc = "Data transmission cycle"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycle"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftdrl_SPEC;
impl crate::sealed::RegSpec for Ftdrl_SPEC {
    type DataType = u8;
}

#[doc = "Transmit FIFO Data Register"]
pub type Ftdrl = crate::RegValueT<Ftdrl_SPEC>;

impl Ftdrl {
    #[doc = "Serial transmit data"]
    #[inline(always)]
    pub fn tdat(
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
pub struct Frdrhl_SPEC;
impl crate::sealed::RegSpec for Frdrhl_SPEC {
    type DataType = u16;
}

#[doc = "Receive FIFO Data Register"]
pub type Frdrhl = crate::RegValueT<Frdrhl_SPEC>;

impl Frdrhl {
    #[doc = "Serial receive data"]
    #[inline(always)]
    pub fn rdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Frdrhl_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Frdrhl_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Multi-Processor Bit Flag"]
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

    #[doc = "Receive Data Ready Flag"]
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

    #[doc = "Parity Error Flag"]
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

    #[doc = "Framing Error Flag"]
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

    #[doc = "Overrun Error Flag"]
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

    #[doc = "Receive FIFO Data Full Flag"]
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
}
impl ::core::default::Default for Frdrhl {
    #[inline(always)]
    fn default() -> Frdrhl {
        <crate::RegValueT<Frdrhl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod frdrhl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpb_SPEC;
    pub type Mpb = crate::EnumBitfieldStruct<u8, Mpb_SPEC>;
    impl Mpb {
        #[doc = "Data transmission cycle"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycle"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
        #[doc = "Receiving is in progress, or no received data remains in the FRDRH and FRDRL registers after successfully completed reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Next receive data is not received for a period after successfully completed reception"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "No parity error occurred in the first data of FRDRH and FRDRL"]
        pub const _0: Self = Self::new(0);

        #[doc = "Parity error occurred in the first data of FRDRH and FRDRL"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        #[doc = "No framing error occurred in the first data of FRDRH and FRDRL"]
        pub const _0: Self = Self::new(0);

        #[doc = "Framing error occurred in the first data of FRDRH and FRDRL"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        #[doc = "No overrun error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Overrun error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdf_SPEC;
    pub type Rdf = crate::EnumBitfieldStruct<u8, Rdf_SPEC>;
    impl Rdf {
        #[doc = "The amount of receive data written in FRDRH and FRDRL is less than the specified receive triggering number"]
        pub const _0: Self = Self::new(0);

        #[doc = "The amount of receive data written in FRDRH and FRDRL is equal to or greater than the specified receive triggering number"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdrhl_SPEC;
impl crate::sealed::RegSpec for Rdrhl_SPEC {
    type DataType = u16;
}

#[doc = "Receive Data Register for Non-Manchester mode (MMR.MANEN = 0)"]
pub type Rdrhl = crate::RegValueT<Rdrhl_SPEC>;

impl Rdrhl {
    #[doc = "Serial Receive Data"]
    #[inline(always)]
    pub fn rdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Rdrhl_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Rdrhl_SPEC,crate::common::R>::from_register(self,0)
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
pub struct RdrhlMan_SPEC;
impl crate::sealed::RegSpec for RdrhlMan_SPEC {
    type DataType = u16;
}

#[doc = "Receive Data Register for Manchester mode (MMR.MANEN = 1)"]
pub type RdrhlMan = crate::RegValueT<RdrhlMan_SPEC>;

impl RdrhlMan {
    #[doc = "Serial receive data"]
    #[inline(always)]
    pub fn rdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, RdrhlMan_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,RdrhlMan_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Multi-processor bit"]
    #[inline(always)]
    pub fn mpb(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        rdrhl_man::Mpb,
        rdrhl_man::Mpb,
        RdrhlMan_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            rdrhl_man::Mpb,
            rdrhl_man::Mpb,
            RdrhlMan_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive SYNC data bit"]
    #[inline(always)]
    pub fn rsync(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        rdrhl_man::Rsync,
        rdrhl_man::Rsync,
        RdrhlMan_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            rdrhl_man::Rsync,
            rdrhl_man::Rsync,
            RdrhlMan_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RdrhlMan {
    #[inline(always)]
    fn default() -> RdrhlMan {
        <crate::RegValueT<RdrhlMan_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rdrhl_man {

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
    pub struct Rsync_SPEC;
    pub type Rsync = crate::EnumBitfieldStruct<u8, Rsync_SPEC>;
    impl Rsync {
        #[doc = "The received the Start Bit is DATA SYNC"]
        pub const _0: Self = Self::new(0);

        #[doc = "The received the Start Bit is COMMAND SYNC"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frdrh_SPEC;
impl crate::sealed::RegSpec for Frdrh_SPEC {
    type DataType = u8;
}

#[doc = "Receive FIFO Data Register"]
pub type Frdrh = crate::RegValueT<Frdrh_SPEC>;

impl Frdrh {
    #[doc = "Multi-Processor Bit Flag"]
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

    #[doc = "Receive Data Ready Flag"]
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

    #[doc = "Parity Error Flag"]
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

    #[doc = "Framing Error Flag"]
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

    #[doc = "Overrun Error Flag"]
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

    #[doc = "Receive FIFO Data Full Flag"]
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
}
impl ::core::default::Default for Frdrh {
    #[inline(always)]
    fn default() -> Frdrh {
        <crate::RegValueT<Frdrh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod frdrh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpb_SPEC;
    pub type Mpb = crate::EnumBitfieldStruct<u8, Mpb_SPEC>;
    impl Mpb {
        #[doc = "Data transmission cycle"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycle"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
        #[doc = "Receiving is in progress, or no received data remains in the FRDRH and FRDRL registers after successfully completed reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Next receive data is not received for a period after successfully completed reception"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "No parity error occurred in the first data of FRDRH and FRDRL"]
        pub const _0: Self = Self::new(0);

        #[doc = "Parity error occurred in the first data of FRDRH and FRDRL"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        #[doc = "No framing error occurred in the first data of FRDRH and FRDRL"]
        pub const _0: Self = Self::new(0);

        #[doc = "Framing error occurred in the first data of FRDRH and FRDRL"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        #[doc = "No overrun error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Overrun error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdf_SPEC;
    pub type Rdf = crate::EnumBitfieldStruct<u8, Rdf_SPEC>;
    impl Rdf {
        #[doc = "The amount of receive data written in FRDRH and FRDRL is less than the specified receive triggering number"]
        pub const _0: Self = Self::new(0);

        #[doc = "The amount of receive data written in FRDRH and FRDRL is equal to or greater than the specified receive triggering number"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frdrl_SPEC;
impl crate::sealed::RegSpec for Frdrl_SPEC {
    type DataType = u8;
}

#[doc = "Receive FIFO Data Register"]
pub type Frdrl = crate::RegValueT<Frdrl_SPEC>;

impl Frdrl {
    #[doc = "Serial receive data"]
    #[inline(always)]
    pub fn rdat(
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

impl NoBitfieldReg<Mddr_SPEC> for Mddr {}
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

    #[doc = "ID Frame Select"]
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

    #[doc = "Data Compare Match Enable"]
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
}
impl ::core::default::Default for Dccr {
    #[inline(always)]
    fn default() -> Dccr {
        <crate::RegValueT<Dccr_SPEC> as RegisterValue<_>>::new(64)
    }
}
pub mod dccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcmf_SPEC;
    pub type Dcmf = crate::EnumBitfieldStruct<u8, Dcmf_SPEC>;
    impl Dcmf {
        #[doc = "Not matched"]
        pub const _0: Self = Self::new(0);

        #[doc = "Matched"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dper_SPEC;
    pub type Dper = crate::EnumBitfieldStruct<u8, Dper_SPEC>;
    impl Dper {
        #[doc = "No parity error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Parity error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfer_SPEC;
    pub type Dfer = crate::EnumBitfieldStruct<u8, Dfer_SPEC>;
    impl Dfer {
        #[doc = "No framing error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Framing error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idsel_SPEC;
    pub type Idsel = crate::EnumBitfieldStruct<u8, Idsel_SPEC>;
    impl Idsel {
        #[doc = "Always compare data regardless of the MPB bit value"]
        pub const _0: Self = Self::new(0);

        #[doc = "Only compare data when MPB bit = 1 (ID frame)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcme_SPEC;
    pub type Dcme = crate::EnumBitfieldStruct<u8, Dcme_SPEC>;
    impl Dcme {
        #[doc = "Disable address match function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable address match function"]
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
    #[doc = "FIFO Mode Select"]
    #[inline(always)]
    pub fn fm(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, fcr::Fm, fcr::Fm, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,fcr::Fm,fcr::Fm,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive FIFO Data Register Reset"]
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

    #[doc = "Transmit FIFO Data Register Reset"]
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

    #[doc = "Receive Data Ready Error Select"]
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

    #[doc = "Transmit FIFO Data Trigger Number"]
    #[inline(always)]
    pub fn ttrg(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive FIFO Data Trigger Number"]
    #[inline(always)]
    pub fn rtrg(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "RTS Output Active Trigger Number Select"]
    #[inline(always)]
    pub fn rstrg(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Fm_SPEC;
    pub type Fm = crate::EnumBitfieldStruct<u8, Fm_SPEC>;
    impl Fm {
        #[doc = "Non-FIFO mode. Selects TDR/RDR or TDRHL/RDRHL for communication."]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO mode. Selects FTDRHL/FRDRHL for communication."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfrst_SPEC;
    pub type Rfrst = crate::EnumBitfieldStruct<u8, Rfrst_SPEC>;
    impl Rfrst {
        #[doc = "Do not reset FRDRHL"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset FRDRHL"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfrst_SPEC;
    pub type Tfrst = crate::EnumBitfieldStruct<u8, Tfrst_SPEC>;
    impl Tfrst {
        #[doc = "Do not reset FTDRHL"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset FTDRHL"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dres_SPEC;
    pub type Dres = crate::EnumBitfieldStruct<u8, Dres_SPEC>;
    impl Dres {
        #[doc = "Receive data full interrupt (SCIn_RXI)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Receive error interrupt (SCIn_ERI)"]
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
    #[doc = "Receive FIFO Data Count"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Fdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Fdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmit FIFO Data Count"]
    #[inline(always)]
    pub fn t(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Fdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Fdr_SPEC,crate::common::R>::from_register(self,0)
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
    #[doc = "Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, lsr::Orer, lsr::Orer, Lsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,lsr::Orer,lsr::Orer,Lsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Framing Error Count"]
    #[inline(always)]
    pub fn fnum(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, Lsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,Lsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Parity Error Count"]
    #[inline(always)]
    pub fn pnum(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Lsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Lsr_SPEC,crate::common::R>::from_register(self,0)
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

        #[doc = "Overrun error occurred"]
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
    #[doc = "Compare Match Data"]
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
    #[doc = "Serial Input Data Monitor"]
    #[inline(always)]
    pub fn rxdmon(self) -> crate::common::RegisterFieldBool<0, 1, 0, Sptr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sptr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Port Break Data Select"]
    #[inline(always)]
    pub fn spb2dt(self) -> crate::common::RegisterFieldBool<1, 1, 0, Sptr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Sptr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Port Break I/O"]
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

    #[doc = "RXD invert bit"]
    #[inline(always)]
    pub fn rinv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sptr::Rinv,
        sptr::Rinv,
        Sptr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sptr::Rinv,
            sptr::Rinv,
            Sptr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TXD invert bit"]
    #[inline(always)]
    pub fn tinv(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sptr::Tinv,
        sptr::Tinv,
        Sptr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sptr::Tinv,
            sptr::Tinv,
            Sptr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjust receive sampling timing enable"]
    #[inline(always)]
    pub fn asen(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sptr::Asen,
        sptr::Asen,
        Sptr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sptr::Asen,
            sptr::Asen,
            Sptr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjust transmit timing enable"]
    #[inline(always)]
    pub fn aten(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sptr::Aten,
        sptr::Aten,
        Sptr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sptr::Aten,
            sptr::Aten,
            Sptr_SPEC,
            crate::common::RW,
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
        #[doc = "Do not output value of SPB2DT bit on TXDn pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output value of SPB2DT bit on TXDn pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rinv_SPEC;
    pub type Rinv = crate::EnumBitfieldStruct<u8, Rinv_SPEC>;
    impl Rinv {
        #[doc = "Received data from RXDn is not inverted and input."]
        pub const _0: Self = Self::new(0);

        #[doc = "Received data from RXDn is inverted and input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tinv_SPEC;
    pub type Tinv = crate::EnumBitfieldStruct<u8, Tinv_SPEC>;
    impl Tinv {
        #[doc = "Transmit data is not inverted and output to TXDn."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit data is inverted and output to TXDn."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asen_SPEC;
    pub type Asen = crate::EnumBitfieldStruct<u8, Asen_SPEC>;
    impl Asen {
        #[doc = "Adjust sampling timing disable."]
        pub const _0: Self = Self::new(0);

        #[doc = "Adjust sampling timing enable."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aten_SPEC;
    pub type Aten = crate::EnumBitfieldStruct<u8, Aten_SPEC>;
    impl Aten {
        #[doc = "Adjust transmit timing disable."]
        pub const _0: Self = Self::new(0);

        #[doc = "Adjust transmit timing enable."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Actr_SPEC;
impl crate::sealed::RegSpec for Actr_SPEC {
    type DataType = u8;
}

#[doc = "Adjustment Communication Timing Register"]
pub type Actr = crate::RegValueT<Actr_SPEC>;

impl Actr {
    #[doc = "Adjustment value for receive Sampling Timing"]
    #[inline(always)]
    pub fn ast(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Actr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Actr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Adjustment Direction for receive sampling timing"]
    #[inline(always)]
    pub fn ajd(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        actr::Ajd,
        actr::Ajd,
        Actr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            actr::Ajd,
            actr::Ajd,
            Actr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustment value for Transmit timing"]
    #[inline(always)]
    pub fn att(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Actr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Actr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Adjustment edge for transmit timing"]
    #[inline(always)]
    pub fn aet(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        actr::Aet,
        actr::Aet,
        Actr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            actr::Aet,
            actr::Aet,
            Actr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Actr {
    #[inline(always)]
    fn default() -> Actr {
        <crate::RegValueT<Actr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod actr {

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
        #[doc = "Adjust the rising edge timing."]
        pub const _0: Self = Self::new(0);

        #[doc = "Adjust the falling edge timing."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmr_SPEC;
impl crate::sealed::RegSpec for Mmr_SPEC {
    type DataType = u8;
}

#[doc = "Manchester Mode Register"]
pub type Mmr = crate::RegValueT<Mmr_SPEC>;

impl Mmr {
    #[doc = "Polarity of Received Manchester Code"]
    #[inline(always)]
    pub fn rmpol(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmr::Rmpol,
        mmr::Rmpol,
        Mmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmr::Rmpol,
            mmr::Rmpol,
            Mmr_SPEC,
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
        mmr::Tmpol,
        mmr::Tmpol,
        Mmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmr::Tmpol,
            mmr::Tmpol,
            Mmr_SPEC,
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
        mmr::Erten,
        mmr::Erten,
        Mmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmr::Erten,
            mmr::Erten,
            Mmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYNC value Setting"]
    #[inline(always)]
    pub fn synval(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mmr::Synval,
        mmr::Synval,
        Mmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mmr::Synval,
            mmr::Synval,
            Mmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
        mmr::Synsel,
        mmr::Synsel,
        Mmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mmr::Synsel,
            mmr::Synsel,
            Mmr_SPEC,
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
        mmr::Sbsel,
        mmr::Sbsel,
        Mmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mmr::Sbsel,
            mmr::Sbsel,
            Mmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Manchester Mode Enable"]
    #[inline(always)]
    pub fn manen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mmr::Manen,
        mmr::Manen,
        Mmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mmr::Manen,
            mmr::Manen,
            Mmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmr {
    #[inline(always)]
    fn default() -> Mmr {
        <crate::RegValueT<Mmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmr {

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
    pub struct Synval_SPEC;
    pub type Synval = crate::EnumBitfieldStruct<u8, Synval_SPEC>;
    impl Synval {
        #[doc = "The start bit is added as a zero-to-one transition."]
        pub const _0: Self = Self::new(0);

        #[doc = "The start bit is added as a one-to-zero transition."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Synsel_SPEC;
    pub type Synsel = crate::EnumBitfieldStruct<u8, Synsel_SPEC>;
    impl Synsel {
        #[doc = "The start bit pattern is set with the SYNVAL bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "The start bit pattern is set with the TSYNC bit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbsel_SPEC;
    pub type Sbsel = crate::EnumBitfieldStruct<u8, Sbsel_SPEC>;
    impl Sbsel {
        #[doc = "The start bit area consists of one bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "The start bit area consists of three bits (COMMAND SYNC or DATA SYNC)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Manen_SPEC;
    pub type Manen = crate::EnumBitfieldStruct<u8, Manen_SPEC>;
    impl Manen {
        #[doc = "Disables the Manchester mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables the Manchester mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmpr_SPEC;
impl crate::sealed::RegSpec for Tmpr_SPEC {
    type DataType = u8;
}

#[doc = "Transmit Manchester Preface Setting Register"]
pub type Tmpr = crate::RegValueT<Tmpr_SPEC>;

impl Tmpr {
    #[doc = "Transmit preface length"]
    #[inline(always)]
    pub fn tplen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        tmpr::Tplen,
        tmpr::Tplen,
        Tmpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            tmpr::Tplen,
            tmpr::Tplen,
            Tmpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit preface pattern"]
    #[inline(always)]
    pub fn tppat(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        tmpr::Tppat,
        tmpr::Tppat,
        Tmpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            tmpr::Tppat,
            tmpr::Tppat,
            Tmpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tmpr {
    #[inline(always)]
    fn default() -> Tmpr {
        <crate::RegValueT<Tmpr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tmpr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tplen_SPEC;
    pub type Tplen = crate::EnumBitfieldStruct<u8, Tplen_SPEC>;
    impl Tplen {
        #[doc = "Disables the transmit preface generation"]
        pub const _0_X_0: Self = Self::new(0);
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmpr_SPEC;
impl crate::sealed::RegSpec for Rmpr_SPEC {
    type DataType = u8;
}

#[doc = "Receive Manchester Preface Setting Register"]
pub type Rmpr = crate::RegValueT<Rmpr_SPEC>;

impl Rmpr {
    #[doc = "Receive Preface Length"]
    #[inline(always)]
    pub fn rplen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        rmpr::Rplen,
        rmpr::Rplen,
        Rmpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            rmpr::Rplen,
            rmpr::Rplen,
            Rmpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Preface Pattern"]
    #[inline(always)]
    pub fn rppat(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        rmpr::Rppat,
        rmpr::Rppat,
        Rmpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            rmpr::Rppat,
            rmpr::Rppat,
            Rmpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rmpr {
    #[inline(always)]
    fn default() -> Rmpr {
        <crate::RegValueT<Rmpr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rmpr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rplen_SPEC;
    pub type Rplen = crate::EnumBitfieldStruct<u8, Rplen_SPEC>;
    impl Rplen {
        #[doc = "Disables the receive preface generation"]
        pub const _0: Self = Self::new(0);
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mesr_SPEC;
impl crate::sealed::RegSpec for Mesr_SPEC {
    type DataType = u8;
}

#[doc = "Manchester Extended Error Status Register"]
pub type Mesr = crate::RegValueT<Mesr_SPEC>;

impl Mesr {
    #[doc = "Preface Error flag"]
    #[inline(always)]
    pub fn pfer(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mesr::Pfer,
        mesr::Pfer,
        Mesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mesr::Pfer,
            mesr::Pfer,
            Mesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYNC Error flag"]
    #[inline(always)]
    pub fn syer(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mesr::Syer,
        mesr::Syer,
        Mesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mesr::Syer,
            mesr::Syer,
            Mesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start Bit Error flag"]
    #[inline(always)]
    pub fn sber(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mesr::Sber,
        mesr::Sber,
        Mesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mesr::Sber,
            mesr::Sber,
            Mesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mesr {
    #[inline(always)]
    fn default() -> Mesr {
        <crate::RegValueT<Mesr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mesr {

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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mecr_SPEC;
impl crate::sealed::RegSpec for Mecr_SPEC {
    type DataType = u8;
}

#[doc = "Manchester Extended Error Control Register"]
pub type Mecr = crate::RegValueT<Mecr_SPEC>;

impl Mecr {
    #[doc = "Preface Error Enable"]
    #[inline(always)]
    pub fn pferen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mecr::Pferen,
        mecr::Pferen,
        Mecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mecr::Pferen,
            mecr::Pferen,
            Mecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive SYNC Error Enable"]
    #[inline(always)]
    pub fn syeren(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mecr::Syeren,
        mecr::Syeren,
        Mecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mecr::Syeren,
            mecr::Syeren,
            Mecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start Bit Error Enable"]
    #[inline(always)]
    pub fn sberen(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mecr::Sberen,
        mecr::Sberen,
        Mecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mecr::Sberen,
            mecr::Sberen,
            Mecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mecr {
    #[inline(always)]
    fn default() -> Mecr {
        <crate::RegValueT<Mecr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mecr {

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
