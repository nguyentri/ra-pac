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
// Generated from SVD 1.20.02, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:01:00 +0000

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

    #[doc = "Module Stop Control Register A"]
    #[inline(always)]
    pub const fn mstpcra(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
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

    #[doc = "Lower Power Operation Control Register"]
    #[inline(always)]
    pub const fn lpopt(&self) -> &'static crate::common::Reg<self::Lpopt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lpopt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
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

    #[doc = "High-Speed On-Chip Oscillator Wait Control Register"]
    #[inline(always)]
    pub const fn hocowtcr(
        &self,
    ) -> &'static crate::common::Reg<self::Hocowtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hocowtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(165usize),
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

    #[doc = "24-bit Sigma-delta A/D Converter Clock Control Register"]
    #[inline(always)]
    pub const fn sdadcckcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadcckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(209usize),
            )
        }
    }

    #[doc = "Sub Clock Oscillation Stop Detection Control Register"]
    #[inline(always)]
    pub const fn sostd(&self) -> &'static crate::common::Reg<self::Sostd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sostd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }

    #[doc = "Main Oscillation Stop Detection Control Register"]
    #[inline(always)]
    pub const fn mostd(&self) -> &'static crate::common::Reg<self::Mostd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mostd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(218usize),
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

    #[doc = "Voltage Monitor Circuit Control Register"]
    #[inline(always)]
    pub const fn lvcmpcr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvcmpcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvcmpcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1047usize),
            )
        }
    }

    #[doc = "Voltage Detection Level Select Register"]
    #[inline(always)]
    pub const fn lvdlvlr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvdlvlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvdlvlr_SPEC, crate::common::RW>::from_ptr(
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

    #[doc = "EXLVDVBAT Circuit Control Register"]
    #[inline(always)]
    pub const fn vbtlvdcr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtlvdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtlvdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1216usize),
            )
        }
    }

    #[doc = "EXLVDVBAT Circuit Status Register"]
    #[inline(always)]
    pub const fn vbtlvdsr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtlvdsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtlvdsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1217usize),
            )
        }
    }

    #[doc = "EXLVDVBAT Comparator Control Register"]
    #[inline(always)]
    pub const fn vbtcmpcr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtcmpcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtcmpcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1218usize),
            )
        }
    }

    #[doc = "EXLVDVBAT Pin Low Voltage Detect Interrupt Control Register"]
    #[inline(always)]
    pub const fn vbtlvdicr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtlvdicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtlvdicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1219usize),
            )
        }
    }

    #[doc = "LVDVRTC Circuit Control Register"]
    #[inline(always)]
    pub const fn vrtlvdcr(
        &self,
    ) -> &'static crate::common::Reg<self::Vrtlvdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vrtlvdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1220usize),
            )
        }
    }

    #[doc = "VRTC Status Register"]
    #[inline(always)]
    pub const fn vrtsr(&self) -> &'static crate::common::Reg<self::Vrtsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vrtsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1221usize),
            )
        }
    }

    #[doc = "VRTC Comparator Control Register"]
    #[inline(always)]
    pub const fn vrtcmpcr(
        &self,
    ) -> &'static crate::common::Reg<self::Vrtcmpcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vrtcmpcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1222usize),
            )
        }
    }

    #[doc = "VRTC Pin Low Voltage Detect Interrupt Control Register"]
    #[inline(always)]
    pub const fn vrtlvdicr(
        &self,
    ) -> &'static crate::common::Reg<self::Vrtlvdicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vrtlvdicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1223usize),
            )
        }
    }

    #[doc = "EXLVD Circuit Control Register"]
    #[inline(always)]
    pub const fn exlvdcr(
        &self,
    ) -> &'static crate::common::Reg<self::Exlvdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Exlvdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1224usize),
            )
        }
    }

    #[doc = "EXLVD Circuit Status Register"]
    #[inline(always)]
    pub const fn exlvdsr(
        &self,
    ) -> &'static crate::common::Reg<self::Exlvdsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Exlvdsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1225usize),
            )
        }
    }

    #[doc = "EXLVD Comparator Control Register"]
    #[inline(always)]
    pub const fn exlvdcmpcr(
        &self,
    ) -> &'static crate::common::Reg<self::Exlvdcmpcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Exlvdcmpcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1226usize),
            )
        }
    }

    #[doc = "EXLVD Pin Low Voltage Detect Interrupt Control Register"]
    #[inline(always)]
    pub const fn exlvdicr(
        &self,
    ) -> &'static crate::common::Reg<self::Exlvdicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Exlvdicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1227usize),
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
pub struct Mstpcra_SPEC;
impl crate::sealed::RegSpec for Mstpcra_SPEC {
    type DataType = u32;
}

#[doc = "Module Stop Control Register A"]
pub type Mstpcra = crate::RegValueT<Mstpcra_SPEC>;

