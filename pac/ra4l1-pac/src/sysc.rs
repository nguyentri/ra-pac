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
// Generated from SVD 0.90.02, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:05:50 +0000

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

    #[doc = "Memory Wait Cycle Control Register for Code Flash"]
    #[inline(always)]
    pub const fn memwait(
        &self,
    ) -> &'static crate::common::Reg<self::Memwait_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Memwait_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(49usize),
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

    #[doc = "Sub-Clock Oscillation Stop Detection Control Register"]
    #[inline(always)]
    pub const fn sostdcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sostdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sostdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[doc = "Segment LCD Source Clock Control Register"]
    #[inline(always)]
    pub const fn slcdsckcr(
        &self,
    ) -> &'static crate::common::Reg<self::Slcdsckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Slcdsckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
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

    #[doc = "CANFD Clock Division Control Register"]
    #[inline(always)]
    pub const fn canfdckdivcr(
        &self,
    ) -> &'static crate::common::Reg<self::Canfdckdivcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Canfdckdivcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(110usize),
            )
        }
    }

    #[doc = "I3C Clock Division Control Register"]
    #[inline(always)]
    pub const fn i3cckdivcr(
        &self,
    ) -> &'static crate::common::Reg<self::I3Cckdivcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3Cckdivcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(113usize),
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

    #[doc = "CANFD Clock Control Register"]
    #[inline(always)]
    pub const fn canfdckcr(
        &self,
    ) -> &'static crate::common::Reg<self::Canfdckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Canfdckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(118usize),
            )
        }
    }

    #[doc = "I3C Clock Control Register"]
    #[inline(always)]
    pub const fn i3cckcr(
        &self,
    ) -> &'static crate::common::Reg<self::I3Cckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3Cckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(121usize),
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

    #[doc = "Power Save Memory Control Register"]
    #[inline(always)]
    pub const fn psmcr(&self) -> &'static crate::common::Reg<self::Psmcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psmcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(159usize),
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

    #[doc = "LDO Control Register"]
    #[inline(always)]
    pub const fn ldocr(&self) -> &'static crate::common::Reg<self::Ldocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ldocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
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

    #[doc = "Voltage Monitor 1 Circuit Control Register"]
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

    #[doc = "Protect Register"]
    #[inline(always)]
    pub const fn prcr(&self) -> &'static crate::common::Reg<self::Prcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Prcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1022usize),
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

    #[doc = "Sub-Clock Oscillator Margin Check Register"]
    #[inline(always)]
    pub const fn somrg(&self) -> &'static crate::common::Reg<self::Somrg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Somrg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1154usize),
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
        <crate::RegValueT<Sbycr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sbycr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssby_SPEC;
    pub type Ssby = crate::EnumBitfieldStruct<u8, Ssby_SPEC>;
    impl Ssby {
        #[doc = "Sleep mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby mode"]
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
        <crate::RegValueT<Sckdivcr_SPEC> as RegisterValue<_>>::new(1140868164)
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
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Pllccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Pllccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pllccr {
    #[inline(always)]
    fn default() -> Pllccr {
        <crate::RegValueT<Pllccr_SPEC> as RegisterValue<_>>::new(1792)
    }
}
pub mod pllccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plidiv_SPEC;
    pub type Plidiv = crate::EnumBitfieldStruct<u8, Plidiv_SPEC>;
    impl Plidiv {
        #[doc = "/1"]
        pub const _00: Self = Self::new(0);

        #[doc = "/4"]
        pub const _01: Self = Self::new(1);

        #[doc = "/6"]
        pub const _10: Self = Self::new(2);
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

        #[doc = "PLL is stopped"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memwait_SPEC;
impl crate::sealed::RegSpec for Memwait_SPEC {
    type DataType = u8;
}

#[doc = "Memory Wait Cycle Control Register for Code Flash"]
pub type Memwait = crate::RegValueT<Memwait_SPEC>;

impl Memwait {
    #[doc = "Memory Wait Cycle Select for Code Flash"]
    #[inline(always)]
    pub fn memwait(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        memwait::Memwait,
        memwait::Memwait,
        Memwait_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            memwait::Memwait,
            memwait::Memwait,
            Memwait_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Memwait {
    #[inline(always)]
    fn default() -> Memwait {
        <crate::RegValueT<Memwait_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod memwait {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Memwait_SPEC;
    pub type Memwait = crate::EnumBitfieldStruct<u8, Memwait_SPEC>;
    impl Memwait {
        #[doc = "No wait"]
        pub const _00: Self = Self::new(0);

        #[doc = "One wait"]
        pub const _01: Self = Self::new(1);

        #[doc = "Two waits"]
        pub const _10: Self = Self::new(2);
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
pub struct Sostdcr_SPEC;
impl crate::sealed::RegSpec for Sostdcr_SPEC {
    type DataType = u16;
}

#[doc = "Sub-Clock Oscillation Stop Detection Control Register"]
pub type Sostdcr = crate::RegValueT<Sostdcr_SPEC>;

impl Sostdcr {
    #[doc = "Sub-Clock Oscillation Stop Detection Time"]
    #[inline(always)]
    pub fn sosdccmp(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Sostdcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Sostdcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sub-Clock Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub fn sostdie(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        sostdcr::Sostdie,
        sostdcr::Sostdie,
        Sostdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            sostdcr::Sostdie,
            sostdcr::Sostdie,
            Sostdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sub-Clock Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub fn sostdf(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        sostdcr::Sostdf,
        sostdcr::Sostdf,
        Sostdcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            sostdcr::Sostdf,
            sostdcr::Sostdf,
            Sostdcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Status of Sub-Clock Oscillation Stop Detector Operation"]
    #[inline(always)]
    pub fn sosdcf(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        sostdcr::Sosdcf,
        sostdcr::Sosdcf,
        Sostdcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            sostdcr::Sosdcf,
            sostdcr::Sosdcf,
            Sostdcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Control of Sub-Clock Oscillation Stop Detector Operation"]
    #[inline(always)]
    pub fn sosdce(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        sostdcr::Sosdce,
        sostdcr::Sosdce,
        Sostdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            sostdcr::Sosdce,
            sostdcr::Sosdce,
            Sostdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sostdcr {
    #[inline(always)]
    fn default() -> Sostdcr {
        <crate::RegValueT<Sostdcr_SPEC> as RegisterValue<_>>::new(4095)
    }
}
pub mod sostdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sostdie_SPEC;
    pub type Sostdie = crate::EnumBitfieldStruct<u8, Sostdie_SPEC>;
    impl Sostdie {
        #[doc = "Disable sub-clock oscillation stop detection interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable sub-clock oscillation stop detection interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sostdf_SPEC;
    pub type Sostdf = crate::EnumBitfieldStruct<u8, Sostdf_SPEC>;
    impl Sostdf {
        #[doc = "Sub-clock oscillation stop not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sub-clock oscillation stop detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sosdcf_SPEC;
    pub type Sosdcf = crate::EnumBitfieldStruct<u8, Sosdcf_SPEC>;
    impl Sosdcf {
        #[doc = "Stop operation of the sub-clock oscillation stop detector"]
        pub const _0: Self = Self::new(0);

        #[doc = "Run operation of the sub-clock oscillation stop detector"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sosdce_SPEC;
    pub type Sosdce = crate::EnumBitfieldStruct<u8, Sosdce_SPEC>;
    impl Sosdce {
        #[doc = "Stop operation of the sub-clock oscillation stop detector"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start operation of the sub-clock oscillation stop detector"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slcdsckcr_SPEC;
impl crate::sealed::RegSpec for Slcdsckcr_SPEC {
    type DataType = u8;
}

#[doc = "Segment LCD Source Clock Control Register"]
pub type Slcdsckcr = crate::RegValueT<Slcdsckcr_SPEC>;

impl Slcdsckcr {
    #[doc = "Segment LCD Source Clock (LCDSRCCLK) Select"]
    #[inline(always)]
    pub fn lcdscksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        slcdsckcr::Lcdscksel,
        slcdsckcr::Lcdscksel,
        Slcdsckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            slcdsckcr::Lcdscksel,
            slcdsckcr::Lcdscksel,
            Slcdsckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Segment LCD Source Clock (LCDSRCCLK) Out Enable"]
    #[inline(always)]
    pub fn lcdscken(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        slcdsckcr::Lcdscken,
        slcdsckcr::Lcdscken,
        Slcdsckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            slcdsckcr::Lcdscken,
            slcdsckcr::Lcdscken,
            Slcdsckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Slcdsckcr {
    #[inline(always)]
    fn default() -> Slcdsckcr {
        <crate::RegValueT<Slcdsckcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod slcdsckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdscksel_SPEC;
    pub type Lcdscksel = crate::EnumBitfieldStruct<u8, Lcdscksel_SPEC>;
    impl Lcdscksel {
        #[doc = "LOCO"]
        pub const _000: Self = Self::new(0);

        #[doc = "SOSC"]
        pub const _001: Self = Self::new(1);

        #[doc = "MOSC"]
        pub const _010: Self = Self::new(2);

        #[doc = "HOCO"]
        pub const _100: Self = Self::new(4);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdscken_SPEC;
    pub type Lcdscken = crate::EnumBitfieldStruct<u8, Lcdscken_SPEC>;
    impl Lcdscken {
        #[doc = "Segment LCD source clock out is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Segment LCD source clock out is enabled"]
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
pub struct Canfdckdivcr_SPEC;
impl crate::sealed::RegSpec for Canfdckdivcr_SPEC {
    type DataType = u8;
}

#[doc = "CANFD Clock Division Control Register"]
pub type Canfdckdivcr = crate::RegValueT<Canfdckdivcr_SPEC>;

impl Canfdckdivcr {
    #[doc = "CANFD clock (CANFDCLK) Division Select"]
    #[inline(always)]
    pub fn canfdckdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        canfdckdivcr::Canfdckdiv,
        canfdckdivcr::Canfdckdiv,
        Canfdckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            canfdckdivcr::Canfdckdiv,
            canfdckdivcr::Canfdckdiv,
            Canfdckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Canfdckdivcr {
    #[inline(always)]
    fn default() -> Canfdckdivcr {
        <crate::RegValueT<Canfdckdivcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod canfdckdivcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Canfdckdiv_SPEC;
    pub type Canfdckdiv = crate::EnumBitfieldStruct<u8, Canfdckdiv_SPEC>;
    impl Canfdckdiv {
        #[doc = "/1 (value after reset)"]
        pub const _000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "/6"]
        pub const _011: Self = Self::new(3);

        #[doc = "/8"]
        pub const _100: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3Cckdivcr_SPEC;
impl crate::sealed::RegSpec for I3Cckdivcr_SPEC {
    type DataType = u8;
}

#[doc = "I3C Clock Division Control Register"]
pub type I3Cckdivcr = crate::RegValueT<I3Cckdivcr_SPEC>;

impl I3Cckdivcr {
    #[doc = "I3C Clock (I3CCLK) Division Select"]
    #[inline(always)]
    pub fn i3cckdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        i3cckdivcr::I3Cckdiv,
        i3cckdivcr::I3Cckdiv,
        I3Cckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            i3cckdivcr::I3Cckdiv,
            i3cckdivcr::I3Cckdiv,
            I3Cckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3Cckdivcr {
    #[inline(always)]
    fn default() -> I3Cckdivcr {
        <crate::RegValueT<I3Cckdivcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod i3cckdivcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct I3Cckdiv_SPEC;
    pub type I3Cckdiv = crate::EnumBitfieldStruct<u8, I3Cckdiv_SPEC>;
    impl I3Cckdiv {
        #[doc = "/1 (value after reset)"]
        pub const _000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "/6"]
        pub const _011: Self = Self::new(3);

        #[doc = "/8"]
        pub const _100: Self = Self::new(4);

        #[doc = "/3"]
        pub const _101: Self = Self::new(5);

        #[doc = "/5"]
        pub const _110: Self = Self::new(6);
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
pub struct Canfdckcr_SPEC;
impl crate::sealed::RegSpec for Canfdckcr_SPEC {
    type DataType = u8;
}

#[doc = "CANFD Clock Control Register"]
pub type Canfdckcr = crate::RegValueT<Canfdckcr_SPEC>;

impl Canfdckcr {
    #[doc = "CANFD clock (CANFDCLK) Source Select"]
    #[inline(always)]
    pub fn canfdcksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        canfdckcr::Canfdcksel,
        canfdckcr::Canfdcksel,
        Canfdckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            canfdckcr::Canfdcksel,
            canfdckcr::Canfdcksel,
            Canfdckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CANFD clock (CANFDCLK) Switching Request"]
    #[inline(always)]
    pub fn canfdcksreq(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        canfdckcr::Canfdcksreq,
        canfdckcr::Canfdcksreq,
        Canfdckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            canfdckcr::Canfdcksreq,
            canfdckcr::Canfdcksreq,
            Canfdckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CANFD clock (CANFDCLK) Switching Ready State Flag"]
    #[inline(always)]
    pub fn canfdcksrdy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        canfdckcr::Canfdcksrdy,
        canfdckcr::Canfdcksrdy,
        Canfdckcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            canfdckcr::Canfdcksrdy,
            canfdckcr::Canfdcksrdy,
            Canfdckcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Canfdckcr {
    #[inline(always)]
    fn default() -> Canfdckcr {
        <crate::RegValueT<Canfdckcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod canfdckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Canfdcksel_SPEC;
    pub type Canfdcksel = crate::EnumBitfieldStruct<u8, Canfdcksel_SPEC>;
    impl Canfdcksel {
        #[doc = "HOCO"]
        pub const _000: Self = Self::new(0);

        #[doc = "MOCO (value after reset)"]
        pub const _001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _010: Self = Self::new(2);

        #[doc = "Main clock oscillator"]
        pub const _011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator"]
        pub const _100: Self = Self::new(4);

        #[doc = "PLL"]
        pub const _101: Self = Self::new(5);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Canfdcksreq_SPEC;
    pub type Canfdcksreq = crate::EnumBitfieldStruct<u8, Canfdcksreq_SPEC>;
    impl Canfdcksreq {
        #[doc = "No request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request switching"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Canfdcksrdy_SPEC;
    pub type Canfdcksrdy = crate::EnumBitfieldStruct<u8, Canfdcksrdy_SPEC>;
    impl Canfdcksrdy {
        #[doc = "Switching not possible"]
        pub const _0: Self = Self::new(0);

        #[doc = "Switching possible"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3Cckcr_SPEC;
impl crate::sealed::RegSpec for I3Cckcr_SPEC {
    type DataType = u8;
}

#[doc = "I3C Clock Control Register"]
pub type I3Cckcr = crate::RegValueT<I3Cckcr_SPEC>;

impl I3Cckcr {
    #[doc = "I3C clock (I3CCLK) Source Select"]
    #[inline(always)]
    pub fn i3ccksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        i3cckcr::I3Ccksel,
        i3cckcr::I3Ccksel,
        I3Cckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            i3cckcr::I3Ccksel,
            i3cckcr::I3Ccksel,
            I3Cckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I3C clock (I3CCLK) Switching Request"]
    #[inline(always)]
    pub fn i3ccksreq(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        i3cckcr::I3Ccksreq,
        i3cckcr::I3Ccksreq,
        I3Cckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            i3cckcr::I3Ccksreq,
            i3cckcr::I3Ccksreq,
            I3Cckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I3C clock (I3CCLK) Switching Ready State Flag"]
    #[inline(always)]
    pub fn i3ccksrdy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        i3cckcr::I3Ccksrdy,
        i3cckcr::I3Ccksrdy,
        I3Cckcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            i3cckcr::I3Ccksrdy,
            i3cckcr::I3Ccksrdy,
            I3Cckcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3Cckcr {
    #[inline(always)]
    fn default() -> I3Cckcr {
        <crate::RegValueT<I3Cckcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod i3cckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct I3Ccksel_SPEC;
    pub type I3Ccksel = crate::EnumBitfieldStruct<u8, I3Ccksel_SPEC>;
    impl I3Ccksel {
        #[doc = "HOCO"]
        pub const _000: Self = Self::new(0);

        #[doc = "MOCO (value after reset)"]
        pub const _001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _010: Self = Self::new(2);

        #[doc = "Main clock oscillator"]
        pub const _011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator"]
        pub const _100: Self = Self::new(4);

        #[doc = "PLL"]
        pub const _101: Self = Self::new(5);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct I3Ccksreq_SPEC;
    pub type I3Ccksreq = crate::EnumBitfieldStruct<u8, I3Ccksreq_SPEC>;
    impl I3Ccksreq {
        #[doc = "No request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request switching"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct I3Ccksrdy_SPEC;
    pub type I3Ccksrdy = crate::EnumBitfieldStruct<u8, I3Ccksrdy_SPEC>;
    impl I3Ccksrdy {
        #[doc = "Impossible to switch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Possible to switch"]
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
    #[doc = "CTSU non-touch automatic judgment Snooze End Enable"]
    #[inline(always)]
    pub fn ctsunajed(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        snzedcr1::Ctsunajed,
        snzedcr1::Ctsunajed,
        Snzedcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            snzedcr1::Ctsunajed,
            snzedcr1::Ctsunajed,
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
    pub struct Ctsunajed_SPEC;
    pub type Ctsunajed = crate::EnumBitfieldStruct<u8, Ctsunajed_SPEC>;
    impl Ctsunajed {
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

    #[doc = "Enable IRQ10 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen10,
        snzreqcr0::Snzreqen10,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen10,
            snzreqcr0::Snzreqen10,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ11 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen11,
        snzreqcr0::Snzreqen11,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen11,
            snzreqcr0::Snzreqen11,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ12 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen12,
        snzreqcr0::Snzreqen12,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen12,
            snzreqcr0::Snzreqen12,
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

    #[doc = "Enable IRQ14 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen14,
        snzreqcr0::Snzreqen14,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen14,
            snzreqcr0::Snzreqen14,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ15 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen15,
        snzreqcr0::Snzreqen15,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen15,
            snzreqcr0::Snzreqen15,
            Snzreqcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable ACMP_LP0 snooze request"]
    #[inline(always)]
    pub fn snzreqen23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        snzreqcr0::Snzreqen23,
        snzreqcr0::Snzreqen23,
        Snzreqcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            snzreqcr0::Snzreqen23,
            snzreqcr0::Snzreqen23,
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
    pub struct Snzreqen10_SPEC;
    pub type Snzreqen10 = crate::EnumBitfieldStruct<u8, Snzreqen10_SPEC>;
    impl Snzreqen10 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen11_SPEC;
    pub type Snzreqen11 = crate::EnumBitfieldStruct<u8, Snzreqen11_SPEC>;
    impl Snzreqen11 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen12_SPEC;
    pub type Snzreqen12 = crate::EnumBitfieldStruct<u8, Snzreqen12_SPEC>;
    impl Snzreqen12 {
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
    pub struct Snzreqen14_SPEC;
    pub type Snzreqen14 = crate::EnumBitfieldStruct<u8, Snzreqen14_SPEC>;
    impl Snzreqen14 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen15_SPEC;
    pub type Snzreqen15 = crate::EnumBitfieldStruct<u8, Snzreqen15_SPEC>;
    impl Snzreqen15 {
        #[doc = "Disable the snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen23_SPEC;
    pub type Snzreqen23 = crate::EnumBitfieldStruct<u8, Snzreqen23_SPEC>;
    impl Snzreqen23 {
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
pub struct Psmcr_SPEC;
impl crate::sealed::RegSpec for Psmcr_SPEC {
    type DataType = u8;
}

#[doc = "Power Save Memory Control Register"]
pub type Psmcr = crate::RegValueT<Psmcr_SPEC>;

impl Psmcr {
    #[doc = "Power Save Memory Control bit 0"]
    #[inline(always)]
    pub fn psmc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        psmcr::Psmc0,
        psmcr::Psmc0,
        Psmcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            psmcr::Psmc0,
            psmcr::Psmc0,
            Psmcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Power Save Memory Control bit 1"]
    #[inline(always)]
    pub fn psmc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        psmcr::Psmc1,
        psmcr::Psmc1,
        Psmcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            psmcr::Psmc1,
            psmcr::Psmc1,
            Psmcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Power Save Memory Control bit 2"]
    #[inline(always)]
    pub fn psmc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        psmcr::Psmc2,
        psmcr::Psmc2,
        Psmcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            psmcr::Psmc2,
            psmcr::Psmc2,
            Psmcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Power Save Memory Control bit 3"]
    #[inline(always)]
    pub fn psmc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        psmcr::Psmc3,
        psmcr::Psmc3,
        Psmcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            psmcr::Psmc3,
            psmcr::Psmc3,
            Psmcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Psmcr {
    #[inline(always)]
    fn default() -> Psmcr {
        <crate::RegValueT<Psmcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod psmcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psmc0_SPEC;
    pub type Psmc0 = crate::EnumBitfieldStruct<u8, Psmc0_SPEC>;
    impl Psmc0 {
        #[doc = "The SRAM data of 0x2000_0000 to 0x2000_3FFF area is retained in Software Standby mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SRAM data of 0x2000_0000 to 0x2000_3FFF area is not retained in Software Standby mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psmc1_SPEC;
    pub type Psmc1 = crate::EnumBitfieldStruct<u8, Psmc1_SPEC>;
    impl Psmc1 {
        #[doc = "The SRAM data of 0x2000_4000 to 0x2000_7FFF area is retained in Software Standby mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SRAM data of 0x2000_4000 to 0x2000_7FFF area is not retained in Software Standby mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psmc2_SPEC;
    pub type Psmc2 = crate::EnumBitfieldStruct<u8, Psmc2_SPEC>;
    impl Psmc2 {
        #[doc = "The SRAM data of 0x2000_8000 to 0x2000_BFFF area is retained in Software Standby mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SRAM data of 0x2000_8000 to 0x2000_BFFF area is not retained in Software Standby mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psmc3_SPEC;
    pub type Psmc3 = crate::EnumBitfieldStruct<u8, Psmc3_SPEC>;
    impl Psmc3 {
        #[doc = "The SRAM data of 0x2000_C000 to 0x2000_FFFF area is retained in Software Standby mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SRAM data of 0x2000_C000 to 0x2000_FFFF area is not retained in Software Standby mode."]
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
        <crate::RegValueT<Opccr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod opccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opcm_SPEC;
    pub type Opcm = crate::EnumBitfieldStruct<u8, Opcm_SPEC>;
    impl Opcm {
        #[doc = "High-speed mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle-speed mode"]
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
        #[doc = "Wait time = 2 cycles (0.25 us)"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Wait time = 1024 cycles (128 us)"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "Wait time = 2048 cycles (256 us)"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "Wait time = 4096 cycles (512 us)"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "Wait time = 8192 cycles (1024 us)"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "Wait time = 16384 cycles (2048 us)"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "Wait time = 32768 cycles (4096 us)"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "Wait time = 65536 cycles (8192 us)"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "Wait time = 131072 cycles (16384 us)"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "Wait time = 262144 cycles (32768 us)"]
        pub const _0_X_9: Self = Self::new(9);
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
pub struct Ldocr_SPEC;
impl crate::sealed::RegSpec for Ldocr_SPEC {
    type DataType = u8;
}

#[doc = "LDO Control Register"]
pub type Ldocr = crate::RegValueT<Ldocr_SPEC>;

impl Ldocr {
    #[doc = "Charge up 0"]
    #[inline(always)]
    pub fn chg0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ldocr::Chg0,
        ldocr::Chg0,
        Ldocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ldocr::Chg0,
            ldocr::Chg0,
            Ldocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ldocr {
    #[inline(always)]
    fn default() -> Ldocr {
        <crate::RegValueT<Ldocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ldocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chg0_SPEC;
    pub type Chg0 = crate::EnumBitfieldStruct<u8, Chg0_SPEC>;
    impl Chg0 {
        #[doc = "LDO normal operating mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "LDO high performance mode"]
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

    #[doc = "SRAM ECC Error Reset Detect Flag"]
    #[inline(always)]
    pub fn reerf(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        rstsr1::Reerf,
        rstsr1::Reerf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            rstsr1::Reerf,
            rstsr1::Reerf,
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

    #[doc = "Cache Parity Error Reset Detect Flag"]
    #[inline(always)]
    pub fn cperf(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        rstsr1::Cperf,
        rstsr1::Cperf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            rstsr1::Cperf,
            rstsr1::Cperf,
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
    pub struct Reerf_SPEC;
    pub type Reerf = crate::EnumBitfieldStruct<u8, Reerf_SPEC>;
    impl Reerf {
        #[doc = "SRAM ECC error reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "SRAM ECC error reset detected"]
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
        #[doc = "TrustZone error reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "TrustZone error reset detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cperf_SPEC;
    pub type Cperf = crate::EnumBitfieldStruct<u8, Cperf_SPEC>;
    impl Cperf {
        #[doc = "Cache Parity error reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cache Parity error reset detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd1Cr1_SPEC;
impl crate::sealed::RegSpec for Lvd1Cr1_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 1 Circuit Control Register"]
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
        #[doc = "When VCC or the EXLVD pin >= Vdet2 (rise) is detected"]
        pub const _00: Self = Self::new(0);

        #[doc = "When VCC or the EXLVD pin < Vdet2 (fall) is detected"]
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
        #[doc = "VCC or the EXLVD pin < Vdet2"]
        pub const _0: Self = Self::new(0);

        #[doc = "VCC or the EXLVD pin >= Vdet2 or MON is disabled"]
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

    #[doc = "Non Secure Attribute bit 18"]
    #[inline(always)]
    pub fn nonsec18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        cgfsar::Nonsec18,
        cgfsar::Nonsec18,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            cgfsar::Nonsec18,
            cgfsar::Nonsec18,
            Cgfsar_SPEC,
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
        cgfsar::Nonsec20,
        cgfsar::Nonsec20,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            cgfsar::Nonsec20,
            cgfsar::Nonsec20,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non Secure Attribute bit 26"]
    #[inline(always)]
    pub fn nonsec26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        cgfsar::Nonsec26,
        cgfsar::Nonsec26,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            cgfsar::Nonsec26,
            cgfsar::Nonsec26,
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
    pub struct Nonsec20_SPEC;
    pub type Nonsec20 = crate::EnumBitfieldStruct<u8, Nonsec20_SPEC>;
    impl Nonsec20 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec26_SPEC;
    pub type Nonsec26 = crate::EnumBitfieldStruct<u8, Nonsec26_SPEC>;
    impl Nonsec26 {
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

    #[doc = "Enable writing to the registers related to the low power modes"]
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
pub struct Syocdcr_SPEC;
impl crate::sealed::RegSpec for Syocdcr_SPEC {
    type DataType = u8;
}

#[doc = "System Control OCD Control Register"]
pub type Syocdcr = crate::RegValueT<Syocdcr_SPEC>;

impl Syocdcr {
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
    #[doc = "Main Clock Oscillator Drive Capability 1 Switching"]
    #[inline(always)]
    pub fn modrv1(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        momcr::Modrv1,
        momcr::Modrv1,
        Momcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            momcr::Modrv1,
            momcr::Modrv1,
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
    pub struct Modrv1_SPEC;
    pub type Modrv1 = crate::EnumBitfieldStruct<u8, Modrv1_SPEC>;
    impl Modrv1 {
        #[doc = "10 MHz to 20 MHz"]
        pub const _0: Self = Self::new(0);

        #[doc = "1 MHz to 10 MHz"]
        pub const _1: Self = Self::new(1);
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
pub struct Lvd1Cmpcr_SPEC;
impl crate::sealed::RegSpec for Lvd1Cmpcr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitoring 1 Comparator Control Register"]
pub type Lvd1Cmpcr = crate::RegValueT<Lvd1Cmpcr_SPEC>;

impl Lvd1Cmpcr {
    #[doc = "Voltage Detection 1 Level Select"]
    #[inline(always)]
    pub fn lvd1lvl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        lvd1cmpcr::Lvd1Lvl,
        lvd1cmpcr::Lvd1Lvl,
        Lvd1Cmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
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
        <crate::RegValueT<Lvd1Cmpcr_SPEC> as RegisterValue<_>>::new(11)
    }
}
pub mod lvd1cmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Lvl_SPEC;
    pub type Lvd1Lvl = crate::EnumBitfieldStruct<u8, Lvd1Lvl_SPEC>;
    impl Lvd1Lvl {
        #[doc = "Vdet1_0"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "Vdet1_1"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "Vdet1_2"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "Vdet1_3"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "Vdet1_4"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "Vdet1_5"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "Vdet1_6"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "Vdet1_7"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "Vdet1_8"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "Vdet1_9"]
        pub const _0_X_09: Self = Self::new(9);

        #[doc = "Vdet1_A"]
        pub const _0_X_0_A: Self = Self::new(10);

        #[doc = "Vdet1_B"]
        pub const _0_X_0_B: Self = Self::new(11);
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
    #[doc = "Voltage Detection 2 Level Select"]
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

    #[doc = "Voltage Detection 2 Comparison (Voltage external input select)"]
    #[inline(always)]
    pub fn exvccinp2(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        lvd2cmpcr::Exvccinp2,
        lvd2cmpcr::Exvccinp2,
        Lvd2Cmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            lvd2cmpcr::Exvccinp2,
            lvd2cmpcr::Exvccinp2,
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
        #[doc = "Vdet2_0"]
        pub const _000: Self = Self::new(0);

        #[doc = "Vdet2_1"]
        pub const _001: Self = Self::new(1);

        #[doc = "Vdet2_2"]
        pub const _010: Self = Self::new(2);

        #[doc = "Vdet2_3"]
        pub const _011: Self = Self::new(3);

        #[doc = "Vdet2_4"]
        pub const _100: Self = Self::new(4);

        #[doc = "Vdet2_5"]
        pub const _101: Self = Self::new(5);

        #[doc = "Vdet2_6"]
        pub const _110: Self = Self::new(6);

        #[doc = "Vdet2_7"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exvccinp2_SPEC;
    pub type Exvccinp2 = crate::EnumBitfieldStruct<u8, Exvccinp2_SPEC>;
    impl Exvccinp2 {
        #[doc = "Power supply voltage (VCC)"]
        pub const _0: Self = Self::new(0);

        #[doc = "EXLVD pin input voltage"]
        pub const _1: Self = Self::new(1);
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
        <crate::RegValueT<Lvd1Cr0_SPEC> as RegisterValue<_>>::new(128)
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
    pub struct Cmpe_SPEC;
    pub type Cmpe = crate::EnumBitfieldStruct<u8, Cmpe_SPEC>;
    impl Cmpe {
        #[doc = "Disable voltage monitor 1 circuit comparison result output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable voltage monitor 1 circuit comparison result output"]
        pub const _1: Self = Self::new(1);
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
        <crate::RegValueT<Lvd2Cr0_SPEC> as RegisterValue<_>>::new(128)
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
    pub struct Cmpe_SPEC;
    pub type Cmpe = crate::EnumBitfieldStruct<u8, Cmpe_SPEC>;
    impl Cmpe {
        #[doc = "Disable voltage monitor 2 circuit comparison result output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable voltage monitor 2 circuit comparison result output"]
        pub const _1: Self = Self::new(1);
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
        #[doc = "Negate after a stabilization time (tLVD2) when VCC or the EXLVD pin > Vdet2 is detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Negate after a stabilization time (tLVD2) on assertion of the LVD2 reset"]
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
        <crate::RegValueT<Sosccr_SPEC> as RegisterValue<_>>::new(1)
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
        0,
        0x3,
        1,
        0,
        somcr::Sodrv,
        somcr::Sodrv,
        Somcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
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
        #[doc = "Normal mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "Low power mode 1"]
        pub const _01: Self = Self::new(1);

        #[doc = "Low power mode 2"]
        pub const _10: Self = Self::new(2);

        #[doc = "Low power mode 3"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Somrg_SPEC;
impl crate::sealed::RegSpec for Somrg_SPEC {
    type DataType = u8;
}

#[doc = "Sub-Clock Oscillator Margin Check Register"]
pub type Somrg = crate::RegValueT<Somrg_SPEC>;

impl Somrg {
    #[doc = "Sub Clock Oscillator Margin check Switching"]
    #[inline(always)]
    pub fn soscmrg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        somrg::Soscmrg,
        somrg::Soscmrg,
        Somrg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            somrg::Soscmrg,
            somrg::Soscmrg,
            Somrg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Somrg {
    #[inline(always)]
    fn default() -> Somrg {
        <crate::RegValueT<Somrg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod somrg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Soscmrg_SPEC;
    pub type Soscmrg = crate::EnumBitfieldStruct<u8, Soscmrg_SPEC>;
    impl Soscmrg {
        #[doc = "Normal current"]
        pub const _00: Self = Self::new(0);

        #[doc = "Lower margin check"]
        pub const _01: Self = Self::new(1);

        #[doc = "Upper margin check"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
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
