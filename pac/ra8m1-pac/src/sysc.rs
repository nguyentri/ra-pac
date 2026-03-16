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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:57:20 +0000

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

    #[doc = "Software Standby Control Register 2"]
    #[inline(always)]
    pub const fn sscr2(&self) -> &'static crate::common::Reg<self::Sscr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sscr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Flash Standby Control Register"]
    #[inline(always)]
    pub const fn flscr(&self) -> &'static crate::common::Reg<self::Flscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Flscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
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

    #[doc = "System Clock Division Control Register 2"]
    #[inline(always)]
    pub const fn sckdivcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Sckdivcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sckdivcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
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

    #[doc = "External Bus Clock Control Register"]
    #[inline(always)]
    pub const fn bckcr(&self) -> &'static crate::common::Reg<self::Bckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
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

    #[doc = "Oscillator Monitor Register"]
    #[inline(always)]
    pub const fn oscmonr(
        &self,
    ) -> &'static crate::common::Reg<self::Oscmonr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Oscmonr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(67usize),
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

    #[doc = "PLL Clock Control Register 2"]
    #[inline(always)]
    pub const fn pllccr2(
        &self,
    ) -> &'static crate::common::Reg<self::Pllccr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pllccr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "PLL2 Clock Control Register 2"]
    #[inline(always)]
    pub const fn pll2ccr2(
        &self,
    ) -> &'static crate::common::Reg<self::Pll2Ccr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pll2Ccr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(78usize),
            )
        }
    }

    #[doc = "External Bus Clock Output Control Register"]
    #[inline(always)]
    pub const fn ebckocr(
        &self,
    ) -> &'static crate::common::Reg<self::Ebckocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ebckocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(82usize),
            )
        }
    }

    #[doc = "SDRAM Clock Output Control Register"]
    #[inline(always)]
    pub const fn sdckocr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdckocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdckocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(83usize),
            )
        }
    }

    #[doc = "SCI clock Division control register"]
    #[inline(always)]
    pub const fn scickdivcr(
        &self,
    ) -> &'static crate::common::Reg<self::Scickdivcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scickdivcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "SCI clock control register"]
    #[inline(always)]
    pub const fn scickcr(
        &self,
    ) -> &'static crate::common::Reg<self::Scickcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scickcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(85usize),
            )
        }
    }

    #[doc = "SPI clock Division control register"]
    #[inline(always)]
    pub const fn spickdivcr(
        &self,
    ) -> &'static crate::common::Reg<self::Spickdivcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spickdivcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(86usize),
            )
        }
    }

    #[doc = "SPI clock control register"]
    #[inline(always)]
    pub const fn spickcr(
        &self,
    ) -> &'static crate::common::Reg<self::Spickcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spickcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(87usize),
            )
        }
    }

    #[doc = "ADC clock Division control register"]
    #[inline(always)]
    pub const fn adcckdivcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcckdivcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcckdivcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(90usize),
            )
        }
    }

    #[doc = "ADC clock control register"]
    #[inline(always)]
    pub const fn adcckcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(91usize),
            )
        }
    }

    #[doc = "GPT clock Division control register"]
    #[inline(always)]
    pub const fn gptckdivcr(
        &self,
    ) -> &'static crate::common::Reg<self::Gptckdivcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gptckdivcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "GPT clock control register"]
    #[inline(always)]
    pub const fn gptckcr(
        &self,
    ) -> &'static crate::common::Reg<self::Gptckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gptckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(93usize),
            )
        }
    }

    #[doc = "LCD clock Division control register"]
    #[inline(always)]
    pub const fn lcdckdivcr(
        &self,
    ) -> &'static crate::common::Reg<self::Lcdckdivcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcdckdivcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(94usize),
            )
        }
    }

    #[doc = "LCD clock control register"]
    #[inline(always)]
    pub const fn lcdckcr(
        &self,
    ) -> &'static crate::common::Reg<self::Lcdckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcdckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(95usize),
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

    #[doc = "USB clock Division control register"]
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

    #[doc = "Octal-SPI clock Division control register"]
    #[inline(always)]
    pub const fn octackdivcr(
        &self,
    ) -> &'static crate::common::Reg<self::Octackdivcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Octackdivcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(109usize),
            )
        }
    }

    #[doc = "CANFD Core clock Division control register"]
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

    #[doc = "USB60 clock Division control register"]
    #[inline(always)]
    pub const fn usb60ckdivcr(
        &self,
    ) -> &'static crate::common::Reg<self::Usb60Ckdivcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Usb60Ckdivcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(111usize),
            )
        }
    }

    #[doc = "I3C clock Division control register"]
    #[inline(always)]
    pub const fn i3cckdivcr(
        &self,
    ) -> &'static crate::common::Reg<self::I3Cckdivcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3Cckdivcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "USB clock control register"]
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

    #[doc = "Octal-SPI clock control register"]
    #[inline(always)]
    pub const fn octackcr(
        &self,
    ) -> &'static crate::common::Reg<self::Octackcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Octackcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(117usize),
            )
        }
    }

    #[doc = "CANFD Core clock control register"]
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

    #[doc = "USB60 clock control register"]
    #[inline(always)]
    pub const fn usb60ckcr(
        &self,
    ) -> &'static crate::common::Reg<self::Usb60Ckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Usb60Ckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(119usize),
            )
        }
    }

    #[doc = "I3C clock control register"]
    #[inline(always)]
    pub const fn i3cckcr(
        &self,
    ) -> &'static crate::common::Reg<self::I3Cckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3Cckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "Main Clock Oscillator Standby Control Register"]
    #[inline(always)]
    pub const fn moscscr(
        &self,
    ) -> &'static crate::common::Reg<self::Moscscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Moscscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = "High-Speed On-Chip Oscillator Standby Control Register"]
    #[inline(always)]
    pub const fn hocoscr(
        &self,
    ) -> &'static crate::common::Reg<self::Hocoscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hocoscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(125usize),
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

    #[doc = "System Register Access Control Register"]
    #[inline(always)]
    pub const fn syraccr(
        &self,
    ) -> &'static crate::common::Reg<self::Syraccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syraccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[doc = "Voltage Monitor %s Circuit Control Register 1"]
    #[inline(always)]
    pub const fn pvdcr1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pvdcr1_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe0usize))
        }
    }
    #[inline(always)]
    pub const fn pvd1cr1(
        &self,
    ) -> &'static crate::common::Reg<self::Pvdcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pvdcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pvd2cr1(
        &self,
    ) -> &'static crate::common::Reg<self::Pvdcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pvdcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe2usize),
            )
        }
    }

    #[doc = "Voltage Monitor %s Circuit Status Register"]
    #[inline(always)]
    pub const fn pvdsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pvdsr_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe1usize))
        }
    }
    #[inline(always)]
    pub const fn pvd1sr(&self) -> &'static crate::common::Reg<self::Pvdsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pvdsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pvd2sr(&self) -> &'static crate::common::Reg<self::Pvdsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pvdsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe3usize),
            )
        }
    }

    #[doc = "Clock Recovery System Control Register"]
    #[inline(always)]
    pub const fn crvsyscr(
        &self,
    ) -> &'static crate::common::Reg<self::Crvsyscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crvsyscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[doc = "Graphics Power Domain Control Register"]
    #[inline(always)]
    pub const fn pdctrgd(
        &self,
    ) -> &'static crate::common::Reg<self::Pdctrgd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdctrgd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "SRAM power domain Standby Control Register 0"]
    #[inline(always)]
    pub const fn pdramscr0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdramscr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdramscr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[doc = "SRAM power domain Standby Control Register 1"]
    #[inline(always)]
    pub const fn pdramscr1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdramscr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdramscr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(322usize),
            )
        }
    }

    #[doc = "VBATT Backup Register Security Attribute Boundary Address Register"]
    #[inline(always)]
    pub const fn vbrsabar(
        &self,
    ) -> &'static crate::common::Reg<self::Vbrsabar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbrsabar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(944usize),
            )
        }
    }

    #[doc = "VBATT Backup Register Privilege Attribute Boundary Address Register for Secure Region"]
    #[inline(always)]
    pub const fn vbrpabars(
        &self,
    ) -> &'static crate::common::Reg<self::Vbrpabars_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbrpabars_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(948usize),
            )
        }
    }

    #[doc = "VBATT Backup Register Privilege Attribute Boundary Address Register for Non-secure Region"]
    #[inline(always)]
    pub const fn vbrpabarns(
        &self,
    ) -> &'static crate::common::Reg<self::Vbrpabarns_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbrpabarns_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(952usize),
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

    #[doc = "Programable Voltage Detection Security Attribution Register"]
    #[inline(always)]
    pub const fn pvdsar(
        &self,
    ) -> &'static crate::common::Reg<self::Pvdsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pvdsar_SPEC, crate::common::RW>::from_ptr(
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

    #[doc = "Power Gating Control Security Attribution Register"]
    #[inline(always)]
    pub const fn pgcsar(
        &self,
    ) -> &'static crate::common::Reg<self::Pgcsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pgcsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(984usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Factor Security Attribution Register"]
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

    #[doc = "RAM Standby Control Security Attribution Register"]
    #[inline(always)]
    pub const fn rscsar(
        &self,
    ) -> &'static crate::common::Reg<self::Rscsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rscsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(996usize),
            )
        }
    }

    #[doc = "Protect Register for Secure Register"]
    #[inline(always)]
    pub const fn prcr_s(&self) -> &'static crate::common::Reg<self::PrcrS_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PrcrS_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1018usize),
            )
        }
    }

    #[doc = "Protect Register for Non-secure Register"]
    #[inline(always)]
    pub const fn prcr_ns(
        &self,
    ) -> &'static crate::common::Reg<self::PrcrNs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PrcrNs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1022usize),
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
                self._svd2pac_as_ptr().add(1024usize),
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
                self._svd2pac_as_ptr().add(1026usize),
            )
        }
    }

    #[doc = "Deep Standby Control Register"]
    #[inline(always)]
    pub const fn dpsbycr(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsbycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsbycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2560usize),
            )
        }
    }

    #[doc = "Deep Standby Wait Control Register"]
    #[inline(always)]
    pub const fn dpswcr(
        &self,
    ) -> &'static crate::common::Reg<self::Dpswcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpswcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2564usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn dpsier0(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsier0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsier0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2568usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn dpsier1(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsier1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsier1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2572usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Enable Register 2"]
    #[inline(always)]
    pub const fn dpsier2(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsier2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsier2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2576usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Enable Register 3"]
    #[inline(always)]
    pub const fn dpsier3(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsier3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsier3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2580usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Flag Register 0"]
    #[inline(always)]
    pub const fn dpsifr0(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsifr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsifr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2584usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Flag Register 1"]
    #[inline(always)]
    pub const fn dpsifr1(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsifr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsifr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2588usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Flag Register 2"]
    #[inline(always)]
    pub const fn dpsifr2(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsifr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsifr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2592usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Flag Register 3"]
    #[inline(always)]
    pub const fn dpsifr3(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsifr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsifr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2596usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Edge Register 0"]
    #[inline(always)]
    pub const fn dpsiegr0(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsiegr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsiegr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2600usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Edge Register 1"]
    #[inline(always)]
    pub const fn dpsiegr1(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsiegr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsiegr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2604usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Edge Register 2"]
    #[inline(always)]
    pub const fn dpsiegr2(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsiegr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsiegr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2608usize),
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
                self._svd2pac_as_ptr().add(2616usize),
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
                self._svd2pac_as_ptr().add(2624usize),
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
                self._svd2pac_as_ptr().add(2628usize),
            )
        }
    }

    #[doc = "Reset Status Register 3"]
    #[inline(always)]
    pub const fn rstsr3(
        &self,
    ) -> &'static crate::common::Reg<self::Rstsr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstsr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2632usize),
            )
        }
    }

    #[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
    #[inline(always)]
    pub const fn momcr(&self) -> &'static crate::common::Reg<self::Momcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Momcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2640usize),
            )
        }
    }

    #[doc = "Flash Write Erase Protect Register"]
    #[inline(always)]
    pub const fn fwepror(
        &self,
    ) -> &'static crate::common::Reg<self::Fwepror_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwepror_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2644usize),
            )
        }
    }

    #[doc = "Voltage Monitor 1 Comparator Control Register"]
    #[inline(always)]
    pub const fn pvd1cmpcr(
        &self,
    ) -> &'static crate::common::Reg<self::Pvd1Cmpcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pvd1Cmpcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2648usize),
            )
        }
    }

    #[doc = "Voltage Monitor 2 Comparator Control Register"]
    #[inline(always)]
    pub const fn pvd2cmpcr(
        &self,
    ) -> &'static crate::common::Reg<self::Pvd2Cmpcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pvd2Cmpcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2652usize),
            )
        }
    }

    #[doc = "Voltage Monitor %s Circuit Control Register 0"]
    #[inline(always)]
    pub const fn pvdcr0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pvdcr0_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa70usize))
        }
    }
    #[inline(always)]
    pub const fn pvd1cr0(
        &self,
    ) -> &'static crate::common::Reg<self::Pvdcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pvdcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pvd2cr0(
        &self,
    ) -> &'static crate::common::Reg<self::Pvdcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pvdcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa74usize),
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
                self._svd2pac_as_ptr().add(2692usize),
            )
        }
    }

    #[doc = "VBATT Battery Power Supply Control Register 1"]
    #[inline(always)]
    pub const fn vbtbpcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbpcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbpcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2696usize),
            )
        }
    }

    #[doc = "Low Power State Control Register"]
    #[inline(always)]
    pub const fn lpscr(&self) -> &'static crate::common::Reg<self::Lpscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lpscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2704usize),
            )
        }
    }

    #[doc = "Software Standby Control Register 1"]
    #[inline(always)]
    pub const fn sscr1(&self) -> &'static crate::common::Reg<self::Sscr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sscr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2712usize),
            )
        }
    }

    #[doc = "Low Power State Control Register"]
    #[inline(always)]
    pub const fn lvocr(&self) -> &'static crate::common::Reg<self::Lvocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2736usize),
            )
        }
    }

    #[doc = "System Reset Mask Control Register0"]
    #[inline(always)]
    pub const fn syrstmsk0(
        &self,
    ) -> &'static crate::common::Reg<self::Syrstmsk0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syrstmsk0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2768usize),
            )
        }
    }

    #[doc = "System Reset Mask Control Register1"]
    #[inline(always)]
    pub const fn syrstmsk1(
        &self,
    ) -> &'static crate::common::Reg<self::Syrstmsk1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syrstmsk1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2772usize),
            )
        }
    }

    #[doc = "System Reset Mask Control Register2"]
    #[inline(always)]
    pub const fn syrstmsk2(
        &self,
    ) -> &'static crate::common::Reg<self::Syrstmsk2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syrstmsk2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2776usize),
            )
        }
    }

    #[doc = "PLL1-LDO Control Register"]
    #[inline(always)]
    pub const fn pll1ldocr(
        &self,
    ) -> &'static crate::common::Reg<self::Pll1Ldocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pll1Ldocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2820usize),
            )
        }
    }

    #[doc = "PLL2-LDO Control Register"]
    #[inline(always)]
    pub const fn pll2ldocr(
        &self,
    ) -> &'static crate::common::Reg<self::Pll2Ldocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pll2Ldocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2824usize),
            )
        }
    }

    #[doc = "HOCO-LDO Control Register"]
    #[inline(always)]
    pub const fn hocoldocr(
        &self,
    ) -> &'static crate::common::Reg<self::Hocoldocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hocoldocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2828usize),
            )
        }
    }

    #[doc = "Main Clock Oscillator Mode Control Register 2"]
    #[inline(always)]
    pub const fn momcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Momcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Momcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2832usize),
            )
        }
    }

    #[doc = "Sub-clock oscillator control register"]
    #[inline(always)]
    pub const fn sosccr(
        &self,
    ) -> &'static crate::common::Reg<self::Sosccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sosccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3072usize),
            )
        }
    }

    #[doc = "Sub Clock Oscillator Mode Control Register"]
    #[inline(always)]
    pub const fn somcr(&self) -> &'static crate::common::Reg<self::Somcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Somcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3073usize),
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
                self._svd2pac_as_ptr().add(3136usize),
            )
        }
    }

    #[doc = "VBATT Battery Power Supply Control Register 2"]
    #[inline(always)]
    pub const fn vbtbpcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbpcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbpcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3141usize),
            )
        }
    }

    #[doc = "VBATT Battery Power Supply Status Register"]
    #[inline(always)]
    pub const fn vbtbpsr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbpsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbpsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3142usize),
            )
        }
    }

    #[doc = "VBATT Tamper detection Status Register"]
    #[inline(always)]
    pub const fn vbtadsr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtadsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtadsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3144usize),
            )
        }
    }

    #[doc = "VBATT Tamper detection Control  Register 1"]
    #[inline(always)]
    pub const fn vbtadcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtadcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtadcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3145usize),
            )
        }
    }

    #[doc = "VBATT Tamper detection Control  Register 2"]
    #[inline(always)]
    pub const fn vbtadcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtadcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtadcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3146usize),
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
                self._svd2pac_as_ptr().add(3148usize),
            )
        }
    }

    #[doc = "VBATT Input Control Register 2"]
    #[inline(always)]
    pub const fn vbtictlr2(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtictlr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtictlr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3149usize),
            )
        }
    }

    #[doc = "VBATT Input Monitor Register"]
    #[inline(always)]
    pub const fn vbtimonr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtimonr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtimonr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3150usize),
            )
        }
    }

    #[doc = "VBATT Backup Register %s"]
    #[inline(always)]
    pub const fn vbtbkr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW>,
        128,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xd00usize))
        }
    }
    #[inline(always)]
    pub const fn vbtbkr0(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr1(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd01usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr2(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd02usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr3(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd03usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr4(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr5(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd05usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr6(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd06usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr7(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd07usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr8(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr9(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd09usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr10(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr11(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr12(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr13(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr14(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr15(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr16(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr17(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd11usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr18(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd12usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr19(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd13usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr20(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr21(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd15usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr22(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd16usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr23(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd17usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr24(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr25(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd19usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr26(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd1ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr27(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd1busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr28(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr29(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd1dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr30(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd1eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr31(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd1fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr32(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr33(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd21usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr34(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd22usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr35(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd23usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr36(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr37(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd25usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr38(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd26usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr39(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd27usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr40(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr41(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd29usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr42(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd2ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr43(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd2busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr44(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr45(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd2dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr46(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd2eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr47(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd2fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr48(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr49(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd31usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr50(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd32usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr51(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd33usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr52(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr53(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd35usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr54(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd36usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr55(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd37usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr56(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr57(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd39usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr58(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd3ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr59(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd3busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr60(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr61(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd3dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr62(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd3eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr63(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd3fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr64(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr65(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd41usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr66(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd42usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr67(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd43usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr68(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr69(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd45usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr70(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd46usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr71(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd47usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr72(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr73(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd49usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr74(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr75(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr76(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr77(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr78(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr79(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr80(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr81(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd51usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr82(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd52usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr83(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd53usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr84(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr85(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd55usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr86(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd56usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr87(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd57usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr88(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr89(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd59usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr90(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd5ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr91(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd5busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr92(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr93(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd5dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr94(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd5eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr95(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd5fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr96(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr97(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd61usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr98(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd62usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr99(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd63usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr100(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr101(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd65usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr102(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd66usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr103(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd67usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr104(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr105(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd69usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr106(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd6ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr107(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd6busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr108(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr109(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd6dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr110(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd6eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr111(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd6fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr112(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr113(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd71usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr114(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd72usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr115(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd73usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr116(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr117(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd75usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr118(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd76usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr119(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd77usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr120(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr121(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd79usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr122(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd7ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr123(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd7busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr124(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr125(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd7dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr126(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd7eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr127(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd7fusize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbycr_SPEC;
impl crate::sealed::RegSpec for Sbycr_SPEC {
    type DataType = u8;
}

#[doc = "Standby Control Register"]
pub type Sbycr = crate::RegValueT<Sbycr_SPEC>;

impl Sbycr {
    #[doc = "Output Port Enable"]
    #[inline(always)]
    pub fn ope(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sbycr::Ope,
        sbycr::Ope,
        Sbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sbycr::Ope,
            sbycr::Ope,
            Sbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Sbycr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Sbycr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Sbycr {
    #[inline(always)]
    fn default() -> Sbycr {
        <crate::RegValueT<Sbycr_SPEC> as RegisterValue<_>>::new(64)
    }
}
pub mod sbycr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ope_SPEC;
    pub type Ope = crate::EnumBitfieldStruct<u8, Ope_SPEC>;
    impl Ope {
        #[doc = "In software standby mode or deep software standby mode, the address bus and bus control signals are set to the high-impedance state."]
        pub const _0: Self = Self::new(0);

        #[doc = "In software standby mode or deep software standby mode, the address bus and bus control signals retain the output state.."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sscr2_SPEC;
impl crate::sealed::RegSpec for Sscr2_SPEC {
    type DataType = u8;
}

#[doc = "Software Standby Control Register 2"]
pub type Sscr2 = crate::RegValueT<Sscr2_SPEC>;

impl Sscr2 {
    #[doc = "Software Standby 1 regulator status flag"]
    #[inline(always)]
    pub fn ss1rsf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sscr2::Ss1Rsf,
        sscr2::Ss1Rsf,
        Sscr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sscr2::Ss1Rsf,
            sscr2::Ss1Rsf,
            Sscr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sscr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sscr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sscr2 {
    #[inline(always)]
    fn default() -> Sscr2 {
        <crate::RegValueT<Sscr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sscr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ss1Rsf_SPEC;
    pub type Ss1Rsf = crate::EnumBitfieldStruct<u8, Ss1Rsf_SPEC>;
    impl Ss1Rsf {
        #[doc = "After returning from Software Standby mode 1, the regulator is fully stabilized ."]
        pub const _0: Self = Self::new(0);

        #[doc = "After returning from Software Standby mode 1, the regulator is not fully stabilized ."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flscr_SPEC;
impl crate::sealed::RegSpec for Flscr_SPEC {
    type DataType = u8;
}

#[doc = "Flash Standby Control Register"]
pub type Flscr = crate::RegValueT<Flscr_SPEC>;

impl Flscr {
    #[doc = "Flash Stabilization wait completion flag"]
    #[inline(always)]
    pub fn flswcf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        flscr::Flswcf,
        flscr::Flswcf,
        Flscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            flscr::Flswcf,
            flscr::Flswcf,
            Flscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Flscr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Flscr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Flscr {
    #[inline(always)]
    fn default() -> Flscr {
        <crate::RegValueT<Flscr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod flscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flswcf_SPEC;
    pub type Flswcf = crate::EnumBitfieldStruct<u8, Flswcf_SPEC>;
    impl Flswcf {
        #[doc = "Flash stabilization wait time is not completed when returning to Normal mode from Software standby mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flash stabilization wait time is completed when returning to Normal mode from Software standby mode."]
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
        0xf,
        1,
        0,
        sckdivcr::Pckd,
        sckdivcr::Pckd,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
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
        0xf,
        1,
        0,
        sckdivcr::Pckc,
        sckdivcr::Pckc,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
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
        0xf,
        1,
        0,
        sckdivcr::Pckb,
        sckdivcr::Pckb,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
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
        0xf,
        1,
        0,
        sckdivcr::Pcka,
        sckdivcr::Pcka,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            sckdivcr::Pcka,
            sckdivcr::Pcka,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "External Bus Clock (BCLK) Select"]
    #[inline(always)]
    pub fn bck(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        sckdivcr::Bck,
        sckdivcr::Bck,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            sckdivcr::Bck,
            sckdivcr::Bck,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Module Clock E (PCLKE) Select"]
    #[inline(always)]
    pub fn pcke(
        self,
    ) -> crate::common::RegisterField<
        20,
        0xf,
        1,
        0,
        sckdivcr::Pcke,
        sckdivcr::Pcke,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0xf,
            1,
            0,
            sckdivcr::Pcke,
            sckdivcr::Pcke,
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
        0xf,
        1,
        0,
        sckdivcr::Ick,
        sckdivcr::Ick,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            sckdivcr::Ick,
            sckdivcr::Ick,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Flash IF Clock (FCLK) Select"]
    #[inline(always)]
    pub fn fck(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        sckdivcr::Fck,
        sckdivcr::Fck,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
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
        <crate::RegValueT<Sckdivcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sckdivcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pckd_SPEC;
    pub type Pckd = crate::EnumBitfieldStruct<u8, Pckd_SPEC>;
    impl Pckd {
        #[doc = "/1 (value after reset)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _0010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _0011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _0100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _0101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _0110: Self = Self::new(6);

        #[doc = "/3"]
        pub const _1000: Self = Self::new(8);

        #[doc = "/6"]
        pub const _1001: Self = Self::new(9);

        #[doc = "/12"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pckc_SPEC;
    pub type Pckc = crate::EnumBitfieldStruct<u8, Pckc_SPEC>;
    impl Pckc {
        #[doc = "/1 (value after reset)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _0010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _0011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _0100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _0101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _0110: Self = Self::new(6);

        #[doc = "/3"]
        pub const _1000: Self = Self::new(8);

        #[doc = "/6"]
        pub const _1001: Self = Self::new(9);

        #[doc = "/12"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pckb_SPEC;
    pub type Pckb = crate::EnumBitfieldStruct<u8, Pckb_SPEC>;
    impl Pckb {
        #[doc = "/1 (value after reset)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _0010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _0011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _0100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _0101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _0110: Self = Self::new(6);

        #[doc = "/3"]
        pub const _1000: Self = Self::new(8);

        #[doc = "/6"]
        pub const _1001: Self = Self::new(9);

        #[doc = "/12"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcka_SPEC;
    pub type Pcka = crate::EnumBitfieldStruct<u8, Pcka_SPEC>;
    impl Pcka {
        #[doc = "/1 (value after reset)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _0010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _0011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _0100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _0101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _0110: Self = Self::new(6);

        #[doc = "/3"]
        pub const _1000: Self = Self::new(8);

        #[doc = "/6"]
        pub const _1001: Self = Self::new(9);

        #[doc = "/12"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bck_SPEC;
    pub type Bck = crate::EnumBitfieldStruct<u8, Bck_SPEC>;
    impl Bck {
        #[doc = "/1 (value after reset)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _0010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _0011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _0100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _0101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _0110: Self = Self::new(6);

        #[doc = "/3"]
        pub const _1000: Self = Self::new(8);

        #[doc = "/6"]
        pub const _1001: Self = Self::new(9);

        #[doc = "/12"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcke_SPEC;
    pub type Pcke = crate::EnumBitfieldStruct<u8, Pcke_SPEC>;
    impl Pcke {
        #[doc = "/1 (value after reset)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _0010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _0011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _0100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _0101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _0110: Self = Self::new(6);

        #[doc = "/3"]
        pub const _1000: Self = Self::new(8);

        #[doc = "/6"]
        pub const _1001: Self = Self::new(9);

        #[doc = "/12"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ick_SPEC;
    pub type Ick = crate::EnumBitfieldStruct<u8, Ick_SPEC>;
    impl Ick {
        #[doc = "/1 (value after reset)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _0010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _0011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _0100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _0101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _0110: Self = Self::new(6);

        #[doc = "/3"]
        pub const _1000: Self = Self::new(8);

        #[doc = "/6"]
        pub const _1001: Self = Self::new(9);

        #[doc = "/12"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fck_SPEC;
    pub type Fck = crate::EnumBitfieldStruct<u8, Fck_SPEC>;
    impl Fck {
        #[doc = "/1 (value after reset)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _0010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _0011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _0100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _0101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _0110: Self = Self::new(6);

        #[doc = "/3"]
        pub const _1000: Self = Self::new(8);

        #[doc = "/6"]
        pub const _1001: Self = Self::new(9);

        #[doc = "/12"]
        pub const _1010: Self = Self::new(10);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sckdivcr2_SPEC;
impl crate::sealed::RegSpec for Sckdivcr2_SPEC {
    type DataType = u8;
}

#[doc = "System Clock Division Control Register 2"]
pub type Sckdivcr2 = crate::RegValueT<Sckdivcr2_SPEC>;

impl Sckdivcr2 {
    #[doc = "CPU Clock (CPUCLK) Select"]
    #[inline(always)]
    pub fn cpuck(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        sckdivcr2::Cpuck,
        sckdivcr2::Cpuck,
        Sckdivcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            sckdivcr2::Cpuck,
            sckdivcr2::Cpuck,
            Sckdivcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Sckdivcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Sckdivcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sckdivcr2 {
    #[inline(always)]
    fn default() -> Sckdivcr2 {
        <crate::RegValueT<Sckdivcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sckdivcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpuck_SPEC;
    pub type Cpuck = crate::EnumBitfieldStruct<u8, Cpuck_SPEC>;
    impl Cpuck {
        #[doc = "/1 (value after reset)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _0010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _0011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _0100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _0101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _0110: Self = Self::new(6);

        #[doc = "/3"]
        pub const _1000: Self = Self::new(8);

        #[doc = "/6"]
        pub const _1001: Self = Self::new(9);

        #[doc = "/12"]
        pub const _1010: Self = Self::new(10);
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

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Sckscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Sckscr_SPEC,crate::common::RW>::from_register(self,0)
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

        #[doc = "Main clock oscillator"]
        pub const _011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator"]
        pub const _100: Self = Self::new(4);

        #[doc = "PLL"]
        pub const _101: Self = Self::new(5);
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
    #[doc = "PLL1 Input Frequency Division Ratio Select"]
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

    #[doc = "PLL1 Clock Source Select"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Pllccr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pllccr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PLL1 Frequency Multiplication Fractional Factor Select"]
    #[inline(always)]
    pub fn pllmulnf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        pllccr::Pllmulnf,
        pllccr::Pllmulnf,
        Pllccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            pllccr::Pllmulnf,
            pllccr::Pllmulnf,
            Pllccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PLL1 Frequency Multiplication Factor Select"]
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

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllmulnf_SPEC;
    pub type Pllmulnf = crate::EnumBitfieldStruct<u8, Pllmulnf_SPEC>;
    impl Pllmulnf {
        #[doc = "0.00 (Value after reset)"]
        pub const _00: Self = Self::new(0);

        #[doc = "0.33 (1/3)"]
        pub const _01: Self = Self::new(1);

        #[doc = "0.66 (2/3)"]
        pub const _10: Self = Self::new(2);

        #[doc = "0.50 (1/2)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllmul_SPEC;
    pub type Pllmul = crate::EnumBitfieldStruct<u8, Pllmul_SPEC>;
    impl Pllmul {
        #[doc = "×26 (Value after reset)"]
        pub const _00011001: Self = Self::new(25);

        #[doc = "×27"]
        pub const _00011010: Self = Self::new(26);

        #[doc = "×28"]
        pub const _00011011: Self = Self::new(27);

        #[doc = "×89"]
        pub const _01011000: Self = Self::new(88);

        #[doc = "×90"]
        pub const _01011001: Self = Self::new(89);

        #[doc = "×91"]
        pub const _01011010: Self = Self::new(90);

        #[doc = "×179"]
        pub const _10110010: Self = Self::new(178);

        #[doc = "×180"]
        pub const _10110011: Self = Self::new(179);
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
    #[doc = "PLL1 Stop Control"]
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Pllcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Pllcr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Operate the PLL1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the PLL1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bckcr_SPEC;
impl crate::sealed::RegSpec for Bckcr_SPEC {
    type DataType = u8;
}

#[doc = "External Bus Clock Control Register"]
pub type Bckcr = crate::RegValueT<Bckcr_SPEC>;

impl Bckcr {
    #[doc = "BCLK Pin Output Select"]
    #[inline(always)]
    pub fn bclkdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bckcr::Bclkdiv,
        bckcr::Bclkdiv,
        Bckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bckcr::Bclkdiv,
            bckcr::Bclkdiv,
            Bckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Bckcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Bckcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bckcr {
    #[inline(always)]
    fn default() -> Bckcr {
        <crate::RegValueT<Bckcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bclkdiv_SPEC;
    pub type Bclkdiv = crate::EnumBitfieldStruct<u8, Bclkdiv_SPEC>;
    impl Bclkdiv {
        #[doc = "BCLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "BCLK/2"]
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Mosccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Mosccr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Main clock oscillator is operating."]
        pub const _0: Self = Self::new(0);

        #[doc = "Main clock oscillator is stopped."]
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Hococr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Hococr_SPEC,crate::common::RW>::from_register(self,0)
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Mococr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Mococr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Operate the MOCO clock"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the MOCO clock"]
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Fllcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Fllcr1_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "FLL function is disabled."]
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

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, u8, Fllcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1f,1,0,u8,u8,Fllcr2_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "HOCO Clock Oscillation Stabilization FlagNOTE: The HOCOSF bit value after a reset is 1 when the OFS1.HOCOEN bit is 0. It is 0 when the OFS1.HOCOEN bit is 1."]
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

    #[doc = "PLL1 Clock Oscillation Stabilization Flag"]
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

    #[doc = "This bit is read as 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Oscsf_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Oscsf_SPEC, crate::common::R>::from_register(
            self, 0,
        )
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
        #[doc = "HOCO clock is stopped or is not yet stable"]
        pub const _0: Self = Self::new(0);

        #[doc = "HOCO clock is stable, so is available for use as the system clock"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moscsf_SPEC;
    pub type Moscsf = crate::EnumBitfieldStruct<u8, Moscsf_SPEC>;
    impl Moscsf {
        #[doc = "Main clock oscillator is stopped (MOSTP = 1) or is not yet stable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Main clock oscillator is stable, so is available for use as the system clock"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllsf_SPEC;
    pub type Pllsf = crate::EnumBitfieldStruct<u8, Pllsf_SPEC>;
    impl Pllsf {
        #[doc = "The PLL1 clock is stopped or oscillation of the PLL1 clock has not yet become stable."]
        pub const _0: Self = Self::new(0);

        #[doc = "Oscillation of the PLL1 clock is stable so the clock is available for use as the system clock."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll2Sf_SPEC;
    pub type Pll2Sf = crate::EnumBitfieldStruct<u8, Pll2Sf_SPEC>;
    impl Pll2Sf {
        #[doc = "The PLL2 clock is stopped or oscillation of the PLL2 clock has not yet become stable."]
        pub const _0: Self = Self::new(0);

        #[doc = "Oscillation of the PLL2 clock is stable so the clock is available for use as the system clock."]
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
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ckocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ckocr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock out input frequency Division Select"]
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

    #[doc = "Clock out enable"]
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
    pub struct Ckodiv_SPEC;
    pub type Ckodiv = crate::EnumBitfieldStruct<u8, Ckodiv_SPEC>;
    impl Ckodiv {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "/128"]
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
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Trckcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Trckcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Trace Clock source select"]
    #[inline(always)]
    pub fn trcksel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        trckcr::Trcksel,
        trckcr::Trcksel,
        Trckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            trckcr::Trcksel,
            trckcr::Trcksel,
            Trckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, Trckcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,Trckcr_SPEC,crate::common::RW>::from_register(self,0)
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
        <crate::RegValueT<Trckcr_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod trckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trcksel_SPEC;
    pub type Trcksel = crate::EnumBitfieldStruct<u8, Trcksel_SPEC>;
    impl Trcksel {
        #[doc = "System clock source (value after reset)"]
        pub const _0: Self = Self::new(0);

        #[doc = "HOCO (oscillation in debug mode)"]
        pub const _1: Self = Self::new(1);
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

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, Ostdcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,Ostdcr_SPEC,crate::common::RW>::from_register(self,0)
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Ostdsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Ostdsr_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Oscmonr_SPEC;
impl crate::sealed::RegSpec for Oscmonr_SPEC {
    type DataType = u8;
}

#[doc = "Oscillator Monitor Register"]
pub type Oscmonr = crate::RegValueT<Oscmonr_SPEC>;

impl Oscmonr {
    #[doc = "MOCO operation monitor"]
    #[inline(always)]
    pub fn mocomon(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        oscmonr::Mocomon,
        oscmonr::Mocomon,
        Oscmonr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            oscmonr::Mocomon,
            oscmonr::Mocomon,
            Oscmonr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "LOCO operation monitor"]
    #[inline(always)]
    pub fn locomon(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        oscmonr::Locomon,
        oscmonr::Locomon,
        Oscmonr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            oscmonr::Locomon,
            oscmonr::Locomon,
            Oscmonr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Oscmonr_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Oscmonr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Oscmonr {
    #[inline(always)]
    fn default() -> Oscmonr {
        <crate::RegValueT<Oscmonr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod oscmonr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mocomon_SPEC;
    pub type Mocomon = crate::EnumBitfieldStruct<u8, Mocomon_SPEC>;
    impl Mocomon {
        #[doc = "MOCO is set to operate."]
        pub const _0: Self = Self::new(0);

        #[doc = "MOCO is set to stop."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Locomon_SPEC;
    pub type Locomon = crate::EnumBitfieldStruct<u8, Locomon_SPEC>;
    impl Locomon {
        #[doc = "LOCO is set to operate."]
        pub const _0: Self = Self::new(0);

        #[doc = "LOCO is set to stop"]
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

    #[doc = "PLL Clock Source Select"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Pll2Ccr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pll2Ccr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PLL2 Frequency Multiplication Fractional Factor Select"]
    #[inline(always)]
    pub fn pll2mulnf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        pll2ccr::Pll2Mulnf,
        pll2ccr::Pll2Mulnf,
        Pll2Ccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            pll2ccr::Pll2Mulnf,
            pll2ccr::Pll2Mulnf,
            Pll2Ccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PLL2 Frequency Multiplication Factor Select"]
    #[inline(always)]
    pub fn pll2mul(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        pll2ccr::Pll2Mul,
        pll2ccr::Pll2Mul,
        Pll2Ccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            pll2ccr::Pll2Mul,
            pll2ccr::Pll2Mul,
            Pll2Ccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pll2Ccr {
    #[inline(always)]
    fn default() -> Pll2Ccr {
        <crate::RegValueT<Pll2Ccr_SPEC> as RegisterValue<_>>::new(6400)
    }
}
pub mod pll2ccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pl2Idiv_SPEC;
    pub type Pl2Idiv = crate::EnumBitfieldStruct<u8, Pl2Idiv_SPEC>;
    impl Pl2Idiv {
        #[doc = "/1 (Value after reset)"]
        pub const _00: Self = Self::new(0);

        #[doc = "/2"]
        pub const _01: Self = Self::new(1);

        #[doc = "/3"]
        pub const _10: Self = Self::new(2);

        #[doc = "/4"]
        pub const _11: Self = Self::new(3);
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll2Mulnf_SPEC;
    pub type Pll2Mulnf = crate::EnumBitfieldStruct<u8, Pll2Mulnf_SPEC>;
    impl Pll2Mulnf {
        #[doc = "0.00 (Value after reset)"]
        pub const _00: Self = Self::new(0);

        #[doc = "0.33 (1/3)"]
        pub const _01: Self = Self::new(1);

        #[doc = "0.66 (2/3)"]
        pub const _10: Self = Self::new(2);

        #[doc = "0.50 (1/2)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll2Mul_SPEC;
    pub type Pll2Mul = crate::EnumBitfieldStruct<u8, Pll2Mul_SPEC>;
    impl Pll2Mul {
        #[doc = "×26 (Value after reset)"]
        pub const _00011001: Self = Self::new(25);

        #[doc = "×27"]
        pub const _00011010: Self = Self::new(26);

        #[doc = "×28"]
        pub const _00011011: Self = Self::new(27);

        #[doc = "×89"]
        pub const _01011000: Self = Self::new(88);

        #[doc = "×90"]
        pub const _01011001: Self = Self::new(89);

        #[doc = "×91"]
        pub const _01011010: Self = Self::new(90);

        #[doc = "×179"]
        pub const _10110010: Self = Self::new(178);

        #[doc = "×180"]
        pub const _10110011: Self = Self::new(179);
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Pll2Cr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Pll2Cr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Operate the PLL2"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the PLL2."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllccr2_SPEC;
impl crate::sealed::RegSpec for Pllccr2_SPEC {
    type DataType = u16;
}

#[doc = "PLL Clock Control Register 2"]
pub type Pllccr2 = crate::RegValueT<Pllccr2_SPEC>;

impl Pllccr2 {
    #[doc = "PLL1 Output Frequency Division Ratio Select for output clock P"]
    #[inline(always)]
    pub fn plodivp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        pllccr2::Plodivp,
        pllccr2::Plodivp,
        Pllccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            pllccr2::Plodivp,
            pllccr2::Plodivp,
            Pllccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PLL1 Output Frequency Division Ratio Select for output clock Q"]
    #[inline(always)]
    pub fn plodivq(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        pllccr2::Plodivq,
        pllccr2::Plodivq,
        Pllccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            pllccr2::Plodivq,
            pllccr2::Plodivq,
            Pllccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PLL1 Output Frequency Division Ratio Select for output clock R"]
    #[inline(always)]
    pub fn plodivr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        pllccr2::Plodivr,
        pllccr2::Plodivr,
        Pllccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            pllccr2::Plodivr,
            pllccr2::Plodivr,
            Pllccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Pllccr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Pllccr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pllccr2 {
    #[inline(always)]
    fn default() -> Pllccr2 {
        <crate::RegValueT<Pllccr2_SPEC> as RegisterValue<_>>::new(1365)
    }
}
pub mod pllccr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plodivp_SPEC;
    pub type Plodivp = crate::EnumBitfieldStruct<u8, Plodivp_SPEC>;
    impl Plodivp {
        #[doc = "×1"]
        pub const _0000: Self = Self::new(0);

        #[doc = "×1 / 2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "prohibited"]
        pub const _0010: Self = Self::new(2);

        #[doc = "×1 / 4"]
        pub const _0011: Self = Self::new(3);

        #[doc = "prohibited"]
        pub const _0100: Self = Self::new(4);

        #[doc = "×1 / 6 (Value after reset)"]
        pub const _0101: Self = Self::new(5);

        #[doc = "prohibited"]
        pub const _0110: Self = Self::new(6);

        #[doc = "×1 / 8"]
        pub const _0111: Self = Self::new(7);

        #[doc = "prohibited"]
        pub const _1000: Self = Self::new(8);

        #[doc = "prohibited"]
        pub const _1001: Self = Self::new(9);

        #[doc = "prohibited"]
        pub const _1010: Self = Self::new(10);

        #[doc = "prohibited"]
        pub const _1011: Self = Self::new(11);

        #[doc = "prohibited"]
        pub const _1100: Self = Self::new(12);

        #[doc = "prohibited"]
        pub const _1101: Self = Self::new(13);

        #[doc = "prohibited"]
        pub const _1110: Self = Self::new(14);

        #[doc = "×1 / 16"]
        pub const _1111: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plodivq_SPEC;
    pub type Plodivq = crate::EnumBitfieldStruct<u8, Plodivq_SPEC>;
    impl Plodivq {
        #[doc = "prohibited"]
        pub const _0000: Self = Self::new(0);

        #[doc = "×1 / 2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "×1 / 3"]
        pub const _0010: Self = Self::new(2);

        #[doc = "×1 / 4"]
        pub const _0011: Self = Self::new(3);

        #[doc = "×1 / 5"]
        pub const _0100: Self = Self::new(4);

        #[doc = "×1 / 6 (Value after reset)"]
        pub const _0101: Self = Self::new(5);

        #[doc = "prohibited"]
        pub const _0110: Self = Self::new(6);

        #[doc = "×1 / 8"]
        pub const _0111: Self = Self::new(7);

        #[doc = "×1 / 9"]
        pub const _1000: Self = Self::new(8);

        #[doc = "prohibited"]
        pub const _1001: Self = Self::new(9);

        #[doc = "prohibited"]
        pub const _1010: Self = Self::new(10);

        #[doc = "prohibited"]
        pub const _1011: Self = Self::new(11);

        #[doc = "prohibited"]
        pub const _1100: Self = Self::new(12);

        #[doc = "prohibited"]
        pub const _1101: Self = Self::new(13);

        #[doc = "prohibited"]
        pub const _1110: Self = Self::new(14);

        #[doc = "prohibited"]
        pub const _1111: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plodivr_SPEC;
    pub type Plodivr = crate::EnumBitfieldStruct<u8, Plodivr_SPEC>;
    impl Plodivr {
        #[doc = "prohibited"]
        pub const _0000: Self = Self::new(0);

        #[doc = "×1 / 2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "×1 / 3"]
        pub const _0010: Self = Self::new(2);

        #[doc = "×1 / 4"]
        pub const _0011: Self = Self::new(3);

        #[doc = "×1 / 5"]
        pub const _0100: Self = Self::new(4);

        #[doc = "×1 / 6 (Value after reset)"]
        pub const _0101: Self = Self::new(5);

        #[doc = "prohibited"]
        pub const _0110: Self = Self::new(6);

        #[doc = "×1 / 8"]
        pub const _0111: Self = Self::new(7);

        #[doc = "×1 / 9"]
        pub const _1000: Self = Self::new(8);

        #[doc = "prohibited"]
        pub const _1001: Self = Self::new(9);

        #[doc = "prohibited"]
        pub const _1010: Self = Self::new(10);

        #[doc = "prohibited"]
        pub const _1011: Self = Self::new(11);

        #[doc = "prohibited"]
        pub const _1100: Self = Self::new(12);

        #[doc = "prohibited"]
        pub const _1101: Self = Self::new(13);

        #[doc = "prohibited"]
        pub const _1110: Self = Self::new(14);

        #[doc = "prohibited"]
        pub const _1111: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll2Ccr2_SPEC;
impl crate::sealed::RegSpec for Pll2Ccr2_SPEC {
    type DataType = u16;
}

#[doc = "PLL2 Clock Control Register 2"]
pub type Pll2Ccr2 = crate::RegValueT<Pll2Ccr2_SPEC>;

impl Pll2Ccr2 {
    #[doc = "PLL2 Output Frequency Division Ratio Select for output clock P"]
    #[inline(always)]
    pub fn pl2odivp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        pll2ccr2::Pl2Odivp,
        pll2ccr2::Pl2Odivp,
        Pll2Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            pll2ccr2::Pl2Odivp,
            pll2ccr2::Pl2Odivp,
            Pll2Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PLL2 Output Frequency Division Ratio Select for output clock Q"]
    #[inline(always)]
    pub fn pl2odivq(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        pll2ccr2::Pl2Odivq,
        pll2ccr2::Pl2Odivq,
        Pll2Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            pll2ccr2::Pl2Odivq,
            pll2ccr2::Pl2Odivq,
            Pll2Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PLL2 Output Frequency Division Ratio Select for output clock R"]
    #[inline(always)]
    pub fn pl2odivr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        pll2ccr2::Pl2Odivr,
        pll2ccr2::Pl2Odivr,
        Pll2Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            pll2ccr2::Pl2Odivr,
            pll2ccr2::Pl2Odivr,
            Pll2Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Pll2Ccr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Pll2Ccr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pll2Ccr2 {
    #[inline(always)]
    fn default() -> Pll2Ccr2 {
        <crate::RegValueT<Pll2Ccr2_SPEC> as RegisterValue<_>>::new(1365)
    }
}
pub mod pll2ccr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pl2Odivp_SPEC;
    pub type Pl2Odivp = crate::EnumBitfieldStruct<u8, Pl2Odivp_SPEC>;
    impl Pl2Odivp {
        #[doc = "×1"]
        pub const _0000: Self = Self::new(0);

        #[doc = "×1 / 2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "prohibited"]
        pub const _0010: Self = Self::new(2);

        #[doc = "×1 / 4"]
        pub const _0011: Self = Self::new(3);

        #[doc = "prohibited"]
        pub const _0100: Self = Self::new(4);

        #[doc = "×1 / 6 (Value after reset)"]
        pub const _0101: Self = Self::new(5);

        #[doc = "prohibited"]
        pub const _0110: Self = Self::new(6);

        #[doc = "×1 / 8"]
        pub const _0111: Self = Self::new(7);

        #[doc = "prohibited"]
        pub const _1000: Self = Self::new(8);

        #[doc = "prohibited"]
        pub const _1001: Self = Self::new(9);

        #[doc = "prohibited"]
        pub const _1010: Self = Self::new(10);

        #[doc = "prohibited"]
        pub const _1011: Self = Self::new(11);

        #[doc = "prohibited"]
        pub const _1100: Self = Self::new(12);

        #[doc = "prohibited"]
        pub const _1101: Self = Self::new(13);

        #[doc = "prohibited"]
        pub const _1110: Self = Self::new(14);

        #[doc = "×1 / 16"]
        pub const _1111: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pl2Odivq_SPEC;
    pub type Pl2Odivq = crate::EnumBitfieldStruct<u8, Pl2Odivq_SPEC>;
    impl Pl2Odivq {
        #[doc = "prohibited"]
        pub const _0000: Self = Self::new(0);

        #[doc = "×1 / 2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "×1 / 3"]
        pub const _0010: Self = Self::new(2);

        #[doc = "×1 / 4"]
        pub const _0011: Self = Self::new(3);

        #[doc = "×1 / 5"]
        pub const _0100: Self = Self::new(4);

        #[doc = "×1 / 6 (Value after reset)"]
        pub const _0101: Self = Self::new(5);

        #[doc = "prohibited"]
        pub const _0110: Self = Self::new(6);

        #[doc = "×1 / 8"]
        pub const _0111: Self = Self::new(7);

        #[doc = "×1 / 9"]
        pub const _1000: Self = Self::new(8);

        #[doc = "prohibited"]
        pub const _1001: Self = Self::new(9);

        #[doc = "prohibited"]
        pub const _1010: Self = Self::new(10);

        #[doc = "prohibited"]
        pub const _1011: Self = Self::new(11);

        #[doc = "prohibited"]
        pub const _1100: Self = Self::new(12);

        #[doc = "prohibited"]
        pub const _1101: Self = Self::new(13);

        #[doc = "prohibited"]
        pub const _1110: Self = Self::new(14);

        #[doc = "prohibited"]
        pub const _1111: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pl2Odivr_SPEC;
    pub type Pl2Odivr = crate::EnumBitfieldStruct<u8, Pl2Odivr_SPEC>;
    impl Pl2Odivr {
        #[doc = "prohibited"]
        pub const _0000: Self = Self::new(0);

        #[doc = "×1 / 2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "×1 / 3"]
        pub const _0010: Self = Self::new(2);

        #[doc = "×1 / 4"]
        pub const _0011: Self = Self::new(3);

        #[doc = "×1 / 5"]
        pub const _0100: Self = Self::new(4);

        #[doc = "×1 / 6 (Value after reset)"]
        pub const _0101: Self = Self::new(5);

        #[doc = "prohibited"]
        pub const _0110: Self = Self::new(6);

        #[doc = "×1 / 8"]
        pub const _0111: Self = Self::new(7);

        #[doc = "×1 / 9"]
        pub const _1000: Self = Self::new(8);

        #[doc = "prohibited"]
        pub const _1001: Self = Self::new(9);

        #[doc = "prohibited"]
        pub const _1010: Self = Self::new(10);

        #[doc = "prohibited"]
        pub const _1011: Self = Self::new(11);

        #[doc = "prohibited"]
        pub const _1100: Self = Self::new(12);

        #[doc = "prohibited"]
        pub const _1101: Self = Self::new(13);

        #[doc = "prohibited"]
        pub const _1110: Self = Self::new(14);

        #[doc = "prohibited"]
        pub const _1111: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ebckocr_SPEC;
impl crate::sealed::RegSpec for Ebckocr_SPEC {
    type DataType = u8;
}

#[doc = "External Bus Clock Output Control Register"]
pub type Ebckocr = crate::RegValueT<Ebckocr_SPEC>;

impl Ebckocr {
    #[doc = "BCLK Pin Output Control"]
    #[inline(always)]
    pub fn ebckoen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ebckocr::Ebckoen,
        ebckocr::Ebckoen,
        Ebckocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ebckocr::Ebckoen,
            ebckocr::Ebckoen,
            Ebckocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Ebckocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Ebckocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ebckocr {
    #[inline(always)]
    fn default() -> Ebckocr {
        <crate::RegValueT<Ebckocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ebckocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ebckoen_SPEC;
    pub type Ebckoen = crate::EnumBitfieldStruct<u8, Ebckoen_SPEC>;
    impl Ebckoen {
        #[doc = "Disable EBCLK pin output (fixed high)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable EBCLK pin output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdckocr_SPEC;
impl crate::sealed::RegSpec for Sdckocr_SPEC {
    type DataType = u8;
}

#[doc = "SDRAM Clock Output Control Register"]
pub type Sdckocr = crate::RegValueT<Sdckocr_SPEC>;

impl Sdckocr {
    #[doc = "SDCLK Pin Output Control"]
    #[inline(always)]
    pub fn sdckoen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdckocr::Sdckoen,
        sdckocr::Sdckoen,
        Sdckocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdckocr::Sdckoen,
            sdckocr::Sdckoen,
            Sdckocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sdckocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sdckocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdckocr {
    #[inline(always)]
    fn default() -> Sdckocr {
        <crate::RegValueT<Sdckocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdckocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdckoen_SPEC;
    pub type Sdckoen = crate::EnumBitfieldStruct<u8, Sdckoen_SPEC>;
    impl Sdckoen {
        #[doc = "Disable SDCLK pin output (fixed high)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable SDCLK pin output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scickdivcr_SPEC;
impl crate::sealed::RegSpec for Scickdivcr_SPEC {
    type DataType = u8;
}

#[doc = "SCI clock Division control register"]
pub type Scickdivcr = crate::RegValueT<Scickdivcr_SPEC>;

impl Scickdivcr {
    #[doc = "Clock Division Select"]
    #[inline(always)]
    pub fn ckdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        scickdivcr::Ckdiv,
        scickdivcr::Ckdiv,
        Scickdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            scickdivcr::Ckdiv,
            scickdivcr::Ckdiv,
            Scickdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Scickdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Scickdivcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Scickdivcr {
    #[inline(always)]
    fn default() -> Scickdivcr {
        <crate::RegValueT<Scickdivcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scickdivcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckdiv_SPEC;
    pub type Ckdiv = crate::EnumBitfieldStruct<u8, Ckdiv_SPEC>;
    impl Ckdiv {
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
pub struct Scickcr_SPEC;
impl crate::sealed::RegSpec for Scickcr_SPEC {
    type DataType = u8;
}

#[doc = "SCI clock control register"]
pub type Scickcr = crate::RegValueT<Scickcr_SPEC>;

impl Scickcr {
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub fn cksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        scickcr::Cksel,
        scickcr::Cksel,
        Scickcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            scickcr::Cksel,
            scickcr::Cksel,
            Scickcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Scickcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Scickcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock Switching Request"]
    #[inline(always)]
    pub fn cksreq(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        scickcr::Cksreq,
        scickcr::Cksreq,
        Scickcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            scickcr::Cksreq,
            scickcr::Cksreq,
            Scickcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Switching Ready state flag"]
    #[inline(always)]
    pub fn cksrdy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        scickcr::Cksrdy,
        scickcr::Cksrdy,
        Scickcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            scickcr::Cksrdy,
            scickcr::Cksrdy,
            Scickcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scickcr {
    #[inline(always)]
    fn default() -> Scickcr {
        <crate::RegValueT<Scickcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod scickcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksel_SPEC;
    pub type Cksel = crate::EnumBitfieldStruct<u8, Cksel_SPEC>;
    impl Cksel {
        #[doc = "HOCO"]
        pub const _0000: Self = Self::new(0);

        #[doc = "MOCO (value after reset)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Main clock oscillator"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator"]
        pub const _0100: Self = Self::new(4);

        #[doc = "PLL1P"]
        pub const _0101: Self = Self::new(5);

        #[doc = "PLL2P"]
        pub const _0110: Self = Self::new(6);

        #[doc = "PLL1Q"]
        pub const _0111: Self = Self::new(7);

        #[doc = "PLL1R"]
        pub const _1000: Self = Self::new(8);

        #[doc = "PLL2Q"]
        pub const _1001: Self = Self::new(9);

        #[doc = "PLL2R"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksreq_SPEC;
    pub type Cksreq = crate::EnumBitfieldStruct<u8, Cksreq_SPEC>;
    impl Cksreq {
        #[doc = "No request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request switching"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksrdy_SPEC;
    pub type Cksrdy = crate::EnumBitfieldStruct<u8, Cksrdy_SPEC>;
    impl Cksrdy {
        #[doc = "Impossible to Switch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Possible to Switch"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spickdivcr_SPEC;
impl crate::sealed::RegSpec for Spickdivcr_SPEC {
    type DataType = u8;
}

#[doc = "SPI clock Division control register"]
pub type Spickdivcr = crate::RegValueT<Spickdivcr_SPEC>;

impl Spickdivcr {
    #[doc = "Clock Division Select"]
    #[inline(always)]
    pub fn ckdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        spickdivcr::Ckdiv,
        spickdivcr::Ckdiv,
        Spickdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            spickdivcr::Ckdiv,
            spickdivcr::Ckdiv,
            Spickdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Spickdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Spickdivcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spickdivcr {
    #[inline(always)]
    fn default() -> Spickdivcr {
        <crate::RegValueT<Spickdivcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spickdivcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckdiv_SPEC;
    pub type Ckdiv = crate::EnumBitfieldStruct<u8, Ckdiv_SPEC>;
    impl Ckdiv {
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
pub struct Spickcr_SPEC;
impl crate::sealed::RegSpec for Spickcr_SPEC {
    type DataType = u8;
}

#[doc = "SPI clock control register"]
pub type Spickcr = crate::RegValueT<Spickcr_SPEC>;

impl Spickcr {
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub fn cksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        spickcr::Cksel,
        spickcr::Cksel,
        Spickcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            spickcr::Cksel,
            spickcr::Cksel,
            Spickcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Spickcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Spickcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock Switching Request"]
    #[inline(always)]
    pub fn cksreq(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        spickcr::Cksreq,
        spickcr::Cksreq,
        Spickcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            spickcr::Cksreq,
            spickcr::Cksreq,
            Spickcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Switching Ready state flag"]
    #[inline(always)]
    pub fn cksrdy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spickcr::Cksrdy,
        spickcr::Cksrdy,
        Spickcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spickcr::Cksrdy,
            spickcr::Cksrdy,
            Spickcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Spickcr {
    #[inline(always)]
    fn default() -> Spickcr {
        <crate::RegValueT<Spickcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod spickcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksel_SPEC;
    pub type Cksel = crate::EnumBitfieldStruct<u8, Cksel_SPEC>;
    impl Cksel {
        #[doc = "HOCO"]
        pub const _0000: Self = Self::new(0);

        #[doc = "MOCO (value after reset)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Main clock oscillator"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator"]
        pub const _0100: Self = Self::new(4);

        #[doc = "PLL1P"]
        pub const _0101: Self = Self::new(5);

        #[doc = "PLL2P"]
        pub const _0110: Self = Self::new(6);

        #[doc = "PLL1Q"]
        pub const _0111: Self = Self::new(7);

        #[doc = "PLL1R"]
        pub const _1000: Self = Self::new(8);

        #[doc = "PLL2Q"]
        pub const _1001: Self = Self::new(9);

        #[doc = "PLL2R"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksreq_SPEC;
    pub type Cksreq = crate::EnumBitfieldStruct<u8, Cksreq_SPEC>;
    impl Cksreq {
        #[doc = "No request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request switching"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksrdy_SPEC;
    pub type Cksrdy = crate::EnumBitfieldStruct<u8, Cksrdy_SPEC>;
    impl Cksrdy {
        #[doc = "Impossible to Switch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Possible to Switch"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcckdivcr_SPEC;
impl crate::sealed::RegSpec for Adcckdivcr_SPEC {
    type DataType = u8;
}

#[doc = "ADC clock Division control register"]
pub type Adcckdivcr = crate::RegValueT<Adcckdivcr_SPEC>;

impl Adcckdivcr {
    #[doc = "Clock Division Select"]
    #[inline(always)]
    pub fn ckdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        adcckdivcr::Ckdiv,
        adcckdivcr::Ckdiv,
        Adcckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            adcckdivcr::Ckdiv,
            adcckdivcr::Ckdiv,
            Adcckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Adcckdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Adcckdivcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adcckdivcr {
    #[inline(always)]
    fn default() -> Adcckdivcr {
        <crate::RegValueT<Adcckdivcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcckdivcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckdiv_SPEC;
    pub type Ckdiv = crate::EnumBitfieldStruct<u8, Ckdiv_SPEC>;
    impl Ckdiv {
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
pub struct Adcckcr_SPEC;
impl crate::sealed::RegSpec for Adcckcr_SPEC {
    type DataType = u8;
}

#[doc = "ADC clock control register"]
pub type Adcckcr = crate::RegValueT<Adcckcr_SPEC>;

impl Adcckcr {
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub fn cksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        adcckcr::Cksel,
        adcckcr::Cksel,
        Adcckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            adcckcr::Cksel,
            adcckcr::Cksel,
            Adcckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Adcckcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Adcckcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock Switching Request"]
    #[inline(always)]
    pub fn cksreq(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcckcr::Cksreq,
        adcckcr::Cksreq,
        Adcckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcckcr::Cksreq,
            adcckcr::Cksreq,
            Adcckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Switching Ready state flag"]
    #[inline(always)]
    pub fn cksrdy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcckcr::Cksrdy,
        adcckcr::Cksrdy,
        Adcckcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcckcr::Cksrdy,
            adcckcr::Cksrdy,
            Adcckcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcckcr {
    #[inline(always)]
    fn default() -> Adcckcr {
        <crate::RegValueT<Adcckcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod adcckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksel_SPEC;
    pub type Cksel = crate::EnumBitfieldStruct<u8, Cksel_SPEC>;
    impl Cksel {
        #[doc = "HOCO"]
        pub const _0000: Self = Self::new(0);

        #[doc = "MOCO (value after reset)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Main clock oscillator"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator"]
        pub const _0100: Self = Self::new(4);

        #[doc = "PLL1P"]
        pub const _0101: Self = Self::new(5);

        #[doc = "PLL2P"]
        pub const _0110: Self = Self::new(6);

        #[doc = "PLL1Q"]
        pub const _0111: Self = Self::new(7);

        #[doc = "PLL1R"]
        pub const _1000: Self = Self::new(8);

        #[doc = "PLL2Q"]
        pub const _1001: Self = Self::new(9);

        #[doc = "PLL2R"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksreq_SPEC;
    pub type Cksreq = crate::EnumBitfieldStruct<u8, Cksreq_SPEC>;
    impl Cksreq {
        #[doc = "No request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request switching"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksrdy_SPEC;
    pub type Cksrdy = crate::EnumBitfieldStruct<u8, Cksrdy_SPEC>;
    impl Cksrdy {
        #[doc = "Impossible to Switch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Possible to Switch"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gptckdivcr_SPEC;
impl crate::sealed::RegSpec for Gptckdivcr_SPEC {
    type DataType = u8;
}

#[doc = "GPT clock Division control register"]
pub type Gptckdivcr = crate::RegValueT<Gptckdivcr_SPEC>;

impl Gptckdivcr {
    #[doc = "Clock Division Select"]
    #[inline(always)]
    pub fn ckdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        gptckdivcr::Ckdiv,
        gptckdivcr::Ckdiv,
        Gptckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gptckdivcr::Ckdiv,
            gptckdivcr::Ckdiv,
            Gptckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Gptckdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Gptckdivcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gptckdivcr {
    #[inline(always)]
    fn default() -> Gptckdivcr {
        <crate::RegValueT<Gptckdivcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gptckdivcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckdiv_SPEC;
    pub type Ckdiv = crate::EnumBitfieldStruct<u8, Ckdiv_SPEC>;
    impl Ckdiv {
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
pub struct Gptckcr_SPEC;
impl crate::sealed::RegSpec for Gptckcr_SPEC {
    type DataType = u8;
}

#[doc = "GPT clock control register"]
pub type Gptckcr = crate::RegValueT<Gptckcr_SPEC>;

impl Gptckcr {
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub fn cksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        gptckcr::Cksel,
        gptckcr::Cksel,
        Gptckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            gptckcr::Cksel,
            gptckcr::Cksel,
            Gptckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Gptckcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Gptckcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock Switching Request"]
    #[inline(always)]
    pub fn cksreq(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gptckcr::Cksreq,
        gptckcr::Cksreq,
        Gptckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gptckcr::Cksreq,
            gptckcr::Cksreq,
            Gptckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Switching Ready state flag"]
    #[inline(always)]
    pub fn cksrdy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gptckcr::Cksrdy,
        gptckcr::Cksrdy,
        Gptckcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gptckcr::Cksrdy,
            gptckcr::Cksrdy,
            Gptckcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gptckcr {
    #[inline(always)]
    fn default() -> Gptckcr {
        <crate::RegValueT<Gptckcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod gptckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksel_SPEC;
    pub type Cksel = crate::EnumBitfieldStruct<u8, Cksel_SPEC>;
    impl Cksel {
        #[doc = "HOCO"]
        pub const _0000: Self = Self::new(0);

        #[doc = "MOCO (value after reset)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Main clock oscillator"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator"]
        pub const _0100: Self = Self::new(4);

        #[doc = "PLL1P"]
        pub const _0101: Self = Self::new(5);

        #[doc = "PLL2P"]
        pub const _0110: Self = Self::new(6);

        #[doc = "PLL1Q"]
        pub const _0111: Self = Self::new(7);

        #[doc = "PLL1R"]
        pub const _1000: Self = Self::new(8);

        #[doc = "PLL2Q"]
        pub const _1001: Self = Self::new(9);

        #[doc = "PLL2R"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksreq_SPEC;
    pub type Cksreq = crate::EnumBitfieldStruct<u8, Cksreq_SPEC>;
    impl Cksreq {
        #[doc = "No request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request switching"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksrdy_SPEC;
    pub type Cksrdy = crate::EnumBitfieldStruct<u8, Cksrdy_SPEC>;
    impl Cksrdy {
        #[doc = "Impossible to Switch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Possible to Switch"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdckdivcr_SPEC;
impl crate::sealed::RegSpec for Lcdckdivcr_SPEC {
    type DataType = u8;
}

#[doc = "LCD clock Division control register"]
pub type Lcdckdivcr = crate::RegValueT<Lcdckdivcr_SPEC>;

impl Lcdckdivcr {
    #[doc = "Clock Division Select"]
    #[inline(always)]
    pub fn ckdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        lcdckdivcr::Ckdiv,
        lcdckdivcr::Ckdiv,
        Lcdckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            lcdckdivcr::Ckdiv,
            lcdckdivcr::Ckdiv,
            Lcdckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Lcdckdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Lcdckdivcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lcdckdivcr {
    #[inline(always)]
    fn default() -> Lcdckdivcr {
        <crate::RegValueT<Lcdckdivcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lcdckdivcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckdiv_SPEC;
    pub type Ckdiv = crate::EnumBitfieldStruct<u8, Ckdiv_SPEC>;
    impl Ckdiv {
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
pub struct Lcdckcr_SPEC;
impl crate::sealed::RegSpec for Lcdckcr_SPEC {
    type DataType = u8;
}

#[doc = "LCD clock control register"]
pub type Lcdckcr = crate::RegValueT<Lcdckcr_SPEC>;

impl Lcdckcr {
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub fn cksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        lcdckcr::Cksel,
        lcdckcr::Cksel,
        Lcdckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            lcdckcr::Cksel,
            lcdckcr::Cksel,
            Lcdckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Lcdckcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Lcdckcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock Switching Request"]
    #[inline(always)]
    pub fn cksreq(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        lcdckcr::Cksreq,
        lcdckcr::Cksreq,
        Lcdckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            lcdckcr::Cksreq,
            lcdckcr::Cksreq,
            Lcdckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Switching Ready state flag"]
    #[inline(always)]
    pub fn cksrdy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        lcdckcr::Cksrdy,
        lcdckcr::Cksrdy,
        Lcdckcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            lcdckcr::Cksrdy,
            lcdckcr::Cksrdy,
            Lcdckcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lcdckcr {
    #[inline(always)]
    fn default() -> Lcdckcr {
        <crate::RegValueT<Lcdckcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod lcdckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksel_SPEC;
    pub type Cksel = crate::EnumBitfieldStruct<u8, Cksel_SPEC>;
    impl Cksel {
        #[doc = "HOCO"]
        pub const _0000: Self = Self::new(0);

        #[doc = "MOCO (value after reset)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Main clock oscillator"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator"]
        pub const _0100: Self = Self::new(4);

        #[doc = "PLL1P"]
        pub const _0101: Self = Self::new(5);

        #[doc = "PLL2P"]
        pub const _0110: Self = Self::new(6);

        #[doc = "PLL1Q"]
        pub const _0111: Self = Self::new(7);

        #[doc = "PLL1R"]
        pub const _1000: Self = Self::new(8);

        #[doc = "PLL2Q"]
        pub const _1001: Self = Self::new(9);

        #[doc = "PLL2R"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksreq_SPEC;
    pub type Cksreq = crate::EnumBitfieldStruct<u8, Cksreq_SPEC>;
    impl Cksreq {
        #[doc = "No request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request switching"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksrdy_SPEC;
    pub type Cksrdy = crate::EnumBitfieldStruct<u8, Cksrdy_SPEC>;
    impl Cksrdy {
        #[doc = "Impossible to Switch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Possible to Switch"]
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
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        mocoutcr::Mocoutrm,
        mocoutcr::Mocoutrm,
        Mocoutcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            mocoutcr::Mocoutrm,
            mocoutcr::Mocoutrm,
            Mocoutcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mocoutcr {
    #[inline(always)]
    fn default() -> Mocoutcr {
        <crate::RegValueT<Mocoutcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mocoutcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mocoutrm_SPEC;
    pub type Mocoutrm = crate::EnumBitfieldStruct<u8, Mocoutrm_SPEC>;
    impl Mocoutrm {
        #[doc = "-128"]
        pub const _1000_0000: Self = Self::new(128);

        #[doc = "-127"]
        pub const _1000_0001: Self = Self::new(129);

        #[doc = "-126"]
        pub const _1000_0010: Self = Self::new(130);

        #[doc = "-1"]
        pub const _1111_1111: Self = Self::new(255);

        #[doc = "Center Code"]
        pub const _0000_0000: Self = Self::new(0);

        #[doc = "+1"]
        pub const _0000_0001: Self = Self::new(1);

        #[doc = "+125"]
        pub const _0111_1101: Self = Self::new(125);

        #[doc = "+126"]
        pub const _0111_1110: Self = Self::new(126);

        #[doc = "+127"]
        pub const _0111_1111: Self = Self::new(127);
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
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        hocoutcr::Hocoutrm,
        hocoutcr::Hocoutrm,
        Hocoutcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            hocoutcr::Hocoutrm,
            hocoutcr::Hocoutrm,
            Hocoutcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Hocoutcr {
    #[inline(always)]
    fn default() -> Hocoutcr {
        <crate::RegValueT<Hocoutcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hocoutcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hocoutrm_SPEC;
    pub type Hocoutrm = crate::EnumBitfieldStruct<u8, Hocoutrm_SPEC>;
    impl Hocoutrm {
        #[doc = "-128"]
        pub const _1000_0000: Self = Self::new(128);

        #[doc = "-127"]
        pub const _1000_0001: Self = Self::new(129);

        #[doc = "-126"]
        pub const _1000_0010: Self = Self::new(130);

        #[doc = "-1"]
        pub const _1111_1111: Self = Self::new(255);

        #[doc = "Center Code"]
        pub const _0000_0000: Self = Self::new(0);

        #[doc = "+1"]
        pub const _0000_0001: Self = Self::new(1);

        #[doc = "+125"]
        pub const _0111_1101: Self = Self::new(125);

        #[doc = "+126"]
        pub const _0111_1110: Self = Self::new(126);

        #[doc = "+127"]
        pub const _0111_1111: Self = Self::new(127);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbckdivcr_SPEC;
impl crate::sealed::RegSpec for Usbckdivcr_SPEC {
    type DataType = u8;
}

#[doc = "USB clock Division control register"]
pub type Usbckdivcr = crate::RegValueT<Usbckdivcr_SPEC>;

impl Usbckdivcr {
    #[doc = "USB clock (USBCLK) Division Select"]
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

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Usbckdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Usbckdivcr_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Octackdivcr_SPEC;
impl crate::sealed::RegSpec for Octackdivcr_SPEC {
    type DataType = u8;
}

#[doc = "Octal-SPI clock Division control register"]
pub type Octackdivcr = crate::RegValueT<Octackdivcr_SPEC>;

impl Octackdivcr {
    #[doc = "Octal-SPI clock (OCTACLK) Division Select"]
    #[inline(always)]
    pub fn octackdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        octackdivcr::Octackdiv,
        octackdivcr::Octackdiv,
        Octackdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            octackdivcr::Octackdiv,
            octackdivcr::Octackdiv,
            Octackdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Octackdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Octackdivcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Octackdivcr {
    #[inline(always)]
    fn default() -> Octackdivcr {
        <crate::RegValueT<Octackdivcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod octackdivcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Octackdiv_SPEC;
    pub type Octackdiv = crate::EnumBitfieldStruct<u8, Octackdiv_SPEC>;
    impl Octackdiv {
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
pub struct Canfdckdivcr_SPEC;
impl crate::sealed::RegSpec for Canfdckdivcr_SPEC {
    type DataType = u8;
}

#[doc = "CANFD Core clock Division control register"]
pub type Canfdckdivcr = crate::RegValueT<Canfdckdivcr_SPEC>;

impl Canfdckdivcr {
    #[doc = "CANFD Core clock (CANFDCLK) Division Select"]
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

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Canfdckdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Canfdckdivcr_SPEC,crate::common::RW>::from_register(self,0)
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

        #[doc = "/3"]
        pub const _101: Self = Self::new(5);

        #[doc = "/5"]
        pub const _110: Self = Self::new(6);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb60Ckdivcr_SPEC;
impl crate::sealed::RegSpec for Usb60Ckdivcr_SPEC {
    type DataType = u8;
}

#[doc = "USB60 clock Division control register"]
pub type Usb60Ckdivcr = crate::RegValueT<Usb60Ckdivcr_SPEC>;

impl Usb60Ckdivcr {
    #[doc = "USB clock (USB60CLK) Division Select"]
    #[inline(always)]
    pub fn usb60ckdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        usb60ckdivcr::Usb60Ckdiv,
        usb60ckdivcr::Usb60Ckdiv,
        Usb60Ckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            usb60ckdivcr::Usb60Ckdiv,
            usb60ckdivcr::Usb60Ckdiv,
            Usb60Ckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Usb60Ckdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Usb60Ckdivcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Usb60Ckdivcr {
    #[inline(always)]
    fn default() -> Usb60Ckdivcr {
        <crate::RegValueT<Usb60Ckdivcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod usb60ckdivcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usb60Ckdiv_SPEC;
    pub type Usb60Ckdiv = crate::EnumBitfieldStruct<u8, Usb60Ckdiv_SPEC>;
    impl Usb60Ckdiv {
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
pub struct I3Cckdivcr_SPEC;
impl crate::sealed::RegSpec for I3Cckdivcr_SPEC {
    type DataType = u8;
}

#[doc = "I3C clock Division control register"]
pub type I3Cckdivcr = crate::RegValueT<I3Cckdivcr_SPEC>;

impl I3Cckdivcr {
    #[doc = "I3C clock (I3CCLK) Division Select"]
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

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, I3Cckdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,I3Cckdivcr_SPEC,crate::common::RW>::from_register(self,0)
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

#[doc = "USB clock control register"]
pub type Usbckcr = crate::RegValueT<Usbckcr_SPEC>;

impl Usbckcr {
    #[doc = "USB clock (USBCLK) Source Select"]
    #[inline(always)]
    pub fn usbcksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        usbckcr::Usbcksel,
        usbckcr::Usbcksel,
        Usbckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            usbckcr::Usbcksel,
            usbckcr::Usbcksel,
            Usbckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Usbckcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Usbckcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "USB clock (USBCLK) Switching Request"]
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

    #[doc = "USB clock (USBCLK) Switching Ready state flag"]
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
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            usbckcr::Usbcksrdy,
            usbckcr::Usbcksrdy,
            Usbckcr_SPEC,
            crate::common::RW,
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
        #[doc = "HOCO"]
        pub const _0000: Self = Self::new(0);

        #[doc = "MOCO (value after reset)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Main clock oscillator"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator"]
        pub const _0100: Self = Self::new(4);

        #[doc = "PLL1P"]
        pub const _0101: Self = Self::new(5);

        #[doc = "PLL2P"]
        pub const _0110: Self = Self::new(6);

        #[doc = "PLL1Q"]
        pub const _0111: Self = Self::new(7);

        #[doc = "PLL1R"]
        pub const _1000: Self = Self::new(8);

        #[doc = "PLL2Q"]
        pub const _1001: Self = Self::new(9);

        #[doc = "PLL2R"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbcksreq_SPEC;
    pub type Usbcksreq = crate::EnumBitfieldStruct<u8, Usbcksreq_SPEC>;
    impl Usbcksreq {
        #[doc = "No request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request switching"]
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
pub struct Octackcr_SPEC;
impl crate::sealed::RegSpec for Octackcr_SPEC {
    type DataType = u8;
}

#[doc = "Octal-SPI clock control register"]
pub type Octackcr = crate::RegValueT<Octackcr_SPEC>;

impl Octackcr {
    #[doc = "Octal-SPI clock (OCTACLK) Source Select"]
    #[inline(always)]
    pub fn octacksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        octackcr::Octacksel,
        octackcr::Octacksel,
        Octackcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            octackcr::Octacksel,
            octackcr::Octacksel,
            Octackcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Octackcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Octackcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Octal-SPI clock (OCTACLK) Switching Request"]
    #[inline(always)]
    pub fn octacksreq(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        octackcr::Octacksreq,
        octackcr::Octacksreq,
        Octackcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            octackcr::Octacksreq,
            octackcr::Octacksreq,
            Octackcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Octal-SPI clock (OCTACLK) Switching Ready state flag"]
    #[inline(always)]
    pub fn octacksrdy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        octackcr::Octacksrdy,
        octackcr::Octacksrdy,
        Octackcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            octackcr::Octacksrdy,
            octackcr::Octacksrdy,
            Octackcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Octackcr {
    #[inline(always)]
    fn default() -> Octackcr {
        <crate::RegValueT<Octackcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod octackcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Octacksel_SPEC;
    pub type Octacksel = crate::EnumBitfieldStruct<u8, Octacksel_SPEC>;
    impl Octacksel {
        #[doc = "HOCO"]
        pub const _0000: Self = Self::new(0);

        #[doc = "MOCO (value after reset)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Main clock oscillator"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator"]
        pub const _0100: Self = Self::new(4);

        #[doc = "PLL1P"]
        pub const _0101: Self = Self::new(5);

        #[doc = "PLL2P"]
        pub const _0110: Self = Self::new(6);

        #[doc = "PLL1Q"]
        pub const _0111: Self = Self::new(7);

        #[doc = "PLL1R"]
        pub const _1000: Self = Self::new(8);

        #[doc = "PLL2Q"]
        pub const _1001: Self = Self::new(9);

        #[doc = "PLL2R"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Octacksreq_SPEC;
    pub type Octacksreq = crate::EnumBitfieldStruct<u8, Octacksreq_SPEC>;
    impl Octacksreq {
        #[doc = "No request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request switching"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Octacksrdy_SPEC;
    pub type Octacksrdy = crate::EnumBitfieldStruct<u8, Octacksrdy_SPEC>;
    impl Octacksrdy {
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

#[doc = "CANFD Core clock control register"]
pub type Canfdckcr = crate::RegValueT<Canfdckcr_SPEC>;

impl Canfdckcr {
    #[doc = "CANFD Core clock (CANFDCLK) Source Select"]
    #[inline(always)]
    pub fn canfdcksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        canfdckcr::Canfdcksel,
        canfdckcr::Canfdcksel,
        Canfdckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            canfdckcr::Canfdcksel,
            canfdckcr::Canfdcksel,
            Canfdckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Canfdckcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Canfdckcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CANFD Core clock (CANFDCLK) Switching Request"]
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

    #[doc = "CANFD Core clock (CANFDCLK) Switching Ready state flag"]
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
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            canfdckcr::Canfdcksrdy,
            canfdckcr::Canfdcksrdy,
            Canfdckcr_SPEC,
            crate::common::RW,
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
        pub const _0000: Self = Self::new(0);

        #[doc = "MOCO (value after reset)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Main clock oscillator"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator"]
        pub const _0100: Self = Self::new(4);

        #[doc = "PLL1P"]
        pub const _0101: Self = Self::new(5);

        #[doc = "PLL2P"]
        pub const _0110: Self = Self::new(6);

        #[doc = "PLL1Q"]
        pub const _0111: Self = Self::new(7);

        #[doc = "PLL1R"]
        pub const _1000: Self = Self::new(8);

        #[doc = "PLL2Q"]
        pub const _1001: Self = Self::new(9);

        #[doc = "PLL2R"]
        pub const _1010: Self = Self::new(10);
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
        #[doc = "Impossible to Switch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Possible to Switch"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb60Ckcr_SPEC;
impl crate::sealed::RegSpec for Usb60Ckcr_SPEC {
    type DataType = u8;
}

#[doc = "USB60 clock control register"]
pub type Usb60Ckcr = crate::RegValueT<Usb60Ckcr_SPEC>;

impl Usb60Ckcr {
    #[doc = "USB clock (USB60CLK) Source Select"]
    #[inline(always)]
    pub fn usb60cksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        usb60ckcr::Usb60Cksel,
        usb60ckcr::Usb60Cksel,
        Usb60Ckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            usb60ckcr::Usb60Cksel,
            usb60ckcr::Usb60Cksel,
            Usb60Ckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Usb60Ckcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Usb60Ckcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "USB clock (USB60CLK)  Switching Request"]
    #[inline(always)]
    pub fn usb60cksreq(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        usb60ckcr::Usb60Cksreq,
        usb60ckcr::Usb60Cksreq,
        Usb60Ckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            usb60ckcr::Usb60Cksreq,
            usb60ckcr::Usb60Cksreq,
            Usb60Ckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USB clock (USB60CLK)  Switching Ready state flag"]
    #[inline(always)]
    pub fn usb60cksrdy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        usb60ckcr::Usb60Cksrdy,
        usb60ckcr::Usb60Cksrdy,
        Usb60Ckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            usb60ckcr::Usb60Cksrdy,
            usb60ckcr::Usb60Cksrdy,
            Usb60Ckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Usb60Ckcr {
    #[inline(always)]
    fn default() -> Usb60Ckcr {
        <crate::RegValueT<Usb60Ckcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod usb60ckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usb60Cksel_SPEC;
    pub type Usb60Cksel = crate::EnumBitfieldStruct<u8, Usb60Cksel_SPEC>;
    impl Usb60Cksel {
        #[doc = "HOCO"]
        pub const _0000: Self = Self::new(0);

        #[doc = "MOCO (value after reset)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Main clock oscillator"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator"]
        pub const _0100: Self = Self::new(4);

        #[doc = "PLL1P"]
        pub const _0101: Self = Self::new(5);

        #[doc = "PLL2P"]
        pub const _0110: Self = Self::new(6);

        #[doc = "PLL1Q"]
        pub const _0111: Self = Self::new(7);

        #[doc = "PLL1R"]
        pub const _1000: Self = Self::new(8);

        #[doc = "PLL2Q"]
        pub const _1001: Self = Self::new(9);

        #[doc = "PLL2R"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usb60Cksreq_SPEC;
    pub type Usb60Cksreq = crate::EnumBitfieldStruct<u8, Usb60Cksreq_SPEC>;
    impl Usb60Cksreq {
        #[doc = "No request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request switching"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usb60Cksrdy_SPEC;
    pub type Usb60Cksrdy = crate::EnumBitfieldStruct<u8, Usb60Cksrdy_SPEC>;
    impl Usb60Cksrdy {
        #[doc = "Impossible to Switch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Possible to Switch"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3Cckcr_SPEC;
impl crate::sealed::RegSpec for I3Cckcr_SPEC {
    type DataType = u8;
}

#[doc = "I3C clock control register"]
pub type I3Cckcr = crate::RegValueT<I3Cckcr_SPEC>;

impl I3Cckcr {
    #[doc = "I3C clock (I3CCLK) Source Select"]
    #[inline(always)]
    pub fn i3ccksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        i3cckcr::I3Ccksel,
        i3cckcr::I3Ccksel,
        I3Cckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            i3cckcr::I3Ccksel,
            i3cckcr::I3Ccksel,
            I3Cckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, I3Cckcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,I3Cckcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "I3C clock (I3CCLK) Switching Request"]
    #[inline(always)]
    pub fn i3cckreq(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        i3cckcr::I3Cckreq,
        i3cckcr::I3Cckreq,
        I3Cckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            i3cckcr::I3Cckreq,
            i3cckcr::I3Cckreq,
            I3Cckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I3C clock (I3CCLK) Switching Ready state flag"]
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
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            i3cckcr::I3Ccksrdy,
            i3cckcr::I3Ccksrdy,
            I3Cckcr_SPEC,
            crate::common::RW,
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
        pub const _0000: Self = Self::new(0);

        #[doc = "MOCO (value after reset)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Main clock oscillator"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator"]
        pub const _0100: Self = Self::new(4);

        #[doc = "PLL1P"]
        pub const _0101: Self = Self::new(5);

        #[doc = "PLL2P"]
        pub const _0110: Self = Self::new(6);

        #[doc = "PLL1Q"]
        pub const _0111: Self = Self::new(7);

        #[doc = "PLL1R"]
        pub const _1000: Self = Self::new(8);

        #[doc = "PLL2Q"]
        pub const _1001: Self = Self::new(9);

        #[doc = "PLL2R"]
        pub const _1010: Self = Self::new(10);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct I3Cckreq_SPEC;
    pub type I3Cckreq = crate::EnumBitfieldStruct<u8, I3Cckreq_SPEC>;
    impl I3Cckreq {
        #[doc = "No request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request switching"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct I3Ccksrdy_SPEC;
    pub type I3Ccksrdy = crate::EnumBitfieldStruct<u8, I3Ccksrdy_SPEC>;
    impl I3Ccksrdy {
        #[doc = "Impossible to Switch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Possible to Switch"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Moscscr_SPEC;
impl crate::sealed::RegSpec for Moscscr_SPEC {
    type DataType = u8;
}

#[doc = "Main Clock Oscillator Standby Control Register"]
pub type Moscscr = crate::RegValueT<Moscscr_SPEC>;

impl Moscscr {
    #[doc = "Main Clock Oscillator Standby Oscillation Keep select"]
    #[inline(always)]
    pub fn moscsokp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        moscscr::Moscsokp,
        moscscr::Moscsokp,
        Moscscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            moscscr::Moscsokp,
            moscscr::Moscsokp,
            Moscscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Moscscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Moscscr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Moscscr {
    #[inline(always)]
    fn default() -> Moscscr {
        <crate::RegValueT<Moscscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod moscscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moscsokp_SPEC;
    pub type Moscsokp = crate::EnumBitfieldStruct<u8, Moscsokp_SPEC>;
    impl Moscsokp {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hocoscr_SPEC;
impl crate::sealed::RegSpec for Hocoscr_SPEC {
    type DataType = u8;
}

#[doc = "High-Speed On-Chip Oscillator Standby Control Register"]
pub type Hocoscr = crate::RegValueT<Hocoscr_SPEC>;

impl Hocoscr {
    #[doc = "HOCO Standby Oscillation Keep select"]
    #[inline(always)]
    pub fn hocosokp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        hocoscr::Hocosokp,
        hocoscr::Hocosokp,
        Hocoscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            hocoscr::Hocosokp,
            hocoscr::Hocosokp,
            Hocoscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Hocoscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Hocoscr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Hocoscr {
    #[inline(always)]
    fn default() -> Hocoscr {
        <crate::RegValueT<Hocoscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hocoscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hocosokp_SPEC;
    pub type Hocosokp = crate::EnumBitfieldStruct<u8, Hocosokp_SPEC>;
    impl Hocosokp {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
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

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Opccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Opccr_SPEC,crate::common::RW>::from_register(self,0)
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

        #[doc = "Prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Prohibited"]
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
    #[doc = "Main clock oscillator wait time setting"]
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

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Moscwtcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Moscwtcr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Wait time=    11.4us (3 cycles) / 15.3us (4 cycles)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "Wait time=   133.5us (35 cycles)  / 137.3us (36 cycles)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "Wait time=   255.6us (67 cycles) / 259.4us (68 cycles)"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Wait time=   499.7us (131 cycles) / 503.5us (132 cycles)"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Wait time=   988.0us (259 cycles) / 991.8us (260 cycles)"]
        pub const _0100: Self = Self::new(4);

        #[doc = "Wait time=  2086.6us (547 cycles) (value after reset) / 2090.5us (548 cycles) (value after reset)"]
        pub const _0101: Self = Self::new(5);

        #[doc = "Wait time=  4039.8us (1059 cycles) / 4043.6us (1060 cycles)"]
        pub const _0110: Self = Self::new(6);

        #[doc = "Wait time=  8190.2us (2147 cycles) / 8194.0us (2148 cycles)"]
        pub const _0111: Self = Self::new(7);

        #[doc = "Wait time= 16368.9us (4291 cycles) / 16372.7us (4292 cycles)"]
        pub const _1000: Self = Self::new(8);

        #[doc = "Wait time= 31139.4us (8163 cycles) / 31143.2us (8164 cycles)"]
        pub const _1001: Self = Self::new(9);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsr1_SPEC;
impl crate::sealed::RegSpec for Rstsr1_SPEC {
    type DataType = u32;
}

#[doc = "Reset Status Register 1"]
pub type Rstsr1 = crate::RegValueT<Rstsr1_SPEC>;

impl Rstsr1 {
    #[doc = "Independent Watchdog Timer Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
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

    #[doc = "Watchdog Timer0 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn wdt0rf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rstsr1::Wdt0Rf,
        rstsr1::Wdt0Rf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rstsr1::Wdt0Rf,
            rstsr1::Wdt0Rf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
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

    #[doc = "CPU0 Lockup Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn clu0rf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        rstsr1::Clu0Rf,
        rstsr1::Clu0Rf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            rstsr1::Clu0Rf,
            rstsr1::Clu0Rf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Local memory 0 error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn lm0rf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        rstsr1::Lm0Rf,
        rstsr1::Lm0Rf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            rstsr1::Lm0Rf,
            rstsr1::Lm0Rf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn busrf(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        rstsr1::Busrf,
        rstsr1::Busrf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            rstsr1::Busrf,
            rstsr1::Busrf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Common memory error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn cmrf(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        rstsr1::Cmrf,
        rstsr1::Cmrf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            rstsr1::Cmrf,
            rstsr1::Cmrf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watchdog Timer1 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn wdt1rf(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        rstsr1::Wdt1Rf,
        rstsr1::Wdt1Rf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            rstsr1::Wdt1Rf,
            rstsr1::Wdt1Rf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Local memory 1 error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn lm1rf(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        rstsr1::Lm1Rf,
        rstsr1::Lm1Rf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            rstsr1::Lm1Rf,
            rstsr1::Lm1Rf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Network Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn nwrf(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        rstsr1::Nwrf,
        rstsr1::Nwrf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            rstsr1::Nwrf,
            rstsr1::Nwrf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000. The write value should be 000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<23, 0x1ff, 1, 0, u16, u16, Rstsr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1ff,1,0,u16,u16,Rstsr1_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Independent watchdog timer reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Independent watchdog timer reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdt0Rf_SPEC;
    pub type Wdt0Rf = crate::EnumBitfieldStruct<u8, Wdt0Rf_SPEC>;
    impl Wdt0Rf {
        #[doc = "Watchdog timer0 reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Watchdog timer0 reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrf_SPEC;
    pub type Swrf = crate::EnumBitfieldStruct<u8, Swrf_SPEC>;
    impl Swrf {
        #[doc = "Software reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Software reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clu0Rf_SPEC;
    pub type Clu0Rf = crate::EnumBitfieldStruct<u8, Clu0Rf_SPEC>;
    impl Clu0Rf {
        #[doc = "CPU0 Lockup reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU0 Lockup reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lm0Rf_SPEC;
    pub type Lm0Rf = crate::EnumBitfieldStruct<u8, Lm0Rf_SPEC>;
    impl Lm0Rf {
        #[doc = "Local memory 0 error reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Local memory 0 error reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busrf_SPEC;
    pub type Busrf = crate::EnumBitfieldStruct<u8, Busrf_SPEC>;
    impl Busrf {
        #[doc = "Bus error reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus error reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmrf_SPEC;
    pub type Cmrf = crate::EnumBitfieldStruct<u8, Cmrf_SPEC>;
    impl Cmrf {
        #[doc = "Common memory error reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Common memory error reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdt1Rf_SPEC;
    pub type Wdt1Rf = crate::EnumBitfieldStruct<u8, Wdt1Rf_SPEC>;
    impl Wdt1Rf {
        #[doc = "Watchdog timer1 reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Watchdog timer1 reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lm1Rf_SPEC;
    pub type Lm1Rf = crate::EnumBitfieldStruct<u8, Lm1Rf_SPEC>;
    impl Lm1Rf {
        #[doc = "Local memory 1 error reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Local memory 1 error reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nwrf_SPEC;
    pub type Nwrf = crate::EnumBitfieldStruct<u8, Nwrf_SPEC>;
    impl Nwrf {
        #[doc = "Network Reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Network Reset detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syraccr_SPEC;
impl crate::sealed::RegSpec for Syraccr_SPEC {
    type DataType = u8;
}

#[doc = "System Register Access Control Register"]
pub type Syraccr = crate::RegValueT<Syraccr_SPEC>;

impl Syraccr {
    #[doc = "Access Ready monitor"]
    #[inline(always)]
    pub fn busy(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syraccr::Busy,
        syraccr::Busy,
        Syraccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syraccr::Busy,
            syraccr::Busy,
            Syraccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Syraccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Syraccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Syraccr {
    #[inline(always)]
    fn default() -> Syraccr {
        <crate::RegValueT<Syraccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syraccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busy_SPEC;
    pub type Busy = crate::EnumBitfieldStruct<u8, Busy_SPEC>;
    impl Busy {
        #[doc = "Ready to read/write access"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing in progress"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pvdcr1_SPEC;
impl crate::sealed::RegSpec for Pvdcr1_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor %s Circuit Control Register 1"]
pub type Pvdcr1 = crate::RegValueT<Pvdcr1_SPEC>;

impl Pvdcr1 {
    #[doc = "Voltage Monitor  Interrupt Generation Condition Select"]
    #[inline(always)]
    pub fn idtsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        pvdcr1::Idtsel,
        pvdcr1::Idtsel,
        Pvdcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            pvdcr1::Idtsel,
            pvdcr1::Idtsel,
            Pvdcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor  Interrupt Type Select"]
    #[inline(always)]
    pub fn irqsel(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pvdcr1::Irqsel,
        pvdcr1::Irqsel,
        Pvdcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pvdcr1::Irqsel,
            pvdcr1::Irqsel,
            Pvdcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Pvdcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Pvdcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pvdcr1 {
    #[inline(always)]
    fn default() -> Pvdcr1 {
        <crate::RegValueT<Pvdcr1_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod pvdcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idtsel_SPEC;
    pub type Idtsel = crate::EnumBitfieldStruct<u8, Idtsel_SPEC>;
    impl Idtsel {
        #[doc = "Generate when VCC>=Vdet (rise) is detected"]
        pub const _00: Self = Self::new(0);

        #[doc = "Generate when VCC<Vdet (fall) is detected"]
        pub const _01: Self = Self::new(1);

        #[doc = "Generate when fall and rise are detected"]
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
pub struct Pvdsr_SPEC;
impl crate::sealed::RegSpec for Pvdsr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor %s Circuit Status Register"]
pub type Pvdsr = crate::RegValueT<Pvdsr_SPEC>;

impl Pvdsr {
    #[doc = "Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0."]
    #[inline(always)]
    pub fn det(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pvdsr::Det,
        pvdsr::Det,
        Pvdsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pvdsr::Det,
            pvdsr::Det,
            Pvdsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor Signal Monitor Flag"]
    #[inline(always)]
    pub fn mon(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pvdsr::Mon,
        pvdsr::Mon,
        Pvdsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pvdsr::Mon,
            pvdsr::Mon,
            Pvdsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Pvdsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Pvdsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pvdsr {
    #[inline(always)]
    fn default() -> Pvdsr {
        <crate::RegValueT<Pvdsr_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod pvdsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Det_SPEC;
    pub type Det = crate::EnumBitfieldStruct<u8, Det_SPEC>;
    impl Det {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Vdet passage detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mon_SPEC;
    pub type Mon = crate::EnumBitfieldStruct<u8, Mon_SPEC>;
    impl Mon {
        #[doc = "VCC <= Vdet"]
        pub const _0: Self = Self::new(0);

        #[doc = "VCC > Vdet or MON bit is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crvsyscr_SPEC;
impl crate::sealed::RegSpec for Crvsyscr_SPEC {
    type DataType = u8;
}

#[doc = "Clock Recovery System Control Register"]
pub type Crvsyscr = crate::RegValueT<Crvsyscr_SPEC>;

impl Crvsyscr {
    #[doc = "Clock Recovery Enable"]
    #[inline(always)]
    pub fn crven(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        crvsyscr::Crven,
        crvsyscr::Crven,
        Crvsyscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            crvsyscr::Crven,
            crvsyscr::Crven,
            Crvsyscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Crvsyscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Crvsyscr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Crvsyscr {
    #[inline(always)]
    fn default() -> Crvsyscr {
        <crate::RegValueT<Crvsyscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod crvsyscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crven_SPEC;
    pub type Crven = crate::EnumBitfieldStruct<u8, Crven_SPEC>;
    impl Crven {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdctrgd_SPEC;
impl crate::sealed::RegSpec for Pdctrgd_SPEC {
    type DataType = u8;
}

#[doc = "Graphics Power Domain Control Register"]
pub type Pdctrgd = crate::RegValueT<Pdctrgd_SPEC>;

impl Pdctrgd {
    #[doc = "Power control enable"]
    #[inline(always)]
    pub fn pdde(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdctrgd::Pdde,
        pdctrgd::Pdde,
        Pdctrgd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdctrgd::Pdde,
            pdctrgd::Pdde,
            Pdctrgd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x1f, 1, 0, u8, u8, Pdctrgd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1f,1,0,u8,u8,Pdctrgd_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Power control status flag"]
    #[inline(always)]
    pub fn pdcsf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pdctrgd::Pdcsf,
        pdctrgd::Pdcsf,
        Pdctrgd_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pdctrgd::Pdcsf,
            pdctrgd::Pdcsf,
            Pdctrgd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Power gating status flag"]
    #[inline(always)]
    pub fn pdpgsf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pdctrgd::Pdpgsf,
        pdctrgd::Pdpgsf,
        Pdctrgd_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pdctrgd::Pdpgsf,
            pdctrgd::Pdpgsf,
            Pdctrgd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdctrgd {
    #[inline(always)]
    fn default() -> Pdctrgd {
        <crate::RegValueT<Pdctrgd_SPEC> as RegisterValue<_>>::new(129)
    }
}
pub mod pdctrgd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdde_SPEC;
    pub type Pdde = crate::EnumBitfieldStruct<u8, Pdde_SPEC>;
    impl Pdde {
        #[doc = "Power on the target domain"]
        pub const _0: Self = Self::new(0);

        #[doc = "Power off the target domain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdcsf_SPEC;
    pub type Pdcsf = crate::EnumBitfieldStruct<u8, Pdcsf_SPEC>;
    impl Pdcsf {
        #[doc = "Power gating control is not executed (idle)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Power gating control is in progress"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdpgsf_SPEC;
    pub type Pdpgsf = crate::EnumBitfieldStruct<u8, Pdpgsf_SPEC>;
    impl Pdpgsf {
        #[doc = "Target domain is power on (not gating)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Target domain is power off (during Gating)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdramscr0_SPEC;
impl crate::sealed::RegSpec for Pdramscr0_SPEC {
    type DataType = u16;
}

#[doc = "SRAM power domain Standby Control Register 0"]
pub type Pdramscr0 = crate::RegValueT<Pdramscr0_SPEC>;

impl Pdramscr0 {
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pdramscr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Pdramscr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdramscr0 {
    #[inline(always)]
    fn default() -> Pdramscr0 {
        <crate::RegValueT<Pdramscr0_SPEC> as RegisterValue<_>>::new(32767)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdramscr1_SPEC;
impl crate::sealed::RegSpec for Pdramscr1_SPEC {
    type DataType = u8;
}

#[doc = "SRAM power domain Standby Control Register 1"]
pub type Pdramscr1 = crate::RegValueT<Pdramscr1_SPEC>;

impl Pdramscr1 {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Pdramscr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Pdramscr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdramscr1 {
    #[inline(always)]
    fn default() -> Pdramscr1 {
        <crate::RegValueT<Pdramscr1_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbrsabar_SPEC;
impl crate::sealed::RegSpec for Vbrsabar_SPEC {
    type DataType = u16;
}

#[doc = "VBATT Backup Register Security Attribute Boundary Address Register"]
pub type Vbrsabar = crate::RegValueT<Vbrsabar_SPEC>;

impl Vbrsabar {
    #[doc = "Security Attribute Boundary Address"]
    #[inline(always)]
    pub fn saba(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Vbrsabar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Vbrsabar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbrsabar {
    #[inline(always)]
    fn default() -> Vbrsabar {
        <crate::RegValueT<Vbrsabar_SPEC> as RegisterValue<_>>::new(65504)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbrpabars_SPEC;
impl crate::sealed::RegSpec for Vbrpabars_SPEC {
    type DataType = u16;
}

#[doc = "VBATT Backup Register Privilege Attribute Boundary Address Register for Secure Region"]
pub type Vbrpabars = crate::RegValueT<Vbrpabars_SPEC>;

impl Vbrpabars {
    #[doc = "Privilege Attribute Boundary Address for Secure Region"]
    #[inline(always)]
    pub fn pabas(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Vbrpabars_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Vbrpabars_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbrpabars {
    #[inline(always)]
    fn default() -> Vbrpabars {
        <crate::RegValueT<Vbrpabars_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbrpabarns_SPEC;
impl crate::sealed::RegSpec for Vbrpabarns_SPEC {
    type DataType = u16;
}

#[doc = "VBATT Backup Register Privilege Attribute Boundary Address Register for Non-secure Region"]
pub type Vbrpabarns = crate::RegValueT<Vbrpabarns_SPEC>;

impl Vbrpabarns {
    #[doc = "Privilege Attribute Boundary Address for Non-secure Region"]
    #[inline(always)]
    pub fn pabans(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Vbrpabarns_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Vbrpabarns_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbrpabarns {
    #[inline(always)]
    fn default() -> Vbrpabarns {
        <crate::RegValueT<Vbrpabarns_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Non-secure Attribute bit 0"]
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

    #[doc = "Non-secure Attribute bit 2"]
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

    #[doc = "Non-secure Attribute bit 3"]
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

    #[doc = "Non-secure Attribute bit 4"]
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

    #[doc = "Non-secure Attribute bit 5"]
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

    #[doc = "Non-secure Attribute bit 6"]
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

    #[doc = "Non-secure Attribute bit 7"]
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

    #[doc = "Non-secure Attribute bit 8"]
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

    #[doc = "Non-secure Attribute bit 9"]
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

    #[doc = "Non-secure Attribute bit 11"]
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

    #[doc = "Non-secure Attribute bit 12"]
    #[inline(always)]
    pub fn nonsec12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        cgfsar::Nonsec12,
        cgfsar::Nonsec12,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            cgfsar::Nonsec12,
            cgfsar::Nonsec12,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 13"]
    #[inline(always)]
    pub fn nonsec13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        cgfsar::Nonsec13,
        cgfsar::Nonsec13,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            cgfsar::Nonsec13,
            cgfsar::Nonsec13,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 16"]
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

    #[doc = "Non-secure Attribute bit 17"]
    #[inline(always)]
    pub fn nonsec17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        cgfsar::Nonsec17,
        cgfsar::Nonsec17,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            cgfsar::Nonsec17,
            cgfsar::Nonsec17,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 18"]
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

    #[doc = "Non-secure Attribute bit 19"]
    #[inline(always)]
    pub fn nonsec19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        cgfsar::Nonsec19,
        cgfsar::Nonsec19,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            cgfsar::Nonsec19,
            cgfsar::Nonsec19,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 20"]
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

    #[doc = "Non-secure Attribute bit 21"]
    #[inline(always)]
    pub fn nonsec21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        cgfsar::Nonsec21,
        cgfsar::Nonsec21,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            cgfsar::Nonsec21,
            cgfsar::Nonsec21,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 22"]
    #[inline(always)]
    pub fn nonsec22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        cgfsar::Nonsec22,
        cgfsar::Nonsec22,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            cgfsar::Nonsec22,
            cgfsar::Nonsec22,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 24"]
    #[inline(always)]
    pub fn nonsec24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        cgfsar::Nonsec24,
        cgfsar::Nonsec24,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            cgfsar::Nonsec24,
            cgfsar::Nonsec24,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 25"]
    #[inline(always)]
    pub fn nonsec25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        cgfsar::Nonsec25,
        cgfsar::Nonsec25,
        Cgfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            cgfsar::Nonsec25,
            cgfsar::Nonsec25,
            Cgfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 26"]
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

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, Cgfsar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,Cgfsar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cgfsar {
    #[inline(always)]
    fn default() -> Cgfsar {
        <crate::RegValueT<Cgfsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cgfsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec00_SPEC;
    pub type Nonsec00 = crate::EnumBitfieldStruct<u8, Nonsec00_SPEC>;
    impl Nonsec00 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec02_SPEC;
    pub type Nonsec02 = crate::EnumBitfieldStruct<u8, Nonsec02_SPEC>;
    impl Nonsec02 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec03_SPEC;
    pub type Nonsec03 = crate::EnumBitfieldStruct<u8, Nonsec03_SPEC>;
    impl Nonsec03 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec04_SPEC;
    pub type Nonsec04 = crate::EnumBitfieldStruct<u8, Nonsec04_SPEC>;
    impl Nonsec04 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec05_SPEC;
    pub type Nonsec05 = crate::EnumBitfieldStruct<u8, Nonsec05_SPEC>;
    impl Nonsec05 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec06_SPEC;
    pub type Nonsec06 = crate::EnumBitfieldStruct<u8, Nonsec06_SPEC>;
    impl Nonsec06 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec07_SPEC;
    pub type Nonsec07 = crate::EnumBitfieldStruct<u8, Nonsec07_SPEC>;
    impl Nonsec07 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec08_SPEC;
    pub type Nonsec08 = crate::EnumBitfieldStruct<u8, Nonsec08_SPEC>;
    impl Nonsec08 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec09_SPEC;
    pub type Nonsec09 = crate::EnumBitfieldStruct<u8, Nonsec09_SPEC>;
    impl Nonsec09 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec11_SPEC;
    pub type Nonsec11 = crate::EnumBitfieldStruct<u8, Nonsec11_SPEC>;
    impl Nonsec11 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec12_SPEC;
    pub type Nonsec12 = crate::EnumBitfieldStruct<u8, Nonsec12_SPEC>;
    impl Nonsec12 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec13_SPEC;
    pub type Nonsec13 = crate::EnumBitfieldStruct<u8, Nonsec13_SPEC>;
    impl Nonsec13 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec16_SPEC;
    pub type Nonsec16 = crate::EnumBitfieldStruct<u8, Nonsec16_SPEC>;
    impl Nonsec16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec17_SPEC;
    pub type Nonsec17 = crate::EnumBitfieldStruct<u8, Nonsec17_SPEC>;
    impl Nonsec17 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec18_SPEC;
    pub type Nonsec18 = crate::EnumBitfieldStruct<u8, Nonsec18_SPEC>;
    impl Nonsec18 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec19_SPEC;
    pub type Nonsec19 = crate::EnumBitfieldStruct<u8, Nonsec19_SPEC>;
    impl Nonsec19 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec20_SPEC;
    pub type Nonsec20 = crate::EnumBitfieldStruct<u8, Nonsec20_SPEC>;
    impl Nonsec20 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec21_SPEC;
    pub type Nonsec21 = crate::EnumBitfieldStruct<u8, Nonsec21_SPEC>;
    impl Nonsec21 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec22_SPEC;
    pub type Nonsec22 = crate::EnumBitfieldStruct<u8, Nonsec22_SPEC>;
    impl Nonsec22 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec24_SPEC;
    pub type Nonsec24 = crate::EnumBitfieldStruct<u8, Nonsec24_SPEC>;
    impl Nonsec24 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec25_SPEC;
    pub type Nonsec25 = crate::EnumBitfieldStruct<u8, Nonsec25_SPEC>;
    impl Nonsec25 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec26_SPEC;
    pub type Nonsec26 = crate::EnumBitfieldStruct<u8, Nonsec26_SPEC>;
    impl Nonsec26 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
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
    #[doc = "Non-secure Attribute bit 0"]
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

    #[doc = "Non-secure Attribute bit 1"]
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

    #[doc = "Non-secure Attribute bit 2"]
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

    #[doc = "Non-secure Attribute bit 3"]
    #[inline(always)]
    pub fn nonsec3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rstsar::Nonsec3,
        rstsar::Nonsec3,
        Rstsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rstsar::Nonsec3,
            rstsar::Nonsec3,
            Rstsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000000000000000. The write value should be 0000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfffffff, 1, 0, u32, u32, Rstsar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfffffff,1,0,u32,u32,Rstsar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rstsar {
    #[inline(always)]
    fn default() -> Rstsar {
        <crate::RegValueT<Rstsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec0_SPEC;
    pub type Nonsec0 = crate::EnumBitfieldStruct<u8, Nonsec0_SPEC>;
    impl Nonsec0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec1_SPEC;
    pub type Nonsec1 = crate::EnumBitfieldStruct<u8, Nonsec1_SPEC>;
    impl Nonsec1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec2_SPEC;
    pub type Nonsec2 = crate::EnumBitfieldStruct<u8, Nonsec2_SPEC>;
    impl Nonsec2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec3_SPEC;
    pub type Nonsec3 = crate::EnumBitfieldStruct<u8, Nonsec3_SPEC>;
    impl Nonsec3 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
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
    #[doc = "Non-secure Attribute bit 00"]
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

    #[doc = "Non-secure Attribute bit 01"]
    #[inline(always)]
    pub fn nonsec1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        lpmsar::Nonsec1,
        lpmsar::Nonsec1,
        Lpmsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            lpmsar::Nonsec1,
            lpmsar::Nonsec1,
            Lpmsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 02"]
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

    #[doc = "Non-secure Attribute bit 03"]
    #[inline(always)]
    pub fn nonsec3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        lpmsar::Nonsec3,
        lpmsar::Nonsec3,
        Lpmsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            lpmsar::Nonsec3,
            lpmsar::Nonsec3,
            Lpmsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 08"]
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

    #[doc = "Non-secure Attribute bit 16"]
    #[inline(always)]
    pub fn nonsec16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        lpmsar::Nonsec16,
        lpmsar::Nonsec16,
        Lpmsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            lpmsar::Nonsec16,
            lpmsar::Nonsec16,
            Lpmsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 17"]
    #[inline(always)]
    pub fn nonsec17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        lpmsar::Nonsec17,
        lpmsar::Nonsec17,
        Lpmsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            lpmsar::Nonsec17,
            lpmsar::Nonsec17,
            Lpmsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 18"]
    #[inline(always)]
    pub fn nonsec18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        lpmsar::Nonsec18,
        lpmsar::Nonsec18,
        Lpmsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            lpmsar::Nonsec18,
            lpmsar::Nonsec18,
            Lpmsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 19"]
    #[inline(always)]
    pub fn nonsec19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        lpmsar::Nonsec19,
        lpmsar::Nonsec19,
        Lpmsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            lpmsar::Nonsec19,
            lpmsar::Nonsec19,
            Lpmsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 21"]
    #[inline(always)]
    pub fn nonsec21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        lpmsar::Nonsec21,
        lpmsar::Nonsec21,
        Lpmsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            lpmsar::Nonsec21,
            lpmsar::Nonsec21,
            Lpmsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<22, 0x3ff, 1, 0, u16, u16, Lpmsar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3ff,1,0,u16,u16,Lpmsar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lpmsar {
    #[inline(always)]
    fn default() -> Lpmsar {
        <crate::RegValueT<Lpmsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lpmsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec0_SPEC;
    pub type Nonsec0 = crate::EnumBitfieldStruct<u8, Nonsec0_SPEC>;
    impl Nonsec0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec1_SPEC;
    pub type Nonsec1 = crate::EnumBitfieldStruct<u8, Nonsec1_SPEC>;
    impl Nonsec1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec2_SPEC;
    pub type Nonsec2 = crate::EnumBitfieldStruct<u8, Nonsec2_SPEC>;
    impl Nonsec2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec3_SPEC;
    pub type Nonsec3 = crate::EnumBitfieldStruct<u8, Nonsec3_SPEC>;
    impl Nonsec3 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec8_SPEC;
    pub type Nonsec8 = crate::EnumBitfieldStruct<u8, Nonsec8_SPEC>;
    impl Nonsec8 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec16_SPEC;
    pub type Nonsec16 = crate::EnumBitfieldStruct<u8, Nonsec16_SPEC>;
    impl Nonsec16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec17_SPEC;
    pub type Nonsec17 = crate::EnumBitfieldStruct<u8, Nonsec17_SPEC>;
    impl Nonsec17 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec18_SPEC;
    pub type Nonsec18 = crate::EnumBitfieldStruct<u8, Nonsec18_SPEC>;
    impl Nonsec18 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec19_SPEC;
    pub type Nonsec19 = crate::EnumBitfieldStruct<u8, Nonsec19_SPEC>;
    impl Nonsec19 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec21_SPEC;
    pub type Nonsec21 = crate::EnumBitfieldStruct<u8, Nonsec21_SPEC>;
    impl Nonsec21 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pvdsar_SPEC;
impl crate::sealed::RegSpec for Pvdsar_SPEC {
    type DataType = u32;
}

#[doc = "Programable Voltage Detection Security Attribution Register"]
pub type Pvdsar = crate::RegValueT<Pvdsar_SPEC>;

impl Pvdsar {
    #[doc = "Non-secure Attribute bit 0"]
    #[inline(always)]
    pub fn nonsec0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pvdsar::Nonsec0,
        pvdsar::Nonsec0,
        Pvdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pvdsar::Nonsec0,
            pvdsar::Nonsec0,
            Pvdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 1"]
    #[inline(always)]
    pub fn nonsec1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pvdsar::Nonsec1,
        pvdsar::Nonsec1,
        Pvdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pvdsar::Nonsec1,
            pvdsar::Nonsec1,
            Pvdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000000000000000000. The write value should be 000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3fffffff, 1, 0, u32, u32, Pvdsar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3fffffff,1,0,u32,u32,Pvdsar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pvdsar {
    #[inline(always)]
    fn default() -> Pvdsar {
        <crate::RegValueT<Pvdsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pvdsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec0_SPEC;
    pub type Nonsec0 = crate::EnumBitfieldStruct<u8, Nonsec0_SPEC>;
    impl Nonsec0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec1_SPEC;
    pub type Nonsec1 = crate::EnumBitfieldStruct<u8, Nonsec1_SPEC>;
    impl Nonsec1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
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
    #[doc = "Non-secure Attribute bit 0"]
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

    #[doc = "Non-secure Attribute bit 1"]
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

    #[doc = "Non-secure Attribute bit 2"]
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

    #[doc = "Non-secure Attribute bit 3"]
    #[inline(always)]
    pub fn nonsec3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        bbfsar::Nonsec3,
        bbfsar::Nonsec3,
        Bbfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            bbfsar::Nonsec3,
            bbfsar::Nonsec3,
            Bbfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 4"]
    #[inline(always)]
    pub fn nonsec4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        bbfsar::Nonsec4,
        bbfsar::Nonsec4,
        Bbfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            bbfsar::Nonsec4,
            bbfsar::Nonsec4,
            Bbfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000000000000000. The write value should be 000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, u32, Bbfsar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7ffffff,1,0,u32,u32,Bbfsar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bbfsar {
    #[inline(always)]
    fn default() -> Bbfsar {
        <crate::RegValueT<Bbfsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bbfsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec0_SPEC;
    pub type Nonsec0 = crate::EnumBitfieldStruct<u8, Nonsec0_SPEC>;
    impl Nonsec0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec1_SPEC;
    pub type Nonsec1 = crate::EnumBitfieldStruct<u8, Nonsec1_SPEC>;
    impl Nonsec1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec2_SPEC;
    pub type Nonsec2 = crate::EnumBitfieldStruct<u8, Nonsec2_SPEC>;
    impl Nonsec2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec3_SPEC;
    pub type Nonsec3 = crate::EnumBitfieldStruct<u8, Nonsec3_SPEC>;
    impl Nonsec3 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec4_SPEC;
    pub type Nonsec4 = crate::EnumBitfieldStruct<u8, Nonsec4_SPEC>;
    impl Nonsec4 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pgcsar_SPEC;
impl crate::sealed::RegSpec for Pgcsar_SPEC {
    type DataType = u32;
}

#[doc = "Power Gating Control Security Attribution Register"]
pub type Pgcsar = crate::RegValueT<Pgcsar_SPEC>;

impl Pgcsar {
    #[doc = "Non-secure Attribute bit 01"]
    #[inline(always)]
    pub fn nonsec1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pgcsar::Nonsec1,
        pgcsar::Nonsec1,
        Pgcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pgcsar::Nonsec1,
            pgcsar::Nonsec1,
            Pgcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Non-secure Attribute bit 02"]
    #[inline(always)]
    pub fn nonsec2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pgcsar::Nonsec2,
        pgcsar::Nonsec2,
        Pgcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pgcsar::Nonsec2,
            pgcsar::Nonsec2,
            Pgcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Pgcsar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Pgcsar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pgcsar {
    #[inline(always)]
    fn default() -> Pgcsar {
        <crate::RegValueT<Pgcsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pgcsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec1_SPEC;
    pub type Nonsec1 = crate::EnumBitfieldStruct<u8, Nonsec1_SPEC>;
    impl Nonsec1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nonsec2_SPEC;
    pub type Nonsec2 = crate::EnumBitfieldStruct<u8, Nonsec2_SPEC>;
    impl Nonsec2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpfsar_SPEC;
impl crate::sealed::RegSpec for Dpfsar_SPEC {
    type DataType = u32;
}

#[doc = "Deep Standby Interrupt Factor Security Attribution Register"]
pub type Dpfsar = crate::RegValueT<Dpfsar_SPEC>;

impl Dpfsar {
    #[doc = "Deep Standby Interrupt Factor Security Attribute bit 16"]
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

    #[doc = "Deep Standby Interrupt Factor Security Attribute bit 17"]
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

    #[doc = "Deep Standby Interrupt Factor Security Attribute bit 18"]
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

    #[doc = "Deep Standby Interrupt Factor Security Attribute bit 19"]
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

    #[doc = "Deep Standby Interrupt Factor Security Attribute bit 20"]
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

    #[doc = "Deep Standby Interrupt Factor Security Attribute bit 24"]
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

    #[doc = "Deep Standby Interrupt Factor Security Attribute bit 25"]
    #[inline(always)]
    pub fn dpfsa25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        dpfsar::Dpfsa25,
        dpfsar::Dpfsa25,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            dpfsar::Dpfsa25,
            dpfsar::Dpfsa25,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Standby Interrupt Factor Security Attribute bit 26"]
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

    #[doc = "Deep Standby Interrupt Factor Security Attribute bit 27"]
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

    #[doc = "Deep Standby Interrupt Factor Security Attribute bit 29"]
    #[inline(always)]
    pub fn dpfsa29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        dpfsar::Dpfsa29,
        dpfsar::Dpfsa29,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            dpfsar::Dpfsa29,
            dpfsar::Dpfsa29,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Dpfsar_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Dpfsar_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Deep Standby Interrupt Factor Security Attribute bit 31"]
    #[inline(always)]
    pub fn dpfsa31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        dpfsar::Dpfsa31,
        dpfsar::Dpfsa31,
        Dpfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            dpfsar::Dpfsa31,
            dpfsar::Dpfsa31,
            Dpfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpfsar {
    #[inline(always)]
    fn default() -> Dpfsar {
        <crate::RegValueT<Dpfsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpfsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa16_SPEC;
    pub type Dpfsa16 = crate::EnumBitfieldStruct<u8, Dpfsa16_SPEC>;
    impl Dpfsa16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa17_SPEC;
    pub type Dpfsa17 = crate::EnumBitfieldStruct<u8, Dpfsa17_SPEC>;
    impl Dpfsa17 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa18_SPEC;
    pub type Dpfsa18 = crate::EnumBitfieldStruct<u8, Dpfsa18_SPEC>;
    impl Dpfsa18 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa19_SPEC;
    pub type Dpfsa19 = crate::EnumBitfieldStruct<u8, Dpfsa19_SPEC>;
    impl Dpfsa19 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa20_SPEC;
    pub type Dpfsa20 = crate::EnumBitfieldStruct<u8, Dpfsa20_SPEC>;
    impl Dpfsa20 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa24_SPEC;
    pub type Dpfsa24 = crate::EnumBitfieldStruct<u8, Dpfsa24_SPEC>;
    impl Dpfsa24 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa25_SPEC;
    pub type Dpfsa25 = crate::EnumBitfieldStruct<u8, Dpfsa25_SPEC>;
    impl Dpfsa25 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa26_SPEC;
    pub type Dpfsa26 = crate::EnumBitfieldStruct<u8, Dpfsa26_SPEC>;
    impl Dpfsa26 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa27_SPEC;
    pub type Dpfsa27 = crate::EnumBitfieldStruct<u8, Dpfsa27_SPEC>;
    impl Dpfsa27 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa29_SPEC;
    pub type Dpfsa29 = crate::EnumBitfieldStruct<u8, Dpfsa29_SPEC>;
    impl Dpfsa29 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpfsa31_SPEC;
    pub type Dpfsa31 = crate::EnumBitfieldStruct<u8, Dpfsa31_SPEC>;
    impl Dpfsa31 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rscsar_SPEC;
impl crate::sealed::RegSpec for Rscsar_SPEC {
    type DataType = u32;
}

#[doc = "RAM Standby Control Security Attribution Register"]
pub type Rscsar = crate::RegValueT<Rscsar_SPEC>;

impl Rscsar {
    #[doc = "RAM Standby Control Security Attribute bit 00"]
    #[inline(always)]
    pub fn rscsa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rscsar::Rscsa0,
        rscsar::Rscsa0,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rscsar::Rscsa0,
            rscsar::Rscsa0,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 01"]
    #[inline(always)]
    pub fn rscsa1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rscsar::Rscsa1,
        rscsar::Rscsa1,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rscsar::Rscsa1,
            rscsar::Rscsa1,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 02"]
    #[inline(always)]
    pub fn rscsa2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rscsar::Rscsa2,
        rscsar::Rscsa2,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rscsar::Rscsa2,
            rscsar::Rscsa2,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 03"]
    #[inline(always)]
    pub fn rscsa3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rscsar::Rscsa3,
        rscsar::Rscsa3,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rscsar::Rscsa3,
            rscsar::Rscsa3,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 04"]
    #[inline(always)]
    pub fn rscsa4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        rscsar::Rscsa4,
        rscsar::Rscsa4,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            rscsar::Rscsa4,
            rscsar::Rscsa4,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 05"]
    #[inline(always)]
    pub fn rscsa5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        rscsar::Rscsa5,
        rscsar::Rscsa5,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            rscsar::Rscsa5,
            rscsar::Rscsa5,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 06"]
    #[inline(always)]
    pub fn rscsa6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        rscsar::Rscsa6,
        rscsar::Rscsa6,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            rscsar::Rscsa6,
            rscsar::Rscsa6,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 07"]
    #[inline(always)]
    pub fn rscsa7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rscsar::Rscsa7,
        rscsar::Rscsa7,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rscsar::Rscsa7,
            rscsar::Rscsa7,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 08"]
    #[inline(always)]
    pub fn rscsa8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        rscsar::Rscsa8,
        rscsar::Rscsa8,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            rscsar::Rscsa8,
            rscsar::Rscsa8,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 09"]
    #[inline(always)]
    pub fn rscsa9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        rscsar::Rscsa9,
        rscsar::Rscsa9,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            rscsar::Rscsa9,
            rscsar::Rscsa9,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 10"]
    #[inline(always)]
    pub fn rscsa10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        rscsar::Rscsa10,
        rscsar::Rscsa10,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            rscsar::Rscsa10,
            rscsar::Rscsa10,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 11"]
    #[inline(always)]
    pub fn rscsa11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        rscsar::Rscsa11,
        rscsar::Rscsa11,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            rscsar::Rscsa11,
            rscsar::Rscsa11,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 12"]
    #[inline(always)]
    pub fn rscsa12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        rscsar::Rscsa12,
        rscsar::Rscsa12,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            rscsar::Rscsa12,
            rscsar::Rscsa12,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 13"]
    #[inline(always)]
    pub fn rscsa13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        rscsar::Rscsa13,
        rscsar::Rscsa13,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            rscsar::Rscsa13,
            rscsar::Rscsa13,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 14"]
    #[inline(always)]
    pub fn rscsa14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        rscsar::Rscsa14,
        rscsar::Rscsa14,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            rscsar::Rscsa14,
            rscsar::Rscsa14,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 16"]
    #[inline(always)]
    pub fn rscsa16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        rscsar::Rscsa16,
        rscsar::Rscsa16,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            rscsar::Rscsa16,
            rscsar::Rscsa16,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Standby Control Security Attribute bit 17"]
    #[inline(always)]
    pub fn rscsa17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        rscsar::Rscsa17,
        rscsar::Rscsa17,
        Rscsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            rscsar::Rscsa17,
            rscsar::Rscsa17,
            Rscsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000000000. The write value should be 00000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<18, 0x3fff, 1, 0, u16, u16, Rscsar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3fff,1,0,u16,u16,Rscsar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rscsar {
    #[inline(always)]
    fn default() -> Rscsar {
        <crate::RegValueT<Rscsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rscsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa0_SPEC;
    pub type Rscsa0 = crate::EnumBitfieldStruct<u8, Rscsa0_SPEC>;
    impl Rscsa0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa1_SPEC;
    pub type Rscsa1 = crate::EnumBitfieldStruct<u8, Rscsa1_SPEC>;
    impl Rscsa1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa2_SPEC;
    pub type Rscsa2 = crate::EnumBitfieldStruct<u8, Rscsa2_SPEC>;
    impl Rscsa2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa3_SPEC;
    pub type Rscsa3 = crate::EnumBitfieldStruct<u8, Rscsa3_SPEC>;
    impl Rscsa3 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa4_SPEC;
    pub type Rscsa4 = crate::EnumBitfieldStruct<u8, Rscsa4_SPEC>;
    impl Rscsa4 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa5_SPEC;
    pub type Rscsa5 = crate::EnumBitfieldStruct<u8, Rscsa5_SPEC>;
    impl Rscsa5 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa6_SPEC;
    pub type Rscsa6 = crate::EnumBitfieldStruct<u8, Rscsa6_SPEC>;
    impl Rscsa6 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa7_SPEC;
    pub type Rscsa7 = crate::EnumBitfieldStruct<u8, Rscsa7_SPEC>;
    impl Rscsa7 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa8_SPEC;
    pub type Rscsa8 = crate::EnumBitfieldStruct<u8, Rscsa8_SPEC>;
    impl Rscsa8 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa9_SPEC;
    pub type Rscsa9 = crate::EnumBitfieldStruct<u8, Rscsa9_SPEC>;
    impl Rscsa9 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa10_SPEC;
    pub type Rscsa10 = crate::EnumBitfieldStruct<u8, Rscsa10_SPEC>;
    impl Rscsa10 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa11_SPEC;
    pub type Rscsa11 = crate::EnumBitfieldStruct<u8, Rscsa11_SPEC>;
    impl Rscsa11 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa12_SPEC;
    pub type Rscsa12 = crate::EnumBitfieldStruct<u8, Rscsa12_SPEC>;
    impl Rscsa12 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa13_SPEC;
    pub type Rscsa13 = crate::EnumBitfieldStruct<u8, Rscsa13_SPEC>;
    impl Rscsa13 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa14_SPEC;
    pub type Rscsa14 = crate::EnumBitfieldStruct<u8, Rscsa14_SPEC>;
    impl Rscsa14 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa16_SPEC;
    pub type Rscsa16 = crate::EnumBitfieldStruct<u8, Rscsa16_SPEC>;
    impl Rscsa16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscsa17_SPEC;
    pub type Rscsa17 = crate::EnumBitfieldStruct<u8, Rscsa17_SPEC>;
    impl Rscsa17 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrcrS_SPEC;
impl crate::sealed::RegSpec for PrcrS_SPEC {
    type DataType = u16;
}

#[doc = "Protect Register for Secure Register"]
pub type PrcrS = crate::RegValueT<PrcrS_SPEC>;

impl PrcrS {
    #[doc = "Enables writing to the registers related to the clock generation circuit."]
    #[inline(always)]
    pub fn prc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        prcr_s::Prc0,
        prcr_s::Prc0,
        PrcrS_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            prcr_s::Prc0,
            prcr_s::Prc0,
            PrcrS_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enables writing to the registers related to the operating modes, the low power modes, and the battery backup function."]
    #[inline(always)]
    pub fn prc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        prcr_s::Prc1,
        prcr_s::Prc1,
        PrcrS_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            prcr_s::Prc1,
            prcr_s::Prc1,
            PrcrS_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enables writing to the registers related to the PVD."]
    #[inline(always)]
    pub fn prc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        prcr_s::Prc3,
        prcr_s::Prc3,
        PrcrS_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            prcr_s::Prc3,
            prcr_s::Prc3,
            PrcrS_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enables writing to the registers related to the security and privilege setting registers."]
    #[inline(always)]
    pub fn prc4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        prcr_s::Prc4,
        prcr_s::Prc4,
        PrcrS_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            prcr_s::Prc4,
            prcr_s::Prc4,
            PrcrS_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enables writing to the registers related the reset control."]
    #[inline(always)]
    pub fn prc5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        prcr_s::Prc5,
        prcr_s::Prc5,
        PrcrS_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            prcr_s::Prc5,
            prcr_s::Prc5,
            PrcrS_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PrcrS_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, PrcrS_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PRC Key Code"]
    #[inline(always)]
    pub fn prkey(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        prcr_s::Prkey,
        prcr_s::Prkey,
        PrcrS_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            prcr_s::Prkey,
            prcr_s::Prkey,
            PrcrS_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PrcrS {
    #[inline(always)]
    fn default() -> PrcrS {
        <crate::RegValueT<PrcrS_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod prcr_s {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc0_SPEC;
    pub type Prc0 = crate::EnumBitfieldStruct<u8, Prc0_SPEC>;
    impl Prc0 {
        #[doc = "Write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc1_SPEC;
    pub type Prc1 = crate::EnumBitfieldStruct<u8, Prc1_SPEC>;
    impl Prc1 {
        #[doc = "Write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc3_SPEC;
    pub type Prc3 = crate::EnumBitfieldStruct<u8, Prc3_SPEC>;
    impl Prc3 {
        #[doc = "Write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc4_SPEC;
    pub type Prc4 = crate::EnumBitfieldStruct<u8, Prc4_SPEC>;
    impl Prc4 {
        #[doc = "Write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc5_SPEC;
    pub type Prc5 = crate::EnumBitfieldStruct<u8, Prc5_SPEC>;
    impl Prc5 {
        #[doc = "Write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prkey_SPEC;
    pub type Prkey = crate::EnumBitfieldStruct<u8, Prkey_SPEC>;
    impl Prkey {
        #[doc = "Enables writing to the PRCR_S register."]
        pub const _0_X_A_5: Self = Self::new(165);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrcrNs_SPEC;
impl crate::sealed::RegSpec for PrcrNs_SPEC {
    type DataType = u16;
}

#[doc = "Protect Register for Non-secure Register"]
pub type PrcrNs = crate::RegValueT<PrcrNs_SPEC>;

impl PrcrNs {
    #[doc = "Enables writing to the registers related to the clock generation circuit."]
    #[inline(always)]
    pub fn prc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        prcr_ns::Prc0,
        prcr_ns::Prc0,
        PrcrNs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            prcr_ns::Prc0,
            prcr_ns::Prc0,
            PrcrNs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enables writing to the registers related to the operating modes, the low power modes, and the battery backup function."]
    #[inline(always)]
    pub fn prc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        prcr_ns::Prc1,
        prcr_ns::Prc1,
        PrcrNs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            prcr_ns::Prc1,
            prcr_ns::Prc1,
            PrcrNs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enables writing to the registers related to the PVD."]
    #[inline(always)]
    pub fn prc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        prcr_ns::Prc3,
        prcr_ns::Prc3,
        PrcrNs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            prcr_ns::Prc3,
            prcr_ns::Prc3,
            PrcrNs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enables writing to the registers related to the privilege setting registers."]
    #[inline(always)]
    pub fn prc4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        prcr_ns::Prc4,
        prcr_ns::Prc4,
        PrcrNs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            prcr_ns::Prc4,
            prcr_ns::Prc4,
            PrcrNs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PrcrNs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, PrcrNs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PRC Key Code"]
    #[inline(always)]
    pub fn prkey(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        prcr_ns::Prkey,
        prcr_ns::Prkey,
        PrcrNs_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            prcr_ns::Prkey,
            prcr_ns::Prkey,
            PrcrNs_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PrcrNs {
    #[inline(always)]
    fn default() -> PrcrNs {
        <crate::RegValueT<PrcrNs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod prcr_ns {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc0_SPEC;
    pub type Prc0 = crate::EnumBitfieldStruct<u8, Prc0_SPEC>;
    impl Prc0 {
        #[doc = "Write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc1_SPEC;
    pub type Prc1 = crate::EnumBitfieldStruct<u8, Prc1_SPEC>;
    impl Prc1 {
        #[doc = "Write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc3_SPEC;
    pub type Prc3 = crate::EnumBitfieldStruct<u8, Prc3_SPEC>;
    impl Prc3 {
        #[doc = "Write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc4_SPEC;
    pub type Prc4 = crate::EnumBitfieldStruct<u8, Prc4_SPEC>;
    impl Prc4 {
        #[doc = "Write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prkey_SPEC;
    pub type Prkey = crate::EnumBitfieldStruct<u8, Prkey_SPEC>;
    impl Prkey {
        #[doc = "Enables writing to the PRCR_NS register."]
        pub const _0_X_A_5: Self = Self::new(165);
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Lococr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Lococr_SPEC,crate::common::RW>::from_register(self,0)
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
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        locoutcr::Locoutrm,
        locoutcr::Locoutrm,
        Locoutcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            locoutcr::Locoutrm,
            locoutcr::Locoutrm,
            Locoutcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Locoutcr {
    #[inline(always)]
    fn default() -> Locoutcr {
        <crate::RegValueT<Locoutcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod locoutcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Locoutrm_SPEC;
    pub type Locoutrm = crate::EnumBitfieldStruct<u8, Locoutrm_SPEC>;
    impl Locoutrm {
        #[doc = "-128"]
        pub const _1000_0000: Self = Self::new(128);

        #[doc = "-127"]
        pub const _1000_0001: Self = Self::new(129);

        #[doc = "-126"]
        pub const _1000_0010: Self = Self::new(130);

        #[doc = "-1"]
        pub const _1111_1111: Self = Self::new(255);

        #[doc = "Center Code"]
        pub const _0000_0000: Self = Self::new(0);

        #[doc = "+1"]
        pub const _0000_0001: Self = Self::new(1);

        #[doc = "+125"]
        pub const _0111_1101: Self = Self::new(125);

        #[doc = "+126"]
        pub const _0111_1110: Self = Self::new(126);

        #[doc = "+127"]
        pub const _0111_1111: Self = Self::new(127);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsbycr_SPEC;
impl crate::sealed::RegSpec for Dpsbycr_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Control Register"]
pub type Dpsbycr = crate::RegValueT<Dpsbycr_SPEC>;

impl Dpsbycr {
    #[doc = "DCDC SSMODE"]
    #[inline(always)]
    pub fn dcssmode(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsbycr::Dcssmode,
        dpsbycr::Dcssmode,
        Dpsbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsbycr::Dcssmode,
            dpsbycr::Dcssmode,
            Dpsbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Standby RAM Retention"]
    #[inline(always)]
    pub fn srkeep(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsbycr::Srkeep,
        dpsbycr::Srkeep,
        Dpsbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsbycr::Srkeep,
            dpsbycr::Srkeep,
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dpsbycr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Dpsbycr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Dpsbycr {
    #[inline(always)]
    fn default() -> Dpsbycr {
        <crate::RegValueT<Dpsbycr_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod dpsbycr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcssmode_SPEC;
    pub type Dcssmode = crate::EnumBitfieldStruct<u8, Dcssmode_SPEC>;
    impl Dcssmode {
        #[doc = "When the Deep Software Standby mode is canceled, the time required to recover is the standard time."]
        pub const _0: Self = Self::new(0);

        #[doc = "When the Deep Software Standby mode is canceled, the time required to recover is shortened."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Srkeep_SPEC;
    pub type Srkeep = crate::EnumBitfieldStruct<u8, Srkeep_SPEC>;
    impl Srkeep {
        #[doc = "When entering the Software Standby mode or the Deep Software Standby mode, the contents of Standby RAM are not kept."]
        pub const _0: Self = Self::new(0);

        #[doc = "When entering the Software Standby mode or the Deep Software Standby mode 1, the contents of Standby RAM are kept."]
        pub const _1: Self = Self::new(1);
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpswcr_SPEC;
impl crate::sealed::RegSpec for Dpswcr_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Wait Control Register"]
pub type Dpswcr = crate::RegValueT<Dpswcr_SPEC>;

impl Dpswcr {
    #[doc = "Deep Software Wait Standby Time Setting Bit"]
    #[inline(always)]
    pub fn wtsts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        dpswcr::Wtsts,
        dpswcr::Wtsts,
        Dpswcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
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
        <crate::RegValueT<Dpswcr_SPEC> as RegisterValue<_>>::new(11)
    }
}
pub mod dpswcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wtsts_SPEC;
    pub type Wtsts = crate::EnumBitfieldStruct<u8, Wtsts_SPEC>;
    impl Wtsts {
        #[doc = "Wait cycle for fast recovery"]
        pub const _0_X_0_B: Self = Self::new(11);

        #[doc = "Wait cycle for slow recovery"]
        pub const _0_X_9_A: Self = Self::new(154);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsier0_SPEC;
impl crate::sealed::RegSpec for Dpsier0_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Enable Register 0"]
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

    #[doc = "IRQ2-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq2e(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsier0::Dirq2E,
        dpsier0::Dirq2E,
        Dpsier0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsier0::Dirq2E,
            dpsier0::Dirq2E,
            Dpsier0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ3-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq3e(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsier0::Dirq3E,
        dpsier0::Dirq3E,
        Dpsier0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsier0::Dirq3E,
            dpsier0::Dirq3E,
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
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq1E_SPEC;
    pub type Dirq1E = crate::EnumBitfieldStruct<u8, Dirq1E_SPEC>;
    impl Dirq1E {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq2E_SPEC;
    pub type Dirq2E = crate::EnumBitfieldStruct<u8, Dirq2E_SPEC>;
    impl Dirq2E {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq3E_SPEC;
    pub type Dirq3E = crate::EnumBitfieldStruct<u8, Dirq3E_SPEC>;
    impl Dirq3E {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq4E_SPEC;
    pub type Dirq4E = crate::EnumBitfieldStruct<u8, Dirq4E_SPEC>;
    impl Dirq4E {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq5E_SPEC;
    pub type Dirq5E = crate::EnumBitfieldStruct<u8, Dirq5E_SPEC>;
    impl Dirq5E {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq6E_SPEC;
    pub type Dirq6E = crate::EnumBitfieldStruct<u8, Dirq6E_SPEC>;
    impl Dirq6E {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq7E_SPEC;
    pub type Dirq7E = crate::EnumBitfieldStruct<u8, Dirq7E_SPEC>;
    impl Dirq7E {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsier1_SPEC;
impl crate::sealed::RegSpec for Dpsier1_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Enable Register 1"]
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

    #[doc = "IRQ10-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq10e(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsier1::Dirq10E,
        dpsier1::Dirq10E,
        Dpsier1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsier1::Dirq10E,
            dpsier1::Dirq10E,
            Dpsier1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ11-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq11e(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsier1::Dirq11E,
        dpsier1::Dirq11E,
        Dpsier1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsier1::Dirq11E,
            dpsier1::Dirq11E,
            Dpsier1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ12-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq12e(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsier1::Dirq12E,
        dpsier1::Dirq12E,
        Dpsier1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsier1::Dirq12E,
            dpsier1::Dirq12E,
            Dpsier1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ13-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq13e(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dpsier1::Dirq13E,
        dpsier1::Dirq13E,
        Dpsier1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dpsier1::Dirq13E,
            dpsier1::Dirq13E,
            Dpsier1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ14-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq14e(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dpsier1::Dirq14E,
        dpsier1::Dirq14E,
        Dpsier1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dpsier1::Dirq14E,
            dpsier1::Dirq14E,
            Dpsier1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ15-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq15e(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpsier1::Dirq15E,
        dpsier1::Dirq15E,
        Dpsier1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpsier1::Dirq15E,
            dpsier1::Dirq15E,
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
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq9E_SPEC;
    pub type Dirq9E = crate::EnumBitfieldStruct<u8, Dirq9E_SPEC>;
    impl Dirq9E {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq10E_SPEC;
    pub type Dirq10E = crate::EnumBitfieldStruct<u8, Dirq10E_SPEC>;
    impl Dirq10E {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq11E_SPEC;
    pub type Dirq11E = crate::EnumBitfieldStruct<u8, Dirq11E_SPEC>;
    impl Dirq11E {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq12E_SPEC;
    pub type Dirq12E = crate::EnumBitfieldStruct<u8, Dirq12E_SPEC>;
    impl Dirq12E {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq13E_SPEC;
    pub type Dirq13E = crate::EnumBitfieldStruct<u8, Dirq13E_SPEC>;
    impl Dirq13E {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq14E_SPEC;
    pub type Dirq14E = crate::EnumBitfieldStruct<u8, Dirq14E_SPEC>;
    impl Dirq14E {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq15E_SPEC;
    pub type Dirq15E = crate::EnumBitfieldStruct<u8, Dirq15E_SPEC>;
    impl Dirq15E {
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

#[doc = "Deep Standby Interrupt Enable Register 2"]
pub type Dpsier2 = crate::RegValueT<Dpsier2_SPEC>;

impl Dpsier2 {
    #[doc = "PVD1 Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dpvd1ie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsier2::Dpvd1Ie,
        dpsier2::Dpvd1Ie,
        Dpsier2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsier2::Dpvd1Ie,
            dpsier2::Dpvd1Ie,
            Dpsier2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PVD2 Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dpvd2ie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsier2::Dpvd2Ie,
        dpsier2::Dpvd2Ie,
        Dpsier2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsier2::Dpvd2Ie,
            dpsier2::Dpvd2Ie,
            Dpsier2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Interval interrupt Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dtrtciie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsier2::Dtrtciie,
        dpsier2::Dtrtciie,
        Dpsier2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsier2::Dtrtciie,
            dpsier2::Dtrtciie,
            Dpsier2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Alarm interrupt Deep Standby Cancel Signal Enable"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dpsier2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Dpsier2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
    pub struct Dpvd1Ie_SPEC;
    pub type Dpvd1Ie = crate::EnumBitfieldStruct<u8, Dpvd1Ie_SPEC>;
    impl Dpvd1Ie {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpvd2Ie_SPEC;
    pub type Dpvd2Ie = crate::EnumBitfieldStruct<u8, Dpvd2Ie_SPEC>;
    impl Dpvd2Ie {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtrtciie_SPEC;
    pub type Dtrtciie = crate::EnumBitfieldStruct<u8, Dtrtciie_SPEC>;
    impl Dtrtciie {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drtcaie_SPEC;
    pub type Drtcaie = crate::EnumBitfieldStruct<u8, Drtcaie_SPEC>;
    impl Drtcaie {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dnmie_SPEC;
    pub type Dnmie = crate::EnumBitfieldStruct<u8, Dnmie_SPEC>;
    impl Dnmie {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsier3_SPEC;
impl crate::sealed::RegSpec for Dpsier3_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Enable Register 3"]
pub type Dpsier3 = crate::RegValueT<Dpsier3_SPEC>;

impl Dpsier3 {
    #[doc = "USBFS Suspend/Resume Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dusbfsie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsier3::Dusbfsie,
        dpsier3::Dusbfsie,
        Dpsier3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsier3::Dusbfsie,
            dpsier3::Dusbfsie,
            Dpsier3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USBHS Suspend/Resume Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dusbhsie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsier3::Dusbhsie,
        dpsier3::Dusbhsie,
        Dpsier3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsier3::Dusbhsie,
            dpsier3::Dusbhsie,
            Dpsier3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPT0 Overflow Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dulpt0ie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsier3::Dulpt0Ie,
        dpsier3::Dulpt0Ie,
        Dpsier3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsier3::Dulpt0Ie,
            dpsier3::Dulpt0Ie,
            Dpsier3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPT1 Overflow Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dulpt1ie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsier3::Dulpt1Ie,
        dpsier3::Dulpt1Ie,
        Dpsier3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsier3::Dulpt1Ie,
            dpsier3::Dulpt1Ie,
            Dpsier3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IWDT Overflow Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn diwdtie(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dpsier3::Diwdtie,
        dpsier3::Diwdtie,
        Dpsier3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dpsier3::Diwdtie,
            dpsier3::Diwdtie,
            Dpsier3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dpsier3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Dpsier3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "VBATT Tamper Detection Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dvbattadie(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpsier3::Dvbattadie,
        dpsier3::Dvbattadie,
        Dpsier3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpsier3::Dvbattadie,
            dpsier3::Dvbattadie,
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
    pub struct Dusbfsie_SPEC;
    pub type Dusbfsie = crate::EnumBitfieldStruct<u8, Dusbfsie_SPEC>;
    impl Dusbfsie {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dusbhsie_SPEC;
    pub type Dusbhsie = crate::EnumBitfieldStruct<u8, Dusbhsie_SPEC>;
    impl Dusbhsie {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dulpt0Ie_SPEC;
    pub type Dulpt0Ie = crate::EnumBitfieldStruct<u8, Dulpt0Ie_SPEC>;
    impl Dulpt0Ie {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dulpt1Ie_SPEC;
    pub type Dulpt1Ie = crate::EnumBitfieldStruct<u8, Dulpt1Ie_SPEC>;
    impl Dulpt1Ie {
        #[doc = "Cancelling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diwdtie_SPEC;
    pub type Diwdtie = crate::EnumBitfieldStruct<u8, Diwdtie_SPEC>;
    impl Diwdtie {
        #[doc = "Cancelling Deep Software Standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cancelling Deep Software Standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvbattadie_SPEC;
    pub type Dvbattadie = crate::EnumBitfieldStruct<u8, Dvbattadie_SPEC>;
    impl Dvbattadie {
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

#[doc = "Deep Standby Interrupt Flag Register 0"]
pub type Dpsifr0 = crate::RegValueT<Dpsifr0_SPEC>;

impl Dpsifr0 {
    #[doc = "IRQ0-DS Pin Deep Standby Cancel Flag"]
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

    #[doc = "IRQ1-DS Pin Deep Standby Cancel Flag"]
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

    #[doc = "IRQ2-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq2f(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsifr0::Dirq2F,
        dpsifr0::Dirq2F,
        Dpsifr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsifr0::Dirq2F,
            dpsifr0::Dirq2F,
            Dpsifr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ3-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq3f(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsifr0::Dirq3F,
        dpsifr0::Dirq3F,
        Dpsifr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsifr0::Dirq3F,
            dpsifr0::Dirq3F,
            Dpsifr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ4-DS Pin Deep Standby Cancel Flag"]
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

    #[doc = "IRQ5-DS Pin Deep Standby Cancel Flag"]
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

    #[doc = "IRQ6-DS Pin Deep Standby Cancel Flag"]
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

    #[doc = "IRQ7-DS Pin Deep Standby Cancel Flag"]
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
    pub struct Dirq2F_SPEC;
    pub type Dirq2F = crate::EnumBitfieldStruct<u8, Dirq2F_SPEC>;
    impl Dirq2F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq3F_SPEC;
    pub type Dirq3F = crate::EnumBitfieldStruct<u8, Dirq3F_SPEC>;
    impl Dirq3F {
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

#[doc = "Deep Standby Interrupt Flag Register 1"]
pub type Dpsifr1 = crate::RegValueT<Dpsifr1_SPEC>;

impl Dpsifr1 {
    #[doc = "IRQ8-DS Pin Deep Standby Cancel Flag"]
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

    #[doc = "IRQ9-DS Pin Deep Standby Cancel Flag"]
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

    #[doc = "IRQ10-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq10f(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsifr1::Dirq10F,
        dpsifr1::Dirq10F,
        Dpsifr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsifr1::Dirq10F,
            dpsifr1::Dirq10F,
            Dpsifr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ11-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq11f(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsifr1::Dirq11F,
        dpsifr1::Dirq11F,
        Dpsifr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsifr1::Dirq11F,
            dpsifr1::Dirq11F,
            Dpsifr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ12-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq12f(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsifr1::Dirq12F,
        dpsifr1::Dirq12F,
        Dpsifr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsifr1::Dirq12F,
            dpsifr1::Dirq12F,
            Dpsifr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ13-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq13f(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dpsifr1::Dirq13F,
        dpsifr1::Dirq13F,
        Dpsifr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dpsifr1::Dirq13F,
            dpsifr1::Dirq13F,
            Dpsifr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ14-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq14f(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dpsifr1::Dirq14F,
        dpsifr1::Dirq14F,
        Dpsifr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dpsifr1::Dirq14F,
            dpsifr1::Dirq14F,
            Dpsifr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ15-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq15f(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpsifr1::Dirq15F,
        dpsifr1::Dirq15F,
        Dpsifr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpsifr1::Dirq15F,
            dpsifr1::Dirq15F,
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq10F_SPEC;
    pub type Dirq10F = crate::EnumBitfieldStruct<u8, Dirq10F_SPEC>;
    impl Dirq10F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq11F_SPEC;
    pub type Dirq11F = crate::EnumBitfieldStruct<u8, Dirq11F_SPEC>;
    impl Dirq11F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq12F_SPEC;
    pub type Dirq12F = crate::EnumBitfieldStruct<u8, Dirq12F_SPEC>;
    impl Dirq12F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq13F_SPEC;
    pub type Dirq13F = crate::EnumBitfieldStruct<u8, Dirq13F_SPEC>;
    impl Dirq13F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq14F_SPEC;
    pub type Dirq14F = crate::EnumBitfieldStruct<u8, Dirq14F_SPEC>;
    impl Dirq14F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq15F_SPEC;
    pub type Dirq15F = crate::EnumBitfieldStruct<u8, Dirq15F_SPEC>;
    impl Dirq15F {
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

#[doc = "Deep Standby Interrupt Flag Register 2"]
pub type Dpsifr2 = crate::RegValueT<Dpsifr2_SPEC>;

impl Dpsifr2 {
    #[doc = "PVD1 Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dpvd1if(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsifr2::Dpvd1If,
        dpsifr2::Dpvd1If,
        Dpsifr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsifr2::Dpvd1If,
            dpsifr2::Dpvd1If,
            Dpsifr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PVD2 Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dpvd2if(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsifr2::Dpvd2If,
        dpsifr2::Dpvd2If,
        Dpsifr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsifr2::Dpvd2If,
            dpsifr2::Dpvd2If,
            Dpsifr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Interval interrupt Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dtrtciif(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsifr2::Dtrtciif,
        dpsifr2::Dtrtciif,
        Dpsifr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsifr2::Dtrtciif,
            dpsifr2::Dtrtciif,
            Dpsifr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Alarm interrupt Deep Standby Cancel Flag"]
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

    #[doc = "NMI Pin Deep Standby Cancel Flag"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dpsifr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Dpsifr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
    pub struct Dpvd1If_SPEC;
    pub type Dpvd1If = crate::EnumBitfieldStruct<u8, Dpvd1If_SPEC>;
    impl Dpvd1If {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpvd2If_SPEC;
    pub type Dpvd2If = crate::EnumBitfieldStruct<u8, Dpvd2If_SPEC>;
    impl Dpvd2If {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtrtciif_SPEC;
    pub type Dtrtciif = crate::EnumBitfieldStruct<u8, Dtrtciif_SPEC>;
    impl Dtrtciif {
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

#[doc = "Deep Standby Interrupt Flag Register 3"]
pub type Dpsifr3 = crate::RegValueT<Dpsifr3_SPEC>;

impl Dpsifr3 {
    #[doc = "USBFS Suspend/Resume Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dusbfsif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsifr3::Dusbfsif,
        dpsifr3::Dusbfsif,
        Dpsifr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsifr3::Dusbfsif,
            dpsifr3::Dusbfsif,
            Dpsifr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USBHS Suspend/Resume Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dusbhsif(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsifr3::Dusbhsif,
        dpsifr3::Dusbhsif,
        Dpsifr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsifr3::Dusbhsif,
            dpsifr3::Dusbhsif,
            Dpsifr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPT0 Overflow Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dulpt0if(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsifr3::Dulpt0If,
        dpsifr3::Dulpt0If,
        Dpsifr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsifr3::Dulpt0If,
            dpsifr3::Dulpt0If,
            Dpsifr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPT1 Overflow Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dulpt1if(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsifr3::Dulpt1If,
        dpsifr3::Dulpt1If,
        Dpsifr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsifr3::Dulpt1If,
            dpsifr3::Dulpt1If,
            Dpsifr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IWDT Overflow Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn diwdtif(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dpsifr3::Diwdtif,
        dpsifr3::Diwdtif,
        Dpsifr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dpsifr3::Diwdtif,
            dpsifr3::Diwdtif,
            Dpsifr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dpsifr3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Dpsifr3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "VBATT Tamper Detection Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dvbattadif(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpsifr3::Dvbattadif,
        dpsifr3::Dvbattadif,
        Dpsifr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpsifr3::Dvbattadif,
            dpsifr3::Dvbattadif,
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
    pub struct Dusbfsif_SPEC;
    pub type Dusbfsif = crate::EnumBitfieldStruct<u8, Dusbfsif_SPEC>;
    impl Dusbfsif {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dusbhsif_SPEC;
    pub type Dusbhsif = crate::EnumBitfieldStruct<u8, Dusbhsif_SPEC>;
    impl Dusbhsif {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dulpt0If_SPEC;
    pub type Dulpt0If = crate::EnumBitfieldStruct<u8, Dulpt0If_SPEC>;
    impl Dulpt0If {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dulpt1If_SPEC;
    pub type Dulpt1If = crate::EnumBitfieldStruct<u8, Dulpt1If_SPEC>;
    impl Dulpt1If {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diwdtif_SPEC;
    pub type Diwdtif = crate::EnumBitfieldStruct<u8, Diwdtif_SPEC>;
    impl Diwdtif {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvbattadif_SPEC;
    pub type Dvbattadif = crate::EnumBitfieldStruct<u8, Dvbattadif_SPEC>;
    impl Dvbattadif {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsiegr0_SPEC;
impl crate::sealed::RegSpec for Dpsiegr0_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Edge Register 0"]
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

    #[doc = "IRQ2-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq2eg(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsiegr0::Dirq2Eg,
        dpsiegr0::Dirq2Eg,
        Dpsiegr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsiegr0::Dirq2Eg,
            dpsiegr0::Dirq2Eg,
            Dpsiegr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ3-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq3eg(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsiegr0::Dirq3Eg,
        dpsiegr0::Dirq3Eg,
        Dpsiegr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsiegr0::Dirq3Eg,
            dpsiegr0::Dirq3Eg,
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
    pub struct Dirq2Eg_SPEC;
    pub type Dirq2Eg = crate::EnumBitfieldStruct<u8, Dirq2Eg_SPEC>;
    impl Dirq2Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq3Eg_SPEC;
    pub type Dirq3Eg = crate::EnumBitfieldStruct<u8, Dirq3Eg_SPEC>;
    impl Dirq3Eg {
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

#[doc = "Deep Standby Interrupt Edge Register 1"]
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

    #[doc = "IRQ10-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq10eg(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsiegr1::Dirq10Eg,
        dpsiegr1::Dirq10Eg,
        Dpsiegr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsiegr1::Dirq10Eg,
            dpsiegr1::Dirq10Eg,
            Dpsiegr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ11-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq11eg(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsiegr1::Dirq11Eg,
        dpsiegr1::Dirq11Eg,
        Dpsiegr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsiegr1::Dirq11Eg,
            dpsiegr1::Dirq11Eg,
            Dpsiegr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ12-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq12eg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsiegr1::Dirq12Eg,
        dpsiegr1::Dirq12Eg,
        Dpsiegr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsiegr1::Dirq12Eg,
            dpsiegr1::Dirq12Eg,
            Dpsiegr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ13-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq13eg(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dpsiegr1::Dirq13Eg,
        dpsiegr1::Dirq13Eg,
        Dpsiegr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dpsiegr1::Dirq13Eg,
            dpsiegr1::Dirq13Eg,
            Dpsiegr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ14-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq14eg(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dpsiegr1::Dirq14Eg,
        dpsiegr1::Dirq14Eg,
        Dpsiegr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dpsiegr1::Dirq14Eg,
            dpsiegr1::Dirq14Eg,
            Dpsiegr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ15-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq15eg(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpsiegr1::Dirq15Eg,
        dpsiegr1::Dirq15Eg,
        Dpsiegr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpsiegr1::Dirq15Eg,
            dpsiegr1::Dirq15Eg,
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
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq9Eg_SPEC;
    pub type Dirq9Eg = crate::EnumBitfieldStruct<u8, Dirq9Eg_SPEC>;
    impl Dirq9Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq10Eg_SPEC;
    pub type Dirq10Eg = crate::EnumBitfieldStruct<u8, Dirq10Eg_SPEC>;
    impl Dirq10Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq11Eg_SPEC;
    pub type Dirq11Eg = crate::EnumBitfieldStruct<u8, Dirq11Eg_SPEC>;
    impl Dirq11Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq12Eg_SPEC;
    pub type Dirq12Eg = crate::EnumBitfieldStruct<u8, Dirq12Eg_SPEC>;
    impl Dirq12Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq13Eg_SPEC;
    pub type Dirq13Eg = crate::EnumBitfieldStruct<u8, Dirq13Eg_SPEC>;
    impl Dirq13Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq14Eg_SPEC;
    pub type Dirq14Eg = crate::EnumBitfieldStruct<u8, Dirq14Eg_SPEC>;
    impl Dirq14Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq15Eg_SPEC;
    pub type Dirq15Eg = crate::EnumBitfieldStruct<u8, Dirq15Eg_SPEC>;
    impl Dirq15Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsiegr2_SPEC;
impl crate::sealed::RegSpec for Dpsiegr2_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Edge Register 2"]
pub type Dpsiegr2 = crate::RegValueT<Dpsiegr2_SPEC>;

impl Dpsiegr2 {
    #[doc = "PVD1 Edge Select"]
    #[inline(always)]
    pub fn dpvd1eg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsiegr2::Dpvd1Eg,
        dpsiegr2::Dpvd1Eg,
        Dpsiegr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsiegr2::Dpvd1Eg,
            dpsiegr2::Dpvd1Eg,
            Dpsiegr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PVD2 Edge Select"]
    #[inline(always)]
    pub fn dpvd2eg(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsiegr2::Dpvd2Eg,
        dpsiegr2::Dpvd2Eg,
        Dpsiegr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsiegr2::Dpvd2Eg,
            dpsiegr2::Dpvd2Eg,
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dpsiegr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Dpsiegr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
    pub struct Dpvd1Eg_SPEC;
    pub type Dpvd1Eg = crate::EnumBitfieldStruct<u8, Dpvd1Eg_SPEC>;
    impl Dpvd1Eg {
        #[doc = "A cancel request is generated when VCC<Vdet1 (fall) is detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated when VCC>=Vdet1 (rise) is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpvd2Eg_SPEC;
    pub type Dpvd2Eg = crate::EnumBitfieldStruct<u8, Dpvd2Eg_SPEC>;
    impl Dpvd2Eg {
        #[doc = "A cancel request is generated when VCC<Vdet2 (fall) is detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated when VCC>=Vdet2 (rise) is detected"]
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
    #[doc = "Deep Standby OCD flag"]
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

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, Syocdcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,Syocdcr_SPEC,crate::common::RW>::from_register(self,0)
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
        <crate::RegValueT<Syocdcr_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod syocdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Docdf_SPEC;
    pub type Docdf = crate::EnumBitfieldStruct<u8, Docdf_SPEC>;
    impl Docdf {
        #[doc = "DBIRQ is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "DBIRQ is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbgen_SPEC;
    pub type Dbgen = crate::EnumBitfieldStruct<u8, Dbgen_SPEC>;
    impl Dbgen {
        #[doc = "Disable on-chip debugger function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable on-chip debugger function"]
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
    #[doc = "Power-On Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
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

    #[doc = "Voltage Monitor 0 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn pvd0rf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rstsr0::Pvd0Rf,
        rstsr0::Pvd0Rf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rstsr0::Pvd0Rf,
            rstsr0::Pvd0Rf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn pvd1rf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rstsr0::Pvd1Rf,
        rstsr0::Pvd1Rf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rstsr0::Pvd1Rf,
            rstsr0::Pvd1Rf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn pvd2rf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rstsr0::Pvd2Rf,
        rstsr0::Pvd2Rf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rstsr0::Pvd2Rf,
            rstsr0::Pvd2Rf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 3 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn pvd3rf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        rstsr0::Pvd3Rf,
        rstsr0::Pvd3Rf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            rstsr0::Pvd3Rf,
            rstsr0::Pvd3Rf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 4 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn pvd4rf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        rstsr0::Pvd4Rf,
        rstsr0::Pvd4Rf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            rstsr0::Pvd4Rf,
            rstsr0::Pvd4Rf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 5 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn pvd5rf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        rstsr0::Pvd5Rf,
        rstsr0::Pvd5Rf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            rstsr0::Pvd5Rf,
            rstsr0::Pvd5Rf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Software Standby Reset FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
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
        #[doc = "Power-on reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Power-on reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd0Rf_SPEC;
    pub type Pvd0Rf = crate::EnumBitfieldStruct<u8, Pvd0Rf_SPEC>;
    impl Pvd0Rf {
        #[doc = "Voltage Monitor 0 reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage Monitor 0 reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd1Rf_SPEC;
    pub type Pvd1Rf = crate::EnumBitfieldStruct<u8, Pvd1Rf_SPEC>;
    impl Pvd1Rf {
        #[doc = "Voltage Monitor 1 reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage Monitor 1 reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd2Rf_SPEC;
    pub type Pvd2Rf = crate::EnumBitfieldStruct<u8, Pvd2Rf_SPEC>;
    impl Pvd2Rf {
        #[doc = "Voltage Monitor 2 reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage Monitor 2 reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd3Rf_SPEC;
    pub type Pvd3Rf = crate::EnumBitfieldStruct<u8, Pvd3Rf_SPEC>;
    impl Pvd3Rf {
        #[doc = "Voltage Monitor 3 reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage Monitor 3 reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd4Rf_SPEC;
    pub type Pvd4Rf = crate::EnumBitfieldStruct<u8, Pvd4Rf_SPEC>;
    impl Pvd4Rf {
        #[doc = "Voltage Monitor 4 reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage Monitor 4 reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd5Rf_SPEC;
    pub type Pvd5Rf = crate::EnumBitfieldStruct<u8, Pvd5Rf_SPEC>;
    impl Pvd5Rf {
        #[doc = "Voltage Monitor 5 reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage Monitor 5 reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpsrstf_SPEC;
    pub type Dpsrstf = crate::EnumBitfieldStruct<u8, Dpsrstf_SPEC>;
    impl Dpsrstf {
        #[doc = "Deep software standby mode cancelation not requested by an interrupt or a reset."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep software standby mode cancelation requested by an interrupt or a reset."]
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Rstsr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Rstsr2_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Rstsr3_SPEC;
impl crate::sealed::RegSpec for Rstsr3_SPEC {
    type DataType = u8;
}

#[doc = "Reset Status Register 3"]
pub type Rstsr3 = crate::RegValueT<Rstsr3_SPEC>;

impl Rstsr3 {
    #[doc = "Overcurrent protection reset Detect Flag"]
    #[inline(always)]
    pub fn ocprf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        rstsr3::Ocprf,
        rstsr3::Ocprf,
        Rstsr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            rstsr3::Ocprf,
            rstsr3::Ocprf,
            Rstsr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Rstsr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Rstsr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rstsr3 {
    #[inline(always)]
    fn default() -> Rstsr3 {
        <crate::RegValueT<Rstsr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstsr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ocprf_SPEC;
    pub type Ocprf = crate::EnumBitfieldStruct<u8, Ocprf_SPEC>;
    impl Ocprf {
        #[doc = "Overcurrent protection reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Overcurrent protection reset detected."]
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
    pub fn modrv0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7,
        1,
        0,
        momcr::Modrv0,
        momcr::Modrv0,
        Momcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7,
            1,
            0,
            momcr::Modrv0,
            momcr::Modrv0,
            Momcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Auto Gain Control Enable"]
    #[inline(always)]
    pub fn agcen(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        momcr::Agcen,
        momcr::Agcen,
        Momcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            momcr::Agcen,
            momcr::Agcen,
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Momcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Momcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
    pub struct Modrv0_SPEC;
    pub type Modrv0 = crate::EnumBitfieldStruct<u8, Modrv0_SPEC>;
    impl Modrv0 {
        #[doc = "8MHz"]
        pub const _000: Self = Self::new(0);

        #[doc = "8.1MHz to 16MHz"]
        pub const _001: Self = Self::new(1);

        #[doc = "16.1MHz to 20MHz"]
        pub const _010: Self = Self::new(2);

        #[doc = "20.1MHz to 26MHz"]
        pub const _011: Self = Self::new(3);

        #[doc = "48MHz"]
        pub const _100: Self = Self::new(4);

        #[doc = "48MHz (value after reset)"]
        pub const _101: Self = Self::new(5);

        #[doc = "48MHz"]
        pub const _110: Self = Self::new(6);

        #[doc = "48MHz"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agcen_SPEC;
    pub type Agcen = crate::EnumBitfieldStruct<u8, Agcen_SPEC>;
    impl Agcen {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
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
pub struct Fwepror_SPEC;
impl crate::sealed::RegSpec for Fwepror_SPEC {
    type DataType = u8;
}

#[doc = "Flash Write Erase Protect Register"]
pub type Fwepror = crate::RegValueT<Fwepror_SPEC>;

impl Fwepror {
    #[doc = "Flash Programing and Erasure"]
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

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Fwepror_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Fwepror_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Prohibits programming and erasure of the code flash, data flash or blank checking."]
        pub const _00: Self = Self::new(0);

        #[doc = "Permits programming and erasure of the code flash, data flash or blank checking."]
        pub const _01: Self = Self::new(1);

        #[doc = "Prohibits programming and erasure of the code flash, data flash or blank checking."]
        pub const _10: Self = Self::new(2);

        #[doc = "Prohibits programming and erasure of the code flash, data flash or blank checking."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pvd1Cmpcr_SPEC;
impl crate::sealed::RegSpec for Pvd1Cmpcr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 1 Comparator Control Register"]
pub type Pvd1Cmpcr = crate::RegValueT<Pvd1Cmpcr_SPEC>;

impl Pvd1Cmpcr {
    #[doc = "Detection Voltage 1 Level Select(Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub fn pvdlvl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        pvd1cmpcr::Pvdlvl,
        pvd1cmpcr::Pvdlvl,
        Pvd1Cmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            pvd1cmpcr::Pvdlvl,
            pvd1cmpcr::Pvdlvl,
            Pvd1Cmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, Pvd1Cmpcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,Pvd1Cmpcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Voltage Detection 1 Enable"]
    #[inline(always)]
    pub fn pvde(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pvd1cmpcr::Pvde,
        pvd1cmpcr::Pvde,
        Pvd1Cmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pvd1cmpcr::Pvde,
            pvd1cmpcr::Pvde,
            Pvd1Cmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pvd1Cmpcr {
    #[inline(always)]
    fn default() -> Pvd1Cmpcr {
        <crate::RegValueT<Pvd1Cmpcr_SPEC> as RegisterValue<_>>::new(15)
    }
}
pub mod pvd1cmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvdlvl_SPEC;
    pub type Pvdlvl = crate::EnumBitfieldStruct<u8, Pvdlvl_SPEC>;
    impl Pvdlvl {
        #[doc = "4.29V (Vdetm_0)"]
        pub const _00000: Self = Self::new(0);

        #[doc = "4.16V (Vdetm_1)"]
        pub const _00001: Self = Self::new(1);

        #[doc = "4.03V (Vdetm_2)"]
        pub const _00010: Self = Self::new(2);

        #[doc = "3.86V (Vdetm_3)"]
        pub const _00011: Self = Self::new(3);

        #[doc = "3.14V (Vdetm_4)"]
        pub const _00100: Self = Self::new(4);

        #[doc = "3.10V (Vdetm_5)"]
        pub const _00101: Self = Self::new(5);

        #[doc = "3.08V (Vdetm_6)"]
        pub const _00110: Self = Self::new(6);

        #[doc = "2.85V (Vdetm_7)"]
        pub const _00111: Self = Self::new(7);

        #[doc = "2.83V (Vdetm_8)"]
        pub const _01000: Self = Self::new(8);

        #[doc = "2.80V (Vdetm_9)"]
        pub const _01001: Self = Self::new(9);

        #[doc = "2.62V (Vdetm_10)"]
        pub const _01010: Self = Self::new(10);

        #[doc = "2.33V (Vdetm_11)"]
        pub const _01011: Self = Self::new(11);

        #[doc = "1.90V (Vdetm_12)"]
        pub const _01100: Self = Self::new(12);

        #[doc = "1.86V (Vdetm_13)"]
        pub const _01101: Self = Self::new(13);

        #[doc = "1.74V (Vdetm_14)"]
        pub const _01110: Self = Self::new(14);

        #[doc = "1.71V (Vdetm_15)"]
        pub const _01111: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvde_SPEC;
    pub type Pvde = crate::EnumBitfieldStruct<u8, Pvde_SPEC>;
    impl Pvde {
        #[doc = "Voltage detection 1 circuit disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage detection 1 circuit enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pvd2Cmpcr_SPEC;
impl crate::sealed::RegSpec for Pvd2Cmpcr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 2 Comparator Control Register"]
pub type Pvd2Cmpcr = crate::RegValueT<Pvd2Cmpcr_SPEC>;

impl Pvd2Cmpcr {
    #[doc = "Detection Voltage 2 Level Select(Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub fn pvdlvl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        pvd2cmpcr::Pvdlvl,
        pvd2cmpcr::Pvdlvl,
        Pvd2Cmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            pvd2cmpcr::Pvdlvl,
            pvd2cmpcr::Pvdlvl,
            Pvd2Cmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, Pvd2Cmpcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,Pvd2Cmpcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Voltage Detection 2 Enable"]
    #[inline(always)]
    pub fn pvde(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pvd2cmpcr::Pvde,
        pvd2cmpcr::Pvde,
        Pvd2Cmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pvd2cmpcr::Pvde,
            pvd2cmpcr::Pvde,
            Pvd2Cmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pvd2Cmpcr {
    #[inline(always)]
    fn default() -> Pvd2Cmpcr {
        <crate::RegValueT<Pvd2Cmpcr_SPEC> as RegisterValue<_>>::new(15)
    }
}
pub mod pvd2cmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvdlvl_SPEC;
    pub type Pvdlvl = crate::EnumBitfieldStruct<u8, Pvdlvl_SPEC>;
    impl Pvdlvl {
        #[doc = "4.29V (Vdetm_0)"]
        pub const _00000: Self = Self::new(0);

        #[doc = "4.16V (Vdetm_1)"]
        pub const _00001: Self = Self::new(1);

        #[doc = "4.03V (Vdetm_2)"]
        pub const _00010: Self = Self::new(2);

        #[doc = "3.86V (Vdetm_3)"]
        pub const _00011: Self = Self::new(3);

        #[doc = "3.14V (Vdetm_4)"]
        pub const _00100: Self = Self::new(4);

        #[doc = "3.10V (Vdetm_5)"]
        pub const _00101: Self = Self::new(5);

        #[doc = "3.08V (Vdetm_6)"]
        pub const _00110: Self = Self::new(6);

        #[doc = "2.85V (Vdetm_7)"]
        pub const _00111: Self = Self::new(7);

        #[doc = "2.83V (Vdetm_8)"]
        pub const _01000: Self = Self::new(8);

        #[doc = "2.80V (Vdetm_9)"]
        pub const _01001: Self = Self::new(9);

        #[doc = "2.62V (Vdetm_10)"]
        pub const _01010: Self = Self::new(10);

        #[doc = "2.33V (Vdetm_11)"]
        pub const _01011: Self = Self::new(11);

        #[doc = "1.90V (Vdetm_12)"]
        pub const _01100: Self = Self::new(12);

        #[doc = "1.86V (Vdetm_13)"]
        pub const _01101: Self = Self::new(13);

        #[doc = "1.74V (Vdetm_14)"]
        pub const _01110: Self = Self::new(14);

        #[doc = "1.71V (Vdetm_15)"]
        pub const _01111: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvde_SPEC;
    pub type Pvde = crate::EnumBitfieldStruct<u8, Pvde_SPEC>;
    impl Pvde {
        #[doc = "Voltage detection 2 circuit disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage detection 2 circuit enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pvdcr0_SPEC;
impl crate::sealed::RegSpec for Pvdcr0_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor %s Circuit Control Register 0"]
pub type Pvdcr0 = crate::RegValueT<Pvdcr0_SPEC>;

impl Pvdcr0 {
    #[doc = "Voltage Monitor  Interrupt/Reset Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pvdcr0::Rie,
        pvdcr0::Rie,
        Pvdcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pvdcr0::Rie,
            pvdcr0::Rie,
            Pvdcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor Digital Filter Disable Mode Select"]
    #[inline(always)]
    pub fn dfdis(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pvdcr0::Dfdis,
        pvdcr0::Dfdis,
        Pvdcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pvdcr0::Dfdis,
            pvdcr0::Dfdis,
            Pvdcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub fn cmpe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pvdcr0::Cmpe,
        pvdcr0::Cmpe,
        Pvdcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pvdcr0::Cmpe,
            pvdcr0::Cmpe,
            Pvdcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 1. The write value should be 1."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Pvdcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pvdcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
        pvdcr0::Fsamp,
        pvdcr0::Fsamp,
        Pvdcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            pvdcr0::Fsamp,
            pvdcr0::Fsamp,
            Pvdcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor Circuit Mode Select"]
    #[inline(always)]
    pub fn ri(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pvdcr0::Ri,
        pvdcr0::Ri,
        Pvdcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pvdcr0::Ri,
            pvdcr0::Ri,
            Pvdcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor Reset Negate Select"]
    #[inline(always)]
    pub fn rn(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pvdcr0::Rn,
        pvdcr0::Rn,
        Pvdcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pvdcr0::Rn,
            pvdcr0::Rn,
            Pvdcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pvdcr0 {
    #[inline(always)]
    fn default() -> Pvdcr0 {
        <crate::RegValueT<Pvdcr0_SPEC> as RegisterValue<_>>::new(130)
    }
}
pub mod pvdcr0 {

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
        #[doc = "Enable digital filter"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable digital filter"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpe_SPEC;
    pub type Cmpe = crate::EnumBitfieldStruct<u8, Cmpe_SPEC>;
    impl Cmpe {
        #[doc = "Disable voltage monitor 1 circuit comparison result output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable voltage monitor 1 circuit comparison result output."]
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
        #[doc = "Voltage Monitor  interrupt during Vdet1 passage"]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage Monitor  reset enabled when the voltage falls to and below Vdet1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rn_SPEC;
    pub type Rn = crate::EnumBitfieldStruct<u8, Rn_SPEC>;
    impl Rn {
        #[doc = "Negation follows a stabilization time (tPVD) after VCC > Vdet is detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Negation follows a stabilization time (tPVD) after assertion of the PVD reset."]
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
    #[doc = "VBATT Voltage Monitor Function Select Bit"]
    #[inline(always)]
    pub fn vbattmnsel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Vbattmnselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Vbattmnselr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Vbattmnselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Vbattmnselr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbattmnselr {
    #[inline(always)]
    fn default() -> Vbattmnselr {
        <crate::RegValueT<Vbattmnselr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtbpcr1_SPEC;
impl crate::sealed::RegSpec for Vbtbpcr1_SPEC {
    type DataType = u8;
}

#[doc = "VBATT Battery Power Supply Control Register 1"]
pub type Vbtbpcr1 = crate::RegValueT<Vbtbpcr1_SPEC>;

impl Vbtbpcr1 {
    #[doc = "Battery Power Supply Switch Stop"]
    #[inline(always)]
    pub fn bpwswstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtbpcr1::Bpwswstp,
        vbtbpcr1::Bpwswstp,
        Vbtbpcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtbpcr1::Bpwswstp,
            vbtbpcr1::Bpwswstp,
            Vbtbpcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Vbtbpcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Vbtbpcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtbpcr1 {
    #[inline(always)]
    fn default() -> Vbtbpcr1 {
        <crate::RegValueT<Vbtbpcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtbpcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpwswstp_SPEC;
    pub type Bpwswstp = crate::EnumBitfieldStruct<u8, Bpwswstp_SPEC>;
    impl Bpwswstp {
        #[doc = "Battery power supply switch enable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Battery power supply switch stop"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpscr_SPEC;
impl crate::sealed::RegSpec for Lpscr_SPEC {
    type DataType = u8;
}

#[doc = "Low Power State Control Register"]
pub type Lpscr = crate::RegValueT<Lpscr_SPEC>;

impl Lpscr {
    #[doc = "Low power mode setting bit"]
    #[inline(always)]
    pub fn lpmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        lpscr::Lpmd,
        lpscr::Lpmd,
        Lpscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            lpscr::Lpmd,
            lpscr::Lpmd,
            Lpscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Lpscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Lpscr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lpscr {
    #[inline(always)]
    fn default() -> Lpscr {
        <crate::RegValueT<Lpscr_SPEC> as RegisterValue<_>>::new(4)
    }
}
pub mod lpscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lpmd_SPEC;
    pub type Lpmd = crate::EnumBitfieldStruct<u8, Lpmd_SPEC>;
    impl Lpmd {
        #[doc = "System Active"]
        pub const _0_H: Self = Self::new(0);

        #[doc = "Prohibited. (reserved)"]
        pub const _1_H: Self = Self::new(1);

        #[doc = "Prohibited. (reserved) for CPU0 Deep Sleep"]
        pub const _2_H: Self = Self::new(2);

        #[doc = "Prohibited. (reserved) CPU0 Power Gating"]
        pub const _3_H: Self = Self::new(3);

        #[doc = "Software Standby mode 1"]
        pub const _4_H: Self = Self::new(4);

        #[doc = "Software Standby mode 2"]
        pub const _5_H: Self = Self::new(5);

        #[doc = "Prohibited. (reserved for Software Standby mode 3)"]
        pub const _6_H: Self = Self::new(6);

        #[doc = "Prohibited. (reserved)"]
        pub const _7_H: Self = Self::new(7);

        #[doc = "Deep Software Standby mode 1"]
        pub const _8_H: Self = Self::new(8);

        #[doc = "Deep Software Standby mode 2"]
        pub const _9_H: Self = Self::new(9);

        #[doc = "Deep Software Standby mode 3"]
        pub const AH: Self = Self::new(10);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sscr1_SPEC;
impl crate::sealed::RegSpec for Sscr1_SPEC {
    type DataType = u8;
}

#[doc = "Software Standby Control Register 1"]
pub type Sscr1 = crate::RegValueT<Sscr1_SPEC>;

impl Sscr1 {
    #[doc = "Software Standby 1 Fast Return"]
    #[inline(always)]
    pub fn ss1fr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sscr1::Ss1Fr,
        sscr1::Ss1Fr,
        Sscr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sscr1::Ss1Fr,
            sscr1::Ss1Fr,
            Sscr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sscr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sscr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sscr1 {
    #[inline(always)]
    fn default() -> Sscr1 {
        <crate::RegValueT<Sscr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sscr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ss1Fr_SPEC;
    pub type Ss1Fr = crate::EnumBitfieldStruct<u8, Ss1Fr_SPEC>;
    impl Ss1Fr {
        #[doc = "When returning from Software Standby mode 1, fast return function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "When returning from Software Standby mode 1, fast return function is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvocr_SPEC;
impl crate::sealed::RegSpec for Lvocr_SPEC {
    type DataType = u8;
}

#[doc = "Low Power State Control Register"]
pub type Lvocr = crate::RegValueT<Lvocr_SPEC>;

impl Lvocr {
    #[doc = "Low Voltage Operation 0 Enable"]
    #[inline(always)]
    pub fn lvo0e(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lvocr::Lvo0E,
        lvocr::Lvo0E,
        Lvocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lvocr::Lvo0E,
            lvocr::Lvo0E,
            Lvocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Low Voltage Operation 1 Enable"]
    #[inline(always)]
    pub fn lvo1e(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        lvocr::Lvo1E,
        lvocr::Lvo1E,
        Lvocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            lvocr::Lvo1E,
            lvocr::Lvo1E,
            Lvocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Lvocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Lvocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lvocr {
    #[inline(always)]
    fn default() -> Lvocr {
        <crate::RegValueT<Lvocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lvocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvo0E_SPEC;
    pub type Lvo0E = crate::EnumBitfieldStruct<u8, Lvo0E_SPEC>;
    impl Lvo0E {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvo1E_SPEC;
    pub type Lvo1E = crate::EnumBitfieldStruct<u8, Lvo1E_SPEC>;
    impl Lvo1E {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syrstmsk0_SPEC;
impl crate::sealed::RegSpec for Syrstmsk0_SPEC {
    type DataType = u8;
}

#[doc = "System Reset Mask Control Register0"]
pub type Syrstmsk0 = crate::RegValueT<Syrstmsk0_SPEC>;

impl Syrstmsk0 {
    #[doc = "Independent watchdog timer Reset Mask"]
    #[inline(always)]
    pub fn iwdtmask(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syrstmsk0::Iwdtmask,
        syrstmsk0::Iwdtmask,
        Syrstmsk0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syrstmsk0::Iwdtmask,
            syrstmsk0::Iwdtmask,
            Syrstmsk0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU0 Watchdog timer Reset Mask"]
    #[inline(always)]
    pub fn wdt0mask(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syrstmsk0::Wdt0Mask,
        syrstmsk0::Wdt0Mask,
        Syrstmsk0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syrstmsk0::Wdt0Mask,
            syrstmsk0::Wdt0Mask,
            Syrstmsk0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software Reset Mask"]
    #[inline(always)]
    pub fn swmask(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syrstmsk0::Swmask,
        syrstmsk0::Swmask,
        Syrstmsk0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syrstmsk0::Swmask,
            syrstmsk0::Swmask,
            Syrstmsk0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Syrstmsk0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Syrstmsk0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CPU0 Lockup Reset Mask"]
    #[inline(always)]
    pub fn clup0mask(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        syrstmsk0::Clup0Mask,
        syrstmsk0::Clup0Mask,
        Syrstmsk0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            syrstmsk0::Clup0Mask,
            syrstmsk0::Clup0Mask,
            Syrstmsk0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Local memory 0 error Reset Mask"]
    #[inline(always)]
    pub fn lm0mask(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        syrstmsk0::Lm0Mask,
        syrstmsk0::Lm0Mask,
        Syrstmsk0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            syrstmsk0::Lm0Mask,
            syrstmsk0::Lm0Mask,
            Syrstmsk0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Common memory error Reset Mask"]
    #[inline(always)]
    pub fn cmmask(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        syrstmsk0::Cmmask,
        syrstmsk0::Cmmask,
        Syrstmsk0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            syrstmsk0::Cmmask,
            syrstmsk0::Cmmask,
            Syrstmsk0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BUS error Reset Mask"]
    #[inline(always)]
    pub fn busmask(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        syrstmsk0::Busmask,
        syrstmsk0::Busmask,
        Syrstmsk0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            syrstmsk0::Busmask,
            syrstmsk0::Busmask,
            Syrstmsk0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syrstmsk0 {
    #[inline(always)]
    fn default() -> Syrstmsk0 {
        <crate::RegValueT<Syrstmsk0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syrstmsk0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtmask_SPEC;
    pub type Iwdtmask = crate::EnumBitfieldStruct<u8, Iwdtmask_SPEC>;
    impl Iwdtmask {
        #[doc = "Reset occurrence is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset occurrence is disabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdt0Mask_SPEC;
    pub type Wdt0Mask = crate::EnumBitfieldStruct<u8, Wdt0Mask_SPEC>;
    impl Wdt0Mask {
        #[doc = "Reset occurrence is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset occurrence is disabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swmask_SPEC;
    pub type Swmask = crate::EnumBitfieldStruct<u8, Swmask_SPEC>;
    impl Swmask {
        #[doc = "Reset occurrence is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset occurrence is disabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clup0Mask_SPEC;
    pub type Clup0Mask = crate::EnumBitfieldStruct<u8, Clup0Mask_SPEC>;
    impl Clup0Mask {
        #[doc = "Reset occurrence is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset occurrence is disabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lm0Mask_SPEC;
    pub type Lm0Mask = crate::EnumBitfieldStruct<u8, Lm0Mask_SPEC>;
    impl Lm0Mask {
        #[doc = "Reset occurrence is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset occurrence is disabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmmask_SPEC;
    pub type Cmmask = crate::EnumBitfieldStruct<u8, Cmmask_SPEC>;
    impl Cmmask {
        #[doc = "Reset occurrence is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset occurrence is disabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busmask_SPEC;
    pub type Busmask = crate::EnumBitfieldStruct<u8, Busmask_SPEC>;
    impl Busmask {
        #[doc = "Reset occurrence is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset occurrence is disabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syrstmsk1_SPEC;
impl crate::sealed::RegSpec for Syrstmsk1_SPEC {
    type DataType = u8;
}

#[doc = "System Reset Mask Control Register1"]
pub type Syrstmsk1 = crate::RegValueT<Syrstmsk1_SPEC>;

impl Syrstmsk1 {
    #[doc = "Local memory 1 error Reset Mask"]
    #[inline(always)]
    pub fn lm1mask(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        syrstmsk1::Lm1Mask,
        syrstmsk1::Lm1Mask,
        Syrstmsk1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            syrstmsk1::Lm1Mask,
            syrstmsk1::Lm1Mask,
            Syrstmsk1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Syrstmsk1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Syrstmsk1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Network Reset Mask"]
    #[inline(always)]
    pub fn nwmask(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        syrstmsk1::Nwmask,
        syrstmsk1::Nwmask,
        Syrstmsk1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            syrstmsk1::Nwmask,
            syrstmsk1::Nwmask,
            Syrstmsk1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syrstmsk1 {
    #[inline(always)]
    fn default() -> Syrstmsk1 {
        <crate::RegValueT<Syrstmsk1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syrstmsk1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lm1Mask_SPEC;
    pub type Lm1Mask = crate::EnumBitfieldStruct<u8, Lm1Mask_SPEC>;
    impl Lm1Mask {
        #[doc = "Reset occurrence is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset occurrence is disabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nwmask_SPEC;
    pub type Nwmask = crate::EnumBitfieldStruct<u8, Nwmask_SPEC>;
    impl Nwmask {
        #[doc = "Reset occurrence is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset occurrence is disabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syrstmsk2_SPEC;
impl crate::sealed::RegSpec for Syrstmsk2_SPEC {
    type DataType = u8;
}

#[doc = "System Reset Mask Control Register2"]
pub type Syrstmsk2 = crate::RegValueT<Syrstmsk2_SPEC>;

impl Syrstmsk2 {
    #[doc = "Voltage Monitor 1 Reset  Mask"]
    #[inline(always)]
    pub fn pvd1mask(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syrstmsk2::Pvd1Mask,
        syrstmsk2::Pvd1Mask,
        Syrstmsk2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syrstmsk2::Pvd1Mask,
            syrstmsk2::Pvd1Mask,
            Syrstmsk2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Reset  Mask"]
    #[inline(always)]
    pub fn pvd2mask(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syrstmsk2::Pvd2Mask,
        syrstmsk2::Pvd2Mask,
        Syrstmsk2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syrstmsk2::Pvd2Mask,
            syrstmsk2::Pvd2Mask,
            Syrstmsk2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 3 Reset  Mask"]
    #[inline(always)]
    pub fn pvd3mask(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syrstmsk2::Pvd3Mask,
        syrstmsk2::Pvd3Mask,
        Syrstmsk2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syrstmsk2::Pvd3Mask,
            syrstmsk2::Pvd3Mask,
            Syrstmsk2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 4 Reset  Mask"]
    #[inline(always)]
    pub fn pvd4mask(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        syrstmsk2::Pvd4Mask,
        syrstmsk2::Pvd4Mask,
        Syrstmsk2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            syrstmsk2::Pvd4Mask,
            syrstmsk2::Pvd4Mask,
            Syrstmsk2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 5 Reset  Mask"]
    #[inline(always)]
    pub fn pvd5mask(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        syrstmsk2::Pvd5Mask,
        syrstmsk2::Pvd5Mask,
        Syrstmsk2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            syrstmsk2::Pvd5Mask,
            syrstmsk2::Pvd5Mask,
            Syrstmsk2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Syrstmsk2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Syrstmsk2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Syrstmsk2 {
    #[inline(always)]
    fn default() -> Syrstmsk2 {
        <crate::RegValueT<Syrstmsk2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syrstmsk2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd1Mask_SPEC;
    pub type Pvd1Mask = crate::EnumBitfieldStruct<u8, Pvd1Mask_SPEC>;
    impl Pvd1Mask {
        #[doc = "Reset occurrence is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset occurrence is disabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd2Mask_SPEC;
    pub type Pvd2Mask = crate::EnumBitfieldStruct<u8, Pvd2Mask_SPEC>;
    impl Pvd2Mask {
        #[doc = "Reset occurrence is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset occurrence is disabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd3Mask_SPEC;
    pub type Pvd3Mask = crate::EnumBitfieldStruct<u8, Pvd3Mask_SPEC>;
    impl Pvd3Mask {
        #[doc = "Reset occurrence is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset occurrence is disabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd4Mask_SPEC;
    pub type Pvd4Mask = crate::EnumBitfieldStruct<u8, Pvd4Mask_SPEC>;
    impl Pvd4Mask {
        #[doc = "Reset occurrence is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset occurrence is disabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd5Mask_SPEC;
    pub type Pvd5Mask = crate::EnumBitfieldStruct<u8, Pvd5Mask_SPEC>;
    impl Pvd5Mask {
        #[doc = "Reset occurrence is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset occurrence is disabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1Ldocr_SPEC;
impl crate::sealed::RegSpec for Pll1Ldocr_SPEC {
    type DataType = u8;
}

#[doc = "PLL1-LDO Control Register"]
pub type Pll1Ldocr = crate::RegValueT<Pll1Ldocr_SPEC>;

impl Pll1Ldocr {
    #[doc = "LDO Stop"]
    #[inline(always)]
    pub fn ldostp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pll1ldocr::Ldostp,
        pll1ldocr::Ldostp,
        Pll1Ldocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pll1ldocr::Ldostp,
            pll1ldocr::Ldostp,
            Pll1Ldocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "STBY Keep"]
    #[inline(always)]
    pub fn skeep(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pll1ldocr::Skeep,
        pll1ldocr::Skeep,
        Pll1Ldocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pll1ldocr::Skeep,
            pll1ldocr::Skeep,
            Pll1Ldocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Pll1Ldocr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Pll1Ldocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pll1Ldocr {
    #[inline(always)]
    fn default() -> Pll1Ldocr {
        <crate::RegValueT<Pll1Ldocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pll1ldocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ldostp_SPEC;
    pub type Ldostp = crate::EnumBitfieldStruct<u8, Ldostp_SPEC>;
    impl Ldostp {
        #[doc = "PLL1-LDO is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "PLL1-LDO is stopped"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Skeep_SPEC;
    pub type Skeep = crate::EnumBitfieldStruct<u8, Skeep_SPEC>;
    impl Skeep {
        #[doc = "PLL1-LDO is stopped during Software Standby mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "PLL1-LDO state before Software Standby mode is retained during Software Standby mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll2Ldocr_SPEC;
impl crate::sealed::RegSpec for Pll2Ldocr_SPEC {
    type DataType = u8;
}

#[doc = "PLL2-LDO Control Register"]
pub type Pll2Ldocr = crate::RegValueT<Pll2Ldocr_SPEC>;

impl Pll2Ldocr {
    #[doc = "LDO Stop"]
    #[inline(always)]
    pub fn ldostp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pll2ldocr::Ldostp,
        pll2ldocr::Ldostp,
        Pll2Ldocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pll2ldocr::Ldostp,
            pll2ldocr::Ldostp,
            Pll2Ldocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "STBY Keep"]
    #[inline(always)]
    pub fn skeep(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pll2ldocr::Skeep,
        pll2ldocr::Skeep,
        Pll2Ldocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pll2ldocr::Skeep,
            pll2ldocr::Skeep,
            Pll2Ldocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Pll2Ldocr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Pll2Ldocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pll2Ldocr {
    #[inline(always)]
    fn default() -> Pll2Ldocr {
        <crate::RegValueT<Pll2Ldocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pll2ldocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ldostp_SPEC;
    pub type Ldostp = crate::EnumBitfieldStruct<u8, Ldostp_SPEC>;
    impl Ldostp {
        #[doc = "PLL2-LDO is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "PLL2-LDO is stopped"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Skeep_SPEC;
    pub type Skeep = crate::EnumBitfieldStruct<u8, Skeep_SPEC>;
    impl Skeep {
        #[doc = "PLL2-LDO is stopped during Software Standby mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "PLL2-LDO state before Software Standby mode is retained during Software Standby mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hocoldocr_SPEC;
impl crate::sealed::RegSpec for Hocoldocr_SPEC {
    type DataType = u8;
}

#[doc = "HOCO-LDO Control Register"]
pub type Hocoldocr = crate::RegValueT<Hocoldocr_SPEC>;

impl Hocoldocr {
    #[doc = "LDO Stop"]
    #[inline(always)]
    pub fn ldostp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        hocoldocr::Ldostp,
        hocoldocr::Ldostp,
        Hocoldocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            hocoldocr::Ldostp,
            hocoldocr::Ldostp,
            Hocoldocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "STBY Keep"]
    #[inline(always)]
    pub fn skeep(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        hocoldocr::Skeep,
        hocoldocr::Skeep,
        Hocoldocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            hocoldocr::Skeep,
            hocoldocr::Skeep,
            Hocoldocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Hocoldocr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Hocoldocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Hocoldocr {
    #[inline(always)]
    fn default() -> Hocoldocr {
        <crate::RegValueT<Hocoldocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hocoldocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ldostp_SPEC;
    pub type Ldostp = crate::EnumBitfieldStruct<u8, Ldostp_SPEC>;
    impl Ldostp {
        #[doc = "HOCO-LDO is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "HOCO-LDO is stopped"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Skeep_SPEC;
    pub type Skeep = crate::EnumBitfieldStruct<u8, Skeep_SPEC>;
    impl Skeep {
        #[doc = "HOCO-LDO is stopped during Software Standby mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "HOCO-LDO state before Software Standby mode is retained during Software Standby mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Momcr2_SPEC;
impl crate::sealed::RegSpec for Momcr2_SPEC {
    type DataType = u8;
}

#[doc = "Main Clock Oscillator Mode Control Register 2"]
pub type Momcr2 = crate::RegValueT<Momcr2_SPEC>;

impl Momcr2 {
    #[doc = "Main Clock Oscillator Mode Select"]
    #[inline(always)]
    pub fn momode(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        momcr2::Momode,
        momcr2::Momode,
        Momcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            momcr2::Momode,
            momcr2::Momode,
            Momcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Momcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Momcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Momcr2 {
    #[inline(always)]
    fn default() -> Momcr2 {
        <crate::RegValueT<Momcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod momcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Momode_SPEC;
    pub type Momode = crate::EnumBitfieldStruct<u8, Momode_SPEC>;
    impl Momode {
        #[doc = "Normal crystal oscillator mode (value after reset)"]
        pub const _0: Self = Self::new(0);

        #[doc = "8M to 12MHz custom ceramic mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccr_SPEC;
impl crate::sealed::RegSpec for Sosccr_SPEC {
    type DataType = u8;
}

#[doc = "Sub-clock oscillator control register"]
pub type Sosccr = crate::RegValueT<Sosccr_SPEC>;

impl Sosccr {
    #[doc = "Sub-Clock Oscillator Stop"]
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sosccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sosccr_SPEC,crate::common::RW>::from_register(self,0)
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

#[doc = "Sub Clock Oscillator Mode Control Register"]
pub type Somcr = crate::RegValueT<Somcr_SPEC>;

impl Somcr {
    #[doc = "Sub Clock Oscillator Drive Capability Switching"]
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

    #[doc = "Sub Clock Oscillator Switching"]
    #[inline(always)]
    pub fn sosel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        somcr::Sosel,
        somcr::Sosel,
        Somcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            somcr::Sosel,
            somcr::Sosel,
            Somcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Somcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Somcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
        #[doc = ":Standard(12.5pf) (value after reset)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Low power mode 1 (9pf)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Low power mode 2 (7pf)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Low power mode 3 (4pf)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sosel_SPEC;
    pub type Sosel = crate::EnumBitfieldStruct<u8, Sosel_SPEC>;
    impl Sosel {
        #[doc = "Resonator"]
        pub const _0: Self = Self::new(0);

        #[doc = "External clock input"]
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

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Vbtber_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Vbtber_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "disable to access VBTBKR"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable to access VBTBKR"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtbpcr2_SPEC;
impl crate::sealed::RegSpec for Vbtbpcr2_SPEC {
    type DataType = u8;
}

#[doc = "VBATT Battery Power Supply Control Register 2"]
pub type Vbtbpcr2 = crate::RegValueT<Vbtbpcr2_SPEC>;

impl Vbtbpcr2 {
    #[doc = "VDETBAT Level Select"]
    #[inline(always)]
    pub fn vdetlvl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        vbtbpcr2::Vdetlvl,
        vbtbpcr2::Vdetlvl,
        Vbtbpcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            vbtbpcr2::Vdetlvl,
            vbtbpcr2::Vdetlvl,
            Vbtbpcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage drop detection enable"]
    #[inline(always)]
    pub fn vdete(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        vbtbpcr2::Vdete,
        vbtbpcr2::Vdete,
        Vbtbpcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            vbtbpcr2::Vdete,
            vbtbpcr2::Vdete,
            Vbtbpcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Vbtbpcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Vbtbpcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtbpcr2 {
    #[inline(always)]
    fn default() -> Vbtbpcr2 {
        <crate::RegValueT<Vbtbpcr2_SPEC> as RegisterValue<_>>::new(6)
    }
}
pub mod vbtbpcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vdetlvl_SPEC;
    pub type Vdetlvl = crate::EnumBitfieldStruct<u8, Vdetlvl_SPEC>;
    impl Vdetlvl {
        #[doc = "2.8V"]
        pub const _000: Self = Self::new(0);

        #[doc = "2.53V"]
        pub const _001: Self = Self::new(1);

        #[doc = "2.10V"]
        pub const _010: Self = Self::new(2);

        #[doc = "1.95V"]
        pub const _011: Self = Self::new(3);

        #[doc = "1.85V"]
        pub const _100: Self = Self::new(4);

        #[doc = "1.75V"]
        pub const _101: Self = Self::new(5);

        #[doc = "1.65V"]
        pub const _110: Self = Self::new(6);

        #[doc = "prohibited (1.55V)"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vdete_SPEC;
    pub type Vdete = crate::EnumBitfieldStruct<u8, Vdete_SPEC>;
    impl Vdete {
        #[doc = "VCC Voltage drop detection disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "VCC Voltage drop detection enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtbpsr_SPEC;
impl crate::sealed::RegSpec for Vbtbpsr_SPEC {
    type DataType = u8;
}

#[doc = "VBATT Battery Power Supply Status Register"]
pub type Vbtbpsr = crate::RegValueT<Vbtbpsr_SPEC>;

impl Vbtbpsr {
    #[doc = "VBATT_POR Flag"]
    #[inline(always)]
    pub fn vbporf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtbpsr::Vbporf,
        vbtbpsr::Vbporf,
        Vbtbpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtbpsr::Vbporf,
            vbtbpsr::Vbporf,
            Vbtbpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT_POR Monitor"]
    #[inline(always)]
    pub fn vbporm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        vbtbpsr::Vbporm,
        vbtbpsr::Vbporm,
        Vbtbpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            vbtbpsr::Vbporm,
            vbtbpsr::Vbporm,
            Vbtbpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Battery Power Supply Switch Status Monitor"]
    #[inline(always)]
    pub fn bpwswm(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        vbtbpsr::Bpwswm,
        vbtbpsr::Bpwswm,
        Vbtbpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            vbtbpsr::Bpwswm,
            vbtbpsr::Bpwswm,
            Vbtbpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Vbtbpsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Vbtbpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtbpsr {
    #[inline(always)]
    fn default() -> Vbtbpsr {
        <crate::RegValueT<Vbtbpsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtbpsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbporf_SPEC;
    pub type Vbporf = crate::EnumBitfieldStruct<u8, Vbporf_SPEC>;
    impl Vbporf {
        #[doc = "VBATT_R voltage drop is not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "VBATT_R voltage drop is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbporm_SPEC;
    pub type Vbporm = crate::EnumBitfieldStruct<u8, Vbporm_SPEC>;
    impl Vbporm {
        #[doc = "VBATT_R voltage < VVBATPOR"]
        pub const _0: Self = Self::new(0);

        #[doc = "VBATT_R voltage > VVBATPOR"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpwswm_SPEC;
    pub type Bpwswm = crate::EnumBitfieldStruct<u8, Bpwswm_SPEC>;
    impl Bpwswm {
        #[doc = "VCC voltage < VDETBAT"]
        pub const _0: Self = Self::new(0);

        #[doc = "VCC voltage > VDETBAT"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtadsr_SPEC;
impl crate::sealed::RegSpec for Vbtadsr_SPEC {
    type DataType = u8;
}

#[doc = "VBATT Tamper detection Status Register"]
pub type Vbtadsr = crate::RegValueT<Vbtadsr_SPEC>;

impl Vbtadsr {
    #[doc = "VBATT Tamper Detection flag 0"]
    #[inline(always)]
    pub fn vbtadf0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtadsr::Vbtadf0,
        vbtadsr::Vbtadf0,
        Vbtadsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtadsr::Vbtadf0,
            vbtadsr::Vbtadf0,
            Vbtadsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT Tamper Detection flag 1"]
    #[inline(always)]
    pub fn vbtadf1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vbtadsr::Vbtadf1,
        vbtadsr::Vbtadf1,
        Vbtadsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vbtadsr::Vbtadf1,
            vbtadsr::Vbtadf1,
            Vbtadsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT Tamper Detection flag 2"]
    #[inline(always)]
    pub fn vbtadf2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        vbtadsr::Vbtadf2,
        vbtadsr::Vbtadf2,
        Vbtadsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            vbtadsr::Vbtadf2,
            vbtadsr::Vbtadf2,
            Vbtadsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Vbtadsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Vbtadsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtadsr {
    #[inline(always)]
    fn default() -> Vbtadsr {
        <crate::RegValueT<Vbtadsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtadsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtadf0_SPEC;
    pub type Vbtadf0 = crate::EnumBitfieldStruct<u8, Vbtadf0_SPEC>;
    impl Vbtadf0 {
        #[doc = "RTCIC2 input edge is not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCIC2 input edge is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtadf1_SPEC;
    pub type Vbtadf1 = crate::EnumBitfieldStruct<u8, Vbtadf1_SPEC>;
    impl Vbtadf1 {
        #[doc = "RTCIC1 input edge is not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCIC1 input edge is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtadf2_SPEC;
    pub type Vbtadf2 = crate::EnumBitfieldStruct<u8, Vbtadf2_SPEC>;
    impl Vbtadf2 {
        #[doc = "RTCIC2 input edge is not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCIC2 input edge is detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtadcr1_SPEC;
impl crate::sealed::RegSpec for Vbtadcr1_SPEC {
    type DataType = u8;
}

#[doc = "VBATT Tamper detection Control  Register 1"]
pub type Vbtadcr1 = crate::RegValueT<Vbtadcr1_SPEC>;

impl Vbtadcr1 {
    #[doc = "VBATT Tamper Detection Interrupt Enable 0"]
    #[inline(always)]
    pub fn vbtadie0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtadcr1::Vbtadie0,
        vbtadcr1::Vbtadie0,
        Vbtadcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtadcr1::Vbtadie0,
            vbtadcr1::Vbtadie0,
            Vbtadcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT Tamper Detection Interrupt Enable 1"]
    #[inline(always)]
    pub fn vbtadie1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vbtadcr1::Vbtadie1,
        vbtadcr1::Vbtadie1,
        Vbtadcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vbtadcr1::Vbtadie1,
            vbtadcr1::Vbtadie1,
            Vbtadcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT Tamper Detection Interrupt Enable 2"]
    #[inline(always)]
    pub fn vbtadie2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        vbtadcr1::Vbtadie2,
        vbtadcr1::Vbtadie2,
        Vbtadcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            vbtadcr1::Vbtadie2,
            vbtadcr1::Vbtadie2,
            Vbtadcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT Tamper Detection Backup Register Clear Enable 0"]
    #[inline(always)]
    pub fn vbtadcle0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        vbtadcr1::Vbtadcle0,
        vbtadcr1::Vbtadcle0,
        Vbtadcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            vbtadcr1::Vbtadcle0,
            vbtadcr1::Vbtadcle0,
            Vbtadcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT Tamper Detection Backup Register Clear Enable 1"]
    #[inline(always)]
    pub fn vbtadcle1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        vbtadcr1::Vbtadcle1,
        vbtadcr1::Vbtadcle1,
        Vbtadcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            vbtadcr1::Vbtadcle1,
            vbtadcr1::Vbtadcle1,
            Vbtadcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT Tamper Detection Backup Register Clear Enable 2"]
    #[inline(always)]
    pub fn vbtadcle2(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        vbtadcr1::Vbtadcle2,
        vbtadcr1::Vbtadcle2,
        Vbtadcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            vbtadcr1::Vbtadcle2,
            vbtadcr1::Vbtadcle2,
            Vbtadcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Vbtadcr1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Vbtadcr1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Vbtadcr1 {
    #[inline(always)]
    fn default() -> Vbtadcr1 {
        <crate::RegValueT<Vbtadcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtadcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtadie0_SPEC;
    pub type Vbtadie0 = crate::EnumBitfieldStruct<u8, Vbtadie0_SPEC>;
    impl Vbtadie0 {
        #[doc = "Interrupt by VBTADF0 flag is disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt by VBTADF0 flag is enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtadie1_SPEC;
    pub type Vbtadie1 = crate::EnumBitfieldStruct<u8, Vbtadie1_SPEC>;
    impl Vbtadie1 {
        #[doc = "Interrupt by VBTADF1 flag is disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt by VBTADF1 flag is enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtadie2_SPEC;
    pub type Vbtadie2 = crate::EnumBitfieldStruct<u8, Vbtadie2_SPEC>;
    impl Vbtadie2 {
        #[doc = "Interrupt by VBTADF2 flag is disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt by VBTADF2 flag is enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtadcle0_SPEC;
    pub type Vbtadcle0 = crate::EnumBitfieldStruct<u8, Vbtadcle0_SPEC>;
    impl Vbtadcle0 {
        #[doc = "Clear Backup Register by VBTADF0 flag is disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear Backup Register by VBTADF0 flag is enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtadcle1_SPEC;
    pub type Vbtadcle1 = crate::EnumBitfieldStruct<u8, Vbtadcle1_SPEC>;
    impl Vbtadcle1 {
        #[doc = "Clear Backup Register by VBTADF1 flag is disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear Backup Register by VBTADF1 flag is enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtadcle2_SPEC;
    pub type Vbtadcle2 = crate::EnumBitfieldStruct<u8, Vbtadcle2_SPEC>;
    impl Vbtadcle2 {
        #[doc = "Clear Backup Register by VBTADF2 flag is disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear Backup Register by VBTADF2 flag is enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtadcr2_SPEC;
impl crate::sealed::RegSpec for Vbtadcr2_SPEC {
    type DataType = u8;
}

#[doc = "VBATT Tamper detection Control  Register 2"]
pub type Vbtadcr2 = crate::RegValueT<Vbtadcr2_SPEC>;

impl Vbtadcr2 {
    #[doc = "VBATT RTC Time Capture Event Source Select 0"]
    #[inline(always)]
    pub fn vbrtces0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtadcr2::Vbrtces0,
        vbtadcr2::Vbrtces0,
        Vbtadcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtadcr2::Vbrtces0,
            vbtadcr2::Vbrtces0,
            Vbtadcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT RTC Time Capture Event Source Select 1"]
    #[inline(always)]
    pub fn vbrtces1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vbtadcr2::Vbrtces1,
        vbtadcr2::Vbrtces1,
        Vbtadcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vbtadcr2::Vbrtces1,
            vbtadcr2::Vbrtces1,
            Vbtadcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT RTC Time Capture Event Source Select 2"]
    #[inline(always)]
    pub fn vbrtces2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        vbtadcr2::Vbrtces2,
        vbtadcr2::Vbrtces2,
        Vbtadcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            vbtadcr2::Vbrtces2,
            vbtadcr2::Vbrtces2,
            Vbtadcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Vbtadcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Vbtadcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtadcr2 {
    #[inline(always)]
    fn default() -> Vbtadcr2 {
        <crate::RegValueT<Vbtadcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtadcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbrtces0_SPEC;
    pub type Vbrtces0 = crate::EnumBitfieldStruct<u8, Vbrtces0_SPEC>;
    impl Vbrtces0 {
        #[doc = "RTCIC0"]
        pub const _0: Self = Self::new(0);

        #[doc = "VBTADF0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbrtces1_SPEC;
    pub type Vbrtces1 = crate::EnumBitfieldStruct<u8, Vbrtces1_SPEC>;
    impl Vbrtces1 {
        #[doc = "RTCIC1"]
        pub const _0: Self = Self::new(0);

        #[doc = "VBTADF1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbrtces2_SPEC;
    pub type Vbrtces2 = crate::EnumBitfieldStruct<u8, Vbrtces2_SPEC>;
    impl Vbrtces2 {
        #[doc = "RTCIC2"]
        pub const _0: Self = Self::new(0);

        #[doc = "VBTADF2"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "RTCIC0 Input Enable"]
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

    #[doc = "RTCIC1 Input Enable"]
    #[inline(always)]
    pub fn vch1inen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vbtictlr::Vch1Inen,
        vbtictlr::Vch1Inen,
        Vbtictlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vbtictlr::Vch1Inen,
            vbtictlr::Vch1Inen,
            Vbtictlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTCIC2 Input Enable"]
    #[inline(always)]
    pub fn vch2inen(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        vbtictlr::Vch2Inen,
        vbtictlr::Vch2Inen,
        Vbtictlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            vbtictlr::Vch2Inen,
            vbtictlr::Vch2Inen,
            Vbtictlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Vbtictlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Vbtictlr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch1Inen_SPEC;
    pub type Vch1Inen = crate::EnumBitfieldStruct<u8, Vch1Inen_SPEC>;
    impl Vch1Inen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch2Inen_SPEC;
    pub type Vch2Inen = crate::EnumBitfieldStruct<u8, Vch2Inen_SPEC>;
    impl Vch2Inen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtictlr2_SPEC;
impl crate::sealed::RegSpec for Vbtictlr2_SPEC {
    type DataType = u8;
}

#[doc = "VBATT Input Control Register 2"]
pub type Vbtictlr2 = crate::RegValueT<Vbtictlr2_SPEC>;

impl Vbtictlr2 {
    #[doc = "VBATT CH0 Input Noise Canceler Enable"]
    #[inline(always)]
    pub fn vch0nce(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtictlr2::Vch0Nce,
        vbtictlr2::Vch0Nce,
        Vbtictlr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtictlr2::Vch0Nce,
            vbtictlr2::Vch0Nce,
            Vbtictlr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT CH1 Input Noise Canceler Enable"]
    #[inline(always)]
    pub fn vch1nce(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vbtictlr2::Vch1Nce,
        vbtictlr2::Vch1Nce,
        Vbtictlr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vbtictlr2::Vch1Nce,
            vbtictlr2::Vch1Nce,
            Vbtictlr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT CH2 Input Noise Canceler Enable"]
    #[inline(always)]
    pub fn vch2nce(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        vbtictlr2::Vch2Nce,
        vbtictlr2::Vch2Nce,
        Vbtictlr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            vbtictlr2::Vch2Nce,
            vbtictlr2::Vch2Nce,
            Vbtictlr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT CH0 Input Edge Select"]
    #[inline(always)]
    pub fn vch0eg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        vbtictlr2::Vch0Eg,
        vbtictlr2::Vch0Eg,
        Vbtictlr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            vbtictlr2::Vch0Eg,
            vbtictlr2::Vch0Eg,
            Vbtictlr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT CH1 Input Edge Select"]
    #[inline(always)]
    pub fn vch1eg(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        vbtictlr2::Vch1Eg,
        vbtictlr2::Vch1Eg,
        Vbtictlr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            vbtictlr2::Vch1Eg,
            vbtictlr2::Vch1Eg,
            Vbtictlr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT CH2 Input Edge Select"]
    #[inline(always)]
    pub fn vch2eg(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        vbtictlr2::Vch2Eg,
        vbtictlr2::Vch2Eg,
        Vbtictlr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            vbtictlr2::Vch2Eg,
            vbtictlr2::Vch2Eg,
            Vbtictlr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Vbtictlr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Vbtictlr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtictlr2 {
    #[inline(always)]
    fn default() -> Vbtictlr2 {
        <crate::RegValueT<Vbtictlr2_SPEC> as RegisterValue<_>>::new(112)
    }
}
pub mod vbtictlr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch0Nce_SPEC;
    pub type Vch0Nce = crate::EnumBitfieldStruct<u8, Vch0Nce_SPEC>;
    impl Vch0Nce {
        #[doc = "RTCIC0 pin input noise canceler disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCIC0 pin input noise canceler enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch1Nce_SPEC;
    pub type Vch1Nce = crate::EnumBitfieldStruct<u8, Vch1Nce_SPEC>;
    impl Vch1Nce {
        #[doc = "RTCIC1 pin inputs noise canceler disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCIC1 pin input noise canceler enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch2Nce_SPEC;
    pub type Vch2Nce = crate::EnumBitfieldStruct<u8, Vch2Nce_SPEC>;
    impl Vch2Nce {
        #[doc = "RTCIC2 pin input noise canceler disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCIC2 pin input noise canceler enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch0Eg_SPEC;
    pub type Vch0Eg = crate::EnumBitfieldStruct<u8, Vch0Eg_SPEC>;
    impl Vch0Eg {
        #[doc = "RTCIC0 pin input event is detected on falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCIC0 pin input event is detected on rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch1Eg_SPEC;
    pub type Vch1Eg = crate::EnumBitfieldStruct<u8, Vch1Eg_SPEC>;
    impl Vch1Eg {
        #[doc = "RTCIC1 pin input event is detected on falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCIC1 pin input event is detected on rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch2Eg_SPEC;
    pub type Vch2Eg = crate::EnumBitfieldStruct<u8, Vch2Eg_SPEC>;
    impl Vch2Eg {
        #[doc = "RTCIC2 pin input event is detected on falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCIC2 pin input event is detected on rising edge"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtimonr_SPEC;
impl crate::sealed::RegSpec for Vbtimonr_SPEC {
    type DataType = u8;
}

#[doc = "VBATT Input Monitor Register"]
pub type Vbtimonr = crate::RegValueT<Vbtimonr_SPEC>;

impl Vbtimonr {
    #[doc = "VBATT CH0 Input monitor"]
    #[inline(always)]
    pub fn vch0mon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtimonr::Vch0Mon,
        vbtimonr::Vch0Mon,
        Vbtimonr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtimonr::Vch0Mon,
            vbtimonr::Vch0Mon,
            Vbtimonr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT CH1 Input monitor"]
    #[inline(always)]
    pub fn vch1mon(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vbtimonr::Vch1Mon,
        vbtimonr::Vch1Mon,
        Vbtimonr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vbtimonr::Vch1Mon,
            vbtimonr::Vch1Mon,
            Vbtimonr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT CH2 Input monitor"]
    #[inline(always)]
    pub fn vch2mon(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        vbtimonr::Vch2Mon,
        vbtimonr::Vch2Mon,
        Vbtimonr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            vbtimonr::Vch2Mon,
            vbtimonr::Vch2Mon,
            Vbtimonr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Vbtimonr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Vbtimonr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtimonr {
    #[inline(always)]
    fn default() -> Vbtimonr {
        <crate::RegValueT<Vbtimonr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtimonr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch0Mon_SPEC;
    pub type Vch0Mon = crate::EnumBitfieldStruct<u8, Vch0Mon_SPEC>;
    impl Vch0Mon {
        #[doc = "RTCIC0 pin input is low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCIC0 pin input is high level."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch1Mon_SPEC;
    pub type Vch1Mon = crate::EnumBitfieldStruct<u8, Vch1Mon_SPEC>;
    impl Vch1Mon {
        #[doc = "RTCIC1 pin input is low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCIC1 pin input is high level."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch2Mon_SPEC;
    pub type Vch2Mon = crate::EnumBitfieldStruct<u8, Vch2Mon_SPEC>;
    impl Vch2Mon {
        #[doc = "RTCIC2 pin input is low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCIC2 pin input is high level."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtbkr_SPEC;
impl crate::sealed::RegSpec for Vbtbkr_SPEC {
    type DataType = u8;
}

#[doc = "VBATT Backup Register %s"]
pub type Vbtbkr = crate::RegValueT<Vbtbkr_SPEC>;

impl Vbtbkr {
    #[doc = "VBTBKRn \\[7:0\\] (n=0 to 127)"]
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
