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
// Generated from SVD 1.50.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:47:04 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"12-bit A/D Converter"]
unsafe impl ::core::marker::Send for super::Adc120 {}
unsafe impl ::core::marker::Sync for super::Adc120 {}
impl super::Adc120 {
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select Register 0"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select Register 1"]
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

    #[doc = "A/D-Converted Value Addition/Average Count Select Register"]
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

    #[doc = "A/D Conversion Extended Input Control Registers"]
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

    #[doc = "A/D Self-Diagnosis Data Register"]
    #[inline(always)]
    pub const fn adrd(&self) -> &'static crate::common::Reg<self::Adrd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adrd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[doc = "A/D CTSU TSCAP Voltage Data Register"]
    #[inline(always)]
    pub const fn adctdr(&self) -> &'static crate::common::Reg<self::Adctdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adctdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "A/D Data Registers %s"]
    #[inline(always)]
    pub const fn addr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Addr_SPEC, crate::common::R>,
        4,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x42usize))
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

    #[doc = "A/D Conversion Operation Mode Select Register"]
    #[inline(always)]
    pub const fn adacsr(
        &self,
    ) -> &'static crate::common::Reg<self::Adacsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adacsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(126usize),
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

    #[doc = "A/D High-Potential/Low-Potential Reference Voltage Control Register"]
    #[inline(always)]
    pub const fn adhvrefcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Adhvrefcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adhvrefcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(138usize),
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

    #[doc = "A/D Compare Function Window A Lower-Side/Upper-Side Level Setting Register"]
    #[inline(always)]
    pub const fn adcmpdr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adcmpdr_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x9cusize))
        }
    }
    #[inline(always)]
    pub const fn adcmpdr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn adcmpdr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9eusize),
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

    #[doc = "A/D Compare Function Window A Channel Status Register1"]
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

    #[doc = "A/D Compare Function Window B Channel Select Register"]
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

    #[doc = "A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register"]
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

    #[doc = "A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register"]
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

    #[doc = "A/D Sampling State Register"]
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

    #[doc = "A/D Sampling State Register"]
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

    #[doc = "A/D Sampling State Register"]
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

    #[doc = "A/D Sampling State Register"]
    #[inline(always)]
    pub const fn adsstr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adsstr_SPEC, crate::common::RW>,
        15,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe0usize))
        }
    }
    #[inline(always)]
    pub const fn adsstr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr2(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr3(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr4(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr5(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr6(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr7(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr8(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr9(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe9usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr10(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeausize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr11(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xebusize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr12(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr13(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xedusize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr14(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeeusize),
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
    #[doc = "Double Trigger Channel Select"]
    #[inline(always)]
    pub fn dblans(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Adcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Adcsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Group B Scan End Interrupt and ELC Event Enable"]
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

    #[doc = "A/D Conversion Mode Select"]
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
}
impl ::core::default::Default for Adcsr {
    #[inline(always)]
    fn default() -> Adcsr {
        <crate::RegValueT<Adcsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gbadie_SPEC;
    pub type Gbadie = crate::EnumBitfieldStruct<u8, Gbadie_SPEC>;
    impl Gbadie {
        #[doc = "Disable ADC120_GBADI interrupt generation on group B scan completion."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable ADC120_GBADI interrupt generation on group B scan completion."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dble_SPEC;
    pub type Dble = crate::EnumBitfieldStruct<u8, Dble_SPEC>;
    impl Dble {
        #[doc = "Deselect double-trigger mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select double-trigger mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Extrg_SPEC;
    pub type Extrg = crate::EnumBitfieldStruct<u8, Extrg_SPEC>;
    impl Extrg {
        #[doc = "Start A/D conversion by the synchronous trigger (ELC)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start A/D conversion by the asynchronous trigger (ADTRG0)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trge_SPEC;
    pub type Trge = crate::EnumBitfieldStruct<u8, Trge_SPEC>;
    impl Trge {
        #[doc = "Disable A/D conversion to be started by the synchronous or asynchronous trigger"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable A/D conversion to be started by the synchronous or asynchronous trigger"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adhsc_SPEC;
    pub type Adhsc = crate::EnumBitfieldStruct<u8, Adhsc_SPEC>;
    impl Adhsc {
        #[doc = "High-speed A/D conversion mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low-power A/D conversion mode"]
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
    pub struct Adst_SPEC;
    pub type Adst = crate::EnumBitfieldStruct<u8, Adst_SPEC>;
    impl Adst {
        #[doc = "Stop A/D conversion process."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start A/D conversion process."]
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
    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adansa0::Ansa09,
        adansa0::Ansa09,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adansa0::Ansa09,
            adansa0::Ansa09,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adansa0::Ansa10,
        adansa0::Ansa10,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adansa0::Ansa10,
            adansa0::Ansa10,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adansa0::Ansa11,
        adansa0::Ansa11,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adansa0::Ansa11,
            adansa0::Ansa11,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adansa0::Ansa12,
        adansa0::Ansa12,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adansa0::Ansa12,
            adansa0::Ansa12,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adansa0::Ansa13,
        adansa0::Ansa13,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adansa0::Ansa13,
            adansa0::Ansa13,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adansa0::Ansa14,
        adansa0::Ansa14,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adansa0::Ansa14,
            adansa0::Ansa14,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adansa0::Ansa15,
        adansa0::Ansa15,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adansa0::Ansa15,
            adansa0::Ansa15,
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
    pub struct Ansa00_SPEC;
    pub type Ansa00 = crate::EnumBitfieldStruct<u8, Ansa00_SPEC>;
    impl Ansa00 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa01_SPEC;
    pub type Ansa01 = crate::EnumBitfieldStruct<u8, Ansa01_SPEC>;
    impl Ansa01 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa02_SPEC;
    pub type Ansa02 = crate::EnumBitfieldStruct<u8, Ansa02_SPEC>;
    impl Ansa02 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa03_SPEC;
    pub type Ansa03 = crate::EnumBitfieldStruct<u8, Ansa03_SPEC>;
    impl Ansa03 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa04_SPEC;
    pub type Ansa04 = crate::EnumBitfieldStruct<u8, Ansa04_SPEC>;
    impl Ansa04 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa05_SPEC;
    pub type Ansa05 = crate::EnumBitfieldStruct<u8, Ansa05_SPEC>;
    impl Ansa05 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa06_SPEC;
    pub type Ansa06 = crate::EnumBitfieldStruct<u8, Ansa06_SPEC>;
    impl Ansa06 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa07_SPEC;
    pub type Ansa07 = crate::EnumBitfieldStruct<u8, Ansa07_SPEC>;
    impl Ansa07 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa08_SPEC;
    pub type Ansa08 = crate::EnumBitfieldStruct<u8, Ansa08_SPEC>;
    impl Ansa08 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa09_SPEC;
    pub type Ansa09 = crate::EnumBitfieldStruct<u8, Ansa09_SPEC>;
    impl Ansa09 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa10_SPEC;
    pub type Ansa10 = crate::EnumBitfieldStruct<u8, Ansa10_SPEC>;
    impl Ansa10 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa11_SPEC;
    pub type Ansa11 = crate::EnumBitfieldStruct<u8, Ansa11_SPEC>;
    impl Ansa11 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa12_SPEC;
    pub type Ansa12 = crate::EnumBitfieldStruct<u8, Ansa12_SPEC>;
    impl Ansa12 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa13_SPEC;
    pub type Ansa13 = crate::EnumBitfieldStruct<u8, Ansa13_SPEC>;
    impl Ansa13 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa14_SPEC;
    pub type Ansa14 = crate::EnumBitfieldStruct<u8, Ansa14_SPEC>;
    impl Ansa14 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa15_SPEC;
    pub type Ansa15 = crate::EnumBitfieldStruct<u8, Ansa15_SPEC>;
    impl Ansa15 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
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
    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa25(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adansa1::Ansa25,
        adansa1::Ansa25,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adansa1::Ansa25,
            adansa1::Ansa25,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa26(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adansa1::Ansa26,
        adansa1::Ansa26,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adansa1::Ansa26,
            adansa1::Ansa26,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa27(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adansa1::Ansa27,
        adansa1::Ansa27,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adansa1::Ansa27,
            adansa1::Ansa27,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa28(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adansa1::Ansa28,
        adansa1::Ansa28,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adansa1::Ansa28,
            adansa1::Ansa28,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa29(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adansa1::Ansa29,
        adansa1::Ansa29,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adansa1::Ansa29,
            adansa1::Ansa29,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa30(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adansa1::Ansa30,
        adansa1::Ansa30,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adansa1::Ansa30,
            adansa1::Ansa30,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansa31(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adansa1::Ansa31,
        adansa1::Ansa31,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adansa1::Ansa31,
            adansa1::Ansa31,
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
    pub struct Ansa16_SPEC;
    pub type Ansa16 = crate::EnumBitfieldStruct<u8, Ansa16_SPEC>;
    impl Ansa16 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa17_SPEC;
    pub type Ansa17 = crate::EnumBitfieldStruct<u8, Ansa17_SPEC>;
    impl Ansa17 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa18_SPEC;
    pub type Ansa18 = crate::EnumBitfieldStruct<u8, Ansa18_SPEC>;
    impl Ansa18 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa19_SPEC;
    pub type Ansa19 = crate::EnumBitfieldStruct<u8, Ansa19_SPEC>;
    impl Ansa19 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa20_SPEC;
    pub type Ansa20 = crate::EnumBitfieldStruct<u8, Ansa20_SPEC>;
    impl Ansa20 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa21_SPEC;
    pub type Ansa21 = crate::EnumBitfieldStruct<u8, Ansa21_SPEC>;
    impl Ansa21 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa22_SPEC;
    pub type Ansa22 = crate::EnumBitfieldStruct<u8, Ansa22_SPEC>;
    impl Ansa22 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa23_SPEC;
    pub type Ansa23 = crate::EnumBitfieldStruct<u8, Ansa23_SPEC>;
    impl Ansa23 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa24_SPEC;
    pub type Ansa24 = crate::EnumBitfieldStruct<u8, Ansa24_SPEC>;
    impl Ansa24 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa25_SPEC;
    pub type Ansa25 = crate::EnumBitfieldStruct<u8, Ansa25_SPEC>;
    impl Ansa25 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa26_SPEC;
    pub type Ansa26 = crate::EnumBitfieldStruct<u8, Ansa26_SPEC>;
    impl Ansa26 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa27_SPEC;
    pub type Ansa27 = crate::EnumBitfieldStruct<u8, Ansa27_SPEC>;
    impl Ansa27 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa28_SPEC;
    pub type Ansa28 = crate::EnumBitfieldStruct<u8, Ansa28_SPEC>;
    impl Ansa28 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa29_SPEC;
    pub type Ansa29 = crate::EnumBitfieldStruct<u8, Ansa29_SPEC>;
    impl Ansa29 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa30_SPEC;
    pub type Ansa30 = crate::EnumBitfieldStruct<u8, Ansa30_SPEC>;
    impl Ansa30 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa31_SPEC;
    pub type Ansa31 = crate::EnumBitfieldStruct<u8, Ansa31_SPEC>;
    impl Ansa31 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adads0_SPEC;
impl crate::sealed::RegSpec for Adads0_SPEC {
    type DataType = u16;
}

#[doc = "A/D-Converted Value Addition/Average Channel Select Register 0"]
pub type Adads0 = crate::RegValueT<Adads0_SPEC>;

impl Adads0 {
    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adads0::Ads09,
        adads0::Ads09,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adads0::Ads09,
            adads0::Ads09,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adads0::Ads10,
        adads0::Ads10,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adads0::Ads10,
            adads0::Ads10,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adads0::Ads11,
        adads0::Ads11,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adads0::Ads11,
            adads0::Ads11,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adads0::Ads12,
        adads0::Ads12,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adads0::Ads12,
            adads0::Ads12,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adads0::Ads13,
        adads0::Ads13,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adads0::Ads13,
            adads0::Ads13,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adads0::Ads14,
        adads0::Ads14,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adads0::Ads14,
            adads0::Ads14,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adads0::Ads15,
        adads0::Ads15,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adads0::Ads15,
            adads0::Ads15,
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
    pub struct Ads00_SPEC;
    pub type Ads00 = crate::EnumBitfieldStruct<u8, Ads00_SPEC>;
    impl Ads00 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads01_SPEC;
    pub type Ads01 = crate::EnumBitfieldStruct<u8, Ads01_SPEC>;
    impl Ads01 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads02_SPEC;
    pub type Ads02 = crate::EnumBitfieldStruct<u8, Ads02_SPEC>;
    impl Ads02 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads03_SPEC;
    pub type Ads03 = crate::EnumBitfieldStruct<u8, Ads03_SPEC>;
    impl Ads03 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads04_SPEC;
    pub type Ads04 = crate::EnumBitfieldStruct<u8, Ads04_SPEC>;
    impl Ads04 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads05_SPEC;
    pub type Ads05 = crate::EnumBitfieldStruct<u8, Ads05_SPEC>;
    impl Ads05 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads06_SPEC;
    pub type Ads06 = crate::EnumBitfieldStruct<u8, Ads06_SPEC>;
    impl Ads06 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads07_SPEC;
    pub type Ads07 = crate::EnumBitfieldStruct<u8, Ads07_SPEC>;
    impl Ads07 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads08_SPEC;
    pub type Ads08 = crate::EnumBitfieldStruct<u8, Ads08_SPEC>;
    impl Ads08 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads09_SPEC;
    pub type Ads09 = crate::EnumBitfieldStruct<u8, Ads09_SPEC>;
    impl Ads09 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads10_SPEC;
    pub type Ads10 = crate::EnumBitfieldStruct<u8, Ads10_SPEC>;
    impl Ads10 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads11_SPEC;
    pub type Ads11 = crate::EnumBitfieldStruct<u8, Ads11_SPEC>;
    impl Ads11 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads12_SPEC;
    pub type Ads12 = crate::EnumBitfieldStruct<u8, Ads12_SPEC>;
    impl Ads12 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads13_SPEC;
    pub type Ads13 = crate::EnumBitfieldStruct<u8, Ads13_SPEC>;
    impl Ads13 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads14_SPEC;
    pub type Ads14 = crate::EnumBitfieldStruct<u8, Ads14_SPEC>;
    impl Ads14 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads15_SPEC;
    pub type Ads15 = crate::EnumBitfieldStruct<u8, Ads15_SPEC>;
    impl Ads15 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adads1_SPEC;
impl crate::sealed::RegSpec for Adads1_SPEC {
    type DataType = u16;
}

#[doc = "A/D-Converted Value Addition/Average Channel Select Register 1"]
pub type Adads1 = crate::RegValueT<Adads1_SPEC>;

impl Adads1 {
    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads25(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adads1::Ads25,
        adads1::Ads25,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adads1::Ads25,
            adads1::Ads25,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads26(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adads1::Ads26,
        adads1::Ads26,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adads1::Ads26,
            adads1::Ads26,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads27(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adads1::Ads27,
        adads1::Ads27,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adads1::Ads27,
            adads1::Ads27,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads28(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adads1::Ads28,
        adads1::Ads28,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adads1::Ads28,
            adads1::Ads28,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads29(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adads1::Ads29,
        adads1::Ads29,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adads1::Ads29,
            adads1::Ads29,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads30(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adads1::Ads30,
        adads1::Ads30,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adads1::Ads30,
            adads1::Ads30,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn ads31(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adads1::Ads31,
        adads1::Ads31,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adads1::Ads31,
            adads1::Ads31,
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
    pub struct Ads16_SPEC;
    pub type Ads16 = crate::EnumBitfieldStruct<u8, Ads16_SPEC>;
    impl Ads16 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads17_SPEC;
    pub type Ads17 = crate::EnumBitfieldStruct<u8, Ads17_SPEC>;
    impl Ads17 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads18_SPEC;
    pub type Ads18 = crate::EnumBitfieldStruct<u8, Ads18_SPEC>;
    impl Ads18 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads19_SPEC;
    pub type Ads19 = crate::EnumBitfieldStruct<u8, Ads19_SPEC>;
    impl Ads19 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads20_SPEC;
    pub type Ads20 = crate::EnumBitfieldStruct<u8, Ads20_SPEC>;
    impl Ads20 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads21_SPEC;
    pub type Ads21 = crate::EnumBitfieldStruct<u8, Ads21_SPEC>;
    impl Ads21 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads22_SPEC;
    pub type Ads22 = crate::EnumBitfieldStruct<u8, Ads22_SPEC>;
    impl Ads22 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads23_SPEC;
    pub type Ads23 = crate::EnumBitfieldStruct<u8, Ads23_SPEC>;
    impl Ads23 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads24_SPEC;
    pub type Ads24 = crate::EnumBitfieldStruct<u8, Ads24_SPEC>;
    impl Ads24 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads25_SPEC;
    pub type Ads25 = crate::EnumBitfieldStruct<u8, Ads25_SPEC>;
    impl Ads25 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads26_SPEC;
    pub type Ads26 = crate::EnumBitfieldStruct<u8, Ads26_SPEC>;
    impl Ads26 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads27_SPEC;
    pub type Ads27 = crate::EnumBitfieldStruct<u8, Ads27_SPEC>;
    impl Ads27 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads28_SPEC;
    pub type Ads28 = crate::EnumBitfieldStruct<u8, Ads28_SPEC>;
    impl Ads28 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads29_SPEC;
    pub type Ads29 = crate::EnumBitfieldStruct<u8, Ads29_SPEC>;
    impl Ads29 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads30_SPEC;
    pub type Ads30 = crate::EnumBitfieldStruct<u8, Ads30_SPEC>;
    impl Ads30 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads31_SPEC;
    pub type Ads31 = crate::EnumBitfieldStruct<u8, Ads31_SPEC>;
    impl Ads31 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adadc_SPEC;
impl crate::sealed::RegSpec for Adadc_SPEC {
    type DataType = u8;
}

#[doc = "A/D-Converted Value Addition/Average Count Select Register"]
pub type Adadc = crate::RegValueT<Adadc_SPEC>;

impl Adadc {
    #[doc = "Addition/Average Count Select"]
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

    #[doc = "Average Mode Select"]
    #[inline(always)]
    pub fn avee(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adadc::Avee,
        adadc::Avee,
        Adadc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adadc::Avee,
            adadc::Avee,
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
        #[doc = "1-time conversion (no addition, same as normal conversion)"]
        pub const _000: Self = Self::new(0);

        #[doc = "2-time conversion (1 addition)"]
        pub const _001: Self = Self::new(1);

        #[doc = "3-time conversion (2 additions)"]
        pub const _010: Self = Self::new(2);

        #[doc = "4-time conversion (3 additions)"]
        pub const _011: Self = Self::new(3);

        #[doc = "16-time conversion (15 additions)"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Avee_SPEC;
    pub type Avee = crate::EnumBitfieldStruct<u8, Avee_SPEC>;
    impl Avee {
        #[doc = "Enable addition mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable average mode"]
        pub const _1: Self = Self::new(1);
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

    #[doc = "A/D Data Register Format Select"]
    #[inline(always)]
    pub fn adrfmt(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adcer::Adrfmt,
        adcer::Adrfmt,
        Adcer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adcer::Adrfmt,
            adcer::Adrfmt,
            Adcer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    pub struct Ace_SPEC;
    pub type Ace = crate::EnumBitfieldStruct<u8, Ace_SPEC>;
    impl Ace {
        #[doc = "Disable automatic clearing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable automatic clearing"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diagval_SPEC;
    pub type Diagval = crate::EnumBitfieldStruct<u8, Diagval_SPEC>;
    impl Diagval {
        #[doc = "Setting prohibited when self-diagnosis is enabled"]
        pub const _00: Self = Self::new(0);

        #[doc = "0 volts"]
        pub const _01: Self = Self::new(1);

        #[doc = "Reference voltage × 1/2"]
        pub const _10: Self = Self::new(2);

        #[doc = "Reference voltage"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diagld_SPEC;
    pub type Diagld = crate::EnumBitfieldStruct<u8, Diagld_SPEC>;
    impl Diagld {
        #[doc = "Select rotation mode for self-diagnosis voltage"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select mixed mode for self-diagnosis voltage"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diagm_SPEC;
    pub type Diagm = crate::EnumBitfieldStruct<u8, Diagm_SPEC>;
    impl Diagm {
        #[doc = "Disable ADC12 self-diagnosis"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable ADC12 self-diagnosis"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adrfmt_SPEC;
    pub type Adrfmt = crate::EnumBitfieldStruct<u8, Adrfmt_SPEC>;
    impl Adrfmt {
        #[doc = "Select right-justified for the A/D data register format"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select left-justified for the A/D data register format"]
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
    #[doc = "A/D Conversion Start Trigger Select for Group B"]
    #[inline(always)]
    pub fn trsb(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Adstrgr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Adstrgr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "A/D Conversion Start Trigger Select"]
    #[inline(always)]
    pub fn trsa(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Adstrgr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Adstrgr_SPEC,crate::common::RW>::from_register(self,0)
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

#[doc = "A/D Conversion Extended Input Control Registers"]
pub type Adexicr = crate::RegValueT<Adexicr_SPEC>;

impl Adexicr {
    #[doc = "Temperature Sensor Output A/D-Converted Value Addition/Average Mode Select"]
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

    #[doc = "Internal Reference Voltage A/D-Converted Value Addition/Average Mode Select"]
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
}
impl ::core::default::Default for Adexicr {
    #[inline(always)]
    fn default() -> Adexicr {
        <crate::RegValueT<Adexicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adexicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tssad_SPEC;
    pub type Tssad = crate::EnumBitfieldStruct<u8, Tssad_SPEC>;
    impl Tssad {
        #[doc = "Do not select addition/average mode for temperature sensor output."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select addition/average mode for temperature sensor output."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ocsad_SPEC;
    pub type Ocsad = crate::EnumBitfieldStruct<u8, Ocsad_SPEC>;
    impl Ocsad {
        #[doc = "Do not select addition/average mode for internal reference voltage."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select addition/average mode for internal reference voltage."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tssa_SPEC;
    pub type Tssa = crate::EnumBitfieldStruct<u8, Tssa_SPEC>;
    impl Tssa {
        #[doc = "Disable A/D conversion of temperature sensor output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable A/D conversion of temperature sensor output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ocsa_SPEC;
    pub type Ocsa = crate::EnumBitfieldStruct<u8, Ocsa_SPEC>;
    impl Ocsa {
        #[doc = "Disable A/D conversion of internal reference voltage"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable A/D conversion of internal reference voltage"]
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
    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adansb0::Ansb09,
        adansb0::Ansb09,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adansb0::Ansb09,
            adansb0::Ansb09,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adansb0::Ansb10,
        adansb0::Ansb10,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adansb0::Ansb10,
            adansb0::Ansb10,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adansb0::Ansb11,
        adansb0::Ansb11,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adansb0::Ansb11,
            adansb0::Ansb11,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adansb0::Ansb12,
        adansb0::Ansb12,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adansb0::Ansb12,
            adansb0::Ansb12,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adansb0::Ansb13,
        adansb0::Ansb13,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adansb0::Ansb13,
            adansb0::Ansb13,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adansb0::Ansb14,
        adansb0::Ansb14,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adansb0::Ansb14,
            adansb0::Ansb14,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adansb0::Ansb15,
        adansb0::Ansb15,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adansb0::Ansb15,
            adansb0::Ansb15,
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
    pub struct Ansb00_SPEC;
    pub type Ansb00 = crate::EnumBitfieldStruct<u8, Ansb00_SPEC>;
    impl Ansb00 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb01_SPEC;
    pub type Ansb01 = crate::EnumBitfieldStruct<u8, Ansb01_SPEC>;
    impl Ansb01 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb02_SPEC;
    pub type Ansb02 = crate::EnumBitfieldStruct<u8, Ansb02_SPEC>;
    impl Ansb02 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb03_SPEC;
    pub type Ansb03 = crate::EnumBitfieldStruct<u8, Ansb03_SPEC>;
    impl Ansb03 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb04_SPEC;
    pub type Ansb04 = crate::EnumBitfieldStruct<u8, Ansb04_SPEC>;
    impl Ansb04 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb05_SPEC;
    pub type Ansb05 = crate::EnumBitfieldStruct<u8, Ansb05_SPEC>;
    impl Ansb05 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb06_SPEC;
    pub type Ansb06 = crate::EnumBitfieldStruct<u8, Ansb06_SPEC>;
    impl Ansb06 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb07_SPEC;
    pub type Ansb07 = crate::EnumBitfieldStruct<u8, Ansb07_SPEC>;
    impl Ansb07 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb08_SPEC;
    pub type Ansb08 = crate::EnumBitfieldStruct<u8, Ansb08_SPEC>;
    impl Ansb08 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb09_SPEC;
    pub type Ansb09 = crate::EnumBitfieldStruct<u8, Ansb09_SPEC>;
    impl Ansb09 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb10_SPEC;
    pub type Ansb10 = crate::EnumBitfieldStruct<u8, Ansb10_SPEC>;
    impl Ansb10 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb11_SPEC;
    pub type Ansb11 = crate::EnumBitfieldStruct<u8, Ansb11_SPEC>;
    impl Ansb11 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb12_SPEC;
    pub type Ansb12 = crate::EnumBitfieldStruct<u8, Ansb12_SPEC>;
    impl Ansb12 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb13_SPEC;
    pub type Ansb13 = crate::EnumBitfieldStruct<u8, Ansb13_SPEC>;
    impl Ansb13 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb14_SPEC;
    pub type Ansb14 = crate::EnumBitfieldStruct<u8, Ansb14_SPEC>;
    impl Ansb14 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb15_SPEC;
    pub type Ansb15 = crate::EnumBitfieldStruct<u8, Ansb15_SPEC>;
    impl Ansb15 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
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
    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
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

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb25(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adansb1::Ansb25,
        adansb1::Ansb25,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adansb1::Ansb25,
            adansb1::Ansb25,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb26(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adansb1::Ansb26,
        adansb1::Ansb26,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adansb1::Ansb26,
            adansb1::Ansb26,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb27(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adansb1::Ansb27,
        adansb1::Ansb27,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adansb1::Ansb27,
            adansb1::Ansb27,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb28(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adansb1::Ansb28,
        adansb1::Ansb28,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adansb1::Ansb28,
            adansb1::Ansb28,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb29(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adansb1::Ansb29,
        adansb1::Ansb29,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adansb1::Ansb29,
            adansb1::Ansb29,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb30(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adansb1::Ansb30,
        adansb1::Ansb30,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adansb1::Ansb30,
            adansb1::Ansb30,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansb31(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adansb1::Ansb31,
        adansb1::Ansb31,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adansb1::Ansb31,
            adansb1::Ansb31,
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
    pub struct Ansb16_SPEC;
    pub type Ansb16 = crate::EnumBitfieldStruct<u8, Ansb16_SPEC>;
    impl Ansb16 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb17_SPEC;
    pub type Ansb17 = crate::EnumBitfieldStruct<u8, Ansb17_SPEC>;
    impl Ansb17 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb18_SPEC;
    pub type Ansb18 = crate::EnumBitfieldStruct<u8, Ansb18_SPEC>;
    impl Ansb18 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb19_SPEC;
    pub type Ansb19 = crate::EnumBitfieldStruct<u8, Ansb19_SPEC>;
    impl Ansb19 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb20_SPEC;
    pub type Ansb20 = crate::EnumBitfieldStruct<u8, Ansb20_SPEC>;
    impl Ansb20 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb21_SPEC;
    pub type Ansb21 = crate::EnumBitfieldStruct<u8, Ansb21_SPEC>;
    impl Ansb21 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb22_SPEC;
    pub type Ansb22 = crate::EnumBitfieldStruct<u8, Ansb22_SPEC>;
    impl Ansb22 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb23_SPEC;
    pub type Ansb23 = crate::EnumBitfieldStruct<u8, Ansb23_SPEC>;
    impl Ansb23 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb24_SPEC;
    pub type Ansb24 = crate::EnumBitfieldStruct<u8, Ansb24_SPEC>;
    impl Ansb24 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb25_SPEC;
    pub type Ansb25 = crate::EnumBitfieldStruct<u8, Ansb25_SPEC>;
    impl Ansb25 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb26_SPEC;
    pub type Ansb26 = crate::EnumBitfieldStruct<u8, Ansb26_SPEC>;
    impl Ansb26 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb27_SPEC;
    pub type Ansb27 = crate::EnumBitfieldStruct<u8, Ansb27_SPEC>;
    impl Ansb27 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb28_SPEC;
    pub type Ansb28 = crate::EnumBitfieldStruct<u8, Ansb28_SPEC>;
    impl Ansb28 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb29_SPEC;
    pub type Ansb29 = crate::EnumBitfieldStruct<u8, Ansb29_SPEC>;
    impl Ansb29 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb30_SPEC;
    pub type Ansb30 = crate::EnumBitfieldStruct<u8, Ansb30_SPEC>;
    impl Ansb30 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb31_SPEC;
    pub type Ansb31 = crate::EnumBitfieldStruct<u8, Ansb31_SPEC>;
    impl Ansb31 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
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
    #[doc = "Converted Value 15 to 0"]
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
    #[doc = "Converted Value 15 to 0"]
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
    #[doc = "Converted Value 15 to 0"]
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
pub struct Adrd_SPEC;
impl crate::sealed::RegSpec for Adrd_SPEC {
    type DataType = u16;
}

#[doc = "A/D Self-Diagnosis Data Register"]
pub type Adrd = crate::RegValueT<Adrd_SPEC>;

impl Adrd {
    #[doc = "Converted Value 11 to 0"]
    #[inline(always)]
    pub fn ad(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Adrd_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Adrd_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Self-Diagnosis Status"]
    #[inline(always)]
    pub fn diagst(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        adrd::Diagst,
        adrd::Diagst,
        Adrd_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            adrd::Diagst,
            adrd::Diagst,
            Adrd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adrd {
    #[inline(always)]
    fn default() -> Adrd {
        <crate::RegValueT<Adrd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adrd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diagst_SPEC;
    pub type Diagst = crate::EnumBitfieldStruct<u8, Diagst_SPEC>;
    impl Diagst {
        #[doc = "Self-diagnosis not executed after power-on."]
        pub const _00: Self = Self::new(0);

        #[doc = "Self-diagnosis was executed using the 0 V voltage."]
        pub const _01: Self = Self::new(1);

        #[doc = "Self-diagnosis was executed using the reference voltage × 1/2."]
        pub const _10: Self = Self::new(2);

        #[doc = "Self-diagnosis was executed using the reference voltage ."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adctdr_SPEC;
impl crate::sealed::RegSpec for Adctdr_SPEC {
    type DataType = u16;
}

#[doc = "A/D CTSU TSCAP Voltage Data Register"]
pub type Adctdr = crate::RegValueT<Adctdr_SPEC>;

impl Adctdr {
    #[doc = "Converted Value 15 to 0"]
    #[inline(always)]
    pub fn adctdr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adctdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adctdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Adctdr {
    #[inline(always)]
    fn default() -> Adctdr {
        <crate::RegValueT<Adctdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addr_SPEC;
impl crate::sealed::RegSpec for Addr_SPEC {
    type DataType = u16;
}

#[doc = "A/D Data Registers %s"]
pub type Addr = crate::RegValueT<Addr_SPEC>;

impl Addr {
    #[doc = "Converted Value 15 to 0"]
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
    #[doc = "Disconnection Detection Assist Setting"]
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

    #[doc = "Precharge/discharge select"]
    #[inline(always)]
    pub fn pchg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        addiscr::Pchg,
        addiscr::Pchg,
        Addiscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            addiscr::Pchg,
            addiscr::Pchg,
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
    pub struct Adndis_SPEC;
    pub type Adndis = crate::EnumBitfieldStruct<u8, Adndis_SPEC>;
    impl Adndis {
        #[doc = "The disconnection detection assist function is disabled"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "The number of states for the discharge or precharge period."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pchg_SPEC;
    pub type Pchg = crate::EnumBitfieldStruct<u8, Pchg_SPEC>;
    impl Pchg {
        #[doc = "Discharge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Precharge"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adacsr_SPEC;
impl crate::sealed::RegSpec for Adacsr_SPEC {
    type DataType = u8;
}

#[doc = "A/D Conversion Operation Mode Select Register"]
pub type Adacsr = crate::RegValueT<Adacsr_SPEC>;

impl Adacsr {
    #[doc = "Successive Approximation Control Setting"]
    #[inline(always)]
    pub fn adsac(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adacsr::Adsac,
        adacsr::Adsac,
        Adacsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adacsr::Adsac,
            adacsr::Adsac,
            Adacsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adacsr {
    #[inline(always)]
    fn default() -> Adacsr {
        <crate::RegValueT<Adacsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adacsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsac_SPEC;
    pub type Adsac = crate::EnumBitfieldStruct<u8, Adsac_SPEC>;
    impl Adsac {
        #[doc = "Normal conversion mode (default)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Fast conversion mode"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Group Priority Operation Setting"]
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

    #[doc = "Lower-Priority Group Restart Setting"]
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

    #[doc = "Single Scan Continuous Start"]
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
}
impl ::core::default::Default for Adgspcr {
    #[inline(always)]
    fn default() -> Adgspcr {
        <crate::RegValueT<Adgspcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adgspcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgs_SPEC;
    pub type Pgs = crate::EnumBitfieldStruct<u8, Pgs_SPEC>;
    impl Pgs {
        #[doc = "Operate without group priority control."]
        pub const _0: Self = Self::new(0);

        #[doc = "Operate with group priority control."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gbrscn_SPEC;
    pub type Gbrscn = crate::EnumBitfieldStruct<u8, Gbrscn_SPEC>;
    impl Gbrscn {
        #[doc = "Disable rescanning of the group that was stopped in group priority operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable rescanning of the group that was stopped in group priority operation."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gbrp_SPEC;
    pub type Gbrp = crate::EnumBitfieldStruct<u8, Gbrp_SPEC>;
    impl Gbrp {
        #[doc = "Single scan is not continuously activated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Single scan for the group with the lower-priority is continuously activated."]
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
    #[doc = "Converted Value 15 to 0"]
    #[inline(always)]
    pub fn addbldr(
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
    #[doc = "Converted Value 15 to 0"]
    #[inline(always)]
    pub fn addbldr(
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
pub struct Adhvrefcnt_SPEC;
impl crate::sealed::RegSpec for Adhvrefcnt_SPEC {
    type DataType = u8;
}

#[doc = "A/D High-Potential/Low-Potential Reference Voltage Control Register"]
pub type Adhvrefcnt = crate::RegValueT<Adhvrefcnt_SPEC>;

impl Adhvrefcnt {
    #[doc = "High-Potential Reference Voltage Select"]
    #[inline(always)]
    pub fn hvsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adhvrefcnt::Hvsel,
        adhvrefcnt::Hvsel,
        Adhvrefcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adhvrefcnt::Hvsel,
            adhvrefcnt::Hvsel,
            Adhvrefcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Low-Potential Reference Voltage Select"]
    #[inline(always)]
    pub fn lvsel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adhvrefcnt::Lvsel,
        adhvrefcnt::Lvsel,
        Adhvrefcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adhvrefcnt::Lvsel,
            adhvrefcnt::Lvsel,
            Adhvrefcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sleep"]
    #[inline(always)]
    pub fn adslp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adhvrefcnt::Adslp,
        adhvrefcnt::Adslp,
        Adhvrefcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adhvrefcnt::Adslp,
            adhvrefcnt::Adslp,
            Adhvrefcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adhvrefcnt {
    #[inline(always)]
    fn default() -> Adhvrefcnt {
        <crate::RegValueT<Adhvrefcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adhvrefcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hvsel_SPEC;
    pub type Hvsel = crate::EnumBitfieldStruct<u8, Hvsel_SPEC>;
    impl Hvsel {
        #[doc = "AVCC0 is selected as the high-potential reference voltage"]
        pub const _00: Self = Self::new(0);

        #[doc = "VREFH0 is selected as the high-potential reference voltage"]
        pub const _01: Self = Self::new(1);

        #[doc = "Internal reference voltage is selected as the high-potential reference voltage"]
        pub const _10: Self = Self::new(2);

        #[doc = "No reference voltage pin is selected (internal node discharge)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvsel_SPEC;
    pub type Lvsel = crate::EnumBitfieldStruct<u8, Lvsel_SPEC>;
    impl Lvsel {
        #[doc = "AVSS0 is selected as the low-potential reference voltage."]
        pub const _0: Self = Self::new(0);

        #[doc = "VREFL0 is selected as the low-potential reference voltage."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adslp_SPEC;
    pub type Adslp = crate::EnumBitfieldStruct<u8, Adslp_SPEC>;
    impl Adslp {
        #[doc = "Normal operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Standby state"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Combination Result Monitor"]
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
}
impl ::core::default::Default for Adwinmon {
    #[inline(always)]
    fn default() -> Adwinmon {
        <crate::RegValueT<Adwinmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adwinmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moncomb_SPEC;
    pub type Moncomb = crate::EnumBitfieldStruct<u8, Moncomb_SPEC>;
    impl Moncomb {
        #[doc = "Window A/B composite conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Window A/B composite conditions are met."]
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
    pub struct Moncmpb_SPEC;
    pub type Moncmpb = crate::EnumBitfieldStruct<u8, Moncmpb_SPEC>;
    impl Moncmpb {
        #[doc = "Window B comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Window B comparison conditions are met."]
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
    #[doc = "Window A/B Composite Conditions Setting"]
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
}
impl ::core::default::Default for Adcmpcr {
    #[inline(always)]
    fn default() -> Adcmpcr {
        <crate::RegValueT<Adcmpcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpab_SPEC;
    pub type Cmpab = crate::EnumBitfieldStruct<u8, Cmpab_SPEC>;
    impl Cmpab {
        #[doc = "Output ADC120_WCMPM when window A OR window B comparison conditions are met. Otherwise, output ADC120_WCMPUM."]
        pub const _00: Self = Self::new(0);

        #[doc = "Output ADC120_WCMPM when window A EXOR window B comparison conditions are met. Otherwise, output ADC120_WCMPUM."]
        pub const _01: Self = Self::new(1);

        #[doc = "Output ADC120_WCMPM when window A AND window B comparison conditions are met. Otherwise, output ADC120_WCMPUM."]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpbe_SPEC;
    pub type Cmpbe = crate::EnumBitfieldStruct<u8, Cmpbe_SPEC>;
    impl Cmpbe {
        #[doc = "Disable compare window B operation. Disable ADC120_WCMPM and ADC120_WCMPUM outputs."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare window B operation."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpae_SPEC;
    pub type Cmpae = crate::EnumBitfieldStruct<u8, Cmpae_SPEC>;
    impl Cmpae {
        #[doc = "Disable compare window A operation. Disable ADC120_WCMPM and ADC120_WCMPUM outputs."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare window A operation."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpbie_SPEC;
    pub type Cmpbie = crate::EnumBitfieldStruct<u8, Cmpbie_SPEC>;
    impl Cmpbie {
        #[doc = "Disable ADC120_CMPBI interrupt when comparison conditions (window B) are met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable ADC120_CMPBI interrupt when comparison conditions (window B) are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wcmpe_SPEC;
    pub type Wcmpe = crate::EnumBitfieldStruct<u8, Wcmpe_SPEC>;
    impl Wcmpe {
        #[doc = "Disable window function Window A and window B operate as a comparator to compare the single value on the lower side with the A/D conversion result."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable window function Window A and window B operate as a comparator to compare the two values on the upper and lower sides with the A/D conversion result."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpaie_SPEC;
    pub type Cmpaie = crate::EnumBitfieldStruct<u8, Cmpaie_SPEC>;
    impl Cmpaie {
        #[doc = "Disable ADC120_CMPAI interrupt when comparison conditions (window A) are met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable ADC120_CMPAI interrupt when comparison conditions (window A) are met."]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Temperature Sensor Output Compare Select"]
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

    #[doc = "Internal Reference Voltage Compare Select"]
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
}
impl ::core::default::Default for Adcmpanser {
    #[inline(always)]
    fn default() -> Adcmpanser {
        <crate::RegValueT<Adcmpanser_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpanser {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptsa_SPEC;
    pub type Cmptsa = crate::EnumBitfieldStruct<u8, Cmptsa_SPEC>;
    impl Cmptsa {
        #[doc = "Exclude the temperature sensor output from the compare Window A target range."]
        pub const _0: Self = Self::new(0);

        #[doc = "Include the temperature sensor output in the compare Window A target range."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpoca_SPEC;
    pub type Cmpoca = crate::EnumBitfieldStruct<u8, Cmpoca_SPEC>;
    impl Cmpoca {
        #[doc = "Exclude the internal reference voltage from the compare Window A target range."]
        pub const _0: Self = Self::new(0);

        #[doc = "Include the internal reference voltage in the compare Window A target range."]
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

    #[doc = "Compare Window A Internal Reference Voltage Comparison Condition Select"]
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
}
impl ::core::default::Default for Adcmpler {
    #[inline(always)]
    fn default() -> Adcmpler {
        <crate::RegValueT<Adcmpler_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpler {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpltsa_SPEC;
    pub type Cmpltsa = crate::EnumBitfieldStruct<u8, Cmpltsa_SPEC>;
    impl Cmpltsa {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value > A/D-converted valueCompare Window A Temperature Sensor Output Comparison Condition Select When window function is enabled (ADCMPCR.WCMPE = 1) : Compare Window A Temperature Sensor Output Comparison ConditionA/D-converted value < ADCMPDR0 value, or A/D-converted value > ADCMPDR1 value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1) : ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmploca_SPEC;
    pub type Cmploca = crate::EnumBitfieldStruct<u8, Cmploca_SPEC>;
    impl Cmploca {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0) : ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or A/D-converted value > ADCMPDR1 value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
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
    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpcha09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha09,
        adcmpansr0::Cmpcha09,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha09,
            adcmpansr0::Cmpcha09,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpcha10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha10,
        adcmpansr0::Cmpcha10,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha10,
            adcmpansr0::Cmpcha10,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpcha11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha11,
        adcmpansr0::Cmpcha11,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha11,
            adcmpansr0::Cmpcha11,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpcha12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha12,
        adcmpansr0::Cmpcha12,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha12,
            adcmpansr0::Cmpcha12,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpcha13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha13,
        adcmpansr0::Cmpcha13,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha13,
            adcmpansr0::Cmpcha13,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpcha14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha14,
        adcmpansr0::Cmpcha14,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha14,
            adcmpansr0::Cmpcha14,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpcha15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha15,
        adcmpansr0::Cmpcha15,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha15,
            adcmpansr0::Cmpcha15,
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
    pub struct Cmpcha00_SPEC;
    pub type Cmpcha00 = crate::EnumBitfieldStruct<u8, Cmpcha00_SPEC>;
    impl Cmpcha00 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha01_SPEC;
    pub type Cmpcha01 = crate::EnumBitfieldStruct<u8, Cmpcha01_SPEC>;
    impl Cmpcha01 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha02_SPEC;
    pub type Cmpcha02 = crate::EnumBitfieldStruct<u8, Cmpcha02_SPEC>;
    impl Cmpcha02 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha03_SPEC;
    pub type Cmpcha03 = crate::EnumBitfieldStruct<u8, Cmpcha03_SPEC>;
    impl Cmpcha03 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha04_SPEC;
    pub type Cmpcha04 = crate::EnumBitfieldStruct<u8, Cmpcha04_SPEC>;
    impl Cmpcha04 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha05_SPEC;
    pub type Cmpcha05 = crate::EnumBitfieldStruct<u8, Cmpcha05_SPEC>;
    impl Cmpcha05 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha06_SPEC;
    pub type Cmpcha06 = crate::EnumBitfieldStruct<u8, Cmpcha06_SPEC>;
    impl Cmpcha06 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha07_SPEC;
    pub type Cmpcha07 = crate::EnumBitfieldStruct<u8, Cmpcha07_SPEC>;
    impl Cmpcha07 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha08_SPEC;
    pub type Cmpcha08 = crate::EnumBitfieldStruct<u8, Cmpcha08_SPEC>;
    impl Cmpcha08 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha09_SPEC;
    pub type Cmpcha09 = crate::EnumBitfieldStruct<u8, Cmpcha09_SPEC>;
    impl Cmpcha09 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha10_SPEC;
    pub type Cmpcha10 = crate::EnumBitfieldStruct<u8, Cmpcha10_SPEC>;
    impl Cmpcha10 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha11_SPEC;
    pub type Cmpcha11 = crate::EnumBitfieldStruct<u8, Cmpcha11_SPEC>;
    impl Cmpcha11 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha12_SPEC;
    pub type Cmpcha12 = crate::EnumBitfieldStruct<u8, Cmpcha12_SPEC>;
    impl Cmpcha12 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha13_SPEC;
    pub type Cmpcha13 = crate::EnumBitfieldStruct<u8, Cmpcha13_SPEC>;
    impl Cmpcha13 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha14_SPEC;
    pub type Cmpcha14 = crate::EnumBitfieldStruct<u8, Cmpcha14_SPEC>;
    impl Cmpcha14 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha15_SPEC;
    pub type Cmpcha15 = crate::EnumBitfieldStruct<u8, Cmpcha15_SPEC>;
    impl Cmpcha15 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
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
    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
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

    #[doc = "Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpcha25(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha25,
        adcmpansr1::Cmpcha25,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha25,
            adcmpansr1::Cmpcha25,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpcha26(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha26,
        adcmpansr1::Cmpcha26,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha26,
            adcmpansr1::Cmpcha26,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpcha27(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha27,
        adcmpansr1::Cmpcha27,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha27,
            adcmpansr1::Cmpcha27,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpcha28(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha28,
        adcmpansr1::Cmpcha28,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha28,
            adcmpansr1::Cmpcha28,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpcha29(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha29,
        adcmpansr1::Cmpcha29,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha29,
            adcmpansr1::Cmpcha29,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpcha30(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha30,
        adcmpansr1::Cmpcha30,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha30,
            adcmpansr1::Cmpcha30,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpcha31(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha31,
        adcmpansr1::Cmpcha31,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha31,
            adcmpansr1::Cmpcha31,
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
    pub struct Cmpcha16_SPEC;
    pub type Cmpcha16 = crate::EnumBitfieldStruct<u8, Cmpcha16_SPEC>;
    impl Cmpcha16 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha17_SPEC;
    pub type Cmpcha17 = crate::EnumBitfieldStruct<u8, Cmpcha17_SPEC>;
    impl Cmpcha17 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha18_SPEC;
    pub type Cmpcha18 = crate::EnumBitfieldStruct<u8, Cmpcha18_SPEC>;
    impl Cmpcha18 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha19_SPEC;
    pub type Cmpcha19 = crate::EnumBitfieldStruct<u8, Cmpcha19_SPEC>;
    impl Cmpcha19 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha20_SPEC;
    pub type Cmpcha20 = crate::EnumBitfieldStruct<u8, Cmpcha20_SPEC>;
    impl Cmpcha20 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha21_SPEC;
    pub type Cmpcha21 = crate::EnumBitfieldStruct<u8, Cmpcha21_SPEC>;
    impl Cmpcha21 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha22_SPEC;
    pub type Cmpcha22 = crate::EnumBitfieldStruct<u8, Cmpcha22_SPEC>;
    impl Cmpcha22 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha23_SPEC;
    pub type Cmpcha23 = crate::EnumBitfieldStruct<u8, Cmpcha23_SPEC>;
    impl Cmpcha23 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha24_SPEC;
    pub type Cmpcha24 = crate::EnumBitfieldStruct<u8, Cmpcha24_SPEC>;
    impl Cmpcha24 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha25_SPEC;
    pub type Cmpcha25 = crate::EnumBitfieldStruct<u8, Cmpcha25_SPEC>;
    impl Cmpcha25 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha26_SPEC;
    pub type Cmpcha26 = crate::EnumBitfieldStruct<u8, Cmpcha26_SPEC>;
    impl Cmpcha26 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha27_SPEC;
    pub type Cmpcha27 = crate::EnumBitfieldStruct<u8, Cmpcha27_SPEC>;
    impl Cmpcha27 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha28_SPEC;
    pub type Cmpcha28 = crate::EnumBitfieldStruct<u8, Cmpcha28_SPEC>;
    impl Cmpcha28 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha29_SPEC;
    pub type Cmpcha29 = crate::EnumBitfieldStruct<u8, Cmpcha29_SPEC>;
    impl Cmpcha29 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha30_SPEC;
    pub type Cmpcha30 = crate::EnumBitfieldStruct<u8, Cmpcha30_SPEC>;
    impl Cmpcha30 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha31_SPEC;
    pub type Cmpcha31 = crate::EnumBitfieldStruct<u8, Cmpcha31_SPEC>;
    impl Cmpcha31 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
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
    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplcha09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha09,
        adcmplr0::Cmplcha09,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha09,
            adcmplr0::Cmplcha09,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplcha10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha10,
        adcmplr0::Cmplcha10,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha10,
            adcmplr0::Cmplcha10,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplcha11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha11,
        adcmplr0::Cmplcha11,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha11,
            adcmplr0::Cmplcha11,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplcha12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha12,
        adcmplr0::Cmplcha12,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha12,
            adcmplr0::Cmplcha12,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplcha13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha13,
        adcmplr0::Cmplcha13,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha13,
            adcmplr0::Cmplcha13,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplcha14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha14,
        adcmplr0::Cmplcha14,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha14,
            adcmplr0::Cmplcha14,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplcha15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha15,
        adcmplr0::Cmplcha15,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha15,
            adcmplr0::Cmplcha15,
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
    pub struct Cmplcha00_SPEC;
    pub type Cmplcha00 = crate::EnumBitfieldStruct<u8, Cmplcha00_SPEC>;
    impl Cmplcha00 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha01_SPEC;
    pub type Cmplcha01 = crate::EnumBitfieldStruct<u8, Cmplcha01_SPEC>;
    impl Cmplcha01 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha02_SPEC;
    pub type Cmplcha02 = crate::EnumBitfieldStruct<u8, Cmplcha02_SPEC>;
    impl Cmplcha02 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha03_SPEC;
    pub type Cmplcha03 = crate::EnumBitfieldStruct<u8, Cmplcha03_SPEC>;
    impl Cmplcha03 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha04_SPEC;
    pub type Cmplcha04 = crate::EnumBitfieldStruct<u8, Cmplcha04_SPEC>;
    impl Cmplcha04 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha05_SPEC;
    pub type Cmplcha05 = crate::EnumBitfieldStruct<u8, Cmplcha05_SPEC>;
    impl Cmplcha05 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha06_SPEC;
    pub type Cmplcha06 = crate::EnumBitfieldStruct<u8, Cmplcha06_SPEC>;
    impl Cmplcha06 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha07_SPEC;
    pub type Cmplcha07 = crate::EnumBitfieldStruct<u8, Cmplcha07_SPEC>;
    impl Cmplcha07 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha08_SPEC;
    pub type Cmplcha08 = crate::EnumBitfieldStruct<u8, Cmplcha08_SPEC>;
    impl Cmplcha08 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha09_SPEC;
    pub type Cmplcha09 = crate::EnumBitfieldStruct<u8, Cmplcha09_SPEC>;
    impl Cmplcha09 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha10_SPEC;
    pub type Cmplcha10 = crate::EnumBitfieldStruct<u8, Cmplcha10_SPEC>;
    impl Cmplcha10 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha11_SPEC;
    pub type Cmplcha11 = crate::EnumBitfieldStruct<u8, Cmplcha11_SPEC>;
    impl Cmplcha11 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha12_SPEC;
    pub type Cmplcha12 = crate::EnumBitfieldStruct<u8, Cmplcha12_SPEC>;
    impl Cmplcha12 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha13_SPEC;
    pub type Cmplcha13 = crate::EnumBitfieldStruct<u8, Cmplcha13_SPEC>;
    impl Cmplcha13 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha14_SPEC;
    pub type Cmplcha14 = crate::EnumBitfieldStruct<u8, Cmplcha14_SPEC>;
    impl Cmplcha14 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha15_SPEC;
    pub type Cmplcha15 = crate::EnumBitfieldStruct<u8, Cmplcha15_SPEC>;
    impl Cmplcha15 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
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
    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
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

    #[doc = "Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplcha25(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha25,
        adcmplr1::Cmplcha25,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha25,
            adcmplr1::Cmplcha25,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplcha26(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha26,
        adcmplr1::Cmplcha26,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha26,
            adcmplr1::Cmplcha26,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplcha27(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha27,
        adcmplr1::Cmplcha27,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha27,
            adcmplr1::Cmplcha27,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplcha28(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha28,
        adcmplr1::Cmplcha28,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha28,
            adcmplr1::Cmplcha28,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplcha29(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha29,
        adcmplr1::Cmplcha29,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha29,
            adcmplr1::Cmplcha29,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplcha30(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha30,
        adcmplr1::Cmplcha30,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha30,
            adcmplr1::Cmplcha30,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select"]
    #[inline(always)]
    pub fn cmplcha31(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha31,
        adcmplr1::Cmplcha31,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha31,
            adcmplr1::Cmplcha31,
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
    pub struct Cmplcha16_SPEC;
    pub type Cmplcha16 = crate::EnumBitfieldStruct<u8, Cmplcha16_SPEC>;
    impl Cmplcha16 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha17_SPEC;
    pub type Cmplcha17 = crate::EnumBitfieldStruct<u8, Cmplcha17_SPEC>;
    impl Cmplcha17 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha18_SPEC;
    pub type Cmplcha18 = crate::EnumBitfieldStruct<u8, Cmplcha18_SPEC>;
    impl Cmplcha18 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha19_SPEC;
    pub type Cmplcha19 = crate::EnumBitfieldStruct<u8, Cmplcha19_SPEC>;
    impl Cmplcha19 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha20_SPEC;
    pub type Cmplcha20 = crate::EnumBitfieldStruct<u8, Cmplcha20_SPEC>;
    impl Cmplcha20 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha21_SPEC;
    pub type Cmplcha21 = crate::EnumBitfieldStruct<u8, Cmplcha21_SPEC>;
    impl Cmplcha21 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha22_SPEC;
    pub type Cmplcha22 = crate::EnumBitfieldStruct<u8, Cmplcha22_SPEC>;
    impl Cmplcha22 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha23_SPEC;
    pub type Cmplcha23 = crate::EnumBitfieldStruct<u8, Cmplcha23_SPEC>;
    impl Cmplcha23 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha24_SPEC;
    pub type Cmplcha24 = crate::EnumBitfieldStruct<u8, Cmplcha24_SPEC>;
    impl Cmplcha24 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha25_SPEC;
    pub type Cmplcha25 = crate::EnumBitfieldStruct<u8, Cmplcha25_SPEC>;
    impl Cmplcha25 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha26_SPEC;
    pub type Cmplcha26 = crate::EnumBitfieldStruct<u8, Cmplcha26_SPEC>;
    impl Cmplcha26 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha27_SPEC;
    pub type Cmplcha27 = crate::EnumBitfieldStruct<u8, Cmplcha27_SPEC>;
    impl Cmplcha27 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha28_SPEC;
    pub type Cmplcha28 = crate::EnumBitfieldStruct<u8, Cmplcha28_SPEC>;
    impl Cmplcha28 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha29_SPEC;
    pub type Cmplcha29 = crate::EnumBitfieldStruct<u8, Cmplcha29_SPEC>;
    impl Cmplcha29 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha30_SPEC;
    pub type Cmplcha30 = crate::EnumBitfieldStruct<u8, Cmplcha30_SPEC>;
    impl Cmplcha30 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha31_SPEC;
    pub type Cmplcha31 = crate::EnumBitfieldStruct<u8, Cmplcha31_SPEC>;
    impl Cmplcha31 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpdr_SPEC;
impl crate::sealed::RegSpec for Adcmpdr_SPEC {
    type DataType = u16;
}

#[doc = "A/D Compare Function Window A Lower-Side/Upper-Side Level Setting Register"]
pub type Adcmpdr = crate::RegValueT<Adcmpdr_SPEC>;

impl NoBitfieldReg<Adcmpdr_SPEC> for Adcmpdr {}
impl ::core::default::Default for Adcmpdr {
    #[inline(always)]
    fn default() -> Adcmpdr {
        <crate::RegValueT<Adcmpdr_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha09,
        adcmpsr0::Cmpstcha09,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha09,
            adcmpsr0::Cmpstcha09,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha10,
        adcmpsr0::Cmpstcha10,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha10,
            adcmpsr0::Cmpstcha10,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha11,
        adcmpsr0::Cmpstcha11,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha11,
            adcmpsr0::Cmpstcha11,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha12,
        adcmpsr0::Cmpstcha12,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha12,
            adcmpsr0::Cmpstcha12,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha13,
        adcmpsr0::Cmpstcha13,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha13,
            adcmpsr0::Cmpstcha13,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha14,
        adcmpsr0::Cmpstcha14,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha14,
            adcmpsr0::Cmpstcha14,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha15,
        adcmpsr0::Cmpstcha15,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha15,
            adcmpsr0::Cmpstcha15,
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
    pub struct Cmpstcha00_SPEC;
    pub type Cmpstcha00 = crate::EnumBitfieldStruct<u8, Cmpstcha00_SPEC>;
    impl Cmpstcha00 {
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
    pub struct Cmpstcha02_SPEC;
    pub type Cmpstcha02 = crate::EnumBitfieldStruct<u8, Cmpstcha02_SPEC>;
    impl Cmpstcha02 {
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
    pub struct Cmpstcha04_SPEC;
    pub type Cmpstcha04 = crate::EnumBitfieldStruct<u8, Cmpstcha04_SPEC>;
    impl Cmpstcha04 {
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
    pub struct Cmpstcha06_SPEC;
    pub type Cmpstcha06 = crate::EnumBitfieldStruct<u8, Cmpstcha06_SPEC>;
    impl Cmpstcha06 {
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
    pub struct Cmpstcha08_SPEC;
    pub type Cmpstcha08 = crate::EnumBitfieldStruct<u8, Cmpstcha08_SPEC>;
    impl Cmpstcha08 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha09_SPEC;
    pub type Cmpstcha09 = crate::EnumBitfieldStruct<u8, Cmpstcha09_SPEC>;
    impl Cmpstcha09 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha10_SPEC;
    pub type Cmpstcha10 = crate::EnumBitfieldStruct<u8, Cmpstcha10_SPEC>;
    impl Cmpstcha10 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha11_SPEC;
    pub type Cmpstcha11 = crate::EnumBitfieldStruct<u8, Cmpstcha11_SPEC>;
    impl Cmpstcha11 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha12_SPEC;
    pub type Cmpstcha12 = crate::EnumBitfieldStruct<u8, Cmpstcha12_SPEC>;
    impl Cmpstcha12 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha13_SPEC;
    pub type Cmpstcha13 = crate::EnumBitfieldStruct<u8, Cmpstcha13_SPEC>;
    impl Cmpstcha13 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha14_SPEC;
    pub type Cmpstcha14 = crate::EnumBitfieldStruct<u8, Cmpstcha14_SPEC>;
    impl Cmpstcha14 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha15_SPEC;
    pub type Cmpstcha15 = crate::EnumBitfieldStruct<u8, Cmpstcha15_SPEC>;
    impl Cmpstcha15 {
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

#[doc = "A/D Compare Function Window A Channel Status Register1"]
pub type Adcmpsr1 = crate::RegValueT<Adcmpsr1_SPEC>;

impl Adcmpsr1 {
    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
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

    #[doc = "Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha25(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha25,
        adcmpsr1::Cmpstcha25,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha25,
            adcmpsr1::Cmpstcha25,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha26(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha26,
        adcmpsr1::Cmpstcha26,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha26,
            adcmpsr1::Cmpstcha26,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha27(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha27,
        adcmpsr1::Cmpstcha27,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha27,
            adcmpsr1::Cmpstcha27,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha28(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha28,
        adcmpsr1::Cmpstcha28,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha28,
            adcmpsr1::Cmpstcha28,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha29(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha29,
        adcmpsr1::Cmpstcha29,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha29,
            adcmpsr1::Cmpstcha29,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha30(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha30,
        adcmpsr1::Cmpstcha30,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha30,
            adcmpsr1::Cmpstcha30,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstcha31(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha31,
        adcmpsr1::Cmpstcha31,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha31,
            adcmpsr1::Cmpstcha31,
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
    pub struct Cmpstcha16_SPEC;
    pub type Cmpstcha16 = crate::EnumBitfieldStruct<u8, Cmpstcha16_SPEC>;
    impl Cmpstcha16 {
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
    pub struct Cmpstcha18_SPEC;
    pub type Cmpstcha18 = crate::EnumBitfieldStruct<u8, Cmpstcha18_SPEC>;
    impl Cmpstcha18 {
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
    pub struct Cmpstcha20_SPEC;
    pub type Cmpstcha20 = crate::EnumBitfieldStruct<u8, Cmpstcha20_SPEC>;
    impl Cmpstcha20 {
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
    pub struct Cmpstcha22_SPEC;
    pub type Cmpstcha22 = crate::EnumBitfieldStruct<u8, Cmpstcha22_SPEC>;
    impl Cmpstcha22 {
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
    pub struct Cmpstcha24_SPEC;
    pub type Cmpstcha24 = crate::EnumBitfieldStruct<u8, Cmpstcha24_SPEC>;
    impl Cmpstcha24 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha25_SPEC;
    pub type Cmpstcha25 = crate::EnumBitfieldStruct<u8, Cmpstcha25_SPEC>;
    impl Cmpstcha25 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha26_SPEC;
    pub type Cmpstcha26 = crate::EnumBitfieldStruct<u8, Cmpstcha26_SPEC>;
    impl Cmpstcha26 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha27_SPEC;
    pub type Cmpstcha27 = crate::EnumBitfieldStruct<u8, Cmpstcha27_SPEC>;
    impl Cmpstcha27 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha28_SPEC;
    pub type Cmpstcha28 = crate::EnumBitfieldStruct<u8, Cmpstcha28_SPEC>;
    impl Cmpstcha28 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha29_SPEC;
    pub type Cmpstcha29 = crate::EnumBitfieldStruct<u8, Cmpstcha29_SPEC>;
    impl Cmpstcha29 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha30_SPEC;
    pub type Cmpstcha30 = crate::EnumBitfieldStruct<u8, Cmpstcha30_SPEC>;
    impl Cmpstcha30 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha31_SPEC;
    pub type Cmpstcha31 = crate::EnumBitfieldStruct<u8, Cmpstcha31_SPEC>;
    impl Cmpstcha31 {
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
    #[doc = "Compare Window A Temperature Sensor Output Compare Flag"]
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

    #[doc = "Compare Window A Internal Reference Voltage Compare Flag"]
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
}
impl ::core::default::Default for Adcmpser {
    #[inline(always)]
    fn default() -> Adcmpser {
        <crate::RegValueT<Adcmpser_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpser {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpsttsa_SPEC;
    pub type Cmpsttsa = crate::EnumBitfieldStruct<u8, Cmpsttsa_SPEC>;
    impl Cmpsttsa {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstoca_SPEC;
    pub type Cmpstoca = crate::EnumBitfieldStruct<u8, Cmpstoca_SPEC>;
    impl Cmpstoca {
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

#[doc = "A/D Compare Function Window B Channel Select Register"]
pub type Adcmpbnsr = crate::RegValueT<Adcmpbnsr_SPEC>;

impl Adcmpbnsr {
    #[doc = "Compare Window B Channel Select"]
    #[inline(always)]
    pub fn cmpchb(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Adcmpbnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Adcmpbnsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Compare Window B Comparison Condition Setting"]
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
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADWINLLB value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADWINLLB value, or ADWINULB value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADWINLLB value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADWINLLB value < A/D-converted value < ADWINULB value"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adwinllb_SPEC;
impl crate::sealed::RegSpec for Adwinllb_SPEC {
    type DataType = u16;
}

#[doc = "A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register"]
pub type Adwinllb = crate::RegValueT<Adwinllb_SPEC>;

impl NoBitfieldReg<Adwinllb_SPEC> for Adwinllb {}
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

#[doc = "A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register"]
pub type Adwinulb = crate::RegValueT<Adwinulb_SPEC>;

impl NoBitfieldReg<Adwinulb_SPEC> for Adwinulb {}
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
    #[doc = "Compare Window B Flag"]
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

#[doc = "A/D Sampling State Register"]
pub type Adsstrl = crate::RegValueT<Adsstrl_SPEC>;

impl Adsstrl {
    #[doc = "Sampling Time Setting"]
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

#[doc = "A/D Sampling State Register"]
pub type Adsstrt = crate::RegValueT<Adsstrt_SPEC>;

impl Adsstrt {
    #[doc = "Sampling Time Setting"]
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

#[doc = "A/D Sampling State Register"]
pub type Adsstro = crate::RegValueT<Adsstro_SPEC>;

impl Adsstro {
    #[doc = "Sampling Time Setting"]
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
pub struct Adsstr_SPEC;
impl crate::sealed::RegSpec for Adsstr_SPEC {
    type DataType = u8;
}

#[doc = "A/D Sampling State Register"]
pub type Adsstr = crate::RegValueT<Adsstr_SPEC>;

impl Adsstr {
    #[doc = "Sampling Time Setting"]
    #[inline(always)]
    pub fn sst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adsstr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adsstr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstr {
    #[inline(always)]
    fn default() -> Adsstr {
        <crate::RegValueT<Adsstr_SPEC> as RegisterValue<_>>::new(13)
    }
}
