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
// Generated from SVD 1.20.02, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:53:12 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r""]
unsafe impl ::core::marker::Send for super::MipiDsiNs {}
unsafe impl ::core::marker::Sync for super::MipiDsiNs {}
impl super::MipiDsiNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &'static crate::common::Reg<self::Isr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Isr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Link Status Register"]
    #[inline(always)]
    pub const fn linksr(&self) -> &'static crate::common::Reg<self::Linksr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Linksr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Transmit Set Register"]
    #[inline(always)]
    pub const fn txsetr(
        &self,
    ) -> &'static crate::common::Reg<self::Txsetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Txsetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "HS Clock Set Register"]
    #[inline(always)]
    pub const fn hsclksetr(
        &self,
    ) -> &'static crate::common::Reg<self::Hsclksetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hsclksetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "ULPS Set Register"]
    #[inline(always)]
    pub const fn ulpssetr(
        &self,
    ) -> &'static crate::common::Reg<self::Ulpssetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ulpssetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "ULPS Control Register"]
    #[inline(always)]
    pub const fn ulpscr(&self) -> &'static crate::common::Reg<self::Ulpscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ulpscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[doc = "Reset Control Register"]
    #[inline(always)]
    pub const fn rstcr(&self) -> &'static crate::common::Reg<self::Rstcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "Reset Status Register"]
    #[inline(always)]
    pub const fn rstsr(&self) -> &'static crate::common::Reg<self::Rstsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rstsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(276usize),
            )
        }
    }

    #[doc = "DSI Set Register"]
    #[inline(always)]
    pub const fn dsisetr(
        &self,
    ) -> &'static crate::common::Reg<self::Dsisetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsisetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "Transmit Packet Payload Data 0 Register"]
    #[inline(always)]
    pub const fn txppd0r(
        &self,
    ) -> &'static crate::common::Reg<self::Txppd0R_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Txppd0R_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(352usize),
            )
        }
    }

    #[doc = "Transmit Packet Payload Data 1 Register"]
    #[inline(always)]
    pub const fn txppd1r(
        &self,
    ) -> &'static crate::common::Reg<self::Txppd1R_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Txppd1R_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(356usize),
            )
        }
    }

    #[doc = "Transmit Packet Payload Data 2 Register"]
    #[inline(always)]
    pub const fn txppd2r(
        &self,
    ) -> &'static crate::common::Reg<self::Txppd2R_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Txppd2R_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(360usize),
            )
        }
    }

    #[doc = "Transmit Packet Payload Data 3 Register"]
    #[inline(always)]
    pub const fn txppd3r(
        &self,
    ) -> &'static crate::common::Reg<self::Txppd3R_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Txppd3R_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(364usize),
            )
        }
    }

    #[doc = "Receive Status Register"]
    #[inline(always)]
    pub const fn rxsr(&self) -> &'static crate::common::Reg<self::Rxsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rxsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "Receive Status Clear Register"]
    #[inline(always)]
    pub const fn rxscr(&self) -> &'static crate::common::Reg<self::Rxscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Rxscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }

    #[doc = "Receive Interrupt Enable Register"]
    #[inline(always)]
    pub const fn rxier(&self) -> &'static crate::common::Reg<self::Rxier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rxier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(520usize),
            )
        }
    }

    #[doc = "Peripheral Response Timeout BTA Set Register"]
    #[inline(always)]
    pub const fn presptobtasetr(
        &self,
    ) -> &'static crate::common::Reg<self::Presptobtasetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Presptobtasetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(528usize),
            )
        }
    }

    #[doc = "Peripheral Response Timeout LP Set Register"]
    #[inline(always)]
    pub const fn presptolpsetr(
        &self,
    ) -> &'static crate::common::Reg<self::Presptolpsetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Presptolpsetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(532usize),
            )
        }
    }

    #[doc = "Peripheral Response Timeout HS Set Register"]
    #[inline(always)]
    pub const fn presptohssetr(
        &self,
    ) -> &'static crate::common::Reg<self::Presptohssetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Presptohssetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(536usize),
            )
        }
    }

    #[doc = "Acknowledge and Error Report Packet Parameter Latest Info Register"]
    #[inline(always)]
    pub const fn akeplatir(
        &self,
    ) -> &'static crate::common::Reg<self::Akeplatir_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Akeplatir_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(544usize),
            )
        }
    }

    #[doc = "Acknowledge and Error Report Packet Parameter Accumulate Status Register"]
    #[inline(always)]
    pub const fn akepacmsr(
        &self,
    ) -> &'static crate::common::Reg<self::Akepacmsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Akepacmsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(548usize),
            )
        }
    }

    #[doc = "Acknowledge and Error Report Packet Parameter Status Clear Register"]
    #[inline(always)]
    pub const fn akepscr(
        &self,
    ) -> &'static crate::common::Reg<self::Akepscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Akepscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(552usize),
            )
        }
    }

    #[doc = "Receive Result Saved Status Clear Register"]
    #[inline(always)]
    pub const fn rxrsscr(
        &self,
    ) -> &'static crate::common::Reg<self::Rxrsscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Rxrsscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(564usize),
            )
        }
    }

    #[doc = "Receive Result Info Overwrite Status Register"]
    #[inline(always)]
    pub const fn rxrinfoowsr(
        &self,
    ) -> &'static crate::common::Reg<self::Rxrinfoowsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rxrinfoowsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(568usize),
            )
        }
    }

    #[doc = "Receive Result Info Overwrite Status Clear Register"]
    #[inline(always)]
    pub const fn rxrinfoowscr(
        &self,
    ) -> &'static crate::common::Reg<self::Rxrinfoowscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Rxrinfoowscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(572usize),
            )
        }
    }

    #[doc = "Receive Result Save Slot-x Register (x = 0 to 3)"]
    #[inline(always)]
    pub const fn rxrssr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Rxrssr_SPEC, crate::common::R>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x240usize))
        }
    }
    #[inline(always)]
    pub const fn rxrss0r(
        &self,
    ) -> &'static crate::common::Reg<self::Rxrssr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rxrssr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x240usize),
            )
        }
    }
    #[inline(always)]
    pub const fn rxrss1r(
        &self,
    ) -> &'static crate::common::Reg<self::Rxrssr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rxrssr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x244usize),
            )
        }
    }
    #[inline(always)]
    pub const fn rxrss2r(
        &self,
    ) -> &'static crate::common::Reg<self::Rxrssr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rxrssr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x248usize),
            )
        }
    }
    #[inline(always)]
    pub const fn rxrss3r(
        &self,
    ) -> &'static crate::common::Reg<self::Rxrssr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rxrssr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x24cusize),
            )
        }
    }

    #[doc = "Receive Packet Payload Data 0 Register"]
    #[inline(always)]
    pub const fn rxppd0r(
        &self,
    ) -> &'static crate::common::Reg<self::Rxppd0R_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rxppd0R_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(704usize),
            )
        }
    }

    #[doc = "Receive Packet Payload Data 1 Register"]
    #[inline(always)]
    pub const fn rxppd1r(
        &self,
    ) -> &'static crate::common::Reg<self::Rxppd1R_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rxppd1R_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(708usize),
            )
        }
    }

    #[doc = "Receive Packet Payload Data 2 Register"]
    #[inline(always)]
    pub const fn rxppd2r(
        &self,
    ) -> &'static crate::common::Reg<self::Rxppd2R_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rxppd2R_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(712usize),
            )
        }
    }

    #[doc = "Receive Packet Payload Data 3 Register"]
    #[inline(always)]
    pub const fn rxppd3r(
        &self,
    ) -> &'static crate::common::Reg<self::Rxppd3R_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rxppd3R_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(716usize),
            )
        }
    }

    #[doc = "HS TX Timeout Set Register"]
    #[inline(always)]
    pub const fn hstxtosetr(
        &self,
    ) -> &'static crate::common::Reg<self::Hstxtosetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hstxtosetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(736usize),
            )
        }
    }

    #[doc = "LRX-H Timeout Set Register"]
    #[inline(always)]
    pub const fn lrxhtosetr(
        &self,
    ) -> &'static crate::common::Reg<self::Lrxhtosetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lrxhtosetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(740usize),
            )
        }
    }

    #[doc = "TA Timeout Set Register"]
    #[inline(always)]
    pub const fn tatosetr(
        &self,
    ) -> &'static crate::common::Reg<self::Tatosetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tatosetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(744usize),
            )
        }
    }

    #[doc = "Fatal Error Status Register"]
    #[inline(always)]
    pub const fn ferrsr(&self) -> &'static crate::common::Reg<self::Ferrsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ferrsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[doc = "Fatal Error Status Clear Register"]
    #[inline(always)]
    pub const fn ferrscr(
        &self,
    ) -> &'static crate::common::Reg<self::Ferrscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ferrscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(772usize),
            )
        }
    }

    #[doc = "Fatal Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ferrier(
        &self,
    ) -> &'static crate::common::Reg<self::Ferrier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ferrier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(776usize),
            )
        }
    }

    #[doc = "Clock Lane Stop Time Set Register"]
    #[inline(always)]
    pub const fn clstptsetr(
        &self,
    ) -> &'static crate::common::Reg<self::Clstptsetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Clstptsetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(788usize),
            )
        }
    }

    #[doc = "LP Transition Time Set Register"]
    #[inline(always)]
    pub const fn lptrnstsetr(
        &self,
    ) -> &'static crate::common::Reg<self::Lptrnstsetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lptrnstsetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(792usize),
            )
        }
    }

    #[doc = "Physical Lane Status Register"]
    #[inline(always)]
    pub const fn plsr(&self) -> &'static crate::common::Reg<self::Plsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Plsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(800usize),
            )
        }
    }

    #[doc = "Physical Lane Status Clear Register"]
    #[inline(always)]
    pub const fn plscr(&self) -> &'static crate::common::Reg<self::Plscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Plscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(804usize),
            )
        }
    }

    #[doc = "Physical Lane Interrupt Enable Register"]
    #[inline(always)]
    pub const fn plier(&self) -> &'static crate::common::Reg<self::Plier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Plier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(808usize),
            )
        }
    }

    #[doc = "Video Mode Set 0 Register"]
    #[inline(always)]
    pub const fn vmset0r(
        &self,
    ) -> &'static crate::common::Reg<self::Vmset0R_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vmset0R_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[doc = "Video Mode Set 1 Register"]
    #[inline(always)]
    pub const fn vmset1r(
        &self,
    ) -> &'static crate::common::Reg<self::Vmset1R_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vmset1R_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1028usize),
            )
        }
    }

    #[doc = "Video Mode Status Register"]
    #[inline(always)]
    pub const fn vmsr(&self) -> &'static crate::common::Reg<self::Vmsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vmsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1040usize),
            )
        }
    }

    #[doc = "Video Mode Status Clear Register"]
    #[inline(always)]
    pub const fn vmscr(&self) -> &'static crate::common::Reg<self::Vmscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vmscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(1044usize),
            )
        }
    }

    #[doc = "Video Mode Interrupt Enable Register"]
    #[inline(always)]
    pub const fn vmier(&self) -> &'static crate::common::Reg<self::Vmier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vmier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1048usize),
            )
        }
    }

    #[doc = "Video Mode Pixel Packet Set Register"]
    #[inline(always)]
    pub const fn vmppsetr(
        &self,
    ) -> &'static crate::common::Reg<self::Vmppsetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vmppsetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1056usize),
            )
        }
    }

    #[doc = "Video Mode Vertical Size Set Register"]
    #[inline(always)]
    pub const fn vmvssetr(
        &self,
    ) -> &'static crate::common::Reg<self::Vmvssetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vmvssetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1064usize),
            )
        }
    }

    #[doc = "Video Mode Vertical Porch Set Register"]
    #[inline(always)]
    pub const fn vmvpsetr(
        &self,
    ) -> &'static crate::common::Reg<self::Vmvpsetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vmvpsetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1068usize),
            )
        }
    }

    #[doc = "Video Mode Horizontal Size Set Register"]
    #[inline(always)]
    pub const fn vmhssetr(
        &self,
    ) -> &'static crate::common::Reg<self::Vmhssetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vmhssetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1072usize),
            )
        }
    }

    #[doc = "Video Mode Horizontal Porch Set Register"]
    #[inline(always)]
    pub const fn vmhpsetr(
        &self,
    ) -> &'static crate::common::Reg<self::Vmhpsetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vmhpsetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1076usize),
            )
        }
    }

    #[doc = "Sequence Channel 0 Set 0 Register"]
    #[inline(always)]
    pub const fn sqch0set0r(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Set0R_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Sqch0Set0R_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(1472usize),
            )
        }
    }

    #[doc = "Sequence Channel 0 Status Register"]
    #[inline(always)]
    pub const fn sqch0sr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Sr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sqch0Sr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1488usize),
            )
        }
    }

    #[doc = "Sequence Channel 0 Status Clear Register"]
    #[inline(always)]
    pub const fn sqch0scr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Scr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Sqch0Scr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(1492usize),
            )
        }
    }

    #[doc = "Sequence Channel 0 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sqch0ier(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Ier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Ier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1496usize),
            )
        }
    }

    #[doc = "Sequence Channel 1 Set 0 Register"]
    #[inline(always)]
    pub const fn sqch1set0r(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Set0R_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Sqch1Set0R_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(1536usize),
            )
        }
    }

    #[doc = "Sequence Channel 1 Status Register"]
    #[inline(always)]
    pub const fn sqch1sr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Sr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sqch1Sr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1552usize),
            )
        }
    }

    #[doc = "Sequence Channel 1 Status Clear Register"]
    #[inline(always)]
    pub const fn sqch1scr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Scr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Sqch1Scr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(1556usize),
            )
        }
    }

    #[doc = "Sequence Channel 1 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sqch1ier(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Ier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Ier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1560usize),
            )
        }
    }

    #[doc = "Sequence Channel 0 Descriptor-m A Register (m = 0 to 7)"]
    #[inline(always)]
    pub const fn sqch0dscar(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sqch0Dscar_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x780usize))
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc0ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x780usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc1ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x790usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc2ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc3ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc4ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc5ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc6ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc7ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7f0usize),
            )
        }
    }

    #[doc = "Sequence Channel 0 Descriptor-m B Register (m = 0 to 7)"]
    #[inline(always)]
    pub const fn sqch0dscbr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sqch0Dscbr_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x784usize))
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc0br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x784usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc1br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x794usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc2br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc3br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc4br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc5br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc6br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc7br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7f4usize),
            )
        }
    }

    #[doc = "Sequence Channel 0 Descriptor-m C Register"]
    #[inline(always)]
    pub const fn sqch0dsccr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sqch0Dsccr_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x788usize))
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc0cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x788usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc1cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x798usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc2cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc3cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc4cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc5cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc6cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc7cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7f8usize),
            )
        }
    }

    #[doc = "Sequence Channel 0 Descriptor-m D Register (m = 0 to 7)"]
    #[inline(always)]
    pub const fn sqch0dscdr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sqch0Dscdr_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x78cusize))
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc0dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc1dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x79cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc2dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc3dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc4dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc5dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc6dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch0dsc7dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch0Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch0Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7fcusize),
            )
        }
    }

    #[doc = "Sequence Channel 1 Descriptor-m A Register (m = 0 to 7)"]
    #[inline(always)]
    pub const fn sqch1dscar(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sqch1Dscar_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x800usize))
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc0ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x800usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc1ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x810usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc2ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x820usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc3ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x830usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc4ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x840usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc5ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x850usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc6ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x860usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc7ar(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x870usize),
            )
        }
    }

    #[doc = "Sequence Channel 1 Descriptor-m B Register (m = 0 to 7)"]
    #[inline(always)]
    pub const fn sqch1dscbr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sqch1Dscbr_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x804usize))
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc0br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x804usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc1br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x814usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc2br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x824usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc3br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x834usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc4br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x844usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc5br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x854usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc6br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x864usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc7br(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x874usize),
            )
        }
    }

    #[doc = "Sequence Channel 1 Descriptor-m C Register"]
    #[inline(always)]
    pub const fn sqch1dsccr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sqch1Dsccr_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x808usize))
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc0cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x808usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc1cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x818usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc2cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x828usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc3cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x838usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc4cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x848usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc5cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x858usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc6cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x868usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc7cr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dsccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dsccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x878usize),
            )
        }
    }

    #[doc = "Sequence Channel 1 Descriptor-m D Register (m = 0 to 7)"]
    #[inline(always)]
    pub const fn sqch1dscdr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sqch1Dscdr_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x80cusize))
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc0dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc1dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x81cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc2dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x82cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc3dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x83cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc4dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x84cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc5dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x85cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc6dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x86cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sqch1dsc7dr(
        &self,
    ) -> &'static crate::common::Reg<self::Sqch1Dscdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sqch1Dscdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x87cusize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr_SPEC;
impl crate::sealed::RegSpec for Isr_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Status Register"]
pub type Isr = crate::RegValueT<Isr_SPEC>;

