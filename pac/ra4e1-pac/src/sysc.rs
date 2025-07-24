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
// Generated from SVD 1.20.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:48:16 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"System Control"]
unsafe impl ::core::marker::Send for super::Sysc {}
unsafe impl ::core::marker::Sync for super::Sysc {}
impl super::Sysc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Standby Control Register"]
    #[inline(always)]
    pub const fn sbycr(&self) -> &'static crate::common::Reg<self::Sbycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sbycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "System Clock Division Control Register"]
    #[inline(always)]
    pub const fn sckdivcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sckdivcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sckdivcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "System Clock Source Control Register"]
    #[inline(always)]
    pub const fn sckscr(
        &self,
    ) -> &'static crate::common::Reg<self::Sckscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sckscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[doc = "PLL Clock Control Register"]
    #[inline(always)]
    pub const fn pllccr(
        &self,
    ) -> &'static crate::common::Reg<self::Pllccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pllccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "PLL Control Register"]
    #[inline(always)]
    pub const fn pllcr(&self) -> &'static crate::common::Reg<self::Pllcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pllcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[doc = "Main Clock Oscillator Control Register"]
    #[inline(always)]
    pub const fn mosccr(
        &self,
    ) -> &'static crate::common::Reg<self::Mosccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mosccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "High-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn hococr(
        &self,
    ) -> &'static crate::common::Reg<self::Hococr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hococr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[doc = "High-Speed On-Chip Oscillator Control Register 2"]
    #[inline(always)]
    pub const fn hococr2(
        &self,
    ) -> &'static crate::common::Reg<self::Hococr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hococr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(55usize),
            )
        }
    }

    #[doc = "Middle-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn mococr(
        &self,
    ) -> &'static crate::common::Reg<self::Mococr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mococr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "FLL Control Register 1"]
    #[inline(always)]
    pub const fn fllcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Fllcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fllcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(57usize),
            )
        }
    }

    #[doc = "FLL Control Register 2"]
    #[inline(always)]
    pub const fn fllcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Fllcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fllcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(58usize),
            )
        }
    }

    #[doc = "Oscillation Stabilization Flag Register"]
    #[inline(always)]
    pub const fn oscsf(&self) -> &'static crate::common::Reg<self::Oscsf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Oscsf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Clock Out Control Register"]
    #[inline(always)]
    pub const fn ckocr(&self) -> &'static crate::common::Reg<self::Ckocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ckocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(62usize),
            )
        }
    }

    #[doc = "Trace Clock Control Register"]
    #[inline(always)]
    pub const fn trckcr(
        &self,
    ) -> &'static crate::common::Reg<self::Trckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Trckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(63usize),
            )
        }
    }

    #[doc = "Oscillation Stop Detection Control Register"]
    #[inline(always)]
    pub const fn ostdcr(
        &self,
    ) -> &'static crate::common::Reg<self::Ostdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ostdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Oscillation Stop Detection Status Register"]
    #[inline(always)]
    pub const fn ostdsr(
        &self,
    ) -> &'static crate::common::Reg<self::Ostdsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ostdsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65usize),
            )
        }
    }

    #[doc = "PLL2 Clock Control Register"]
    #[inline(always)]
    pub const fn pll2ccr(
        &self,
    ) -> &'static crate::common::Reg<self::Pll2Ccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pll2Ccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "PLL2 Control Register"]
    #[inline(always)]
    pub const fn pll2cr(
        &self,
    ) -> &'static crate::common::Reg<self::Pll2Cr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pll2Cr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(74usize),
            )
        }
    }

    #[doc = "MOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn mocoutcr(
        &self,
    ) -> &'static crate::common::Reg<self::Mocoutcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mocoutcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(97usize),
            )
        }
    }

    #[doc = "HOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn hocoutcr(
        &self,
    ) -> &'static crate::common::Reg<self::Hocoutcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hocoutcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(98usize),
            )
        }
    }

    #[doc = "USB Clock Division Control Register"]
    #[inline(always)]
    pub const fn usbckdivcr(
        &self,
    ) -> &'static crate::common::Reg<self::Usbckdivcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Usbckdivcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "USB Clock Control Register"]
    #[inline(always)]
    pub const fn usbckcr(
        &self,
    ) -> &'static crate::common::Reg<self::Usbckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Usbckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "Snooze Request Control Register 1"]
    #[inline(always)]
    pub const fn snzreqcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Snzreqcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snzreqcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "Snooze Control Register"]
    #[inline(always)]
    pub const fn snzcr(&self) -> &'static crate::common::Reg<self::Snzcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snzcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(146usize),
            )
        }
    }

    #[doc = "Snooze End Control Register 0"]
    #[inline(always)]
    pub const fn snzedcr0(
        &self,
    ) -> &'static crate::common::Reg<self::Snzedcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snzedcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "Snooze End Control Register 1"]
    #[inline(always)]
    pub const fn snzedcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Snzedcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snzedcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(149usize),
            )
        }
    }

    #[doc = "Snooze Request Control Register 0"]
    #[inline(always)]
    pub const fn snzreqcr0(
        &self,
    ) -> &'static crate::common::Reg<self::Snzreqcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snzreqcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "Operating Power Control Register"]
    #[inline(always)]
    pub const fn opccr(&self) -> &'static crate::common::Reg<self::Opccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Opccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "Main Clock Oscillator Wait Control Register"]
    #[inline(always)]
    pub const fn moscwtcr(
        &self,
    ) -> &'static crate::common::Reg<self::Moscwtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Moscwtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(162usize),
            )
        }
    }

    #[doc = "Sub Operating Power Control Register"]
    #[inline(always)]
    pub const fn sopccr(
        &self,
    ) -> &'static crate::common::Reg<self::Sopccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sopccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(170usize),
            )
        }
    }

    #[doc = "Reset Status Register 1"]
    #[inline(always)]
    pub const fn rstsr1(
        &self,
    ) -> &'static crate::common::Reg<self::Rstsr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstsr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "Voltage Monitor 1 Circuit Control Register 1"]
    #[inline(always)]
    pub const fn lvd1cr1(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd1Cr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd1Cr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[doc = "Voltage Monitor 1 Circuit Status Register"]
    #[inline(always)]
    pub const fn lvd1sr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd1Sr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd1Sr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(225usize),
            )
        }
    }

    #[doc = "Voltage Monitor 2 Circuit Control Register 1"]
    #[inline(always)]
    pub const fn lvd2cr1(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd2Cr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd2Cr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(226usize),
            )
        }
    }

    #[doc = "Voltage Monitor 2 Circuit Status Register"]
    #[inline(always)]
    pub const fn lvd2sr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd2Sr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd2Sr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(227usize),
            )
        }
    }

    #[doc = "Clock Generation Function Security Attribute Register"]
    #[inline(always)]
    pub const fn cgfsar(
        &self,
    ) -> &'static crate::common::Reg<self::Cgfsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cgfsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(960usize),
            )
        }
    }

    #[doc = "Reset Security Attribution Register"]
    #[inline(always)]
    pub const fn rstsar(
        &self,
    ) -> &'static crate::common::Reg<self::Rstsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(964usize),
            )
        }
    }

    #[doc = "Low Power Mode Security Attribution Register"]
    #[inline(always)]
    pub const fn lpmsar(
        &self,
    ) -> &'static crate::common::Reg<self::Lpmsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lpmsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(968usize),
            )
        }
    }

    #[doc = "Low Voltage Detection Security Attribution Register"]
    #[inline(always)]
    pub const fn lvdsar(
        &self,
    ) -> &'static crate::common::Reg<self::Lvdsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvdsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(972usize),
            )
        }
    }

    #[doc = "Battery Backup Function Security Attribute Register"]
    #[inline(always)]
    pub const fn bbfsar(
        &self,
    ) -> &'static crate::common::Reg<self::Bbfsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bbfsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(976usize),
            )
        }
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribution Register"]
    #[inline(always)]
    pub const fn dpfsar(
        &self,
    ) -> &'static crate::common::Reg<self::Dpfsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpfsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(992usize),
            )
        }
    }

    #[doc = "Protect Register"]
    #[inline(always)]
    pub const fn prcr(&self) -> &'static crate::common::Reg<self::Prcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Prcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1022usize),
            )
        }
    }

    #[doc = "Deep Software Standby Control Register"]
    #[inline(always)]
    pub const fn dpsbycr(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsbycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsbycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[doc = "Deep Software Standby Wait Control Register"]
    #[inline(always)]
    pub const fn dpswcr(
        &self,
    ) -> &'static crate::common::Reg<self::Dpswcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpswcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1025usize),
            )
        }
    }

    #[doc = "Deep Software Standby Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn dpsier0(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsier0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsier0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1026usize),
            )
        }
    }

    #[doc = "Deep Software Standby Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn dpsier1(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsier1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsier1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1027usize),
            )
        }
    }

    #[doc = "Deep Software Standby Interrupt Enable Register 2"]
    #[inline(always)]
    pub const fn dpsier2(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsier2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsier2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1028usize),
            )
        }
    }

    #[doc = "Deep Software Standby Interrupt Enable Register 3"]
    #[inline(always)]
    pub const fn dpsier3(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsier3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsier3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1029usize),
            )
        }
    }

    #[doc = "Deep Software Standby Interrupt Flag Register 0"]
    #[inline(always)]
    pub const fn dpsifr0(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsifr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsifr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1030usize),
            )
        }
    }

    #[doc = "Deep Software Standby Interrupt Flag Register 1"]
    #[inline(always)]
    pub const fn dpsifr1(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsifr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsifr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1031usize),
            )
        }
    }

    #[doc = "Deep Software Standby Interrupt Flag Register 2"]
    #[inline(always)]
    pub const fn dpsifr2(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsifr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsifr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1032usize),
            )
        }
    }

    #[doc = "Deep Software Standby Interrupt Flag Register 3"]
    #[inline(always)]
    pub const fn dpsifr3(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsifr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsifr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1033usize),
            )
        }
    }

    #[doc = "Deep Software Standby Interrupt Edge Register 0"]
    #[inline(always)]
    pub const fn dpsiegr0(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsiegr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsiegr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1034usize),
            )
        }
    }

    #[doc = "Deep Software Standby Interrupt Edge Register 1"]
    #[inline(always)]
    pub const fn dpsiegr1(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsiegr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsiegr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1035usize),
            )
        }
    }

    #[doc = "Deep Software Standby Interrupt Edge Register 2"]
    #[inline(always)]
    pub const fn dpsiegr2(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsiegr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsiegr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1036usize),
            )
        }
    }

    #[doc = "System Control OCD Control Register"]
    #[inline(always)]
    pub const fn syocdcr(
        &self,
    ) -> &'static crate::common::Reg<self::Syocdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syocdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1038usize),
            )
        }
    }

    #[doc = "Reset Status Register 0"]
    #[inline(always)]
    pub const fn rstsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Rstsr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstsr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1040usize),
            )
        }
    }

    #[doc = "Reset Status Register 2"]
    #[inline(always)]
    pub const fn rstsr2(
        &self,
    ) -> &'static crate::common::Reg<self::Rstsr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstsr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1041usize),
            )
        }
    }

    #[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
    #[inline(always)]
    pub const fn momcr(&self) -> &'static crate::common::Reg<self::Momcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Momcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1043usize),
            )
        }
    }

    #[doc = "Flash P/E Protect Register"]
    #[inline(always)]
    pub const fn fwepror(
        &self,
    ) -> &'static crate::common::Reg<self::Fwepror_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwepror_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1046usize),
            )
        }
    }

    #[doc = "Voltage Monitoring 1 Comparator Control Register"]
    #[inline(always)]
    pub const fn lvd1cmpcr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd1Cmpcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd1Cmpcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1047usize),
            )
        }
    }

    #[doc = "Voltage Monitoring 2 Comparator Control Register"]
    #[inline(always)]
    pub const fn lvd2cmpcr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd2Cmpcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd2Cmpcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1048usize),
            )
        }
    }

    #[doc = "Voltage Monitor 1 Circuit Control Register 0"]
    #[inline(always)]
    pub const fn lvd1cr0(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd1Cr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd1Cr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1050usize),
            )
        }
    }

    #[doc = "Voltage Monitor 2 Circuit Control Register 0"]
    #[inline(always)]
    pub const fn lvd2cr0(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd2Cr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd2Cr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1051usize),
            )
        }
    }

    #[doc = "Battery Backup Voltage Monitor Function Select Register"]
    #[inline(always)]
    pub const fn vbattmnselr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbattmnselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbattmnselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1053usize),
            )
        }
    }

    #[doc = "Battery Backup Voltage Monitor Register"]
    #[inline(always)]
    pub const fn vbattmonr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbattmonr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vbattmonr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1054usize),
            )
        }
    }

    #[doc = "LDO Stop Control Register"]
    #[inline(always)]
    pub const fn ldoscr(
        &self,
    ) -> &'static crate::common::Reg<self::Ldoscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ldoscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1088usize),
            )
        }
    }

    #[doc = "PLL2-LDO Stop Control Register"]
    #[inline(always)]
    pub const fn pl2ldoscr(
        &self,
    ) -> &'static crate::common::Reg<self::Pl2Ldoscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pl2Ldoscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1092usize),
            )
        }
    }

    #[doc = "Sub-Clock Oscillator Control Register"]
    #[inline(always)]
    pub const fn sosccr(
        &self,
    ) -> &'static crate::common::Reg<self::Sosccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sosccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1152usize),
            )
        }
    }

    #[doc = "Sub-Clock Oscillator Mode Control Register"]
    #[inline(always)]
    pub const fn somcr(&self) -> &'static crate::common::Reg<self::Somcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Somcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1153usize),
            )
        }
    }

    #[doc = "Low-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn lococr(
        &self,
    ) -> &'static crate::common::Reg<self::Lococr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lococr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1168usize),
            )
        }
    }

    #[doc = "LOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn locoutcr(
        &self,
    ) -> &'static crate::common::Reg<self::Locoutcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Locoutcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1170usize),
            )
        }
    }

    #[doc = "VBATT Input Control Register"]
    #[inline(always)]
    pub const fn vbtictlr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtictlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtictlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1211usize),
            )
        }
    }

    #[doc = "VBATT Backup Enable Register"]
    #[inline(always)]
    pub const fn vbtber(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtber_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtber_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1216usize),
            )
        }
    }

    #[doc = "VBATT Backup Register"]
    #[inline(always)]
    pub const fn vbtbkr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW>,
        128,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x500usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbycr_SPEC;
impl crate::sealed::RegSpec for Sbycr_SPEC {
    type DataType = u16;
}

#[doc = "Standby Control Register"]
pub type Sbycr = crate::RegValueT<Sbycr_SPEC>;