impl Mstpcra {
    #[doc = "Sub Oscillation Stop Detection for SDADCCLK Module Stop"]
    #[inline(always)]
    pub fn mstpa16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mstpcra::Mstpa16,
        mstpcra::Mstpa16,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mstpcra::Mstpa16,
            mstpcra::Mstpa16,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Main Oscillation Stop Detection for SDADCCLK Module Stop"]
    #[inline(always)]
    pub fn mstpa17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mstpcra::Mstpa17,
        mstpcra::Mstpa17,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mstpcra::Mstpa17,
            mstpcra::Mstpa17,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Module Stop"]
    #[inline(always)]
    pub fn mstpa22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mstpcra::Mstpa22,
        mstpcra::Mstpa22,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mstpcra::Mstpa22,
            mstpcra::Mstpa22,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mstpcra {
    #[inline(always)]
    fn default() -> Mstpcra {
        <crate::RegValueT<Mstpcra_SPEC> as RegisterValue<_>>::new(4290772991)
    }
}
pub mod mstpcra {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa16_SPEC;
    pub type Mstpa16 = crate::EnumBitfieldStruct<u8, Mstpa16_SPEC>;
    impl Mstpa16 {
        #[doc = "Supply input clock. SFR used by SOSTD can be read/written."]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop supply of input clock. SFR used by SOSTD cannot be written. SOSTD clock is stopped and error flag is in the reset status."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa17_SPEC;
    pub type Mstpa17 = crate::EnumBitfieldStruct<u8, Mstpa17_SPEC>;
    impl Mstpa17 {
        #[doc = "Supply input clock. SFR used by MOSTD can be read/written."]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop supply of input clock. SFR used by MOSTD cannot be written. MOSTD clock is stopped and error flag is in the reset status."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa22_SPEC;
    pub type Mstpa22 = crate::EnumBitfieldStruct<u8, Mstpa22_SPEC>;
    impl Mstpa22 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
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
}
impl ::core::default::Default for Sckdivcr {
    #[inline(always)]
    fn default() -> Sckdivcr {
        <crate::RegValueT<Sckdivcr_SPEC> as RegisterValue<_>>::new(67109892)
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

        #[doc = "Setting prohibited"]
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
    #[doc = "PLL Frequency Multiplication Factor Select"]
    #[inline(always)]
    pub fn pllmul(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        pllccr::Pllmul,
        pllccr::Pllmul,
        Pllccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            pllccr::Pllmul,
            pllccr::Pllmul,
            Pllccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pllccr {
    #[inline(always)]
    fn default() -> Pllccr {
        <crate::RegValueT<Pllccr_SPEC> as RegisterValue<_>>::new(40452)
    }
}
pub mod pllccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllmul_SPEC;
    pub type Pllmul = crate::EnumBitfieldStruct<u8, Pllmul_SPEC>;
    impl Pllmul {
        #[doc = "× 732 (value after reset)"]
        pub const _0_X_9_E: Self = Self::new(158);

        #[doc = "× 781"]
        pub const _0_X_CF: Self = Self::new(207);
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
        0x1,
        1,
        0,
        memwait::Memwait,
        memwait::Memwait,
        Memwait_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
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
        pub const _0: Self = Self::new(0);

        #[doc = "Wait"]
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
        #[doc = "The PLL clock is stopped or oscillation of the PLL clock is not yet stable"]
        pub const _0: Self = Self::new(0);

        #[doc = "The PLL clock is stable, so is available for use as the SDADCCLK clock"]
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

        #[doc = "PLL"]
        pub const _101: Self = Self::new(5);
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
pub struct Lpopt_SPEC;
impl crate::sealed::RegSpec for Lpopt_SPEC {
    type DataType = u8;
}

#[doc = "Lower Power Operation Control Register"]
pub type Lpopt = crate::RegValueT<Lpopt_SPEC>;

impl Lpopt {
    #[doc = "MPU Clock Disable Control"]
    #[inline(always)]
    pub fn mpudis(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lpopt::Mpudis,
        lpopt::Mpudis,
        Lpopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lpopt::Mpudis,
            lpopt::Mpudis,
            Lpopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Debug Clock Disable Control"]
    #[inline(always)]
    pub fn dclkdis(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        lpopt::Dclkdis,
        lpopt::Dclkdis,
        Lpopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            lpopt::Dclkdis,
            lpopt::Dclkdis,
            Lpopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BPF Clock Disable Control"]
    #[inline(always)]
    pub fn bpfclkdis(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        lpopt::Bpfclkdis,
        lpopt::Bpfclkdis,
        Lpopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            lpopt::Bpfclkdis,
            lpopt::Bpfclkdis,
            Lpopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Lower Power Operation Enable"]
    #[inline(always)]
    pub fn lpopten(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        lpopt::Lpopten,
        lpopt::Lpopten,
        Lpopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            lpopt::Lpopten,
            lpopt::Lpopten,
            Lpopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lpopt {
    #[inline(always)]
    fn default() -> Lpopt {
        <crate::RegValueT<Lpopt_SPEC> as RegisterValue<_>>::new(64)
    }
}
pub mod lpopt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpudis_SPEC;
    pub type Mpudis = crate::EnumBitfieldStruct<u8, Mpudis_SPEC>;
    impl Mpudis {
        #[doc = "MPU operates as normal"]
        pub const _0: Self = Self::new(0);

        #[doc = "MPU operate clock stops (MPU function disable)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dclkdis_SPEC;
    pub type Dclkdis = crate::EnumBitfieldStruct<u8, Dclkdis_SPEC>;
    impl Dclkdis {
        #[doc = "Debug clock does not stop"]
        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpfclkdis_SPEC;
    pub type Bpfclkdis = crate::EnumBitfieldStruct<u8, Bpfclkdis_SPEC>;
    impl Bpfclkdis {
        #[doc = "Flash register R/W clock operates as normal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Flash register R/W clock stops."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lpopten_SPEC;
    pub type Lpopten = crate::EnumBitfieldStruct<u8, Lpopten_SPEC>;
    impl Lpopten {
        #[doc = "All lower power counter measure disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "All lower power counter measure enable"]
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
    #[doc = "LCD Source Clock (LCDSRCCLK) Select"]
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

    #[doc = "LCD Source Clock Out Enable"]
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

        #[doc = "MOCO"]
        pub const _011: Self = Self::new(3);

        #[doc = "HOCO"]
        pub const _100: Self = Self::new(4);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdscken_SPEC;
    pub type Lcdscken = crate::EnumBitfieldStruct<u8, Lcdscken_SPEC>;
    impl Lcdscken {
        #[doc = "LCD source clock out disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "LCD source clock out enabled"]
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
    #[doc = "AGTW1 Underflow Snooze End Enable"]
    #[inline(always)]
    pub fn agtwunfed(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        snzedcr0::Agtwunfed,
        snzedcr0::Agtwunfed,
        Snzedcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            snzedcr0::Agtwunfed,
            snzedcr0::Agtwunfed,
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
    pub struct Agtwunfed_SPEC;
    pub type Agtwunfed = crate::EnumBitfieldStruct<u8, Agtwunfed_SPEC>;
    impl Agtwunfed {
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
pub struct Snzreqcr0_SPEC;
impl crate::sealed::RegSpec for Snzreqcr0_SPEC {
    type DataType = u32;
}

#[doc = "Snooze Request Control Register 0"]
pub type Snzreqcr0 = crate::RegValueT<Snzreqcr0_SPEC>;

impl Snzreqcr0 {
    #[doc = "Enable IRQ0 Pin Snooze Request"]
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

    #[doc = "Enable IRQ1 Pin Snooze Request"]
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

    #[doc = "Enable IRQ2 Pin Snooze Request"]
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

    #[doc = "Enable IRQ3 Pin Snooze Request"]
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

    #[doc = "Enable IRQ4 Pin Snooze Request"]
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

    #[doc = "Enable IRQ5 Pin Snooze Request"]
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

    #[doc = "Enable IRQ6 Pin Snooze Request"]
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

    #[doc = "Enable IRQ7 Pin Snooze Request"]
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

    #[doc = "Enable IRQ8 Pin Snooze Request"]
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

    #[doc = "Enable IRQ9 Pin Snooze Request"]
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

    #[doc = "Enable IRQ10 Pin Snooze Request"]
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

    #[doc = "Enable IRQ11 Pin Snooze Request"]
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

    #[doc = "Enable RTC Alarm 1 Snooze Request"]
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

    #[doc = "Enable RTC Alarm 0 Snooze Request"]
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

    #[doc = "Enable RTC Period Snooze Request"]
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

    #[doc = "Enable AGTW1 Underflow Snooze Request"]
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

    #[doc = "Enable AGTW1 Compare Match A Snooze Request"]
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

    #[doc = "Enable AGTW1 Compare Match B Snooze Request"]
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
    #[doc = "Power Save Memory Control"]
    #[inline(always)]
    pub fn psmc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        psmcr::Psmc,
        psmcr::Psmc,
        Psmcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            psmcr::Psmc,
            psmcr::Psmc,
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
    pub struct Psmc_SPEC;
    pub type Psmc = crate::EnumBitfieldStruct<u8, Psmc_SPEC>;
    impl Psmc {
        #[doc = "All SRAMs are on in Software Standby mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "8 KB SRAM (0x2000_4000 to 0x2000_5FFF) is on in Software Standby mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
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
pub struct Hocowtcr_SPEC;
impl crate::sealed::RegSpec for Hocowtcr_SPEC {
    type DataType = u8;
}

#[doc = "High-Speed On-Chip Oscillator Wait Control Register"]
pub type Hocowtcr = crate::RegValueT<Hocowtcr_SPEC>;

impl Hocowtcr {
    #[doc = "HOCO Wait Time Setting"]
    #[inline(always)]
    pub fn hsts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        hocowtcr::Hsts,
        hocowtcr::Hsts,
        Hocowtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            hocowtcr::Hsts,
            hocowtcr::Hsts,
            Hocowtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Hocowtcr {
    #[inline(always)]
    fn default() -> Hocowtcr {
        <crate::RegValueT<Hocowtcr_SPEC> as RegisterValue<_>>::new(5)
    }
}
pub mod hocowtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsts_SPEC;
    pub type Hsts = crate::EnumBitfieldStruct<u8, Hsts_SPEC>;
    impl Hsts {
        #[doc = "Value after reset."]
        pub const _101: Self = Self::new(5);

        #[doc = "Before starting high-speed on-chip oscillator by setting HOCOCR.HCSTP bit, the HSTS\\[2:0\\] bits must be set to 011b beforehand. Wait time = 46 cycles (5.75 µs) Wait time is calculated at MOCO = 8 MHz (typically 0.125 µs)."]
        pub const _011: Self = Self::new(3);
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

    #[doc = "Bus Slave MPU Error Reset Detect Flag"]
    #[inline(always)]
    pub fn bussrf(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        rstsr1::Bussrf,
        rstsr1::Bussrf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            rstsr1::Bussrf,
            rstsr1::Bussrf,
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

    #[doc = "CPU Stack Pointer Error Reset Detect Flag"]
    #[inline(always)]
    pub fn sperf(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        rstsr1::Sperf,
        rstsr1::Sperf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            rstsr1::Sperf,
            rstsr1::Sperf,
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
    pub struct Bussrf_SPEC;
    pub type Bussrf = crate::EnumBitfieldStruct<u8, Bussrf_SPEC>;
    impl Bussrf {
        #[doc = "Bus slave MPU error reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus slave MPU error reset detected"]
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
    pub struct Sperf_SPEC;
    pub type Sperf = crate::EnumBitfieldStruct<u8, Sperf_SPEC>;
    impl Sperf {
        #[doc = "CPU stack pointer error reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU stack pointer error reset detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadcckcr_SPEC;
impl crate::sealed::RegSpec for Sdadcckcr_SPEC {
    type DataType = u8;
}

#[doc = "24-bit Sigma-delta A/D Converter Clock Control Register"]
pub type Sdadcckcr = crate::RegValueT<Sdadcckcr_SPEC>;

impl Sdadcckcr {
    #[doc = "24-bit Sigma-delta A/D Converter Clock Select"]
    #[inline(always)]
    pub fn cksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        sdadcckcr::Cksel,
        sdadcckcr::Cksel,
        Sdadcckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            sdadcckcr::Cksel,
            sdadcckcr::Cksel,
            Sdadcckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Switch Enable for Oscillation Stop Detected"]
    #[inline(always)]
    pub fn ostdcse(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sdadcckcr::Ostdcse,
        sdadcckcr::Ostdcse,
        Sdadcckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sdadcckcr::Ostdcse,
            sdadcckcr::Ostdcse,
            Sdadcckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadcckcr {
    #[inline(always)]
    fn default() -> Sdadcckcr {
        <crate::RegValueT<Sdadcckcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadcckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksel_SPEC;
    pub type Cksel = crate::EnumBitfieldStruct<u8, Cksel_SPEC>;
    impl Cksel {
        #[doc = "MOSC"]
        pub const _00: Self = Self::new(0);

        #[doc = "HOCO"]
        pub const _01: Self = Self::new(1);

        #[doc = "PLL"]
        pub const _10: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostdcse_SPEC;
    pub type Ostdcse = crate::EnumBitfieldStruct<u8, Ostdcse_SPEC>;
    impl Ostdcse {
        #[doc = "Not switched to HOCO when oscillation stop detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Switched to HOCO, when oscillation stop detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sostd_SPEC;
impl crate::sealed::RegSpec for Sostd_SPEC {
    type DataType = u16;
}

#[doc = "Sub Clock Oscillation Stop Detection Control Register"]
pub type Sostd = crate::RegValueT<Sostd_SPEC>;

impl Sostd {
    #[doc = "Oscillation Stop Detection Time"]
    #[inline(always)]
    pub fn osdccmp(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Sostd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Sostd_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Status of Oscillation Stop Detector Operation"]
    #[inline(always)]
    pub fn osdcf(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        sostd::Osdcf,
        sostd::Osdcf,
        Sostd_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            sostd::Osdcf,
            sostd::Osdcf,
            Sostd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Control of Oscillation Stop Detector Operation"]
    #[inline(always)]
    pub fn osdce(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        sostd::Osdce,
        sostd::Osdce,
        Sostd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            sostd::Osdce,
            sostd::Osdce,
            Sostd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sostd {
    #[inline(always)]
    fn default() -> Sostd {
        <crate::RegValueT<Sostd_SPEC> as RegisterValue<_>>::new(4095)
    }
}
pub mod sostd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Osdcf_SPEC;
    pub type Osdcf = crate::EnumBitfieldStruct<u8, Osdcf_SPEC>;
    impl Osdcf {
        #[doc = "Stop operation of the oscillation stop detector"]
        pub const _0: Self = Self::new(0);

        #[doc = "Run operation of the oscillation stop detector"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Osdce_SPEC;
    pub type Osdce = crate::EnumBitfieldStruct<u8, Osdce_SPEC>;
    impl Osdce {
        #[doc = "Stop operation of the oscillation stop detector"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start operation of the oscillation stop detector"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mostd_SPEC;
impl crate::sealed::RegSpec for Mostd_SPEC {
    type DataType = u16;
}

#[doc = "Main Oscillation Stop Detection Control Register"]
pub type Mostd = crate::RegValueT<Mostd_SPEC>;

impl Mostd {
    #[doc = "Oscillation Stop Detection Time"]
    #[inline(always)]
    pub fn osdccmp(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Mostd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Mostd_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Status of Oscillation Stop Detector Operation"]
    #[inline(always)]
    pub fn osdcf(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        mostd::Osdcf,
        mostd::Osdcf,
        Mostd_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            mostd::Osdcf,
            mostd::Osdcf,
            Mostd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Control of Oscillation Stop Detector Operation"]
    #[inline(always)]
    pub fn osdce(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        mostd::Osdce,
        mostd::Osdce,
        Mostd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            mostd::Osdce,
            mostd::Osdce,
            Mostd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mostd {
    #[inline(always)]
    fn default() -> Mostd {
        <crate::RegValueT<Mostd_SPEC> as RegisterValue<_>>::new(4095)
    }
}
pub mod mostd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Osdcf_SPEC;
    pub type Osdcf = crate::EnumBitfieldStruct<u8, Osdcf_SPEC>;
    impl Osdcf {
        #[doc = "Stop operation of the oscillation stop detector"]
        pub const _0: Self = Self::new(0);

        #[doc = "Run operation of the oscillation stop detector"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Osdce_SPEC;
    pub type Osdce = crate::EnumBitfieldStruct<u8, Osdce_SPEC>;
    impl Osdce {
        #[doc = "Stop operation of the oscillation stop detector"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start operation of the oscillation stop detector"]
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
        #[doc = "When VCC >= Vdet2 (rise) is detected"]
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

        #[doc = "VCC >= Vdet2 or MON is disabled"]
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
pub struct Lvcmpcr_SPEC;
impl crate::sealed::RegSpec for Lvcmpcr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor Circuit Control Register"]
pub type Lvcmpcr = crate::RegValueT<Lvcmpcr_SPEC>;

impl Lvcmpcr {
    #[doc = "Voltage Detection 1 Enable"]
    #[inline(always)]
    pub fn lvd1e(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        lvcmpcr::Lvd1E,
        lvcmpcr::Lvd1E,
        Lvcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            lvcmpcr::Lvd1E,
            lvcmpcr::Lvd1E,
            Lvcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Detection 2 Enable"]
    #[inline(always)]
    pub fn lvd2e(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        lvcmpcr::Lvd2E,
        lvcmpcr::Lvd2E,
        Lvcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            lvcmpcr::Lvd2E,
            lvcmpcr::Lvd2E,
            Lvcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvcmpcr {
    #[inline(always)]
    fn default() -> Lvcmpcr {
        <crate::RegValueT<Lvcmpcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lvcmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1E_SPEC;
    pub type Lvd1E = crate::EnumBitfieldStruct<u8, Lvd1E_SPEC>;
    impl Lvd1E {
        #[doc = "Voltage detection 1 circuit disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage detection 1 circuit enabled"]
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
pub struct Lvdlvlr_SPEC;
impl crate::sealed::RegSpec for Lvdlvlr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Detection Level Select Register"]
pub type Lvdlvlr = crate::RegValueT<Lvdlvlr_SPEC>;

impl Lvdlvlr {
    #[doc = "Voltage Detection 1 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    pub fn lvd1lvl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        lvdlvlr::Lvd1Lvl,
        lvdlvlr::Lvd1Lvl,
        Lvdlvlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            lvdlvlr::Lvd1Lvl,
            lvdlvlr::Lvd1Lvl,
            Lvdlvlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Detection 2 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    pub fn lvd2lvl(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7,
        1,
        0,
        lvdlvlr::Lvd2Lvl,
        lvdlvlr::Lvd2Lvl,
        Lvdlvlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7,
            1,
            0,
            lvdlvlr::Lvd2Lvl,
            lvdlvlr::Lvd2Lvl,
            Lvdlvlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvdlvlr {
    #[inline(always)]
    fn default() -> Lvdlvlr {
        <crate::RegValueT<Lvdlvlr_SPEC> as RegisterValue<_>>::new(7)
    }
}
pub mod lvdlvlr {

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

        #[doc = "Vdet1_C"]
        pub const _0_X_0_C: Self = Self::new(12);

        #[doc = "Vdet1_D"]
        pub const _0_X_0_D: Self = Self::new(13);

        #[doc = "Vdet1_E"]
        pub const _0_X_0_E: Self = Self::new(14);

        #[doc = "Vdet1_F"]
        pub const _0_X_0_F: Self = Self::new(15);
    }
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
        #[doc = "Negate after a stabilization time (tLVD2) when VCC > Vdet2 is detected"]
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
        #[doc = "Normal Mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "Low Power Mode 1"]
        pub const _01: Self = Self::new(1);

        #[doc = "Low Power Mode 2"]
        pub const _10: Self = Self::new(2);

        #[doc = "Low Power Mode 3"]
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
    #[doc = "Sub Clock Oscillator Margin Check Switching"]
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

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtlvdcr_SPEC;
impl crate::sealed::RegSpec for Vbtlvdcr_SPEC {
    type DataType = u8;
}

#[doc = "EXLVDVBAT Circuit Control Register"]
pub type Vbtlvdcr = crate::RegValueT<Vbtlvdcr_SPEC>;

impl Vbtlvdcr {
    #[doc = "EXLVDVBAT Pin Low Voltage Detect Enable"]
    #[inline(always)]
    pub fn lvde(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        vbtlvdcr::Lvde,
        vbtlvdcr::Lvde,
        Vbtlvdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            vbtlvdcr::Lvde,
            vbtlvdcr::Lvde,
            Vbtlvdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "EXLVDVBAT Pin Low Voltage Detect Level Select"]
    #[inline(always)]
    pub fn lvl(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7,
        1,
        0,
        vbtlvdcr::Lvl,
        vbtlvdcr::Lvl,
        Vbtlvdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7,
            1,
            0,
            vbtlvdcr::Lvl,
            vbtlvdcr::Lvl,
            Vbtlvdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbtlvdcr {
    #[inline(always)]
    fn default() -> Vbtlvdcr {
        <crate::RegValueT<Vbtlvdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtlvdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvde_SPEC;
    pub type Lvde = crate::EnumBitfieldStruct<u8, Lvde_SPEC>;
    impl Lvde {
        #[doc = "EXLVDVBAT pin low voltage detection disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "EXLVDVBAT pin low voltage detection enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvl_SPEC;
    pub type Lvl = crate::EnumBitfieldStruct<u8, Lvl_SPEC>;
    impl Lvl {
        #[doc = "2.2 V"]
        pub const _000: Self = Self::new(0);

        #[doc = "2.4 V"]
        pub const _001: Self = Self::new(1);

        #[doc = "2.6 V"]
        pub const _010: Self = Self::new(2);

        #[doc = "2.7 V"]
        pub const _011: Self = Self::new(3);

        #[doc = "2.8 V"]
        pub const _100: Self = Self::new(4);

        #[doc = "2.9 V"]
        pub const _101: Self = Self::new(5);

        #[doc = "3.1 V"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtlvdsr_SPEC;
impl crate::sealed::RegSpec for Vbtlvdsr_SPEC {
    type DataType = u8;
}

#[doc = "EXLVDVBAT Circuit Status Register"]
pub type Vbtlvdsr = crate::RegValueT<Vbtlvdsr_SPEC>;

impl Vbtlvdsr {
    #[doc = "EXLVDVBAT Pin Low Voltage Detect Flag"]
    #[inline(always)]
    pub fn det(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vbtlvdsr::Det,
        vbtlvdsr::Det,
        Vbtlvdsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vbtlvdsr::Det,
            vbtlvdsr::Det,
            Vbtlvdsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "EXLVDVBAT Pin Low Voltage Monitor Flag"]
    #[inline(always)]
    pub fn mon(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        vbtlvdsr::Mon,
        vbtlvdsr::Mon,
        Vbtlvdsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            vbtlvdsr::Mon,
            vbtlvdsr::Mon,
            Vbtlvdsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbtlvdsr {
    #[inline(always)]
    fn default() -> Vbtlvdsr {
        <crate::RegValueT<Vbtlvdsr_SPEC> as RegisterValue<_>>::new(32)
    }
}
pub mod vbtlvdsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Det_SPEC;
    pub type Det = crate::EnumBitfieldStruct<u8, Det_SPEC>;
    impl Det {
        #[doc = "EXLVDVBAT pin low voltage not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "EXLVDVBAT pin low voltage detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mon_SPEC;
    pub type Mon = crate::EnumBitfieldStruct<u8, Mon_SPEC>;
    impl Mon {
        #[doc = "EXLVDVBAT < Vdet_lvdvbat"]
        pub const _0: Self = Self::new(0);

        #[doc = "EXLVDVBAT ≥ Vdet_lvdvbat or MON is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtcmpcr_SPEC;
impl crate::sealed::RegSpec for Vbtcmpcr_SPEC {
    type DataType = u8;
}

#[doc = "EXLVDVBAT Comparator Control Register"]
pub type Vbtcmpcr = crate::RegValueT<Vbtcmpcr_SPEC>;

impl Vbtcmpcr {
    #[doc = "EXLVDVBAT Pin Low Voltage Detect Circuit Output Enable"]
    #[inline(always)]
    pub fn cmpe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtcmpcr::Cmpe,
        vbtcmpcr::Cmpe,
        Vbtcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtcmpcr::Cmpe,
            vbtcmpcr::Cmpe,
            Vbtcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbtcmpcr {
    #[inline(always)]
    fn default() -> Vbtcmpcr {
        <crate::RegValueT<Vbtcmpcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtcmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpe_SPEC;
    pub type Cmpe = crate::EnumBitfieldStruct<u8, Cmpe_SPEC>;
    impl Cmpe {
        #[doc = "EXLVDVBAT pin low voltage detect circuit output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "EXLVDVBAT pin low voltage detect circuit output enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtlvdicr_SPEC;
impl crate::sealed::RegSpec for Vbtlvdicr_SPEC {
    type DataType = u8;
}

#[doc = "EXLVDVBAT Pin Low Voltage Detect Interrupt Control Register"]
pub type Vbtlvdicr = crate::RegValueT<Vbtlvdicr_SPEC>;

impl Vbtlvdicr {
    #[doc = "EXLVDVBAT Pin Low Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn ie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtlvdicr::Ie,
        vbtlvdicr::Ie,
        Vbtlvdicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtlvdicr::Ie,
            vbtlvdicr::Ie,
            Vbtlvdicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "EXLVDVBAT Pin Low Voltage Detect Interrupt Generation Condition Select"]
    #[inline(always)]
    pub fn idtsel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        vbtlvdicr::Idtsel,
        vbtlvdicr::Idtsel,
        Vbtlvdicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            vbtlvdicr::Idtsel,
            vbtlvdicr::Idtsel,
            Vbtlvdicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbtlvdicr {
    #[inline(always)]
    fn default() -> Vbtlvdicr {
        <crate::RegValueT<Vbtlvdicr_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod vbtlvdicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ie_SPEC;
    pub type Ie = crate::EnumBitfieldStruct<u8, Ie_SPEC>;
    impl Ie {
        #[doc = "EXLVDVBAT pin low voltage detection interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "EXLVDVBAT pin low voltage detection interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idtsel_SPEC;
    pub type Idtsel = crate::EnumBitfieldStruct<u8, Idtsel_SPEC>;
    impl Idtsel {
        #[doc = "When EXLVDVBAT > Vdet_lvdvbat (rise) is detected"]
        pub const _00: Self = Self::new(0);

        #[doc = "When EXLVDVBAT < Vdet_lvdvbat (fall) is detected"]
        pub const _01: Self = Self::new(1);

        #[doc = "When fall and rise are detected"]
        pub const _10: Self = Self::new(2);

        #[doc = "Settings prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vrtlvdcr_SPEC;
impl crate::sealed::RegSpec for Vrtlvdcr_SPEC {
    type DataType = u8;
}

#[doc = "LVDVRTC Circuit Control Register"]
pub type Vrtlvdcr = crate::RegValueT<Vrtlvdcr_SPEC>;

impl Vrtlvdcr {
    #[doc = "VRTC Pin Low Voltage Detect Enable"]
    #[inline(always)]
    pub fn lvde(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        vrtlvdcr::Lvde,
        vrtlvdcr::Lvde,
        Vrtlvdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            vrtlvdcr::Lvde,
            vrtlvdcr::Lvde,
            Vrtlvdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VRTC Pin Low Voltage Detect Level Select"]
    #[inline(always)]
    pub fn lvl(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        vrtlvdcr::Lvl,
        vrtlvdcr::Lvl,
        Vrtlvdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            vrtlvdcr::Lvl,
            vrtlvdcr::Lvl,
            Vrtlvdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vrtlvdcr {
    #[inline(always)]
    fn default() -> Vrtlvdcr {
        <crate::RegValueT<Vrtlvdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vrtlvdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvde_SPEC;
    pub type Lvde = crate::EnumBitfieldStruct<u8, Lvde_SPEC>;
    impl Lvde {
        #[doc = "VRTC pin low voltage detection disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "VRTC pin low voltage detection enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvl_SPEC;
    pub type Lvl = crate::EnumBitfieldStruct<u8, Lvl_SPEC>;
    impl Lvl {
        #[doc = "2.2 V"]
        pub const _00: Self = Self::new(0);

        #[doc = "2.4 V"]
        pub const _01: Self = Self::new(1);

        #[doc = "2.6 V"]
        pub const _10: Self = Self::new(2);

        #[doc = "2.8 V"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vrtsr_SPEC;
impl crate::sealed::RegSpec for Vrtsr_SPEC {
    type DataType = u8;
}

#[doc = "VRTC Status Register"]
pub type Vrtsr = crate::RegValueT<Vrtsr_SPEC>;

impl Vrtsr {
    #[doc = "VRTC-domain Power-on Reset Detect Flag"]
    #[inline(always)]
    pub fn pordet(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vrtsr::Pordet,
        vrtsr::Pordet,
        Vrtsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vrtsr::Pordet,
            vrtsr::Pordet,
            Vrtsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VRTC Voltage Variation Detection Flag"]
    #[inline(always)]
    pub fn det(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vrtsr::Det,
        vrtsr::Det,
        Vrtsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vrtsr::Det,
            vrtsr::Det,
            Vrtsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VRTC-domain Valid"]
    #[inline(always)]
    pub fn vrtvld(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        vrtsr::Vrtvld,
        vrtsr::Vrtvld,
        Vrtsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            vrtsr::Vrtvld,
            vrtsr::Vrtvld,
            Vrtsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "VRTC Signal Monitor Flag"]
    #[inline(always)]
    pub fn mon(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        vrtsr::Mon,
        vrtsr::Mon,
        Vrtsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            vrtsr::Mon,
            vrtsr::Mon,
            Vrtsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vrtsr {
    #[inline(always)]
    fn default() -> Vrtsr {
        <crate::RegValueT<Vrtsr_SPEC> as RegisterValue<_>>::new(33)
    }
}
pub mod vrtsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pordet_SPEC;
    pub type Pordet = crate::EnumBitfieldStruct<u8, Pordet_SPEC>;
    impl Pordet {
        #[doc = "VRTC-domain power-on reset not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "VRTC-domain power-on reset detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Det_SPEC;
    pub type Det = crate::EnumBitfieldStruct<u8, Det_SPEC>;
    impl Det {
        #[doc = "Vdet_vrtc crossing not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Vdet_vrtc crossing detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vrtvld_SPEC;
    pub type Vrtvld = crate::EnumBitfieldStruct<u8, Vrtvld_SPEC>;
    impl Vrtvld {
        #[doc = "VRTC-domain area not valid"]
        pub const _0: Self = Self::new(0);

        #[doc = "VRTC-domain area valid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mon_SPEC;
    pub type Mon = crate::EnumBitfieldStruct<u8, Mon_SPEC>;
    impl Mon {
        #[doc = "VRTC < Vdet_vrtc"]
        pub const _0: Self = Self::new(0);

        #[doc = "VRTC ≥ Vdet_vrtc or MON is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vrtcmpcr_SPEC;
impl crate::sealed::RegSpec for Vrtcmpcr_SPEC {
    type DataType = u8;
}

#[doc = "VRTC Comparator Control Register"]
pub type Vrtcmpcr = crate::RegValueT<Vrtcmpcr_SPEC>;

impl Vrtcmpcr {
    #[doc = "VRTC Pin Low Voltage Detect Circuit Output Enable"]
    #[inline(always)]
    pub fn cmpe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vrtcmpcr::Cmpe,
        vrtcmpcr::Cmpe,
        Vrtcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vrtcmpcr::Cmpe,
            vrtcmpcr::Cmpe,
            Vrtcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vrtcmpcr {
    #[inline(always)]
    fn default() -> Vrtcmpcr {
        <crate::RegValueT<Vrtcmpcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vrtcmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpe_SPEC;
    pub type Cmpe = crate::EnumBitfieldStruct<u8, Cmpe_SPEC>;
    impl Cmpe {
        #[doc = "VRTC pin low voltage detect circuit output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "VRTC pin low voltage detect circuit output enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vrtlvdicr_SPEC;
impl crate::sealed::RegSpec for Vrtlvdicr_SPEC {
    type DataType = u8;
}

#[doc = "VRTC Pin Low Voltage Detect Interrupt Control Register"]
pub type Vrtlvdicr = crate::RegValueT<Vrtlvdicr_SPEC>;

impl Vrtlvdicr {
    #[doc = "VRTC Pin Low Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn ie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vrtlvdicr::Ie,
        vrtlvdicr::Ie,
        Vrtlvdicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vrtlvdicr::Ie,
            vrtlvdicr::Ie,
            Vrtlvdicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VRTC Pin Low Voltage Detect Interrupt Generation Condition Select"]
    #[inline(always)]
    pub fn idtsel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        vrtlvdicr::Idtsel,
        vrtlvdicr::Idtsel,
        Vrtlvdicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            vrtlvdicr::Idtsel,
            vrtlvdicr::Idtsel,
            Vrtlvdicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vrtlvdicr {
    #[inline(always)]
    fn default() -> Vrtlvdicr {
        <crate::RegValueT<Vrtlvdicr_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod vrtlvdicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ie_SPEC;
    pub type Ie = crate::EnumBitfieldStruct<u8, Ie_SPEC>;
    impl Ie {
        #[doc = "VRTC pin low voltage detection interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "VRTC pin low voltage detection interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idtsel_SPEC;
    pub type Idtsel = crate::EnumBitfieldStruct<u8, Idtsel_SPEC>;
    impl Idtsel {
        #[doc = "When VRTC > Vdet_vrtc (rise) is detected"]
        pub const _00: Self = Self::new(0);

        #[doc = "When VRTC < Vdet_vrtc (fall) is detected"]
        pub const _01: Self = Self::new(1);

        #[doc = "When fall and rise are detected"]
        pub const _10: Self = Self::new(2);

        #[doc = "Settings prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exlvdcr_SPEC;
impl crate::sealed::RegSpec for Exlvdcr_SPEC {
    type DataType = u8;
}

#[doc = "EXLVD Circuit Control Register"]
pub type Exlvdcr = crate::RegValueT<Exlvdcr_SPEC>;

impl Exlvdcr {
    #[doc = "EXLVD Pin Low Voltage Detect Enable"]
    #[inline(always)]
    pub fn lvde(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        exlvdcr::Lvde,
        exlvdcr::Lvde,
        Exlvdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            exlvdcr::Lvde,
            exlvdcr::Lvde,
            Exlvdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Exlvdcr {
    #[inline(always)]
    fn default() -> Exlvdcr {
        <crate::RegValueT<Exlvdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod exlvdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvde_SPEC;
    pub type Lvde = crate::EnumBitfieldStruct<u8, Lvde_SPEC>;
    impl Lvde {
        #[doc = "EXLVD pin low voltage detection disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "EXLVD pin low voltage detection enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exlvdsr_SPEC;
impl crate::sealed::RegSpec for Exlvdsr_SPEC {
    type DataType = u8;
}

#[doc = "EXLVD Circuit Status Register"]
pub type Exlvdsr = crate::RegValueT<Exlvdsr_SPEC>;

impl Exlvdsr {
    #[doc = "EXLVD pin Low Voltage Detect Flag"]
    #[inline(always)]
    pub fn det(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        exlvdsr::Det,
        exlvdsr::Det,
        Exlvdsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            exlvdsr::Det,
            exlvdsr::Det,
            Exlvdsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "EXLVD pin Low Voltage Monitor Flag"]
    #[inline(always)]
    pub fn mon(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        exlvdsr::Mon,
        exlvdsr::Mon,
        Exlvdsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            exlvdsr::Mon,
            exlvdsr::Mon,
            Exlvdsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Exlvdsr {
    #[inline(always)]
    fn default() -> Exlvdsr {
        <crate::RegValueT<Exlvdsr_SPEC> as RegisterValue<_>>::new(32)
    }
}
pub mod exlvdsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Det_SPEC;
    pub type Det = crate::EnumBitfieldStruct<u8, Det_SPEC>;
    impl Det {
        #[doc = "EXLVD pin low voltage not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "EXLVD pin low voltage detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mon_SPEC;
    pub type Mon = crate::EnumBitfieldStruct<u8, Mon_SPEC>;
    impl Mon {
        #[doc = "EXLVD < Vdet_exlvd"]
        pub const _0: Self = Self::new(0);

        #[doc = "EXLVD ≥ Vdet_exlvd or MON is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exlvdcmpcr_SPEC;
impl crate::sealed::RegSpec for Exlvdcmpcr_SPEC {
    type DataType = u8;
}

#[doc = "EXLVD Comparator Control Register"]
pub type Exlvdcmpcr = crate::RegValueT<Exlvdcmpcr_SPEC>;

impl Exlvdcmpcr {
    #[doc = "EXLVD Pin Low Voltage Detect Circuit Output Enable"]
    #[inline(always)]
    pub fn cmpe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        exlvdcmpcr::Cmpe,
        exlvdcmpcr::Cmpe,
        Exlvdcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            exlvdcmpcr::Cmpe,
            exlvdcmpcr::Cmpe,
            Exlvdcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Exlvdcmpcr {
    #[inline(always)]
    fn default() -> Exlvdcmpcr {
        <crate::RegValueT<Exlvdcmpcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod exlvdcmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpe_SPEC;
    pub type Cmpe = crate::EnumBitfieldStruct<u8, Cmpe_SPEC>;
    impl Cmpe {
        #[doc = "EXLVD pin low voltage detect circuit output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "EXLVD pin low voltage detect circuit output enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exlvdicr_SPEC;
impl crate::sealed::RegSpec for Exlvdicr_SPEC {
    type DataType = u8;
}

#[doc = "EXLVD Pin Low Voltage Detect Interrupt Control Register"]
pub type Exlvdicr = crate::RegValueT<Exlvdicr_SPEC>;

impl Exlvdicr {
    #[doc = "EXLVD Pin Low Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn ie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        exlvdicr::Ie,
        exlvdicr::Ie,
        Exlvdicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            exlvdicr::Ie,
            exlvdicr::Ie,
            Exlvdicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "EXLVD Pin Low Voltage Detect Interrupt Generation Condition Select"]
    #[inline(always)]
    pub fn idtsel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        exlvdicr::Idtsel,
        exlvdicr::Idtsel,
        Exlvdicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            exlvdicr::Idtsel,
            exlvdicr::Idtsel,
            Exlvdicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Exlvdicr {
    #[inline(always)]
    fn default() -> Exlvdicr {
        <crate::RegValueT<Exlvdicr_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod exlvdicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ie_SPEC;
    pub type Ie = crate::EnumBitfieldStruct<u8, Ie_SPEC>;
    impl Ie {
        #[doc = "EXLVD pin low voltage detection interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "EXLVD pin low voltage detection interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idtsel_SPEC;
    pub type Idtsel = crate::EnumBitfieldStruct<u8, Idtsel_SPEC>;
    impl Idtsel {
        #[doc = "When EXLVD > Vdet_exlvd (rise) is detected"]
        pub const _00: Self = Self::new(0);

        #[doc = "When EXLVD < Vdet_exlvd (fall) is detected"]
        pub const _01: Self = Self::new(1);

        #[doc = "When fall and rise are detected"]
        pub const _10: Self = Self::new(2);

        #[doc = "Settings prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
