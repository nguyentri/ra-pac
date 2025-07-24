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
// Generated from SVD 1.1, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:46:11 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"16-bit A/D Converter"]
unsafe impl ::core::marker::Send for super::Adc160 {}
unsafe impl ::core::marker::Sync for super::Adc160 {}
impl super::Adc160 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "A/D Control Register"]
    #[inline(always)]
    pub const fn adcsr(&self) -> &'static crate::common::Reg<self::Adcsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "A/D Channel Select Register A0"]
    #[inline(always)]
    pub const fn adansa0(
        &self,
    ) -> &'static crate::common::Reg<self::Adansa0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adansa0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "A/D Channel Select Register A1"]
    #[inline(always)]
    pub const fn adansa1(
        &self,
    ) -> &'static crate::common::Reg<self::Adansa1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adansa1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "A/D-Converted Value Average Channel Select Register 0"]
    #[inline(always)]
    pub const fn adads0(
        &self,
    ) -> &'static crate::common::Reg<self::Adads0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adads0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "A/D-Converted Value Average Channel Select Register 1"]
    #[inline(always)]
    pub const fn adads1(
        &self,
    ) -> &'static crate::common::Reg<self::Adads1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adads1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "A/D-Converted Value Average Count Select Register"]
    #[inline(always)]
    pub const fn adadc(&self) -> &'static crate::common::Reg<self::Adadc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adadc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "A/D Control Extended Register"]
    #[inline(always)]
    pub const fn adcer(&self) -> &'static crate::common::Reg<self::Adcer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "A/D Conversion Start Trigger Select Register"]
    #[inline(always)]
    pub const fn adstrgr(
        &self,
    ) -> &'static crate::common::Reg<self::Adstrgr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adstrgr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "A/D Conversion Extended Input Control Register"]
    #[inline(always)]
    pub const fn adexicr(
        &self,
    ) -> &'static crate::common::Reg<self::Adexicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adexicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "A/D Channel Select Register B0"]
    #[inline(always)]
    pub const fn adansb0(
        &self,
    ) -> &'static crate::common::Reg<self::Adansb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adansb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "A/D Channel Select Register B1"]
    #[inline(always)]
    pub const fn adansb1(
        &self,
    ) -> &'static crate::common::Reg<self::Adansb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adansb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "A/D Data Duplexing Register"]
    #[inline(always)]
    pub const fn addbldr(
        &self,
    ) -> &'static crate::common::Reg<self::Addbldr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addbldr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "A/D Temperature Sensor Data Register"]
    #[inline(always)]
    pub const fn adtsdr(&self) -> &'static crate::common::Reg<self::Adtsdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adtsdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[doc = "A/D Internal Reference Voltage Data Register"]
    #[inline(always)]
    pub const fn adocdr(&self) -> &'static crate::common::Reg<self::Adocdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adocdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "A/D Data Register %s"]
    #[inline(always)]
    pub const fn addr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Addr_SPEC, crate::common::R>,
        9,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }
    #[inline(always)]
    pub const fn addr16(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn addr17(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x42usize),
            )
        }
    }
    #[inline(always)]
    pub const fn addr18(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn addr19(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x46usize),
            )
        }
    }
    #[inline(always)]
    pub const fn addr20(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn addr21(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x4ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn addr22(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn addr23(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x4eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn addr24(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }

    #[doc = "A/D Disconnection Detection Control Register"]
    #[inline(always)]
    pub const fn addiscr(
        &self,
    ) -> &'static crate::common::Reg<self::Addiscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Addiscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(122usize),
            )
        }
    }

    #[doc = "A/D Interrupt Control Register"]
    #[inline(always)]
    pub const fn adicr(&self) -> &'static crate::common::Reg<self::Adicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(125usize),
            )
        }
    }

    #[doc = "A/D Group Scan Priority Control Register"]
    #[inline(always)]
    pub const fn adgspcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adgspcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adgspcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "A/D Data Duplexing Register A"]
    #[inline(always)]
    pub const fn addbldra(
        &self,
    ) -> &'static crate::common::Reg<self::Addbldra_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addbldra_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "A/D Data Duplexing Register B"]
    #[inline(always)]
    pub const fn addbldrb(
        &self,
    ) -> &'static crate::common::Reg<self::Addbldrb_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addbldrb_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(134usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window A/B Status Monitor Register"]
    #[inline(always)]
    pub const fn adwinmon(
        &self,
    ) -> &'static crate::common::Reg<self::Adwinmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adwinmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "A/D Compare Function Control Register"]
    #[inline(always)]
    pub const fn adcmpcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window A Extended Input Select Register"]
    #[inline(always)]
    pub const fn adcmpanser(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpanser_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpanser_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(146usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window A Extended Input Comparison Condition Setting Register"]
    #[inline(always)]
    pub const fn adcmpler(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpler_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpler_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(147usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window A Channel Select Register 0"]
    #[inline(always)]
    pub const fn adcmpansr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpansr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpansr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window A Channel Select Register 1"]
    #[inline(always)]
    pub const fn adcmpansr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpansr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpansr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(150usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window A Comparison Condition Setting Register 0"]
    #[inline(always)]
    pub const fn adcmplr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmplr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmplr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window A Comparison Condition Setting Register 1"]
    #[inline(always)]
    pub const fn adcmplr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmplr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmplr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(154usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window A Lower-Side Level Setting Register"]
    #[inline(always)]
    pub const fn adcmpdr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpdr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpdr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window A Upper-Side Level Setting Register"]
    #[inline(always)]
    pub const fn adcmpdr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpdr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpdr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(158usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window A Channel Status Register 0"]
    #[inline(always)]
    pub const fn adcmpsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpsr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpsr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window A Channel Status Register 1"]
    #[inline(always)]
    pub const fn adcmpsr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpsr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpsr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(162usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window A Extended Input Channel Status Register"]
    #[inline(always)]
    pub const fn adcmpser(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpser_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpser_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window B Channel Selection Register"]
    #[inline(always)]
    pub const fn adcmpbnsr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpbnsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpbnsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(166usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window B Lower-Side Level Setting Register"]
    #[inline(always)]
    pub const fn adwinllb(
        &self,
    ) -> &'static crate::common::Reg<self::Adwinllb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adwinllb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window B Upper-Side Level Setting Register"]
    #[inline(always)]
    pub const fn adwinulb(
        &self,
    ) -> &'static crate::common::Reg<self::Adwinulb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adwinulb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(170usize),
            )
        }
    }

    #[doc = "A/D Compare Function Window B Status Register"]
    #[inline(always)]
    pub const fn adcmpbsr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpbsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpbsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[doc = "A/D Sampling State Register L"]
    #[inline(always)]
    pub const fn adsstrl(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(221usize),
            )
        }
    }

    #[doc = "A/D Sampling State Register T"]
    #[inline(always)]
    pub const fn adsstrt(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstrt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstrt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(222usize),
            )
        }
    }

    #[doc = "A/D Sampling State Register O"]
    #[inline(always)]
    pub const fn adsstro(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstro_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstro_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(223usize),
            )
        }
    }

    #[doc = "A/D Sampling State Register %s"]
    #[inline(always)]
    pub const fn adsstr0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adsstr0_SPEC, crate::common::RW>,
        9,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe0usize))
        }
    }
    #[inline(always)]
    pub const fn adsstr00(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr01(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr02(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr03(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr04(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr05(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr06(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr07(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr08(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe8usize),
            )
        }
    }

    #[doc = "A/D Channel Input Mode Select Register"]
    #[inline(always)]
    pub const fn adanim(
        &self,
    ) -> &'static crate::common::Reg<self::Adanim_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adanim_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[doc = "A/D Calibration Execution Register"]
    #[inline(always)]
    pub const fn adcalexe(
        &self,
    ) -> &'static crate::common::Reg<self::Adcalexe_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcalexe_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(242usize),
            )
        }
    }

    #[doc = "A/D Dedicated Reference Voltage Circuit Control Register"]
    #[inline(always)]
    pub const fn vrefampcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Vrefampcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vrefampcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[doc = "A/D Self-Diagnosis Data Register"]
    #[inline(always)]
    pub const fn adrd(&self) -> &'static crate::common::Reg<self::Adrd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adrd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }

    #[doc = "A/D Self-Diagnostic Status Register"]
    #[inline(always)]
    pub const fn adrst(&self) -> &'static crate::common::Reg<self::Adrst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adrst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(250usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcsr_SPEC;
impl crate::sealed::RegSpec for Adcsr_SPEC {
    type DataType = u16;
}

#[doc = "A/D Control Register"]
pub type Adcsr = crate::RegValueT<Adcsr_SPEC>;

impl Adcsr {
    #[doc = "A/D Conversion Start"]
    #[inline(always)]
    pub fn adst(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adcsr::Adst,
        adcsr::Adst,
        Adcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adcsr::Adst,
            adcsr::Adst,
            Adcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Scan Mode Select"]
    #[inline(always)]
    pub fn adcs(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x3,
        1,
        0,
        adcsr::Adcs,
        adcsr::Adcs,
        Adcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x3,
            1,
            0,
            adcsr::Adcs,
            adcsr::Adcs,
            Adcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Operation Mode Select"]
    #[inline(always)]
    pub fn adhsc(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcsr::Adhsc,
        adcsr::Adhsc,
        Adcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcsr::Adhsc,
            adcsr::Adhsc,
            Adcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Trigger Start Enable"]
    #[inline(always)]
    pub fn trge(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcsr::Trge,
        adcsr::Trge,
        Adcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcsr::Trge,
            adcsr::Trge,
            Adcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Trigger Select"]
    #[inline(always)]
    pub fn extrg(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcsr::Extrg,
        adcsr::Extrg,
        Adcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcsr::Extrg,
            adcsr::Extrg,
            Adcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Double Trigger Mode Select"]
    #[inline(always)]
    pub fn dble(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcsr::Dble,
        adcsr::Dble,
        Adcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcsr::Dble,
            adcsr::Dble,
            Adcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Group B Scan End Interrupt Enable"]
    #[inline(always)]
    pub fn gbadie(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcsr::Gbadie,
        adcsr::Gbadie,
        Adcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcsr::Gbadie,
            adcsr::Gbadie,
            Adcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Adcsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Adcsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Double Trigger Channel SelectThese bits select one analog input channel for double triggered operation. The setting is only effective while double trigger mode is selected."]
    #[inline(always)]
    pub fn dblans(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Adcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Adcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adcsr {
    #[inline(always)]
    fn default() -> Adcsr {
        <crate::RegValueT<Adcsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adst_SPEC;
    pub type Adst = crate::EnumBitfieldStruct<u8, Adst_SPEC>;
    impl Adst {
        #[doc = "Stops A/D conversion process."]
        pub const _0: Self = Self::new(0);

        #[doc = "Starts A/D conversion process."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adcs_SPEC;
    pub type Adcs = crate::EnumBitfieldStruct<u8, Adcs_SPEC>;
    impl Adcs {
        #[doc = "Single scan mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "Group scan mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "Continuous scan mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adhsc_SPEC;
    pub type Adhsc = crate::EnumBitfieldStruct<u8, Adhsc_SPEC>;
    impl Adhsc {
        #[doc = "High speed A/D conversion mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low current A/D conversion mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trge_SPEC;
    pub type Trge = crate::EnumBitfieldStruct<u8, Trge_SPEC>;
    impl Trge {
        #[doc = "Disables A/D conversion to be started by the synchronous or asynchronous trigger."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables A/D conversion to be started by the synchronous or asynchronous trigger."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Extrg_SPEC;
    pub type Extrg = crate::EnumBitfieldStruct<u8, Extrg_SPEC>;
    impl Extrg {
        #[doc = "A/D conversion is started by the synchronous trigger (ELC)."]
        pub const _0: Self = Self::new(0);

        #[doc = "A/D conversion is started by the asynchronous trigger (ADTRG0)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dble_SPEC;
    pub type Dble = crate::EnumBitfieldStruct<u8, Dble_SPEC>;
    impl Dble {
        #[doc = "Double trigger mode non-selection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Double trigger mode selection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gbadie_SPEC;
    pub type Gbadie = crate::EnumBitfieldStruct<u8, Gbadie_SPEC>;
    impl Gbadie {
        #[doc = "Disables ADC160_GBADI interrupt generation upon group B scan completion."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables ADC160_GBADI interrupt generation upon group B scan completion."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adansa0_SPEC;
impl crate::sealed::RegSpec for Adansa0_SPEC {
    type DataType = u16;
}

#[doc = "A/D Channel Select Register A0"]
pub type Adansa0 = crate::RegValueT<Adansa0_SPEC>;

impl Adansa0 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Adansa0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Adansa0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AN008 Select"]
    #[inline(always)]
    pub fn ansa08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adansa0::Ansa08,
        adansa0::Ansa08,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adansa0::Ansa08,
            adansa0::Ansa08,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN007 Select"]
    #[inline(always)]
    pub fn ansa07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adansa0::Ansa07,
        adansa0::Ansa07,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adansa0::Ansa07,
            adansa0::Ansa07,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN006 Select"]
    #[inline(always)]
    pub fn ansa06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adansa0::Ansa06,
        adansa0::Ansa06,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adansa0::Ansa06,
            adansa0::Ansa06,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN005 Select"]
    #[inline(always)]
    pub fn ansa05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adansa0::Ansa05,
        adansa0::Ansa05,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adansa0::Ansa05,
            adansa0::Ansa05,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN004 Select"]
    #[inline(always)]
    pub fn ansa04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adansa0::Ansa04,
        adansa0::Ansa04,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adansa0::Ansa04,
            adansa0::Ansa04,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN003 Select"]
    #[inline(always)]
    pub fn ansa03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adansa0::Ansa03,
        adansa0::Ansa03,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adansa0::Ansa03,
            adansa0::Ansa03,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN002 Select"]
    #[inline(always)]
    pub fn ansa02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adansa0::Ansa02,
        adansa0::Ansa02,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adansa0::Ansa02,
            adansa0::Ansa02,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN001 Select"]
    #[inline(always)]
    pub fn ansa01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adansa0::Ansa01,
        adansa0::Ansa01,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adansa0::Ansa01,
            adansa0::Ansa01,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN000 Select"]
    #[inline(always)]
    pub fn ansa00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adansa0::Ansa00,
        adansa0::Ansa00,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adansa0::Ansa00,
            adansa0::Ansa00,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adansa0 {
    #[inline(always)]
    fn default() -> Adansa0 {
        <crate::RegValueT<Adansa0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adansa0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa08_SPEC;
    pub type Ansa08 = crate::EnumBitfieldStruct<u8, Ansa08_SPEC>;
    impl Ansa08 {
        #[doc = "AN008 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN008 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa07_SPEC;
    pub type Ansa07 = crate::EnumBitfieldStruct<u8, Ansa07_SPEC>;
    impl Ansa07 {
        #[doc = "AN007 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN007 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa06_SPEC;
    pub type Ansa06 = crate::EnumBitfieldStruct<u8, Ansa06_SPEC>;
    impl Ansa06 {
        #[doc = "AN006 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN006 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa05_SPEC;
    pub type Ansa05 = crate::EnumBitfieldStruct<u8, Ansa05_SPEC>;
    impl Ansa05 {
        #[doc = "AN005 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN005 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa04_SPEC;
    pub type Ansa04 = crate::EnumBitfieldStruct<u8, Ansa04_SPEC>;
    impl Ansa04 {
        #[doc = "AN004 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN004 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa03_SPEC;
    pub type Ansa03 = crate::EnumBitfieldStruct<u8, Ansa03_SPEC>;
    impl Ansa03 {
        #[doc = "AN003 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN003 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa02_SPEC;
    pub type Ansa02 = crate::EnumBitfieldStruct<u8, Ansa02_SPEC>;
    impl Ansa02 {
        #[doc = "AN002 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN002 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa01_SPEC;
    pub type Ansa01 = crate::EnumBitfieldStruct<u8, Ansa01_SPEC>;
    impl Ansa01 {
        #[doc = "AN001 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN001 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa00_SPEC;
    pub type Ansa00 = crate::EnumBitfieldStruct<u8, Ansa00_SPEC>;
    impl Ansa00 {
        #[doc = "AN000 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN000 is selected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adansa1_SPEC;
impl crate::sealed::RegSpec for Adansa1_SPEC {
    type DataType = u16;
}

#[doc = "A/D Channel Select Register A1"]
pub type Adansa1 = crate::RegValueT<Adansa1_SPEC>;

impl Adansa1 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Adansa1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Adansa1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AN024 Select"]
    #[inline(always)]
    pub fn ansa24(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adansa1::Ansa24,
        adansa1::Ansa24,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adansa1::Ansa24,
            adansa1::Ansa24,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN023 Select"]
    #[inline(always)]
    pub fn ansa23(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adansa1::Ansa23,
        adansa1::Ansa23,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adansa1::Ansa23,
            adansa1::Ansa23,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN022 Select"]
    #[inline(always)]
    pub fn ansa22(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adansa1::Ansa22,
        adansa1::Ansa22,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adansa1::Ansa22,
            adansa1::Ansa22,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN021 Select"]
    #[inline(always)]
    pub fn ansa21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adansa1::Ansa21,
        adansa1::Ansa21,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adansa1::Ansa21,
            adansa1::Ansa21,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN020 Select"]
    #[inline(always)]
    pub fn ansa20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adansa1::Ansa20,
        adansa1::Ansa20,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adansa1::Ansa20,
            adansa1::Ansa20,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN019 Select"]
    #[inline(always)]
    pub fn ansa19(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adansa1::Ansa19,
        adansa1::Ansa19,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adansa1::Ansa19,
            adansa1::Ansa19,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN018 Select"]
    #[inline(always)]
    pub fn ansa18(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adansa1::Ansa18,
        adansa1::Ansa18,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adansa1::Ansa18,
            adansa1::Ansa18,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN017 Select"]
    #[inline(always)]
    pub fn ansa17(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adansa1::Ansa17,
        adansa1::Ansa17,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adansa1::Ansa17,
            adansa1::Ansa17,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN016 Select"]
    #[inline(always)]
    pub fn ansa16(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adansa1::Ansa16,
        adansa1::Ansa16,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adansa1::Ansa16,
            adansa1::Ansa16,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adansa1 {
    #[inline(always)]
    fn default() -> Adansa1 {
        <crate::RegValueT<Adansa1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adansa1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa24_SPEC;
    pub type Ansa24 = crate::EnumBitfieldStruct<u8, Ansa24_SPEC>;
    impl Ansa24 {
        #[doc = "AN024 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN024 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa23_SPEC;
    pub type Ansa23 = crate::EnumBitfieldStruct<u8, Ansa23_SPEC>;
    impl Ansa23 {
        #[doc = "AN023 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN023 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa22_SPEC;
    pub type Ansa22 = crate::EnumBitfieldStruct<u8, Ansa22_SPEC>;
    impl Ansa22 {
        #[doc = "AN022 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN022 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa21_SPEC;
    pub type Ansa21 = crate::EnumBitfieldStruct<u8, Ansa21_SPEC>;
    impl Ansa21 {
        #[doc = "AN021 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN021 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa20_SPEC;
    pub type Ansa20 = crate::EnumBitfieldStruct<u8, Ansa20_SPEC>;
    impl Ansa20 {
        #[doc = "AN020 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN020 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa19_SPEC;
    pub type Ansa19 = crate::EnumBitfieldStruct<u8, Ansa19_SPEC>;
    impl Ansa19 {
        #[doc = "AN019 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN019 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa18_SPEC;
    pub type Ansa18 = crate::EnumBitfieldStruct<u8, Ansa18_SPEC>;
    impl Ansa18 {
        #[doc = "AN018 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN018 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa17_SPEC;
    pub type Ansa17 = crate::EnumBitfieldStruct<u8, Ansa17_SPEC>;
    impl Ansa17 {
        #[doc = "AN017 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN017 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa16_SPEC;
    pub type Ansa16 = crate::EnumBitfieldStruct<u8, Ansa16_SPEC>;
    impl Ansa16 {
        #[doc = "AN016 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN016 is selected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adads0_SPEC;
impl crate::sealed::RegSpec for Adads0_SPEC {
    type DataType = u16;
}

#[doc = "A/D-Converted Value Average Channel Select Register 0"]
pub type Adads0 = crate::RegValueT<Adads0_SPEC>;

impl Adads0 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Adads0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Adads0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "A/D-Converted Value Average Channel AN008 Select"]
    #[inline(always)]
    pub fn ads08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adads0::Ads08,
        adads0::Ads08,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adads0::Ads08,
            adads0::Ads08,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN007 Select"]
    #[inline(always)]
    pub fn ads07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adads0::Ads07,
        adads0::Ads07,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adads0::Ads07,
            adads0::Ads07,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN006 Select"]
    #[inline(always)]
    pub fn ads06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adads0::Ads06,
        adads0::Ads06,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adads0::Ads06,
            adads0::Ads06,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN005 Select"]
    #[inline(always)]
    pub fn ads05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adads0::Ads05,
        adads0::Ads05,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adads0::Ads05,
            adads0::Ads05,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN004 Select"]
    #[inline(always)]
    pub fn ads04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adads0::Ads04,
        adads0::Ads04,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adads0::Ads04,
            adads0::Ads04,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN003 Select"]
    #[inline(always)]
    pub fn ads03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adads0::Ads03,
        adads0::Ads03,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adads0::Ads03,
            adads0::Ads03,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN002 Select"]
    #[inline(always)]
    pub fn ads02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adads0::Ads02,
        adads0::Ads02,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adads0::Ads02,
            adads0::Ads02,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN001 Select"]
    #[inline(always)]
    pub fn ads01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adads0::Ads01,
        adads0::Ads01,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adads0::Ads01,
            adads0::Ads01,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN000 Select"]
    #[inline(always)]
    pub fn ads00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adads0::Ads00,
        adads0::Ads00,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adads0::Ads00,
            adads0::Ads00,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adads0 {
    #[inline(always)]
    fn default() -> Adads0 {
        <crate::RegValueT<Adads0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adads0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads08_SPEC;
    pub type Ads08 = crate::EnumBitfieldStruct<u8, Ads08_SPEC>;
    impl Ads08 {
        #[doc = "AN008 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN008 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads07_SPEC;
    pub type Ads07 = crate::EnumBitfieldStruct<u8, Ads07_SPEC>;
    impl Ads07 {
        #[doc = "AN007 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN007 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads06_SPEC;
    pub type Ads06 = crate::EnumBitfieldStruct<u8, Ads06_SPEC>;
    impl Ads06 {
        #[doc = "AN006 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN006 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads05_SPEC;
    pub type Ads05 = crate::EnumBitfieldStruct<u8, Ads05_SPEC>;
    impl Ads05 {
        #[doc = "AN005 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN005 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads04_SPEC;
    pub type Ads04 = crate::EnumBitfieldStruct<u8, Ads04_SPEC>;
    impl Ads04 {
        #[doc = "AN004 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN004 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads03_SPEC;
    pub type Ads03 = crate::EnumBitfieldStruct<u8, Ads03_SPEC>;
    impl Ads03 {
        #[doc = "AN003 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN003 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads02_SPEC;
    pub type Ads02 = crate::EnumBitfieldStruct<u8, Ads02_SPEC>;
    impl Ads02 {
        #[doc = "AN002 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN002 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads01_SPEC;
    pub type Ads01 = crate::EnumBitfieldStruct<u8, Ads01_SPEC>;
    impl Ads01 {
        #[doc = "AN001 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN001 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads00_SPEC;
    pub type Ads00 = crate::EnumBitfieldStruct<u8, Ads00_SPEC>;
    impl Ads00 {
        #[doc = "AN000 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN000 is selected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adads1_SPEC;
impl crate::sealed::RegSpec for Adads1_SPEC {
    type DataType = u16;
}

#[doc = "A/D-Converted Value Average Channel Select Register 1"]
pub type Adads1 = crate::RegValueT<Adads1_SPEC>;

impl Adads1 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Adads1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Adads1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "A/D-Converted Value Average Channel AN024 Select"]
    #[inline(always)]
    pub fn ads24(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adads1::Ads24,
        adads1::Ads24,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adads1::Ads24,
            adads1::Ads24,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN023 Select"]
    #[inline(always)]
    pub fn ads23(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adads1::Ads23,
        adads1::Ads23,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adads1::Ads23,
            adads1::Ads23,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN022 Select"]
    #[inline(always)]
    pub fn ads22(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adads1::Ads22,
        adads1::Ads22,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adads1::Ads22,
            adads1::Ads22,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN021 Select"]
    #[inline(always)]
    pub fn ads21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adads1::Ads21,
        adads1::Ads21,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adads1::Ads21,
            adads1::Ads21,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN020 Select"]
    #[inline(always)]
    pub fn ads20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adads1::Ads20,
        adads1::Ads20,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adads1::Ads20,
            adads1::Ads20,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN019 Select"]
    #[inline(always)]
    pub fn ads19(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adads1::Ads19,
        adads1::Ads19,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adads1::Ads19,
            adads1::Ads19,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN018 Select"]
    #[inline(always)]
    pub fn ads18(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adads1::Ads18,
        adads1::Ads18,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adads1::Ads18,
            adads1::Ads18,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN017 Select"]
    #[inline(always)]
    pub fn ads17(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adads1::Ads17,
        adads1::Ads17,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adads1::Ads17,
            adads1::Ads17,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Average Channel AN016 Select"]
    #[inline(always)]
    pub fn ads16(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adads1::Ads16,
        adads1::Ads16,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adads1::Ads16,
            adads1::Ads16,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adads1 {
    #[inline(always)]
    fn default() -> Adads1 {
        <crate::RegValueT<Adads1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adads1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads24_SPEC;
    pub type Ads24 = crate::EnumBitfieldStruct<u8, Ads24_SPEC>;
    impl Ads24 {
        #[doc = "AN024 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN024 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads23_SPEC;
    pub type Ads23 = crate::EnumBitfieldStruct<u8, Ads23_SPEC>;
    impl Ads23 {
        #[doc = "AN023 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN023 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads22_SPEC;
    pub type Ads22 = crate::EnumBitfieldStruct<u8, Ads22_SPEC>;
    impl Ads22 {
        #[doc = "AN022 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN022 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads21_SPEC;
    pub type Ads21 = crate::EnumBitfieldStruct<u8, Ads21_SPEC>;
    impl Ads21 {
        #[doc = "AN021 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN021 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads20_SPEC;
    pub type Ads20 = crate::EnumBitfieldStruct<u8, Ads20_SPEC>;
    impl Ads20 {
        #[doc = "AN020 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN020 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads19_SPEC;
    pub type Ads19 = crate::EnumBitfieldStruct<u8, Ads19_SPEC>;
    impl Ads19 {
        #[doc = "AN019 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN019 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads18_SPEC;
    pub type Ads18 = crate::EnumBitfieldStruct<u8, Ads18_SPEC>;
    impl Ads18 {
        #[doc = "AN018 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN018 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads17_SPEC;
    pub type Ads17 = crate::EnumBitfieldStruct<u8, Ads17_SPEC>;
    impl Ads17 {
        #[doc = "AN017 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN017 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads16_SPEC;
    pub type Ads16 = crate::EnumBitfieldStruct<u8, Ads16_SPEC>;
    impl Ads16 {
        #[doc = "AN016 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN016 is selected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adadc_SPEC;
impl crate::sealed::RegSpec for Adadc_SPEC {
    type DataType = u8;
}

#[doc = "A/D-Converted Value Average Count Select Register"]
pub type Adadc = crate::RegValueT<Adadc_SPEC>;

impl Adadc {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Adadc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Adadc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Count Select"]
    #[inline(always)]
    pub fn adc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        adadc::Adc,
        adadc::Adc,
        Adadc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            adadc::Adc,
            adadc::Adc,
            Adadc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adadc {
    #[inline(always)]
    fn default() -> Adadc {
        <crate::RegValueT<Adadc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adadc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adc_SPEC;
    pub type Adc = crate::EnumBitfieldStruct<u8, Adc_SPEC>;
    impl Adc {
        #[doc = "1-time conversion (same as normal conversion)"]
        pub const _000: Self = Self::new(0);

        #[doc = "2-time conversion (acquire the average of 2-time conversion)"]
        pub const _001: Self = Self::new(1);

        #[doc = "4-time conversion (acquire the average of 4-time conversion)"]
        pub const _011: Self = Self::new(3);

        #[doc = "8-time conversion (acquire the average of 8-time conversion)"]
        pub const _100: Self = Self::new(4);

        #[doc = "16-time conversion (acquire the average of 16-time conversion)"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcer_SPEC;
impl crate::sealed::RegSpec for Adcer_SPEC {
    type DataType = u16;
}

#[doc = "A/D Control Extended Register"]
pub type Adcer = crate::RegValueT<Adcer_SPEC>;

impl Adcer {
    #[doc = "Single-End Input A/D Converted Data Inversion Select"]
    #[inline(always)]
    pub fn adinv(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adcer::Adinv,
        adcer::Adinv,
        Adcer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adcer::Adinv,
            adcer::Adinv,
            Adcer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Self-Diagnosis Enable"]
    #[inline(always)]
    pub fn diagm(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adcer::Diagm,
        adcer::Diagm,
        Adcer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adcer::Diagm,
            adcer::Diagm,
            Adcer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Self-Diagnosis Mode Select"]
    #[inline(always)]
    pub fn diagld(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcer::Diagld,
        adcer::Diagld,
        Adcer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcer::Diagld,
            adcer::Diagld,
            Adcer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Self-Diagnosis Conversion Voltage Select"]
    #[inline(always)]
    pub fn diagval(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        adcer::Diagval,
        adcer::Diagval,
        Adcer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            adcer::Diagval,
            adcer::Diagval,
            Adcer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Data Register Automatic Clearing Enable"]
    #[inline(always)]
    pub fn ace(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcer::Ace,
        adcer::Ace,
        Adcer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcer::Ace,
            adcer::Ace,
            Adcer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Adcer_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Adcer_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adcer {
    #[inline(always)]
    fn default() -> Adcer {
        <crate::RegValueT<Adcer_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcer {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adinv_SPEC;
    pub type Adinv = crate::EnumBitfieldStruct<u8, Adinv_SPEC>;
    impl Adinv {
        #[doc = "Data is stored in a range of -32768 to 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data is stored in a range of 0 to 32767"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diagm_SPEC;
    pub type Diagm = crate::EnumBitfieldStruct<u8, Diagm_SPEC>;
    impl Diagm {
        #[doc = "Self-diagnosis of ADC16 disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Self-diagnosis of ADC16 enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diagld_SPEC;
    pub type Diagld = crate::EnumBitfieldStruct<u8, Diagld_SPEC>;
    impl Diagld {
        #[doc = "Rotation mode for self-diagnosis voltage"]
        pub const _0: Self = Self::new(0);

        #[doc = "Fixed mode for self-diagnosis voltage"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diagval_SPEC;
    pub type Diagval = crate::EnumBitfieldStruct<u8, Diagval_SPEC>;
    impl Diagval {
        #[doc = "Setting prohibited when self-diagnosis is enabled"]
        pub const _00: Self = Self::new(0);

        #[doc = "-VREFH0 (Ideal value of A/D conversion result is 8000h)"]
        pub const _01: Self = Self::new(1);

        #[doc = "VREFL0 (Ideal value of A/D conversion result is 0000h)"]
        pub const _10: Self = Self::new(2);

        #[doc = "VREFH0 (Ideal value of A/D conversion result is 7FFFh)."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ace_SPEC;
    pub type Ace = crate::EnumBitfieldStruct<u8, Ace_SPEC>;
    impl Ace {
        #[doc = "Automatic clearing disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Automatic clearing enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adstrgr_SPEC;
impl crate::sealed::RegSpec for Adstrgr_SPEC {
    type DataType = u16;
}

#[doc = "A/D Conversion Start Trigger Select Register"]
pub type Adstrgr = crate::RegValueT<Adstrgr_SPEC>;

impl Adstrgr {
    #[doc = "A/D Conversion Start Trigger SelectSelect the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
    #[inline(always)]
    pub fn trsa(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Adstrgr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Adstrgr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Adstrgr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Adstrgr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "A/D Conversion Start Trigger Select for Group BSelect the A/D conversion start trigger for group B in group scan mode."]
    #[inline(always)]
    pub fn trsb(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Adstrgr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Adstrgr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adstrgr {
    #[inline(always)]
    fn default() -> Adstrgr {
        <crate::RegValueT<Adstrgr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adexicr_SPEC;
impl crate::sealed::RegSpec for Adexicr_SPEC {
    type DataType = u16;
}

#[doc = "A/D Conversion Extended Input Control Register"]
pub type Adexicr = crate::RegValueT<Adexicr_SPEC>;

impl Adexicr {
    #[doc = "Internal Reference Voltage A/D Conversion Select"]
    #[inline(always)]
    pub fn ocsa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adexicr::Ocsa,
        adexicr::Ocsa,
        Adexicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adexicr::Ocsa,
            adexicr::Ocsa,
            Adexicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Temperature Sensor Output A/D Conversion Select"]
    #[inline(always)]
    pub fn tssa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adexicr::Tssa,
        adexicr::Tssa,
        Adexicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adexicr::Tssa,
            adexicr::Tssa,
            Adexicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Adexicr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Adexicr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Internal Reference Voltage A/D converted Value Average Mode Select"]
    #[inline(always)]
    pub fn ocsad(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adexicr::Ocsad,
        adexicr::Ocsad,
        Adexicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adexicr::Ocsad,
            adexicr::Ocsad,
            Adexicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Temperature Sensor Output A/D converted Value Average Mode Select"]
    #[inline(always)]
    pub fn tssad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adexicr::Tssad,
        adexicr::Tssad,
        Adexicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adexicr::Tssad,
            adexicr::Tssad,
            Adexicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adexicr {
    #[inline(always)]
    fn default() -> Adexicr {
        <crate::RegValueT<Adexicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adexicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ocsa_SPEC;
    pub type Ocsa = crate::EnumBitfieldStruct<u8, Ocsa_SPEC>;
    impl Ocsa {
        #[doc = "A/D conversion of internal reference voltage disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "A/D conversion of internal reference voltage enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tssa_SPEC;
    pub type Tssa = crate::EnumBitfieldStruct<u8, Tssa_SPEC>;
    impl Tssa {
        #[doc = "A/D conversion of temperature sensor output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "A/D conversion of temperature sensor output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ocsad_SPEC;
    pub type Ocsad = crate::EnumBitfieldStruct<u8, Ocsad_SPEC>;
    impl Ocsad {
        #[doc = "Internal reference voltage A/D-converted value average mode not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal reference voltage A/D-converted value average mode selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tssad_SPEC;
    pub type Tssad = crate::EnumBitfieldStruct<u8, Tssad_SPEC>;
    impl Tssad {
        #[doc = "Temperature sensor output A/D-converted value average mode not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Temperature sensor output A/D-converted value average mode selected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adansb0_SPEC;
impl crate::sealed::RegSpec for Adansb0_SPEC {
    type DataType = u16;
}

#[doc = "A/D Channel Select Register B0"]
pub type Adansb0 = crate::RegValueT<Adansb0_SPEC>;

impl Adansb0 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Adansb0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Adansb0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AN008 Select"]
    #[inline(always)]
    pub fn ansb08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adansb0::Ansb08,
        adansb0::Ansb08,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adansb0::Ansb08,
            adansb0::Ansb08,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN007 Select"]
    #[inline(always)]
    pub fn ansb07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adansb0::Ansb07,
        adansb0::Ansb07,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adansb0::Ansb07,
            adansb0::Ansb07,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN006 Select"]
    #[inline(always)]
    pub fn ansb06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adansb0::Ansb06,
        adansb0::Ansb06,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adansb0::Ansb06,
            adansb0::Ansb06,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN005 Select"]
    #[inline(always)]
    pub fn ansb05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adansb0::Ansb05,
        adansb0::Ansb05,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adansb0::Ansb05,
            adansb0::Ansb05,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN004 Select"]
    #[inline(always)]
    pub fn ansb04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adansb0::Ansb04,
        adansb0::Ansb04,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adansb0::Ansb04,
            adansb0::Ansb04,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN003 Select"]
    #[inline(always)]
    pub fn ansb03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adansb0::Ansb03,
        adansb0::Ansb03,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adansb0::Ansb03,
            adansb0::Ansb03,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN002 Select"]
    #[inline(always)]
    pub fn ansb02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adansb0::Ansb02,
        adansb0::Ansb02,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adansb0::Ansb02,
            adansb0::Ansb02,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN001 Select"]
    #[inline(always)]
    pub fn ansb01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adansb0::Ansb01,
        adansb0::Ansb01,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adansb0::Ansb01,
            adansb0::Ansb01,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN000 Select"]
    #[inline(always)]
    pub fn ansb00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adansb0::Ansb00,
        adansb0::Ansb00,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adansb0::Ansb00,
            adansb0::Ansb00,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adansb0 {
    #[inline(always)]
    fn default() -> Adansb0 {
        <crate::RegValueT<Adansb0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adansb0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb08_SPEC;
    pub type Ansb08 = crate::EnumBitfieldStruct<u8, Ansb08_SPEC>;
    impl Ansb08 {
        #[doc = "AN008 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN008 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb07_SPEC;
    pub type Ansb07 = crate::EnumBitfieldStruct<u8, Ansb07_SPEC>;
    impl Ansb07 {
        #[doc = "AN007 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN007 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb06_SPEC;
    pub type Ansb06 = crate::EnumBitfieldStruct<u8, Ansb06_SPEC>;
    impl Ansb06 {
        #[doc = "AN006 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN006 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb05_SPEC;
    pub type Ansb05 = crate::EnumBitfieldStruct<u8, Ansb05_SPEC>;
    impl Ansb05 {
        #[doc = "AN005 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN005 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb04_SPEC;
    pub type Ansb04 = crate::EnumBitfieldStruct<u8, Ansb04_SPEC>;
    impl Ansb04 {
        #[doc = "AN004 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN004 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb03_SPEC;
    pub type Ansb03 = crate::EnumBitfieldStruct<u8, Ansb03_SPEC>;
    impl Ansb03 {
        #[doc = "AN003 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN003 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb02_SPEC;
    pub type Ansb02 = crate::EnumBitfieldStruct<u8, Ansb02_SPEC>;
    impl Ansb02 {
        #[doc = "AN002 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN002 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb01_SPEC;
    pub type Ansb01 = crate::EnumBitfieldStruct<u8, Ansb01_SPEC>;
    impl Ansb01 {
        #[doc = "AN001 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN001 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb00_SPEC;
    pub type Ansb00 = crate::EnumBitfieldStruct<u8, Ansb00_SPEC>;
    impl Ansb00 {
        #[doc = "AN000 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN000 is selected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adansb1_SPEC;
impl crate::sealed::RegSpec for Adansb1_SPEC {
    type DataType = u16;
}

#[doc = "A/D Channel Select Register B1"]
pub type Adansb1 = crate::RegValueT<Adansb1_SPEC>;

impl Adansb1 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Adansb1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Adansb1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AN024 Select"]
    #[inline(always)]
    pub fn ansb24(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adansb1::Ansb24,
        adansb1::Ansb24,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adansb1::Ansb24,
            adansb1::Ansb24,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN023 Select"]
    #[inline(always)]
    pub fn ansb23(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adansb1::Ansb23,
        adansb1::Ansb23,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adansb1::Ansb23,
            adansb1::Ansb23,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN022 Select"]
    #[inline(always)]
    pub fn ansb22(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adansb1::Ansb22,
        adansb1::Ansb22,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adansb1::Ansb22,
            adansb1::Ansb22,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN021 Select"]
    #[inline(always)]
    pub fn ansb21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adansb1::Ansb21,
        adansb1::Ansb21,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adansb1::Ansb21,
            adansb1::Ansb21,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN020 Select"]
    #[inline(always)]
    pub fn ansb20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adansb1::Ansb20,
        adansb1::Ansb20,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adansb1::Ansb20,
            adansb1::Ansb20,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN019 Select"]
    #[inline(always)]
    pub fn ansb19(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adansb1::Ansb19,
        adansb1::Ansb19,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adansb1::Ansb19,
            adansb1::Ansb19,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN018 Select"]
    #[inline(always)]
    pub fn ansb18(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adansb1::Ansb18,
        adansb1::Ansb18,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adansb1::Ansb18,
            adansb1::Ansb18,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN017 Select"]
    #[inline(always)]
    pub fn ansb17(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adansb1::Ansb17,
        adansb1::Ansb17,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adansb1::Ansb17,
            adansb1::Ansb17,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN016 Select"]
    #[inline(always)]
    pub fn ansb16(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adansb1::Ansb16,
        adansb1::Ansb16,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adansb1::Ansb16,
            adansb1::Ansb16,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adansb1 {
    #[inline(always)]
    fn default() -> Adansb1 {
        <crate::RegValueT<Adansb1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adansb1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb24_SPEC;
    pub type Ansb24 = crate::EnumBitfieldStruct<u8, Ansb24_SPEC>;
    impl Ansb24 {
        #[doc = "AN024 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN024 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb23_SPEC;
    pub type Ansb23 = crate::EnumBitfieldStruct<u8, Ansb23_SPEC>;
    impl Ansb23 {
        #[doc = "AN023 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN023 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb22_SPEC;
    pub type Ansb22 = crate::EnumBitfieldStruct<u8, Ansb22_SPEC>;
    impl Ansb22 {
        #[doc = "AN022 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN022 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb21_SPEC;
    pub type Ansb21 = crate::EnumBitfieldStruct<u8, Ansb21_SPEC>;
    impl Ansb21 {
        #[doc = "AN021 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN021 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb20_SPEC;
    pub type Ansb20 = crate::EnumBitfieldStruct<u8, Ansb20_SPEC>;
    impl Ansb20 {
        #[doc = "AN020 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN020 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb19_SPEC;
    pub type Ansb19 = crate::EnumBitfieldStruct<u8, Ansb19_SPEC>;
    impl Ansb19 {
        #[doc = "AN019 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN019 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb18_SPEC;
    pub type Ansb18 = crate::EnumBitfieldStruct<u8, Ansb18_SPEC>;
    impl Ansb18 {
        #[doc = "AN018 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN018 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb17_SPEC;
    pub type Ansb17 = crate::EnumBitfieldStruct<u8, Ansb17_SPEC>;
    impl Ansb17 {
        #[doc = "AN017 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN017 is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb16_SPEC;
    pub type Ansb16 = crate::EnumBitfieldStruct<u8, Ansb16_SPEC>;
    impl Ansb16 {
        #[doc = "AN016 is not selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "AN016 is selected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addbldr_SPEC;
impl crate::sealed::RegSpec for Addbldr_SPEC {
    type DataType = u16;
}

#[doc = "A/D Data Duplexing Register"]
pub type Addbldr = crate::RegValueT<Addbldr_SPEC>;

impl Addbldr {
    #[doc = "This is a 16-bit read-only register for storing the result of A/D conversion in response to the second trigger in double trigger mode."]
    #[inline(always)]
    pub fn addbldr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Addbldr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Addbldr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Addbldr {
    #[inline(always)]
    fn default() -> Addbldr {
        <crate::RegValueT<Addbldr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adtsdr_SPEC;
impl crate::sealed::RegSpec for Adtsdr_SPEC {
    type DataType = u16;
}

#[doc = "A/D Temperature Sensor Data Register"]
pub type Adtsdr = crate::RegValueT<Adtsdr_SPEC>;

impl Adtsdr {
    #[doc = "This is a 16-bit read-only register for storing the A/D conversion result of temperature sensor output."]
    #[inline(always)]
    pub fn adtsdr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adtsdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adtsdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Adtsdr {
    #[inline(always)]
    fn default() -> Adtsdr {
        <crate::RegValueT<Adtsdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adocdr_SPEC;
impl crate::sealed::RegSpec for Adocdr_SPEC {
    type DataType = u16;
}

#[doc = "A/D Internal Reference Voltage Data Register"]
pub type Adocdr = crate::RegValueT<Adocdr_SPEC>;

impl Adocdr {
    #[doc = "This is a 16-bit read-only register for storing the A/D result of internal reference voltage."]
    #[inline(always)]
    pub fn adocdr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adocdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adocdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Adocdr {
    #[inline(always)]
    fn default() -> Adocdr {
        <crate::RegValueT<Adocdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addr_SPEC;
impl crate::sealed::RegSpec for Addr_SPEC {
    type DataType = u16;
}

#[doc = "A/D Data Register %s"]
pub type Addr = crate::RegValueT<Addr_SPEC>;

impl Addr {
    #[doc = "The ADDR register is a 16-bit read-only registers for storing the result of A/D conversion."]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Addr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Addr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Addr {
    #[inline(always)]
    fn default() -> Addr {
        <crate::RegValueT<Addr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addiscr_SPEC;
impl crate::sealed::RegSpec for Addiscr_SPEC {
    type DataType = u8;
}

#[doc = "A/D Disconnection Detection Control Register"]
pub type Addiscr = crate::RegValueT<Addiscr_SPEC>;

impl Addiscr {
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Addiscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Addiscr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Precharge/discharge select for even analog input channels from AN000 to AN008 or analog input channels from AN016 to AN023"]
    #[inline(always)]
    pub fn pchg1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        addiscr::Pchg1,
        addiscr::Pchg1,
        Addiscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            addiscr::Pchg1,
            addiscr::Pchg1,
            Addiscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Precharge/discharge select for odd analog input channels from AN000 to AN007"]
    #[inline(always)]
    pub fn pchg2(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        addiscr::Pchg2,
        addiscr::Pchg2,
        Addiscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            addiscr::Pchg2,
            addiscr::Pchg2,
            Addiscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Precharg/discharge period"]
    #[inline(always)]
    pub fn adndis(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        addiscr::Adndis,
        addiscr::Adndis,
        Addiscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            addiscr::Adndis,
            addiscr::Adndis,
            Addiscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Addiscr {
    #[inline(always)]
    fn default() -> Addiscr {
        <crate::RegValueT<Addiscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod addiscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pchg1_SPEC;
    pub type Pchg1 = crate::EnumBitfieldStruct<u8, Pchg1_SPEC>;
    impl Pchg1 {
        #[doc = "The analog input path 1 voltage is discharged"]
        pub const _0: Self = Self::new(0);

        #[doc = "The analog input path 1 voltage is precharged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pchg2_SPEC;
    pub type Pchg2 = crate::EnumBitfieldStruct<u8, Pchg2_SPEC>;
    impl Pchg2 {
        #[doc = "The analog input path 2 voltage is discharged"]
        pub const _0: Self = Self::new(0);

        #[doc = "The analog input path 2 voltage is precharged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adndis_SPEC;
    pub type Adndis = crate::EnumBitfieldStruct<u8, Adndis_SPEC>;
    impl Adndis {
        #[doc = "The disconnection detection assist function is disabled"]
        pub const _0000: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _0001: Self = Self::new(1);

        #[doc = "( 1 / ADCLK ) x ADNDIS"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adicr_SPEC;
impl crate::sealed::RegSpec for Adicr_SPEC {
    type DataType = u8;
}

#[doc = "A/D Interrupt Control Register"]
pub type Adicr = crate::RegValueT<Adicr_SPEC>;

impl Adicr {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Adicr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Adicr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "A/D Interrupt Control"]
    #[inline(always)]
    pub fn adic(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adicr::Adic,
        adicr::Adic,
        Adicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adicr::Adic,
            adicr::Adic,
            Adicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adicr {
    #[inline(always)]
    fn default() -> Adicr {
        <crate::RegValueT<Adicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adic_SPEC;
    pub type Adic = crate::EnumBitfieldStruct<u8, Adic_SPEC>;
    impl Adic {
        #[doc = "ADC160_ADI is generated at the end of A/D scan"]
        pub const _00: Self = Self::new(0);

        #[doc = "ADC160_ADI is generated at the end of calibration"]
        pub const _11: Self = Self::new(3);

        #[doc = "Settings are prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adgspcr_SPEC;
impl crate::sealed::RegSpec for Adgspcr_SPEC {
    type DataType = u16;
}

#[doc = "A/D Group Scan Priority Control Register"]
pub type Adgspcr = crate::RegValueT<Adgspcr_SPEC>;

impl Adgspcr {
    #[doc = "Group B Single Scan Continuous Start(Enabled only when PGS = 1. Reserved when PGS = 0.)Note: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit."]
    #[inline(always)]
    pub fn gbrp(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adgspcr::Gbrp,
        adgspcr::Gbrp,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adgspcr::Gbrp,
            adgspcr::Gbrp,
            Adgspcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000. The write value should be 0000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x1fff, 1, 0, u16, u16, Adgspcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1fff,1,0,u16,u16,Adgspcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Group B Restart Setting(Enabled only when PGS = 1. Reserved when PGS = 0.)"]
    #[inline(always)]
    pub fn gbrscn(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adgspcr::Gbrscn,
        adgspcr::Gbrscn,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adgspcr::Gbrscn,
            adgspcr::Gbrscn,
            Adgspcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Group A priority control setting bit.Note: When the PGS bit is to be set to 1, the ADCSR.ADCS\\[1:0\\] bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed."]
    #[inline(always)]
    pub fn pgs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adgspcr::Pgs,
        adgspcr::Pgs,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adgspcr::Pgs,
            adgspcr::Pgs,
            Adgspcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adgspcr {
    #[inline(always)]
    fn default() -> Adgspcr {
        <crate::RegValueT<Adgspcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adgspcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gbrp_SPEC;
    pub type Gbrp = crate::EnumBitfieldStruct<u8, Gbrp_SPEC>;
    impl Gbrp {
        #[doc = "Single scan for group B is not continuously activated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Single scan for group B is continuously activated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gbrscn_SPEC;
    pub type Gbrscn = crate::EnumBitfieldStruct<u8, Gbrscn_SPEC>;
    impl Gbrscn {
        #[doc = "Scanning for group B is not restarted after having been discontinued due to group A priority control."]
        pub const _0: Self = Self::new(0);

        #[doc = "Scanning for group B is restarted after having been discontinued due to group A priority control."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgs_SPEC;
    pub type Pgs = crate::EnumBitfieldStruct<u8, Pgs_SPEC>;
    impl Pgs {
        #[doc = "Operation is without group A priority control"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operation is with group A priority control"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addbldra_SPEC;
impl crate::sealed::RegSpec for Addbldra_SPEC {
    type DataType = u16;
}

#[doc = "A/D Data Duplexing Register A"]
pub type Addbldra = crate::RegValueT<Addbldra_SPEC>;

impl Addbldra {
    #[doc = "This register is a 16-bit read-only registers for storing the result of A/D conversion in response to the respective triggers during extended operation in double trigger mode."]
    #[inline(always)]
    pub fn addbldra(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Addbldra_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Addbldra_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Addbldra {
    #[inline(always)]
    fn default() -> Addbldra {
        <crate::RegValueT<Addbldra_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addbldrb_SPEC;
impl crate::sealed::RegSpec for Addbldrb_SPEC {
    type DataType = u16;
}

#[doc = "A/D Data Duplexing Register B"]
pub type Addbldrb = crate::RegValueT<Addbldrb_SPEC>;

impl Addbldrb {
    #[doc = "This register is a 16-bit read-only registers for storing the result of A/D conversion in response to the respective triggers during extended operation in double trigger mode."]
    #[inline(always)]
    pub fn addbldrb(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Addbldrb_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Addbldrb_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Addbldrb {
    #[inline(always)]
    fn default() -> Addbldrb {
        <crate::RegValueT<Addbldrb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adwinmon_SPEC;
impl crate::sealed::RegSpec for Adwinmon_SPEC {
    type DataType = u8;
}

#[doc = "A/D Compare Function Window A/B Status Monitor Register"]
pub type Adwinmon = crate::RegValueT<Adwinmon_SPEC>;

impl Adwinmon {
    #[doc = "Comparison Result Monitor B"]
    #[inline(always)]
    pub fn moncmpb(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adwinmon::Moncmpb,
        adwinmon::Moncmpb,
        Adwinmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adwinmon::Moncmpb,
            adwinmon::Moncmpb,
            Adwinmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Comparison Result Monitor A"]
    #[inline(always)]
    pub fn moncmpa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adwinmon::Moncmpa,
        adwinmon::Moncmpa,
        Adwinmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adwinmon::Moncmpa,
            adwinmon::Moncmpa,
            Adwinmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, u8, Adwinmon_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x7,1,0,u8,u8,Adwinmon_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Combination result monitorThis bit indicates the combination result. This bit is valid when both window A operation and window B operation are enabled."]
    #[inline(always)]
    pub fn moncomb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adwinmon::Moncomb,
        adwinmon::Moncomb,
        Adwinmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adwinmon::Moncomb,
            adwinmon::Moncomb,
            Adwinmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adwinmon {
    #[inline(always)]
    fn default() -> Adwinmon {
        <crate::RegValueT<Adwinmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adwinmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moncmpb_SPEC;
    pub type Moncmpb = crate::EnumBitfieldStruct<u8, Moncmpb_SPEC>;
    impl Moncmpb {
        #[doc = "Window B comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Window B comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moncmpa_SPEC;
    pub type Moncmpa = crate::EnumBitfieldStruct<u8, Moncmpa_SPEC>;
    impl Moncmpa {
        #[doc = "Window A comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Window A comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moncomb_SPEC;
    pub type Moncomb = crate::EnumBitfieldStruct<u8, Moncomb_SPEC>;
    impl Moncomb {
        #[doc = "Window A / window B composite conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Window A / window B composite conditions are met."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpcr_SPEC;
impl crate::sealed::RegSpec for Adcmpcr_SPEC {
    type DataType = u16;
}

#[doc = "A/D Compare Function Control Register"]
pub type Adcmpcr = crate::RegValueT<Adcmpcr_SPEC>;

impl Adcmpcr {
    #[doc = "Compare A Interrupt Enable"]
    #[inline(always)]
    pub fn cmpaie(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adcmpcr::Cmpaie,
        adcmpcr::Cmpaie,
        Adcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adcmpcr::Cmpaie,
            adcmpcr::Cmpaie,
            Adcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Window Function Setting"]
    #[inline(always)]
    pub fn wcmpe(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adcmpcr::Wcmpe,
        adcmpcr::Wcmpe,
        Adcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adcmpcr::Wcmpe,
            adcmpcr::Wcmpe,
            Adcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare B Interrupt Enable"]
    #[inline(always)]
    pub fn cmpbie(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adcmpcr::Cmpbie,
        adcmpcr::Cmpbie,
        Adcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adcmpcr::Cmpbie,
            adcmpcr::Cmpbie,
            Adcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Operation Enable"]
    #[inline(always)]
    pub fn cmpae(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adcmpcr::Cmpae,
        adcmpcr::Cmpae,
        Adcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adcmpcr::Cmpae,
            adcmpcr::Cmpae,
            Adcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window B Operation Enable"]
    #[inline(always)]
    pub fn cmpbe(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcmpcr::Cmpbe,
        adcmpcr::Cmpbe,
        Adcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcmpcr::Cmpbe,
            adcmpcr::Cmpbe,
            Adcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x7f, 1, 0, u8, u8, Adcmpcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x7f,1,0,u8,u8,Adcmpcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Window A/B Composite Conditions SettingNOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1)."]
    #[inline(always)]
    pub fn cmpab(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adcmpcr::Cmpab,
        adcmpcr::Cmpab,
        Adcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adcmpcr::Cmpab,
            adcmpcr::Cmpab,
            Adcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpcr {
    #[inline(always)]
    fn default() -> Adcmpcr {
        <crate::RegValueT<Adcmpcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpaie_SPEC;
    pub type Cmpaie = crate::EnumBitfieldStruct<u8, Cmpaie_SPEC>;
    impl Cmpaie {
        #[doc = "ADC160_CMPAI interrupt is disabled when comparison conditions (window A) are met."]
        pub const _0: Self = Self::new(0);

        #[doc = "ADC160_CMPAI interrupt is enabled when comparison conditions (window A) are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wcmpe_SPEC;
    pub type Wcmpe = crate::EnumBitfieldStruct<u8, Wcmpe_SPEC>;
    impl Wcmpe {
        #[doc = "Window function is disabled. Window A and window B operate as a comparator to comparator the single value on the lower side with the A/D conversion result."]
        pub const _0: Self = Self::new(0);

        #[doc = "Window function is enabled. Window A and window B operate as a comparator to comparator the two values on the upper and lower sides with the A/D conversion result."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpbie_SPEC;
    pub type Cmpbie = crate::EnumBitfieldStruct<u8, Cmpbie_SPEC>;
    impl Cmpbie {
        #[doc = "ADC160_CMPBI interrupt is disabled when comparison conditions (window B) are met."]
        pub const _0: Self = Self::new(0);

        #[doc = "ADC160_CMPBI interrupt is enabled when comparison conditions (window B) are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpae_SPEC;
    pub type Cmpae = crate::EnumBitfieldStruct<u8, Cmpae_SPEC>;
    impl Cmpae {
        #[doc = "Compare window A operation is disabled. ADC160_WCMPM and ADC160_WCMPUM outputs are disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare window A operation is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpbe_SPEC;
    pub type Cmpbe = crate::EnumBitfieldStruct<u8, Cmpbe_SPEC>;
    impl Cmpbe {
        #[doc = "Compare window B operation is disabled. ADC160_WCMPM and ADC160_WCMPUM outputs are disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare window B operation is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpab_SPEC;
    pub type Cmpab = crate::EnumBitfieldStruct<u8, Cmpab_SPEC>;
    impl Cmpab {
        #[doc = "ADC160_WCMPM is output when window A comparison conditions are met OR window B comparison conditions are met. ADC160_WCMPUM is output in other cases."]
        pub const _00: Self = Self::new(0);

        #[doc = "ADC160_WCMPM is output when window A comparison conditions are met EXOR window B comparison conditions are met. ADC160_WCMPUM is output in other cases."]
        pub const _01: Self = Self::new(1);

        #[doc = "ADC160_WCMPM is output when window A comparison conditions are met and window B comparison conditions are met. ADC160_WCMPUM is output in other cases."]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpanser_SPEC;
impl crate::sealed::RegSpec for Adcmpanser_SPEC {
    type DataType = u8;
}

#[doc = "A/D Compare Function Window A Extended Input Select Register"]
pub type Adcmpanser = crate::RegValueT<Adcmpanser_SPEC>;

impl Adcmpanser {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Adcmpanser_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Adcmpanser_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Internal reference voltage Compare selection bit."]
    #[inline(always)]
    pub fn cmpoca(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpanser::Cmpoca,
        adcmpanser::Cmpoca,
        Adcmpanser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpanser::Cmpoca,
            adcmpanser::Cmpoca,
            Adcmpanser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Temperature sensor output Compare selection bit."]
    #[inline(always)]
    pub fn cmptsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpanser::Cmptsa,
        adcmpanser::Cmptsa,
        Adcmpanser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpanser::Cmptsa,
            adcmpanser::Cmptsa,
            Adcmpanser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpanser {
    #[inline(always)]
    fn default() -> Adcmpanser {
        <crate::RegValueT<Adcmpanser_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpanser {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpoca_SPEC;
    pub type Cmpoca = crate::EnumBitfieldStruct<u8, Cmpoca_SPEC>;
    impl Cmpoca {
        #[doc = "Excludes the internal reference voltage from the compare window A target range."]
        pub const _0: Self = Self::new(0);

        #[doc = "Includes the internal reference voltage in the compare window A target range."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptsa_SPEC;
    pub type Cmptsa = crate::EnumBitfieldStruct<u8, Cmptsa_SPEC>;
    impl Cmptsa {
        #[doc = "Excludes the temperature sensor output from the compare window A target range."]
        pub const _0: Self = Self::new(0);

        #[doc = "Includes the temperature sensor output in the compare window A target range."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpler_SPEC;
impl crate::sealed::RegSpec for Adcmpler_SPEC {
    type DataType = u8;
}

#[doc = "A/D Compare Function Window A Extended Input Comparison Condition Setting Register"]
pub type Adcmpler = crate::RegValueT<Adcmpler_SPEC>;

impl Adcmpler {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Adcmpler_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Adcmpler_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Compare Window A Internal Reference Voltage ComparisonCondition Select"]
    #[inline(always)]
    pub fn cmploca(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpler::Cmploca,
        adcmpler::Cmploca,
        Adcmpler_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpler::Cmploca,
            adcmpler::Cmploca,
            Adcmpler_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Temperature Sensor Output Comparison Condition Select"]
    #[inline(always)]
    pub fn cmpltsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpler::Cmpltsa,
        adcmpler::Cmpltsa,
        Adcmpler_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpler::Cmpltsa,
            adcmpler::Cmpltsa,
            Adcmpler_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpler {
    #[inline(always)]
    fn default() -> Adcmpler {
        <crate::RegValueT<Adcmpler_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpler {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmploca_SPEC;
    pub type Cmploca = crate::EnumBitfieldStruct<u8, Cmploca_SPEC>;
    impl Cmploca {
        #[doc = "ADCMPDR0 value > A/D converted value(ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or A/D converted value > ADCMPDR1 value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 value < A/D converted value < ADCMPDR1 value(ADCMPCR.WCMPE=1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpltsa_SPEC;
    pub type Cmpltsa = crate::EnumBitfieldStruct<u8, Cmpltsa_SPEC>;
    impl Cmpltsa {
        #[doc = "ADCMPDR0 register value > A/D-converted value(ADCMPCR.WCMPE=0) / AD-converted value < ADCMPDR0 register value or A/D-converted value > ADCMPDR1 register value(ADCMPCR.WCMPE=1)."]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 register value < A/D-converted value(ADCMPCR.WCMPE=0) / ADCMPDR0 register value < A/D-converted value < ADCMPDR1 register value(ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpansr0_SPEC;
impl crate::sealed::RegSpec for Adcmpansr0_SPEC {
    type DataType = u16;
}

#[doc = "A/D Compare Function Window A Channel Select Register 0"]
pub type Adcmpansr0 = crate::RegValueT<Adcmpansr0_SPEC>;

impl Adcmpansr0 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Adcmpansr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Adcmpansr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AN008 Select"]
    #[inline(always)]
    pub fn cmpcha08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha08,
        adcmpansr0::Cmpcha08,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha08,
            adcmpansr0::Cmpcha08,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN007 Select"]
    #[inline(always)]
    pub fn cmpcha07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha07,
        adcmpansr0::Cmpcha07,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha07,
            adcmpansr0::Cmpcha07,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN006 Select"]
    #[inline(always)]
    pub fn cmpcha06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha06,
        adcmpansr0::Cmpcha06,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha06,
            adcmpansr0::Cmpcha06,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN005 Select"]
    #[inline(always)]
    pub fn cmpcha05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha05,
        adcmpansr0::Cmpcha05,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha05,
            adcmpansr0::Cmpcha05,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN004 Select"]
    #[inline(always)]
    pub fn cmpcha04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha04,
        adcmpansr0::Cmpcha04,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha04,
            adcmpansr0::Cmpcha04,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN003 Select"]
    #[inline(always)]
    pub fn cmpcha03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha03,
        adcmpansr0::Cmpcha03,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha03,
            adcmpansr0::Cmpcha03,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN002 Select"]
    #[inline(always)]
    pub fn cmpcha02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha02,
        adcmpansr0::Cmpcha02,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha02,
            adcmpansr0::Cmpcha02,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN001 Select"]
    #[inline(always)]
    pub fn cmpcha01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha01,
        adcmpansr0::Cmpcha01,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha01,
            adcmpansr0::Cmpcha01,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN000 Select"]
    #[inline(always)]
    pub fn cmpcha00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha00,
        adcmpansr0::Cmpcha00,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha00,
            adcmpansr0::Cmpcha00,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpansr0 {
    #[inline(always)]
    fn default() -> Adcmpansr0 {
        <crate::RegValueT<Adcmpansr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpansr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha08_SPEC;
    pub type Cmpcha08 = crate::EnumBitfieldStruct<u8, Cmpcha08_SPEC>;
    impl Cmpcha08 {
        #[doc = "Compare function disabled for AN008"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN008"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha07_SPEC;
    pub type Cmpcha07 = crate::EnumBitfieldStruct<u8, Cmpcha07_SPEC>;
    impl Cmpcha07 {
        #[doc = "Compare function disabled for AN007"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN007"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha06_SPEC;
    pub type Cmpcha06 = crate::EnumBitfieldStruct<u8, Cmpcha06_SPEC>;
    impl Cmpcha06 {
        #[doc = "Compare function disabled for AN006"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN006"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha05_SPEC;
    pub type Cmpcha05 = crate::EnumBitfieldStruct<u8, Cmpcha05_SPEC>;
    impl Cmpcha05 {
        #[doc = "Compare function disabled for AN005"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN005"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha04_SPEC;
    pub type Cmpcha04 = crate::EnumBitfieldStruct<u8, Cmpcha04_SPEC>;
    impl Cmpcha04 {
        #[doc = "Compare function disabled for AN004"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN004"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha03_SPEC;
    pub type Cmpcha03 = crate::EnumBitfieldStruct<u8, Cmpcha03_SPEC>;
    impl Cmpcha03 {
        #[doc = "Compare function disabled for AN003"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN003"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha02_SPEC;
    pub type Cmpcha02 = crate::EnumBitfieldStruct<u8, Cmpcha02_SPEC>;
    impl Cmpcha02 {
        #[doc = "Compare function disabled for AN002"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN002"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha01_SPEC;
    pub type Cmpcha01 = crate::EnumBitfieldStruct<u8, Cmpcha01_SPEC>;
    impl Cmpcha01 {
        #[doc = "Compare function disabled for AN001"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN001"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha00_SPEC;
    pub type Cmpcha00 = crate::EnumBitfieldStruct<u8, Cmpcha00_SPEC>;
    impl Cmpcha00 {
        #[doc = "Compare function disabled for AN000"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN000"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpansr1_SPEC;
impl crate::sealed::RegSpec for Adcmpansr1_SPEC {
    type DataType = u16;
}

#[doc = "A/D Compare Function Window A Channel Select Register 1"]
pub type Adcmpansr1 = crate::RegValueT<Adcmpansr1_SPEC>;

impl Adcmpansr1 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Adcmpansr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Adcmpansr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AN024 Select"]
    #[inline(always)]
    pub fn cmpcha24(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha24,
        adcmpansr1::Cmpcha24,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha24,
            adcmpansr1::Cmpcha24,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN023 Select"]
    #[inline(always)]
    pub fn cmpcha23(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha23,
        adcmpansr1::Cmpcha23,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha23,
            adcmpansr1::Cmpcha23,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN022 Select"]
    #[inline(always)]
    pub fn cmpcha22(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha22,
        adcmpansr1::Cmpcha22,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha22,
            adcmpansr1::Cmpcha22,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN021 Select"]
    #[inline(always)]
    pub fn cmpcha21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha21,
        adcmpansr1::Cmpcha21,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha21,
            adcmpansr1::Cmpcha21,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN020 Select"]
    #[inline(always)]
    pub fn cmpcha20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha20,
        adcmpansr1::Cmpcha20,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha20,
            adcmpansr1::Cmpcha20,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN019 Select"]
    #[inline(always)]
    pub fn cmpcha19(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha19,
        adcmpansr1::Cmpcha19,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha19,
            adcmpansr1::Cmpcha19,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN018 Select"]
    #[inline(always)]
    pub fn cmpcha18(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha18,
        adcmpansr1::Cmpcha18,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha18,
            adcmpansr1::Cmpcha18,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN017 Select"]
    #[inline(always)]
    pub fn cmpcha17(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha17,
        adcmpansr1::Cmpcha17,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha17,
            adcmpansr1::Cmpcha17,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN016 Select"]
    #[inline(always)]
    pub fn cmpcha16(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha16,
        adcmpansr1::Cmpcha16,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha16,
            adcmpansr1::Cmpcha16,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpansr1 {
    #[inline(always)]
    fn default() -> Adcmpansr1 {
        <crate::RegValueT<Adcmpansr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpansr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha24_SPEC;
    pub type Cmpcha24 = crate::EnumBitfieldStruct<u8, Cmpcha24_SPEC>;
    impl Cmpcha24 {
        #[doc = "Compare function disabled for AN024"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN024"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha23_SPEC;
    pub type Cmpcha23 = crate::EnumBitfieldStruct<u8, Cmpcha23_SPEC>;
    impl Cmpcha23 {
        #[doc = "Compare function disabled for AN023"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN023"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha22_SPEC;
    pub type Cmpcha22 = crate::EnumBitfieldStruct<u8, Cmpcha22_SPEC>;
    impl Cmpcha22 {
        #[doc = "Compare function disabled for AN022"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN022"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha21_SPEC;
    pub type Cmpcha21 = crate::EnumBitfieldStruct<u8, Cmpcha21_SPEC>;
    impl Cmpcha21 {
        #[doc = "Compare function disabled for AN021"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN021"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha20_SPEC;
    pub type Cmpcha20 = crate::EnumBitfieldStruct<u8, Cmpcha20_SPEC>;
    impl Cmpcha20 {
        #[doc = "Compare function disabled for AN020"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN020"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha19_SPEC;
    pub type Cmpcha19 = crate::EnumBitfieldStruct<u8, Cmpcha19_SPEC>;
    impl Cmpcha19 {
        #[doc = "Compare function disabled for AN019"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN019"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha18_SPEC;
    pub type Cmpcha18 = crate::EnumBitfieldStruct<u8, Cmpcha18_SPEC>;
    impl Cmpcha18 {
        #[doc = "Compare function disabled for AN018"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN018"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha17_SPEC;
    pub type Cmpcha17 = crate::EnumBitfieldStruct<u8, Cmpcha17_SPEC>;
    impl Cmpcha17 {
        #[doc = "Compare function disabled for AN017"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN017"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha16_SPEC;
    pub type Cmpcha16 = crate::EnumBitfieldStruct<u8, Cmpcha16_SPEC>;
    impl Cmpcha16 {
        #[doc = "Compare function disabled for AN016"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare function enabled for AN016"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmplr0_SPEC;
impl crate::sealed::RegSpec for Adcmplr0_SPEC {
    type DataType = u16;
}

#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 0"]
pub type Adcmplr0 = crate::RegValueT<Adcmplr0_SPEC>;

impl Adcmplr0 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Adcmplr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Adcmplr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Comparison condition of AN008"]
    #[inline(always)]
    pub fn cmplcha08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha08,
        adcmplr0::Cmplcha08,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha08,
            adcmplr0::Cmplcha08,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition of AN007"]
    #[inline(always)]
    pub fn cmplcha07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha07,
        adcmplr0::Cmplcha07,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha07,
            adcmplr0::Cmplcha07,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition of AN006"]
    #[inline(always)]
    pub fn cmplcha06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha06,
        adcmplr0::Cmplcha06,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha06,
            adcmplr0::Cmplcha06,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition of AN005"]
    #[inline(always)]
    pub fn cmplcha05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha05,
        adcmplr0::Cmplcha05,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha05,
            adcmplr0::Cmplcha05,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition of AN004"]
    #[inline(always)]
    pub fn cmplcha04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha04,
        adcmplr0::Cmplcha04,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha04,
            adcmplr0::Cmplcha04,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition of AN003"]
    #[inline(always)]
    pub fn cmplcha03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha03,
        adcmplr0::Cmplcha03,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha03,
            adcmplr0::Cmplcha03,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition of AN002"]
    #[inline(always)]
    pub fn cmplcha02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha02,
        adcmplr0::Cmplcha02,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha02,
            adcmplr0::Cmplcha02,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition of AN001"]
    #[inline(always)]
    pub fn cmplcha01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha01,
        adcmplr0::Cmplcha01,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha01,
            adcmplr0::Cmplcha01,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition of AN000"]
    #[inline(always)]
    pub fn cmplcha00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha00,
        adcmplr0::Cmplcha00,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha00,
            adcmplr0::Cmplcha00,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmplr0 {
    #[inline(always)]
    fn default() -> Adcmplr0 {
        <crate::RegValueT<Adcmplr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmplr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha08_SPEC;
    pub type Cmplcha08 = crate::EnumBitfieldStruct<u8, Cmplcha08_SPEC>;
    impl Cmplcha08 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha07_SPEC;
    pub type Cmplcha07 = crate::EnumBitfieldStruct<u8, Cmplcha07_SPEC>;
    impl Cmplcha07 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha06_SPEC;
    pub type Cmplcha06 = crate::EnumBitfieldStruct<u8, Cmplcha06_SPEC>;
    impl Cmplcha06 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha05_SPEC;
    pub type Cmplcha05 = crate::EnumBitfieldStruct<u8, Cmplcha05_SPEC>;
    impl Cmplcha05 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha04_SPEC;
    pub type Cmplcha04 = crate::EnumBitfieldStruct<u8, Cmplcha04_SPEC>;
    impl Cmplcha04 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha03_SPEC;
    pub type Cmplcha03 = crate::EnumBitfieldStruct<u8, Cmplcha03_SPEC>;
    impl Cmplcha03 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha02_SPEC;
    pub type Cmplcha02 = crate::EnumBitfieldStruct<u8, Cmplcha02_SPEC>;
    impl Cmplcha02 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha01_SPEC;
    pub type Cmplcha01 = crate::EnumBitfieldStruct<u8, Cmplcha01_SPEC>;
    impl Cmplcha01 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha00_SPEC;
    pub type Cmplcha00 = crate::EnumBitfieldStruct<u8, Cmplcha00_SPEC>;
    impl Cmplcha00 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmplr1_SPEC;
impl crate::sealed::RegSpec for Adcmplr1_SPEC {
    type DataType = u16;
}

#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 1"]
pub type Adcmplr1 = crate::RegValueT<Adcmplr1_SPEC>;

impl Adcmplr1 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Adcmplr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Adcmplr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Comparison condition for AN024"]
    #[inline(always)]
    pub fn cmplcha24(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha24,
        adcmplr1::Cmplcha24,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha24,
            adcmplr1::Cmplcha24,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition for AN023"]
    #[inline(always)]
    pub fn cmplcha23(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha23,
        adcmplr1::Cmplcha23,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha23,
            adcmplr1::Cmplcha23,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition for AN022"]
    #[inline(always)]
    pub fn cmplcha22(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha22,
        adcmplr1::Cmplcha22,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha22,
            adcmplr1::Cmplcha22,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition for AN021"]
    #[inline(always)]
    pub fn cmplcha21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha21,
        adcmplr1::Cmplcha21,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha21,
            adcmplr1::Cmplcha21,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition for AN020"]
    #[inline(always)]
    pub fn cmplcha20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha20,
        adcmplr1::Cmplcha20,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha20,
            adcmplr1::Cmplcha20,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition for AN019"]
    #[inline(always)]
    pub fn cmplcha19(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha19,
        adcmplr1::Cmplcha19,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha19,
            adcmplr1::Cmplcha19,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition for AN018"]
    #[inline(always)]
    pub fn cmplcha18(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha18,
        adcmplr1::Cmplcha18,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha18,
            adcmplr1::Cmplcha18,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition for AN017"]
    #[inline(always)]
    pub fn cmplcha17(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha17,
        adcmplr1::Cmplcha17,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha17,
            adcmplr1::Cmplcha17,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Comparison condition for AN016"]
    #[inline(always)]
    pub fn cmplcha16(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha16,
        adcmplr1::Cmplcha16,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha16,
            adcmplr1::Cmplcha16,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmplr1 {
    #[inline(always)]
    fn default() -> Adcmplr1 {
        <crate::RegValueT<Adcmplr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmplr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha24_SPEC;
    pub type Cmplcha24 = crate::EnumBitfieldStruct<u8, Cmplcha24_SPEC>;
    impl Cmplcha24 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha23_SPEC;
    pub type Cmplcha23 = crate::EnumBitfieldStruct<u8, Cmplcha23_SPEC>;
    impl Cmplcha23 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha22_SPEC;
    pub type Cmplcha22 = crate::EnumBitfieldStruct<u8, Cmplcha22_SPEC>;
    impl Cmplcha22 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha21_SPEC;
    pub type Cmplcha21 = crate::EnumBitfieldStruct<u8, Cmplcha21_SPEC>;
    impl Cmplcha21 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha20_SPEC;
    pub type Cmplcha20 = crate::EnumBitfieldStruct<u8, Cmplcha20_SPEC>;
    impl Cmplcha20 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha19_SPEC;
    pub type Cmplcha19 = crate::EnumBitfieldStruct<u8, Cmplcha19_SPEC>;
    impl Cmplcha19 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha18_SPEC;
    pub type Cmplcha18 = crate::EnumBitfieldStruct<u8, Cmplcha18_SPEC>;
    impl Cmplcha18 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha17_SPEC;
    pub type Cmplcha17 = crate::EnumBitfieldStruct<u8, Cmplcha17_SPEC>;
    impl Cmplcha17 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha16_SPEC;
    pub type Cmplcha16 = crate::EnumBitfieldStruct<u8, Cmplcha16_SPEC>;
    impl Cmplcha16 {
        #[doc = "ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1)."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpdr0_SPEC;
impl crate::sealed::RegSpec for Adcmpdr0_SPEC {
    type DataType = u16;
}

#[doc = "A/D Compare Function Window A Lower-Side Level Setting Register"]
pub type Adcmpdr0 = crate::RegValueT<Adcmpdr0_SPEC>;

impl Adcmpdr0 {
    #[doc = "The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A."]
    #[inline(always)]
    pub fn adcmpdr0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adcmpdr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adcmpdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adcmpdr0 {
    #[inline(always)]
    fn default() -> Adcmpdr0 {
        <crate::RegValueT<Adcmpdr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpdr1_SPEC;
impl crate::sealed::RegSpec for Adcmpdr1_SPEC {
    type DataType = u16;
}

#[doc = "A/D Compare Function Window A Upper-Side Level Setting Register"]
pub type Adcmpdr1 = crate::RegValueT<Adcmpdr1_SPEC>;

impl Adcmpdr1 {
    #[doc = "The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A."]
    #[inline(always)]
    pub fn adcmpdr1(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adcmpdr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adcmpdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adcmpdr1 {
    #[inline(always)]
    fn default() -> Adcmpdr1 {
        <crate::RegValueT<Adcmpdr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpsr0_SPEC;
impl crate::sealed::RegSpec for Adcmpsr0_SPEC {
    type DataType = u16;
}

#[doc = "A/D Compare Function Window A Channel Status Register 0"]
pub type Adcmpsr0 = crate::RegValueT<Adcmpsr0_SPEC>;

impl Adcmpsr0 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Adcmpsr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Adcmpsr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Compare window A flag for AN008"]
    #[inline(always)]
    pub fn cmpstcha08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha08,
        adcmpsr0::Cmpstcha08,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha08,
            adcmpsr0::Cmpstcha08,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN007"]
    #[inline(always)]
    pub fn cmpstcha07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha07,
        adcmpsr0::Cmpstcha07,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha07,
            adcmpsr0::Cmpstcha07,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN006"]
    #[inline(always)]
    pub fn cmpstcha06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha06,
        adcmpsr0::Cmpstcha06,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha06,
            adcmpsr0::Cmpstcha06,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN005"]
    #[inline(always)]
    pub fn cmpstcha05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha05,
        adcmpsr0::Cmpstcha05,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha05,
            adcmpsr0::Cmpstcha05,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN004"]
    #[inline(always)]
    pub fn cmpstcha04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha04,
        adcmpsr0::Cmpstcha04,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha04,
            adcmpsr0::Cmpstcha04,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN003"]
    #[inline(always)]
    pub fn cmpstcha03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha03,
        adcmpsr0::Cmpstcha03,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha03,
            adcmpsr0::Cmpstcha03,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN002"]
    #[inline(always)]
    pub fn cmpstcha02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha02,
        adcmpsr0::Cmpstcha02,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha02,
            adcmpsr0::Cmpstcha02,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN001"]
    #[inline(always)]
    pub fn cmpstcha01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha01,
        adcmpsr0::Cmpstcha01,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha01,
            adcmpsr0::Cmpstcha01,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN000"]
    #[inline(always)]
    pub fn cmpstcha00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha00,
        adcmpsr0::Cmpstcha00,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha00,
            adcmpsr0::Cmpstcha00,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpsr0 {
    #[inline(always)]
    fn default() -> Adcmpsr0 {
        <crate::RegValueT<Adcmpsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha08_SPEC;
    pub type Cmpstcha08 = crate::EnumBitfieldStruct<u8, Cmpstcha08_SPEC>;
    impl Cmpstcha08 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha07_SPEC;
    pub type Cmpstcha07 = crate::EnumBitfieldStruct<u8, Cmpstcha07_SPEC>;
    impl Cmpstcha07 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha06_SPEC;
    pub type Cmpstcha06 = crate::EnumBitfieldStruct<u8, Cmpstcha06_SPEC>;
    impl Cmpstcha06 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha05_SPEC;
    pub type Cmpstcha05 = crate::EnumBitfieldStruct<u8, Cmpstcha05_SPEC>;
    impl Cmpstcha05 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha04_SPEC;
    pub type Cmpstcha04 = crate::EnumBitfieldStruct<u8, Cmpstcha04_SPEC>;
    impl Cmpstcha04 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha03_SPEC;
    pub type Cmpstcha03 = crate::EnumBitfieldStruct<u8, Cmpstcha03_SPEC>;
    impl Cmpstcha03 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha02_SPEC;
    pub type Cmpstcha02 = crate::EnumBitfieldStruct<u8, Cmpstcha02_SPEC>;
    impl Cmpstcha02 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha01_SPEC;
    pub type Cmpstcha01 = crate::EnumBitfieldStruct<u8, Cmpstcha01_SPEC>;
    impl Cmpstcha01 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha00_SPEC;
    pub type Cmpstcha00 = crate::EnumBitfieldStruct<u8, Cmpstcha00_SPEC>;
    impl Cmpstcha00 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpsr1_SPEC;
impl crate::sealed::RegSpec for Adcmpsr1_SPEC {
    type DataType = u16;
}

#[doc = "A/D Compare Function Window A Channel Status Register 1"]
pub type Adcmpsr1 = crate::RegValueT<Adcmpsr1_SPEC>;

impl Adcmpsr1 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Adcmpsr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Adcmpsr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Compare window A flag for AN024"]
    #[inline(always)]
    pub fn cmpstcha24(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha24,
        adcmpsr1::Cmpstcha24,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha24,
            adcmpsr1::Cmpstcha24,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN023"]
    #[inline(always)]
    pub fn cmpstcha23(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha23,
        adcmpsr1::Cmpstcha23,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha23,
            adcmpsr1::Cmpstcha23,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN022"]
    #[inline(always)]
    pub fn cmpstcha22(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha22,
        adcmpsr1::Cmpstcha22,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha22,
            adcmpsr1::Cmpstcha22,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN021"]
    #[inline(always)]
    pub fn cmpstcha21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha21,
        adcmpsr1::Cmpstcha21,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha21,
            adcmpsr1::Cmpstcha21,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN020"]
    #[inline(always)]
    pub fn cmpstcha20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha20,
        adcmpsr1::Cmpstcha20,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha20,
            adcmpsr1::Cmpstcha20,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN019"]
    #[inline(always)]
    pub fn cmpstcha19(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha19,
        adcmpsr1::Cmpstcha19,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha19,
            adcmpsr1::Cmpstcha19,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN018"]
    #[inline(always)]
    pub fn cmpstcha18(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha18,
        adcmpsr1::Cmpstcha18,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha18,
            adcmpsr1::Cmpstcha18,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN017"]
    #[inline(always)]
    pub fn cmpstcha17(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha17,
        adcmpsr1::Cmpstcha17,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha17,
            adcmpsr1::Cmpstcha17,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare window A flag for AN016"]
    #[inline(always)]
    pub fn cmpstcha16(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha16,
        adcmpsr1::Cmpstcha16,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha16,
            adcmpsr1::Cmpstcha16,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpsr1 {
    #[inline(always)]
    fn default() -> Adcmpsr1 {
        <crate::RegValueT<Adcmpsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpsr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha24_SPEC;
    pub type Cmpstcha24 = crate::EnumBitfieldStruct<u8, Cmpstcha24_SPEC>;
    impl Cmpstcha24 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha23_SPEC;
    pub type Cmpstcha23 = crate::EnumBitfieldStruct<u8, Cmpstcha23_SPEC>;
    impl Cmpstcha23 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha22_SPEC;
    pub type Cmpstcha22 = crate::EnumBitfieldStruct<u8, Cmpstcha22_SPEC>;
    impl Cmpstcha22 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha21_SPEC;
    pub type Cmpstcha21 = crate::EnumBitfieldStruct<u8, Cmpstcha21_SPEC>;
    impl Cmpstcha21 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha20_SPEC;
    pub type Cmpstcha20 = crate::EnumBitfieldStruct<u8, Cmpstcha20_SPEC>;
    impl Cmpstcha20 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha19_SPEC;
    pub type Cmpstcha19 = crate::EnumBitfieldStruct<u8, Cmpstcha19_SPEC>;
    impl Cmpstcha19 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha18_SPEC;
    pub type Cmpstcha18 = crate::EnumBitfieldStruct<u8, Cmpstcha18_SPEC>;
    impl Cmpstcha18 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha17_SPEC;
    pub type Cmpstcha17 = crate::EnumBitfieldStruct<u8, Cmpstcha17_SPEC>;
    impl Cmpstcha17 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha16_SPEC;
    pub type Cmpstcha16 = crate::EnumBitfieldStruct<u8, Cmpstcha16_SPEC>;
    impl Cmpstcha16 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpser_SPEC;
impl crate::sealed::RegSpec for Adcmpser_SPEC {
    type DataType = u8;
}

#[doc = "A/D Compare Function Window A Extended Input Channel Status Register"]
pub type Adcmpser = crate::RegValueT<Adcmpser_SPEC>;

impl Adcmpser {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Adcmpser_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Adcmpser_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Compare Window A Internal Reference Voltage Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
    #[inline(always)]
    pub fn cmpstoca(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpser::Cmpstoca,
        adcmpser::Cmpstoca,
        Adcmpser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpser::Cmpstoca,
            adcmpser::Cmpstoca,
            Adcmpser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Temperature Sensor Output Compare Flag When window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
    #[inline(always)]
    pub fn cmpsttsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpser::Cmpsttsa,
        adcmpser::Cmpsttsa,
        Adcmpser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpser::Cmpsttsa,
            adcmpser::Cmpsttsa,
            Adcmpser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpser {
    #[inline(always)]
    fn default() -> Adcmpser {
        <crate::RegValueT<Adcmpser_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpser {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstoca_SPEC;
    pub type Cmpstoca = crate::EnumBitfieldStruct<u8, Cmpstoca_SPEC>;
    impl Cmpstoca {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpsttsa_SPEC;
    pub type Cmpsttsa = crate::EnumBitfieldStruct<u8, Cmpsttsa_SPEC>;
    impl Cmpsttsa {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpbnsr_SPEC;
impl crate::sealed::RegSpec for Adcmpbnsr_SPEC {
    type DataType = u8;
}

#[doc = "A/D Compare Function Window B Channel Selection Register"]
pub type Adcmpbnsr = crate::RegValueT<Adcmpbnsr_SPEC>;

impl Adcmpbnsr {
    #[doc = "Compare window B Compare condition setting bit."]
    #[inline(always)]
    pub fn cmplb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpbnsr::Cmplb,
        adcmpbnsr::Cmplb,
        Adcmpbnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpbnsr::Cmplb,
            adcmpbnsr::Cmplb,
            Adcmpbnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Adcmpbnsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Adcmpbnsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Compare window B channel selection bit.The channel that compares it on the condition of compare window B is selected."]
    #[inline(always)]
    pub fn cmpchb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        adcmpbnsr::Cmpchb,
        adcmpbnsr::Cmpchb,
        Adcmpbnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            adcmpbnsr::Cmpchb,
            adcmpbnsr::Cmpchb,
            Adcmpbnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpbnsr {
    #[inline(always)]
    fn default() -> Adcmpbnsr {
        <crate::RegValueT<Adcmpbnsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpbnsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplb_SPEC;
    pub type Cmplb = crate::EnumBitfieldStruct<u8, Cmplb_SPEC>;
    impl Cmplb {
        #[doc = "CMPLLB value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < CMPLLB value or CMPULB value < A/D converted value  (ADCMPCR.WCMPE=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "CMPLLB value < A/D converted value(ADCMPCR.WCMPE=0) / CMPLLB value < A/D converted value < CMPULB value (ADCMPCR.WCMPE=1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchb_SPEC;
    pub type Cmpchb = crate::EnumBitfieldStruct<u8, Cmpchb_SPEC>;
    impl Cmpchb {
        #[doc = "AN000"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "AN001"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "AN002"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "AN003"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "AN004"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "AN005"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "AN006"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "AN007"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "AN008"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "AN016"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "AN017"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "AN018"]
        pub const _0_X_12: Self = Self::new(18);

        #[doc = "AN019"]
        pub const _0_X_13: Self = Self::new(19);

        #[doc = "AN020"]
        pub const _0_X_14: Self = Self::new(20);

        #[doc = "AN021"]
        pub const _0_X_15: Self = Self::new(21);

        #[doc = "AN022"]
        pub const _0_X_16: Self = Self::new(22);

        #[doc = "AN023"]
        pub const _0_X_17: Self = Self::new(23);

        #[doc = "Temperature sensor"]
        pub const _0_X_20: Self = Self::new(32);

        #[doc = "Internal reference voltage"]
        pub const _0_X_21: Self = Self::new(33);

        #[doc = "No channel is selected"]
        pub const _0_X_3_F: Self = Self::new(63);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adwinllb_SPEC;
impl crate::sealed::RegSpec for Adwinllb_SPEC {
    type DataType = u16;
}

#[doc = "A/D Compare Function Window B Lower-Side Level Setting Register"]
pub type Adwinllb = crate::RegValueT<Adwinllb_SPEC>;

impl Adwinllb {
    #[doc = "This register is used to compare A window function is used to set the lower level of the window B."]
    #[inline(always)]
    pub fn adwinllb(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adwinllb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adwinllb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adwinllb {
    #[inline(always)]
    fn default() -> Adwinllb {
        <crate::RegValueT<Adwinllb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adwinulb_SPEC;
impl crate::sealed::RegSpec for Adwinulb_SPEC {
    type DataType = u16;
}

#[doc = "A/D Compare Function Window B Upper-Side Level Setting Register"]
pub type Adwinulb = crate::RegValueT<Adwinulb_SPEC>;

impl Adwinulb {
    #[doc = "This register is used to compare A window function is used to set the higher level of the window B."]
    #[inline(always)]
    pub fn adwinulb(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adwinulb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adwinulb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adwinulb {
    #[inline(always)]
    fn default() -> Adwinulb {
        <crate::RegValueT<Adwinulb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpbsr_SPEC;
impl crate::sealed::RegSpec for Adcmpbsr_SPEC {
    type DataType = u8;
}

#[doc = "A/D Compare Function Window B Status Register"]
pub type Adcmpbsr = crate::RegValueT<Adcmpbsr_SPEC>;

impl Adcmpbsr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Adcmpbsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Adcmpbsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Compare window B flagWhen window B operation is enabled (ADCMPCR.CMPBE = 1), this bit indicates the comparison result of channels AN000 to AN008, AN016 to AN023, reference voltage of SDADC24 (SBIAS/VREFI), temperature sensor output, and internal reference voltage to which window B comparison conditions are applied."]
    #[inline(always)]
    pub fn cmpstb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpbsr::Cmpstb,
        adcmpbsr::Cmpstb,
        Adcmpbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpbsr::Cmpstb,
            adcmpbsr::Cmpstb,
            Adcmpbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpbsr {
    #[inline(always)]
    fn default() -> Adcmpbsr {
        <crate::RegValueT<Adcmpbsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpbsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstb_SPEC;
    pub type Cmpstb = crate::EnumBitfieldStruct<u8, Cmpstb_SPEC>;
    impl Cmpstb {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstrl_SPEC;
impl crate::sealed::RegSpec for Adsstrl_SPEC {
    type DataType = u8;
}

#[doc = "A/D Sampling State Register L"]
pub type Adsstrl = crate::RegValueT<Adsstrl_SPEC>;

impl Adsstrl {
    #[doc = "Sampling Time Setting (AN016-AN023, SBIAS/VREFI)"]
    #[inline(always)]
    pub fn sst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adsstrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adsstrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstrl {
    #[inline(always)]
    fn default() -> Adsstrl {
        <crate::RegValueT<Adsstrl_SPEC> as RegisterValue<_>>::new(13)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstrt_SPEC;
impl crate::sealed::RegSpec for Adsstrt_SPEC {
    type DataType = u8;
}

#[doc = "A/D Sampling State Register T"]
pub type Adsstrt = crate::RegValueT<Adsstrt_SPEC>;

impl Adsstrt {
    #[doc = "Sampling Time Setting (Temperature sensor output)"]
    #[inline(always)]
    pub fn sst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adsstrt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adsstrt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstrt {
    #[inline(always)]
    fn default() -> Adsstrt {
        <crate::RegValueT<Adsstrt_SPEC> as RegisterValue<_>>::new(13)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstro_SPEC;
impl crate::sealed::RegSpec for Adsstro_SPEC {
    type DataType = u8;
}

#[doc = "A/D Sampling State Register O"]
pub type Adsstro = crate::RegValueT<Adsstro_SPEC>;

impl Adsstro {
    #[doc = "Sampling Time Setting (Internal reference voltage)"]
    #[inline(always)]
    pub fn sst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adsstro_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adsstro_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstro {
    #[inline(always)]
    fn default() -> Adsstro {
        <crate::RegValueT<Adsstro_SPEC> as RegisterValue<_>>::new(13)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstr0_SPEC;
impl crate::sealed::RegSpec for Adsstr0_SPEC {
    type DataType = u8;
}

#[doc = "A/D Sampling State Register %s"]
pub type Adsstr0 = crate::RegValueT<Adsstr0_SPEC>;

impl Adsstr0 {
    #[doc = "Sampling time setting"]
    #[inline(always)]
    pub fn sst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adsstr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adsstr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstr0 {
    #[inline(always)]
    fn default() -> Adsstr0 {
        <crate::RegValueT<Adsstr0_SPEC> as RegisterValue<_>>::new(13)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adanim_SPEC;
impl crate::sealed::RegSpec for Adanim_SPEC {
    type DataType = u16;
}

#[doc = "A/D Channel Input Mode Select Register"]
pub type Adanim = crate::RegValueT<Adanim_SPEC>;

impl Adanim {
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, u16, Adanim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfff,1,0,u16,u16,Adanim_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Analog Channel Input Mode Select"]
    #[inline(always)]
    pub fn anim(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Adanim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Adanim_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adanim {
    #[inline(always)]
    fn default() -> Adanim {
        <crate::RegValueT<Adanim_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcalexe_SPEC;
impl crate::sealed::RegSpec for Adcalexe_SPEC {
    type DataType = u8;
}

#[doc = "A/D Calibration Execution Register"]
pub type Adcalexe = crate::RegValueT<Adcalexe_SPEC>;

impl Adcalexe {
    #[doc = "Calibration Start"]
    #[inline(always)]
    pub fn calexe(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcalexe::Calexe,
        adcalexe::Calexe,
        Adcalexe_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcalexe::Calexe,
            adcalexe::Calexe,
            Adcalexe_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Calibration Status Flag"]
    #[inline(always)]
    pub fn calmon(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcalexe::Calmon,
        adcalexe::Calmon,
        Adcalexe_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcalexe::Calmon,
            adcalexe::Calmon,
            Adcalexe_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Adcalexe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Adcalexe_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adcalexe {
    #[inline(always)]
    fn default() -> Adcalexe {
        <crate::RegValueT<Adcalexe_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcalexe {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Calexe_SPEC;
    pub type Calexe = crate::EnumBitfieldStruct<u8, Calexe_SPEC>;
    impl Calexe {
        #[doc = "Calibration does not start"]
        pub const _0: Self = Self::new(0);

        #[doc = "Calibration starts"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Calmon_SPEC;
    pub type Calmon = crate::EnumBitfieldStruct<u8, Calmon_SPEC>;
    impl Calmon {
        #[doc = "Calibration is not in progress"]
        pub const _0: Self = Self::new(0);

        #[doc = "Calibration is in progress"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vrefampcnt_SPEC;
impl crate::sealed::RegSpec for Vrefampcnt_SPEC {
    type DataType = u8;
}

#[doc = "A/D Dedicated Reference Voltage Circuit Control Register"]
pub type Vrefampcnt = crate::RegValueT<Vrefampcnt_SPEC>;

impl Vrefampcnt {
    #[doc = "Sleep"]
    #[inline(always)]
    pub fn adslp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        vrefampcnt::Adslp,
        vrefampcnt::Adslp,
        Vrefampcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            vrefampcnt::Adslp,
            vrefampcnt::Adslp,
            Vrefampcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, Vrefampcnt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,Vrefampcnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Low-Potential Reference Voltage Select"]
    #[inline(always)]
    pub fn bgren(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        vrefampcnt::Bgren,
        vrefampcnt::Bgren,
        Vrefampcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            vrefampcnt::Bgren,
            vrefampcnt::Bgren,
            Vrefampcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VREFADCG Enable"]
    #[inline(always)]
    pub fn vrefadcen(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        vrefampcnt::Vrefadcen,
        vrefampcnt::Vrefadcen,
        Vrefampcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            vrefampcnt::Vrefadcen,
            vrefampcnt::Vrefadcen,
            Vrefampcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VREFADC Output Voltage Control"]
    #[inline(always)]
    pub fn vrefadcg(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        vrefampcnt::Vrefadcg,
        vrefampcnt::Vrefadcg,
        Vrefampcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            vrefampcnt::Vrefadcg,
            vrefampcnt::Vrefadcg,
            Vrefampcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "OLDET Enable"]
    #[inline(always)]
    pub fn oldeten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vrefampcnt::Oldeten,
        vrefampcnt::Oldeten,
        Vrefampcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vrefampcnt::Oldeten,
            vrefampcnt::Oldeten,
            Vrefampcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vrefampcnt {
    #[inline(always)]
    fn default() -> Vrefampcnt {
        <crate::RegValueT<Vrefampcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vrefampcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adslp_SPEC;
    pub type Adslp = crate::EnumBitfieldStruct<u8, Adslp_SPEC>;
    impl Adslp {
        #[doc = "Normal operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Standby state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bgren_SPEC;
    pub type Bgren = crate::EnumBitfieldStruct<u8, Bgren_SPEC>;
    impl Bgren {
        #[doc = "Select AVSS0 as the low-potential reference voltage"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select VREFL0 as the low-potential reference voltage."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vrefadcen_SPEC;
    pub type Vrefadcen = crate::EnumBitfieldStruct<u8, Vrefadcen_SPEC>;
    impl Vrefadcen {
        #[doc = "Disable the VREFADC output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the VREFADC output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vrefadcg_SPEC;
    pub type Vrefadcg = crate::EnumBitfieldStruct<u8, Vrefadcg_SPEC>;
    impl Vrefadcg {
        #[doc = "1.5 V"]
        pub const _00: Self = Self::new(0);

        #[doc = "1.5 V"]
        pub const _01: Self = Self::new(1);

        #[doc = "2.0 V"]
        pub const _10: Self = Self::new(2);

        #[doc = "2.5 V"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oldeten_SPEC;
    pub type Oldeten = crate::EnumBitfieldStruct<u8, Oldeten_SPEC>;
    impl Oldeten {
        #[doc = "Disable the over current detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the over current detection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adrd_SPEC;
impl crate::sealed::RegSpec for Adrd_SPEC {
    type DataType = u16;
}

#[doc = "A/D Self-Diagnosis Data Register"]
pub type Adrd = crate::RegValueT<Adrd_SPEC>;

impl Adrd {
    #[doc = "The ADRD register is a 16-bit read-only register that holds the A/D conversion results based on the self-diagnosis of the ADC16."]
    #[inline(always)]
    pub fn adrd(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adrd_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adrd_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Adrd {
    #[inline(always)]
    fn default() -> Adrd {
        <crate::RegValueT<Adrd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adrst_SPEC;
impl crate::sealed::RegSpec for Adrst_SPEC {
    type DataType = u8;
}

#[doc = "A/D Self-Diagnostic Status Register"]
pub type Adrst = crate::RegValueT<Adrst_SPEC>;

impl Adrst {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Adrst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Adrst_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Self-Diagnosis Status"]
    #[inline(always)]
    pub fn diagst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adrst::Diagst,
        adrst::Diagst,
        Adrst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adrst::Diagst,
            adrst::Diagst,
            Adrst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adrst {
    #[inline(always)]
    fn default() -> Adrst {
        <crate::RegValueT<Adrst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adrst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diagst_SPEC;
    pub type Diagst = crate::EnumBitfieldStruct<u8, Diagst_SPEC>;
    impl Diagst {
        #[doc = "Self-diagnosis has not been executed since power-on"]
        pub const _00: Self = Self::new(0);

        #[doc = "Self-diagnosis was executed under a condition that an ideal value of the A/D conversion result is 8000h"]
        pub const _01: Self = Self::new(1);

        #[doc = "Self-diagnosis was executed under a condition that an ideal value of the A/D conversion result is 0000h"]
        pub const _10: Self = Self::new(2);

        #[doc = "Self-diagnosis was executed under a condition that an ideal value of the A/D conversion result is 7FFFh."]
        pub const _11: Self = Self::new(3);
    }
}
