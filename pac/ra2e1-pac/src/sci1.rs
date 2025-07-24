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
// Generated from SVD 1.51.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:46:27 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Serial Communication Interface 0"]
unsafe impl ::core::marker::Send for super::Sci1 {}
unsafe impl ::core::marker::Sync for super::Sci1 {}
impl super::Sci1 {
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

    #[doc = "Serial Status Register for Non-Smart Card Interface and Non-FIFO Mode (SCMR.SMIF = 0, FCR.FM = 0)"]
    #[inline(always)]
    pub const fn ssr(&self) -> &'static crate::common::Reg<self::Ssr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Serial Status Register for Smart Card Interface Mode (SCMR.SMIF = 1)"]
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
    pub const fn rdr(&self) -> &'static crate::common::Reg<self::Rdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rdr_SPEC, crate::common::RW>::from_ptr(
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

    #[doc = "Transmit Data Register"]
    #[inline(always)]
    pub const fn tdrhl(&self) -> &'static crate::common::Reg<self::Tdrhl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdrhl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Receive Data Register"]
    #[inline(always)]
    pub const fn rdrhl(&self) -> &'static crate::common::Reg<self::Rdrhl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdrhl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
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

        #[doc = "In asynchronous mode, input a clock with a frequency 16 times the bit rate from the SCKn pin when the SEMR.ABCS bit is 0. Input a clock signal with a frequency eight times the bit rate when the SEMR.ABCS bit is 1. In clock synchronous mode, the SCKn pin functions as the clock input pin."]
        pub const OTHERS: Self = Self::new(0);
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

        #[doc = "When data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF, ORER, and FER in SSR to 1. When data with the multi-processor bit set to 1 is received, the MPIE bit is automatically set to 0, and normal reception is resumed."]
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

#[doc = "Serial Status Register for Non-Smart Card Interface and Non-FIFO Mode (SCMR.SMIF = 0, FCR.FM = 0)"]
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
pub struct SsrSmci_SPEC;
impl crate::sealed::RegSpec for SsrSmci_SPEC {
    type DataType = u8;
}

#[doc = "Serial Status Register for Smart Card Interface Mode (SCMR.SMIF = 1)"]
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

        #[doc = "Baud rate is 6 base clock cycles for 1-bit period"]
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

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
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

        #[doc = "(IICDL - 1) to (IICDL) cycles"]
        pub const OTHERS: Self = Self::new(0);
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
pub struct Tdrhl_SPEC;
impl crate::sealed::RegSpec for Tdrhl_SPEC {
    type DataType = u16;
}

#[doc = "Transmit Data Register"]
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
pub struct Rdrhl_SPEC;
impl crate::sealed::RegSpec for Rdrhl_SPEC {
    type DataType = u16;
}

#[doc = "Receive Data Register"]
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
}