impl Sbycr {
    #[doc = "Software Standby Mode Select"]
    #[inline(always)]
    pub fn ssby(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        sbycr::Ssby,
        sbycr::Ssby,
        Sbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            sbycr::Ssby,
            sbycr::Ssby,
            Sbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sbycr {
    #[inline(always)]
    fn default() -> Sbycr {
        <crate::RegValueT<Sbycr_SPEC> as RegisterValue<_>>::new(16384)
    }
}
pub mod sbycr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssby_SPEC;
    pub type Ssby = crate::EnumBitfieldStruct<u8, Ssby_SPEC>;
    impl Ssby {
        #[doc = "Sleep mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sckdivcr_SPEC;
impl crate::sealed::RegSpec for Sckdivcr_SPEC {
    type DataType = u32;
}

#[doc = "System Clock Division Control Register"]
pub type Sckdivcr = crate::RegValueT<Sckdivcr_SPEC>;

impl Sckdivcr {
    #[doc = "Peripheral Module Clock D (PCLKD) Select"]
    #[inline(always)]
    pub fn pckd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        sckdivcr::Pckd,
        sckdivcr::Pckd,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            sckdivcr::Pckd,
            sckdivcr::Pckd,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Module Clock C (PCLKC) Select"]
    #[inline(always)]
    pub fn pckc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        sckdivcr::Pckc,
        sckdivcr::Pckc,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            sckdivcr::Pckc,
            sckdivcr::Pckc,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Module Clock B (PCLKB) Select"]
    #[inline(always)]
    pub fn pckb(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        sckdivcr::Pckb,
        sckdivcr::Pckb,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            sckdivcr::Pckb,
            sckdivcr::Pckb,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Module Clock A (PCLKA) Select"]
    #[inline(always)]
    pub fn pcka(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        sckdivcr::Pcka,
        sckdivcr::Pcka,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            sckdivcr::Pcka,
            sckdivcr::Pcka,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Reserved. Set these bits to the same value as PCKB\\[2:0\\]."]
    #[inline(always)]
    pub fn rsv(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        sckdivcr::Rsv,
        sckdivcr::Rsv,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            sckdivcr::Rsv,
            sckdivcr::Rsv,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "System Clock (ICLK) Select"]
    #[inline(always)]
    pub fn ick(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        sckdivcr::Ick,
        sckdivcr::Ick,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            sckdivcr::Ick,
            sckdivcr::Ick,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FlashIF Clock (FCLK) Select"]
    #[inline(always)]
    pub fn fck(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
        sckdivcr::Fck,
        sckdivcr::Fck,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x7,
            1,
            0,
            sckdivcr::Fck,
            sckdivcr::Fck,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sckdivcr {
    #[inline(always)]
    fn default() -> Sckdivcr {
        <crate::RegValueT<Sckdivcr_SPEC> as RegisterValue<_>>::new(570565154)
    }
}
pub mod sckdivcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pckd_SPEC;
    pub type Pckd = crate::EnumBitfieldStruct<u8, Pckd_SPEC>;
    impl Pckd {
        #[doc = "x 1/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "x 1/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "x 1/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "x 1/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "x 1/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "x 1/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "x 1/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pckc_SPEC;
    pub type Pckc = crate::EnumBitfieldStruct<u8, Pckc_SPEC>;
    impl Pckc {
        #[doc = "x 1/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "x 1/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "x 1/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "x 1/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "x 1/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "x 1/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "x 1/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pckb_SPEC;
    pub type Pckb = crate::EnumBitfieldStruct<u8, Pckb_SPEC>;
    impl Pckb {
        #[doc = "x 1/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "x 1/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "x 1/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "x 1/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "x 1/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "x 1/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "x 1/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcka_SPEC;
    pub type Pcka = crate::EnumBitfieldStruct<u8, Pcka_SPEC>;
    impl Pcka {
        #[doc = "x 1/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "x 1/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "x 1/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "x 1/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "x 1/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "x 1/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "x 1/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsv_SPEC;
    pub type Rsv = crate::EnumBitfieldStruct<u8, Rsv_SPEC>;
    impl Rsv {
        #[doc = "x 1/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "x 1/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "x 1/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "x 1/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "x 1/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "x 1/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "x 1/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "Settings prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ick_SPEC;
    pub type Ick = crate::EnumBitfieldStruct<u8, Ick_SPEC>;
    impl Ick {
        #[doc = "x 1/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "x 1/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "x 1/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "x 1/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "x 1/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "x 1/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "x 1/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fck_SPEC;
    pub type Fck = crate::EnumBitfieldStruct<u8, Fck_SPEC>;
    impl Fck {
        #[doc = "x 1/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "x 1/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "x 1/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "x 1/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "x 1/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "x 1/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "x 1/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sckscr_SPEC;
impl crate::sealed::RegSpec for Sckscr_SPEC {
    type DataType = u8;
}

#[doc = "System Clock Source Control Register"]
pub type Sckscr = crate::RegValueT<Sckscr_SPEC>;

impl Sckscr {
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub fn cksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        sckscr::Cksel,
        sckscr::Cksel,
        Sckscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            sckscr::Cksel,
            sckscr::Cksel,
            Sckscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sckscr {
    #[inline(always)]
    fn default() -> Sckscr {
        <crate::RegValueT<Sckscr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod sckscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksel_SPEC;
    pub type Cksel = crate::EnumBitfieldStruct<u8, Cksel_SPEC>;
    impl Cksel {
        #[doc = "HOCO"]
        pub const _000: Self = Self::new(0);

        #[doc = "MOCO"]
        pub const _001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _010: Self = Self::new(2);

        #[doc = "Main clock oscillator (MOSC)"]
        pub const _011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator (SOSC)"]
        pub const _100: Self = Self::new(4);

        #[doc = "PLL"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllccr_SPEC;
impl crate::sealed::RegSpec for Pllccr_SPEC {
    type DataType = u16;
}

#[doc = "PLL Clock Control Register"]
pub type Pllccr = crate::RegValueT<Pllccr_SPEC>;

impl Pllccr {
    #[doc = "PLL Input Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn plidiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        pllccr::Plidiv,
        pllccr::Plidiv,
        Pllccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            pllccr::Plidiv,
            pllccr::Plidiv,
            Pllccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PLL Clock Source Select"]
    #[inline(always)]
    pub fn plsrcsel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pllccr::Plsrcsel,
        pllccr::Plsrcsel,
        Pllccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pllccr::Plsrcsel,
            pllccr::Plsrcsel,
            Pllccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PLL Frequency Multiplication Factor Select"]
    #[inline(always)]
    pub fn pllmul(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Pllccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Pllccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pllccr {
    #[inline(always)]
    fn default() -> Pllccr {
        <crate::RegValueT<Pllccr_SPEC> as RegisterValue<_>>::new(4864)
    }
}
pub mod pllccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plidiv_SPEC;
    pub type Plidiv = crate::EnumBitfieldStruct<u8, Plidiv_SPEC>;
    impl Plidiv {
        #[doc = "/1"]
        pub const _00: Self = Self::new(0);

        #[doc = "/2"]
        pub const _01: Self = Self::new(1);

        #[doc = "/3"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plsrcsel_SPEC;
    pub type Plsrcsel = crate::EnumBitfieldStruct<u8, Plsrcsel_SPEC>;
    impl Plsrcsel {
        #[doc = "Main clock oscillator"]
        pub const _0: Self = Self::new(0);

        #[doc = "HOCO"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllcr_SPEC;
impl crate::sealed::RegSpec for Pllcr_SPEC {
    type DataType = u8;
}

#[doc = "PLL Control Register"]
pub type Pllcr = crate::RegValueT<Pllcr_SPEC>;

impl Pllcr {
    #[doc = "PLL Stop Control"]
    #[inline(always)]
    pub fn pllstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pllcr::Pllstp,
        pllcr::Pllstp,
        Pllcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pllcr::Pllstp,
            pllcr::Pllstp,
            Pllcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pllcr {
    #[inline(always)]
    fn default() -> Pllcr {
        <crate::RegValueT<Pllcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod pllcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllstp_SPEC;
    pub type Pllstp = crate::EnumBitfieldStruct<u8, Pllstp_SPEC>;
    impl Pllstp {
        #[doc = "PLL is operating"]
        pub const _0: Self = Self::new(0);

        #[doc = "PLL is stopped."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mosccr_SPEC;
impl crate::sealed::RegSpec for Mosccr_SPEC {
    type DataType = u8;
}

#[doc = "Main Clock Oscillator Control Register"]
pub type Mosccr = crate::RegValueT<Mosccr_SPEC>;

impl Mosccr {
    #[doc = "Main Clock Oscillator Stop"]
    #[inline(always)]
    pub fn mostp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mosccr::Mostp,
        mosccr::Mostp,
        Mosccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mosccr::Mostp,
            mosccr::Mostp,
            Mosccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mosccr {
    #[inline(always)]
    fn default() -> Mosccr {
        <crate::RegValueT<Mosccr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod mosccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mostp_SPEC;
    pub type Mostp = crate::EnumBitfieldStruct<u8, Mostp_SPEC>;
    impl Mostp {
        #[doc = "Operate the main clock oscillator"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the main clock oscillator"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hococr_SPEC;
impl crate::sealed::RegSpec for Hococr_SPEC {
    type DataType = u8;
}

#[doc = "High-Speed On-Chip Oscillator Control Register"]
pub type Hococr = crate::RegValueT<Hococr_SPEC>;

impl Hococr {
    #[doc = "HOCO Stop"]
    #[inline(always)]
    pub fn hcstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        hococr::Hcstp,
        hococr::Hcstp,
        Hococr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            hococr::Hcstp,
            hococr::Hcstp,
            Hococr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Hococr {
    #[inline(always)]
    fn default() -> Hococr {
        <crate::RegValueT<Hococr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hococr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hcstp_SPEC;
    pub type Hcstp = crate::EnumBitfieldStruct<u8, Hcstp_SPEC>;
    impl Hcstp {
        #[doc = "Operate the HOCO clock"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the HOCO clock"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hococr2_SPEC;
impl crate::sealed::RegSpec for Hococr2_SPEC {
    type DataType = u8;
}

#[doc = "High-Speed On-Chip Oscillator Control Register 2"]
pub type Hococr2 = crate::RegValueT<Hococr2_SPEC>;

impl Hococr2 {
    #[doc = "HOCO Frequency Setting 0"]
    #[inline(always)]
    pub fn hcfrq0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        hococr2::Hcfrq0,
        hococr2::Hcfrq0,
        Hococr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            hococr2::Hcfrq0,
            hococr2::Hcfrq0,
            Hococr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Hococr2 {
    #[inline(always)]
    fn default() -> Hococr2 {
        <crate::RegValueT<Hococr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hococr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hcfrq0_SPEC;
    pub type Hcfrq0 = crate::EnumBitfieldStruct<u8, Hcfrq0_SPEC>;
    impl Hcfrq0 {
        #[doc = "16 MHz"]
        pub const _00: Self = Self::new(0);

        #[doc = "18 MHz"]
        pub const _01: Self = Self::new(1);

        #[doc = "20 MHz"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mococr_SPEC;
impl crate::sealed::RegSpec for Mococr_SPEC {
    type DataType = u8;
}

#[doc = "Middle-Speed On-Chip Oscillator Control Register"]
pub type Mococr = crate::RegValueT<Mococr_SPEC>;

impl Mococr {
    #[doc = "MOCO Stop"]
    #[inline(always)]
    pub fn mcstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mococr::Mcstp,
        mococr::Mcstp,
        Mococr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mococr::Mcstp,
            mococr::Mcstp,
            Mococr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mococr {
    #[inline(always)]
    fn default() -> Mococr {
        <crate::RegValueT<Mococr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mococr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcstp_SPEC;
    pub type Mcstp = crate::EnumBitfieldStruct<u8, Mcstp_SPEC>;
    impl Mcstp {
        #[doc = "MOCO clock is operating"]
        pub const _0: Self = Self::new(0);

        #[doc = "MOCO clock is stopped"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fllcr1_SPEC;
impl crate::sealed::RegSpec for Fllcr1_SPEC {
    type DataType = u8;
}

#[doc = "FLL Control Register 1"]
pub type Fllcr1 = crate::RegValueT<Fllcr1_SPEC>;

impl Fllcr1 {
    #[doc = "FLL Enable"]
    #[inline(always)]
    pub fn fllen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fllcr1::Fllen,
        fllcr1::Fllen,
        Fllcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fllcr1::Fllen,
            fllcr1::Fllen,
            Fllcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fllcr1 {
    #[inline(always)]
    fn default() -> Fllcr1 {
        <crate::RegValueT<Fllcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fllcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fllen_SPEC;
    pub type Fllen = crate::EnumBitfieldStruct<u8, Fllen_SPEC>;
    impl Fllen {
        #[doc = "FLL function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "FLL function is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fllcr2_SPEC;
impl crate::sealed::RegSpec for Fllcr2_SPEC {
    type DataType = u16;
}

#[doc = "FLL Control Register 2"]
pub type Fllcr2 = crate::RegValueT<Fllcr2_SPEC>;

impl Fllcr2 {
    #[doc = "FLL Multiplication Control"]
    #[inline(always)]
    pub fn fllcntl(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Fllcr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Fllcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fllcr2 {
    #[inline(always)]
    fn default() -> Fllcr2 {
        <crate::RegValueT<Fllcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscsf_SPEC;
impl crate::sealed::RegSpec for Oscsf_SPEC {
    type DataType = u8;
}

#[doc = "Oscillation Stabilization Flag Register"]
pub type Oscsf = crate::RegValueT<Oscsf_SPEC>;

impl Oscsf {
    #[doc = "HOCO Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn hocosf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        oscsf::Hocosf,
        oscsf::Hocosf,
        Oscsf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            oscsf::Hocosf,
            oscsf::Hocosf,
            Oscsf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Main Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn moscsf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        oscsf::Moscsf,
        oscsf::Moscsf,
        Oscsf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            oscsf::Moscsf,
            oscsf::Moscsf,
            Oscsf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PLL Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn pllsf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        oscsf::Pllsf,
        oscsf::Pllsf,
        Oscsf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            oscsf::Pllsf,
            oscsf::Pllsf,
            Oscsf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PLL2 Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn pll2sf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        oscsf::Pll2Sf,
        oscsf::Pll2Sf,
        Oscsf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            oscsf::Pll2Sf,
            oscsf::Pll2Sf,
            Oscsf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Oscsf {
    #[inline(always)]
    fn default() -> Oscsf {
        <crate::RegValueT<Oscsf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod oscsf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hocosf_SPEC;
    pub type Hocosf = crate::EnumBitfieldStruct<u8, Hocosf_SPEC>;
    impl Hocosf {
        #[doc = "The HOCO clock is stopped or is not yet stable"]
        pub const _0: Self = Self::new(0);

        #[doc = "The HOCO clock is stable, so is available for use as the system clock"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moscsf_SPEC;
    pub type Moscsf = crate::EnumBitfieldStruct<u8, Moscsf_SPEC>;
    impl Moscsf {
        #[doc = "The main clock oscillator is stopped (MOSTP = 1) or is not yet stable"]
        pub const _0: Self = Self::new(0);

        #[doc = "The main clock oscillator is stable, so is available for use as the system clock"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllsf_SPEC;
    pub type Pllsf = crate::EnumBitfieldStruct<u8, Pllsf_SPEC>;
    impl Pllsf {
        #[doc = "The PLL clock is stopped, or oscillation of the PLL clock is not stable yet"]
        pub const _0: Self = Self::new(0);

        #[doc = "The PLL clock is stable, so is available for use as the system clock"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll2Sf_SPEC;
    pub type Pll2Sf = crate::EnumBitfieldStruct<u8, Pll2Sf_SPEC>;
    impl Pll2Sf {
        #[doc = "The PLL2 clock is stopped, or oscillation of the PLL2 clock is not stable yet"]
        pub const _0: Self = Self::new(0);

        #[doc = "The PLL2 clock is stable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ckocr_SPEC;
impl crate::sealed::RegSpec for Ckocr_SPEC {
    type DataType = u8;
}

#[doc = "Clock Out Control Register"]
pub type Ckocr = crate::RegValueT<Ckocr_SPEC>;

impl Ckocr {
    #[doc = "Clock Out Source Select"]
    #[inline(always)]
    pub fn ckosel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        ckocr::Ckosel,
        ckocr::Ckosel,
        Ckocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            ckocr::Ckosel,
            ckocr::Ckosel,
            Ckocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Output Frequency Division Ratio"]
    #[inline(always)]
    pub fn ckodiv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        ckocr::Ckodiv,
        ckocr::Ckodiv,
        Ckocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            ckocr::Ckodiv,
            ckocr::Ckodiv,
            Ckocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Out Enable"]
    #[inline(always)]
    pub fn ckoen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ckocr::Ckoen,
        ckocr::Ckoen,
        Ckocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ckocr::Ckoen,
            ckocr::Ckoen,
            Ckocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ckocr {
    #[inline(always)]
    fn default() -> Ckocr {
        <crate::RegValueT<Ckocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ckocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckosel_SPEC;
    pub type Ckosel = crate::EnumBitfieldStruct<u8, Ckosel_SPEC>;
    impl Ckosel {
        #[doc = "HOCO (value after reset)"]
        pub const _000: Self = Self::new(0);

        #[doc = "MOCO"]
        pub const _001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _010: Self = Self::new(2);

        #[doc = "MOSC"]
        pub const _011: Self = Self::new(3);

        #[doc = "SOSC"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckodiv_SPEC;
    pub type Ckodiv = crate::EnumBitfieldStruct<u8, Ckodiv_SPEC>;
    impl Ckodiv {
        #[doc = "x 1/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "x 1/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "x 1/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "x 1/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "x 1/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "x 1/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "x 1/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "x 1/128"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckoen_SPEC;
    pub type Ckoen = crate::EnumBitfieldStruct<u8, Ckoen_SPEC>;
    impl Ckoen {
        #[doc = "Disable clock out"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable clock out"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trckcr_SPEC;
impl crate::sealed::RegSpec for Trckcr_SPEC {
    type DataType = u8;
}

#[doc = "Trace Clock Control Register"]
pub type Trckcr = crate::RegValueT<Trckcr_SPEC>;

impl Trckcr {
    #[doc = "Trace Clock operating frequency select"]
    #[inline(always)]
    pub fn trck(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        trckcr::Trck,
        trckcr::Trck,
        Trckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            trckcr::Trck,
            trckcr::Trck,
            Trckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Trace Clock operating Enable"]
    #[inline(always)]
    pub fn trcken(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        trckcr::Trcken,
        trckcr::Trcken,
        Trckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            trckcr::Trcken,
            trckcr::Trcken,
            Trckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Trckcr {
    #[inline(always)]
    fn default() -> Trckcr {
        <crate::RegValueT<Trckcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod trckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trck_SPEC;
    pub type Trck = crate::EnumBitfieldStruct<u8, Trck_SPEC>;
    impl Trck {
        #[doc = "/1"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "/2 (value after reset)"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "/4"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trcken_SPEC;
    pub type Trcken = crate::EnumBitfieldStruct<u8, Trcken_SPEC>;
    impl Trcken {
        #[doc = "Stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operation enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostdcr_SPEC;
impl crate::sealed::RegSpec for Ostdcr_SPEC {
    type DataType = u8;
}

#[doc = "Oscillation Stop Detection Control Register"]
pub type Ostdcr = crate::RegValueT<Ostdcr_SPEC>;

impl Ostdcr {
    #[doc = "Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub fn ostdie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ostdcr::Ostdie,
        ostdcr::Ostdie,
        Ostdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ostdcr::Ostdie,
            ostdcr::Ostdie,
            Ostdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Oscillation Stop Detection Function Enable"]
    #[inline(always)]
    pub fn ostde(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ostdcr::Ostde,
        ostdcr::Ostde,
        Ostdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ostdcr::Ostde,
            ostdcr::Ostde,
            Ostdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ostdcr {
    #[inline(always)]
    fn default() -> Ostdcr {
        <crate::RegValueT<Ostdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ostdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostdie_SPEC;
    pub type Ostdie = crate::EnumBitfieldStruct<u8, Ostdie_SPEC>;
    impl Ostdie {
        #[doc = "Disable oscillation stop detection interrupt (do not notify the POEG)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable oscillation stop detection interrupt (notify the POEG)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostde_SPEC;
    pub type Ostde = crate::EnumBitfieldStruct<u8, Ostde_SPEC>;
    impl Ostde {
        #[doc = "Disable oscillation stop detection function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable oscillation stop detection function"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostdsr_SPEC;
impl crate::sealed::RegSpec for Ostdsr_SPEC {
    type DataType = u8;
}

#[doc = "Oscillation Stop Detection Status Register"]
pub type Ostdsr = crate::RegValueT<Ostdsr_SPEC>;

impl Ostdsr {
    #[doc = "Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub fn ostdf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ostdsr::Ostdf,
        ostdsr::Ostdf,
        Ostdsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ostdsr::Ostdf,
            ostdsr::Ostdf,
            Ostdsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ostdsr {
    #[inline(always)]
    fn default() -> Ostdsr {
        <crate::RegValueT<Ostdsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ostdsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostdf_SPEC;
    pub type Ostdf = crate::EnumBitfieldStruct<u8, Ostdf_SPEC>;
    impl Ostdf {
        #[doc = "Main clock oscillation stop not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Main clock oscillation stop detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll2Ccr_SPEC;
impl crate::sealed::RegSpec for Pll2Ccr_SPEC {
    type DataType = u16;
}

#[doc = "PLL2 Clock Control Register"]
pub type Pll2Ccr = crate::RegValueT<Pll2Ccr_SPEC>;

impl Pll2Ccr {
    #[doc = "PLL2 Input Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn pl2idiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        pll2ccr::Pl2Idiv,
        pll2ccr::Pl2Idiv,
        Pll2Ccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            pll2ccr::Pl2Idiv,
            pll2ccr::Pl2Idiv,
            Pll2Ccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PLL2 Clock Source Select"]
    #[inline(always)]
    pub fn pl2srcsel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pll2ccr::Pl2Srcsel,
        pll2ccr::Pl2Srcsel,
        Pll2Ccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pll2ccr::Pl2Srcsel,
            pll2ccr::Pl2Srcsel,
            Pll2Ccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PLL2 Frequency Multiplication Factor Select"]
    #[inline(always)]
    pub fn pll2mul(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Pll2Ccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Pll2Ccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pll2Ccr {
    #[inline(always)]
    fn default() -> Pll2Ccr {
        <crate::RegValueT<Pll2Ccr_SPEC> as RegisterValue<_>>::new(4864)
    }
}
pub mod pll2ccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pl2Idiv_SPEC;
    pub type Pl2Idiv = crate::EnumBitfieldStruct<u8, Pl2Idiv_SPEC>;
    impl Pl2Idiv {
        #[doc = "∕ 1 (value after reset)"]
        pub const _00: Self = Self::new(0);

        #[doc = "∕ 2"]
        pub const _01: Self = Self::new(1);

        #[doc = "∕ 3"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pl2Srcsel_SPEC;
    pub type Pl2Srcsel = crate::EnumBitfieldStruct<u8, Pl2Srcsel_SPEC>;
    impl Pl2Srcsel {
        #[doc = "Main clock oscillator"]
        pub const _0: Self = Self::new(0);

        #[doc = "HOCO"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll2Cr_SPEC;
impl crate::sealed::RegSpec for Pll2Cr_SPEC {
    type DataType = u8;
}

#[doc = "PLL2 Control Register"]
pub type Pll2Cr = crate::RegValueT<Pll2Cr_SPEC>;

impl Pll2Cr {
    #[doc = "PLL2 Stop Control"]
    #[inline(always)]
    pub fn pll2stp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pll2cr::Pll2Stp,
        pll2cr::Pll2Stp,
        Pll2Cr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pll2cr::Pll2Stp,
            pll2cr::Pll2Stp,
            Pll2Cr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pll2Cr {
    #[inline(always)]
    fn default() -> Pll2Cr {
        <crate::RegValueT<Pll2Cr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod pll2cr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll2Stp_SPEC;
    pub type Pll2Stp = crate::EnumBitfieldStruct<u8, Pll2Stp_SPEC>;
    impl Pll2Stp {
        #[doc = "PLL2 is operating"]
        pub const _0: Self = Self::new(0);

        #[doc = "PLL2 is stopped."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mocoutcr_SPEC;
impl crate::sealed::RegSpec for Mocoutcr_SPEC {
    type DataType = u8;
}

#[doc = "MOCO User Trimming Control Register"]
pub type Mocoutcr = crate::RegValueT<Mocoutcr_SPEC>;

impl Mocoutcr {
    #[doc = "MOCO User Trimming"]
    #[inline(always)]
    pub fn mocoutrm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Mocoutcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Mocoutcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mocoutcr {
    #[inline(always)]
    fn default() -> Mocoutcr {
        <crate::RegValueT<Mocoutcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hocoutcr_SPEC;
impl crate::sealed::RegSpec for Hocoutcr_SPEC {
    type DataType = u8;
}

#[doc = "HOCO User Trimming Control Register"]
pub type Hocoutcr = crate::RegValueT<Hocoutcr_SPEC>;

impl Hocoutcr {
    #[doc = "HOCO User Trimming"]
    #[inline(always)]
    pub fn hocoutrm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Hocoutcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Hocoutcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Hocoutcr {
    #[inline(always)]
    fn default() -> Hocoutcr {
        <crate::RegValueT<Hocoutcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbckdivcr_SPEC;
impl crate::sealed::RegSpec for Usbckdivcr_SPEC {
    type DataType = u8;
}

#[doc = "USB Clock Division Control Register"]
pub type Usbckdivcr = crate::RegValueT<Usbckdivcr_SPEC>;

impl Usbckdivcr {
    #[doc = "USB Clock (USBCLK) Division Select"]
    #[inline(always)]
    pub fn usbckdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        usbckdivcr::Usbckdiv,
        usbckdivcr::Usbckdiv,
        Usbckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            usbckdivcr::Usbckdiv,
            usbckdivcr::Usbckdiv,
            Usbckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Usbckdivcr {
    #[inline(always)]
    fn default() -> Usbckdivcr {
        <crate::RegValueT<Usbckdivcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod usbckdivcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbckdiv_SPEC;
    pub type Usbckdiv = crate::EnumBitfieldStruct<u8, Usbckdiv_SPEC>;
    impl Usbckdiv {
        #[doc = "∕ 4"]
        pub const _010: Self = Self::new(2);

        #[doc = "∕ 3"]
        pub const _101: Self = Self::new(5);

        #[doc = "∕ 5"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbckcr_SPEC;
impl crate::sealed::RegSpec for Usbckcr_SPEC {
    type DataType = u8;
}

#[doc = "USB Clock Control Register"]
pub type Usbckcr = crate::RegValueT<Usbckcr_SPEC>;

impl Usbckcr {
    #[doc = "USB Clock (USBCLK) Source Select"]
    #[inline(always)]
    pub fn usbcksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        usbckcr::Usbcksel,
        usbckcr::Usbcksel,
        Usbckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            usbckcr::Usbcksel,
            usbckcr::Usbcksel,
            Usbckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USB Clock (USBCLK) Switching Request"]
    #[inline(always)]
    pub fn usbcksreq(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        usbckcr::Usbcksreq,
        usbckcr::Usbcksreq,
        Usbckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            usbckcr::Usbcksreq,
            usbckcr::Usbcksreq,
            Usbckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USB Clock (USBCLK) Switching Ready state flag"]
    #[inline(always)]
    pub fn usbcksrdy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        usbckcr::Usbcksrdy,
        usbckcr::Usbcksrdy,
        Usbckcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            usbckcr::Usbcksrdy,
            usbckcr::Usbcksrdy,
            Usbckcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Usbckcr {
    #[inline(always)]
    fn default() -> Usbckcr {
        <crate::RegValueT<Usbckcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod usbckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbcksel_SPEC;
    pub type Usbcksel = crate::EnumBitfieldStruct<u8, Usbcksel_SPEC>;
    impl Usbcksel {
        #[doc = "PLL"]
        pub const _101: Self = Self::new(5);

        #[doc = "PLL2"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbcksreq_SPEC;
    pub type Usbcksreq = crate::EnumBitfieldStruct<u8, Usbcksreq_SPEC>;
    impl Usbcksreq {
        #[doc = "No request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request switching."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbcksrdy_SPEC;
    pub type Usbcksrdy = crate::EnumBitfieldStruct<u8, Usbcksrdy_SPEC>;
    impl Usbcksrdy {
        #[doc = "Impossible to Switch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Possible to Switch"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snzreqcr1_SPEC;
impl crate::sealed::RegSpec for Snzreqcr1_SPEC {
    type DataType = u32;
}

#[doc = "Snooze Request Control Register 1"]
pub type Snzreqcr1 = crate::RegValueT<Snzreqcr1_SPEC>;

impl Snzreqcr1 {
    #[doc = "Enable AGT3 underflow snooze request"]
    #[inline(always)]
    pub fn snzreqen0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        snzreqcr1::Snzreqen0,
        snzreqcr1::Snzreqen0,
        Snzreqcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            snzreqcr1::Snzreqen0,
            snzreqcr1::Snzreqen0,
            Snzreqcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable AGT3 compare match A snooze request"]
    #[inline(always)]
    pub fn snzreqen1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        snzreqcr1::Snzreqen1,
        snzreqcr1::Snzreqen1,
        Snzreqcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            snzreqcr1::Snzreqen1,
            snzreqcr1::Snzreqen1,
            Snzreqcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable AGT3 compare match B snooze request"]
    #[inline(always)]
    pub fn snzreqen2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        snzreqcr1::Snzreqen2,
        snzreqcr1::Snzreqen2,
        Snzreqcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            snzreqcr1::Snzreqen2,
            snzreqcr1::Snzreqen2,
            Snzreqcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Snzreqcr1 {
    #[inline(always)]
    fn default() -> Snzreqcr1 {
        <crate::RegValueT<Snzreqcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snzreqcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen0_SPEC;
    pub type Snzreqen0 = crate::EnumBitfieldStruct<u8, Snzreqen0_SPEC>;
    impl Snzreqen0 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen1_SPEC;
    pub type Snzreqen1 = crate::EnumBitfieldStruct<u8, Snzreqen1_SPEC>;
    impl Snzreqen1 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen2_SPEC;
    pub type Snzreqen2 = crate::EnumBitfieldStruct<u8, Snzreqen2_SPEC>;
    impl Snzreqen2 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snzcr_SPEC;
impl crate::sealed::RegSpec for Snzcr_SPEC {
    type DataType = u8;
}

#[doc = "Snooze Control Register"]
pub type Snzcr = crate::RegValueT<Snzcr_SPEC>;

impl Snzcr {
    #[doc = "RXD0 Snooze Request Enable"]
    #[inline(always)]
    pub fn rxdreqen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        snzcr::Rxdreqen,
        snzcr::Rxdreqen,
        Snzcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            snzcr::Rxdreqen,
            snzcr::Rxdreqen,
            Snzcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable in Snooze mode"]
    #[inline(always)]
    pub fn snzdtcen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        snzcr::Snzdtcen,
        snzcr::Snzdtcen,
        Snzcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            snzcr::Snzdtcen,
            snzcr::Snzdtcen,
            Snzcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Snooze mode Enable"]
    #[inline(always)]
    pub fn snze(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        snzcr::Snze,
        snzcr::Snze,
        Snzcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            snzcr::Snze,
            snzcr::Snze,
            Snzcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Snzcr {
    #[inline(always)]
    fn default() -> Snzcr {
        <crate::RegValueT<Snzcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snzcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdreqen_SPEC;
    pub type Rxdreqen = crate::EnumBitfieldStruct<u8, Rxdreqen_SPEC>;
    impl Rxdreqen {
        #[doc = "Ignore RXD0 falling edge in Software Standby mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect RXD0 falling edge in Software Standby mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzdtcen_SPEC;
    pub type Snzdtcen = crate::EnumBitfieldStruct<u8, Snzdtcen_SPEC>;
    impl Snzdtcen {
        #[doc = "Disable DTC operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable DTC operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snze_SPEC;
    pub type Snze = crate::EnumBitfieldStruct<u8, Snze_SPEC>;
    impl Snze {
        #[doc = "Disable Snooze mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable Snooze mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snzedcr0_SPEC;
impl crate::sealed::RegSpec for Snzedcr0_SPEC {
    type DataType = u8;
}

#[doc = "Snooze End Control Register 0"]
pub type Snzedcr0 = crate::RegValueT<Snzedcr0_SPEC>;

impl Snzedcr0 {
    #[doc = "AGT1 Underflow Snooze End Enable"]
    #[inline(always)]
    pub fn agtunfed(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        snzedcr0::Agtunfed,
        snzedcr0::Agtunfed,
        Snzedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            snzedcr0::Agtunfed,
            snzedcr0::Agtunfed,
            Snzedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtczred(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        snzedcr0::Dtczred,
        snzedcr0::Dtczred,
        Snzedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            snzedcr0::Dtczred,
            snzedcr0::Dtczred,
            Snzedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Not Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtcnzred(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        snzedcr0::Dtcnzred,
        snzedcr0::Dtcnzred,
        Snzedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            snzedcr0::Dtcnzred,
            snzedcr0::Dtcnzred,
            Snzedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ADC12 Compare Match Snooze End Enable"]
    #[inline(always)]
    pub fn ad0mated(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        snzedcr0::Ad0Mated,
        snzedcr0::Ad0Mated,
        Snzedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            snzedcr0::Ad0Mated,
            snzedcr0::Ad0Mated,
            Snzedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ADC12 Compare Mismatch Snooze End Enable"]
    #[inline(always)]
    pub fn ad0umted(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        snzedcr0::Ad0Umted,
        snzedcr0::Ad0Umted,
        Snzedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            snzedcr0::Ad0Umted,
            snzedcr0::Ad0Umted,
            Snzedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCI0 Address Mismatch Snooze End Enable"]
    #[inline(always)]
    pub fn sci0umted(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        snzedcr0::Sci0Umted,
        snzedcr0::Sci0Umted,
        Snzedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            snzedcr0::Sci0Umted,
            snzedcr0::Sci0Umted,
            Snzedcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Snzedcr0 {
    #[inline(always)]
    fn default() -> Snzedcr0 {
        <crate::RegValueT<Snzedcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snzedcr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agtunfed_SPEC;
    pub type Agtunfed = crate::EnumBitfieldStruct<u8, Agtunfed_SPEC>;
    impl Agtunfed {
        #[doc = "Disable the snooze end request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze end request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtczred_SPEC;
    pub type Dtczred = crate::EnumBitfieldStruct<u8, Dtczred_SPEC>;
    impl Dtczred {
        #[doc = "Disable the snooze end request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze end request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtcnzred_SPEC;
    pub type Dtcnzred = crate::EnumBitfieldStruct<u8, Dtcnzred_SPEC>;
    impl Dtcnzred {
        #[doc = "Disable the snooze end request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze end request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ad0Mated_SPEC;
    pub type Ad0Mated = crate::EnumBitfieldStruct<u8, Ad0Mated_SPEC>;
    impl Ad0Mated {
        #[doc = "Disable the snooze end request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze end request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ad0Umted_SPEC;
    pub type Ad0Umted = crate::EnumBitfieldStruct<u8, Ad0Umted_SPEC>;
    impl Ad0Umted {
        #[doc = "Disable the snooze end request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze end request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sci0Umted_SPEC;
    pub type Sci0Umted = crate::EnumBitfieldStruct<u8, Sci0Umted_SPEC>;
    impl Sci0Umted {
        #[doc = "Disable the snooze end request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze end request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snzedcr1_SPEC;
impl crate::sealed::RegSpec for Snzedcr1_SPEC {
    type DataType = u8;
}

#[doc = "Snooze End Control Register 1"]
pub type Snzedcr1 = crate::RegValueT<Snzedcr1_SPEC>;

impl Snzedcr1 {
    #[doc = "AGT3 underflow Snooze End Enable"]
    #[inline(always)]
    pub fn agt3unfed(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        snzedcr1::Agt3Unfed,
        snzedcr1::Agt3Unfed,
        Snzedcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            snzedcr1::Agt3Unfed,
            snzedcr1::Agt3Unfed,
            Snzedcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Snzedcr1 {
    #[inline(always)]
    fn default() -> Snzedcr1 {
        <crate::RegValueT<Snzedcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snzedcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt3Unfed_SPEC;
    pub type Agt3Unfed = crate::EnumBitfieldStruct<u8, Agt3Unfed_SPEC>;
    impl Agt3Unfed {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snzreqcr0_SPEC;
impl crate::sealed::RegSpec for Snzreqcr0_SPEC {
    type DataType = u32;
}

#[doc = "Snooze Request Control Register 0"]
pub type Snzreqcr0 = crate::RegValueT<Snzreqcr0_SPEC>;

impl Snzreqcr0 {
    #[doc = "Enable IRQ0 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen0,
        snzreqcr0::Snzreqen0,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen0,
            snzreqcr0::Snzreqen0,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ1 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen1,
        snzreqcr0::Snzreqen1,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen1,
            snzreqcr0::Snzreqen1,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ2 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen2,
        snzreqcr0::Snzreqen2,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen2,
            snzreqcr0::Snzreqen2,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ3 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen3,
        snzreqcr0::Snzreqen3,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen3,
            snzreqcr0::Snzreqen3,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ4 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen4,
        snzreqcr0::Snzreqen4,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen4,
            snzreqcr0::Snzreqen4,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ5 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen5,
        snzreqcr0::Snzreqen5,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen5,
            snzreqcr0::Snzreqen5,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ6 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen6,
        snzreqcr0::Snzreqen6,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen6,
            snzreqcr0::Snzreqen6,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ7 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen7,
        snzreqcr0::Snzreqen7,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen7,
            snzreqcr0::Snzreqen7,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ8 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen8,
        snzreqcr0::Snzreqen8,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen8,
            snzreqcr0::Snzreqen8,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ9 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen9,
        snzreqcr0::Snzreqen9,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen9,
            snzreqcr0::Snzreqen9,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ13 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen13,
        snzreqcr0::Snzreqen13,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen13,
            snzreqcr0::Snzreqen13,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable RTC alarm snooze request"]
    #[inline(always)]
    pub fn snzreqen24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen24,
        snzreqcr0::Snzreqen24,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen24,
            snzreqcr0::Snzreqen24,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable RTC period snooze request"]
    #[inline(always)]
    pub fn snzreqen25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen25,
        snzreqcr0::Snzreqen25,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen25,
            snzreqcr0::Snzreqen25,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable AGT1 underflow snooze request"]
    #[inline(always)]
    pub fn snzreqen28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen28,
        snzreqcr0::Snzreqen28,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen28,
            snzreqcr0::Snzreqen28,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable AGT1 compare match A snooze request"]
    #[inline(always)]
    pub fn snzreqen29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen29,
        snzreqcr0::Snzreqen29,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen29,
            snzreqcr0::Snzreqen29,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable AGT1 compare match B snooze request"]
    #[inline(always)]
    pub fn snzreqen30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen30,
        snzreqcr0::Snzreqen30,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen30,
            snzreqcr0::Snzreqen30,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Snzreqcr0 {
    #[inline(always)]
    fn default() -> Snzreqcr0 {
        <crate::RegValueT<Snzreqcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snzreqcr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen0_SPEC;
    pub type Snzreqen0 = crate::EnumBitfieldStruct<u8, Snzreqen0_SPEC>;
    impl Snzreqen0 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen1_SPEC;
    pub type Snzreqen1 = crate::EnumBitfieldStruct<u8, Snzreqen1_SPEC>;
    impl Snzreqen1 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen2_SPEC;
    pub type Snzreqen2 = crate::EnumBitfieldStruct<u8, Snzreqen2_SPEC>;
    impl Snzreqen2 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen3_SPEC;
    pub type Snzreqen3 = crate::EnumBitfieldStruct<u8, Snzreqen3_SPEC>;
    impl Snzreqen3 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen4_SPEC;
    pub type Snzreqen4 = crate::EnumBitfieldStruct<u8, Snzreqen4_SPEC>;
    impl Snzreqen4 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen5_SPEC;
    pub type Snzreqen5 = crate::EnumBitfieldStruct<u8, Snzreqen5_SPEC>;
    impl Snzreqen5 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen6_SPEC;
    pub type Snzreqen6 = crate::EnumBitfieldStruct<u8, Snzreqen6_SPEC>;
    impl Snzreqen6 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen7_SPEC;
    pub type Snzreqen7 = crate::EnumBitfieldStruct<u8, Snzreqen7_SPEC>;
    impl Snzreqen7 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen8_SPEC;
    pub type Snzreqen8 = crate::EnumBitfieldStruct<u8, Snzreqen8_SPEC>;
    impl Snzreqen8 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen9_SPEC;
    pub type Snzreqen9 = crate::EnumBitfieldStruct<u8, Snzreqen9_SPEC>;
    impl Snzreqen9 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen13_SPEC;
    pub type Snzreqen13 = crate::EnumBitfieldStruct<u8, Snzreqen13_SPEC>;
    impl Snzreqen13 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen24_SPEC;
    pub type Snzreqen24 = crate::EnumBitfieldStruct<u8, Snzreqen24_SPEC>;
    impl Snzreqen24 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen25_SPEC;
    pub type Snzreqen25 = crate::EnumBitfieldStruct<u8, Snzreqen25_SPEC>;
    impl Snzreqen25 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen28_SPEC;
    pub type Snzreqen28 = crate::EnumBitfieldStruct<u8, Snzreqen28_SPEC>;
    impl Snzreqen28 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen29_SPEC;
    pub type Snzreqen29 = crate::EnumBitfieldStruct<u8, Snzreqen29_SPEC>;
    impl Snzreqen29 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen30_SPEC;
    pub type Snzreqen30 = crate::EnumBitfieldStruct<u8, Snzreqen30_SPEC>;
    impl Snzreqen30 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opccr_SPEC;
impl crate::sealed::RegSpec for Opccr_SPEC {
    type DataType = u8;
}

#[doc = "Operating Power Control Register"]
pub type Opccr = crate::RegValueT<Opccr_SPEC>;

impl Opccr {
    #[doc = "Operating Power Control Mode Select"]
    #[inline(always)]
    pub fn opcm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        opccr::Opcm,
        opccr::Opcm,
        Opccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            opccr::Opcm,
            opccr::Opcm,
            Opccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operating Power Control Mode Transition Status Flag"]
    #[inline(always)]
    pub fn opcmtsf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        opccr::Opcmtsf,
        opccr::Opcmtsf,
        Opccr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            opccr::Opcmtsf,
            opccr::Opcmtsf,
            Opccr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Opccr {
    #[inline(always)]
    fn default() -> Opccr {
        <crate::RegValueT<Opccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod opccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opcm_SPEC;
    pub type Opcm = crate::EnumBitfieldStruct<u8, Opcm_SPEC>;
    impl Opcm {
        #[doc = "High-speed mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "Low-speed mode"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opcmtsf_SPEC;
    pub type Opcmtsf = crate::EnumBitfieldStruct<u8, Opcmtsf_SPEC>;
    impl Opcmtsf {
        #[doc = "Transition completed"]
        pub const _0: Self = Self::new(0);

        #[doc = "During transition"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Moscwtcr_SPEC;
impl crate::sealed::RegSpec for Moscwtcr_SPEC {
    type DataType = u8;
}

#[doc = "Main Clock Oscillator Wait Control Register"]
pub type Moscwtcr = crate::RegValueT<Moscwtcr_SPEC>;

impl Moscwtcr {
    #[doc = "Main Clock Oscillator Wait Time Setting"]
    #[inline(always)]
    pub fn msts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        moscwtcr::Msts,
        moscwtcr::Msts,
        Moscwtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            moscwtcr::Msts,
            moscwtcr::Msts,
            Moscwtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Moscwtcr {
    #[inline(always)]
    fn default() -> Moscwtcr {
        <crate::RegValueT<Moscwtcr_SPEC> as RegisterValue<_>>::new(5)
    }
}
pub mod moscwtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Msts_SPEC;
    pub type Msts = crate::EnumBitfieldStruct<u8, Msts_SPEC>;
    impl Msts {
        #[doc = "Wait time = 3 cycles (11.4 us)"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Wait time = 35 cycles (133.5 us)"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "Wait time = 67 cycles (255.6 us)"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "Wait time = 131 cycles (499.7 us)"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "Wait time = 259 cycles (988.0 us)"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "Wait time = 547 cycles (2086.6 us)"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "Wait time = 1059 cycles (4039.8 us)"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "Wait time = 2147 cycles (8190.2 us)"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "Wait time = 4291 cycles (16368.9 us)"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "Wait time = 8163 cycles (31139.4 us)"]
        pub const _0_X_9: Self = Self::new(9);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sopccr_SPEC;
impl crate::sealed::RegSpec for Sopccr_SPEC {
    type DataType = u8;
}

#[doc = "Sub Operating Power Control Register"]
pub type Sopccr = crate::RegValueT<Sopccr_SPEC>;

impl Sopccr {
    #[doc = "Sub Operating Power Control Mode Select"]
    #[inline(always)]
    pub fn sopcm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sopccr::Sopcm,
        sopccr::Sopcm,
        Sopccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sopccr::Sopcm,
            sopccr::Sopcm,
            Sopccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operating Power Control Mode Transition Status Flag"]
    #[inline(always)]
    pub fn sopcmtsf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sopccr::Sopcmtsf,
        sopccr::Sopcmtsf,
        Sopccr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sopccr::Sopcmtsf,
            sopccr::Sopcmtsf,
            Sopccr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sopccr {
    #[inline(always)]
    fn default() -> Sopccr {
        <crate::RegValueT<Sopccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sopccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sopcm_SPEC;
    pub type Sopcm = crate::EnumBitfieldStruct<u8, Sopcm_SPEC>;
    impl Sopcm {
        #[doc = "Other than Subosc-speed mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Subosc-speed mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sopcmtsf_SPEC;
    pub type Sopcmtsf = crate::EnumBitfieldStruct<u8, Sopcmtsf_SPEC>;
    impl Sopcmtsf {
        #[doc = "Transition completed"]
        pub const _0: Self = Self::new(0);

        #[doc = "During transition"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsr1_SPEC;
impl crate::sealed::RegSpec for Rstsr1_SPEC {
    type DataType = u16;
}

#[doc = "Reset Status Register 1"]
pub type Rstsr1 = crate::RegValueT<Rstsr1_SPEC>;

impl Rstsr1 {
    #[doc = "Independent Watchdog Timer Reset Detect Flag"]
    #[inline(always)]
    pub fn iwdtrf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rstsr1::Iwdtrf,
        rstsr1::Iwdtrf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rstsr1::Iwdtrf,
            rstsr1::Iwdtrf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watchdog Timer Reset Detect Flag"]
    #[inline(always)]
    pub fn wdtrf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rstsr1::Wdtrf,
        rstsr1::Wdtrf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rstsr1::Wdtrf,
            rstsr1::Wdtrf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software Reset Detect Flag"]
    #[inline(always)]
    pub fn swrf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rstsr1::Swrf,
        rstsr1::Swrf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rstsr1::Swrf,
            rstsr1::Swrf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SRAM Parity Error Reset Detect Flag"]
    #[inline(always)]
    pub fn rperf(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        rstsr1::Rperf,
        rstsr1::Rperf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            rstsr1::Rperf,
            rstsr1::Rperf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Master MPU Error Reset Detect Flag"]
    #[inline(always)]
    pub fn busmrf(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        rstsr1::Busmrf,
        rstsr1::Busmrf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            rstsr1::Busmrf,
            rstsr1::Busmrf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TrustZone Error Reset Detect Flag"]
    #[inline(always)]
    pub fn tzerf(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        rstsr1::Tzerf,
        rstsr1::Tzerf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            rstsr1::Tzerf,
            rstsr1::Tzerf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rstsr1 {
    #[inline(always)]
    fn default() -> Rstsr1 {
        <crate::RegValueT<Rstsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstsr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtrf_SPEC;
    pub type Iwdtrf = crate::EnumBitfieldStruct<u8, Iwdtrf_SPEC>;
    impl Iwdtrf {
        #[doc = "Independent watchdog timer reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Independent watchdog timer reset detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdtrf_SPEC;
    pub type Wdtrf = crate::EnumBitfieldStruct<u8, Wdtrf_SPEC>;
    impl Wdtrf {
        #[doc = "Watchdog timer reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Watchdog timer reset detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrf_SPEC;
    pub type Swrf = crate::EnumBitfieldStruct<u8, Swrf_SPEC>;
    impl Swrf {
        #[doc = "Software reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software reset detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rperf_SPEC;
    pub type Rperf = crate::EnumBitfieldStruct<u8, Rperf_SPEC>;
    impl Rperf {
        #[doc = "SRAM parity error reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "SRAM parity error reset detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busmrf_SPEC;
    pub type Busmrf = crate::EnumBitfieldStruct<u8, Busmrf_SPEC>;
    impl Busmrf {
        #[doc = "Bus master MPU error reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU error reset detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tzerf_SPEC;
    pub type Tzerf = crate::EnumBitfieldStruct<u8, Tzerf_SPEC>;
    impl Tzerf {
        #[doc = "TrustZone error reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "TrustZone error reset detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd1Cr1_SPEC;
impl crate::sealed::RegSpec for Lvd1Cr1_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 1 Circuit Control Register 1"]
pub type Lvd1Cr1 = crate::RegValueT<Lvd1Cr1_SPEC>;

impl Lvd1Cr1 {
    #[doc = "Voltage Monitor 1 Interrupt Generation Condition Select"]
    #[inline(always)]
    pub fn idtsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        lvd1cr1::Idtsel,
        lvd1cr1::Idtsel,
        Lvd1Cr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            lvd1cr1::Idtsel,
            lvd1cr1::Idtsel,
            Lvd1Cr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Interrupt Type Select"]
    #[inline(always)]
    pub fn irqsel(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        lvd1cr1::Irqsel,
        lvd1cr1::Irqsel,
        Lvd1Cr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            lvd1cr1::Irqsel,
            lvd1cr1::Irqsel,
            Lvd1Cr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd1Cr1 {
    #[inline(always)]
    fn default() -> Lvd1Cr1 {
        <crate::RegValueT<Lvd1Cr1_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod lvd1cr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idtsel_SPEC;
    pub type Idtsel = crate::EnumBitfieldStruct<u8, Idtsel_SPEC>;
    impl Idtsel {
        #[doc = "When VCC >= Vdet1 (rise) is detected"]
        pub const _00: Self = Self::new(0);

        #[doc = "When VCC < Vdet1 (fall) is detected"]
        pub const _01: Self = Self::new(1);

        #[doc = "When fall and rise are detected"]
        pub const _10: Self = Self::new(2);

        #[doc = "Settings prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqsel_SPEC;
    pub type Irqsel = crate::EnumBitfieldStruct<u8, Irqsel_SPEC>;
    impl Irqsel {
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Maskable interrupt"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd1Sr_SPEC;
impl crate::sealed::RegSpec for Lvd1Sr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 1 Circuit Status Register"]
pub type Lvd1Sr = crate::RegValueT<Lvd1Sr_SPEC>;

impl Lvd1Sr {
    #[doc = "Voltage Monitor 1 Voltage Variation Detection Flag"]
    #[inline(always)]
    pub fn det(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lvd1sr::Det,
        lvd1sr::Det,
        Lvd1Sr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lvd1sr::Det,
            lvd1sr::Det,
            Lvd1Sr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Signal Monitor Flag"]
    #[inline(always)]
    pub fn mon(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        lvd1sr::Mon,
        lvd1sr::Mon,
        Lvd1Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            lvd1sr::Mon,
            lvd1sr::Mon,
            Lvd1Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd1Sr {
    #[inline(always)]
    fn default() -> Lvd1Sr {
        <crate::RegValueT<Lvd1Sr_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod lvd1sr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Det_SPEC;
    pub type Det = crate::EnumBitfieldStruct<u8, Det_SPEC>;
    impl Det {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Vdet1 crossing is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mon_SPEC;
    pub type Mon = crate::EnumBitfieldStruct<u8, Mon_SPEC>;
    impl Mon {
        #[doc = "VCC < Vdet1"]
        pub const _0: Self = Self::new(0);

        #[doc = "VCC >= Vdet1 or MON is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd2Cr1_SPEC;
impl crate::sealed::RegSpec for Lvd2Cr1_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 2 Circuit Control Register 1"]
pub type Lvd2Cr1 = crate::RegValueT<Lvd2Cr1_SPEC>;

impl Lvd2Cr1 {
    #[doc = "Voltage Monitor 2 Interrupt Generation Condition Select"]
    #[inline(always)]
    pub fn idtsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        lvd2cr1::Idtsel,
        lvd2cr1::Idtsel,
        Lvd2Cr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            lvd2cr1::Idtsel,
            lvd2cr1::Idtsel,
            Lvd2Cr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Interrupt Type Select"]
    #[inline(always)]
    pub fn irqsel(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        lvd2cr1::Irqsel,
        lvd2cr1::Irqsel,
        Lvd2Cr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            lvd2cr1::Irqsel,
            lvd2cr1::Irqsel,
            Lvd2Cr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd2Cr1 {
    #[inline(always)]
    fn default() -> Lvd2Cr1 {
        <crate::RegValueT<Lvd2Cr1_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod lvd2cr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idtsel_SPEC;
    pub type Idtsel = crate::EnumBitfieldStruct<u8, Idtsel_SPEC>;
    impl Idtsel {
        #[doc = "When VCC>= Vdet2 (rise) is detected"]
        pub const _00: Self = Self::new(0);

        #[doc = "When VCC < Vdet2 (fall) is detected"]
        pub const _01: Self = Self::new(1);

        #[doc = "When fall and rise are detected"]
        pub const _10: Self = Self::new(2);

        #[doc = "Settings prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqsel_SPEC;
    pub type Irqsel = crate::EnumBitfieldStruct<u8, Irqsel_SPEC>;
    impl Irqsel {
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Maskable interrupt"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd2Sr_SPEC;
impl crate::sealed::RegSpec for Lvd2Sr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 2 Circuit Status Register"]
pub type Lvd2Sr = crate::RegValueT<Lvd2Sr_SPEC>;

impl Lvd2Sr {
    #[doc = "Voltage Monitor 2 Voltage Variation Detection Flag"]
    #[inline(always)]
    pub fn det(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lvd2sr::Det,
        lvd2sr::Det,
        Lvd2Sr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lvd2sr::Det,
            lvd2sr::Det,
            Lvd2Sr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Signal Monitor Flag"]
    #[inline(always)]
    pub fn mon(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        lvd2sr::Mon,
        lvd2sr::Mon,
        Lvd2Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            lvd2sr::Mon,
            lvd2sr::Mon,
            Lvd2Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd2Sr {
    #[inline(always)]
    fn default() -> Lvd2Sr {
        <crate::RegValueT<Lvd2Sr_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod lvd2sr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Det_SPEC;
    pub type Det = crate::EnumBitfieldStruct<u8, Det_SPEC>;
    impl Det {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Vdet2 crossing is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mon_SPEC;
    pub type Mon = crate::EnumBitfieldStruct<u8, Mon_SPEC>;
    impl Mon {
        #[doc = "VCC < Vdet2"]
        pub const _0: Self = Self::new(0);

        #[doc = "VCC>= Vdet2 or MON is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cgfsar_SPEC;
impl crate::sealed::RegSpec for Cgfsar_SPEC {
    type DataType = u32;
}

#[doc = "Clock Generation Function Security Attribute Register"]
pub type Cgfsar = crate::RegValueT<Cgfsar_SPEC>;

impl Cgfsar {
    #[doc = "Non Secure Attribute bit 00"]
    #[inline(always)]
    pub fn nonsec00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cgfsar::Nonsec00,
        cgfsar::Nonsec00,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cgfsar::Nonsec00,
            cgfsar::Nonsec00,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 02"]
    #[inline(always)]
    pub fn nonsec02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cgfsar::Nonsec02,
        cgfsar::Nonsec02,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cgfsar::Nonsec02,
            cgfsar::Nonsec02,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 03"]
    #[inline(always)]
    pub fn nonsec03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cgfsar::Nonsec03,
        cgfsar::Nonsec03,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cgfsar::Nonsec03,
            cgfsar::Nonsec03,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 04"]
    #[inline(always)]
    pub fn nonsec04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cgfsar::Nonsec04,
        cgfsar::Nonsec04,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cgfsar::Nonsec04,
            cgfsar::Nonsec04,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 05"]
    #[inline(always)]
    pub fn nonsec05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cgfsar::Nonsec05,
        cgfsar::Nonsec05,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cgfsar::Nonsec05,
            cgfsar::Nonsec05,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 06"]
    #[inline(always)]
    pub fn nonsec06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        cgfsar::Nonsec06,
        cgfsar::Nonsec06,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cgfsar::Nonsec06,
            cgfsar::Nonsec06,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 07"]
    #[inline(always)]
    pub fn nonsec07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cgfsar::Nonsec07,
        cgfsar::Nonsec07,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cgfsar::Nonsec07,
            cgfsar::Nonsec07,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 08"]
    #[inline(always)]
    pub fn nonsec08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cgfsar::Nonsec08,
        cgfsar::Nonsec08,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cgfsar::Nonsec08,
            cgfsar::Nonsec08,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 09"]
    #[inline(always)]
    pub fn nonsec09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cgfsar::Nonsec09,
        cgfsar::Nonsec09,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cgfsar::Nonsec09,
            cgfsar::Nonsec09,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 11"]
    #[inline(always)]
    pub fn nonsec11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        cgfsar::Nonsec11,
        cgfsar::Nonsec11,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            cgfsar::Nonsec11,
            cgfsar::Nonsec11,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 16"]
    #[inline(always)]
    pub fn nonsec16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cgfsar::Nonsec16,
        cgfsar::Nonsec16,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cgfsar::Nonsec16,
            cgfsar::Nonsec16,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cgfsar {
    #[inline(always)]
    fn default() -> Cgfsar {
        <crate::RegValueT<Cgfsar_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod cgfsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec00_SPEC;
    pub type Nonsec00 = crate::EnumBitfieldStruct<u8, Nonsec00_SPEC>;
    impl Nonsec00 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec02_SPEC;
    pub type Nonsec02 = crate::EnumBitfieldStruct<u8, Nonsec02_SPEC>;
    impl Nonsec02 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec03_SPEC;
    pub type Nonsec03 = crate::EnumBitfieldStruct<u8, Nonsec03_SPEC>;
    impl Nonsec03 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec04_SPEC;
    pub type Nonsec04 = crate::EnumBitfieldStruct<u8, Nonsec04_SPEC>;
    impl Nonsec04 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec05_SPEC;
    pub type Nonsec05 = crate::EnumBitfieldStruct<u8, Nonsec05_SPEC>;
    impl Nonsec05 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec06_SPEC;
    pub type Nonsec06 = crate::EnumBitfieldStruct<u8, Nonsec06_SPEC>;
    impl Nonsec06 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec07_SPEC;
    pub type Nonsec07 = crate::EnumBitfieldStruct<u8, Nonsec07_SPEC>;
    impl Nonsec07 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec08_SPEC;
    pub type Nonsec08 = crate::EnumBitfieldStruct<u8, Nonsec08_SPEC>;
    impl Nonsec08 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec09_SPEC;
    pub type Nonsec09 = crate::EnumBitfieldStruct<u8, Nonsec09_SPEC>;
    impl Nonsec09 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec11_SPEC;
    pub type Nonsec11 = crate::EnumBitfieldStruct<u8, Nonsec11_SPEC>;
    impl Nonsec11 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec16_SPEC;
    pub type Nonsec16 = crate::EnumBitfieldStruct<u8, Nonsec16_SPEC>;
    impl Nonsec16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsar_SPEC;
impl crate::sealed::RegSpec for Rstsar_SPEC {
    type DataType = u32;
}

#[doc = "Reset Security Attribution Register"]
pub type Rstsar = crate::RegValueT<Rstsar_SPEC>;

impl Rstsar {
    #[doc = "Non Secure Attribute bit 0"]
    #[inline(always)]
    pub fn nonsec0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rstsar::Nonsec0,
        rstsar::Nonsec0,
        Rstsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rstsar::Nonsec0,
            rstsar::Nonsec0,
            Rstsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 1"]
    #[inline(always)]
    pub fn nonsec1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rstsar::Nonsec1,
        rstsar::Nonsec1,
        Rstsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rstsar::Nonsec1,
            rstsar::Nonsec1,
            Rstsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 2"]
    #[inline(always)]
    pub fn nonsec2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rstsar::Nonsec2,
        rstsar::Nonsec2,
        Rstsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rstsar::Nonsec2,
            rstsar::Nonsec2,
            Rstsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rstsar {
    #[inline(always)]
    fn default() -> Rstsar {
        <crate::RegValueT<Rstsar_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod rstsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec0_SPEC;
    pub type Nonsec0 = crate::EnumBitfieldStruct<u8, Nonsec0_SPEC>;
    impl Nonsec0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec1_SPEC;
    pub type Nonsec1 = crate::EnumBitfieldStruct<u8, Nonsec1_SPEC>;
    impl Nonsec1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec2_SPEC;
    pub type Nonsec2 = crate::EnumBitfieldStruct<u8, Nonsec2_SPEC>;
    impl Nonsec2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpmsar_SPEC;
impl crate::sealed::RegSpec for Lpmsar_SPEC {
    type DataType = u32;
}

#[doc = "Low Power Mode Security Attribution Register"]
pub type Lpmsar = crate::RegValueT<Lpmsar_SPEC>;

impl Lpmsar {
    #[doc = "Non Secure Attribute bit 0"]
    #[inline(always)]
    pub fn nonsec0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lpmsar::Nonsec0,
        lpmsar::Nonsec0,
        Lpmsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lpmsar::Nonsec0,
            lpmsar::Nonsec0,
            Lpmsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 2"]
    #[inline(always)]
    pub fn nonsec2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        lpmsar::Nonsec2,
        lpmsar::Nonsec2,
        Lpmsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            lpmsar::Nonsec2,
            lpmsar::Nonsec2,
            Lpmsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 4"]
    #[inline(always)]
    pub fn nonsec4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        lpmsar::Nonsec4,
        lpmsar::Nonsec4,
        Lpmsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            lpmsar::Nonsec4,
            lpmsar::Nonsec4,
            Lpmsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 8"]
    #[inline(always)]
    pub fn nonsec8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        lpmsar::Nonsec8,
        lpmsar::Nonsec8,
        Lpmsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            lpmsar::Nonsec8,
            lpmsar::Nonsec8,
            Lpmsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 9"]
    #[inline(always)]
    pub fn nonsec9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        lpmsar::Nonsec9,
        lpmsar::Nonsec9,
        Lpmsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            lpmsar::Nonsec9,
            lpmsar::Nonsec9,
            Lpmsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 13"]
    #[inline(always)]
    pub fn nonsec13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        lpmsar::Nonsec13,
        lpmsar::Nonsec13,
        Lpmsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            lpmsar::Nonsec13,
            lpmsar::Nonsec13,
            Lpmsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lpmsar {
    #[inline(always)]
    fn default() -> Lpmsar {
        <crate::RegValueT<Lpmsar_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod lpmsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec0_SPEC;
    pub type Nonsec0 = crate::EnumBitfieldStruct<u8, Nonsec0_SPEC>;
    impl Nonsec0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec2_SPEC;
    pub type Nonsec2 = crate::EnumBitfieldStruct<u8, Nonsec2_SPEC>;
    impl Nonsec2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec4_SPEC;
    pub type Nonsec4 = crate::EnumBitfieldStruct<u8, Nonsec4_SPEC>;
    impl Nonsec4 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec8_SPEC;
    pub type Nonsec8 = crate::EnumBitfieldStruct<u8, Nonsec8_SPEC>;
    impl Nonsec8 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec9_SPEC;
    pub type Nonsec9 = crate::EnumBitfieldStruct<u8, Nonsec9_SPEC>;
    impl Nonsec9 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec13_SPEC;
    pub type Nonsec13 = crate::EnumBitfieldStruct<u8, Nonsec13_SPEC>;
    impl Nonsec13 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdsar_SPEC;
impl crate::sealed::RegSpec for Lvdsar_SPEC {
    type DataType = u32;
}

#[doc = "Low Voltage Detection Security Attribution Register"]
pub type Lvdsar = crate::RegValueT<Lvdsar_SPEC>;

impl Lvdsar {
    #[doc = "Non Secure Attribute bit 0"]
    #[inline(always)]
    pub fn nonsec0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lvdsar::Nonsec0,
        lvdsar::Nonsec0,
        Lvdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lvdsar::Nonsec0,
            lvdsar::Nonsec0,
            Lvdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 1"]
    #[inline(always)]
    pub fn nonsec1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        lvdsar::Nonsec1,
        lvdsar::Nonsec1,
        Lvdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            lvdsar::Nonsec1,
            lvdsar::Nonsec1,
            Lvdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvdsar {
    #[inline(always)]
    fn default() -> Lvdsar {
        <crate::RegValueT<Lvdsar_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod lvdsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec0_SPEC;
    pub type Nonsec0 = crate::EnumBitfieldStruct<u8, Nonsec0_SPEC>;
    impl Nonsec0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec1_SPEC;
    pub type Nonsec1 = crate::EnumBitfieldStruct<u8, Nonsec1_SPEC>;
    impl Nonsec1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bbfsar_SPEC;
impl crate::sealed::RegSpec for Bbfsar_SPEC {
    type DataType = u32;
}

#[doc = "Battery Backup Function Security Attribute Register"]
pub type Bbfsar = crate::RegValueT<Bbfsar_SPEC>;

impl Bbfsar {
    #[doc = "Non Secure Attribute bit 0"]
    #[inline(always)]
    pub fn nonsec0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bbfsar::Nonsec0,
        bbfsar::Nonsec0,
        Bbfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bbfsar::Nonsec0,
            bbfsar::Nonsec0,
            Bbfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 1"]
    #[inline(always)]
    pub fn nonsec1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bbfsar::Nonsec1,
        bbfsar::Nonsec1,
        Bbfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bbfsar::Nonsec1,
            bbfsar::Nonsec1,
            Bbfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 2"]
    #[inline(always)]
    pub fn nonsec2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        bbfsar::Nonsec2,
        bbfsar::Nonsec2,
        Bbfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            bbfsar::Nonsec2,
            bbfsar::Nonsec2,
            Bbfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 16"]
    #[inline(always)]
    pub fn nonsec16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        bbfsar::Nonsec16,
        bbfsar::Nonsec16,
        Bbfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            bbfsar::Nonsec16,
            bbfsar::Nonsec16,
            Bbfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 17"]
    #[inline(always)]
    pub fn nonsec17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        bbfsar::Nonsec17,
        bbfsar::Nonsec17,
        Bbfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            bbfsar::Nonsec17,
            bbfsar::Nonsec17,
            Bbfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 18"]
    #[inline(always)]
    pub fn nonsec18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        bbfsar::Nonsec18,
        bbfsar::Nonsec18,
        Bbfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            bbfsar::Nonsec18,
            bbfsar::Nonsec18,
            Bbfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 19"]
    #[inline(always)]
    pub fn nonsec19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        bbfsar::Nonsec19,
        bbfsar::Nonsec19,
        Bbfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            bbfsar::Nonsec19,
            bbfsar::Nonsec19,
            Bbfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 20"]
    #[inline(always)]
    pub fn nonsec20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        bbfsar::Nonsec20,
        bbfsar::Nonsec20,
        Bbfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            bbfsar::Nonsec20,
            bbfsar::Nonsec20,
            Bbfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 21"]
    #[inline(always)]
    pub fn nonsec21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        bbfsar::Nonsec21,
        bbfsar::Nonsec21,
        Bbfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            bbfsar::Nonsec21,
            bbfsar::Nonsec21,
            Bbfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 22"]
    #[inline(always)]
    pub fn nonsec22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        bbfsar::Nonsec22,
        bbfsar::Nonsec22,
        Bbfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            bbfsar::Nonsec22,
            bbfsar::Nonsec22,
            Bbfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 23"]
    #[inline(always)]
    pub fn nonsec23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        bbfsar::Nonsec23,
        bbfsar::Nonsec23,
        Bbfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            bbfsar::Nonsec23,
            bbfsar::Nonsec23,
            Bbfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bbfsar {
    #[inline(always)]
    fn default() -> Bbfsar {
        <crate::RegValueT<Bbfsar_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod bbfsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec0_SPEC;
    pub type Nonsec0 = crate::EnumBitfieldStruct<u8, Nonsec0_SPEC>;
    impl Nonsec0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec1_SPEC;
    pub type Nonsec1 = crate::EnumBitfieldStruct<u8, Nonsec1_SPEC>;
    impl Nonsec1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec2_SPEC;
    pub type Nonsec2 = crate::EnumBitfieldStruct<u8, Nonsec2_SPEC>;
    impl Nonsec2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec16_SPEC;
    pub type Nonsec16 = crate::EnumBitfieldStruct<u8, Nonsec16_SPEC>;
    impl Nonsec16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec17_SPEC;
    pub type Nonsec17 = crate::EnumBitfieldStruct<u8, Nonsec17_SPEC>;
    impl Nonsec17 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec18_SPEC;
    pub type Nonsec18 = crate::EnumBitfieldStruct<u8, Nonsec18_SPEC>;
    impl Nonsec18 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec19_SPEC;
    pub type Nonsec19 = crate::EnumBitfieldStruct<u8, Nonsec19_SPEC>;
    impl Nonsec19 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec20_SPEC;
    pub type Nonsec20 = crate::EnumBitfieldStruct<u8, Nonsec20_SPEC>;
    impl Nonsec20 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec21_SPEC;
    pub type Nonsec21 = crate::EnumBitfieldStruct<u8, Nonsec21_SPEC>;
    impl Nonsec21 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec22_SPEC;
    pub type Nonsec22 = crate::EnumBitfieldStruct<u8, Nonsec22_SPEC>;
    impl Nonsec22 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec23_SPEC;
    pub type Nonsec23 = crate::EnumBitfieldStruct<u8, Nonsec23_SPEC>;
    impl Nonsec23 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpfsar_SPEC;
impl crate::sealed::RegSpec for Dpfsar_SPEC {
    type DataType = u32;
}

#[doc = "Deep Software Standby Interrupt Factor Security Attribution Register"]
pub type Dpfsar = crate::RegValueT<Dpfsar_SPEC>;

impl Dpfsar {
    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit 0"]
    #[inline(always)]
    pub fn dpfsa00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpfsar::Dpfsa00,
        dpfsar::Dpfsa00,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpfsar::Dpfsa00,
            dpfsar::Dpfsa00,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit 1"]
    #[inline(always)]
    pub fn dpfsa01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpfsar::Dpfsa01,
        dpfsar::Dpfsa01,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpfsar::Dpfsa01,
            dpfsar::Dpfsa01,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 4 to 9)"]
    #[inline(always)]
    pub fn dpfsa04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpfsar::Dpfsa04,
        dpfsar::Dpfsa04,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpfsar::Dpfsa04,
            dpfsar::Dpfsa04,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 4 to 9)"]
    #[inline(always)]
    pub fn dpfsa05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dpfsar::Dpfsa05,
        dpfsar::Dpfsa05,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dpfsar::Dpfsa05,
            dpfsar::Dpfsa05,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 4 to 9)"]
    #[inline(always)]
    pub fn dpfsa06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dpfsar::Dpfsa06,
        dpfsar::Dpfsa06,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dpfsar::Dpfsa06,
            dpfsar::Dpfsa06,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 4 to 9)"]
    #[inline(always)]
    pub fn dpfsa07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpfsar::Dpfsa07,
        dpfsar::Dpfsa07,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpfsar::Dpfsa07,
            dpfsar::Dpfsa07,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 4 to 9)"]
    #[inline(always)]
    pub fn dpfsa08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dpfsar::Dpfsa08,
        dpfsar::Dpfsa08,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dpfsar::Dpfsa08,
            dpfsar::Dpfsa08,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit n (n = 4 to 9)"]
    #[inline(always)]
    pub fn dpfsa09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        dpfsar::Dpfsa09,
        dpfsar::Dpfsa09,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            dpfsar::Dpfsa09,
            dpfsar::Dpfsa09,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit 16"]
    #[inline(always)]
    pub fn dpfsa16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dpfsar::Dpfsa16,
        dpfsar::Dpfsa16,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dpfsar::Dpfsa16,
            dpfsar::Dpfsa16,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit 17"]
    #[inline(always)]
    pub fn dpfsa17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        dpfsar::Dpfsa17,
        dpfsar::Dpfsa17,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            dpfsar::Dpfsa17,
            dpfsar::Dpfsa17,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit 18"]
    #[inline(always)]
    pub fn dpfsa18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        dpfsar::Dpfsa18,
        dpfsar::Dpfsa18,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            dpfsar::Dpfsa18,
            dpfsar::Dpfsa18,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit 19"]
    #[inline(always)]
    pub fn dpfsa19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        dpfsar::Dpfsa19,
        dpfsar::Dpfsa19,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            dpfsar::Dpfsa19,
            dpfsar::Dpfsa19,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit 20"]
    #[inline(always)]
    pub fn dpfsa20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        dpfsar::Dpfsa20,
        dpfsar::Dpfsa20,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            dpfsar::Dpfsa20,
            dpfsar::Dpfsa20,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit 24"]
    #[inline(always)]
    pub fn dpfsa24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        dpfsar::Dpfsa24,
        dpfsar::Dpfsa24,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            dpfsar::Dpfsa24,
            dpfsar::Dpfsa24,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit 26"]
    #[inline(always)]
    pub fn dpfsa26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        dpfsar::Dpfsa26,
        dpfsar::Dpfsa26,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            dpfsar::Dpfsa26,
            dpfsar::Dpfsa26,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Interrupt Factor Security Attribute bit 27"]
    #[inline(always)]
    pub fn dpfsa27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        dpfsar::Dpfsa27,
        dpfsar::Dpfsa27,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            dpfsar::Dpfsa27,
            dpfsar::Dpfsa27,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpfsar {
    #[inline(always)]
    fn default() -> Dpfsar {
        <crate::RegValueT<Dpfsar_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dpfsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa00_SPEC;
    pub type Dpfsa00 = crate::EnumBitfieldStruct<u8, Dpfsa00_SPEC>;
    impl Dpfsa00 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa01_SPEC;
    pub type Dpfsa01 = crate::EnumBitfieldStruct<u8, Dpfsa01_SPEC>;
    impl Dpfsa01 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa04_SPEC;
    pub type Dpfsa04 = crate::EnumBitfieldStruct<u8, Dpfsa04_SPEC>;
    impl Dpfsa04 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa05_SPEC;
    pub type Dpfsa05 = crate::EnumBitfieldStruct<u8, Dpfsa05_SPEC>;
    impl Dpfsa05 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa06_SPEC;
    pub type Dpfsa06 = crate::EnumBitfieldStruct<u8, Dpfsa06_SPEC>;
    impl Dpfsa06 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa07_SPEC;
    pub type Dpfsa07 = crate::EnumBitfieldStruct<u8, Dpfsa07_SPEC>;
    impl Dpfsa07 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa08_SPEC;
    pub type Dpfsa08 = crate::EnumBitfieldStruct<u8, Dpfsa08_SPEC>;
    impl Dpfsa08 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa09_SPEC;
    pub type Dpfsa09 = crate::EnumBitfieldStruct<u8, Dpfsa09_SPEC>;
    impl Dpfsa09 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa16_SPEC;
    pub type Dpfsa16 = crate::EnumBitfieldStruct<u8, Dpfsa16_SPEC>;
    impl Dpfsa16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa17_SPEC;
    pub type Dpfsa17 = crate::EnumBitfieldStruct<u8, Dpfsa17_SPEC>;
    impl Dpfsa17 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa18_SPEC;
    pub type Dpfsa18 = crate::EnumBitfieldStruct<u8, Dpfsa18_SPEC>;
    impl Dpfsa18 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa19_SPEC;
    pub type Dpfsa19 = crate::EnumBitfieldStruct<u8, Dpfsa19_SPEC>;
    impl Dpfsa19 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa20_SPEC;
    pub type Dpfsa20 = crate::EnumBitfieldStruct<u8, Dpfsa20_SPEC>;
    impl Dpfsa20 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa24_SPEC;
    pub type Dpfsa24 = crate::EnumBitfieldStruct<u8, Dpfsa24_SPEC>;
    impl Dpfsa24 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa26_SPEC;
    pub type Dpfsa26 = crate::EnumBitfieldStruct<u8, Dpfsa26_SPEC>;
    impl Dpfsa26 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa27_SPEC;
    pub type Dpfsa27 = crate::EnumBitfieldStruct<u8, Dpfsa27_SPEC>;
    impl Dpfsa27 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prcr_SPEC;
impl crate::sealed::RegSpec for Prcr_SPEC {
    type DataType = u16;
}

#[doc = "Protect Register"]
pub type Prcr = crate::RegValueT<Prcr_SPEC>;

impl Prcr {
    #[doc = "Enable writing to the registers related to the clock generation circuit"]
    #[inline(always)]
    pub fn prc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        prcr::Prc0,
        prcr::Prc0,
        Prcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            prcr::Prc0,
            prcr::Prc0,
            Prcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable writing to the registers related to the low power modes, and the battery backup function"]
    #[inline(always)]
    pub fn prc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        prcr::Prc1,
        prcr::Prc1,
        Prcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            prcr::Prc1,
            prcr::Prc1,
            Prcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable writing to the registers related to the LVD"]
    #[inline(always)]
    pub fn prc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        prcr::Prc3,
        prcr::Prc3,
        Prcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            prcr::Prc3,
            prcr::Prc3,
            Prcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn prc4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        prcr::Prc4,
        prcr::Prc4,
        Prcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            prcr::Prc4,
            prcr::Prc4,
            Prcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PRC Key Code"]
    #[inline(always)]
    pub fn prkey(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Prcr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Prcr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Prcr {
    #[inline(always)]
    fn default() -> Prcr {
        <crate::RegValueT<Prcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod prcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc0_SPEC;
    pub type Prc0 = crate::EnumBitfieldStruct<u8, Prc0_SPEC>;
    impl Prc0 {
        #[doc = "Disable writes"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc1_SPEC;
    pub type Prc1 = crate::EnumBitfieldStruct<u8, Prc1_SPEC>;
    impl Prc1 {
        #[doc = "Disable writes"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc3_SPEC;
    pub type Prc3 = crate::EnumBitfieldStruct<u8, Prc3_SPEC>;
    impl Prc3 {
        #[doc = "Disable writes"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc4_SPEC;
    pub type Prc4 = crate::EnumBitfieldStruct<u8, Prc4_SPEC>;
    impl Prc4 {
        #[doc = "Disable writes"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsbycr_SPEC;
impl crate::sealed::RegSpec for Dpsbycr_SPEC {
    type DataType = u8;
}

#[doc = "Deep Software Standby Control Register"]
pub type Dpsbycr = crate::RegValueT<Dpsbycr_SPEC>;

impl Dpsbycr {
    #[doc = "Power-Supply Control"]
    #[inline(always)]
    pub fn deepcut(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        dpsbycr::Deepcut,
        dpsbycr::Deepcut,
        Dpsbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            dpsbycr::Deepcut,
            dpsbycr::Deepcut,
            Dpsbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I/O Port Retention"]
    #[inline(always)]
    pub fn iokeep(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dpsbycr::Iokeep,
        dpsbycr::Iokeep,
        Dpsbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dpsbycr::Iokeep,
            dpsbycr::Iokeep,
            Dpsbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby"]
    #[inline(always)]
    pub fn dpsby(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpsbycr::Dpsby,
        dpsbycr::Dpsby,
        Dpsbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpsbycr::Dpsby,
            dpsbycr::Dpsby,
            Dpsbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsbycr {
    #[inline(always)]
    fn default() -> Dpsbycr {
        <crate::RegValueT<Dpsbycr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod dpsbycr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Deepcut_SPEC;
    pub type Deepcut = crate::EnumBitfieldStruct<u8, Deepcut_SPEC>;
    impl Deepcut {
        #[doc = "Power to the standby RAM, Low-speed on-chip oscillator, AGTn (n = 0 to 3), and USBFS resume detecting unit is supplied in Deep Software Standby mode."]
        pub const _00: Self = Self::new(0);

        #[doc = "Power to the standby RAM, Low-speed on-chip oscillator, AGT, and USBFS resume detecting unit is not supplied in Deep Software Standby mode."]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "Power to the standby RAM, Low-speed on-chip oscillator, AGT, and USBFS resume detecting unit is not supplied in Deep Software Standby mode. In addition, LVD is disabled and the low power function in a power-on reset circuit is enabled."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iokeep_SPEC;
    pub type Iokeep = crate::EnumBitfieldStruct<u8, Iokeep_SPEC>;
    impl Iokeep {
        #[doc = "When the Deep Software Standby mode is canceled, the I/O ports are in the reset state."]
        pub const _0: Self = Self::new(0);

        #[doc = "When the Deep Software Standby mode is canceled, the I/O ports are in the same state as in the Deep Software Standby mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpsby_SPEC;
    pub type Dpsby = crate::EnumBitfieldStruct<u8, Dpsby_SPEC>;
    impl Dpsby {
        #[doc = "Sleep mode (SBYCR.SSBY=0) / Software Standby mode (SBYCR.SSBY=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sleep mode (SBYCR.SSBY=0)  / Deep Software Standby mode (SBYCR.SSBY=1)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpswcr_SPEC;
impl crate::sealed::RegSpec for Dpswcr_SPEC {
    type DataType = u8;
}

#[doc = "Deep Software Standby Wait Control Register"]
pub type Dpswcr = crate::RegValueT<Dpswcr_SPEC>;

impl Dpswcr {
    #[doc = "Deep Software Wait Standby Time Setting Bit"]
    #[inline(always)]
    pub fn wtsts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        dpswcr::Wtsts,
        dpswcr::Wtsts,
        Dpswcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            dpswcr::Wtsts,
            dpswcr::Wtsts,
            Dpswcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpswcr {
    #[inline(always)]
    fn default() -> Dpswcr {
        <crate::RegValueT<Dpswcr_SPEC> as RegisterValue<_>>::new(25)
    }
}
pub mod dpswcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wtsts_SPEC;
    pub type Wtsts = crate::EnumBitfieldStruct<u8, Wtsts_SPEC>;
    impl Wtsts {
        #[doc = "Wait cycle for fast recovery"]
        pub const _0_X_0_E: Self = Self::new(14);

        #[doc = "Wait cycle for slow recovery"]
        pub const _0_X_19: Self = Self::new(25);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsier0_SPEC;
impl crate::sealed::RegSpec for Dpsier0_SPEC {
    type DataType = u8;
}

#[doc = "Deep Software Standby Interrupt Enable Register 0"]
pub type Dpsier0 = crate::RegValueT<Dpsier0_SPEC>;

impl Dpsier0 {
    #[doc = "IRQ0-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq0e(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsier0::Dirq0E,
        dpsier0::Dirq0E,
        Dpsier0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsier0::Dirq0E,
            dpsier0::Dirq0E,
            Dpsier0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ1-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq1e(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsier0::Dirq1E,
        dpsier0::Dirq1E,
        Dpsier0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsier0::Dirq1E,
            dpsier0::Dirq1E,
            Dpsier0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ4-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq4e(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsier0::Dirq4E,
        dpsier0::Dirq4E,
        Dpsier0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsier0::Dirq4E,
            dpsier0::Dirq4E,
            Dpsier0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ5-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq5e(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dpsier0::Dirq5E,
        dpsier0::Dirq5E,
        Dpsier0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dpsier0::Dirq5E,
            dpsier0::Dirq5E,
            Dpsier0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ6-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq6e(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dpsier0::Dirq6E,
        dpsier0::Dirq6E,
        Dpsier0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dpsier0::Dirq6E,
            dpsier0::Dirq6E,
            Dpsier0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ7-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq7e(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpsier0::Dirq7E,
        dpsier0::Dirq7E,
        Dpsier0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpsier0::Dirq7E,
            dpsier0::Dirq7E,
            Dpsier0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsier0 {
    #[inline(always)]
    fn default() -> Dpsier0 {
        <crate::RegValueT<Dpsier0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsier0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq0E_SPEC;
    pub type Dirq0E = crate::EnumBitfieldStruct<u8, Dirq0E_SPEC>;
    impl Dirq0E {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq1E_SPEC;
    pub type Dirq1E = crate::EnumBitfieldStruct<u8, Dirq1E_SPEC>;
    impl Dirq1E {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq4E_SPEC;
    pub type Dirq4E = crate::EnumBitfieldStruct<u8, Dirq4E_SPEC>;
    impl Dirq4E {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq5E_SPEC;
    pub type Dirq5E = crate::EnumBitfieldStruct<u8, Dirq5E_SPEC>;
    impl Dirq5E {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq6E_SPEC;
    pub type Dirq6E = crate::EnumBitfieldStruct<u8, Dirq6E_SPEC>;
    impl Dirq6E {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq7E_SPEC;
    pub type Dirq7E = crate::EnumBitfieldStruct<u8, Dirq7E_SPEC>;
    impl Dirq7E {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsier1_SPEC;
impl crate::sealed::RegSpec for Dpsier1_SPEC {
    type DataType = u8;
}

#[doc = "Deep Software Standby Interrupt Enable Register 1"]
pub type Dpsier1 = crate::RegValueT<Dpsier1_SPEC>;

impl Dpsier1 {
    #[doc = "IRQ8-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq8e(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsier1::Dirq8E,
        dpsier1::Dirq8E,
        Dpsier1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsier1::Dirq8E,
            dpsier1::Dirq8E,
            Dpsier1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ9-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq9e(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsier1::Dirq9E,
        dpsier1::Dirq9E,
        Dpsier1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsier1::Dirq9E,
            dpsier1::Dirq9E,
            Dpsier1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsier1 {
    #[inline(always)]
    fn default() -> Dpsier1 {
        <crate::RegValueT<Dpsier1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsier1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq8E_SPEC;
    pub type Dirq8E = crate::EnumBitfieldStruct<u8, Dirq8E_SPEC>;
    impl Dirq8E {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq9E_SPEC;
    pub type Dirq9E = crate::EnumBitfieldStruct<u8, Dirq9E_SPEC>;
    impl Dirq9E {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsier2_SPEC;
impl crate::sealed::RegSpec for Dpsier2_SPEC {
    type DataType = u8;
}

#[doc = "Deep Software Standby Interrupt Enable Register 2"]
pub type Dpsier2 = crate::RegValueT<Dpsier2_SPEC>;

impl Dpsier2 {
    #[doc = "LVD1 Deep Software Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dlvd1ie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsier2::Dlvd1Ie,
        dpsier2::Dlvd1Ie,
        Dpsier2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsier2::Dlvd1Ie,
            dpsier2::Dlvd1Ie,
            Dpsier2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVD2 Deep Software Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dlvd2ie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsier2::Dlvd2Ie,
        dpsier2::Dlvd2Ie,
        Dpsier2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsier2::Dlvd2Ie,
            dpsier2::Dlvd2Ie,
            Dpsier2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Interval interrupt Deep Software Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn drtciie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsier2::Drtciie,
        dpsier2::Drtciie,
        Dpsier2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsier2::Drtciie,
            dpsier2::Drtciie,
            Dpsier2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Alarm interrupt Deep Software Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn drtcaie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsier2::Drtcaie,
        dpsier2::Drtcaie,
        Dpsier2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsier2::Drtcaie,
            dpsier2::Drtcaie,
            Dpsier2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NMI Pin Enable"]
    #[inline(always)]
    pub fn dnmie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsier2::Dnmie,
        dpsier2::Dnmie,
        Dpsier2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsier2::Dnmie,
            dpsier2::Dnmie,
            Dpsier2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsier2 {
    #[inline(always)]
    fn default() -> Dpsier2 {
        <crate::RegValueT<Dpsier2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsier2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlvd1Ie_SPEC;
    pub type Dlvd1Ie = crate::EnumBitfieldStruct<u8, Dlvd1Ie_SPEC>;
    impl Dlvd1Ie {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlvd2Ie_SPEC;
    pub type Dlvd2Ie = crate::EnumBitfieldStruct<u8, Dlvd2Ie_SPEC>;
    impl Dlvd2Ie {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drtciie_SPEC;
    pub type Drtciie = crate::EnumBitfieldStruct<u8, Drtciie_SPEC>;
    impl Drtciie {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drtcaie_SPEC;
    pub type Drtcaie = crate::EnumBitfieldStruct<u8, Drtcaie_SPEC>;
    impl Drtcaie {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dnmie_SPEC;
    pub type Dnmie = crate::EnumBitfieldStruct<u8, Dnmie_SPEC>;
    impl Dnmie {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsier3_SPEC;
impl crate::sealed::RegSpec for Dpsier3_SPEC {
    type DataType = u8;
}

#[doc = "Deep Software Standby Interrupt Enable Register 3"]
pub type Dpsier3 = crate::RegValueT<Dpsier3_SPEC>;

impl Dpsier3 {
    #[doc = "USBFS0 Suspend/Resume Deep Software Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dusbfs0ie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsier3::Dusbfs0Ie,
        dpsier3::Dusbfs0Ie,
        Dpsier3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsier3::Dusbfs0Ie,
            dpsier3::Dusbfs0Ie,
            Dpsier3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT1 Underflow Deep Software Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dagt1ie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsier3::Dagt1Ie,
        dpsier3::Dagt1Ie,
        Dpsier3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsier3::Dagt1Ie,
            dpsier3::Dagt1Ie,
            Dpsier3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT3 Underflow Deep Software Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dagt3ie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsier3::Dagt3Ie,
        dpsier3::Dagt3Ie,
        Dpsier3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsier3::Dagt3Ie,
            dpsier3::Dagt3Ie,
            Dpsier3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsier3 {
    #[inline(always)]
    fn default() -> Dpsier3 {
        <crate::RegValueT<Dpsier3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsier3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dusbfs0Ie_SPEC;
    pub type Dusbfs0Ie = crate::EnumBitfieldStruct<u8, Dusbfs0Ie_SPEC>;
    impl Dusbfs0Ie {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dagt1Ie_SPEC;
    pub type Dagt1Ie = crate::EnumBitfieldStruct<u8, Dagt1Ie_SPEC>;
    impl Dagt1Ie {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dagt3Ie_SPEC;
    pub type Dagt3Ie = crate::EnumBitfieldStruct<u8, Dagt3Ie_SPEC>;
    impl Dagt3Ie {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsifr0_SPEC;
impl crate::sealed::RegSpec for Dpsifr0_SPEC {
    type DataType = u8;
}

#[doc = "Deep Software Standby Interrupt Flag Register 0"]
pub type Dpsifr0 = crate::RegValueT<Dpsifr0_SPEC>;

impl Dpsifr0 {
    #[doc = "IRQ0-DS Pin Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq0f(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsifr0::Dirq0F,
        dpsifr0::Dirq0F,
        Dpsifr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsifr0::Dirq0F,
            dpsifr0::Dirq0F,
            Dpsifr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ1-DS Pin Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq1f(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsifr0::Dirq1F,
        dpsifr0::Dirq1F,
        Dpsifr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsifr0::Dirq1F,
            dpsifr0::Dirq1F,
            Dpsifr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ4-DS Pin Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq4f(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsifr0::Dirq4F,
        dpsifr0::Dirq4F,
        Dpsifr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsifr0::Dirq4F,
            dpsifr0::Dirq4F,
            Dpsifr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ5-DS Pin Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq5f(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dpsifr0::Dirq5F,
        dpsifr0::Dirq5F,
        Dpsifr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dpsifr0::Dirq5F,
            dpsifr0::Dirq5F,
            Dpsifr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ6-DS Pin Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq6f(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dpsifr0::Dirq6F,
        dpsifr0::Dirq6F,
        Dpsifr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dpsifr0::Dirq6F,
            dpsifr0::Dirq6F,
            Dpsifr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ7-DS Pin Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq7f(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpsifr0::Dirq7F,
        dpsifr0::Dirq7F,
        Dpsifr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpsifr0::Dirq7F,
            dpsifr0::Dirq7F,
            Dpsifr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsifr0 {
    #[inline(always)]
    fn default() -> Dpsifr0 {
        <crate::RegValueT<Dpsifr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsifr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq0F_SPEC;
    pub type Dirq0F = crate::EnumBitfieldStruct<u8, Dirq0F_SPEC>;
    impl Dirq0F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq1F_SPEC;
    pub type Dirq1F = crate::EnumBitfieldStruct<u8, Dirq1F_SPEC>;
    impl Dirq1F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq4F_SPEC;
    pub type Dirq4F = crate::EnumBitfieldStruct<u8, Dirq4F_SPEC>;
    impl Dirq4F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq5F_SPEC;
    pub type Dirq5F = crate::EnumBitfieldStruct<u8, Dirq5F_SPEC>;
    impl Dirq5F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq6F_SPEC;
    pub type Dirq6F = crate::EnumBitfieldStruct<u8, Dirq6F_SPEC>;
    impl Dirq6F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq7F_SPEC;
    pub type Dirq7F = crate::EnumBitfieldStruct<u8, Dirq7F_SPEC>;
    impl Dirq7F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsifr1_SPEC;
impl crate::sealed::RegSpec for Dpsifr1_SPEC {
    type DataType = u8;
}

#[doc = "Deep Software Standby Interrupt Flag Register 1"]
pub type Dpsifr1 = crate::RegValueT<Dpsifr1_SPEC>;

impl Dpsifr1 {
    #[doc = "IRQ8-DS Pin Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq8f(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsifr1::Dirq8F,
        dpsifr1::Dirq8F,
        Dpsifr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsifr1::Dirq8F,
            dpsifr1::Dirq8F,
            Dpsifr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ9-DS Pin Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq9f(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsifr1::Dirq9F,
        dpsifr1::Dirq9F,
        Dpsifr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsifr1::Dirq9F,
            dpsifr1::Dirq9F,
            Dpsifr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsifr1 {
    #[inline(always)]
    fn default() -> Dpsifr1 {
        <crate::RegValueT<Dpsifr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsifr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq8F_SPEC;
    pub type Dirq8F = crate::EnumBitfieldStruct<u8, Dirq8F_SPEC>;
    impl Dirq8F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq9F_SPEC;
    pub type Dirq9F = crate::EnumBitfieldStruct<u8, Dirq9F_SPEC>;
    impl Dirq9F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsifr2_SPEC;
impl crate::sealed::RegSpec for Dpsifr2_SPEC {
    type DataType = u8;
}

#[doc = "Deep Software Standby Interrupt Flag Register 2"]
pub type Dpsifr2 = crate::RegValueT<Dpsifr2_SPEC>;

impl Dpsifr2 {
    #[doc = "LVD1 Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn dlvd1if(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsifr2::Dlvd1If,
        dpsifr2::Dlvd1If,
        Dpsifr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsifr2::Dlvd1If,
            dpsifr2::Dlvd1If,
            Dpsifr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVD2 Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn dlvd2if(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsifr2::Dlvd2If,
        dpsifr2::Dlvd2If,
        Dpsifr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsifr2::Dlvd2If,
            dpsifr2::Dlvd2If,
            Dpsifr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Interval Interrupt Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn drtciif(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsifr2::Drtciif,
        dpsifr2::Drtciif,
        Dpsifr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsifr2::Drtciif,
            dpsifr2::Drtciif,
            Dpsifr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Alarm Interrupt Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn drtcaif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsifr2::Drtcaif,
        dpsifr2::Drtcaif,
        Dpsifr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsifr2::Drtcaif,
            dpsifr2::Drtcaif,
            Dpsifr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NMI Pin Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn dnmif(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsifr2::Dnmif,
        dpsifr2::Dnmif,
        Dpsifr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsifr2::Dnmif,
            dpsifr2::Dnmif,
            Dpsifr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsifr2 {
    #[inline(always)]
    fn default() -> Dpsifr2 {
        <crate::RegValueT<Dpsifr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsifr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlvd1If_SPEC;
    pub type Dlvd1If = crate::EnumBitfieldStruct<u8, Dlvd1If_SPEC>;
    impl Dlvd1If {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlvd2If_SPEC;
    pub type Dlvd2If = crate::EnumBitfieldStruct<u8, Dlvd2If_SPEC>;
    impl Dlvd2If {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drtciif_SPEC;
    pub type Drtciif = crate::EnumBitfieldStruct<u8, Drtciif_SPEC>;
    impl Drtciif {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drtcaif_SPEC;
    pub type Drtcaif = crate::EnumBitfieldStruct<u8, Drtcaif_SPEC>;
    impl Drtcaif {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dnmif_SPEC;
    pub type Dnmif = crate::EnumBitfieldStruct<u8, Dnmif_SPEC>;
    impl Dnmif {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsifr3_SPEC;
impl crate::sealed::RegSpec for Dpsifr3_SPEC {
    type DataType = u8;
}

#[doc = "Deep Software Standby Interrupt Flag Register 3"]
pub type Dpsifr3 = crate::RegValueT<Dpsifr3_SPEC>;

impl Dpsifr3 {
    #[doc = "USBFS0 Suspend/Resume Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn dusbfs0if(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsifr3::Dusbfs0If,
        dpsifr3::Dusbfs0If,
        Dpsifr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsifr3::Dusbfs0If,
            dpsifr3::Dusbfs0If,
            Dpsifr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT1 Underflow Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn dagt1if(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsifr3::Dagt1If,
        dpsifr3::Dagt1If,
        Dpsifr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsifr3::Dagt1If,
            dpsifr3::Dagt1If,
            Dpsifr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT3 Underflow Deep Software Standby Cancel Flag"]
    #[inline(always)]
    pub fn dagt3if(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsifr3::Dagt3If,
        dpsifr3::Dagt3If,
        Dpsifr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsifr3::Dagt3If,
            dpsifr3::Dagt3If,
            Dpsifr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsifr3 {
    #[inline(always)]
    fn default() -> Dpsifr3 {
        <crate::RegValueT<Dpsifr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsifr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dusbfs0If_SPEC;
    pub type Dusbfs0If = crate::EnumBitfieldStruct<u8, Dusbfs0If_SPEC>;
    impl Dusbfs0If {
        #[doc = "The cancel request is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dagt1If_SPEC;
    pub type Dagt1If = crate::EnumBitfieldStruct<u8, Dagt1If_SPEC>;
    impl Dagt1If {
        #[doc = "The cancel request is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dagt3If_SPEC;
    pub type Dagt3If = crate::EnumBitfieldStruct<u8, Dagt3If_SPEC>;
    impl Dagt3If {
        #[doc = "The cancel request is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsiegr0_SPEC;
impl crate::sealed::RegSpec for Dpsiegr0_SPEC {
    type DataType = u8;
}

#[doc = "Deep Software Standby Interrupt Edge Register 0"]
pub type Dpsiegr0 = crate::RegValueT<Dpsiegr0_SPEC>;

impl Dpsiegr0 {
    #[doc = "IRQ0-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq0eg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsiegr0::Dirq0Eg,
        dpsiegr0::Dirq0Eg,
        Dpsiegr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsiegr0::Dirq0Eg,
            dpsiegr0::Dirq0Eg,
            Dpsiegr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ1-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq1eg(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsiegr0::Dirq1Eg,
        dpsiegr0::Dirq1Eg,
        Dpsiegr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsiegr0::Dirq1Eg,
            dpsiegr0::Dirq1Eg,
            Dpsiegr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ4-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq4eg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsiegr0::Dirq4Eg,
        dpsiegr0::Dirq4Eg,
        Dpsiegr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsiegr0::Dirq4Eg,
            dpsiegr0::Dirq4Eg,
            Dpsiegr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ5-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq5eg(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dpsiegr0::Dirq5Eg,
        dpsiegr0::Dirq5Eg,
        Dpsiegr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dpsiegr0::Dirq5Eg,
            dpsiegr0::Dirq5Eg,
            Dpsiegr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ6-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq6eg(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dpsiegr0::Dirq6Eg,
        dpsiegr0::Dirq6Eg,
        Dpsiegr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dpsiegr0::Dirq6Eg,
            dpsiegr0::Dirq6Eg,
            Dpsiegr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ7-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq7eg(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpsiegr0::Dirq7Eg,
        dpsiegr0::Dirq7Eg,
        Dpsiegr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpsiegr0::Dirq7Eg,
            dpsiegr0::Dirq7Eg,
            Dpsiegr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsiegr0 {
    #[inline(always)]
    fn default() -> Dpsiegr0 {
        <crate::RegValueT<Dpsiegr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsiegr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq0Eg_SPEC;
    pub type Dirq0Eg = crate::EnumBitfieldStruct<u8, Dirq0Eg_SPEC>;
    impl Dirq0Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq1Eg_SPEC;
    pub type Dirq1Eg = crate::EnumBitfieldStruct<u8, Dirq1Eg_SPEC>;
    impl Dirq1Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq4Eg_SPEC;
    pub type Dirq4Eg = crate::EnumBitfieldStruct<u8, Dirq4Eg_SPEC>;
    impl Dirq4Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq5Eg_SPEC;
    pub type Dirq5Eg = crate::EnumBitfieldStruct<u8, Dirq5Eg_SPEC>;
    impl Dirq5Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq6Eg_SPEC;
    pub type Dirq6Eg = crate::EnumBitfieldStruct<u8, Dirq6Eg_SPEC>;
    impl Dirq6Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq7Eg_SPEC;
    pub type Dirq7Eg = crate::EnumBitfieldStruct<u8, Dirq7Eg_SPEC>;
    impl Dirq7Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsiegr1_SPEC;
impl crate::sealed::RegSpec for Dpsiegr1_SPEC {
    type DataType = u8;
}

#[doc = "Deep Software Standby Interrupt Edge Register 1"]
pub type Dpsiegr1 = crate::RegValueT<Dpsiegr1_SPEC>;

impl Dpsiegr1 {
    #[doc = "IRQ8-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq8eg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsiegr1::Dirq8Eg,
        dpsiegr1::Dirq8Eg,
        Dpsiegr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsiegr1::Dirq8Eg,
            dpsiegr1::Dirq8Eg,
            Dpsiegr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ9-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq9eg(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsiegr1::Dirq9Eg,
        dpsiegr1::Dirq9Eg,
        Dpsiegr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsiegr1::Dirq9Eg,
            dpsiegr1::Dirq9Eg,
            Dpsiegr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsiegr1 {
    #[inline(always)]
    fn default() -> Dpsiegr1 {
        <crate::RegValueT<Dpsiegr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsiegr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq8Eg_SPEC;
    pub type Dirq8Eg = crate::EnumBitfieldStruct<u8, Dirq8Eg_SPEC>;
    impl Dirq8Eg {
        #[doc = "A cancel request is generated at a falling edge."]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq9Eg_SPEC;
    pub type Dirq9Eg = crate::EnumBitfieldStruct<u8, Dirq9Eg_SPEC>;
    impl Dirq9Eg {
        #[doc = "A cancel request is generated at a falling edge."]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsiegr2_SPEC;
impl crate::sealed::RegSpec for Dpsiegr2_SPEC {
    type DataType = u8;
}

#[doc = "Deep Software Standby Interrupt Edge Register 2"]
pub type Dpsiegr2 = crate::RegValueT<Dpsiegr2_SPEC>;

impl Dpsiegr2 {
    #[doc = "LVD1 Edge Select"]
    #[inline(always)]
    pub fn dlvd1eg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsiegr2::Dlvd1Eg,
        dpsiegr2::Dlvd1Eg,
        Dpsiegr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsiegr2::Dlvd1Eg,
            dpsiegr2::Dlvd1Eg,
            Dpsiegr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVD2 Edge Select"]
    #[inline(always)]
    pub fn dlvd2eg(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsiegr2::Dlvd2Eg,
        dpsiegr2::Dlvd2Eg,
        Dpsiegr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsiegr2::Dlvd2Eg,
            dpsiegr2::Dlvd2Eg,
            Dpsiegr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NMI Pin Edge Select"]
    #[inline(always)]
    pub fn dnmieg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsiegr2::Dnmieg,
        dpsiegr2::Dnmieg,
        Dpsiegr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsiegr2::Dnmieg,
            dpsiegr2::Dnmieg,
            Dpsiegr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsiegr2 {
    #[inline(always)]
    fn default() -> Dpsiegr2 {
        <crate::RegValueT<Dpsiegr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsiegr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlvd1Eg_SPEC;
    pub type Dlvd1Eg = crate::EnumBitfieldStruct<u8, Dlvd1Eg_SPEC>;
    impl Dlvd1Eg {
        #[doc = "A cancel request is generated when VCC < Vdet1 (fall) is detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated when VCC ≥ Vdet1 (rise) is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlvd2Eg_SPEC;
    pub type Dlvd2Eg = crate::EnumBitfieldStruct<u8, Dlvd2Eg_SPEC>;
    impl Dlvd2Eg {
        #[doc = "A cancel request is generated when VCC < Vdet2 (fall) is detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated when VCC ≥ Vdet2 (rise) is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dnmieg_SPEC;
    pub type Dnmieg = crate::EnumBitfieldStruct<u8, Dnmieg_SPEC>;
    impl Dnmieg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syocdcr_SPEC;
impl crate::sealed::RegSpec for Syocdcr_SPEC {
    type DataType = u8;
}

#[doc = "System Control OCD Control Register"]
pub type Syocdcr = crate::RegValueT<Syocdcr_SPEC>;

impl Syocdcr {
    #[doc = "Deep Software Standby OCD flag"]
    #[inline(always)]
    pub fn docdf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syocdcr::Docdf,
        syocdcr::Docdf,
        Syocdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syocdcr::Docdf,
            syocdcr::Docdf,
            Syocdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Debugger Enable bit"]
    #[inline(always)]
    pub fn dbgen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        syocdcr::Dbgen,
        syocdcr::Dbgen,
        Syocdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            syocdcr::Dbgen,
            syocdcr::Dbgen,
            Syocdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syocdcr {
    #[inline(always)]
    fn default() -> Syocdcr {
        <crate::RegValueT<Syocdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syocdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Docdf_SPEC;
    pub type Docdf = crate::EnumBitfieldStruct<u8, Docdf_SPEC>;
    impl Docdf {
        #[doc = "DBIRQ is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "DBIRQ is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbgen_SPEC;
    pub type Dbgen = crate::EnumBitfieldStruct<u8, Dbgen_SPEC>;
    impl Dbgen {
        #[doc = "On-chip debugger is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "On-chip debugger is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsr0_SPEC;
impl crate::sealed::RegSpec for Rstsr0_SPEC {
    type DataType = u8;
}

#[doc = "Reset Status Register 0"]
pub type Rstsr0 = crate::RegValueT<Rstsr0_SPEC>;

impl Rstsr0 {
    #[doc = "Power-On Reset Detect Flag"]
    #[inline(always)]
    pub fn porf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rstsr0::Porf,
        rstsr0::Porf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rstsr0::Porf,
            rstsr0::Porf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 0 Reset Detect Flag"]
    #[inline(always)]
    pub fn lvd0rf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rstsr0::Lvd0Rf,
        rstsr0::Lvd0Rf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rstsr0::Lvd0Rf,
            rstsr0::Lvd0Rf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Reset Detect Flag"]
    #[inline(always)]
    pub fn lvd1rf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rstsr0::Lvd1Rf,
        rstsr0::Lvd1Rf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rstsr0::Lvd1Rf,
            rstsr0::Lvd1Rf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Reset Detect Flag"]
    #[inline(always)]
    pub fn lvd2rf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rstsr0::Lvd2Rf,
        rstsr0::Lvd2Rf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rstsr0::Lvd2Rf,
            rstsr0::Lvd2Rf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Reset Detect Flag"]
    #[inline(always)]
    pub fn dpsrstf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rstsr0::Dpsrstf,
        rstsr0::Dpsrstf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rstsr0::Dpsrstf,
            rstsr0::Dpsrstf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rstsr0 {
    #[inline(always)]
    fn default() -> Rstsr0 {
        <crate::RegValueT<Rstsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porf_SPEC;
    pub type Porf = crate::EnumBitfieldStruct<u8, Porf_SPEC>;
    impl Porf {
        #[doc = "Power-on reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Power-on reset detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd0Rf_SPEC;
    pub type Lvd0Rf = crate::EnumBitfieldStruct<u8, Lvd0Rf_SPEC>;
    impl Lvd0Rf {
        #[doc = "Voltage monitor 0 reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage monitor 0 reset detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Rf_SPEC;
    pub type Lvd1Rf = crate::EnumBitfieldStruct<u8, Lvd1Rf_SPEC>;
    impl Lvd1Rf {
        #[doc = "Voltage monitor 1 reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage monitor 1 reset detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2Rf_SPEC;
    pub type Lvd2Rf = crate::EnumBitfieldStruct<u8, Lvd2Rf_SPEC>;
    impl Lvd2Rf {
        #[doc = "Voltage monitor 2 reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage monitor 2 reset detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpsrstf_SPEC;
    pub type Dpsrstf = crate::EnumBitfieldStruct<u8, Dpsrstf_SPEC>;
    impl Dpsrstf {
        #[doc = "Deep software standby mode cancellation not requested by an interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep software standby mode cancellation requested by an interrupt."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsr2_SPEC;
impl crate::sealed::RegSpec for Rstsr2_SPEC {
    type DataType = u8;
}

#[doc = "Reset Status Register 2"]
pub type Rstsr2 = crate::RegValueT<Rstsr2_SPEC>;

impl Rstsr2 {
    #[doc = "Cold/Warm Start Determination Flag"]
    #[inline(always)]
    pub fn cwsf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rstsr2::Cwsf,
        rstsr2::Cwsf,
        Rstsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rstsr2::Cwsf,
            rstsr2::Cwsf,
            Rstsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rstsr2 {
    #[inline(always)]
    fn default() -> Rstsr2 {
        <crate::RegValueT<Rstsr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstsr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cwsf_SPEC;
    pub type Cwsf = crate::EnumBitfieldStruct<u8, Cwsf_SPEC>;
    impl Cwsf {
        #[doc = "Cold start"]
        pub const _0: Self = Self::new(0);

        #[doc = "Warm start"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Momcr_SPEC;
impl crate::sealed::RegSpec for Momcr_SPEC {
    type DataType = u8;
}

#[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
pub type Momcr = crate::RegValueT<Momcr_SPEC>;

impl Momcr {
    #[doc = "Main Clock Oscillator Drive Capability 0 Switching"]
    #[inline(always)]
    pub fn modrv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        momcr::Modrv,
        momcr::Modrv,
        Momcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            momcr::Modrv,
            momcr::Modrv,
            Momcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Main Clock Oscillator Switching"]
    #[inline(always)]
    pub fn mosel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        momcr::Mosel,
        momcr::Mosel,
        Momcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            momcr::Mosel,
            momcr::Mosel,
            Momcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Momcr {
    #[inline(always)]
    fn default() -> Momcr {
        <crate::RegValueT<Momcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod momcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Modrv_SPEC;
    pub type Modrv = crate::EnumBitfieldStruct<u8, Modrv_SPEC>;
    impl Modrv {
        #[doc = "20 MHz to 24 MHz"]
        pub const _00: Self = Self::new(0);

        #[doc = "16 MHz to 20 MHz"]
        pub const _01: Self = Self::new(1);

        #[doc = "8 MHz to 16 MHz"]
        pub const _10: Self = Self::new(2);

        #[doc = "8 MHz"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mosel_SPEC;
    pub type Mosel = crate::EnumBitfieldStruct<u8, Mosel_SPEC>;
    impl Mosel {
        #[doc = "Resonator"]
        pub const _0: Self = Self::new(0);

        #[doc = "External clock input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwepror_SPEC;
impl crate::sealed::RegSpec for Fwepror_SPEC {
    type DataType = u8;
}

#[doc = "Flash P/E Protect Register"]
pub type Fwepror = crate::RegValueT<Fwepror_SPEC>;

impl Fwepror {
    #[doc = "Flash Programming and Erasure"]
    #[inline(always)]
    pub fn flwe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        fwepror::Flwe,
        fwepror::Flwe,
        Fwepror_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            fwepror::Flwe,
            fwepror::Flwe,
            Fwepror_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwepror {
    #[inline(always)]
    fn default() -> Fwepror {
        <crate::RegValueT<Fwepror_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod fwepror {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flwe_SPEC;
    pub type Flwe = crate::EnumBitfieldStruct<u8, Flwe_SPEC>;
    impl Flwe {
        #[doc = "Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing."]
        pub const _00: Self = Self::new(0);

        #[doc = "Permits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing."]
        pub const _01: Self = Self::new(1);

        #[doc = "Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing."]
        pub const _10: Self = Self::new(2);

        #[doc = "Prohibits Program, Block Erase, Multi Block Erase, Blank Check, and Configuration set command processing."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd1Cmpcr_SPEC;
impl crate::sealed::RegSpec for Lvd1Cmpcr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitoring 1 Comparator Control Register"]
pub type Lvd1Cmpcr = crate::RegValueT<Lvd1Cmpcr_SPEC>;

impl Lvd1Cmpcr {
    #[doc = "Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub fn lvd1lvl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        lvd1cmpcr::Lvd1Lvl,
        lvd1cmpcr::Lvd1Lvl,
        Lvd1Cmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            lvd1cmpcr::Lvd1Lvl,
            lvd1cmpcr::Lvd1Lvl,
            Lvd1Cmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Detection 1 Enable"]
    #[inline(always)]
    pub fn lvd1e(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        lvd1cmpcr::Lvd1E,
        lvd1cmpcr::Lvd1E,
        Lvd1Cmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            lvd1cmpcr::Lvd1E,
            lvd1cmpcr::Lvd1E,
            Lvd1Cmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd1Cmpcr {
    #[inline(always)]
    fn default() -> Lvd1Cmpcr {
        <crate::RegValueT<Lvd1Cmpcr_SPEC> as RegisterValue<_>>::new(19)
    }
}
pub mod lvd1cmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Lvl_SPEC;
    pub type Lvd1Lvl = crate::EnumBitfieldStruct<u8, Lvd1Lvl_SPEC>;
    impl Lvd1Lvl {
        #[doc = "2.99 V (Vdet1_1)"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "2.92 V (Vdet1_2)"]
        pub const _0_X_12: Self = Self::new(18);

        #[doc = "2.85 V (Vdet1_3)"]
        pub const _0_X_13: Self = Self::new(19);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1E_SPEC;
    pub type Lvd1E = crate::EnumBitfieldStruct<u8, Lvd1E_SPEC>;
    impl Lvd1E {
        #[doc = "Voltage detection 1 circuit disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage detection 1 circuit enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd2Cmpcr_SPEC;
impl crate::sealed::RegSpec for Lvd2Cmpcr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitoring 2 Comparator Control Register"]
pub type Lvd2Cmpcr = crate::RegValueT<Lvd2Cmpcr_SPEC>;

impl Lvd2Cmpcr {
    #[doc = "Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub fn lvd2lvl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        lvd2cmpcr::Lvd2Lvl,
        lvd2cmpcr::Lvd2Lvl,
        Lvd2Cmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            lvd2cmpcr::Lvd2Lvl,
            lvd2cmpcr::Lvd2Lvl,
            Lvd2Cmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Detection 2 Enable"]
    #[inline(always)]
    pub fn lvd2e(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        lvd2cmpcr::Lvd2E,
        lvd2cmpcr::Lvd2E,
        Lvd2Cmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            lvd2cmpcr::Lvd2E,
            lvd2cmpcr::Lvd2E,
            Lvd2Cmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd2Cmpcr {
    #[inline(always)]
    fn default() -> Lvd2Cmpcr {
        <crate::RegValueT<Lvd2Cmpcr_SPEC> as RegisterValue<_>>::new(7)
    }
}
pub mod lvd2cmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2Lvl_SPEC;
    pub type Lvd2Lvl = crate::EnumBitfieldStruct<u8, Lvd2Lvl_SPEC>;
    impl Lvd2Lvl {
        #[doc = "2.99 V (Vdet2_1)"]
        pub const _101: Self = Self::new(5);

        #[doc = "2.92 V (Vdet2_2)"]
        pub const _110: Self = Self::new(6);

        #[doc = "2.85 V (Vdet2_3)"]
        pub const _111: Self = Self::new(7);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2E_SPEC;
    pub type Lvd2E = crate::EnumBitfieldStruct<u8, Lvd2E_SPEC>;
    impl Lvd2E {
        #[doc = "Voltage detection 2 circuit disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage detection 2 circuit enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd1Cr0_SPEC;
impl crate::sealed::RegSpec for Lvd1Cr0_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 1 Circuit Control Register 0"]
pub type Lvd1Cr0 = crate::RegValueT<Lvd1Cr0_SPEC>;

impl Lvd1Cr0 {
    #[doc = "Voltage Monitor 1 Interrupt/Reset Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lvd1cr0::Rie,
        lvd1cr0::Rie,
        Lvd1Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lvd1cr0::Rie,
            lvd1cr0::Rie,
            Lvd1Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage monitor 1 Digital Filter Disabled Mode Select"]
    #[inline(always)]
    pub fn dfdis(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        lvd1cr0::Dfdis,
        lvd1cr0::Dfdis,
        Lvd1Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            lvd1cr0::Dfdis,
            lvd1cr0::Dfdis,
            Lvd1Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub fn cmpe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        lvd1cr0::Cmpe,
        lvd1cr0::Cmpe,
        Lvd1Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            lvd1cr0::Cmpe,
            lvd1cr0::Cmpe,
            Lvd1Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sampling Clock Select"]
    #[inline(always)]
    pub fn fsamp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        lvd1cr0::Fsamp,
        lvd1cr0::Fsamp,
        Lvd1Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            lvd1cr0::Fsamp,
            lvd1cr0::Fsamp,
            Lvd1Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Circuit Mode Select"]
    #[inline(always)]
    pub fn ri(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        lvd1cr0::Ri,
        lvd1cr0::Ri,
        Lvd1Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            lvd1cr0::Ri,
            lvd1cr0::Ri,
            Lvd1Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Reset Negate Select"]
    #[inline(always)]
    pub fn rn(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        lvd1cr0::Rn,
        lvd1cr0::Rn,
        Lvd1Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            lvd1cr0::Rn,
            lvd1cr0::Rn,
            Lvd1Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd1Cr0 {
    #[inline(always)]
    fn default() -> Lvd1Cr0 {
        <crate::RegValueT<Lvd1Cr0_SPEC> as RegisterValue<_>>::new(130)
    }
}
pub mod lvd1cr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfdis_SPEC;
    pub type Dfdis = crate::EnumBitfieldStruct<u8, Dfdis_SPEC>;
    impl Dfdis {
        #[doc = "Enable the digital filter"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable the digital filter"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpe_SPEC;
    pub type Cmpe = crate::EnumBitfieldStruct<u8, Cmpe_SPEC>;
    impl Cmpe {
        #[doc = "Disable voltage monitor 1 circuit comparison result output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable voltage monitor 1 circuit comparison result output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsamp_SPEC;
    pub type Fsamp = crate::EnumBitfieldStruct<u8, Fsamp_SPEC>;
    impl Fsamp {
        #[doc = "1/2 LOCO frequency"]
        pub const _00: Self = Self::new(0);

        #[doc = "1/4 LOCO frequency"]
        pub const _01: Self = Self::new(1);

        #[doc = "1/8 LOCO frequency"]
        pub const _10: Self = Self::new(2);

        #[doc = "1/16 LOCO frequency"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ri_SPEC;
    pub type Ri = crate::EnumBitfieldStruct<u8, Ri_SPEC>;
    impl Ri {
        #[doc = "Generate voltage monitor 1 interrupt on Vdet1 crossing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable voltage monitor 1 reset when the voltage falls to and below Vdet1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rn_SPEC;
    pub type Rn = crate::EnumBitfieldStruct<u8, Rn_SPEC>;
    impl Rn {
        #[doc = "Negate after a stabilization time (tLVD1) when VCC > Vdet1 is detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Negate after a stabilization time (tLVD1) on assertion of the LVD1 reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd2Cr0_SPEC;
impl crate::sealed::RegSpec for Lvd2Cr0_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 2 Circuit Control Register 0"]
pub type Lvd2Cr0 = crate::RegValueT<Lvd2Cr0_SPEC>;

impl Lvd2Cr0 {
    #[doc = "Voltage Monitor 2 Interrupt/Reset Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lvd2cr0::Rie,
        lvd2cr0::Rie,
        Lvd2Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lvd2cr0::Rie,
            lvd2cr0::Rie,
            Lvd2Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage monitor 2 Digital Filter Disabled Mode Select"]
    #[inline(always)]
    pub fn dfdis(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        lvd2cr0::Dfdis,
        lvd2cr0::Dfdis,
        Lvd2Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            lvd2cr0::Dfdis,
            lvd2cr0::Dfdis,
            Lvd2Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub fn cmpe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        lvd2cr0::Cmpe,
        lvd2cr0::Cmpe,
        Lvd2Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            lvd2cr0::Cmpe,
            lvd2cr0::Cmpe,
            Lvd2Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sampling Clock Select"]
    #[inline(always)]
    pub fn fsamp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        lvd2cr0::Fsamp,
        lvd2cr0::Fsamp,
        Lvd2Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            lvd2cr0::Fsamp,
            lvd2cr0::Fsamp,
            Lvd2Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Circuit Mode Select"]
    #[inline(always)]
    pub fn ri(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        lvd2cr0::Ri,
        lvd2cr0::Ri,
        Lvd2Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            lvd2cr0::Ri,
            lvd2cr0::Ri,
            Lvd2Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Reset Negate Select"]
    #[inline(always)]
    pub fn rn(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        lvd2cr0::Rn,
        lvd2cr0::Rn,
        Lvd2Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            lvd2cr0::Rn,
            lvd2cr0::Rn,
            Lvd2Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd2Cr0 {
    #[inline(always)]
    fn default() -> Lvd2Cr0 {
        <crate::RegValueT<Lvd2Cr0_SPEC> as RegisterValue<_>>::new(130)
    }
}
pub mod lvd2cr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfdis_SPEC;
    pub type Dfdis = crate::EnumBitfieldStruct<u8, Dfdis_SPEC>;
    impl Dfdis {
        #[doc = "Enable the digital filter"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable the digital filter"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpe_SPEC;
    pub type Cmpe = crate::EnumBitfieldStruct<u8, Cmpe_SPEC>;
    impl Cmpe {
        #[doc = "Disable voltage monitor 2 circuit comparison result output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable voltage monitor 2 circuit comparison result output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsamp_SPEC;
    pub type Fsamp = crate::EnumBitfieldStruct<u8, Fsamp_SPEC>;
    impl Fsamp {
        #[doc = "1/2 LOCO frequency"]
        pub const _00: Self = Self::new(0);

        #[doc = "1/4 LOCO frequency"]
        pub const _01: Self = Self::new(1);

        #[doc = "1/8 LOCO frequency"]
        pub const _10: Self = Self::new(2);

        #[doc = "1/16 LOCO frequency"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ri_SPEC;
    pub type Ri = crate::EnumBitfieldStruct<u8, Ri_SPEC>;
    impl Ri {
        #[doc = "Generate voltage monitor 2 interrupt on Vdet2 crossing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable voltage monitor 2 reset when the voltage falls to and below Vdet2"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rn_SPEC;
    pub type Rn = crate::EnumBitfieldStruct<u8, Rn_SPEC>;
    impl Rn {
        #[doc = "Negate after a stabilization time (tLVD2) when VCC > Vdet2 is detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Negate after a stabilization time (tLVD2) on assertion of the LVD2 reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbattmnselr_SPEC;
impl crate::sealed::RegSpec for Vbattmnselr_SPEC {
    type DataType = u8;
}

#[doc = "Battery Backup Voltage Monitor Function Select Register"]
pub type Vbattmnselr = crate::RegValueT<Vbattmnselr_SPEC>;

impl Vbattmnselr {
    #[doc = "VBATT Low Voltage Detect Function Select Bit"]
    #[inline(always)]
    pub fn vbattmnsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbattmnselr::Vbattmnsel,
        vbattmnselr::Vbattmnsel,
        Vbattmnselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbattmnselr::Vbattmnsel,
            vbattmnselr::Vbattmnsel,
            Vbattmnselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbattmnselr {
    #[inline(always)]
    fn default() -> Vbattmnselr {
        <crate::RegValueT<Vbattmnselr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbattmnselr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbattmnsel_SPEC;
    pub type Vbattmnsel = crate::EnumBitfieldStruct<u8, Vbattmnsel_SPEC>;
    impl Vbattmnsel {
        #[doc = "Disables VBATT low voltage detect function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables VBATT low voltage detect function"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbattmonr_SPEC;
impl crate::sealed::RegSpec for Vbattmonr_SPEC {
    type DataType = u8;
}

#[doc = "Battery Backup Voltage Monitor Register"]
pub type Vbattmonr = crate::RegValueT<Vbattmonr_SPEC>;

impl Vbattmonr {
    #[doc = "VBATT Voltage Monitor Bit"]
    #[inline(always)]
    pub fn vbattmon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbattmonr::Vbattmon,
        vbattmonr::Vbattmon,
        Vbattmonr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbattmonr::Vbattmon,
            vbattmonr::Vbattmon,
            Vbattmonr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbattmonr {
    #[inline(always)]
    fn default() -> Vbattmonr {
        <crate::RegValueT<Vbattmonr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbattmonr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbattmon_SPEC;
    pub type Vbattmon = crate::EnumBitfieldStruct<u8, Vbattmon_SPEC>;
    impl Vbattmon {
        #[doc = "VBATT ≥ Vbattldet"]
        pub const _0: Self = Self::new(0);

        #[doc = "VBATT < Vbattldet"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldoscr_SPEC;
impl crate::sealed::RegSpec for Ldoscr_SPEC {
    type DataType = u8;
}

#[doc = "LDO Stop Control Register"]
pub type Ldoscr = crate::RegValueT<Ldoscr_SPEC>;

impl Ldoscr {
    #[doc = "LDO0 Stop"]
    #[inline(always)]
    pub fn ldostp0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ldoscr::Ldostp0,
        ldoscr::Ldostp0,
        Ldoscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ldoscr::Ldostp0,
            ldoscr::Ldostp0,
            Ldoscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LDO1 Stop"]
    #[inline(always)]
    pub fn ldostp1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ldoscr::Ldostp1,
        ldoscr::Ldostp1,
        Ldoscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ldoscr::Ldostp1,
            ldoscr::Ldostp1,
            Ldoscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ldoscr {
    #[inline(always)]
    fn default() -> Ldoscr {
        <crate::RegValueT<Ldoscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ldoscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ldostp0_SPEC;
    pub type Ldostp0 = crate::EnumBitfieldStruct<u8, Ldostp0_SPEC>;
    impl Ldostp0 {
        #[doc = "LDO0 is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "LDO0 is stopped"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ldostp1_SPEC;
    pub type Ldostp1 = crate::EnumBitfieldStruct<u8, Ldostp1_SPEC>;
    impl Ldostp1 {
        #[doc = "LDO1 is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "LDO1 is stopped"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pl2Ldoscr_SPEC;
impl crate::sealed::RegSpec for Pl2Ldoscr_SPEC {
    type DataType = u8;
}

#[doc = "PLL2-LDO Stop Control Register"]
pub type Pl2Ldoscr = crate::RegValueT<Pl2Ldoscr_SPEC>;

impl Pl2Ldoscr {
    #[doc = "PLL2-LDO Stop"]
    #[inline(always)]
    pub fn pl2ldostp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pl2ldoscr::Pl2Ldostp,
        pl2ldoscr::Pl2Ldostp,
        Pl2Ldoscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pl2ldoscr::Pl2Ldostp,
            pl2ldoscr::Pl2Ldostp,
            Pl2Ldoscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pl2Ldoscr {
    #[inline(always)]
    fn default() -> Pl2Ldoscr {
        <crate::RegValueT<Pl2Ldoscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pl2ldoscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pl2Ldostp_SPEC;
    pub type Pl2Ldostp = crate::EnumBitfieldStruct<u8, Pl2Ldostp_SPEC>;
    impl Pl2Ldostp {
        #[doc = "PLL2-LDO is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "PLL2-LDO is stopped"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccr_SPEC;
impl crate::sealed::RegSpec for Sosccr_SPEC {
    type DataType = u8;
}

#[doc = "Sub-Clock Oscillator Control Register"]
pub type Sosccr = crate::RegValueT<Sosccr_SPEC>;

impl Sosccr {
    #[doc = "Sub Clock Oscillator Stop"]
    #[inline(always)]
    pub fn sostp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sosccr::Sostp,
        sosccr::Sostp,
        Sosccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sosccr::Sostp,
            sosccr::Sostp,
            Sosccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sosccr {
    #[inline(always)]
    fn default() -> Sosccr {
        <crate::RegValueT<Sosccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sosccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sostp_SPEC;
    pub type Sostp = crate::EnumBitfieldStruct<u8, Sostp_SPEC>;
    impl Sostp {
        #[doc = "Operate the sub-clock oscillator"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the sub-clock oscillator"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Somcr_SPEC;
impl crate::sealed::RegSpec for Somcr_SPEC {
    type DataType = u8;
}

#[doc = "Sub-Clock Oscillator Mode Control Register"]
pub type Somcr = crate::RegValueT<Somcr_SPEC>;

impl Somcr {
    #[doc = "Sub-Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    pub fn sodrv(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        somcr::Sodrv,
        somcr::Sodrv,
        Somcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            somcr::Sodrv,
            somcr::Sodrv,
            Somcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Somcr {
    #[inline(always)]
    fn default() -> Somcr {
        <crate::RegValueT<Somcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod somcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sodrv_SPEC;
    pub type Sodrv = crate::EnumBitfieldStruct<u8, Sodrv_SPEC>;
    impl Sodrv {
        #[doc = "Standard"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lococr_SPEC;
impl crate::sealed::RegSpec for Lococr_SPEC {
    type DataType = u8;
}

#[doc = "Low-Speed On-Chip Oscillator Control Register"]
pub type Lococr = crate::RegValueT<Lococr_SPEC>;

impl Lococr {
    #[doc = "LOCO Stop"]
    #[inline(always)]
    pub fn lcstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lococr::Lcstp,
        lococr::Lcstp,
        Lococr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lococr::Lcstp,
            lococr::Lcstp,
            Lococr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lococr {
    #[inline(always)]
    fn default() -> Lococr {
        <crate::RegValueT<Lococr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lococr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcstp_SPEC;
    pub type Lcstp = crate::EnumBitfieldStruct<u8, Lcstp_SPEC>;
    impl Lcstp {
        #[doc = "Operate the LOCO clock"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the LOCO clock"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Locoutcr_SPEC;
impl crate::sealed::RegSpec for Locoutcr_SPEC {
    type DataType = u8;
}

#[doc = "LOCO User Trimming Control Register"]
pub type Locoutcr = crate::RegValueT<Locoutcr_SPEC>;

impl Locoutcr {
    #[doc = "LOCO User Trimming"]
    #[inline(always)]
    pub fn locoutrm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Locoutcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Locoutcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Locoutcr {
    #[inline(always)]
    fn default() -> Locoutcr {
        <crate::RegValueT<Locoutcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtictlr_SPEC;
impl crate::sealed::RegSpec for Vbtictlr_SPEC {
    type DataType = u8;
}

#[doc = "VBATT Input Control Register"]
pub type Vbtictlr = crate::RegValueT<Vbtictlr_SPEC>;

impl Vbtictlr {
    #[doc = "VBATT CH0 Input Enable"]
    #[inline(always)]
    pub fn vch0inen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtictlr::Vch0Inen,
        vbtictlr::Vch0Inen,
        Vbtictlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtictlr::Vch0Inen,
            vbtictlr::Vch0Inen,
            Vbtictlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbtictlr {
    #[inline(always)]
    fn default() -> Vbtictlr {
        <crate::RegValueT<Vbtictlr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtictlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch0Inen_SPEC;
    pub type Vch0Inen = crate::EnumBitfieldStruct<u8, Vch0Inen_SPEC>;
    impl Vch0Inen {
        #[doc = "RTCIC0 input disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCIC0 input enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtber_SPEC;
impl crate::sealed::RegSpec for Vbtber_SPEC {
    type DataType = u8;
}

#[doc = "VBATT Backup Enable Register"]
pub type Vbtber = crate::RegValueT<Vbtber_SPEC>;

impl Vbtber {
    #[doc = "VBATT backup register access enable bit"]
    #[inline(always)]
    pub fn vbae(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        vbtber::Vbae,
        vbtber::Vbae,
        Vbtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            vbtber::Vbae,
            vbtber::Vbae,
            Vbtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbtber {
    #[inline(always)]
    fn default() -> Vbtber {
        <crate::RegValueT<Vbtber_SPEC> as RegisterValue<_>>::new(8)
    }
}
pub mod vbtber {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbae_SPEC;
    pub type Vbae = crate::EnumBitfieldStruct<u8, Vbae_SPEC>;
    impl Vbae {
        #[doc = "Disable to access VBTBKR"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable to access VBTBKR"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtbkr_SPEC;
impl crate::sealed::RegSpec for Vbtbkr_SPEC {
    type DataType = u8;
}

#[doc = "VBATT Backup Register"]
pub type Vbtbkr = crate::RegValueT<Vbtbkr_SPEC>;

impl Vbtbkr {
    #[doc = "VBATT Backup Register"]
    #[inline(always)]
    pub fn vbtbkr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Vbtbkr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Vbtbkr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtbkr {
    #[inline(always)]
    fn default() -> Vbtbkr {
        <crate::RegValueT<Vbtbkr_SPEC> as RegisterValue<_>>::new(0)
    }
}
