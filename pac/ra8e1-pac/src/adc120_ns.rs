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
// Generated from SVD 1.00.01, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:53:56 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"12-bit A/D Converter 0"]
unsafe impl ::core::marker::Send for super::Adc120Ns {}
unsafe impl ::core::marker::Sync for super::Adc120Ns {}
impl super::Adc120Ns {
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20usize))
        }
    }
    #[inline(always)]
    pub const fn addr16(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn addr17(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x22usize),
            )
        }
    }
    #[inline(always)]
    pub const fn addr18(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn addr19(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x26usize),
            )
        }
    }

    #[doc = "A/D VBATT Monitor Data Register"]
    #[inline(always)]
    pub const fn advmdr(&self) -> &'static crate::common::Reg<self::Advmdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Advmdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
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

    #[doc = "A/D Sample and Hold Operation Mode Selection Register"]
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

    #[doc = "A/D Data Buffer Registers %s"]
    #[inline(always)]
    pub const fn adbuf(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adbuf_SPEC, crate::common::R>,
        16,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xb0usize))
        }
    }
    #[inline(always)]
    pub const fn adbuf0(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf1(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xb2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf2(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf3(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xb6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf4(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf5(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xbausize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf6(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf7(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xbeusize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf8(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf9(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xc2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf10(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf11(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xc6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf12(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf13(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xcausize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf14(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn adbuf15(&self) -> &'static crate::common::Reg<self::Adbuf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adbuf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xceusize),
            )
        }
    }

    #[doc = "A/D Data Buffer Enable Register"]
    #[inline(always)]
    pub const fn adbufen(
        &self,
    ) -> &'static crate::common::Reg<self::Adbufen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adbufen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "A/D Data Buffer Pointer Register"]
    #[inline(always)]
    pub const fn adbufptr(
        &self,
    ) -> &'static crate::common::Reg<self::Adbufptr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adbufptr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(210usize),
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
    pub const fn adsstrv(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstrv_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstrv_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(227usize),
            )
        }
    }

    #[doc = "A/D Sampling State Register"]
    #[inline(always)]
    pub const fn adsstr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adsstr_SPEC, crate::common::RW>,
        4,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xecusize))
        }
    }
    #[inline(always)]
    pub const fn adsstr16(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr17(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xedusize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr18(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeeusize),
            )
        }
    }
    #[inline(always)]
    pub const fn adsstr19(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xefusize),
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
        #[doc = "Disable ADC12i_GBADI (i = 0, 1) interrupt generation on group B scan completion."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable ADC12i_GBADI (i = 0, 1) interrupt generation on group B scan completion."]
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
        #[doc = "Start A/D conversion by the synchronous trigger (GPT, ELC)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start A/D conversion by the asynchronous trigger (ADTRGn)."]
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
    #[doc = "A/D Conversion Channels Select/Average Channel Select"]
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

    #[doc = "A/D Conversion Channels Select/Average Channel Select"]
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

    #[doc = "A/D Conversion Channels Select/Average Channel Select"]
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

    #[doc = "A/D Conversion Channels Select/Average Channel Select"]
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

    #[doc = "A/D Conversion Channels Select/Average Channel Select"]
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

    #[doc = "A/D Conversion Channels Select/Average Channel Select"]
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

    #[doc = "A/D Conversion Channels Select/Average Channel Select"]
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
    pub struct Adprc_SPEC;
    pub type Adprc = crate::EnumBitfieldStruct<u8, Adprc_SPEC>;
    impl Adprc {
        #[doc = "12-bit accuracy"]
        pub const _00: Self = Self::new(0);

        #[doc = "10-bit accuracy"]
        pub const _01: Self = Self::new(1);

        #[doc = "8-bit accuracy"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tssb_SPEC;
    pub type Tssb = crate::EnumBitfieldStruct<u8, Tssb_SPEC>;
    impl Tssb {
        #[doc = "Disable A/D conversion of temperature sensor output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable A/D conversion of temperature sensor output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ocsb_SPEC;
    pub type Ocsb = crate::EnumBitfieldStruct<u8, Ocsb_SPEC>;
    impl Ocsb {
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
pub struct Advmdr_SPEC;
impl crate::sealed::RegSpec for Advmdr_SPEC {
    type DataType = u16;
}

#[doc = "A/D VBATT Monitor Data Register"]
pub type Advmdr = crate::RegValueT<Advmdr_SPEC>;

impl Advmdr {
    #[doc = "Converted Value 15 to 0"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Advmdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Advmdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Advmdr {
    #[inline(always)]
    fn default() -> Advmdr {
        <crate::RegValueT<Advmdr_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Channel-Dedicated Sample-and-Hold Circuit Sampling Time Setting"]
    #[inline(always)]
    pub fn sstsh(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adshcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adshcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Channel-Dedicated Sample-and-Hold Circuit Bypass Select"]
    #[inline(always)]
    pub fn shans(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        adshcr::Shans,
        adshcr::Shans,
        Adshcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            adshcr::Shans,
            adshcr::Shans,
            Adshcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    pub struct Shans_SPEC;
    pub type Shans = crate::EnumBitfieldStruct<u8, Shans_SPEC>;
    impl Shans {
        #[doc = "Bypass the circuits"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use the circuits"]
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
pub struct Adshmsr_SPEC;
impl crate::sealed::RegSpec for Adshmsr_SPEC {
    type DataType = u8;
}

#[doc = "A/D Sample and Hold Operation Mode Selection Register"]
pub type Adshmsr = crate::RegValueT<Adshmsr_SPEC>;

impl Adshmsr {
    #[doc = "Sampling Operation Selection"]
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
        #[doc = "Sampling Operation Selection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable continuous sampling function"]
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

    #[doc = "Enabled only when PGS = 1 and GBRSCN = 1."]
    #[inline(always)]
    pub fn lgrrs(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adgspcr::Lgrrs,
        adgspcr::Lgrrs,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adgspcr::Lgrrs,
            adgspcr::Lgrrs,
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
    pub struct Lgrrs_SPEC;
    pub type Lgrrs = crate::EnumBitfieldStruct<u8, Lgrrs_SPEC>;
    impl Lgrrs {
        #[doc = "Start rescanning from the first channel for scanning"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start rescanning from the channel for which A/D conversion is not completed."]
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
        #[doc = "Output ADC12i_WCMPM (i = 0, 1) when window A OR window B comparison conditions are met. Otherwise, output ADC12i_WCMPUM (i = 0, 1)."]
        pub const _00: Self = Self::new(0);

        #[doc = "Output ADC12i_WCMPM (i = 0, 1) when window A EXOR window B comparison conditions are met. Otherwise, output ADC12i_WCMPUM (i = 0, 1)."]
        pub const _01: Self = Self::new(1);

        #[doc = "Output ADC12i_WCMPM (i = 0, 1) when window A AND window B comparison conditions are met. Otherwise, output ADC12i_WCMPUM (i = 0, 1)."]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpbe_SPEC;
    pub type Cmpbe = crate::EnumBitfieldStruct<u8, Cmpbe_SPEC>;
    impl Cmpbe {
        #[doc = "Disable compare window B operation. Disable ADC12i_WCMPM (i = 0, 1) and ADC12i_WCMPUM (i = 0, 1) outputs."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare window B operation."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpae_SPEC;
    pub type Cmpae = crate::EnumBitfieldStruct<u8, Cmpae_SPEC>;
    impl Cmpae {
        #[doc = "Disable compare window A operation. Disable ADC12i_WCMPM (i = 0, 1) and ADC12i_WCMPUM (i = 0, 1) outputs."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare window A operation."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpbie_SPEC;
    pub type Cmpbie = crate::EnumBitfieldStruct<u8, Cmpbie_SPEC>;
    impl Cmpbie {
        #[doc = "Disable ADC12i_CMPBI (i = 0, 1) interrupt when comparison conditions (window B) are met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable ADC12i_CMPBI (i = 0, 1) interrupt when comparison conditions (window B) are met."]
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
        #[doc = "Disable ADC12i_CMPAI (i = 0, 1) interrupt when comparison conditions (window A) are met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable ADC12i_CMPAI (i = 0, 1) interrupt when comparison conditions (window A) are met."]
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
pub struct Adbuf_SPEC;
impl crate::sealed::RegSpec for Adbuf_SPEC {
    type DataType = u16;
}

#[doc = "A/D Data Buffer Registers %s"]
pub type Adbuf = crate::RegValueT<Adbuf_SPEC>;

impl Adbuf {
    #[doc = "Converted Value 15 to 0"]
    #[inline(always)]
    pub fn adbuf(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adbuf_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adbuf_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Adbuf {
    #[inline(always)]
    fn default() -> Adbuf {
        <crate::RegValueT<Adbuf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adbufen_SPEC;
impl crate::sealed::RegSpec for Adbufen_SPEC {
    type DataType = u8;
}

#[doc = "A/D Data Buffer Enable Register"]
pub type Adbufen = crate::RegValueT<Adbufen_SPEC>;

impl Adbufen {
    #[doc = "Data Buffer Enable"]
    #[inline(always)]
    pub fn bufen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adbufen::Bufen,
        adbufen::Bufen,
        Adbufen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adbufen::Bufen,
            adbufen::Bufen,
            Adbufen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adbufen {
    #[inline(always)]
    fn default() -> Adbufen {
        <crate::RegValueT<Adbufen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adbufen {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bufen_SPEC;
    pub type Bufen = crate::EnumBitfieldStruct<u8, Bufen_SPEC>;
    impl Bufen {
        #[doc = "The data buffer is not used."]
        pub const _0: Self = Self::new(0);

        #[doc = "The data buffer is used."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adbufptr_SPEC;
impl crate::sealed::RegSpec for Adbufptr_SPEC {
    type DataType = u8;
}

#[doc = "A/D Data Buffer Pointer Register"]
pub type Adbufptr = crate::RegValueT<Adbufptr_SPEC>;

impl Adbufptr {
    #[doc = "Data Buffer Pointer"]
    #[inline(always)]
    pub fn bufptr(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Adbufptr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Adbufptr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pointer Overflow Flag"]
    #[inline(always)]
    pub fn ptrovf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adbufptr::Ptrovf,
        adbufptr::Ptrovf,
        Adbufptr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adbufptr::Ptrovf,
            adbufptr::Ptrovf,
            Adbufptr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adbufptr {
    #[inline(always)]
    fn default() -> Adbufptr {
        <crate::RegValueT<Adbufptr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adbufptr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ptrovf_SPEC;
    pub type Ptrovf = crate::EnumBitfieldStruct<u8, Ptrovf_SPEC>;
    impl Ptrovf {
        #[doc = "The data buffer pointer has not overflowed."]
        pub const _0: Self = Self::new(0);

        #[doc = "The data buffer pointer has overflowed."]
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
        <crate::RegValueT<Adsstrl_SPEC> as RegisterValue<_>>::new(11)
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
        <crate::RegValueT<Adsstrt_SPEC> as RegisterValue<_>>::new(11)
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
        <crate::RegValueT<Adsstro_SPEC> as RegisterValue<_>>::new(11)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstrv_SPEC;
impl crate::sealed::RegSpec for Adsstrv_SPEC {
    type DataType = u8;
}

#[doc = "A/D Sampling State Register"]
pub type Adsstrv = crate::RegValueT<Adsstrv_SPEC>;

impl Adsstrv {
    #[doc = "Sampling Time Setting"]
    #[inline(always)]
    pub fn sst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adsstrv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adsstrv_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstrv {
    #[inline(always)]
    fn default() -> Adsstrv {
        <crate::RegValueT<Adsstrv_SPEC> as RegisterValue<_>>::new(11)
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
        <crate::RegValueT<Adsstr_SPEC> as RegisterValue<_>>::new(11)
    }
}