impl Isr {
    #[doc = "Sequence Channel-0 Interrupt Flag"]
    #[inline(always)]
    pub fn sq0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, isr::Sq0, isr::Sq0, Isr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,isr::Sq0,isr::Sq0,Isr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Sequence Channel-1 Interrupt Flag"]
    #[inline(always)]
    pub fn sq1(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, isr::Sq1, isr::Sq1, Isr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,isr::Sq1,isr::Sq1,Isr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Video Mode Interrupt Flag"]
    #[inline(always)]
    pub fn vm(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, isr::Vm, isr::Vm, Isr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,isr::Vm,isr::Vm,Isr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Interrupt Flag"]
    #[inline(always)]
    pub fn rcv(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, isr::Rcv, isr::Rcv, Isr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0x1,1,0,isr::Rcv,isr::Rcv,Isr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Fatal Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, isr::Ferr, isr::Ferr, Isr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            isr::Ferr,
            isr::Ferr,
            Isr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PPI Interrupt Flag"]
    #[inline(always)]
    pub fn ppi(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, isr::Ppi, isr::Ppi, Isr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<20,0x1,1,0,isr::Ppi,isr::Ppi,Isr_SPEC,crate::common::R>::from_register(self,0)
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
    pub struct Sq0_SPEC;
    pub type Sq0 = crate::EnumBitfieldStruct<u8, Sq0_SPEC>;
    impl Sq0 {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sequence Operation Channel-0 interrupt detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sq1_SPEC;
    pub type Sq1 = crate::EnumBitfieldStruct<u8, Sq1_SPEC>;
    impl Sq1 {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sequence Operation Channel-1 interrupt detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vm_SPEC;
    pub type Vm = crate::EnumBitfieldStruct<u8, Vm_SPEC>;
    impl Vm {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Video Mode interrupt detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcv_SPEC;
    pub type Rcv = crate::EnumBitfieldStruct<u8, Rcv_SPEC>;
    impl Rcv {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Receive interrupt detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ferr_SPEC;
    pub type Ferr = crate::EnumBitfieldStruct<u8, Ferr_SPEC>;
    impl Ferr {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Fatal Error interrupt detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppi_SPEC;
    pub type Ppi = crate::EnumBitfieldStruct<u8, Ppi_SPEC>;
    impl Ppi {
        #[doc = "No interrupt detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "PPI interrupt detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Linksr_SPEC;
impl crate::sealed::RegSpec for Linksr_SPEC {
    type DataType = u32;
}

#[doc = "Link Status Register"]
pub type Linksr = crate::RegValueT<Linksr_SPEC>;

impl Linksr {
    #[doc = "Sequence Channel-0 Running Flag"]
    #[inline(always)]
    pub fn sq0run(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        linksr::Sq0Run,
        linksr::Sq0Run,
        Linksr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            linksr::Sq0Run,
            linksr::Sq0Run,
            Linksr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Sequence Channel-1 Running Flag"]
    #[inline(always)]
    pub fn sq1run(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        linksr::Sq1Run,
        linksr::Sq1Run,
        Linksr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            linksr::Sq1Run,
            linksr::Sq1Run,
            Linksr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Video Mode Operation Running Flag"]
    #[inline(always)]
    pub fn vrun(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        linksr::Vrun,
        linksr::Vrun,
        Linksr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            linksr::Vrun,
            linksr::Vrun,
            Linksr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "HS Operation Busy Flag"]
    #[inline(always)]
    pub fn hsbusy(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        linksr::Hsbusy,
        linksr::Hsbusy,
        Linksr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            linksr::Hsbusy,
            linksr::Hsbusy,
            Linksr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "LP Operation Busy Flag"]
    #[inline(always)]
    pub fn lpbusy(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        linksr::Lpbusy,
        linksr::Lpbusy,
        Linksr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            linksr::Lpbusy,
            linksr::Lpbusy,
            Linksr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Linksr {
    #[inline(always)]
    fn default() -> Linksr {
        <crate::RegValueT<Linksr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod linksr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sq0Run_SPEC;
    pub type Sq0Run = crate::EnumBitfieldStruct<u8, Sq0Run_SPEC>;
    impl Sq0Run {
        #[doc = "Sequence Operation Channel-0 stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sequence Operation Channel-0 in operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sq1Run_SPEC;
    pub type Sq1Run = crate::EnumBitfieldStruct<u8, Sq1Run_SPEC>;
    impl Sq1Run {
        #[doc = "Sequence Operation Channel-1 stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sequence Operation Channel-1 in operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vrun_SPEC;
    pub type Vrun = crate::EnumBitfieldStruct<u8, Vrun_SPEC>;
    impl Vrun {
        #[doc = "Video mode stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Video mode in operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsbusy_SPEC;
    pub type Hsbusy = crate::EnumBitfieldStruct<u8, Hsbusy_SPEC>;
    impl Hsbusy {
        #[doc = "HS mode stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "HS mode in operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lpbusy_SPEC;
    pub type Lpbusy = crate::EnumBitfieldStruct<u8, Lpbusy_SPEC>;
    impl Lpbusy {
        #[doc = "LP mode stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "LP mode in operation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txsetr_SPEC;
impl crate::sealed::RegSpec for Txsetr_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Set Register"]
pub type Txsetr = crate::RegValueT<Txsetr_SPEC>;

impl Txsetr {
    #[doc = "Number of Lane"]
    #[inline(always)]
    pub fn numlane(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        txsetr::Numlane,
        txsetr::Numlane,
        Txsetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            txsetr::Numlane,
            txsetr::Numlane,
            Txsetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Lane Enable"]
    #[inline(always)]
    pub fn clen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        txsetr::Clen,
        txsetr::Clen,
        Txsetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            txsetr::Clen,
            txsetr::Clen,
            Txsetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane Enable"]
    #[inline(always)]
    pub fn dlen(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        txsetr::Dlen,
        txsetr::Dlen,
        Txsetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            txsetr::Dlen,
            txsetr::Dlen,
            Txsetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Txsetr {
    #[inline(always)]
    fn default() -> Txsetr {
        <crate::RegValueT<Txsetr_SPEC> as RegisterValue<_>>::new(65537)
    }
}
pub mod txsetr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Numlane_SPEC;
    pub type Numlane = crate::EnumBitfieldStruct<u8, Numlane_SPEC>;
    impl Numlane {
        #[doc = "1 Lane (Use of Lane-0)"]
        pub const _00: Self = Self::new(0);

        #[doc = "2 Lane (Use of Lane-0 and Lane-1)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clen_SPEC;
    pub type Clen = crate::EnumBitfieldStruct<u8, Clen_SPEC>;
    impl Clen {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlen_SPEC;
    pub type Dlen = crate::EnumBitfieldStruct<u8, Dlen_SPEC>;
    impl Dlen {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsclksetr_SPEC;
impl crate::sealed::RegSpec for Hsclksetr_SPEC {
    type DataType = u32;
}

#[doc = "HS Clock Set Register"]
pub type Hsclksetr = crate::RegValueT<Hsclksetr_SPEC>;

impl Hsclksetr {
    #[doc = "HS Clock Start"]
    #[inline(always)]
    pub fn hsclst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        hsclksetr::Hsclst,
        hsclksetr::Hsclst,
        Hsclksetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            hsclksetr::Hsclst,
            hsclksetr::Hsclst,
            Hsclksetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HS Clock Running Mode"]
    #[inline(always)]
    pub fn hsclmd(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        hsclksetr::Hsclmd,
        hsclksetr::Hsclmd,
        Hsclksetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            hsclksetr::Hsclmd,
            hsclksetr::Hsclmd,
            Hsclksetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Hsclksetr {
    #[inline(always)]
    fn default() -> Hsclksetr {
        <crate::RegValueT<Hsclksetr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hsclksetr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsclst_SPEC;
    pub type Hsclst = crate::EnumBitfieldStruct<u8, Hsclst_SPEC>;
    impl Hsclst {
        #[doc = "Stop HS transmission (keep LP state)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start HS transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsclmd_SPEC;
    pub type Hsclmd = crate::EnumBitfieldStruct<u8, Hsclmd_SPEC>;
    impl Hsclmd {
        #[doc = "Non-continuous clock mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Continuous clock mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulpssetr_SPEC;
impl crate::sealed::RegSpec for Ulpssetr_SPEC {
    type DataType = u32;
}

#[doc = "ULPS Set Register"]
pub type Ulpssetr = crate::RegValueT<Ulpssetr_SPEC>;

impl Ulpssetr {
    #[doc = "ULPS Wakeup Period"]
    #[inline(always)]
    pub fn wkup(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ulpssetr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ulpssetr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ulpssetr {
    #[inline(always)]
    fn default() -> Ulpssetr {
        <crate::RegValueT<Ulpssetr_SPEC> as RegisterValue<_>>::new(160)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulpscr_SPEC;
impl crate::sealed::RegSpec for Ulpscr_SPEC {
    type DataType = u32;
}

#[doc = "ULPS Control Register"]
pub type Ulpscr = crate::RegValueT<Ulpscr_SPEC>;

impl Ulpscr {
    #[doc = "CL ULPS Enter"]
    #[inline(always)]
    pub fn clent(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        ulpscr::Clent,
        ulpscr::Clent,
        Ulpscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            ulpscr::Clent,
            ulpscr::Clent,
            Ulpscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "CL ULPS Exit"]
    #[inline(always)]
    pub fn clexit(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ulpscr::Clexit,
        ulpscr::Clexit,
        Ulpscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ulpscr::Clexit,
            ulpscr::Clexit,
            Ulpscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "DL ULPS Enter"]
    #[inline(always)]
    pub fn dlent(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ulpscr::Dlent,
        ulpscr::Dlent,
        Ulpscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ulpscr::Dlent,
            ulpscr::Dlent,
            Ulpscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "DL ULPS Exit"]
    #[inline(always)]
    pub fn dlexit(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ulpscr::Dlexit,
        ulpscr::Dlexit,
        Ulpscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ulpscr::Dlexit,
            ulpscr::Dlexit,
            Ulpscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ulpscr {
    #[inline(always)]
    fn default() -> Ulpscr {
        <crate::RegValueT<Ulpscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ulpscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clent_SPEC;
    pub type Clent = crate::EnumBitfieldStruct<u8, Clent_SPEC>;
    impl Clent {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transition Clock Lane to ULPS"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clexit_SPEC;
    pub type Clexit = crate::EnumBitfieldStruct<u8, Clexit_SPEC>;
    impl Clexit {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clock Lane exits from ULPS"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlent_SPEC;
    pub type Dlent = crate::EnumBitfieldStruct<u8, Dlent_SPEC>;
    impl Dlent {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transition Data Lanes to ULPS"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlexit_SPEC;
    pub type Dlexit = crate::EnumBitfieldStruct<u8, Dlexit_SPEC>;
    impl Dlexit {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data Lanes exit from ULPS"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstcr_SPEC;
impl crate::sealed::RegSpec for Rstcr_SPEC {
    type DataType = u32;
}

#[doc = "Reset Control Register"]
pub type Rstcr = crate::RegValueT<Rstcr_SPEC>;

impl Rstcr {
    #[doc = "Software Reset"]
    #[inline(always)]
    pub fn swrst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rstcr::Swrst,
        rstcr::Swrst,
        Rstcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rstcr::Swrst,
            rstcr::Swrst,
            Rstcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Force Tx Stop Mode"]
    #[inline(always)]
    pub fn ftxstp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        rstcr::Ftxstp,
        rstcr::Ftxstp,
        Rstcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            rstcr::Ftxstp,
            rstcr::Ftxstp,
            Rstcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rstcr {
    #[inline(always)]
    fn default() -> Rstcr {
        <crate::RegValueT<Rstcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrst_SPEC;
    pub type Swrst = crate::EnumBitfieldStruct<u8, Swrst_SPEC>;
    impl Swrst {
        #[doc = "Complete the Software Reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request a Software Reset"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ftxstp_SPEC;
    pub type Ftxstp = crate::EnumBitfieldStruct<u8, Ftxstp_SPEC>;
    impl Ftxstp {
        #[doc = "Finish the Force Stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Force Data Lanes into transmit mode and generate Stop state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsr_SPEC;
impl crate::sealed::RegSpec for Rstsr_SPEC {
    type DataType = u32;
}

#[doc = "Reset Status Register"]
pub type Rstsr = crate::RegValueT<Rstsr_SPEC>;

impl Rstsr {
    #[doc = "HS Software Reset Status"]
    #[inline(always)]
    pub fn rsths(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rstsr::Rsths,
        rstsr::Rsths,
        Rstsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rstsr::Rsths,
            rstsr::Rsths,
            Rstsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "LP Software Reset Status"]
    #[inline(always)]
    pub fn rstlp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rstsr::Rstlp,
        rstsr::Rstlp,
        Rstsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rstsr::Rstlp,
            rstsr::Rstlp,
            Rstsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "APB Software Reset Status"]
    #[inline(always)]
    pub fn rstapb(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rstsr::Rstapb,
        rstsr::Rstapb,
        Rstsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rstsr::Rstapb,
            rstsr::Rstapb,
            Rstsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "AXI Software Reset Status"]
    #[inline(always)]
    pub fn rstaxi(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rstsr::Rstaxi,
        rstsr::Rstaxi,
        Rstsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rstsr::Rstaxi,
            rstsr::Rstaxi,
            Rstsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Video Software Reset Status"]
    #[inline(always)]
    pub fn rstv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        rstsr::Rstv,
        rstsr::Rstv,
        Rstsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            rstsr::Rstv,
            rstsr::Rstv,
            Rstsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane-0 Stop Status"]
    #[inline(always)]
    pub fn dl0stp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        rstsr::Dl0Stp,
        rstsr::Dl0Stp,
        Rstsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            rstsr::Dl0Stp,
            rstsr::Dl0Stp,
            Rstsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane-1 Stop Status"]
    #[inline(always)]
    pub fn dl1stp(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        rstsr::Dl1Stp,
        rstsr::Dl1Stp,
        Rstsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            rstsr::Dl1Stp,
            rstsr::Dl1Stp,
            Rstsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane-0 Direction"]
    #[inline(always)]
    pub fn dl0dir(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        rstsr::Dl0Dir,
        rstsr::Dl0Dir,
        Rstsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            rstsr::Dl0Dir,
            rstsr::Dl0Dir,
            Rstsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rstsr {
    #[inline(always)]
    fn default() -> Rstsr {
        <crate::RegValueT<Rstsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsths_SPEC;
    pub type Rsths = crate::EnumBitfieldStruct<u8, Rsths_SPEC>;
    impl Rsths {
        #[doc = "Not in software reset procedure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Running software reset procedure for HS mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstlp_SPEC;
    pub type Rstlp = crate::EnumBitfieldStruct<u8, Rstlp_SPEC>;
    impl Rstlp {
        #[doc = "Not in software reset procedure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Running software reset procedure for LP mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstapb_SPEC;
    pub type Rstapb = crate::EnumBitfieldStruct<u8, Rstapb_SPEC>;
    impl Rstapb {
        #[doc = "Not in software reset procedure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Running software reset procedure for APB bus"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstaxi_SPEC;
    pub type Rstaxi = crate::EnumBitfieldStruct<u8, Rstaxi_SPEC>;
    impl Rstaxi {
        #[doc = "Not in software reset procedure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Running software reset procedure for AXI bus"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstv_SPEC;
    pub type Rstv = crate::EnumBitfieldStruct<u8, Rstv_SPEC>;
    impl Rstv {
        #[doc = "Not in software reset procedure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Running software reset procedure for Video mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl0Stp_SPEC;
    pub type Dl0Stp = crate::EnumBitfieldStruct<u8, Dl0Stp_SPEC>;
    impl Dl0Stp {
        #[doc = "Not Stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl1Stp_SPEC;
    pub type Dl1Stp = crate::EnumBitfieldStruct<u8, Dl1Stp_SPEC>;
    impl Dl1Stp {
        #[doc = "Not Stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl0Dir_SPEC;
    pub type Dl0Dir = crate::EnumBitfieldStruct<u8, Dl0Dir_SPEC>;
    impl Dl0Dir {
        #[doc = "TX mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "RX mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsisetr_SPEC;
impl crate::sealed::RegSpec for Dsisetr_SPEC {
    type DataType = u32;
}

#[doc = "DSI Set Register"]
pub type Dsisetr = crate::RegValueT<Dsisetr_SPEC>;

impl Dsisetr {
    #[doc = "Maximum Return Packet Size"]
    #[inline(always)]
    pub fn mrpsz(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dsisetr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dsisetr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ECC Check Enable"]
    #[inline(always)]
    pub fn eccen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dsisetr::Eccen,
        dsisetr::Eccen,
        Dsisetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dsisetr::Eccen,
            dsisetr::Eccen,
            Dsisetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VC-0 CRC Check Enable"]
    #[inline(always)]
    pub fn vc0crcen(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        dsisetr::Vc0Crcen,
        dsisetr::Vc0Crcen,
        Dsisetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            dsisetr::Vc0Crcen,
            dsisetr::Vc0Crcen,
            Dsisetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VC-1 CRC Check Enable"]
    #[inline(always)]
    pub fn vc1crcen(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        dsisetr::Vc1Crcen,
        dsisetr::Vc1Crcen,
        Dsisetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            dsisetr::Vc1Crcen,
            dsisetr::Vc1Crcen,
            Dsisetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VC-2 CRC Check Enable"]
    #[inline(always)]
    pub fn vc2crcen(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        dsisetr::Vc2Crcen,
        dsisetr::Vc2Crcen,
        Dsisetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            dsisetr::Vc2Crcen,
            dsisetr::Vc2Crcen,
            Dsisetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VC-3 CRC Check Enable"]
    #[inline(always)]
    pub fn vc3crcen(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        dsisetr::Vc3Crcen,
        dsisetr::Vc3Crcen,
        Dsisetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            dsisetr::Vc3Crcen,
            dsisetr::Vc3Crcen,
            Dsisetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Scramble Enable"]
    #[inline(always)]
    pub fn scren(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        dsisetr::Scren,
        dsisetr::Scren,
        Dsisetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            dsisetr::Scren,
            dsisetr::Scren,
            Dsisetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "External Tearing Effect Detection Sense Select"]
    #[inline(always)]
    pub fn extemd(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        dsisetr::Extemd,
        dsisetr::Extemd,
        Dsisetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            dsisetr::Extemd,
            dsisetr::Extemd,
            Dsisetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HS Transfer EoTp Enable"]
    #[inline(always)]
    pub fn eotpen(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        dsisetr::Eotpen,
        dsisetr::Eotpen,
        Dsisetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            dsisetr::Eotpen,
            dsisetr::Eotpen,
            Dsisetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsisetr {
    #[inline(always)]
    fn default() -> Dsisetr {
        <crate::RegValueT<Dsisetr_SPEC> as RegisterValue<_>>::new(2163277825)
    }
}
pub mod dsisetr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccen_SPEC;
    pub type Eccen = crate::EnumBitfieldStruct<u8, Eccen_SPEC>;
    impl Eccen {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vc0Crcen_SPEC;
    pub type Vc0Crcen = crate::EnumBitfieldStruct<u8, Vc0Crcen_SPEC>;
    impl Vc0Crcen {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vc1Crcen_SPEC;
    pub type Vc1Crcen = crate::EnumBitfieldStruct<u8, Vc1Crcen_SPEC>;
    impl Vc1Crcen {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vc2Crcen_SPEC;
    pub type Vc2Crcen = crate::EnumBitfieldStruct<u8, Vc2Crcen_SPEC>;
    impl Vc2Crcen {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vc3Crcen_SPEC;
    pub type Vc3Crcen = crate::EnumBitfieldStruct<u8, Vc3Crcen_SPEC>;
    impl Vc3Crcen {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scren_SPEC;
    pub type Scren = crate::EnumBitfieldStruct<u8, Scren_SPEC>;
    impl Scren {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Extemd_SPEC;
    pub type Extemd = crate::EnumBitfieldStruct<u8, Extemd_SPEC>;
    impl Extemd {
        #[doc = "Rising edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Falling edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eotpen_SPEC;
    pub type Eotpen = crate::EnumBitfieldStruct<u8, Eotpen_SPEC>;
    impl Eotpen {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txppd0R_SPEC;
impl crate::sealed::RegSpec for Txppd0R_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Packet Payload Data 0 Register"]
pub type Txppd0R = crate::RegValueT<Txppd0R_SPEC>;

impl Txppd0R {
    #[doc = "Payload Data 0"]
    #[inline(always)]
    pub fn data0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Txppd0R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Txppd0R_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Payload Data 1"]
    #[inline(always)]
    pub fn data1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Txppd0R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Txppd0R_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Payload Data 2"]
    #[inline(always)]
    pub fn data2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Txppd0R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Txppd0R_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Payload Data 3"]
    #[inline(always)]
    pub fn data3(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Txppd0R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Txppd0R_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Txppd0R {
    #[inline(always)]
    fn default() -> Txppd0R {
        <crate::RegValueT<Txppd0R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txppd1R_SPEC;
impl crate::sealed::RegSpec for Txppd1R_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Packet Payload Data 1 Register"]
pub type Txppd1R = crate::RegValueT<Txppd1R_SPEC>;

impl Txppd1R {
    #[doc = "Payload Data 4"]
    #[inline(always)]
    pub fn data4(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Txppd1R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Txppd1R_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Payload Data 5"]
    #[inline(always)]
    pub fn data5(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Txppd1R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Txppd1R_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Payload Data 6"]
    #[inline(always)]
    pub fn data6(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Txppd1R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Txppd1R_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Payload Data 7"]
    #[inline(always)]
    pub fn data7(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Txppd1R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Txppd1R_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Txppd1R {
    #[inline(always)]
    fn default() -> Txppd1R {
        <crate::RegValueT<Txppd1R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txppd2R_SPEC;
impl crate::sealed::RegSpec for Txppd2R_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Packet Payload Data 2 Register"]
pub type Txppd2R = crate::RegValueT<Txppd2R_SPEC>;

impl Txppd2R {
    #[doc = "Payload Data 8"]
    #[inline(always)]
    pub fn data8(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Txppd2R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Txppd2R_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Payload Data 9"]
    #[inline(always)]
    pub fn data9(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Txppd2R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Txppd2R_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Payload Data 10"]
    #[inline(always)]
    pub fn data10(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Txppd2R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Txppd2R_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Payload Data 11"]
    #[inline(always)]
    pub fn data11(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Txppd2R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Txppd2R_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Txppd2R {
    #[inline(always)]
    fn default() -> Txppd2R {
        <crate::RegValueT<Txppd2R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txppd3R_SPEC;
impl crate::sealed::RegSpec for Txppd3R_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Packet Payload Data 3 Register"]
pub type Txppd3R = crate::RegValueT<Txppd3R_SPEC>;

impl Txppd3R {
    #[doc = "Payload Data 12"]
    #[inline(always)]
    pub fn data12(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Txppd3R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Txppd3R_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Payload Data 13"]
    #[inline(always)]
    pub fn data13(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Txppd3R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Txppd3R_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Payload Data 14"]
    #[inline(always)]
    pub fn data14(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Txppd3R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Txppd3R_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Payload Data 15"]
    #[inline(always)]
    pub fn data15(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Txppd3R_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Txppd3R_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Txppd3R {
    #[inline(always)]
    fn default() -> Txppd3R {
        <crate::RegValueT<Txppd3R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxsr_SPEC;
impl crate::sealed::RegSpec for Rxsr_SPEC {
    type DataType = u32;
}

#[doc = "Receive Status Register"]
pub type Rxsr = crate::RegValueT<Rxsr_SPEC>;

impl Rxsr {
    #[doc = "BTA Request End Interrupt Flag"]
    #[inline(always)]
    pub fn btarend(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rxsr::Btarend,
        rxsr::Btarend,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rxsr::Btarend,
            rxsr::Btarend,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "LP-RX Host Processor Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn lrxhto(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rxsr::Lrxhto,
        rxsr::Lrxhto,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rxsr::Lrxhto,
            rxsr::Lrxhto,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Turnaround Acknowledge Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn tato(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rxsr::Tato,
        rxsr::Tato,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rxsr::Tato,
            rxsr::Tato,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Response Packet Receive Interrupt Flag"]
    #[inline(always)]
    pub fn rxresp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        rxsr::Rxresp,
        rxsr::Rxresp,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            rxsr::Rxresp,
            rxsr::Rxresp,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "EoTp Receive Interrupt Flag"]
    #[inline(always)]
    pub fn rxeotp(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        rxsr::Rxeotp,
        rxsr::Rxeotp,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            rxsr::Rxeotp,
            rxsr::Rxeotp,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "ACK Trigger Receive Interrupt Flag"]
    #[inline(always)]
    pub fn rxack(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        rxsr::Rxack,
        rxsr::Rxack,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            rxsr::Rxack,
            rxsr::Rxack,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "External Tearing Effect Detect Interrupt Flag"]
    #[inline(always)]
    pub fn extedet(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        rxsr::Extedet,
        rxsr::Extedet,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            rxsr::Extedet,
            rxsr::Extedet,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Malform Error Interrupt Flag"]
    #[inline(always)]
    pub fn mlferr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        rxsr::Mlferr,
        rxsr::Mlferr,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            rxsr::Mlferr,
            rxsr::Mlferr,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Multi Bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn eccerrm(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        rxsr::Eccerrm,
        rxsr::Eccerrm,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            rxsr::Eccerrm,
            rxsr::Eccerrm,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Unexpected Packet Error Interrupt Flag"]
    #[inline(always)]
    pub fn unexerr(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        rxsr::Unexerr,
        rxsr::Unexerr,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            rxsr::Unexerr,
            rxsr::Unexerr,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Word Count Error Interrupt Flag"]
    #[inline(always)]
    pub fn wcerr(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        rxsr::Wcerr,
        rxsr::Wcerr,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            rxsr::Wcerr,
            rxsr::Wcerr,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CRC Error Interrupt Flag"]
    #[inline(always)]
    pub fn crcerr(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        rxsr::Crcerr,
        rxsr::Crcerr,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            rxsr::Crcerr,
            rxsr::Crcerr,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Internal Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn iberr(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        rxsr::Iberr,
        rxsr::Iberr,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            rxsr::Iberr,
            rxsr::Iberr,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Buffer Overflow Error Interrupt Flag"]
    #[inline(always)]
    pub fn rxovferr(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        rxsr::Rxovferr,
        rxsr::Rxovferr,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            rxsr::Rxovferr,
            rxsr::Rxovferr,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Response Timeout Error Interrupt Flag"]
    #[inline(always)]
    pub fn prtoerr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        rxsr::Prtoerr,
        rxsr::Prtoerr,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            rxsr::Prtoerr,
            rxsr::Prtoerr,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "No Response Error Interrupt Flag"]
    #[inline(always)]
    pub fn noreserr(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        rxsr::Noreserr,
        rxsr::Noreserr,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            rxsr::Noreserr,
            rxsr::Noreserr,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Return Packet Size Error Interrupt Flag"]
    #[inline(always)]
    pub fn rsizeerr(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        rxsr::Rsizeerr,
        rxsr::Rsizeerr,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            rxsr::Rsizeerr,
            rxsr::Rsizeerr,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Single Bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn eccerrs(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        rxsr::Eccerrs,
        rxsr::Eccerrs,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            rxsr::Eccerrs,
            rxsr::Eccerrs,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Acknowledge and Error Report Receive Interrupt Flag"]
    #[inline(always)]
    pub fn rxake(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        rxsr::Rxake,
        rxsr::Rxake,
        Rxsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            rxsr::Rxake,
            rxsr::Rxake,
            Rxsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rxsr {
    #[inline(always)]
    fn default() -> Rxsr {
        <crate::RegValueT<Rxsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rxsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Btarend_SPEC;
    pub type Btarend = crate::EnumBitfieldStruct<u8, Btarend_SPEC>;
    impl Btarend {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "BTA Completion detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lrxhto_SPEC;
    pub type Lrxhto = crate::EnumBitfieldStruct<u8, Lrxhto_SPEC>;
    impl Lrxhto {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "LP-RX Host Processor Timeout (LRX-H_TO) detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tato_SPEC;
    pub type Tato = crate::EnumBitfieldStruct<u8, Tato_SPEC>;
    impl Tato {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Turnaround Acknowledge Timeout (TA_TO) detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxresp_SPEC;
    pub type Rxresp = crate::EnumBitfieldStruct<u8, Rxresp_SPEC>;
    impl Rxresp {
        #[doc = "No response received"]
        pub const _0: Self = Self::new(0);

        #[doc = "Response packet received"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxeotp_SPEC;
    pub type Rxeotp = crate::EnumBitfieldStruct<u8, Rxeotp_SPEC>;
    impl Rxeotp {
        #[doc = "No received"]
        pub const _0: Self = Self::new(0);

        #[doc = "EoTp received"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxack_SPEC;
    pub type Rxack = crate::EnumBitfieldStruct<u8, Rxack_SPEC>;
    impl Rxack {
        #[doc = "No trigger received"]
        pub const _0: Self = Self::new(0);

        #[doc = "ACK trigger received"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Extedet_SPEC;
    pub type Extedet = crate::EnumBitfieldStruct<u8, Extedet_SPEC>;
    impl Extedet {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "External Tearing Effect detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mlferr_SPEC;
    pub type Mlferr = crate::EnumBitfieldStruct<u8, Mlferr_SPEC>;
    impl Mlferr {
        #[doc = "No error received"]
        pub const _0: Self = Self::new(0);

        #[doc = "A packet of less than 4 bytes received"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccerrm_SPEC;
    pub type Eccerrm = crate::EnumBitfieldStruct<u8, Eccerrm_SPEC>;
    impl Eccerrm {
        #[doc = "No error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "A multi-bit ECC error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Unexerr_SPEC;
    pub type Unexerr = crate::EnumBitfieldStruct<u8, Unexerr_SPEC>;
    impl Unexerr {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unexpected packet received"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wcerr_SPEC;
    pub type Wcerr = crate::EnumBitfieldStruct<u8, Wcerr_SPEC>;
    impl Wcerr {
        #[doc = "No error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "The length of the received packet is shorter than the WC indicated in the packet header."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crcerr_SPEC;
    pub type Crcerr = crate::EnumBitfieldStruct<u8, Crcerr_SPEC>;
    impl Crcerr {
        #[doc = "No error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "CRC error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iberr_SPEC;
    pub type Iberr = crate::EnumBitfieldStruct<u8, Iberr_SPEC>;
    impl Iberr {
        #[doc = "No error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal AXI bus write failed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxovferr_SPEC;
    pub type Rxovferr = crate::EnumBitfieldStruct<u8, Rxovferr_SPEC>;
    impl Rxovferr {
        #[doc = "No error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "A buffer overflow error detected on receiving long packets"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prtoerr_SPEC;
    pub type Prtoerr = crate::EnumBitfieldStruct<u8, Prtoerr_SPEC>;
    impl Prtoerr {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Peripheral response timeout occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Noreserr_SPEC;
    pub type Noreserr = crate::EnumBitfieldStruct<u8, Noreserr_SPEC>;
    impl Noreserr {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "No triggers or packets returned during BTA period"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsizeerr_SPEC;
    pub type Rsizeerr = crate::EnumBitfieldStruct<u8, Rsizeerr_SPEC>;
    impl Rsizeerr {
        #[doc = "No error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Oversize error in a received long packet detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccerrs_SPEC;
    pub type Eccerrs = crate::EnumBitfieldStruct<u8, Eccerrs_SPEC>;
    impl Eccerrs {
        #[doc = "No error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "A single-bit ECC error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxake_SPEC;
    pub type Rxake = crate::EnumBitfieldStruct<u8, Rxake_SPEC>;
    impl Rxake {
        #[doc = "No received"]
        pub const _0: Self = Self::new(0);

        #[doc = "An Acknowledge and Error Report packet received"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxscr_SPEC;
impl crate::sealed::RegSpec for Rxscr_SPEC {
    type DataType = u32;
}

#[doc = "Receive Status Clear Register"]
pub type Rxscr = crate::RegValueT<Rxscr_SPEC>;

impl Rxscr {
    #[doc = "BTA Request End Interrupt Flag Clear"]
    #[inline(always)]
    pub fn btarend(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rxscr::Btarend,
        rxscr::Btarend,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rxscr::Btarend,
            rxscr::Btarend,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "LP-RX Host Processor Timeout Interrupt Flag Clear"]
    #[inline(always)]
    pub fn lrxhto(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rxscr::Lrxhto,
        rxscr::Lrxhto,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rxscr::Lrxhto,
            rxscr::Lrxhto,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Turnaround Acknowledge Timeout Interrupt Flag Clear"]
    #[inline(always)]
    pub fn tato(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rxscr::Tato,
        rxscr::Tato,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rxscr::Tato,
            rxscr::Tato,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Response Packet Receive Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxresp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        rxscr::Rxresp,
        rxscr::Rxresp,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            rxscr::Rxresp,
            rxscr::Rxresp,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "EoTp Receive Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxeotp(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        rxscr::Rxeotp,
        rxscr::Rxeotp,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            rxscr::Rxeotp,
            rxscr::Rxeotp,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "ACK Trigger Receive Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxack(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        rxscr::Rxack,
        rxscr::Rxack,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            rxscr::Rxack,
            rxscr::Rxack,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "External Tearing Effect Detect Interrupt Flag Clear"]
    #[inline(always)]
    pub fn extedet(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        rxscr::Extedet,
        rxscr::Extedet,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            rxscr::Extedet,
            rxscr::Extedet,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Malform Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn mlferr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        rxscr::Mlferr,
        rxscr::Mlferr,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            rxscr::Mlferr,
            rxscr::Mlferr,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Multi Bit ECC Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn eccerrm(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        rxscr::Eccerrm,
        rxscr::Eccerrm,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            rxscr::Eccerrm,
            rxscr::Eccerrm,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Unexpected Packet Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn unexerr(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        rxscr::Unexerr,
        rxscr::Unexerr,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            rxscr::Unexerr,
            rxscr::Unexerr,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Word Count Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn wcerr(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        rxscr::Wcerr,
        rxscr::Wcerr,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            rxscr::Wcerr,
            rxscr::Wcerr,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "CRC Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn crcerr(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        rxscr::Crcerr,
        rxscr::Crcerr,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            rxscr::Crcerr,
            rxscr::Crcerr,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Internal Bus Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn iberr(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        rxscr::Iberr,
        rxscr::Iberr,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            rxscr::Iberr,
            rxscr::Iberr,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Receive Buffer Overflow Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxovferr(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        rxscr::Rxovferr,
        rxscr::Rxovferr,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            rxscr::Rxovferr,
            rxscr::Rxovferr,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Response Timeout Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn prtoerr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        rxscr::Prtoerr,
        rxscr::Prtoerr,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            rxscr::Prtoerr,
            rxscr::Prtoerr,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "No Response Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn noreserr(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        rxscr::Noreserr,
        rxscr::Noreserr,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            rxscr::Noreserr,
            rxscr::Noreserr,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Return Packet Size Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rsizeerr(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        rxscr::Rsizeerr,
        rxscr::Rsizeerr,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            rxscr::Rsizeerr,
            rxscr::Rsizeerr,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Single Bit ECC Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn eccerrs(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        rxscr::Eccerrs,
        rxscr::Eccerrs,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            rxscr::Eccerrs,
            rxscr::Eccerrs,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Acknowledge and Error Report Receive Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxake(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        rxscr::Rxake,
        rxscr::Rxake,
        Rxscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            rxscr::Rxake,
            rxscr::Rxake,
            Rxscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rxscr {
    #[inline(always)]
    fn default() -> Rxscr {
        <crate::RegValueT<Rxscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rxscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Btarend_SPEC;
    pub type Btarend = crate::EnumBitfieldStruct<u8, Btarend_SPEC>;
    impl Btarend {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.BTAREND flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lrxhto_SPEC;
    pub type Lrxhto = crate::EnumBitfieldStruct<u8, Lrxhto_SPEC>;
    impl Lrxhto {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.LRXHTO flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tato_SPEC;
    pub type Tato = crate::EnumBitfieldStruct<u8, Tato_SPEC>;
    impl Tato {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.TATO flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxresp_SPEC;
    pub type Rxresp = crate::EnumBitfieldStruct<u8, Rxresp_SPEC>;
    impl Rxresp {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.RXRESP flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxeotp_SPEC;
    pub type Rxeotp = crate::EnumBitfieldStruct<u8, Rxeotp_SPEC>;
    impl Rxeotp {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.RXEOTP flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxack_SPEC;
    pub type Rxack = crate::EnumBitfieldStruct<u8, Rxack_SPEC>;
    impl Rxack {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.RXACK flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Extedet_SPEC;
    pub type Extedet = crate::EnumBitfieldStruct<u8, Extedet_SPEC>;
    impl Extedet {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.EXTEDET flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mlferr_SPEC;
    pub type Mlferr = crate::EnumBitfieldStruct<u8, Mlferr_SPEC>;
    impl Mlferr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.MLFERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccerrm_SPEC;
    pub type Eccerrm = crate::EnumBitfieldStruct<u8, Eccerrm_SPEC>;
    impl Eccerrm {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.ECCERRM flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Unexerr_SPEC;
    pub type Unexerr = crate::EnumBitfieldStruct<u8, Unexerr_SPEC>;
    impl Unexerr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.UNEXERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wcerr_SPEC;
    pub type Wcerr = crate::EnumBitfieldStruct<u8, Wcerr_SPEC>;
    impl Wcerr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.WCERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crcerr_SPEC;
    pub type Crcerr = crate::EnumBitfieldStruct<u8, Crcerr_SPEC>;
    impl Crcerr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.CRCERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iberr_SPEC;
    pub type Iberr = crate::EnumBitfieldStruct<u8, Iberr_SPEC>;
    impl Iberr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.IBERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxovferr_SPEC;
    pub type Rxovferr = crate::EnumBitfieldStruct<u8, Rxovferr_SPEC>;
    impl Rxovferr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.RXOVFERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prtoerr_SPEC;
    pub type Prtoerr = crate::EnumBitfieldStruct<u8, Prtoerr_SPEC>;
    impl Prtoerr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.PRTOERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Noreserr_SPEC;
    pub type Noreserr = crate::EnumBitfieldStruct<u8, Noreserr_SPEC>;
    impl Noreserr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.NORESERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsizeerr_SPEC;
    pub type Rsizeerr = crate::EnumBitfieldStruct<u8, Rsizeerr_SPEC>;
    impl Rsizeerr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.RSIZEERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccerrs_SPEC;
    pub type Eccerrs = crate::EnumBitfieldStruct<u8, Eccerrs_SPEC>;
    impl Eccerrs {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.ECCERRS flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxake_SPEC;
    pub type Rxake = crate::EnumBitfieldStruct<u8, Rxake_SPEC>;
    impl Rxake {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXSR.RXAKE flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxier_SPEC;
impl crate::sealed::RegSpec for Rxier_SPEC {
    type DataType = u32;
}

#[doc = "Receive Interrupt Enable Register"]
pub type Rxier = crate::RegValueT<Rxier_SPEC>;

impl Rxier {
    #[doc = "BTA Request End Interrupt Enable"]
    #[inline(always)]
    pub fn btarend(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rxier::Btarend,
        rxier::Btarend,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rxier::Btarend,
            rxier::Btarend,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LP-RX Host Processor Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn lrxhto(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rxier::Lrxhto,
        rxier::Lrxhto,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rxier::Lrxhto,
            rxier::Lrxhto,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Turnaround Acknowledge Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn tato(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rxier::Tato,
        rxier::Tato,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rxier::Tato,
            rxier::Tato,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Response Packet Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxresp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        rxier::Rxresp,
        rxier::Rxresp,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            rxier::Rxresp,
            rxier::Rxresp,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "EoTp Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxeotp(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        rxier::Rxeotp,
        rxier::Rxeotp,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            rxier::Rxeotp,
            rxier::Rxeotp,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACK Trigger Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxack(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        rxier::Rxack,
        rxier::Rxack,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            rxier::Rxack,
            rxier::Rxack,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "External Tearing Effect Detect Interrupt Enable"]
    #[inline(always)]
    pub fn extedet(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        rxier::Extedet,
        rxier::Extedet,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            rxier::Extedet,
            rxier::Extedet,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Malform Error Interrupt Enable"]
    #[inline(always)]
    pub fn mlferr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        rxier::Mlferr,
        rxier::Mlferr,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            rxier::Mlferr,
            rxier::Mlferr,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multi Bit ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccerrm(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        rxier::Eccerrm,
        rxier::Eccerrm,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            rxier::Eccerrm,
            rxier::Eccerrm,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Unexpected Packet Error Interrupt Enable"]
    #[inline(always)]
    pub fn unexerr(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        rxier::Unexerr,
        rxier::Unexerr,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            rxier::Unexerr,
            rxier::Unexerr,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Word Count Error Interrupt Enable"]
    #[inline(always)]
    pub fn wcerr(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        rxier::Wcerr,
        rxier::Wcerr,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            rxier::Wcerr,
            rxier::Wcerr,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn crcerr(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        rxier::Crcerr,
        rxier::Crcerr,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            rxier::Crcerr,
            rxier::Crcerr,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Internal Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn iberr(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        rxier::Iberr,
        rxier::Iberr,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            rxier::Iberr,
            rxier::Iberr,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Buffer Overflow Error Interrupt Enable"]
    #[inline(always)]
    pub fn rxovferr(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        rxier::Rxovferr,
        rxier::Rxovferr,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            rxier::Rxovferr,
            rxier::Rxovferr,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Response Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn prtoerr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        rxier::Prtoerr,
        rxier::Prtoerr,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            rxier::Prtoerr,
            rxier::Prtoerr,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "No Response Error Interrupt Enable"]
    #[inline(always)]
    pub fn noreserr(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        rxier::Noreserr,
        rxier::Noreserr,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            rxier::Noreserr,
            rxier::Noreserr,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Return Packet Size Error Interrupt Enable"]
    #[inline(always)]
    pub fn rsizeerr(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        rxier::Rsizeerr,
        rxier::Rsizeerr,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            rxier::Rsizeerr,
            rxier::Rsizeerr,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Single Bit ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccerrs(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        rxier::Eccerrs,
        rxier::Eccerrs,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            rxier::Eccerrs,
            rxier::Eccerrs,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Acknowledge and Error Report Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxake(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        rxier::Rxake,
        rxier::Rxake,
        Rxier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            rxier::Rxake,
            rxier::Rxake,
            Rxier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rxier {
    #[inline(always)]
    fn default() -> Rxier {
        <crate::RegValueT<Rxier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rxier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Btarend_SPEC;
    pub type Btarend = crate::EnumBitfieldStruct<u8, Btarend_SPEC>;
    impl Btarend {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lrxhto_SPEC;
    pub type Lrxhto = crate::EnumBitfieldStruct<u8, Lrxhto_SPEC>;
    impl Lrxhto {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tato_SPEC;
    pub type Tato = crate::EnumBitfieldStruct<u8, Tato_SPEC>;
    impl Tato {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxresp_SPEC;
    pub type Rxresp = crate::EnumBitfieldStruct<u8, Rxresp_SPEC>;
    impl Rxresp {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxeotp_SPEC;
    pub type Rxeotp = crate::EnumBitfieldStruct<u8, Rxeotp_SPEC>;
    impl Rxeotp {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxack_SPEC;
    pub type Rxack = crate::EnumBitfieldStruct<u8, Rxack_SPEC>;
    impl Rxack {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Extedet_SPEC;
    pub type Extedet = crate::EnumBitfieldStruct<u8, Extedet_SPEC>;
    impl Extedet {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mlferr_SPEC;
    pub type Mlferr = crate::EnumBitfieldStruct<u8, Mlferr_SPEC>;
    impl Mlferr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccerrm_SPEC;
    pub type Eccerrm = crate::EnumBitfieldStruct<u8, Eccerrm_SPEC>;
    impl Eccerrm {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Unexerr_SPEC;
    pub type Unexerr = crate::EnumBitfieldStruct<u8, Unexerr_SPEC>;
    impl Unexerr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wcerr_SPEC;
    pub type Wcerr = crate::EnumBitfieldStruct<u8, Wcerr_SPEC>;
    impl Wcerr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crcerr_SPEC;
    pub type Crcerr = crate::EnumBitfieldStruct<u8, Crcerr_SPEC>;
    impl Crcerr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iberr_SPEC;
    pub type Iberr = crate::EnumBitfieldStruct<u8, Iberr_SPEC>;
    impl Iberr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxovferr_SPEC;
    pub type Rxovferr = crate::EnumBitfieldStruct<u8, Rxovferr_SPEC>;
    impl Rxovferr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prtoerr_SPEC;
    pub type Prtoerr = crate::EnumBitfieldStruct<u8, Prtoerr_SPEC>;
    impl Prtoerr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Noreserr_SPEC;
    pub type Noreserr = crate::EnumBitfieldStruct<u8, Noreserr_SPEC>;
    impl Noreserr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsizeerr_SPEC;
    pub type Rsizeerr = crate::EnumBitfieldStruct<u8, Rsizeerr_SPEC>;
    impl Rsizeerr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccerrs_SPEC;
    pub type Eccerrs = crate::EnumBitfieldStruct<u8, Eccerrs_SPEC>;
    impl Eccerrs {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxake_SPEC;
    pub type Rxake = crate::EnumBitfieldStruct<u8, Rxake_SPEC>;
    impl Rxake {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presptobtasetr_SPEC;
impl crate::sealed::RegSpec for Presptobtasetr_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Response Timeout BTA Set Register"]
pub type Presptobtasetr = crate::RegValueT<Presptobtasetr_SPEC>;

impl Presptobtasetr {
    #[doc = "Peripheral Response Timeout Count"]
    #[inline(always)]
    pub fn prtbta(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Presptobtasetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Presptobtasetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Presptobtasetr {
    #[inline(always)]
    fn default() -> Presptobtasetr {
        <crate::RegValueT<Presptobtasetr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presptolpsetr_SPEC;
impl crate::sealed::RegSpec for Presptolpsetr_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Response Timeout LP Set Register"]
pub type Presptolpsetr = crate::RegValueT<Presptolpsetr_SPEC>;

impl Presptolpsetr {
    #[doc = "LPDT WRITE Request Timeout"]
    #[inline(always)]
    pub fn lpwto(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Presptolpsetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Presptolpsetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LPDT READ Request Timeout"]
    #[inline(always)]
    pub fn lprto(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Presptolpsetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Presptolpsetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Presptolpsetr {
    #[inline(always)]
    fn default() -> Presptolpsetr {
        <crate::RegValueT<Presptolpsetr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presptohssetr_SPEC;
impl crate::sealed::RegSpec for Presptohssetr_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Response Timeout HS Set Register"]
pub type Presptohssetr = crate::RegValueT<Presptohssetr_SPEC>;

impl Presptohssetr {
    #[doc = "HS WRITE Request Timeout"]
    #[inline(always)]
    pub fn hswto(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Presptohssetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Presptohssetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HS READ Request Timeout"]
    #[inline(always)]
    pub fn hsrto(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Presptohssetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Presptohssetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Presptohssetr {
    #[inline(always)]
    fn default() -> Presptohssetr {
        <crate::RegValueT<Presptohssetr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Akeplatir_SPEC;
impl crate::sealed::RegSpec for Akeplatir_SPEC {
    type DataType = u32;
}

#[doc = "Acknowledge and Error Report Packet Parameter Latest Info Register"]
pub type Akeplatir = crate::RegValueT<Akeplatir_SPEC>;

impl Akeplatir {
    #[doc = "Error Report"]
    #[inline(always)]
    pub fn erep(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Akeplatir_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Akeplatir_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Virtual Channel ID"]
    #[inline(always)]
    pub fn vc(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        akeplatir::Vc,
        akeplatir::Vc,
        Akeplatir_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            akeplatir::Vc,
            akeplatir::Vc,
            Akeplatir_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Akeplatir {
    #[inline(always)]
    fn default() -> Akeplatir {
        <crate::RegValueT<Akeplatir_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod akeplatir {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vc_SPEC;
    pub type Vc = crate::EnumBitfieldStruct<u8, Vc_SPEC>;
    impl Vc {
        #[doc = "No Error Report received"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "From VC-0"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "From VC-1"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "From VC-2"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "From VC-3"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "Reserved"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Akepacmsr_SPEC;
impl crate::sealed::RegSpec for Akepacmsr_SPEC {
    type DataType = u32;
}

#[doc = "Acknowledge and Error Report Packet Parameter Accumulate Status Register"]
pub type Akepacmsr = crate::RegValueT<Akepacmsr_SPEC>;

impl Akepacmsr {
    #[doc = "Accumulated Error Report"]
    #[inline(always)]
    pub fn aerep(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Akepacmsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Akepacmsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Virtual Channel-0 Accumulated Information"]
    #[inline(always)]
    pub fn avc0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        akepacmsr::Avc0,
        akepacmsr::Avc0,
        Akepacmsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            akepacmsr::Avc0,
            akepacmsr::Avc0,
            Akepacmsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Virtual Channel-1 Accumulated Information"]
    #[inline(always)]
    pub fn avc1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        akepacmsr::Avc1,
        akepacmsr::Avc1,
        Akepacmsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            akepacmsr::Avc1,
            akepacmsr::Avc1,
            Akepacmsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Virtual Channel-2 Accumulated Information"]
    #[inline(always)]
    pub fn avc2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        akepacmsr::Avc2,
        akepacmsr::Avc2,
        Akepacmsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            akepacmsr::Avc2,
            akepacmsr::Avc2,
            Akepacmsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Virtual Channel-3 Accumulated Information"]
    #[inline(always)]
    pub fn avc3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        akepacmsr::Avc3,
        akepacmsr::Avc3,
        Akepacmsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            akepacmsr::Avc3,
            akepacmsr::Avc3,
            Akepacmsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Akepacmsr {
    #[inline(always)]
    fn default() -> Akepacmsr {
        <crate::RegValueT<Akepacmsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod akepacmsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Avc0_SPEC;
    pub type Avc0 = crate::EnumBitfieldStruct<u8, Avc0_SPEC>;
    impl Avc0 {
        #[doc = "No Error Report received from VC-0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received an Acknowledge and Error Report from VC-0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Avc1_SPEC;
    pub type Avc1 = crate::EnumBitfieldStruct<u8, Avc1_SPEC>;
    impl Avc1 {
        #[doc = "No Error Report received from VC-1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received an Acknowledge and Error Report from VC-1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Avc2_SPEC;
    pub type Avc2 = crate::EnumBitfieldStruct<u8, Avc2_SPEC>;
    impl Avc2 {
        #[doc = "No Error Report received from VC-2"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received an Acknowledge and Error Report from VC-2"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Avc3_SPEC;
    pub type Avc3 = crate::EnumBitfieldStruct<u8, Avc3_SPEC>;
    impl Avc3 {
        #[doc = "No Error Report received from VC-3"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received an Acknowledge and Error Report from VC-3"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Akepscr_SPEC;
impl crate::sealed::RegSpec for Akepscr_SPEC {
    type DataType = u32;
}

#[doc = "Acknowledge and Error Report Packet Parameter Status Clear Register"]
pub type Akepscr = crate::RegValueT<Akepscr_SPEC>;

impl Akepscr {
    #[doc = "Accumulated Error Report Clear"]
    #[inline(always)]
    pub fn aerep(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        akepscr::Aerep,
        akepscr::Aerep,
        Akepscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            akepscr::Aerep,
            akepscr::Aerep,
            Akepscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Virtual Channel-0 Accumulated Information Clear"]
    #[inline(always)]
    pub fn avc0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        akepscr::Avc0,
        akepscr::Avc0,
        Akepscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            akepscr::Avc0,
            akepscr::Avc0,
            Akepscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Virtual Channel-1 Accumulated Information Clear"]
    #[inline(always)]
    pub fn avc1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        akepscr::Avc1,
        akepscr::Avc1,
        Akepscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            akepscr::Avc1,
            akepscr::Avc1,
            Akepscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Virtual Channel-2 Accumulated Information Clear"]
    #[inline(always)]
    pub fn avc2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        akepscr::Avc2,
        akepscr::Avc2,
        Akepscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            akepscr::Avc2,
            akepscr::Avc2,
            Akepscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Virtual Channel-3 Accumulated Information Clear"]
    #[inline(always)]
    pub fn avc3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        akepscr::Avc3,
        akepscr::Avc3,
        Akepscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            akepscr::Avc3,
            akepscr::Avc3,
            Akepscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Akepscr {
    #[inline(always)]
    fn default() -> Akepscr {
        <crate::RegValueT<Akepscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod akepscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aerep_SPEC;
    pub type Aerep = crate::EnumBitfieldStruct<u8, Aerep_SPEC>;
    impl Aerep {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the AKEPACMSR.AEREP\\[15:0\\] bits"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Avc0_SPEC;
    pub type Avc0 = crate::EnumBitfieldStruct<u8, Avc0_SPEC>;
    impl Avc0 {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the AKEPACMSR.AVC0 bit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Avc1_SPEC;
    pub type Avc1 = crate::EnumBitfieldStruct<u8, Avc1_SPEC>;
    impl Avc1 {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the AKEPACMSR.AVC1 bit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Avc2_SPEC;
    pub type Avc2 = crate::EnumBitfieldStruct<u8, Avc2_SPEC>;
    impl Avc2 {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the AKEPACMSR.AVC2 bit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Avc3_SPEC;
    pub type Avc3 = crate::EnumBitfieldStruct<u8, Avc3_SPEC>;
    impl Avc3 {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the AKEPACMSR.AVC3 bit"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxrsscr_SPEC;
impl crate::sealed::RegSpec for Rxrsscr_SPEC {
    type DataType = u32;
}

#[doc = "Receive Result Saved Status Clear Register"]
pub type Rxrsscr = crate::RegValueT<Rxrsscr_SPEC>;

impl Rxrsscr {
    #[doc = "Slot-0 Valid Flag Clear"]
    #[inline(always)]
    pub fn slt0vld(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rxrsscr::Slt0Vld,
        rxrsscr::Slt0Vld,
        Rxrsscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rxrsscr::Slt0Vld,
            rxrsscr::Slt0Vld,
            Rxrsscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Slot-1 Valid Flag Clear"]
    #[inline(always)]
    pub fn slt1vld(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rxrsscr::Slt1Vld,
        rxrsscr::Slt1Vld,
        Rxrsscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rxrsscr::Slt1Vld,
            rxrsscr::Slt1Vld,
            Rxrsscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Slot-2 Valid Flag Clear"]
    #[inline(always)]
    pub fn slt2vld(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rxrsscr::Slt2Vld,
        rxrsscr::Slt2Vld,
        Rxrsscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rxrsscr::Slt2Vld,
            rxrsscr::Slt2Vld,
            Rxrsscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Slot-3 Valid Flag Clear"]
    #[inline(always)]
    pub fn slt3vld(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rxrsscr::Slt3Vld,
        rxrsscr::Slt3Vld,
        Rxrsscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rxrsscr::Slt3Vld,
            rxrsscr::Slt3Vld,
            Rxrsscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rxrsscr {
    #[inline(always)]
    fn default() -> Rxrsscr {
        <crate::RegValueT<Rxrsscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rxrsscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slt0Vld_SPEC;
    pub type Slt0Vld = crate::EnumBitfieldStruct<u8, Slt0Vld_SPEC>;
    impl Slt0Vld {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXRSSR.SLT0VLD flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slt1Vld_SPEC;
    pub type Slt1Vld = crate::EnumBitfieldStruct<u8, Slt1Vld_SPEC>;
    impl Slt1Vld {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXRSSR.SLT1VLD flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slt2Vld_SPEC;
    pub type Slt2Vld = crate::EnumBitfieldStruct<u8, Slt2Vld_SPEC>;
    impl Slt2Vld {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXRSSR.SLT2VLD flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slt3Vld_SPEC;
    pub type Slt3Vld = crate::EnumBitfieldStruct<u8, Slt3Vld_SPEC>;
    impl Slt3Vld {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXRSSR.SLT3VLD flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxrinfoowsr_SPEC;
impl crate::sealed::RegSpec for Rxrinfoowsr_SPEC {
    type DataType = u32;
}

#[doc = "Receive Result Info Overwrite Status Register"]
pub type Rxrinfoowsr = crate::RegValueT<Rxrinfoowsr_SPEC>;

impl Rxrinfoowsr {
    #[doc = "Slot-0 Information Overwrite Flag"]
    #[inline(always)]
    pub fn sl0ow(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rxrinfoowsr::Sl0Ow,
        rxrinfoowsr::Sl0Ow,
        Rxrinfoowsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rxrinfoowsr::Sl0Ow,
            rxrinfoowsr::Sl0Ow,
            Rxrinfoowsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slot-1 Information Overwrite Flag"]
    #[inline(always)]
    pub fn sl1ow(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rxrinfoowsr::Sl1Ow,
        rxrinfoowsr::Sl1Ow,
        Rxrinfoowsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rxrinfoowsr::Sl1Ow,
            rxrinfoowsr::Sl1Ow,
            Rxrinfoowsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slot-2 Information Overwrite Flag"]
    #[inline(always)]
    pub fn sl2ow(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rxrinfoowsr::Sl2Ow,
        rxrinfoowsr::Sl2Ow,
        Rxrinfoowsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rxrinfoowsr::Sl2Ow,
            rxrinfoowsr::Sl2Ow,
            Rxrinfoowsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slot-3 Information Overwrite Flag"]
    #[inline(always)]
    pub fn sl3ow(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rxrinfoowsr::Sl3Ow,
        rxrinfoowsr::Sl3Ow,
        Rxrinfoowsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rxrinfoowsr::Sl3Ow,
            rxrinfoowsr::Sl3Ow,
            Rxrinfoowsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rxrinfoowsr {
    #[inline(always)]
    fn default() -> Rxrinfoowsr {
        <crate::RegValueT<Rxrinfoowsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rxrinfoowsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sl0Ow_SPEC;
    pub type Sl0Ow = crate::EnumBitfieldStruct<u8, Sl0Ow_SPEC>;
    impl Sl0Ow {
        #[doc = "No overwritten"]
        pub const _0: Self = Self::new(0);

        #[doc = "Slot-0 information overwritten"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sl1Ow_SPEC;
    pub type Sl1Ow = crate::EnumBitfieldStruct<u8, Sl1Ow_SPEC>;
    impl Sl1Ow {
        #[doc = "No overwritten"]
        pub const _0: Self = Self::new(0);

        #[doc = "Slot-1 information overwritten"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sl2Ow_SPEC;
    pub type Sl2Ow = crate::EnumBitfieldStruct<u8, Sl2Ow_SPEC>;
    impl Sl2Ow {
        #[doc = "No overwritten"]
        pub const _0: Self = Self::new(0);

        #[doc = "Slot-2 information overwritten"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sl3Ow_SPEC;
    pub type Sl3Ow = crate::EnumBitfieldStruct<u8, Sl3Ow_SPEC>;
    impl Sl3Ow {
        #[doc = "No overwritten"]
        pub const _0: Self = Self::new(0);

        #[doc = "Slot-3 information overwritten"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxrinfoowscr_SPEC;
impl crate::sealed::RegSpec for Rxrinfoowscr_SPEC {
    type DataType = u32;
}

#[doc = "Receive Result Info Overwrite Status Clear Register"]
pub type Rxrinfoowscr = crate::RegValueT<Rxrinfoowscr_SPEC>;

impl Rxrinfoowscr {
    #[doc = "Slot-0 Information Overwrite Flag Clear"]
    #[inline(always)]
    pub fn sl0ow(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rxrinfoowscr::Sl0Ow,
        rxrinfoowscr::Sl0Ow,
        Rxrinfoowscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rxrinfoowscr::Sl0Ow,
            rxrinfoowscr::Sl0Ow,
            Rxrinfoowscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Slot-1 Information Overwrite Flag Clear"]
    #[inline(always)]
    pub fn sl1ow(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rxrinfoowscr::Sl1Ow,
        rxrinfoowscr::Sl1Ow,
        Rxrinfoowscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rxrinfoowscr::Sl1Ow,
            rxrinfoowscr::Sl1Ow,
            Rxrinfoowscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Slot-2 Information Overwrite Flag Clear"]
    #[inline(always)]
    pub fn sl2ow(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rxrinfoowscr::Sl2Ow,
        rxrinfoowscr::Sl2Ow,
        Rxrinfoowscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rxrinfoowscr::Sl2Ow,
            rxrinfoowscr::Sl2Ow,
            Rxrinfoowscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Slot-3 Information Overwrite Flag Clear"]
    #[inline(always)]
    pub fn sl3ow(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rxrinfoowscr::Sl3Ow,
        rxrinfoowscr::Sl3Ow,
        Rxrinfoowscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rxrinfoowscr::Sl3Ow,
            rxrinfoowscr::Sl3Ow,
            Rxrinfoowscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rxrinfoowscr {
    #[inline(always)]
    fn default() -> Rxrinfoowscr {
        <crate::RegValueT<Rxrinfoowscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rxrinfoowscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sl0Ow_SPEC;
    pub type Sl0Ow = crate::EnumBitfieldStruct<u8, Sl0Ow_SPEC>;
    impl Sl0Ow {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXRINFOOWSR.SL0OW flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sl1Ow_SPEC;
    pub type Sl1Ow = crate::EnumBitfieldStruct<u8, Sl1Ow_SPEC>;
    impl Sl1Ow {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXRINFOOWSR.SL1OW flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sl2Ow_SPEC;
    pub type Sl2Ow = crate::EnumBitfieldStruct<u8, Sl2Ow_SPEC>;
    impl Sl2Ow {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXRINFOOWSR.SL2OW flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sl3Ow_SPEC;
    pub type Sl3Ow = crate::EnumBitfieldStruct<u8, Sl3Ow_SPEC>;
    impl Sl3Ow {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the RXRINFOOWSR.SL3OW flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxrssr_SPEC;
impl crate::sealed::RegSpec for Rxrssr_SPEC {
    type DataType = u32;
}

#[doc = "Receive Result Save Slot-x Register (x = 0 to 3)"]
pub type Rxrssr = crate::RegValueT<Rxrssr_SPEC>;

impl Rxrssr {
    #[doc = "Data 0"]
    #[inline(always)]
    pub fn data0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Rxrssr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Rxrssr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Data 1"]
    #[inline(always)]
    pub fn data1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Rxrssr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Rxrssr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Data Type"]
    #[inline(always)]
    pub fn dt(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Rxrssr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,Rxrssr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Virtual Channel"]
    #[inline(always)]
    pub fn vc(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, u8, Rxrssr_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x3,1,0,u8,u8,Rxrssr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Packet Format"]
    #[inline(always)]
    pub fn fmt(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        rxrssr::Fmt,
        rxrssr::Fmt,
        Rxrssr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            rxrssr::Fmt,
            rxrssr::Fmt,
            Rxrssr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Success"]
    #[inline(always)]
    pub fn rxsuc(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        rxrssr::Rxsuc,
        rxrssr::Rxsuc,
        Rxrssr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            rxrssr::Rxsuc,
            rxrssr::Rxsuc,
            Rxrssr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Fatal Error"]
    #[inline(always)]
    pub fn rxferr(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        rxrssr::Rxferr,
        rxrssr::Rxferr,
        Rxrssr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            rxrssr::Rxferr,
            rxrssr::Rxferr,
            Rxrssr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Fail"]
    #[inline(always)]
    pub fn rxfail(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        rxrssr::Rxfail,
        rxrssr::Rxfail,
        Rxrssr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            rxrssr::Rxfail,
            rxrssr::Rxfail,
            Rxrssr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Packet Data Fail"]
    #[inline(always)]
    pub fn rxpfail(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        rxrssr::Rxpfail,
        rxrssr::Rxpfail,
        Rxrssr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            rxrssr::Rxpfail,
            rxrssr::Rxpfail,
            Rxrssr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Correctable Error"]
    #[inline(always)]
    pub fn rxcerr(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        rxrssr::Rxcerr,
        rxrssr::Rxcerr,
        Rxrssr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            rxrssr::Rxcerr,
            rxrssr::Rxcerr,
            Rxrssr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Acknowledge and Error Report Packet"]
    #[inline(always)]
    pub fn rxake(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        rxrssr::Rxake,
        rxrssr::Rxake,
        Rxrssr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            rxrssr::Rxake,
            rxrssr::Rxake,
            Rxrssr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Information Overwrite"]
    #[inline(always)]
    pub fn infoow(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        rxrssr::Infoow,
        rxrssr::Infoow,
        Rxrssr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            rxrssr::Infoow,
            rxrssr::Infoow,
            Rxrssr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rxrssr {
    #[inline(always)]
    fn default() -> Rxrssr {
        <crate::RegValueT<Rxrssr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rxrssr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fmt_SPEC;
    pub type Fmt = crate::EnumBitfieldStruct<u8, Fmt_SPEC>;
    impl Fmt {
        #[doc = "Short packet"]
        pub const _0: Self = Self::new(0);

        #[doc = "Long packet"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxsuc_SPEC;
    pub type Rxsuc = crate::EnumBitfieldStruct<u8, Rxsuc_SPEC>;
    impl Rxsuc {
        #[doc = "No received"]
        pub const _0: Self = Self::new(0);

        #[doc = "Response packet or ACK trigger received"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxferr_SPEC;
    pub type Rxferr = crate::EnumBitfieldStruct<u8, Rxferr_SPEC>;
    impl Rxferr {
        #[doc = "No fatal error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Fatal timeout occurred during BTA"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxfail_SPEC;
    pub type Rxfail = crate::EnumBitfieldStruct<u8, Rxfail_SPEC>;
    impl Rxfail {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Expected receive not done"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxpfail_SPEC;
    pub type Rxpfail = crate::EnumBitfieldStruct<u8, Rxpfail_SPEC>;
    impl Rxpfail {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Payload data not saved correctly"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxcerr_SPEC;
    pub type Rxcerr = crate::EnumBitfieldStruct<u8, Rxcerr_SPEC>;
    impl Rxcerr {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Correctable error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxake_SPEC;
    pub type Rxake = crate::EnumBitfieldStruct<u8, Rxake_SPEC>;
    impl Rxake {
        #[doc = "No received"]
        pub const _0: Self = Self::new(0);

        #[doc = "An Acknowledge and Error Report packet received"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Infoow_SPEC;
    pub type Infoow = crate::EnumBitfieldStruct<u8, Infoow_SPEC>;
    impl Infoow {
        #[doc = "No update"]
        pub const _0: Self = Self::new(0);

        #[doc = "This register information (RXRSSxR\\[30:0\\]) was overwritten"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxppd0R_SPEC;
impl crate::sealed::RegSpec for Rxppd0R_SPEC {
    type DataType = u32;
}

#[doc = "Receive Packet Payload Data 0 Register"]
pub type Rxppd0R = crate::RegValueT<Rxppd0R_SPEC>;

impl Rxppd0R {
    #[doc = "Payload Data 0"]
    #[inline(always)]
    pub fn data0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Rxppd0R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Rxppd0R_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Payload Data 1"]
    #[inline(always)]
    pub fn data1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Rxppd0R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Rxppd0R_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Payload Data 2"]
    #[inline(always)]
    pub fn data2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Rxppd0R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Rxppd0R_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Payload Data 3"]
    #[inline(always)]
    pub fn data3(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Rxppd0R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Rxppd0R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rxppd0R {
    #[inline(always)]
    fn default() -> Rxppd0R {
        <crate::RegValueT<Rxppd0R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxppd1R_SPEC;
impl crate::sealed::RegSpec for Rxppd1R_SPEC {
    type DataType = u32;
}

#[doc = "Receive Packet Payload Data 1 Register"]
pub type Rxppd1R = crate::RegValueT<Rxppd1R_SPEC>;

impl Rxppd1R {
    #[doc = "Payload Data 4"]
    #[inline(always)]
    pub fn data4(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Rxppd1R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Rxppd1R_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Payload Data 5"]
    #[inline(always)]
    pub fn data5(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Rxppd1R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Rxppd1R_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Payload Data 6"]
    #[inline(always)]
    pub fn data6(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Rxppd1R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Rxppd1R_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Payload Data 7"]
    #[inline(always)]
    pub fn data7(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Rxppd1R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Rxppd1R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rxppd1R {
    #[inline(always)]
    fn default() -> Rxppd1R {
        <crate::RegValueT<Rxppd1R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxppd2R_SPEC;
impl crate::sealed::RegSpec for Rxppd2R_SPEC {
    type DataType = u32;
}

#[doc = "Receive Packet Payload Data 2 Register"]
pub type Rxppd2R = crate::RegValueT<Rxppd2R_SPEC>;

impl Rxppd2R {
    #[doc = "Payload Data 8"]
    #[inline(always)]
    pub fn data8(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Rxppd2R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Rxppd2R_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Payload Data 9"]
    #[inline(always)]
    pub fn data9(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Rxppd2R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Rxppd2R_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Payload Data 10"]
    #[inline(always)]
    pub fn data10(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Rxppd2R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Rxppd2R_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Payload Data 11"]
    #[inline(always)]
    pub fn data11(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Rxppd2R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Rxppd2R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rxppd2R {
    #[inline(always)]
    fn default() -> Rxppd2R {
        <crate::RegValueT<Rxppd2R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxppd3R_SPEC;
impl crate::sealed::RegSpec for Rxppd3R_SPEC {
    type DataType = u32;
}

#[doc = "Receive Packet Payload Data 3 Register"]
pub type Rxppd3R = crate::RegValueT<Rxppd3R_SPEC>;

impl Rxppd3R {
    #[doc = "Payload Data 12"]
    #[inline(always)]
    pub fn data12(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Rxppd3R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Rxppd3R_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Payload Data 13"]
    #[inline(always)]
    pub fn data13(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Rxppd3R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Rxppd3R_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Payload Data 14"]
    #[inline(always)]
    pub fn data14(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Rxppd3R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Rxppd3R_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Payload Data 15"]
    #[inline(always)]
    pub fn data15(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Rxppd3R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Rxppd3R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rxppd3R {
    #[inline(always)]
    fn default() -> Rxppd3R {
        <crate::RegValueT<Rxppd3R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hstxtosetr_SPEC;
impl crate::sealed::RegSpec for Hstxtosetr_SPEC {
    type DataType = u32;
}

#[doc = "HS TX Timeout Set Register"]
pub type Hstxtosetr = crate::RegValueT<Hstxtosetr_SPEC>;

impl Hstxtosetr {
    #[doc = "HS TX Timeout Count"]
    #[inline(always)]
    pub fn htxto(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Hstxtosetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Hstxtosetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Hstxtosetr {
    #[inline(always)]
    fn default() -> Hstxtosetr {
        <crate::RegValueT<Hstxtosetr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lrxhtosetr_SPEC;
impl crate::sealed::RegSpec for Lrxhtosetr_SPEC {
    type DataType = u32;
}

#[doc = "LRX-H Timeout Set Register"]
pub type Lrxhtosetr = crate::RegValueT<Lrxhtosetr_SPEC>;

impl Lrxhtosetr {
    #[doc = "LP-RX Host Processor Timeout"]
    #[inline(always)]
    pub fn lrxhto(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Lrxhtosetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Lrxhtosetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lrxhtosetr {
    #[inline(always)]
    fn default() -> Lrxhtosetr {
        <crate::RegValueT<Lrxhtosetr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tatosetr_SPEC;
impl crate::sealed::RegSpec for Tatosetr_SPEC {
    type DataType = u32;
}

#[doc = "TA Timeout Set Register"]
pub type Tatosetr = crate::RegValueT<Tatosetr_SPEC>;

impl Tatosetr {
    #[doc = "Turnaround Acknowledge Timeout"]
    #[inline(always)]
    pub fn tato(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Tatosetr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Tatosetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tatosetr {
    #[inline(always)]
    fn default() -> Tatosetr {
        <crate::RegValueT<Tatosetr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ferrsr_SPEC;
impl crate::sealed::RegSpec for Ferrsr_SPEC {
    type DataType = u32;
}

#[doc = "Fatal Error Status Register"]
pub type Ferrsr = crate::RegValueT<Ferrsr_SPEC>;

impl Ferrsr {
    #[doc = "HS TX Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn htxto(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ferrsr::Htxto,
        ferrsr::Htxto,
        Ferrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ferrsr::Htxto,
            ferrsr::Htxto,
            Ferrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "LP-RX Host Processor Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn lrxhto(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ferrsr::Lrxhto,
        ferrsr::Lrxhto,
        Ferrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ferrsr::Lrxhto,
            ferrsr::Lrxhto,
            Ferrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Turnaround Acknowledge Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn tato(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ferrsr::Tato,
        ferrsr::Tato,
        Ferrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ferrsr::Tato,
            ferrsr::Tato,
            Ferrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Escape mode Entry Error Interrupt Flag"]
    #[inline(always)]
    pub fn escent(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ferrsr::Escent,
        ferrsr::Escent,
        Ferrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ferrsr::Escent,
            ferrsr::Escent,
            Ferrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "LPDT Sync Error Interrupt Flag"]
    #[inline(always)]
    pub fn syncesc(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ferrsr::Syncesc,
        ferrsr::Syncesc,
        Ferrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ferrsr::Syncesc,
            ferrsr::Syncesc,
            Ferrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Control Error Interrupt Flag"]
    #[inline(always)]
    pub fn ctrl(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        ferrsr::Ctrl,
        ferrsr::Ctrl,
        Ferrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            ferrsr::Ctrl,
            ferrsr::Ctrl,
            Ferrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "LP0 Contention Error Interrupt Flag"]
    #[inline(always)]
    pub fn clp0(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        ferrsr::Clp0,
        ferrsr::Clp0,
        Ferrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            ferrsr::Clp0,
            ferrsr::Clp0,
            Ferrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "LP1 Contention Error Interrupt Flag"]
    #[inline(always)]
    pub fn clp1(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ferrsr::Clp1,
        ferrsr::Clp1,
        Ferrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ferrsr::Clp1,
            ferrsr::Clp1,
            Ferrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "LP0 Contention Error Status"]
    #[inline(always)]
    pub fn clp0s(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        ferrsr::Clp0S,
        ferrsr::Clp0S,
        Ferrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            ferrsr::Clp0S,
            ferrsr::Clp0S,
            Ferrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "LP1 Contention Error Status"]
    #[inline(always)]
    pub fn clp1s(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ferrsr::Clp1S,
        ferrsr::Clp1S,
        Ferrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ferrsr::Clp1S,
            ferrsr::Clp1S,
            Ferrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ferrsr {
    #[inline(always)]
    fn default() -> Ferrsr {
        <crate::RegValueT<Ferrsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ferrsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Htxto_SPEC;
    pub type Htxto = crate::EnumBitfieldStruct<u8, Htxto_SPEC>;
    impl Htxto {
        #[doc = "No detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "HS TX Timeout (HTX_TO) detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lrxhto_SPEC;
    pub type Lrxhto = crate::EnumBitfieldStruct<u8, Lrxhto_SPEC>;
    impl Lrxhto {
        #[doc = "No detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "LP-RX Host Processor Timeout (LRX-H_TO) detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tato_SPEC;
    pub type Tato = crate::EnumBitfieldStruct<u8, Tato_SPEC>;
    impl Tato {
        #[doc = "No detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Turnaround Acknowledge Timeout (TA_TO) detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Escent_SPEC;
    pub type Escent = crate::EnumBitfieldStruct<u8, Escent_SPEC>;
    impl Escent {
        #[doc = "No error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Escape mode Entry Error (ErrEsc) detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syncesc_SPEC;
    pub type Syncesc = crate::EnumBitfieldStruct<u8, Syncesc_SPEC>;
    impl Syncesc {
        #[doc = "No error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "LPDT Synchronization Error (ErrSyncEsc) detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctrl_SPEC;
    pub type Ctrl = crate::EnumBitfieldStruct<u8, Ctrl_SPEC>;
    impl Ctrl {
        #[doc = "No error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Control Error (ErrControl) detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clp0_SPEC;
    pub type Clp0 = crate::EnumBitfieldStruct<u8, Clp0_SPEC>;
    impl Clp0 {
        #[doc = "No error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "LP0 Contention Error (ErrContentionLP0) detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clp1_SPEC;
    pub type Clp1 = crate::EnumBitfieldStruct<u8, Clp1_SPEC>;
    impl Clp1 {
        #[doc = "No error detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "LP1 Contention Error (ErrContentionLP1) detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clp0S_SPEC;
    pub type Clp0S = crate::EnumBitfieldStruct<u8, Clp0S_SPEC>;
    impl Clp0S {
        #[doc = "Normal state"]
        pub const _0: Self = Self::new(0);

        #[doc = "LP0 Contention Error (ErrContentionLP0) state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clp1S_SPEC;
    pub type Clp1S = crate::EnumBitfieldStruct<u8, Clp1S_SPEC>;
    impl Clp1S {
        #[doc = "Normal state"]
        pub const _0: Self = Self::new(0);

        #[doc = "LP1 Contention Error (ErrContentionLP1) state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ferrscr_SPEC;
impl crate::sealed::RegSpec for Ferrscr_SPEC {
    type DataType = u32;
}

#[doc = "Fatal Error Status Clear Register"]
pub type Ferrscr = crate::RegValueT<Ferrscr_SPEC>;

impl Ferrscr {
    #[doc = "HS TX Timeout Interrupt Flag Clear"]
    #[inline(always)]
    pub fn htxto(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ferrscr::Htxto,
        ferrscr::Htxto,
        Ferrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ferrscr::Htxto,
            ferrscr::Htxto,
            Ferrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "LP-RX Host Processor Timeout Interrupt Flag Clear"]
    #[inline(always)]
    pub fn lrxhto(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ferrscr::Lrxhto,
        ferrscr::Lrxhto,
        Ferrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ferrscr::Lrxhto,
            ferrscr::Lrxhto,
            Ferrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Turnaround Acknowledge Timeout Interrupt Flag Clear"]
    #[inline(always)]
    pub fn tato(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ferrscr::Tato,
        ferrscr::Tato,
        Ferrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ferrscr::Tato,
            ferrscr::Tato,
            Ferrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Escape mode Entry Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn escent(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ferrscr::Escent,
        ferrscr::Escent,
        Ferrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ferrscr::Escent,
            ferrscr::Escent,
            Ferrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "LPDT Sync Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn syncesc(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ferrscr::Syncesc,
        ferrscr::Syncesc,
        Ferrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ferrscr::Syncesc,
            ferrscr::Syncesc,
            Ferrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Control Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn ctrl(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        ferrscr::Ctrl,
        ferrscr::Ctrl,
        Ferrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            ferrscr::Ctrl,
            ferrscr::Ctrl,
            Ferrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "LP0 Contention Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn clp0(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        ferrscr::Clp0,
        ferrscr::Clp0,
        Ferrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            ferrscr::Clp0,
            ferrscr::Clp0,
            Ferrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "LP1 Contention Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn clp1(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ferrscr::Clp1,
        ferrscr::Clp1,
        Ferrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ferrscr::Clp1,
            ferrscr::Clp1,
            Ferrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ferrscr {
    #[inline(always)]
    fn default() -> Ferrscr {
        <crate::RegValueT<Ferrscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ferrscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Htxto_SPEC;
    pub type Htxto = crate::EnumBitfieldStruct<u8, Htxto_SPEC>;
    impl Htxto {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the FERRSR.HTXTO flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lrxhto_SPEC;
    pub type Lrxhto = crate::EnumBitfieldStruct<u8, Lrxhto_SPEC>;
    impl Lrxhto {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the FERRSR.LRXHTO flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tato_SPEC;
    pub type Tato = crate::EnumBitfieldStruct<u8, Tato_SPEC>;
    impl Tato {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the FERRSR.TATO flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Escent_SPEC;
    pub type Escent = crate::EnumBitfieldStruct<u8, Escent_SPEC>;
    impl Escent {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the FERRSR.ESCENT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syncesc_SPEC;
    pub type Syncesc = crate::EnumBitfieldStruct<u8, Syncesc_SPEC>;
    impl Syncesc {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the FERRSR.SYNCESC flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctrl_SPEC;
    pub type Ctrl = crate::EnumBitfieldStruct<u8, Ctrl_SPEC>;
    impl Ctrl {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the FERRSR.CTRL flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clp0_SPEC;
    pub type Clp0 = crate::EnumBitfieldStruct<u8, Clp0_SPEC>;
    impl Clp0 {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the FERRSR.CLP0 flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clp1_SPEC;
    pub type Clp1 = crate::EnumBitfieldStruct<u8, Clp1_SPEC>;
    impl Clp1 {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the FERRSR.CLP1 flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ferrier_SPEC;
impl crate::sealed::RegSpec for Ferrier_SPEC {
    type DataType = u32;
}

#[doc = "Fatal Error Interrupt Enable Register"]
pub type Ferrier = crate::RegValueT<Ferrier_SPEC>;

impl Ferrier {
    #[doc = "HS TX Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn htxto(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ferrier::Htxto,
        ferrier::Htxto,
        Ferrier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ferrier::Htxto,
            ferrier::Htxto,
            Ferrier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LP-RX Host Processor Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn lrxhto(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ferrier::Lrxhto,
        ferrier::Lrxhto,
        Ferrier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ferrier::Lrxhto,
            ferrier::Lrxhto,
            Ferrier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Turnaround Acknowledge Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn tato(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ferrier::Tato,
        ferrier::Tato,
        Ferrier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ferrier::Tato,
            ferrier::Tato,
            Ferrier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Escape mode Entry Error Interrupt Enable"]
    #[inline(always)]
    pub fn escent(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ferrier::Escent,
        ferrier::Escent,
        Ferrier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ferrier::Escent,
            ferrier::Escent,
            Ferrier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LPDT Sync Error Interrupt Enable"]
    #[inline(always)]
    pub fn syncesc(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ferrier::Syncesc,
        ferrier::Syncesc,
        Ferrier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ferrier::Syncesc,
            ferrier::Syncesc,
            Ferrier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control Error Interrupt Enable"]
    #[inline(always)]
    pub fn ctrl(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        ferrier::Ctrl,
        ferrier::Ctrl,
        Ferrier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            ferrier::Ctrl,
            ferrier::Ctrl,
            Ferrier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LP0 Contention Error Interrupt Enable"]
    #[inline(always)]
    pub fn clp0(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        ferrier::Clp0,
        ferrier::Clp0,
        Ferrier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            ferrier::Clp0,
            ferrier::Clp0,
            Ferrier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LP1 Contention Error Interrupt Enable"]
    #[inline(always)]
    pub fn clp1(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ferrier::Clp1,
        ferrier::Clp1,
        Ferrier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ferrier::Clp1,
            ferrier::Clp1,
            Ferrier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ferrier {
    #[inline(always)]
    fn default() -> Ferrier {
        <crate::RegValueT<Ferrier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ferrier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Htxto_SPEC;
    pub type Htxto = crate::EnumBitfieldStruct<u8, Htxto_SPEC>;
    impl Htxto {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lrxhto_SPEC;
    pub type Lrxhto = crate::EnumBitfieldStruct<u8, Lrxhto_SPEC>;
    impl Lrxhto {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tato_SPEC;
    pub type Tato = crate::EnumBitfieldStruct<u8, Tato_SPEC>;
    impl Tato {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Escent_SPEC;
    pub type Escent = crate::EnumBitfieldStruct<u8, Escent_SPEC>;
    impl Escent {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syncesc_SPEC;
    pub type Syncesc = crate::EnumBitfieldStruct<u8, Syncesc_SPEC>;
    impl Syncesc {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctrl_SPEC;
    pub type Ctrl = crate::EnumBitfieldStruct<u8, Ctrl_SPEC>;
    impl Ctrl {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clp0_SPEC;
    pub type Clp0 = crate::EnumBitfieldStruct<u8, Clp0_SPEC>;
    impl Clp0 {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clp1_SPEC;
    pub type Clp1 = crate::EnumBitfieldStruct<u8, Clp1_SPEC>;
    impl Clp1 {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clstptsetr_SPEC;
impl crate::sealed::RegSpec for Clstptsetr_SPEC {
    type DataType = u32;
}

#[doc = "Clock Lane Stop Time Set Register"]
pub type Clstptsetr = crate::RegValueT<Clstptsetr_SPEC>;

impl Clstptsetr {
    #[doc = "Clock Stop Time"]
    #[inline(always)]
    pub fn clkstpt(
        self,
    ) -> crate::common::RegisterField<2, 0x3ff, 1, 0, u16, u16, Clstptsetr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3ff,1,0,u16,u16,Clstptsetr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock Beforehand Time"]
    #[inline(always)]
    pub fn clkbfht(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Clstptsetr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Clstptsetr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock Keep Time"]
    #[inline(always)]
    pub fn clkkpt(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Clstptsetr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Clstptsetr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Clstptsetr {
    #[inline(always)]
    fn default() -> Clstptsetr {
        <crate::RegValueT<Clstptsetr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lptrnstsetr_SPEC;
impl crate::sealed::RegSpec for Lptrnstsetr_SPEC {
    type DataType = u32;
}

#[doc = "LP Transition Time Set Register"]
pub type Lptrnstsetr = crate::RegValueT<Lptrnstsetr_SPEC>;

impl Lptrnstsetr {
    #[doc = "Go LP and Back Time"]
    #[inline(always)]
    pub fn golpbkt(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Lptrnstsetr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Lptrnstsetr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lptrnstsetr {
    #[inline(always)]
    fn default() -> Lptrnstsetr {
        <crate::RegValueT<Lptrnstsetr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plsr_SPEC;
impl crate::sealed::RegSpec for Plsr_SPEC {
    type DataType = u32;
}

#[doc = "Physical Lane Status Register"]
pub type Plsr = crate::RegValueT<Plsr_SPEC>;

impl Plsr {
    #[doc = "Clock Lane UlpsActiveNot Status"]
    #[inline(always)]
    pub fn cluan(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        plsr::Cluan,
        plsr::Cluan,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            plsr::Cluan,
            plsr::Cluan,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Clock Lane Stop Status"]
    #[inline(always)]
    pub fn clstp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        plsr::Clstp,
        plsr::Clstp,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            plsr::Clstp,
            plsr::Clstp,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane-0 RxLpdtEsc Status"]
    #[inline(always)]
    pub fn dl0rle(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        plsr::Dl0Rle,
        plsr::Dl0Rle,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            plsr::Dl0Rle,
            plsr::Dl0Rle,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane-0 RxUlpsEsc Status"]
    #[inline(always)]
    pub fn dl0rue(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        plsr::Dl0Rue,
        plsr::Dl0Rue,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            plsr::Dl0Rue,
            plsr::Dl0Rue,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane-0 UlpsActiveNot Status"]
    #[inline(always)]
    pub fn dl0uan(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        plsr::Dl0Uan,
        plsr::Dl0Uan,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            plsr::Dl0Uan,
            plsr::Dl0Uan,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane-1 UlpsActiveNot Status"]
    #[inline(always)]
    pub fn dl1uan(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        plsr::Dl1Uan,
        plsr::Dl1Uan,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            plsr::Dl1Uan,
            plsr::Dl1Uan,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane-0 Stop Status"]
    #[inline(always)]
    pub fn dl0stp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        plsr::Dl0Stp,
        plsr::Dl0Stp,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            plsr::Dl0Stp,
            plsr::Dl0Stp,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane-1 Stop Status"]
    #[inline(always)]
    pub fn dl1stp(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        plsr::Dl1Stp,
        plsr::Dl1Stp,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            plsr::Dl1Stp,
            plsr::Dl1Stp,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane-0 RX to TX Transition Interrupt Flag"]
    #[inline(always)]
    pub fn dl0rx2tx(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        plsr::Dl0Rx2Tx,
        plsr::Dl0Rx2Tx,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            plsr::Dl0Rx2Tx,
            plsr::Dl0Rx2Tx,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane-0 TX to RX Transition Interrupt Flag"]
    #[inline(always)]
    pub fn dl0tx2rx(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        plsr::Dl0Tx2Rx,
        plsr::Dl0Tx2Rx,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            plsr::Dl0Tx2Rx,
            plsr::Dl0Tx2Rx,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane-0 Direction"]
    #[inline(always)]
    pub fn dl0dir(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        plsr::Dl0Dir,
        plsr::Dl0Dir,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            plsr::Dl0Dir,
            plsr::Dl0Dir,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Clock Lane ULPS Enter Interrupt Flag"]
    #[inline(always)]
    pub fn clulpent(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        plsr::Clulpent,
        plsr::Clulpent,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            plsr::Clulpent,
            plsr::Clulpent,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Clock Lane ULPS Exit Interrupt Flag"]
    #[inline(always)]
    pub fn clulpext(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        plsr::Clulpext,
        plsr::Clulpext,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            plsr::Clulpext,
            plsr::Clulpext,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Clock Lane LP to HS Transition Interrupt Flag"]
    #[inline(always)]
    pub fn cllp2hs(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        plsr::Cllp2Hs,
        plsr::Cllp2Hs,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            plsr::Cllp2Hs,
            plsr::Cllp2Hs,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Clock Lane HS to LP Transition Interrupt Flag"]
    #[inline(always)]
    pub fn clhs2lp(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        plsr::Clhs2Lp,
        plsr::Clhs2Lp,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            plsr::Clhs2Lp,
            plsr::Clhs2Lp,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane ULPS Enter Interrupt Flag"]
    #[inline(always)]
    pub fn dlulpent(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        plsr::Dlulpent,
        plsr::Dlulpent,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            plsr::Dlulpent,
            plsr::Dlulpent,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane ULPS Exit Interrupt Flag"]
    #[inline(always)]
    pub fn dlulpext(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        plsr::Dlulpext,
        plsr::Dlulpext,
        Plsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            plsr::Dlulpext,
            plsr::Dlulpext,
            Plsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Plsr {
    #[inline(always)]
    fn default() -> Plsr {
        <crate::RegValueT<Plsr_SPEC> as RegisterValue<_>>::new(241)
    }
}
pub mod plsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cluan_SPEC;
    pub type Cluan = crate::EnumBitfieldStruct<u8, Cluan_SPEC>;
    impl Cluan {
        #[doc = "In ULP state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Not in ULP state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clstp_SPEC;
    pub type Clstp = crate::EnumBitfieldStruct<u8, Clstp_SPEC>;
    impl Clstp {
        #[doc = "Not in Stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "In Stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl0Rle_SPEC;
    pub type Dl0Rle = crate::EnumBitfieldStruct<u8, Dl0Rle_SPEC>;
    impl Dl0Rle {
        #[doc = "Not in Escape Low-Power Data Receive mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "In Escape Low-Power Data Receive mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl0Rue_SPEC;
    pub type Dl0Rue = crate::EnumBitfieldStruct<u8, Dl0Rue_SPEC>;
    impl Dl0Rue {
        #[doc = "Not in Escape Ultra-Low Power (Receive) mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "In Escape Ultra-Low Power (Receive) mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl0Uan_SPEC;
    pub type Dl0Uan = crate::EnumBitfieldStruct<u8, Dl0Uan_SPEC>;
    impl Dl0Uan {
        #[doc = "In ULP state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Not in ULP state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl1Uan_SPEC;
    pub type Dl1Uan = crate::EnumBitfieldStruct<u8, Dl1Uan_SPEC>;
    impl Dl1Uan {
        #[doc = "In ULP state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Not in ULP state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl0Stp_SPEC;
    pub type Dl0Stp = crate::EnumBitfieldStruct<u8, Dl0Stp_SPEC>;
    impl Dl0Stp {
        #[doc = "Not in Stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "In Stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl1Stp_SPEC;
    pub type Dl1Stp = crate::EnumBitfieldStruct<u8, Dl1Stp_SPEC>;
    impl Dl1Stp {
        #[doc = "Not in Stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "In Stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl0Rx2Tx_SPEC;
    pub type Dl0Rx2Tx = crate::EnumBitfieldStruct<u8, Dl0Rx2Tx_SPEC>;
    impl Dl0Rx2Tx {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl0Tx2Rx_SPEC;
    pub type Dl0Tx2Rx = crate::EnumBitfieldStruct<u8, Dl0Tx2Rx_SPEC>;
    impl Dl0Tx2Rx {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl0Dir_SPEC;
    pub type Dl0Dir = crate::EnumBitfieldStruct<u8, Dl0Dir_SPEC>;
    impl Dl0Dir {
        #[doc = "Transmitter"]
        pub const _0: Self = Self::new(0);

        #[doc = "Receiver"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clulpent_SPEC;
    pub type Clulpent = crate::EnumBitfieldStruct<u8, Clulpent_SPEC>;
    impl Clulpent {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clulpext_SPEC;
    pub type Clulpext = crate::EnumBitfieldStruct<u8, Clulpext_SPEC>;
    impl Clulpext {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cllp2Hs_SPEC;
    pub type Cllp2Hs = crate::EnumBitfieldStruct<u8, Cllp2Hs_SPEC>;
    impl Cllp2Hs {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clhs2Lp_SPEC;
    pub type Clhs2Lp = crate::EnumBitfieldStruct<u8, Clhs2Lp_SPEC>;
    impl Clhs2Lp {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlulpent_SPEC;
    pub type Dlulpent = crate::EnumBitfieldStruct<u8, Dlulpent_SPEC>;
    impl Dlulpent {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlulpext_SPEC;
    pub type Dlulpext = crate::EnumBitfieldStruct<u8, Dlulpext_SPEC>;
    impl Dlulpext {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plscr_SPEC;
impl crate::sealed::RegSpec for Plscr_SPEC {
    type DataType = u32;
}

#[doc = "Physical Lane Status Clear Register"]
pub type Plscr = crate::RegValueT<Plscr_SPEC>;

impl Plscr {
    #[doc = "Data Lane-0 RX to TX Transition Interrupt Flag Clear"]
    #[inline(always)]
    pub fn dl0rx2tx(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        plscr::Dl0Rx2Tx,
        plscr::Dl0Rx2Tx,
        Plscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            plscr::Dl0Rx2Tx,
            plscr::Dl0Rx2Tx,
            Plscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane-0 TX to RX Transition Interrupt Flag Clear"]
    #[inline(always)]
    pub fn dl0tx2rx(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        plscr::Dl0Tx2Rx,
        plscr::Dl0Tx2Rx,
        Plscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            plscr::Dl0Tx2Rx,
            plscr::Dl0Tx2Rx,
            Plscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Clock Lane ULPS Enter Interrupt Flag Clear"]
    #[inline(always)]
    pub fn clulpent(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        plscr::Clulpent,
        plscr::Clulpent,
        Plscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            plscr::Clulpent,
            plscr::Clulpent,
            Plscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Clock Lane ULPS Exit Interrupt Flag Clear"]
    #[inline(always)]
    pub fn clulpext(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        plscr::Clulpext,
        plscr::Clulpext,
        Plscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            plscr::Clulpext,
            plscr::Clulpext,
            Plscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Clock Lane LP to HS Transition Interrupt Flag Clear"]
    #[inline(always)]
    pub fn cllp2hs(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        plscr::Cllp2Hs,
        plscr::Cllp2Hs,
        Plscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            plscr::Cllp2Hs,
            plscr::Cllp2Hs,
            Plscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Clock Lane HS to LP Transition Interrupt Flag Clear"]
    #[inline(always)]
    pub fn clhs2lp(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        plscr::Clhs2Lp,
        plscr::Clhs2Lp,
        Plscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            plscr::Clhs2Lp,
            plscr::Clhs2Lp,
            Plscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane ULPS Enter Interrupt Flag Clear"]
    #[inline(always)]
    pub fn dlulpent(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        plscr::Dlulpent,
        plscr::Dlulpent,
        Plscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            plscr::Dlulpent,
            plscr::Dlulpent,
            Plscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane ULPS Exit Interrupt Flag Clear"]
    #[inline(always)]
    pub fn dlulpext(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        plscr::Dlulpext,
        plscr::Dlulpext,
        Plscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            plscr::Dlulpext,
            plscr::Dlulpext,
            Plscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Plscr {
    #[inline(always)]
    fn default() -> Plscr {
        <crate::RegValueT<Plscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod plscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl0Rx2Tx_SPEC;
    pub type Dl0Rx2Tx = crate::EnumBitfieldStruct<u8, Dl0Rx2Tx_SPEC>;
    impl Dl0Rx2Tx {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the PLSR.DL0RX2TX flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl0Tx2Rx_SPEC;
    pub type Dl0Tx2Rx = crate::EnumBitfieldStruct<u8, Dl0Tx2Rx_SPEC>;
    impl Dl0Tx2Rx {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the PLSR.DL0TX2RX flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clulpent_SPEC;
    pub type Clulpent = crate::EnumBitfieldStruct<u8, Clulpent_SPEC>;
    impl Clulpent {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the PLSR.CLULPENT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clulpext_SPEC;
    pub type Clulpext = crate::EnumBitfieldStruct<u8, Clulpext_SPEC>;
    impl Clulpext {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the PLSR.CLULPEXT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cllp2Hs_SPEC;
    pub type Cllp2Hs = crate::EnumBitfieldStruct<u8, Cllp2Hs_SPEC>;
    impl Cllp2Hs {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the PLSR.CLLP2HS flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clhs2Lp_SPEC;
    pub type Clhs2Lp = crate::EnumBitfieldStruct<u8, Clhs2Lp_SPEC>;
    impl Clhs2Lp {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the PLSR.CLHS2LP flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlulpent_SPEC;
    pub type Dlulpent = crate::EnumBitfieldStruct<u8, Dlulpent_SPEC>;
    impl Dlulpent {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the PLSR.DLULPENT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlulpext_SPEC;
    pub type Dlulpext = crate::EnumBitfieldStruct<u8, Dlulpext_SPEC>;
    impl Dlulpext {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the PLSR.DLULPEXT flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plier_SPEC;
impl crate::sealed::RegSpec for Plier_SPEC {
    type DataType = u32;
}

#[doc = "Physical Lane Interrupt Enable Register"]
pub type Plier = crate::RegValueT<Plier_SPEC>;

impl Plier {
    #[doc = "Data Lane-0 RX to TX Transition Interrupt Enable"]
    #[inline(always)]
    pub fn dl0rx2tx(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        plier::Dl0Rx2Tx,
        plier::Dl0Rx2Tx,
        Plier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            plier::Dl0Rx2Tx,
            plier::Dl0Rx2Tx,
            Plier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane-0 TX to RX Transition Interrupt Enable"]
    #[inline(always)]
    pub fn dl0tx2rx(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        plier::Dl0Tx2Rx,
        plier::Dl0Tx2Rx,
        Plier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            plier::Dl0Tx2Rx,
            plier::Dl0Tx2Rx,
            Plier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Lane ULPS Enter Interrupt Enable"]
    #[inline(always)]
    pub fn clulpent(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        plier::Clulpent,
        plier::Clulpent,
        Plier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            plier::Clulpent,
            plier::Clulpent,
            Plier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Lane ULPS Exit Interrupt Enable"]
    #[inline(always)]
    pub fn clulpext(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        plier::Clulpext,
        plier::Clulpext,
        Plier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            plier::Clulpext,
            plier::Clulpext,
            Plier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Lane LP to HS Transition Interrupt Enable"]
    #[inline(always)]
    pub fn cllp2hs(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        plier::Cllp2Hs,
        plier::Cllp2Hs,
        Plier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            plier::Cllp2Hs,
            plier::Cllp2Hs,
            Plier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Lane HS to LP Transition Interrupt Enable"]
    #[inline(always)]
    pub fn clhs2lp(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        plier::Clhs2Lp,
        plier::Clhs2Lp,
        Plier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            plier::Clhs2Lp,
            plier::Clhs2Lp,
            Plier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane ULPS Enter Interrupt Enable"]
    #[inline(always)]
    pub fn dlulpent(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        plier::Dlulpent,
        plier::Dlulpent,
        Plier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            plier::Dlulpent,
            plier::Dlulpent,
            Plier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Lane ULPS Exit Interrupt Enable"]
    #[inline(always)]
    pub fn dlulpext(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        plier::Dlulpext,
        plier::Dlulpext,
        Plier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            plier::Dlulpext,
            plier::Dlulpext,
            Plier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Plier {
    #[inline(always)]
    fn default() -> Plier {
        <crate::RegValueT<Plier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod plier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl0Rx2Tx_SPEC;
    pub type Dl0Rx2Tx = crate::EnumBitfieldStruct<u8, Dl0Rx2Tx_SPEC>;
    impl Dl0Rx2Tx {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl0Tx2Rx_SPEC;
    pub type Dl0Tx2Rx = crate::EnumBitfieldStruct<u8, Dl0Tx2Rx_SPEC>;
    impl Dl0Tx2Rx {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clulpent_SPEC;
    pub type Clulpent = crate::EnumBitfieldStruct<u8, Clulpent_SPEC>;
    impl Clulpent {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clulpext_SPEC;
    pub type Clulpext = crate::EnumBitfieldStruct<u8, Clulpext_SPEC>;
    impl Clulpext {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cllp2Hs_SPEC;
    pub type Cllp2Hs = crate::EnumBitfieldStruct<u8, Cllp2Hs_SPEC>;
    impl Cllp2Hs {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clhs2Lp_SPEC;
    pub type Clhs2Lp = crate::EnumBitfieldStruct<u8, Clhs2Lp_SPEC>;
    impl Clhs2Lp {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlulpent_SPEC;
    pub type Dlulpent = crate::EnumBitfieldStruct<u8, Dlulpent_SPEC>;
    impl Dlulpent {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlulpext_SPEC;
    pub type Dlulpext = crate::EnumBitfieldStruct<u8, Dlulpext_SPEC>;
    impl Dlulpext {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmset0R_SPEC;
impl crate::sealed::RegSpec for Vmset0R_SPEC {
    type DataType = u32;
}

#[doc = "Video Mode Set 0 Register"]
pub type Vmset0R = crate::RegValueT<Vmset0R_SPEC>;

impl Vmset0R {
    #[doc = "Video Mode Operation Start"]
    #[inline(always)]
    pub fn vstart(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vmset0r::Vstart,
        vmset0r::Vstart,
        Vmset0R_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vmset0r::Vstart,
            vmset0r::Vstart,
            Vmset0R_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Video Mode Operation Stop"]
    #[inline(always)]
    pub fn vstop(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vmset0r::Vstop,
        vmset0r::Vstop,
        Vmset0R_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vmset0r::Vstop,
            vmset0r::Vstop,
            Vmset0R_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "HSA period No LP"]
    #[inline(always)]
    pub fn hsanolp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        vmset0r::Hsanolp,
        vmset0r::Hsanolp,
        Vmset0R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            vmset0r::Hsanolp,
            vmset0r::Hsanolp,
            Vmset0R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HBP period No LP"]
    #[inline(always)]
    pub fn hbpnolp(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        vmset0r::Hbpnolp,
        vmset0r::Hbpnolp,
        Vmset0R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            vmset0r::Hbpnolp,
            vmset0r::Hbpnolp,
            Vmset0R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HFP period No LP"]
    #[inline(always)]
    pub fn hfpnolp(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        vmset0r::Hfpnolp,
        vmset0r::Hfpnolp,
        Vmset0R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            vmset0r::Hfpnolp,
            vmset0r::Hfpnolp,
            Vmset0R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vmset0R {
    #[inline(always)]
    fn default() -> Vmset0R {
        <crate::RegValueT<Vmset0R_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vmset0r {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vstart_SPEC;
    pub type Vstart = crate::EnumBitfieldStruct<u8, Vstart_SPEC>;
    impl Vstart {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start Video mode operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vstop_SPEC;
    pub type Vstop = crate::EnumBitfieldStruct<u8, Vstop_SPEC>;
    impl Vstop {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop Video mode operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsanolp_SPEC;
    pub type Hsanolp = crate::EnumBitfieldStruct<u8, Hsanolp_SPEC>;
    impl Hsanolp {
        #[doc = "Not suppressing the transition to LP during HSA period"]
        pub const _0: Self = Self::new(0);

        #[doc = "Suppress the transition to LP during HSA period and keep HS"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hbpnolp_SPEC;
    pub type Hbpnolp = crate::EnumBitfieldStruct<u8, Hbpnolp_SPEC>;
    impl Hbpnolp {
        #[doc = "Not suppressing the transition to LP during HBP period"]
        pub const _0: Self = Self::new(0);

        #[doc = "Suppress the transition to LP during HBP period and keep HS"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hfpnolp_SPEC;
    pub type Hfpnolp = crate::EnumBitfieldStruct<u8, Hfpnolp_SPEC>;
    impl Hfpnolp {
        #[doc = "Not suppressing the transition to LP during HFP period"]
        pub const _0: Self = Self::new(0);

        #[doc = "Suppress the transition to LP during HFP period and keep HS"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmset1R_SPEC;
impl crate::sealed::RegSpec for Vmset1R_SPEC {
    type DataType = u32;
}

#[doc = "Video Mode Set 1 Register"]
pub type Vmset1R = crate::RegValueT<Vmset1R_SPEC>;

impl Vmset1R {
    #[doc = "Delay Value"]
    #[inline(always)]
    pub fn dly(
        self,
    ) -> crate::common::RegisterField<2, 0xfff, 1, 0, u16, u16, Vmset1R_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0xfff,1,0,u16,u16,Vmset1R_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vmset1R {
    #[inline(always)]
    fn default() -> Vmset1R {
        <crate::RegValueT<Vmset1R_SPEC> as RegisterValue<_>>::new(273874944)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmsr_SPEC;
impl crate::sealed::RegSpec for Vmsr_SPEC {
    type DataType = u32;
}

#[doc = "Video Mode Status Register"]
pub type Vmsr = crate::RegValueT<Vmsr_SPEC>;

impl Vmsr {
    #[doc = "Video Mode Operation Start Interrupt Flag"]
    #[inline(always)]
    pub fn start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vmsr::Start,
        vmsr::Start,
        Vmsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vmsr::Start,
            vmsr::Start,
            Vmsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Video Mode Operation Stop Interrupt Flag"]
    #[inline(always)]
    pub fn stop(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vmsr::Stop,
        vmsr::Stop,
        Vmsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vmsr::Stop,
            vmsr::Stop,
            Vmsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Video Mode Operation Running Status"]
    #[inline(always)]
    pub fn running(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        vmsr::Running,
        vmsr::Running,
        Vmsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            vmsr::Running,
            vmsr::Running,
            Vmsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Video Mode Operation Ready Interrupt Flag"]
    #[inline(always)]
    pub fn virdy(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        vmsr::Virdy,
        vmsr::Virdy,
        Vmsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            vmsr::Virdy,
            vmsr::Virdy,
            Vmsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Timing Error Interrupt Flag"]
    #[inline(always)]
    pub fn timerr(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        vmsr::Timerr,
        vmsr::Timerr,
        Vmsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            vmsr::Timerr,
            vmsr::Timerr,
            Vmsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Video Buffer Underflow Error Interrupt Flag"]
    #[inline(always)]
    pub fn vbufudf(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        vmsr::Vbufudf,
        vmsr::Vbufudf,
        Vmsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            vmsr::Vbufudf,
            vmsr::Vbufudf,
            Vmsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Video Buffer Overflow Error Interrupt Flag"]
    #[inline(always)]
    pub fn vbufovf(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        vmsr::Vbufovf,
        vmsr::Vbufovf,
        Vmsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            vmsr::Vbufovf,
            vmsr::Vbufovf,
            Vmsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vmsr {
    #[inline(always)]
    fn default() -> Vmsr {
        <crate::RegValueT<Vmsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vmsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Start_SPEC;
    pub type Start = crate::EnumBitfieldStruct<u8, Start_SPEC>;
    impl Start {
        #[doc = "Video mode operation has not started"]
        pub const _0: Self = Self::new(0);

        #[doc = "Video mode operation has started"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stop_SPEC;
    pub type Stop = crate::EnumBitfieldStruct<u8, Stop_SPEC>;
    impl Stop {
        #[doc = "Video mode operation has not stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Video mode operation has stopped"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Running_SPEC;
    pub type Running = crate::EnumBitfieldStruct<u8, Running_SPEC>;
    impl Running {
        #[doc = "Video mode operation is stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Video mode operation is running"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Virdy_SPEC;
    pub type Virdy = crate::EnumBitfieldStruct<u8, Virdy_SPEC>;
    impl Virdy {
        #[doc = "Video mode operation is not ready"]
        pub const _0: Self = Self::new(0);

        #[doc = "Video mode operation is ready"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Timerr_SPEC;
    pub type Timerr = crate::EnumBitfieldStruct<u8, Timerr_SPEC>;
    impl Timerr {
        #[doc = "Timing error is not occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Timing error is occurred during the video mode operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbufudf_SPEC;
    pub type Vbufudf = crate::EnumBitfieldStruct<u8, Vbufudf_SPEC>;
    impl Vbufudf {
        #[doc = "Data underflow is not occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data underflow is occurred in the video buffer"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbufovf_SPEC;
    pub type Vbufovf = crate::EnumBitfieldStruct<u8, Vbufovf_SPEC>;
    impl Vbufovf {
        #[doc = "Data overflow is not occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data overflow is occurred in the video buffer"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmscr_SPEC;
impl crate::sealed::RegSpec for Vmscr_SPEC {
    type DataType = u32;
}

#[doc = "Video Mode Status Clear Register"]
pub type Vmscr = crate::RegValueT<Vmscr_SPEC>;

impl Vmscr {
    #[doc = "Video Mode Operation Start Interrupt Flag Clear"]
    #[inline(always)]
    pub fn start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vmscr::Start,
        vmscr::Start,
        Vmscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vmscr::Start,
            vmscr::Start,
            Vmscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Video Mode Operation Stop Interrupt Flag Clear"]
    #[inline(always)]
    pub fn stop(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vmscr::Stop,
        vmscr::Stop,
        Vmscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vmscr::Stop,
            vmscr::Stop,
            Vmscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Video Mode Operation Ready Interrupt Flag Clear"]
    #[inline(always)]
    pub fn virdy(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        vmscr::Virdy,
        vmscr::Virdy,
        Vmscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            vmscr::Virdy,
            vmscr::Virdy,
            Vmscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Timing Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn timerr(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        vmscr::Timerr,
        vmscr::Timerr,
        Vmscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            vmscr::Timerr,
            vmscr::Timerr,
            Vmscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Video Buffer Underflow Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn vbufudf(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        vmscr::Vbufudf,
        vmscr::Vbufudf,
        Vmscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            vmscr::Vbufudf,
            vmscr::Vbufudf,
            Vmscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Video Buffer Overflow Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn vbufovf(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        vmscr::Vbufovf,
        vmscr::Vbufovf,
        Vmscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            vmscr::Vbufovf,
            vmscr::Vbufovf,
            Vmscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vmscr {
    #[inline(always)]
    fn default() -> Vmscr {
        <crate::RegValueT<Vmscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vmscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Start_SPEC;
    pub type Start = crate::EnumBitfieldStruct<u8, Start_SPEC>;
    impl Start {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the VMSR.START flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stop_SPEC;
    pub type Stop = crate::EnumBitfieldStruct<u8, Stop_SPEC>;
    impl Stop {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the VMSR.STOP flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Virdy_SPEC;
    pub type Virdy = crate::EnumBitfieldStruct<u8, Virdy_SPEC>;
    impl Virdy {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the VMSR.VIRDY flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Timerr_SPEC;
    pub type Timerr = crate::EnumBitfieldStruct<u8, Timerr_SPEC>;
    impl Timerr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the VMSR.TIMERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbufudf_SPEC;
    pub type Vbufudf = crate::EnumBitfieldStruct<u8, Vbufudf_SPEC>;
    impl Vbufudf {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the VMSR.VBUFUDF flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbufovf_SPEC;
    pub type Vbufovf = crate::EnumBitfieldStruct<u8, Vbufovf_SPEC>;
    impl Vbufovf {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the VMSR.VBUFOVF flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmier_SPEC;
impl crate::sealed::RegSpec for Vmier_SPEC {
    type DataType = u32;
}

#[doc = "Video Mode Interrupt Enable Register"]
pub type Vmier = crate::RegValueT<Vmier_SPEC>;

impl Vmier {
    #[doc = "Video Mode Operation Start Interrupt Enable"]
    #[inline(always)]
    pub fn start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vmier::Start,
        vmier::Start,
        Vmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vmier::Start,
            vmier::Start,
            Vmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Video Mode Operation Stop Interrupt Enable"]
    #[inline(always)]
    pub fn stop(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vmier::Stop,
        vmier::Stop,
        Vmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vmier::Stop,
            vmier::Stop,
            Vmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Video Mode Operation Ready Interrupt Enable"]
    #[inline(always)]
    pub fn virdy(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        vmier::Virdy,
        vmier::Virdy,
        Vmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            vmier::Virdy,
            vmier::Virdy,
            Vmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing Error Interrupt Enable"]
    #[inline(always)]
    pub fn timerr(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        vmier::Timerr,
        vmier::Timerr,
        Vmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            vmier::Timerr,
            vmier::Timerr,
            Vmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Video Buffer Underflow Error Interrupt Enable"]
    #[inline(always)]
    pub fn vbufudf(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        vmier::Vbufudf,
        vmier::Vbufudf,
        Vmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            vmier::Vbufudf,
            vmier::Vbufudf,
            Vmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Video Buffer Overflow Error Interrupt Enable"]
    #[inline(always)]
    pub fn vbufovf(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        vmier::Vbufovf,
        vmier::Vbufovf,
        Vmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            vmier::Vbufovf,
            vmier::Vbufovf,
            Vmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vmier {
    #[inline(always)]
    fn default() -> Vmier {
        <crate::RegValueT<Vmier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vmier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Start_SPEC;
    pub type Start = crate::EnumBitfieldStruct<u8, Start_SPEC>;
    impl Start {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stop_SPEC;
    pub type Stop = crate::EnumBitfieldStruct<u8, Stop_SPEC>;
    impl Stop {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Virdy_SPEC;
    pub type Virdy = crate::EnumBitfieldStruct<u8, Virdy_SPEC>;
    impl Virdy {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Timerr_SPEC;
    pub type Timerr = crate::EnumBitfieldStruct<u8, Timerr_SPEC>;
    impl Timerr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbufudf_SPEC;
    pub type Vbufudf = crate::EnumBitfieldStruct<u8, Vbufudf_SPEC>;
    impl Vbufudf {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbufovf_SPEC;
    pub type Vbufovf = crate::EnumBitfieldStruct<u8, Vbufovf_SPEC>;
    impl Vbufovf {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmppsetr_SPEC;
impl crate::sealed::RegSpec for Vmppsetr_SPEC {
    type DataType = u32;
}

#[doc = "Video Mode Pixel Packet Set Register"]
pub type Vmppsetr = crate::RegValueT<Vmppsetr_SPEC>;

impl Vmppsetr {
    #[doc = "Transmit End of Sync Pulse"]
    #[inline(always)]
    pub fn txesync(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        vmppsetr::Txesync,
        vmppsetr::Txesync,
        Vmppsetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            vmppsetr::Txesync,
            vmppsetr::Txesync,
            Vmppsetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Video Mode Data Type"]
    #[inline(always)]
    pub fn dt(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3f,
        1,
        0,
        vmppsetr::Dt,
        vmppsetr::Dt,
        Vmppsetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3f,
            1,
            0,
            vmppsetr::Dt,
            vmppsetr::Dt,
            Vmppsetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Video Mode Virtual Channel"]
    #[inline(always)]
    pub fn vc(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, u8, Vmppsetr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8,u8,Vmppsetr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vmppsetr {
    #[inline(always)]
    fn default() -> Vmppsetr {
        <crate::RegValueT<Vmppsetr_SPEC> as RegisterValue<_>>::new(917504)
    }
}
pub mod vmppsetr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txesync_SPEC;
    pub type Txesync = crate::EnumBitfieldStruct<u8, Txesync_SPEC>;
    impl Txesync {
        #[doc = "HSE and VSE are not transmitted"]
        pub const _0: Self = Self::new(0);

        #[doc = "HSE and VSE are transmitted"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dt_SPEC;
    pub type Dt = crate::EnumBitfieldStruct<u8, Dt_SPEC>;
    impl Dt {
        #[doc = "Packed Pixel Stream, 16 bit RGB"]
        pub const _0_X_0_E: Self = Self::new(14);

        #[doc = "Packed Pixel Stream, 18 bit RGB"]
        pub const _0_X_1_E: Self = Self::new(30);

        #[doc = "Packed Pixel Stream, 24 bit RGB"]
        pub const _0_X_3_E: Self = Self::new(62);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmvssetr_SPEC;
impl crate::sealed::RegSpec for Vmvssetr_SPEC {
    type DataType = u32;
}

#[doc = "Video Mode Vertical Size Set Register"]
pub type Vmvssetr = crate::RegValueT<Vmvssetr_SPEC>;

impl Vmvssetr {
    #[doc = "VSA Lines"]
    #[inline(always)]
    pub fn vsa(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Vmvssetr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Vmvssetr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VSYNC Polarity"]
    #[inline(always)]
    pub fn vspol(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        vmvssetr::Vspol,
        vmvssetr::Vspol,
        Vmvssetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            vmvssetr::Vspol,
            vmvssetr::Vspol,
            Vmvssetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Vertical Active Lines"]
    #[inline(always)]
    pub fn vact(
        self,
    ) -> crate::common::RegisterField<16, 0x7fff, 1, 0, u16, u16, Vmvssetr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7fff,1,0,u16,u16,Vmvssetr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vmvssetr {
    #[inline(always)]
    fn default() -> Vmvssetr {
        <crate::RegValueT<Vmvssetr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vmvssetr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vspol_SPEC;
    pub type Vspol = crate::EnumBitfieldStruct<u8, Vspol_SPEC>;
    impl Vspol {
        #[doc = "High Active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low Active"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmvpsetr_SPEC;
impl crate::sealed::RegSpec for Vmvpsetr_SPEC {
    type DataType = u32;
}

#[doc = "Video Mode Vertical Porch Set Register"]
pub type Vmvpsetr = crate::RegValueT<Vmvpsetr_SPEC>;

impl Vmvpsetr {
    #[doc = "VBP Lines"]
    #[inline(always)]
    pub fn vbp(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Vmvpsetr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Vmvpsetr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VFP Lines"]
    #[inline(always)]
    pub fn vfp(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, u16, Vmvpsetr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1fff,1,0,u16,u16,Vmvpsetr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vmvpsetr {
    #[inline(always)]
    fn default() -> Vmvpsetr {
        <crate::RegValueT<Vmvpsetr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmhssetr_SPEC;
impl crate::sealed::RegSpec for Vmhssetr_SPEC {
    type DataType = u32;
}

#[doc = "Video Mode Horizontal Size Set Register"]
pub type Vmhssetr = crate::RegValueT<Vmhssetr_SPEC>;

impl Vmhssetr {
    #[doc = "HSA Pixels"]
    #[inline(always)]
    pub fn hsa(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Vmhssetr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Vmhssetr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "HSYNC Polarity"]
    #[inline(always)]
    pub fn hspol(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        vmhssetr::Hspol,
        vmhssetr::Hspol,
        Vmhssetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            vmhssetr::Hspol,
            vmhssetr::Hspol,
            Vmhssetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HACT Pixels"]
    #[inline(always)]
    pub fn hact(
        self,
    ) -> crate::common::RegisterField<16, 0x7fff, 1, 0, u16, u16, Vmhssetr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7fff,1,0,u16,u16,Vmhssetr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vmhssetr {
    #[inline(always)]
    fn default() -> Vmhssetr {
        <crate::RegValueT<Vmhssetr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vmhssetr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hspol_SPEC;
    pub type Hspol = crate::EnumBitfieldStruct<u8, Hspol_SPEC>;
    impl Hspol {
        #[doc = "High Active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low Active"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vmhpsetr_SPEC;
impl crate::sealed::RegSpec for Vmhpsetr_SPEC {
    type DataType = u32;
}

#[doc = "Video Mode Horizontal Porch Set Register"]
pub type Vmhpsetr = crate::RegValueT<Vmhpsetr_SPEC>;

impl Vmhpsetr {
    #[doc = "HBP Pixels"]
    #[inline(always)]
    pub fn hbp(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Vmhpsetr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Vmhpsetr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "HFP Pixels"]
    #[inline(always)]
    pub fn hfp(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, u16, Vmhpsetr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1fff,1,0,u16,u16,Vmhpsetr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vmhpsetr {
    #[inline(always)]
    fn default() -> Vmhpsetr {
        <crate::RegValueT<Vmhpsetr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch0Set0R_SPEC;
impl crate::sealed::RegSpec for Sqch0Set0R_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 0 Set 0 Register"]
pub type Sqch0Set0R = crate::RegValueT<Sqch0Set0R_SPEC>;

impl Sqch0Set0R {
    #[doc = "Sequence Operation Start"]
    #[inline(always)]
    pub fn start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sqch0set0r::Start,
        sqch0set0r::Start,
        Sqch0Set0R_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sqch0set0r::Start,
            sqch0set0r::Start,
            Sqch0Set0R_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sqch0Set0R {
    #[inline(always)]
    fn default() -> Sqch0Set0R {
        <crate::RegValueT<Sqch0Set0R_SPEC> as RegisterValue<_>>::new(8388608)
    }
}
pub mod sqch0set0r {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Start_SPEC;
    pub type Start = crate::EnumBitfieldStruct<u8, Start_SPEC>;
    impl Start {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start the sequence operation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch0Sr_SPEC;
impl crate::sealed::RegSpec for Sqch0Sr_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 0 Status Register"]
pub type Sqch0Sr = crate::RegValueT<Sqch0Sr_SPEC>;

impl Sqch0Sr {
    #[doc = "Sequence Operation Running Status"]
    #[inline(always)]
    pub fn running(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sqch0sr::Running,
        sqch0sr::Running,
        Sqch0Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sqch0sr::Running,
            sqch0sr::Running,
            Sqch0Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "All Actions Finish Interrupt Flag"]
    #[inline(always)]
    pub fn aactfin(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sqch0sr::Aactfin,
        sqch0sr::Aactfin,
        Sqch0Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sqch0sr::Aactfin,
            sqch0sr::Aactfin,
            Sqch0Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "All-Descriptors Finish Interrupt Flag"]
    #[inline(always)]
    pub fn adesfin(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sqch0sr::Adesfin,
        sqch0sr::Adesfin,
        Sqch0Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sqch0sr::Adesfin,
            sqch0sr::Adesfin,
            Sqch0Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Tx Internal Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn txiberr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        sqch0sr::Txiberr,
        sqch0sr::Txiberr,
        Sqch0Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            sqch0sr::Txiberr,
            sqch0sr::Txiberr,
            Sqch0Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Fatal Error Interrupt Flag"]
    #[inline(always)]
    pub fn rxferr(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        sqch0sr::Rxferr,
        sqch0sr::Rxferr,
        Sqch0Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            sqch0sr::Rxferr,
            sqch0sr::Rxferr,
            Sqch0Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Fail Interrupt Flag"]
    #[inline(always)]
    pub fn rxfail(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        sqch0sr::Rxfail,
        sqch0sr::Rxfail,
        Sqch0Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            sqch0sr::Rxfail,
            sqch0sr::Rxfail,
            Sqch0Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Packet Data Fail Interrupt Flag"]
    #[inline(always)]
    pub fn rxpfail(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        sqch0sr::Rxpfail,
        sqch0sr::Rxpfail,
        Sqch0Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            sqch0sr::Rxpfail,
            sqch0sr::Rxpfail,
            Sqch0Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Correctable Error Interrupt Flag"]
    #[inline(always)]
    pub fn rxcorerr(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        sqch0sr::Rxcorerr,
        sqch0sr::Rxcorerr,
        Sqch0Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            sqch0sr::Rxcorerr,
            sqch0sr::Rxcorerr,
            Sqch0Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Acknowledge and Error Report Packet Interrupt Flag"]
    #[inline(always)]
    pub fn rxake(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        sqch0sr::Rxake,
        sqch0sr::Rxake,
        Sqch0Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            sqch0sr::Rxake,
            sqch0sr::Rxake,
            Sqch0Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sqch0Sr {
    #[inline(always)]
    fn default() -> Sqch0Sr {
        <crate::RegValueT<Sqch0Sr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sqch0sr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Running_SPEC;
    pub type Running = crate::EnumBitfieldStruct<u8, Running_SPEC>;
    impl Running {
        #[doc = "Sequence operation is stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sequence operation is running"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aactfin_SPEC;
    pub type Aactfin = crate::EnumBitfieldStruct<u8, Aactfin_SPEC>;
    impl Aactfin {
        #[doc = "All actions of a descriptor have not finished"]
        pub const _0: Self = Self::new(0);

        #[doc = "All actions of a descriptor finished operations"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adesfin_SPEC;
    pub type Adesfin = crate::EnumBitfieldStruct<u8, Adesfin_SPEC>;
    impl Adesfin {
        #[doc = "All descriptors have not finished"]
        pub const _0: Self = Self::new(0);

        #[doc = "All descriptors finished operations"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txiberr_SPEC;
    pub type Txiberr = crate::EnumBitfieldStruct<u8, Txiberr_SPEC>;
    impl Txiberr {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal AXI bus read error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxferr_SPEC;
    pub type Rxferr = crate::EnumBitfieldStruct<u8, Rxferr_SPEC>;
    impl Rxferr {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Fatal timeout occurred during BTA"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxfail_SPEC;
    pub type Rxfail = crate::EnumBitfieldStruct<u8, Rxfail_SPEC>;
    impl Rxfail {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Expected receive did not take place"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxpfail_SPEC;
    pub type Rxpfail = crate::EnumBitfieldStruct<u8, Rxpfail_SPEC>;
    impl Rxpfail {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received packet data is not stored correctly"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxcorerr_SPEC;
    pub type Rxcorerr = crate::EnumBitfieldStruct<u8, Rxcorerr_SPEC>;
    impl Rxcorerr {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received packets have correctable error"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxake_SPEC;
    pub type Rxake = crate::EnumBitfieldStruct<u8, Rxake_SPEC>;
    impl Rxake {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Acknowledge and Error Report Packet is received"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch0Scr_SPEC;
impl crate::sealed::RegSpec for Sqch0Scr_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 0 Status Clear Register"]
pub type Sqch0Scr = crate::RegValueT<Sqch0Scr_SPEC>;

impl Sqch0Scr {
    #[doc = "All Actions Finish Interrupt Flag Clear"]
    #[inline(always)]
    pub fn aactfin(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sqch0scr::Aactfin,
        sqch0scr::Aactfin,
        Sqch0Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sqch0scr::Aactfin,
            sqch0scr::Aactfin,
            Sqch0Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "All-Descriptors Finish Interrupt Flag Clear"]
    #[inline(always)]
    pub fn adesfin(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sqch0scr::Adesfin,
        sqch0scr::Adesfin,
        Sqch0Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sqch0scr::Adesfin,
            sqch0scr::Adesfin,
            Sqch0Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Tx Internal Bus Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn txiberr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        sqch0scr::Txiberr,
        sqch0scr::Txiberr,
        Sqch0Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            sqch0scr::Txiberr,
            sqch0scr::Txiberr,
            Sqch0Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Receive Fatal Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxferr(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        sqch0scr::Rxferr,
        sqch0scr::Rxferr,
        Sqch0Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            sqch0scr::Rxferr,
            sqch0scr::Rxferr,
            Sqch0Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Receive Fail Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxfail(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        sqch0scr::Rxfail,
        sqch0scr::Rxfail,
        Sqch0Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            sqch0scr::Rxfail,
            sqch0scr::Rxfail,
            Sqch0Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Receive Packet Data Fail Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxpfail(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        sqch0scr::Rxpfail,
        sqch0scr::Rxpfail,
        Sqch0Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            sqch0scr::Rxpfail,
            sqch0scr::Rxpfail,
            Sqch0Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Receive Correctable Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxcorerr(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        sqch0scr::Rxcorerr,
        sqch0scr::Rxcorerr,
        Sqch0Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            sqch0scr::Rxcorerr,
            sqch0scr::Rxcorerr,
            Sqch0Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Receive Acknowledge and Error Report Packet Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxake(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        sqch0scr::Rxake,
        sqch0scr::Rxake,
        Sqch0Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            sqch0scr::Rxake,
            sqch0scr::Rxake,
            Sqch0Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sqch0Scr {
    #[inline(always)]
    fn default() -> Sqch0Scr {
        <crate::RegValueT<Sqch0Scr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sqch0scr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aactfin_SPEC;
    pub type Aactfin = crate::EnumBitfieldStruct<u8, Aactfin_SPEC>;
    impl Aactfin {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH0SR.AACTFIN flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adesfin_SPEC;
    pub type Adesfin = crate::EnumBitfieldStruct<u8, Adesfin_SPEC>;
    impl Adesfin {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH0SR.ADESFIN flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txiberr_SPEC;
    pub type Txiberr = crate::EnumBitfieldStruct<u8, Txiberr_SPEC>;
    impl Txiberr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH0SR.TXIBERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxferr_SPEC;
    pub type Rxferr = crate::EnumBitfieldStruct<u8, Rxferr_SPEC>;
    impl Rxferr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH0SR.RXFERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxfail_SPEC;
    pub type Rxfail = crate::EnumBitfieldStruct<u8, Rxfail_SPEC>;
    impl Rxfail {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH0SR.RXFAIL flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxpfail_SPEC;
    pub type Rxpfail = crate::EnumBitfieldStruct<u8, Rxpfail_SPEC>;
    impl Rxpfail {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH0SR.RXPFAIL flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxcorerr_SPEC;
    pub type Rxcorerr = crate::EnumBitfieldStruct<u8, Rxcorerr_SPEC>;
    impl Rxcorerr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH0SR.RXCORERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxake_SPEC;
    pub type Rxake = crate::EnumBitfieldStruct<u8, Rxake_SPEC>;
    impl Rxake {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH0SR.RXAKE flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch0Ier_SPEC;
impl crate::sealed::RegSpec for Sqch0Ier_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 0 Interrupt Enable Register"]
pub type Sqch0Ier = crate::RegValueT<Sqch0Ier_SPEC>;

impl Sqch0Ier {
    #[doc = "All Actions Finish Interrupt Enable"]
    #[inline(always)]
    pub fn aactfin(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sqch0ier::Aactfin,
        sqch0ier::Aactfin,
        Sqch0Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sqch0ier::Aactfin,
            sqch0ier::Aactfin,
            Sqch0Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "All-Descriptors Finish Interrupt Enable"]
    #[inline(always)]
    pub fn adesfin(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sqch0ier::Adesfin,
        sqch0ier::Adesfin,
        Sqch0Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sqch0ier::Adesfin,
            sqch0ier::Adesfin,
            Sqch0Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Tx Internal Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn txiberr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        sqch0ier::Txiberr,
        sqch0ier::Txiberr,
        Sqch0Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            sqch0ier::Txiberr,
            sqch0ier::Txiberr,
            Sqch0Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Fatal Error Interrupt Enable"]
    #[inline(always)]
    pub fn rxferr(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        sqch0ier::Rxferr,
        sqch0ier::Rxferr,
        Sqch0Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            sqch0ier::Rxferr,
            sqch0ier::Rxferr,
            Sqch0Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Fail Interrupt Enable"]
    #[inline(always)]
    pub fn rxfail(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        sqch0ier::Rxfail,
        sqch0ier::Rxfail,
        Sqch0Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            sqch0ier::Rxfail,
            sqch0ier::Rxfail,
            Sqch0Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Packet Data Fail Interrupt Enable"]
    #[inline(always)]
    pub fn rxpfail(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        sqch0ier::Rxpfail,
        sqch0ier::Rxpfail,
        Sqch0Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            sqch0ier::Rxpfail,
            sqch0ier::Rxpfail,
            Sqch0Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Correctable Error Interrupt Enable"]
    #[inline(always)]
    pub fn rxcorerr(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        sqch0ier::Rxcorerr,
        sqch0ier::Rxcorerr,
        Sqch0Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            sqch0ier::Rxcorerr,
            sqch0ier::Rxcorerr,
            Sqch0Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Acknowledge and Error Report Packet Interrupt Enable"]
    #[inline(always)]
    pub fn rxake(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        sqch0ier::Rxake,
        sqch0ier::Rxake,
        Sqch0Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            sqch0ier::Rxake,
            sqch0ier::Rxake,
            Sqch0Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sqch0Ier {
    #[inline(always)]
    fn default() -> Sqch0Ier {
        <crate::RegValueT<Sqch0Ier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sqch0ier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aactfin_SPEC;
    pub type Aactfin = crate::EnumBitfieldStruct<u8, Aactfin_SPEC>;
    impl Aactfin {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adesfin_SPEC;
    pub type Adesfin = crate::EnumBitfieldStruct<u8, Adesfin_SPEC>;
    impl Adesfin {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txiberr_SPEC;
    pub type Txiberr = crate::EnumBitfieldStruct<u8, Txiberr_SPEC>;
    impl Txiberr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxferr_SPEC;
    pub type Rxferr = crate::EnumBitfieldStruct<u8, Rxferr_SPEC>;
    impl Rxferr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxfail_SPEC;
    pub type Rxfail = crate::EnumBitfieldStruct<u8, Rxfail_SPEC>;
    impl Rxfail {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxpfail_SPEC;
    pub type Rxpfail = crate::EnumBitfieldStruct<u8, Rxpfail_SPEC>;
    impl Rxpfail {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxcorerr_SPEC;
    pub type Rxcorerr = crate::EnumBitfieldStruct<u8, Rxcorerr_SPEC>;
    impl Rxcorerr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxake_SPEC;
    pub type Rxake = crate::EnumBitfieldStruct<u8, Rxake_SPEC>;
    impl Rxake {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch1Set0R_SPEC;
impl crate::sealed::RegSpec for Sqch1Set0R_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 1 Set 0 Register"]
pub type Sqch1Set0R = crate::RegValueT<Sqch1Set0R_SPEC>;

impl Sqch1Set0R {
    #[doc = "Sequence Operation Start"]
    #[inline(always)]
    pub fn start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sqch1set0r::Start,
        sqch1set0r::Start,
        Sqch1Set0R_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sqch1set0r::Start,
            sqch1set0r::Start,
            Sqch1Set0R_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sqch1Set0R {
    #[inline(always)]
    fn default() -> Sqch1Set0R {
        <crate::RegValueT<Sqch1Set0R_SPEC> as RegisterValue<_>>::new(8388608)
    }
}
pub mod sqch1set0r {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Start_SPEC;
    pub type Start = crate::EnumBitfieldStruct<u8, Start_SPEC>;
    impl Start {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start the sequence operation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch1Sr_SPEC;
impl crate::sealed::RegSpec for Sqch1Sr_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 1 Status Register"]
pub type Sqch1Sr = crate::RegValueT<Sqch1Sr_SPEC>;

impl Sqch1Sr {
    #[doc = "Sequence Operation Running Status"]
    #[inline(always)]
    pub fn running(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sqch1sr::Running,
        sqch1sr::Running,
        Sqch1Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sqch1sr::Running,
            sqch1sr::Running,
            Sqch1Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "All Actions Finish Interrupt Flag"]
    #[inline(always)]
    pub fn aactfin(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sqch1sr::Aactfin,
        sqch1sr::Aactfin,
        Sqch1Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sqch1sr::Aactfin,
            sqch1sr::Aactfin,
            Sqch1Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "All-Descriptors Finish Interrupt Flag"]
    #[inline(always)]
    pub fn adesfin(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sqch1sr::Adesfin,
        sqch1sr::Adesfin,
        Sqch1Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sqch1sr::Adesfin,
            sqch1sr::Adesfin,
            Sqch1Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Packet Size Error Interrupt Flag"]
    #[inline(always)]
    pub fn sizeerr(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        sqch1sr::Sizeerr,
        sqch1sr::Sizeerr,
        Sqch1Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            sqch1sr::Sizeerr,
            sqch1sr::Sizeerr,
            Sqch1Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Tx Internal Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn txiberr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        sqch1sr::Txiberr,
        sqch1sr::Txiberr,
        Sqch1Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            sqch1sr::Txiberr,
            sqch1sr::Txiberr,
            Sqch1Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Fatal Error Interrupt Flag"]
    #[inline(always)]
    pub fn rxferr(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        sqch1sr::Rxferr,
        sqch1sr::Rxferr,
        Sqch1Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            sqch1sr::Rxferr,
            sqch1sr::Rxferr,
            Sqch1Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Fail Interrupt Flag"]
    #[inline(always)]
    pub fn rxfail(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        sqch1sr::Rxfail,
        sqch1sr::Rxfail,
        Sqch1Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            sqch1sr::Rxfail,
            sqch1sr::Rxfail,
            Sqch1Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Packet Data Fail Interrupt Flag"]
    #[inline(always)]
    pub fn rxpfail(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        sqch1sr::Rxpfail,
        sqch1sr::Rxpfail,
        Sqch1Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            sqch1sr::Rxpfail,
            sqch1sr::Rxpfail,
            Sqch1Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Correctable Error Interrupt Flag"]
    #[inline(always)]
    pub fn rxcorerr(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        sqch1sr::Rxcorerr,
        sqch1sr::Rxcorerr,
        Sqch1Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            sqch1sr::Rxcorerr,
            sqch1sr::Rxcorerr,
            Sqch1Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Acknowledge and Error Report Packet Interrupt Flag"]
    #[inline(always)]
    pub fn rxake(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        sqch1sr::Rxake,
        sqch1sr::Rxake,
        Sqch1Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            sqch1sr::Rxake,
            sqch1sr::Rxake,
            Sqch1Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sqch1Sr {
    #[inline(always)]
    fn default() -> Sqch1Sr {
        <crate::RegValueT<Sqch1Sr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sqch1sr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Running_SPEC;
    pub type Running = crate::EnumBitfieldStruct<u8, Running_SPEC>;
    impl Running {
        #[doc = "Sequence operation is stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sequence operation is running"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aactfin_SPEC;
    pub type Aactfin = crate::EnumBitfieldStruct<u8, Aactfin_SPEC>;
    impl Aactfin {
        #[doc = "All actions of a descriptor have not finished"]
        pub const _0: Self = Self::new(0);

        #[doc = "All actions of a descriptor finished operations"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adesfin_SPEC;
    pub type Adesfin = crate::EnumBitfieldStruct<u8, Adesfin_SPEC>;
    impl Adesfin {
        #[doc = "All descriptors have not finished"]
        pub const _0: Self = Self::new(0);

        #[doc = "All descriptors finished operations"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sizeerr_SPEC;
    pub type Sizeerr = crate::EnumBitfieldStruct<u8, Sizeerr_SPEC>;
    impl Sizeerr {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sequence packet size is too large"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txiberr_SPEC;
    pub type Txiberr = crate::EnumBitfieldStruct<u8, Txiberr_SPEC>;
    impl Txiberr {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal AXI bus read error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxferr_SPEC;
    pub type Rxferr = crate::EnumBitfieldStruct<u8, Rxferr_SPEC>;
    impl Rxferr {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Fatal timeout occurred during BTA"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxfail_SPEC;
    pub type Rxfail = crate::EnumBitfieldStruct<u8, Rxfail_SPEC>;
    impl Rxfail {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Expected receive did not take place"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxpfail_SPEC;
    pub type Rxpfail = crate::EnumBitfieldStruct<u8, Rxpfail_SPEC>;
    impl Rxpfail {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received packet data is not stored correctly"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxcorerr_SPEC;
    pub type Rxcorerr = crate::EnumBitfieldStruct<u8, Rxcorerr_SPEC>;
    impl Rxcorerr {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received packets have correctable error"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxake_SPEC;
    pub type Rxake = crate::EnumBitfieldStruct<u8, Rxake_SPEC>;
    impl Rxake {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Acknowledge and Error Report Packet is received"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch1Scr_SPEC;
impl crate::sealed::RegSpec for Sqch1Scr_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 1 Status Clear Register"]
pub type Sqch1Scr = crate::RegValueT<Sqch1Scr_SPEC>;

impl Sqch1Scr {
    #[doc = "All Actions Finish Interrupt Flag Clear"]
    #[inline(always)]
    pub fn aactfin(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sqch1scr::Aactfin,
        sqch1scr::Aactfin,
        Sqch1Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sqch1scr::Aactfin,
            sqch1scr::Aactfin,
            Sqch1Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "All-Descriptors Finish Interrupt Flag Clear"]
    #[inline(always)]
    pub fn adesfin(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sqch1scr::Adesfin,
        sqch1scr::Adesfin,
        Sqch1Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sqch1scr::Adesfin,
            sqch1scr::Adesfin,
            Sqch1Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Packet Size Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn sizeerr(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        sqch1scr::Sizeerr,
        sqch1scr::Sizeerr,
        Sqch1Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            sqch1scr::Sizeerr,
            sqch1scr::Sizeerr,
            Sqch1Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Tx Internal Bus Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn txiberr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        sqch1scr::Txiberr,
        sqch1scr::Txiberr,
        Sqch1Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            sqch1scr::Txiberr,
            sqch1scr::Txiberr,
            Sqch1Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Receive Fatal Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxferr(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        sqch1scr::Rxferr,
        sqch1scr::Rxferr,
        Sqch1Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            sqch1scr::Rxferr,
            sqch1scr::Rxferr,
            Sqch1Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Receive Fail Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxfail(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        sqch1scr::Rxfail,
        sqch1scr::Rxfail,
        Sqch1Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            sqch1scr::Rxfail,
            sqch1scr::Rxfail,
            Sqch1Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Receive Packet Data Fail Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxpfail(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        sqch1scr::Rxpfail,
        sqch1scr::Rxpfail,
        Sqch1Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            sqch1scr::Rxpfail,
            sqch1scr::Rxpfail,
            Sqch1Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Receive Correctable Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxcorerr(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        sqch1scr::Rxcorerr,
        sqch1scr::Rxcorerr,
        Sqch1Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            sqch1scr::Rxcorerr,
            sqch1scr::Rxcorerr,
            Sqch1Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Receive Acknowledge and Error Report Packet Interrupt Flag Clear"]
    #[inline(always)]
    pub fn rxake(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        sqch1scr::Rxake,
        sqch1scr::Rxake,
        Sqch1Scr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            sqch1scr::Rxake,
            sqch1scr::Rxake,
            Sqch1Scr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sqch1Scr {
    #[inline(always)]
    fn default() -> Sqch1Scr {
        <crate::RegValueT<Sqch1Scr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sqch1scr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aactfin_SPEC;
    pub type Aactfin = crate::EnumBitfieldStruct<u8, Aactfin_SPEC>;
    impl Aactfin {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH1SR.AACTFIN flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adesfin_SPEC;
    pub type Adesfin = crate::EnumBitfieldStruct<u8, Adesfin_SPEC>;
    impl Adesfin {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH1SR.ADESFIN flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sizeerr_SPEC;
    pub type Sizeerr = crate::EnumBitfieldStruct<u8, Sizeerr_SPEC>;
    impl Sizeerr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH1SR.SIZEERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txiberr_SPEC;
    pub type Txiberr = crate::EnumBitfieldStruct<u8, Txiberr_SPEC>;
    impl Txiberr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH1SR.TXIBERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxferr_SPEC;
    pub type Rxferr = crate::EnumBitfieldStruct<u8, Rxferr_SPEC>;
    impl Rxferr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH1SR.RXFERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxfail_SPEC;
    pub type Rxfail = crate::EnumBitfieldStruct<u8, Rxfail_SPEC>;
    impl Rxfail {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH1SR.RXFAIL flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxpfail_SPEC;
    pub type Rxpfail = crate::EnumBitfieldStruct<u8, Rxpfail_SPEC>;
    impl Rxpfail {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH1SR.RXPFAIL flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxcorerr_SPEC;
    pub type Rxcorerr = crate::EnumBitfieldStruct<u8, Rxcorerr_SPEC>;
    impl Rxcorerr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH1SR.RXCORERR flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxake_SPEC;
    pub type Rxake = crate::EnumBitfieldStruct<u8, Rxake_SPEC>;
    impl Rxake {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the SQCH1SR.RXAKE flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch1Ier_SPEC;
impl crate::sealed::RegSpec for Sqch1Ier_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 1 Interrupt Enable Register"]
pub type Sqch1Ier = crate::RegValueT<Sqch1Ier_SPEC>;

impl Sqch1Ier {
    #[doc = "All Actions Finish Interrupt Enable"]
    #[inline(always)]
    pub fn aactfin(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sqch1ier::Aactfin,
        sqch1ier::Aactfin,
        Sqch1Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sqch1ier::Aactfin,
            sqch1ier::Aactfin,
            Sqch1Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "All-Descriptors Finish Interrupt Enable"]
    #[inline(always)]
    pub fn adesfin(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sqch1ier::Adesfin,
        sqch1ier::Adesfin,
        Sqch1Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sqch1ier::Adesfin,
            sqch1ier::Adesfin,
            Sqch1Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Packet Size Error Interrupt Enable"]
    #[inline(always)]
    pub fn sizeerr(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        sqch1ier::Sizeerr,
        sqch1ier::Sizeerr,
        Sqch1Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            sqch1ier::Sizeerr,
            sqch1ier::Sizeerr,
            Sqch1Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Tx Internal Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn txiberr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        sqch1ier::Txiberr,
        sqch1ier::Txiberr,
        Sqch1Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            sqch1ier::Txiberr,
            sqch1ier::Txiberr,
            Sqch1Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Fatal Error Interrupt Enable"]
    #[inline(always)]
    pub fn rxferr(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        sqch1ier::Rxferr,
        sqch1ier::Rxferr,
        Sqch1Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            sqch1ier::Rxferr,
            sqch1ier::Rxferr,
            Sqch1Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Fail Interrupt Enable"]
    #[inline(always)]
    pub fn rxfail(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        sqch1ier::Rxfail,
        sqch1ier::Rxfail,
        Sqch1Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            sqch1ier::Rxfail,
            sqch1ier::Rxfail,
            Sqch1Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Packet Data Fail Interrupt Enable"]
    #[inline(always)]
    pub fn rxpfail(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        sqch1ier::Rxpfail,
        sqch1ier::Rxpfail,
        Sqch1Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            sqch1ier::Rxpfail,
            sqch1ier::Rxpfail,
            Sqch1Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Correctable Error Interrupt Enable"]
    #[inline(always)]
    pub fn rxcorerr(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        sqch1ier::Rxcorerr,
        sqch1ier::Rxcorerr,
        Sqch1Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            sqch1ier::Rxcorerr,
            sqch1ier::Rxcorerr,
            Sqch1Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Acknowledge and Error Report Packet Interrupt Enable"]
    #[inline(always)]
    pub fn rxake(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        sqch1ier::Rxake,
        sqch1ier::Rxake,
        Sqch1Ier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            sqch1ier::Rxake,
            sqch1ier::Rxake,
            Sqch1Ier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sqch1Ier {
    #[inline(always)]
    fn default() -> Sqch1Ier {
        <crate::RegValueT<Sqch1Ier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sqch1ier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aactfin_SPEC;
    pub type Aactfin = crate::EnumBitfieldStruct<u8, Aactfin_SPEC>;
    impl Aactfin {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adesfin_SPEC;
    pub type Adesfin = crate::EnumBitfieldStruct<u8, Adesfin_SPEC>;
    impl Adesfin {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sizeerr_SPEC;
    pub type Sizeerr = crate::EnumBitfieldStruct<u8, Sizeerr_SPEC>;
    impl Sizeerr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txiberr_SPEC;
    pub type Txiberr = crate::EnumBitfieldStruct<u8, Txiberr_SPEC>;
    impl Txiberr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxferr_SPEC;
    pub type Rxferr = crate::EnumBitfieldStruct<u8, Rxferr_SPEC>;
    impl Rxferr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxfail_SPEC;
    pub type Rxfail = crate::EnumBitfieldStruct<u8, Rxfail_SPEC>;
    impl Rxfail {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxpfail_SPEC;
    pub type Rxpfail = crate::EnumBitfieldStruct<u8, Rxpfail_SPEC>;
    impl Rxpfail {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxcorerr_SPEC;
    pub type Rxcorerr = crate::EnumBitfieldStruct<u8, Rxcorerr_SPEC>;
    impl Rxcorerr {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxake_SPEC;
    pub type Rxake = crate::EnumBitfieldStruct<u8, Rxake_SPEC>;
    impl Rxake {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch0Dscar_SPEC;
impl crate::sealed::RegSpec for Sqch0Dscar_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 0 Descriptor-m A Register (m = 0 to 7)"]
pub type Sqch0Dscar = crate::RegValueT<Sqch0Dscar_SPEC>;

impl Sqch0Dscar {
    #[doc = "Data 0"]
    #[inline(always)]
    pub fn data0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Sqch0Dscar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Sqch0Dscar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data 1"]
    #[inline(always)]
    pub fn data1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Sqch0Dscar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Sqch0Dscar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data Type"]
    #[inline(always)]
    pub fn dt(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Sqch0Dscar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,Sqch0Dscar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Virtual Channel"]
    #[inline(always)]
    pub fn vc(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, u8, Sqch0Dscar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3,1,0,u8,u8,Sqch0Dscar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Format"]
    #[inline(always)]
    pub fn fmt(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        sqch0dscar::Fmt,
        sqch0dscar::Fmt,
        Sqch0Dscar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            sqch0dscar::Fmt,
            sqch0dscar::Fmt,
            Sqch0Dscar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Speed"]
    #[inline(always)]
    pub fn spd(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        sqch0dscar::Spd,
        sqch0dscar::Spd,
        Sqch0Dscar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            sqch0dscar::Spd,
            sqch0dscar::Spd,
            Sqch0Dscar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Turn Around"]
    #[inline(always)]
    pub fn bta(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x3,
        1,
        0,
        sqch0dscar::Bta,
        sqch0dscar::Bta,
        Sqch0Dscar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x3,
            1,
            0,
            sqch0dscar::Bta,
            sqch0dscar::Bta,
            Sqch0Dscar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Next Action"]
    #[inline(always)]
    pub fn nxact(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        sqch0dscar::Nxact,
        sqch0dscar::Nxact,
        Sqch0Dscar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            sqch0dscar::Nxact,
            sqch0dscar::Nxact,
            Sqch0Dscar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sqch0Dscar {
    #[inline(always)]
    fn default() -> Sqch0Dscar {
        <crate::RegValueT<Sqch0Dscar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sqch0dscar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fmt_SPEC;
    pub type Fmt = crate::EnumBitfieldStruct<u8, Fmt_SPEC>;
    impl Fmt {
        #[doc = "Short Packet"]
        pub const _0: Self = Self::new(0);

        #[doc = "Long Packet"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spd_SPEC;
    pub type Spd = crate::EnumBitfieldStruct<u8, Spd_SPEC>;
    impl Spd {
        #[doc = "High Speed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low Power"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bta_SPEC;
    pub type Bta = crate::EnumBitfieldStruct<u8, Bta_SPEC>;
    impl Bta {
        #[doc = "Tx Request without BTA or no-operation"]
        pub const _00: Self = Self::new(0);

        #[doc = "Tx non-Read Request with BTA"]
        pub const _01: Self = Self::new(1);

        #[doc = "Tx Read Request with BTA"]
        pub const _10: Self = Self::new(2);

        #[doc = "BTA only"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nxact_SPEC;
    pub type Nxact = crate::EnumBitfieldStruct<u8, Nxact_SPEC>;
    impl Nxact {
        #[doc = "Terminate the sequence operation after this descriptor processing is finished."]
        pub const _00: Self = Self::new(0);

        #[doc = "Start the next descriptor processing after this descriptor processing is finished."]
        pub const _01: Self = Self::new(1);

        #[doc = "Settings prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch0Dscbr_SPEC;
impl crate::sealed::RegSpec for Sqch0Dscbr_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 0 Descriptor-m B Register (m = 0 to 7)"]
pub type Sqch0Dscbr = crate::RegValueT<Sqch0Dscbr_SPEC>;

impl Sqch0Dscbr {
    #[doc = "Data Select"]
    #[inline(always)]
    pub fn dtsel(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        sqch0dscbr::Dtsel,
        sqch0dscbr::Dtsel,
        Sqch0Dscbr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            sqch0dscbr::Dtsel,
            sqch0dscbr::Dtsel,
            Sqch0Dscbr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sqch0Dscbr {
    #[inline(always)]
    fn default() -> Sqch0Dscbr {
        <crate::RegValueT<Sqch0Dscbr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sqch0dscbr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtsel_SPEC;
    pub type Dtsel = crate::EnumBitfieldStruct<u8, Dtsel_SPEC>;
    impl Dtsel {
        #[doc = "Use Packet Payload Data Register (TXPPDxR, RXPPDxR)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Use Memory space"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch0Dsccr_SPEC;
impl crate::sealed::RegSpec for Sqch0Dsccr_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 0 Descriptor-m C Register"]
pub type Sqch0Dsccr = crate::RegValueT<Sqch0Dsccr_SPEC>;

impl Sqch0Dsccr {
    #[doc = "Finish Action"]
    #[inline(always)]
    pub fn finact(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sqch0dsccr::Finact,
        sqch0dsccr::Finact,
        Sqch0Dsccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sqch0dsccr::Finact,
            sqch0dsccr::Finact,
            Sqch0Dsccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Auxiliary Operation"]
    #[inline(always)]
    pub fn auxop(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        sqch0dsccr::Auxop,
        sqch0dsccr::Auxop,
        Sqch0Dsccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            sqch0dsccr::Auxop,
            sqch0dsccr::Auxop,
            Sqch0Dsccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Action Code"]
    #[inline(always)]
    pub fn actcode(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Sqch0Dsccr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Sqch0Dsccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sqch0Dsccr {
    #[inline(always)]
    fn default() -> Sqch0Dsccr {
        <crate::RegValueT<Sqch0Dsccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sqch0dsccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Finact_SPEC;
    pub type Finact = crate::EnumBitfieldStruct<u8, Finact_SPEC>;
    impl Finact {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Auxop_SPEC;
    pub type Auxop = crate::EnumBitfieldStruct<u8, Auxop_SPEC>;
    impl Auxop {
        #[doc = "Not use auxiliary operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Execute auxiliary operation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch0Dscdr_SPEC;
impl crate::sealed::RegSpec for Sqch0Dscdr_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 0 Descriptor-m D Register (m = 0 to 7)"]
pub type Sqch0Dscdr = crate::RegValueT<Sqch0Dscdr_SPEC>;

impl Sqch0Dscdr {
    #[doc = "Lower Address"]
    #[inline(always)]
    pub fn laddr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Sqch0Dscdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Sqch0Dscdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sqch0Dscdr {
    #[inline(always)]
    fn default() -> Sqch0Dscdr {
        <crate::RegValueT<Sqch0Dscdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch1Dscar_SPEC;
impl crate::sealed::RegSpec for Sqch1Dscar_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 1 Descriptor-m A Register (m = 0 to 7)"]
pub type Sqch1Dscar = crate::RegValueT<Sqch1Dscar_SPEC>;

impl Sqch1Dscar {
    #[doc = "Data 0"]
    #[inline(always)]
    pub fn data0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Sqch1Dscar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Sqch1Dscar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data 1"]
    #[inline(always)]
    pub fn data1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Sqch1Dscar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Sqch1Dscar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data Type"]
    #[inline(always)]
    pub fn dt(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Sqch1Dscar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,Sqch1Dscar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Virtual Channel"]
    #[inline(always)]
    pub fn vc(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, u8, Sqch1Dscar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3,1,0,u8,u8,Sqch1Dscar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Format"]
    #[inline(always)]
    pub fn fmt(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        sqch1dscar::Fmt,
        sqch1dscar::Fmt,
        Sqch1Dscar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            sqch1dscar::Fmt,
            sqch1dscar::Fmt,
            Sqch1Dscar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Speed"]
    #[inline(always)]
    pub fn spd(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        sqch1dscar::Spd,
        sqch1dscar::Spd,
        Sqch1Dscar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            sqch1dscar::Spd,
            sqch1dscar::Spd,
            Sqch1Dscar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Turn Around"]
    #[inline(always)]
    pub fn bta(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x3,
        1,
        0,
        sqch1dscar::Bta,
        sqch1dscar::Bta,
        Sqch1Dscar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x3,
            1,
            0,
            sqch1dscar::Bta,
            sqch1dscar::Bta,
            Sqch1Dscar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Next Action"]
    #[inline(always)]
    pub fn nxact(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        sqch1dscar::Nxact,
        sqch1dscar::Nxact,
        Sqch1Dscar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            sqch1dscar::Nxact,
            sqch1dscar::Nxact,
            Sqch1Dscar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sqch1Dscar {
    #[inline(always)]
    fn default() -> Sqch1Dscar {
        <crate::RegValueT<Sqch1Dscar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sqch1dscar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fmt_SPEC;
    pub type Fmt = crate::EnumBitfieldStruct<u8, Fmt_SPEC>;
    impl Fmt {
        #[doc = "Short Packet"]
        pub const _0: Self = Self::new(0);

        #[doc = "Long Packet"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spd_SPEC;
    pub type Spd = crate::EnumBitfieldStruct<u8, Spd_SPEC>;
    impl Spd {
        #[doc = "High Speed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low Power"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bta_SPEC;
    pub type Bta = crate::EnumBitfieldStruct<u8, Bta_SPEC>;
    impl Bta {
        #[doc = "Tx Request without BTA or no-operation"]
        pub const _00: Self = Self::new(0);

        #[doc = "Tx non-Read Request with BTA"]
        pub const _01: Self = Self::new(1);

        #[doc = "Tx Read Request with BTA"]
        pub const _10: Self = Self::new(2);

        #[doc = "BTA only"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nxact_SPEC;
    pub type Nxact = crate::EnumBitfieldStruct<u8, Nxact_SPEC>;
    impl Nxact {
        #[doc = "Terminate the sequence operation after this descriptor processing is finished."]
        pub const _00: Self = Self::new(0);

        #[doc = "Start the next descriptor processing after this descriptor processing is finished."]
        pub const _01: Self = Self::new(1);

        #[doc = "Settings prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch1Dscbr_SPEC;
impl crate::sealed::RegSpec for Sqch1Dscbr_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 1 Descriptor-m B Register (m = 0 to 7)"]
pub type Sqch1Dscbr = crate::RegValueT<Sqch1Dscbr_SPEC>;

impl Sqch1Dscbr {
    #[doc = "Data Select"]
    #[inline(always)]
    pub fn dtsel(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        sqch1dscbr::Dtsel,
        sqch1dscbr::Dtsel,
        Sqch1Dscbr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            sqch1dscbr::Dtsel,
            sqch1dscbr::Dtsel,
            Sqch1Dscbr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sqch1Dscbr {
    #[inline(always)]
    fn default() -> Sqch1Dscbr {
        <crate::RegValueT<Sqch1Dscbr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sqch1dscbr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtsel_SPEC;
    pub type Dtsel = crate::EnumBitfieldStruct<u8, Dtsel_SPEC>;
    impl Dtsel {
        #[doc = "Use Packet Payload Data Register (TXPPDxR, RXPPDxR)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Use Memory space"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch1Dsccr_SPEC;
impl crate::sealed::RegSpec for Sqch1Dsccr_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 1 Descriptor-m C Register"]
pub type Sqch1Dsccr = crate::RegValueT<Sqch1Dsccr_SPEC>;

impl Sqch1Dsccr {
    #[doc = "Finish Action"]
    #[inline(always)]
    pub fn finact(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sqch1dsccr::Finact,
        sqch1dsccr::Finact,
        Sqch1Dsccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sqch1dsccr::Finact,
            sqch1dsccr::Finact,
            Sqch1Dsccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Auxiliary Operation"]
    #[inline(always)]
    pub fn auxop(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        sqch1dsccr::Auxop,
        sqch1dsccr::Auxop,
        Sqch1Dsccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            sqch1dsccr::Auxop,
            sqch1dsccr::Auxop,
            Sqch1Dsccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Action Code"]
    #[inline(always)]
    pub fn actcode(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Sqch1Dsccr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Sqch1Dsccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sqch1Dsccr {
    #[inline(always)]
    fn default() -> Sqch1Dsccr {
        <crate::RegValueT<Sqch1Dsccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sqch1dsccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Finact_SPEC;
    pub type Finact = crate::EnumBitfieldStruct<u8, Finact_SPEC>;
    impl Finact {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Auxop_SPEC;
    pub type Auxop = crate::EnumBitfieldStruct<u8, Auxop_SPEC>;
    impl Auxop {
        #[doc = "Not use auxiliary operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Execute auxiliary operation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqch1Dscdr_SPEC;
impl crate::sealed::RegSpec for Sqch1Dscdr_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Channel 1 Descriptor-m D Register (m = 0 to 7)"]
pub type Sqch1Dscdr = crate::RegValueT<Sqch1Dscdr_SPEC>;

impl Sqch1Dscdr {
    #[doc = "Lower Address"]
    #[inline(always)]
    pub fn laddr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Sqch1Dscdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Sqch1Dscdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sqch1Dscdr {
    #[inline(always)]
    fn default() -> Sqch1Dscdr {
        <crate::RegValueT<Sqch1Dscdr_SPEC> as RegisterValue<_>>::new(0)
    }
}
