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
// Generated from SVD 1.0, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:14:52 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"12bit A/D Converter 1"]
unsafe impl ::core::marker::Send for super::Adc121 {}
unsafe impl ::core::marker::Sync for super::Adc121 {}
impl super::Adc121 {
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

    #[doc = "A/D Data Duplication Register"]
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

    #[doc = "A/D Data Register %s"]
    #[inline(always)]
    pub const fn addr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Addr_SPEC, crate::common::R>,
        3,
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

    #[doc = "A/D Sample and Hold Circuit Control Register"]
    #[inline(always)]
    pub const fn adshcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adshcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adshcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(102usize),
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

    #[doc = "A/D Sample and Hold Operation Mode Select Register"]
    #[inline(always)]
    pub const fn adshmsr(
        &self,
    ) -> &'static crate::common::Reg<self::Adshmsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adshmsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
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

    #[doc = "A/D Data Duplication Register A"]
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

    #[doc = "A/D Data Duplication Register B"]
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

    #[doc = "A/D Sampling State Register %s (Corresponding Channel is AN10 )"]
    #[inline(always)]
    pub const fn adsstr0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adsstr0_SPEC, crate::common::RW>,
        3,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe5usize))
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

    #[doc = "A/D Programmable Gain Amplifier Control Register"]
    #[inline(always)]
    pub const fn adpgacr(
        &self,
    ) -> &'static crate::common::Reg<self::Adpgacr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adpgacr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(416usize),
            )
        }
    }

    #[doc = "A/D Programmable Gain Amplifier Gain Setting Register 0"]
    #[inline(always)]
    pub const fn adpgags0(
        &self,
    ) -> &'static crate::common::Reg<self::Adpgags0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adpgags0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(418usize),
            )
        }
    }

    #[doc = "A/D Programmable Gain Amplifier Differential Input Control Register"]
    #[inline(always)]
    pub const fn adpgadcr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adpgadcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adpgadcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(432usize),
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

    #[doc = "Double Trigger Channel Select\nThese bits select one analog input channel for double triggered operation. The setting is only effective while double trigger mode is selected."]
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

        #[doc = "A/D conversion is started by the asynchronous trigger (ADTRG1)."]
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
        #[doc = "Disables ADC12_GBADI1 interrupt generation upon group B scan completion."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables ADC12_GBADI1 interrupt generation upon group B scan completion."]
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
    #[doc = "AN107 Select"]
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

    #[doc = "AN106 Select"]
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

    #[doc = "AN105 Select"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Adansa0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Adansa0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AN102 Select"]
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

    #[doc = "AN101 Select"]
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

    #[doc = "AN100 Select"]
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
    pub struct Ansa07_SPEC;
    pub type Ansa07 = crate::EnumBitfieldStruct<u8, Ansa07_SPEC>;
    impl Ansa07 {
        #[doc = "AN107 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN107 is subjected to conversion."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa06_SPEC;
    pub type Ansa06 = crate::EnumBitfieldStruct<u8, Ansa06_SPEC>;
    impl Ansa06 {
        #[doc = "AN106 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN106 is subjected to conversion."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa05_SPEC;
    pub type Ansa05 = crate::EnumBitfieldStruct<u8, Ansa05_SPEC>;
    impl Ansa05 {
        #[doc = "AN105 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN105 is subjected to conversion."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa02_SPEC;
    pub type Ansa02 = crate::EnumBitfieldStruct<u8, Ansa02_SPEC>;
    impl Ansa02 {
        #[doc = "AN102 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN102 is subjected to conversion."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa01_SPEC;
    pub type Ansa01 = crate::EnumBitfieldStruct<u8, Ansa01_SPEC>;
    impl Ansa01 {
        #[doc = "AN101 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN101 is subjected to conversion."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa00_SPEC;
    pub type Ansa00 = crate::EnumBitfieldStruct<u8, Ansa00_SPEC>;
    impl Ansa00 {
        #[doc = "AN100 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN100 is subjected to conversion."]
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
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Adansa1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Adansa1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AN117 Select"]
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

    #[doc = "AN116 Select"]
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
    pub struct Ansa17_SPEC;
    pub type Ansa17 = crate::EnumBitfieldStruct<u8, Ansa17_SPEC>;
    impl Ansa17 {
        #[doc = "AN117 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN117 is subjected to conversion."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa16_SPEC;
    pub type Ansa16 = crate::EnumBitfieldStruct<u8, Ansa16_SPEC>;
    impl Ansa16 {
        #[doc = "AN116 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN116 is subjected to conversion."]
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
    #[doc = "A/D-Converted Value Addition/Average Channel AN107 Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel AN106 Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel AN105 Select"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Adads0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Adads0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "A/D-Converted Value Addition/Average Channel AN102 Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel AN101 Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel AN100 Select"]
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
    pub struct Ads07_SPEC;
    pub type Ads07 = crate::EnumBitfieldStruct<u8, Ads07_SPEC>;
    impl Ads07 {
        #[doc = "AN107 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN107 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads06_SPEC;
    pub type Ads06 = crate::EnumBitfieldStruct<u8, Ads06_SPEC>;
    impl Ads06 {
        #[doc = "AN106 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN106 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads05_SPEC;
    pub type Ads05 = crate::EnumBitfieldStruct<u8, Ads05_SPEC>;
    impl Ads05 {
        #[doc = "AN105 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN105 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads02_SPEC;
    pub type Ads02 = crate::EnumBitfieldStruct<u8, Ads02_SPEC>;
    impl Ads02 {
        #[doc = "AN102 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN102 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads01_SPEC;
    pub type Ads01 = crate::EnumBitfieldStruct<u8, Ads01_SPEC>;
    impl Ads01 {
        #[doc = "AN101 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN101 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads00_SPEC;
    pub type Ads00 = crate::EnumBitfieldStruct<u8, Ads00_SPEC>;
    impl Ads00 {
        #[doc = "AN100 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN100 is selected."]
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
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Adads1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Adads1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "A/D-Converted Value Addition/Average Channel AN117 Select"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel AN116 Select"]
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
    pub struct Ads17_SPEC;
    pub type Ads17 = crate::EnumBitfieldStruct<u8, Ads17_SPEC>;
    impl Ads17 {
        #[doc = "AN117 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN117 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads16_SPEC;
    pub type Ads16 = crate::EnumBitfieldStruct<u8, Ads16_SPEC>;
    impl Ads16 {
        #[doc = "AN116 is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN116 is selected."]
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
    #[doc = "Average mode enable bit.\nNote: The AVEE bit converts twice, and only when converting it four times, is effective. Please do not set (ADADC.AVEE=1) to conversion (ADADC.ADC 2:0=010b) three times when you select the average mode."]
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

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, Adadc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,Adadc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Addition frequency selection bit.\nNOTE: AVEE bit is valid at the only setting of ADC\\[2:0\\] bits = 001b or 011b. When average mode is selected by setting the ADADC.AVEE bit to 1, do not set the addition count to three times (ADADC.ADC\\[2:0\\] = 010b)"]
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
    pub struct Avee_SPEC;
    pub type Avee = crate::EnumBitfieldStruct<u8, Avee_SPEC>;
    impl Avee {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adc_SPEC;
    pub type Adc = crate::EnumBitfieldStruct<u8, Adc_SPEC>;
    impl Adc {
        #[doc = "1-time conversion (no addition; same as normal conversion)"]
        pub const _000: Self = Self::new(0);

        #[doc = "2-time conversion (addition once)"]
        pub const _001: Self = Self::new(1);

        #[doc = "3-time conversion (addition twice)"]
        pub const _010: Self = Self::new(2);

        #[doc = "4-time conversion (addition three times)"]
        pub const _011: Self = Self::new(3);

        #[doc = "16-time conversion (addition 15 times), can be set when selecting 12-bit accuracy."]
        pub const _101: Self = Self::new(5);
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

    #[doc = "A/D Conversion Accuracy Specify"]
    #[inline(always)]
    pub fn adprc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        adcer::Adprc,
        adcer::Adprc,
        Adcer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            adcer::Adprc,
            adcer::Adprc,
            Adcer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Adcer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Adcer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
    pub struct Adrfmt_SPEC;
    pub type Adrfmt = crate::EnumBitfieldStruct<u8, Adrfmt_SPEC>;
    impl Adrfmt {
        #[doc = "Flush-right is selected for the A/D data register format."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flush-left is selected for the A/D data register format."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diagm_SPEC;
    pub type Diagm = crate::EnumBitfieldStruct<u8, Diagm_SPEC>;
    impl Diagm {
        #[doc = "Disables self-diagnosis of ADC12."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables self-diagnosis of ADC12."]
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
        #[doc = "When the self-diagnosis fixation mode is selected, it set prohibits it."]
        pub const _00: Self = Self::new(0);

        #[doc = "The self-diagnosis by using the voltage of  0V."]
        pub const _01: Self = Self::new(1);

        #[doc = "The self-diagnosis by using the voltage of reference supply x 1/2."]
        pub const _10: Self = Self::new(2);

        #[doc = "The self-diagnosis by using the voltage of the reference supply."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ace_SPEC;
    pub type Ace = crate::EnumBitfieldStruct<u8, Ace_SPEC>;
    impl Ace {
        #[doc = "Disables automatic clearing."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adprc_SPEC;
    pub type Adprc = crate::EnumBitfieldStruct<u8, Adprc_SPEC>;
    impl Adprc {
        #[doc = "A/D conversion is performed with 12-bit accuracy."]
        pub const _00: Self = Self::new(0);

        #[doc = "A/D conversion is performed with 10-bit accuracy."]
        pub const _01: Self = Self::new(1);

        #[doc = "A/D conversion is performed with 8-bit accuracy."]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
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
    #[doc = "A/D Conversion Start Trigger Select\nSelect the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected."]
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

    #[doc = "A/D Conversion Start Trigger Select for Group B\nSelect the A/D conversion start trigger for group B in group scan mode."]
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
    #[doc = "Internal Reference Voltage A/D Conversion Select for Group B"]
    #[inline(always)]
    pub fn ocsb(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adexicr::Ocsb,
        adexicr::Ocsb,
        Adexicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adexicr::Ocsb,
            adexicr::Ocsb,
            Adexicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Temperature Sensor Output A/D Conversion Select for Group B"]
    #[inline(always)]
    pub fn tssb(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adexicr::Tssb,
        adexicr::Tssb,
        Adexicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adexicr::Tssb,
            adexicr::Tssb,
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

    #[doc = "Internal Reference Voltage A/D converted Value Addition/Average Mode Select"]
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

    #[doc = "Temperature Sensor Output A/D converted Value Addition/Average Mode Select"]
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
    pub struct Ocsb_SPEC;
    pub type Ocsb = crate::EnumBitfieldStruct<u8, Ocsb_SPEC>;
    impl Ocsb {
        #[doc = "The internal reference voltage is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "The internal reference voltage is selected for group B in group scan mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tssb_SPEC;
    pub type Tssb = crate::EnumBitfieldStruct<u8, Tssb_SPEC>;
    impl Tssb {
        #[doc = "The temperature sensor output is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "The temperature sensor output is not selected for group B in group scan mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ocsa_SPEC;
    pub type Ocsa = crate::EnumBitfieldStruct<u8, Ocsa_SPEC>;
    impl Ocsa {
        #[doc = "The internal reference voltage is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "The internal reference voltage is selected for group A in single scan mode, continuous scan mode, or group scan mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tssa_SPEC;
    pub type Tssa = crate::EnumBitfieldStruct<u8, Tssa_SPEC>;
    impl Tssa {
        #[doc = "The temperature sensor output is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "The temperature sensor output is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ocsad_SPEC;
    pub type Ocsad = crate::EnumBitfieldStruct<u8, Ocsad_SPEC>;
    impl Ocsad {
        #[doc = "Internal reference voltage A/D-converted value addition/average mode is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal reference voltage A/D-converted value addition/average mode is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tssad_SPEC;
    pub type Tssad = crate::EnumBitfieldStruct<u8, Tssad_SPEC>;
    impl Tssad {
        #[doc = "Temperature sensor output A/D-converted value addition/average mode is not selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Temperature sensor output A/D-converted value addition/average mode is selected."]
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
    #[doc = "AN107 Select"]
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

    #[doc = "AN106 Select"]
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

    #[doc = "AN105 Select"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Adansb0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Adansb0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AN102 Select"]
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

    #[doc = "AN101 Select"]
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

    #[doc = "AN100 Select"]
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
    pub struct Ansb07_SPEC;
    pub type Ansb07 = crate::EnumBitfieldStruct<u8, Ansb07_SPEC>;
    impl Ansb07 {
        #[doc = "AN107 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN107 is subjected to conversion."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb06_SPEC;
    pub type Ansb06 = crate::EnumBitfieldStruct<u8, Ansb06_SPEC>;
    impl Ansb06 {
        #[doc = "AN106 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN106 is subjected to conversion."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb05_SPEC;
    pub type Ansb05 = crate::EnumBitfieldStruct<u8, Ansb05_SPEC>;
    impl Ansb05 {
        #[doc = "AN105 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN105 is subjected to conversion."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb02_SPEC;
    pub type Ansb02 = crate::EnumBitfieldStruct<u8, Ansb02_SPEC>;
    impl Ansb02 {
        #[doc = "AN102 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN102 is subjected to conversion."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb01_SPEC;
    pub type Ansb01 = crate::EnumBitfieldStruct<u8, Ansb01_SPEC>;
    impl Ansb01 {
        #[doc = "AN101 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN101 is subjected to conversion."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb00_SPEC;
    pub type Ansb00 = crate::EnumBitfieldStruct<u8, Ansb00_SPEC>;
    impl Ansb00 {
        #[doc = "AN100 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN100 is subjected to conversion."]
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
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Adansb1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Adansb1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AN117 Select"]
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

    #[doc = "AN116 Select"]
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
    pub struct Ansb17_SPEC;
    pub type Ansb17 = crate::EnumBitfieldStruct<u8, Ansb17_SPEC>;
    impl Ansb17 {
        #[doc = "AN117 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN117 is subjected to conversion."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb16_SPEC;
    pub type Ansb16 = crate::EnumBitfieldStruct<u8, Ansb16_SPEC>;
    impl Ansb16 {
        #[doc = "AN116 is not subjected to conversion."]
        pub const _0: Self = Self::new(0);

        #[doc = "AN116 is subjected to conversion."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addbldr_SPEC;
impl crate::sealed::RegSpec for Addbldr_SPEC {
    type DataType = u16;
}

#[doc = "A/D Data Duplication Register"]
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
pub struct Adrd_SPEC;
impl crate::sealed::RegSpec for Adrd_SPEC {
    type DataType = u16;
}

#[doc = "A/D Self-Diagnosis Data Register"]
pub type Adrd = crate::RegValueT<Adrd_SPEC>;

impl Adrd {
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

    #[doc = "These bits are read as 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, Adrd_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,Adrd_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "A/D-converted value (right-justified)\nNOTE: Unused bits in the AD bit field are fixed \"0\""]
    #[inline(always)]
    pub fn ad(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Adrd_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Adrd_SPEC,crate::common::R>::from_register(self,0)
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
        #[doc = "Self-diagnosis has never been executed since power-on."]
        pub const _00: Self = Self::new(0);

        #[doc = "Self-diagnosis using the voltage of 0 V has been executed."]
        pub const _01: Self = Self::new(1);

        #[doc = "Self-diagnosis using the voltage of reference power supply(VREFH) x 1/2 has been executed."]
        pub const _10: Self = Self::new(2);

        #[doc = "Self-diagnosis using the voltage of reference power supply(VREFH) has been executed."]
        pub const _11: Self = Self::new(3);
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
pub struct Adshcr_SPEC;
impl crate::sealed::RegSpec for Adshcr_SPEC {
    type DataType = u16;
}

#[doc = "A/D Sample and Hold Circuit Control Register"]
pub type Adshcr = crate::RegValueT<Adshcr_SPEC>;

impl Adshcr {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, u8, Adshcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1f,1,0,u8,u8,Adshcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AN102 sample-and-hold circuit Select"]
    #[inline(always)]
    pub fn shans2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adshcr::Shans2,
        adshcr::Shans2,
        Adshcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adshcr::Shans2,
            adshcr::Shans2,
            Adshcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN101 sample-and-hold circuit Select"]
    #[inline(always)]
    pub fn shans1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adshcr::Shans1,
        adshcr::Shans1,
        Adshcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adshcr::Shans1,
            adshcr::Shans1,
            Adshcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AN100 sample-and-hold circuit Select"]
    #[inline(always)]
    pub fn shans0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adshcr::Shans0,
        adshcr::Shans0,
        Adshcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adshcr::Shans0,
            adshcr::Shans0,
            Adshcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel-Dedicated Sample-and-Hold Circuit Sampling Time Setting\n  Set the sampling time (4 to 255 states)"]
    #[inline(always)]
    pub fn sstsh(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adshcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adshcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adshcr {
    #[inline(always)]
    fn default() -> Adshcr {
        <crate::RegValueT<Adshcr_SPEC> as RegisterValue<_>>::new(24)
    }
}
pub mod adshcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shans2_SPEC;
    pub type Shans2 = crate::EnumBitfieldStruct<u8, Shans2_SPEC>;
    impl Shans2 {
        #[doc = "Bypass the sample-and-hold circuit."]
        pub const _0: Self = Self::new(0);

        #[doc = "Use the sample-and-hold circuit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shans1_SPEC;
    pub type Shans1 = crate::EnumBitfieldStruct<u8, Shans1_SPEC>;
    impl Shans1 {
        #[doc = "Bypass the sample-and-hold circuit."]
        pub const _0: Self = Self::new(0);

        #[doc = "Use the sample-and-hold circuit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shans0_SPEC;
    pub type Shans0 = crate::EnumBitfieldStruct<u8, Shans0_SPEC>;
    impl Shans0 {
        #[doc = "Bypass the sample-and-hold circuit."]
        pub const _0: Self = Self::new(0);

        #[doc = "Use the sample-and-hold circuit."]
        pub const _1: Self = Self::new(1);
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
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Addiscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Addiscr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selection of Precharge or Discharge"]
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

    #[doc = "The charging time"]
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
    pub struct Pchg_SPEC;
    pub type Pchg = crate::EnumBitfieldStruct<u8, Pchg_SPEC>;
    impl Pchg {
        #[doc = "Discharge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Precharge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adndis_SPEC;
    pub type Adndis = crate::EnumBitfieldStruct<u8, Adndis_SPEC>;
    impl Adndis {
        #[doc = "Disconnection detection is disabled"]
        pub const _0000: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _0001: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adshmsr_SPEC;
impl crate::sealed::RegSpec for Adshmsr_SPEC {
    type DataType = u8;
}

#[doc = "A/D Sample and Hold Operation Mode Select Register"]
pub type Adshmsr = crate::RegValueT<Adshmsr_SPEC>;

impl Adshmsr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Adshmsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Adshmsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Channel-Dedicated Sample-and-Hold Circuit Operation Mode Select"]
    #[inline(always)]
    pub fn shmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adshmsr::Shmd,
        adshmsr::Shmd,
        Adshmsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adshmsr::Shmd,
            adshmsr::Shmd,
            Adshmsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adshmsr {
    #[inline(always)]
    fn default() -> Adshmsr {
        <crate::RegValueT<Adshmsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adshmsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shmd_SPEC;
    pub type Shmd = crate::EnumBitfieldStruct<u8, Shmd_SPEC>;
    impl Shmd {
        #[doc = "Sampling by channel-dedicated sample-and-hold circuit is disable."]
        pub const _0: Self = Self::new(0);

        #[doc = "Sampling by channel-dedicated sample-and-hold circuit is enable."]
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
    #[doc = "Group B Single Scan Continuous Start\n(Enabled only when PGS = 1. Reserved when PGS = 0.)\nNote: When the GBRP bit has been set to 1, single scan is performed continuously for group B regardless of the setting of the GBRSCN bit."]
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

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Adgspcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Adgspcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Group B Restart Setting\n(Enabled only when PGS = 1. Reserved when PGS = 0.)"]
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

    #[doc = "Group A priority control setting bit.\nNote: When the PGS bit is to be set to 1, the ADCSR.ADCS\\[1:0\\] bits must be set to 01b (group scan mode). If the bits are set to any other values, proper operation is not guaranteed."]
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

#[doc = "A/D Data Duplication Register A"]
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

#[doc = "A/D Data Duplication Register B"]
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

    #[doc = "Combination result monitor\nThis bit indicates the combination result.\nThis bit is valid when both window A operation and window B operation are enabled."]
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

    #[doc = "Window A/B Composite Conditions Setting\nNOTE: These bits are valid when both window A and window B are enabled (CMPAE = 1 and CMPBE = 1)."]
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
        #[doc = "S12ADCMPAIi interrupt is disabled when comparison conditions (window A) are met."]
        pub const _0: Self = Self::new(0);

        #[doc = "S12ADCMPAIi interrupt is enabled when comparison conditions (window A) are met."]
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
        #[doc = "S12ADCMPBIi interrupt is disabled when comparison conditions (window B) are met."]
        pub const _0: Self = Self::new(0);

        #[doc = "S12ADCMPBIi interrupt is enabled when comparison conditions (window B) are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpae_SPEC;
    pub type Cmpae = crate::EnumBitfieldStruct<u8, Cmpae_SPEC>;
    impl Cmpae {
        #[doc = "Compare window A operation is disabled. S12ADWMELC and S12ADWUMELC outputs are disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare window A operation is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpbe_SPEC;
    pub type Cmpbe = crate::EnumBitfieldStruct<u8, Cmpbe_SPEC>;
    impl Cmpbe {
        #[doc = "Compare window B operation is disabled. S12ADWMELC and S12ADWUMELC outputs are disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare window B operation is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpab_SPEC;
    pub type Cmpab = crate::EnumBitfieldStruct<u8, Cmpab_SPEC>;
    impl Cmpab {
        #[doc = "S12ADWMELC is output when window A comparison conditions are met OR window B comparison conditions are met. S12ADWUMELC is output in other cases."]
        pub const _00: Self = Self::new(0);

        #[doc = "S12ADWMELC is output when window A comparison conditions are met EXOR window B comparison conditions are met. S12ADWUMELC is output in other cases."]
        pub const _01: Self = Self::new(1);

        #[doc = "S12ADWMELC is output when window A comparison conditions are met and window B comparison conditions are met. S12ADWUMELC is output in other cases."]
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

    #[doc = "Compare Window A Internal Reference Voltage Comparison\nCondition Select"]
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
        #[doc = "ADCMPDR0 value > A/D converted value(ADCMPCR.WCMPE=0) / A/D converted value &lt; ADCMPDR0 value or A/D converted value &gt; ADCMPDR1 value (ADCMPCR.WCMPE=1)"]
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
    #[doc = "Compare Window A Channel AN107 Select"]
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

    #[doc = "Compare Window A Channel AN106 Select"]
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

    #[doc = "Compare Window A Channel AN105 Select"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Adcmpansr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Adcmpansr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Compare Window A Channel AN102 Select"]
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

    #[doc = "Compare Window A Channel AN101 Select"]
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

    #[doc = "Compare Window A Channel AN100 Select"]
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
    pub struct Cmpcha07_SPEC;
    pub type Cmpcha07 = crate::EnumBitfieldStruct<u8, Cmpcha07_SPEC>;
    impl Cmpcha07 {
        #[doc = "Disable compare function for AN107"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for AN107"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha06_SPEC;
    pub type Cmpcha06 = crate::EnumBitfieldStruct<u8, Cmpcha06_SPEC>;
    impl Cmpcha06 {
        #[doc = "Disable compare function for AN106"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for AN106"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha05_SPEC;
    pub type Cmpcha05 = crate::EnumBitfieldStruct<u8, Cmpcha05_SPEC>;
    impl Cmpcha05 {
        #[doc = "Disable compare function for AN105"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for AN105"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha02_SPEC;
    pub type Cmpcha02 = crate::EnumBitfieldStruct<u8, Cmpcha02_SPEC>;
    impl Cmpcha02 {
        #[doc = "Disable compare function for AN102"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for AN102"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha01_SPEC;
    pub type Cmpcha01 = crate::EnumBitfieldStruct<u8, Cmpcha01_SPEC>;
    impl Cmpcha01 {
        #[doc = "Disable compare function for AN101"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for AN101"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha00_SPEC;
    pub type Cmpcha00 = crate::EnumBitfieldStruct<u8, Cmpcha00_SPEC>;
    impl Cmpcha00 {
        #[doc = "Disable compare function for AN100"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for AN100"]
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
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Adcmpansr1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Adcmpansr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AN117 Select"]
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

    #[doc = "AN116 Select"]
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
    pub struct Cmpcha17_SPEC;
    pub type Cmpcha17 = crate::EnumBitfieldStruct<u8, Cmpcha17_SPEC>;
    impl Cmpcha17 {
        #[doc = "Excludes AN117 from the compare window A target range."]
        pub const _0: Self = Self::new(0);

        #[doc = "Includes AN117  from the compare window A target range."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha16_SPEC;
    pub type Cmpcha16 = crate::EnumBitfieldStruct<u8, Cmpcha16_SPEC>;
    impl Cmpcha16 {
        #[doc = "Excludes AN116 from the compare window A target range."]
        pub const _0: Self = Self::new(0);

        #[doc = "Includes AN116  from the compare window A target range."]
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
    #[doc = "Comparison condition of AN107"]
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

    #[doc = "Comparison condition of AN106"]
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

    #[doc = "Comparison condition of AN105"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Adcmplr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Adcmplr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Comparison condition of AN102"]
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

    #[doc = "Comparison condition of AN101"]
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

    #[doc = "Comparison condition of AN100"]
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
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Adcmplr1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Adcmplr1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Comparison condition of AN117"]
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

    #[doc = "Comparison condition of AN116"]
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
    #[doc = "The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A.."]
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
    #[doc = "Compare window A flag of AN007"]
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

    #[doc = "Compare window A flag of AN006"]
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

    #[doc = "Compare window A flag of AN005"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Adcmpsr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Adcmpsr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Compare window A flag of AN002"]
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

    #[doc = "Compare window A flag of AN001"]
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

    #[doc = "Compare window A flag of AN000"]
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
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Adcmpsr1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Adcmpsr1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Compare window A flag of AN017"]
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

    #[doc = "Compare window A flag of AN016"]
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

    #[doc = "Compare Window A Internal Reference Voltage Compare Flag \nWhen window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
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

    #[doc = "Compare Window A Temperature Sensor Output Compare Flag \nWhen window A operation is enabled (ADCMPCR.CMPAE = 1b), this bit indicates the temperature sensor output comparison result. When window A operation is disabled (ADCMPCR.CMPAE = 0b), comparison conditions for CMPSTTSA are not met any time."]
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

    #[doc = "Compare window B channel selection bit.\nThe channel that compares it on the condition of compare window B is selected."]
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
        #[doc = "AN100"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "AN101"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "AN102"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "AN105"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "AN106"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "AN107"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "AN116"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "AN117"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "Temperature sensor"]
        pub const _0_X_20: Self = Self::new(32);

        #[doc = "Internal reference voltage"]
        pub const _0_X_21: Self = Self::new(33);

        #[doc = "No channel is selected"]
        pub const _0_X_3_F: Self = Self::new(63);
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

    #[doc = "Compare window B flag.\nIt is a status flag that shows the comparative result of CH (AN100-AN102, AN105-AN107, AN116-AN117, temperature sensor, and internal reference voltage) made the object of window B relation condition."]
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
    #[doc = "Sampling Time Setting (AN116-AN117)"]
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
        <crate::RegValueT<Adsstrl_SPEC> as RegisterValue<_>>::new(11)
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
    #[doc = "Sampling Time Setting (temperature sensor output)"]
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
        <crate::RegValueT<Adsstrt_SPEC> as RegisterValue<_>>::new(11)
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
        <crate::RegValueT<Adsstro_SPEC> as RegisterValue<_>>::new(11)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstr0_SPEC;
impl crate::sealed::RegSpec for Adsstr0_SPEC {
    type DataType = u8;
}

#[doc = "A/D Sampling State Register %s (Corresponding Channel is AN10 )"]
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
        <crate::RegValueT<Adsstr0_SPEC> as RegisterValue<_>>::new(11)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adpgacr_SPEC;
impl crate::sealed::RegSpec for Adpgacr_SPEC {
    type DataType = u16;
}

#[doc = "A/D Programmable Gain Amplifier Control Register"]
pub type Adpgacr = crate::RegValueT<Adpgacr_SPEC>;

impl Adpgacr {
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Adpgacr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Adpgacr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PGA P002 gain setting and enable bit"]
    #[inline(always)]
    pub fn p002gen(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adpgacr::P002Gen,
        adpgacr::P002Gen,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adpgacr::P002Gen,
            adpgacr::P002Gen,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Amplifier enable bit for PGA P002"]
    #[inline(always)]
    pub fn p002enamp(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adpgacr::P002Enamp,
        adpgacr::P002Enamp,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adpgacr::P002Enamp,
            adpgacr::P002Enamp,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The amplifier passing is enable for PGA P002"]
    #[inline(always)]
    pub fn p002sel1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adpgacr::P002Sel1,
        adpgacr::P002Sel1,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adpgacr::P002Sel1,
            adpgacr::P002Sel1,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A through amplifier is enable for PGA P002"]
    #[inline(always)]
    pub fn p002sel0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adpgacr::P002Sel0,
        adpgacr::P002Sel0,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adpgacr::P002Sel0,
            adpgacr::P002Sel0,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PGA P001 gain setting and enable bit"]
    #[inline(always)]
    pub fn p001gen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adpgacr::P001Gen,
        adpgacr::P001Gen,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adpgacr::P001Gen,
            adpgacr::P001Gen,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Amplifier enable bit for PGA P001"]
    #[inline(always)]
    pub fn p001enamp(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adpgacr::P001Enamp,
        adpgacr::P001Enamp,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adpgacr::P001Enamp,
            adpgacr::P001Enamp,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The amplifier passing is enable for PGA P001"]
    #[inline(always)]
    pub fn p001sel1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adpgacr::P001Sel1,
        adpgacr::P001Sel1,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adpgacr::P001Sel1,
            adpgacr::P001Sel1,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A through amplifier is enable for PGA P001"]
    #[inline(always)]
    pub fn p001sel0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adpgacr::P001Sel0,
        adpgacr::P001Sel0,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adpgacr::P001Sel0,
            adpgacr::P001Sel0,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PGA P000 gain setting and enable bit"]
    #[inline(always)]
    pub fn p000gen(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adpgacr::P000Gen,
        adpgacr::P000Gen,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adpgacr::P000Gen,
            adpgacr::P000Gen,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Amplifier enable bit for PGA P000"]
    #[inline(always)]
    pub fn p000enamp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adpgacr::P000Enamp,
        adpgacr::P000Enamp,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adpgacr::P000Enamp,
            adpgacr::P000Enamp,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The amplifier passing is enable for PGA P000"]
    #[inline(always)]
    pub fn p000sel1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adpgacr::P000Sel1,
        adpgacr::P000Sel1,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adpgacr::P000Sel1,
            adpgacr::P000Sel1,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A through amplifier is enable for PGA P000"]
    #[inline(always)]
    pub fn p000sel0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adpgacr::P000Sel0,
        adpgacr::P000Sel0,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adpgacr::P000Sel0,
            adpgacr::P000Sel0,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adpgacr {
    #[inline(always)]
    fn default() -> Adpgacr {
        <crate::RegValueT<Adpgacr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adpgacr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P002Gen_SPEC;
    pub type P002Gen = crate::EnumBitfieldStruct<u8, P002Gen_SPEC>;
    impl P002Gen {
        #[doc = "The gain setting is invalidated (AIN is not input in PGA)."]
        pub const _0: Self = Self::new(0);

        #[doc = "The gain setting is effectively done (AIN is input in PGA)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P002Enamp_SPEC;
    pub type P002Enamp = crate::EnumBitfieldStruct<u8, P002Enamp_SPEC>;
    impl P002Enamp {
        #[doc = "The amplifier in PGA is not used."]
        pub const _0: Self = Self::new(0);

        #[doc = "The amplifier in PGA is used."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P002Sel1_SPEC;
    pub type P002Sel1 = crate::EnumBitfieldStruct<u8, P002Sel1_SPEC>;
    impl P002Sel1 {
        #[doc = "By way of the amplifier in PGA."]
        pub const _0: Self = Self::new(0);

        #[doc = "Note 1 that by way of amplifier in PGA"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P002Sel0_SPEC;
    pub type P002Sel0 = crate::EnumBitfieldStruct<u8, P002Sel0_SPEC>;
    impl P002Sel0 {
        #[doc = "Not through the PGA in amplifier"]
        pub const _0: Self = Self::new(0);

        #[doc = "I will through in the PGA amplifier."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P001Gen_SPEC;
    pub type P001Gen = crate::EnumBitfieldStruct<u8, P001Gen_SPEC>;
    impl P001Gen {
        #[doc = "The gain setting is invalidated (AIN is not input in PGA)."]
        pub const _0: Self = Self::new(0);

        #[doc = "The gain setting is effectively done (AIN is input in PGA)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P001Enamp_SPEC;
    pub type P001Enamp = crate::EnumBitfieldStruct<u8, P001Enamp_SPEC>;
    impl P001Enamp {
        #[doc = "The amplifier in PGA is not used."]
        pub const _0: Self = Self::new(0);

        #[doc = "The amplifier in PGA is used."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P001Sel1_SPEC;
    pub type P001Sel1 = crate::EnumBitfieldStruct<u8, P001Sel1_SPEC>;
    impl P001Sel1 {
        #[doc = "By way of the amplifier in PGA."]
        pub const _0: Self = Self::new(0);

        #[doc = "Note 1 that by way of amplifier in PGA"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P001Sel0_SPEC;
    pub type P001Sel0 = crate::EnumBitfieldStruct<u8, P001Sel0_SPEC>;
    impl P001Sel0 {
        #[doc = "Not through the PGA in amplifier"]
        pub const _0: Self = Self::new(0);

        #[doc = "I will through in the PGA amplifier."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P000Gen_SPEC;
    pub type P000Gen = crate::EnumBitfieldStruct<u8, P000Gen_SPEC>;
    impl P000Gen {
        #[doc = "The gain setting is invalidated (AIN is not input in PGA)."]
        pub const _0: Self = Self::new(0);

        #[doc = "The gain setting is effectively done (AIN is input in PGA)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P000Enamp_SPEC;
    pub type P000Enamp = crate::EnumBitfieldStruct<u8, P000Enamp_SPEC>;
    impl P000Enamp {
        #[doc = "The amplifier in PGA is not used."]
        pub const _0: Self = Self::new(0);

        #[doc = "The amplifier in PGA is used."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P000Sel1_SPEC;
    pub type P000Sel1 = crate::EnumBitfieldStruct<u8, P000Sel1_SPEC>;
    impl P000Sel1 {
        #[doc = "By way of the amplifier in PGA."]
        pub const _0: Self = Self::new(0);

        #[doc = "Note 1 that by way of amplifier in PGA"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P000Sel0_SPEC;
    pub type P000Sel0 = crate::EnumBitfieldStruct<u8, P000Sel0_SPEC>;
    impl P000Sel0 {
        #[doc = "Not through the PGA in amplifier"]
        pub const _0: Self = Self::new(0);

        #[doc = "I will through in the PGA amplifier."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adpgags0_SPEC;
impl crate::sealed::RegSpec for Adpgags0_SPEC {
    type DataType = u16;
}

#[doc = "A/D Programmable Gain Amplifier Gain Setting Register 0"]
pub type Adpgags0 = crate::RegValueT<Adpgags0_SPEC>;

impl Adpgags0 {
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Adpgags0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Adpgags0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PGA P002 gain setting bit.\nThe gain magnification of (ADPGSDCR0.P002GEN=0b) when the shingle end is input and each PGA P002 is set. \nWhen the differential motion is input, (ADPGSDCR0.P002GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P002DG 1:0."]
    #[inline(always)]
    pub fn p002gain(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        adpgags0::P002Gain,
        adpgags0::P002Gain,
        Adpgags0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            adpgags0::P002Gain,
            adpgags0::P002Gain,
            Adpgags0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PGA P001 gain setting bit.\nThe gain magnification of (ADPGSDCR0.P001GEN=0b) when the shingle end is input and each PGA P001 is set. \nWhen the differential motion is input, (ADPGSDCR0.P001GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P001DG 1:0."]
    #[inline(always)]
    pub fn p001gain(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        adpgags0::P001Gain,
        adpgags0::P001Gain,
        Adpgags0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            adpgags0::P001Gain,
            adpgags0::P001Gain,
            Adpgags0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PGA P000 gain setting bit.\nThe gain magnification of (ADPGSDCR0.P000GEN=0b) when the shingle end is input and each PGA P000 is set. \nWhen the differential motion is input, (ADPGSDCR0.P000GEN=1b) sets the gain magnification when the differential motion is input by the combination with ADPGSDCR0.P000DG 1:0."]
    #[inline(always)]
    pub fn p000gain(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        adpgags0::P000Gain,
        adpgags0::P000Gain,
        Adpgags0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            adpgags0::P000Gain,
            adpgags0::P000Gain,
            Adpgags0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adpgags0 {
    #[inline(always)]
    fn default() -> Adpgags0 {
        <crate::RegValueT<Adpgags0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adpgags0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P002Gain_SPEC;
    pub type P002Gain = crate::EnumBitfieldStruct<u8, P002Gain_SPEC>;
    impl P002Gain {
        #[doc = "x 2.000 (ADPGADDCR0.P002DEN=0)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "x 2.500 (ADPGADDCR0.P002DEN=0) /  x 1.500 (ADPGADDCR0.P002DEN=1)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "x 2.667 (ADPGADDCR0.P002DEN=0)"]
        pub const _0010: Self = Self::new(2);

        #[doc = "x 2.857 (ADPGADDCR0.P002DEN=0)"]
        pub const _0011: Self = Self::new(3);

        #[doc = "x 3.077 (ADPGADDCR0.P002DEN=0)"]
        pub const _0100: Self = Self::new(4);

        #[doc = "x 3.333 (ADPGADDCR0.P002DEN=0) /  x 2.333 (ADPGADDCR0.P002DEN=1)"]
        pub const _0101: Self = Self::new(5);

        #[doc = "x 3.636 (ADPGADDCR0.P002DEN=0)"]
        pub const _0110: Self = Self::new(6);

        #[doc = "x 4.000 (ADPGADDCR0.P002DEN=0)"]
        pub const _0111: Self = Self::new(7);

        #[doc = "x 4.444 (ADPGADDCR0.P002DEN=0)"]
        pub const _1000: Self = Self::new(8);

        #[doc = "x 5.000 (ADPGADDCR0.P002DEN=0) /  x 4.00 (ADPGADDCR0.P002DEN=1)"]
        pub const _1001: Self = Self::new(9);

        #[doc = "x 5.714 (ADPGADDCR0.P002DEN=0)"]
        pub const _1010: Self = Self::new(10);

        #[doc = "x 6.667 (ADPGADDCR0.P002DEN=0) /  x 5.667 (ADPGADDCR0.P002DEN=1)"]
        pub const _1011: Self = Self::new(11);

        #[doc = "x 8.000 (ADPGADDCR0.P002DEN=0)"]
        pub const _1100: Self = Self::new(12);

        #[doc = "x 10.000 (ADPGADDCR0.P002DEN=0)"]
        pub const _1101: Self = Self::new(13);

        #[doc = "x 13.333 (ADPGADDCR0.P002DEN=0)"]
        pub const _1110: Self = Self::new(14);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P001Gain_SPEC;
    pub type P001Gain = crate::EnumBitfieldStruct<u8, P001Gain_SPEC>;
    impl P001Gain {
        #[doc = "x 2.000 (ADPGADDCR0.P001DEN=0)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "x 2.500 (ADPGADDCR0.P001DEN=0) /  x 1.500 (ADPGADDCR0.P001DEN=1)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "x 2.667 (ADPGADDCR0.P001DEN=0)"]
        pub const _0010: Self = Self::new(2);

        #[doc = "x 2.857 (ADPGADDCR0.P001DEN=0)"]
        pub const _0011: Self = Self::new(3);

        #[doc = "x 3.077 (ADPGADDCR0.P001DEN=0)"]
        pub const _0100: Self = Self::new(4);

        #[doc = "x 3.333 (ADPGADDCR0.P001DEN=0) /  x 2.333 (ADPGADDCR0.P001DEN=1)"]
        pub const _0101: Self = Self::new(5);

        #[doc = "x 3.636 (ADPGADDCR0.P001DEN=0)"]
        pub const _0110: Self = Self::new(6);

        #[doc = "x 4.000 (ADPGADDCR0.P001DEN=0)"]
        pub const _0111: Self = Self::new(7);

        #[doc = "x 4.444 (ADPGADDCR0.P001DEN=0)"]
        pub const _1000: Self = Self::new(8);

        #[doc = "x 5.000 (ADPGADDCR0.P001DEN=0) /  x 4.00 (ADPGADDCR0.P001DEN=1)"]
        pub const _1001: Self = Self::new(9);

        #[doc = "x 5.714 (ADPGADDCR0.P001DEN=0)"]
        pub const _1010: Self = Self::new(10);

        #[doc = "x 6.667 (ADPGADDCR0.P001DEN=0) /  x 5.667 (ADPGADDCR0.P001DEN=1)"]
        pub const _1011: Self = Self::new(11);

        #[doc = "x 8.000 (ADPGADDCR0.P001DEN=0)"]
        pub const _1100: Self = Self::new(12);

        #[doc = "x 10.000 (ADPGADDCR0.P001DEN=0)"]
        pub const _1101: Self = Self::new(13);

        #[doc = "x 13.333 (ADPGADDCR0.P001DEN=0)"]
        pub const _1110: Self = Self::new(14);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P000Gain_SPEC;
    pub type P000Gain = crate::EnumBitfieldStruct<u8, P000Gain_SPEC>;
    impl P000Gain {
        #[doc = "x 2.000 (ADPGADDCR0.P000DEN=0)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "x 2.500 (ADPGADDCR0.P000DEN=0) /  x 1.500 (ADPGADDCR0.P000DEN=1)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "x 2.667 (ADPGADDCR0.P000DEN=0)"]
        pub const _0010: Self = Self::new(2);

        #[doc = "x 2.857 (ADPGADDCR0.P000DEN=0)"]
        pub const _0011: Self = Self::new(3);

        #[doc = "x 3.077 (ADPGADDCR0.P000DEN=0)"]
        pub const _0100: Self = Self::new(4);

        #[doc = "x 3.333 (ADPGADDCR0.P000DEN=0) /  x 2.333 (ADPGADDCR0.P000DEN=1)"]
        pub const _0101: Self = Self::new(5);

        #[doc = "x 3.636 (ADPGADDCR0.P000DEN=0)"]
        pub const _0110: Self = Self::new(6);

        #[doc = "x 4.000 (ADPGADDCR0.P000DEN=0)"]
        pub const _0111: Self = Self::new(7);

        #[doc = "x 4.444 (ADPGADDCR0.P000DEN=0)"]
        pub const _1000: Self = Self::new(8);

        #[doc = "x 5.000 (ADPGADDCR0.P000DEN=0) /  x 4.00 (ADPGADDCR0.P000DEN=1)"]
        pub const _1001: Self = Self::new(9);

        #[doc = "x 5.714 (ADPGADDCR0.P000DEN=0)"]
        pub const _1010: Self = Self::new(10);

        #[doc = "x 6.667 (ADPGADDCR0.P000DEN=0) /  x 5.667 (ADPGADDCR0.P000DEN=1)"]
        pub const _1011: Self = Self::new(11);

        #[doc = "x 8.000 (ADPGADDCR0.P000DEN=0)"]
        pub const _1100: Self = Self::new(12);

        #[doc = "x 10.000 (ADPGADDCR0.P000DEN=0)"]
        pub const _1101: Self = Self::new(13);

        #[doc = "x 13.333 (ADPGADDCR0.P000DEN=0)"]
        pub const _1110: Self = Self::new(14);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adpgadcr0_SPEC;
impl crate::sealed::RegSpec for Adpgadcr0_SPEC {
    type DataType = u16;
}

#[doc = "A/D Programmable Gain Amplifier Differential Input Control Register"]
pub type Adpgadcr0 = crate::RegValueT<Adpgadcr0_SPEC>;

impl Adpgadcr0 {
    #[doc = "P003 Differential Input Gain Setting\nNOTE: When these bits are used, set {P003DEN, P003GEN} to 11b."]
    #[inline(always)]
    pub fn p003dg(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        adpgadcr0::P003Dg,
        adpgadcr0::P003Dg,
        Adpgadcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            adpgadcr0::P003Dg,
            adpgadcr0::P003Dg,
            Adpgadcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P002 Differential Input Enable"]
    #[inline(always)]
    pub fn p002den(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adpgadcr0::P002Den,
        adpgadcr0::P002Den,
        Adpgadcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adpgadcr0::P002Den,
            adpgadcr0::P002Den,
            Adpgadcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P002 Differential Input Gain Setting\nNOTE: When these bits are used, set {P002DEN, P002GEN} to 11b."]
    #[inline(always)]
    pub fn p002dg(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        adpgadcr0::P002Dg,
        adpgadcr0::P002Dg,
        Adpgadcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            adpgadcr0::P002Dg,
            adpgadcr0::P002Dg,
            Adpgadcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P001 Differential Input Enable"]
    #[inline(always)]
    pub fn p001den(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adpgadcr0::P001Den,
        adpgadcr0::P001Den,
        Adpgadcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adpgadcr0::P001Den,
            adpgadcr0::P001Den,
            Adpgadcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P001 Differential Input Gain Setting\nNOTE: When these bits are used, set {P001DEN, P001GEN} to 11b."]
    #[inline(always)]
    pub fn p001dg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        adpgadcr0::P001Dg,
        adpgadcr0::P001Dg,
        Adpgadcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            adpgadcr0::P001Dg,
            adpgadcr0::P001Dg,
            Adpgadcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P000 Differential Input Enable"]
    #[inline(always)]
    pub fn p000den(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adpgadcr0::P000Den,
        adpgadcr0::P000Den,
        Adpgadcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adpgadcr0::P000Den,
            adpgadcr0::P000Den,
            Adpgadcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Adpgadcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Adpgadcr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "P000 Differential Input Gain Setting\nNOTE: When these bits are used, set {P000DEN, P000GEN} to 11b."]
    #[inline(always)]
    pub fn p000dg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adpgadcr0::P000Dg,
        adpgadcr0::P000Dg,
        Adpgadcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adpgadcr0::P000Dg,
            adpgadcr0::P000Dg,
            Adpgadcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adpgadcr0 {
    #[inline(always)]
    fn default() -> Adpgadcr0 {
        <crate::RegValueT<Adpgadcr0_SPEC> as RegisterValue<_>>::new(34952)
    }
}
pub mod adpgadcr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P003Dg_SPEC;
    pub type P003Dg = crate::EnumBitfieldStruct<u8, P003Dg_SPEC>;
    impl P003Dg {
        #[doc = "x 1.5"]
        pub const _00: Self = Self::new(0);

        #[doc = "x 2.333"]
        pub const _01: Self = Self::new(1);

        #[doc = "x 4.0"]
        pub const _10: Self = Self::new(2);

        #[doc = "x 5.667"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P002Den_SPEC;
    pub type P002Den = crate::EnumBitfieldStruct<u8, P002Den_SPEC>;
    impl P002Den {
        #[doc = "Differential input is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Differential input is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P002Dg_SPEC;
    pub type P002Dg = crate::EnumBitfieldStruct<u8, P002Dg_SPEC>;
    impl P002Dg {
        #[doc = "x 1.5"]
        pub const _00: Self = Self::new(0);

        #[doc = "x 2.333"]
        pub const _01: Self = Self::new(1);

        #[doc = "x 4.0"]
        pub const _10: Self = Self::new(2);

        #[doc = "x 5.667"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P001Den_SPEC;
    pub type P001Den = crate::EnumBitfieldStruct<u8, P001Den_SPEC>;
    impl P001Den {
        #[doc = "Differential input is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Differential input is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P001Dg_SPEC;
    pub type P001Dg = crate::EnumBitfieldStruct<u8, P001Dg_SPEC>;
    impl P001Dg {
        #[doc = "x 1.5"]
        pub const _00: Self = Self::new(0);

        #[doc = "x 2.333"]
        pub const _01: Self = Self::new(1);

        #[doc = "x 4.0"]
        pub const _10: Self = Self::new(2);

        #[doc = "x 5.667"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P000Den_SPEC;
    pub type P000Den = crate::EnumBitfieldStruct<u8, P000Den_SPEC>;
    impl P000Den {
        #[doc = "Differential input is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Differential input is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P000Dg_SPEC;
    pub type P000Dg = crate::EnumBitfieldStruct<u8, P000Dg_SPEC>;
    impl P000Dg {
        #[doc = "x 1.5"]
        pub const _00: Self = Self::new(0);

        #[doc = "x 2.333"]
        pub const _01: Self = Self::new(1);

        #[doc = "x 4.0"]
        pub const _10: Self = Self::new(2);

        #[doc = "x 5.667"]
        pub const _11: Self = Self::new(3);
    }
}
