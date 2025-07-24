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
// Generated from SVD 1.20.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:50:30 +0000

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
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x38usize))
        }
    }
    #[inline(always)]
    pub const fn addr12(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn addr13(&self) -> &'static crate::common::Reg<self::Addr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x3ausize),
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
        2,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xecusize))
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
    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adansa0::Ansa0,
        adansa0::Ansa0,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adansa0::Ansa0,
            adansa0::Ansa0,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adansa0::Ansa1,
        adansa0::Ansa1,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adansa0::Ansa1,
            adansa0::Ansa1,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adansa0::Ansa2,
        adansa0::Ansa2,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adansa0::Ansa2,
            adansa0::Ansa2,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adansa0::Ansa3,
        adansa0::Ansa3,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adansa0::Ansa3,
            adansa0::Ansa3,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adansa0::Ansa4,
        adansa0::Ansa4,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adansa0::Ansa4,
            adansa0::Ansa4,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adansa0::Ansa5,
        adansa0::Ansa5,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adansa0::Ansa5,
            adansa0::Ansa5,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adansa0::Ansa6,
        adansa0::Ansa6,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adansa0::Ansa6,
            adansa0::Ansa6,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adansa0::Ansa7,
        adansa0::Ansa7,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adansa0::Ansa7,
            adansa0::Ansa7,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansa8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adansa0::Ansa8,
        adansa0::Ansa8,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adansa0::Ansa8,
            adansa0::Ansa8,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
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

    #[doc = "A/D Conversion Channels Select n"]
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
}
impl ::core::default::Default for Adansa0 {
    #[inline(always)]
    fn default() -> Adansa0 {
        <crate::RegValueT<Adansa0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adansa0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa0_SPEC;
    pub type Ansa0 = crate::EnumBitfieldStruct<u8, Ansa0_SPEC>;
    impl Ansa0 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa1_SPEC;
    pub type Ansa1 = crate::EnumBitfieldStruct<u8, Ansa1_SPEC>;
    impl Ansa1 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa2_SPEC;
    pub type Ansa2 = crate::EnumBitfieldStruct<u8, Ansa2_SPEC>;
    impl Ansa2 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa3_SPEC;
    pub type Ansa3 = crate::EnumBitfieldStruct<u8, Ansa3_SPEC>;
    impl Ansa3 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa4_SPEC;
    pub type Ansa4 = crate::EnumBitfieldStruct<u8, Ansa4_SPEC>;
    impl Ansa4 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa5_SPEC;
    pub type Ansa5 = crate::EnumBitfieldStruct<u8, Ansa5_SPEC>;
    impl Ansa5 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa6_SPEC;
    pub type Ansa6 = crate::EnumBitfieldStruct<u8, Ansa6_SPEC>;
    impl Ansa6 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa7_SPEC;
    pub type Ansa7 = crate::EnumBitfieldStruct<u8, Ansa7_SPEC>;
    impl Ansa7 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa8_SPEC;
    pub type Ansa8 = crate::EnumBitfieldStruct<u8, Ansa8_SPEC>;
    impl Ansa8 {
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
    #[doc = "A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adads0::Ads0,
        adads0::Ads0,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adads0::Ads0,
            adads0::Ads0,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adads0::Ads1,
        adads0::Ads1,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adads0::Ads1,
            adads0::Ads1,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adads0::Ads2,
        adads0::Ads2,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adads0::Ads2,
            adads0::Ads2,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adads0::Ads3,
        adads0::Ads3,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adads0::Ads3,
            adads0::Ads3,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adads0::Ads4,
        adads0::Ads4,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adads0::Ads4,
            adads0::Ads4,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adads0::Ads5,
        adads0::Ads5,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adads0::Ads5,
            adads0::Ads5,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adads0::Ads6,
        adads0::Ads6,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adads0::Ads6,
            adads0::Ads6,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adads0::Ads7,
        adads0::Ads7,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adads0::Ads7,
            adads0::Ads7,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select n"]
    #[inline(always)]
    pub fn ads8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adads0::Ads8,
        adads0::Ads8,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adads0::Ads8,
            adads0::Ads8,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D-Converted Value Addition/Average Channel Select n"]
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

    #[doc = "A/D-Converted Value Addition/Average Channel Select n"]
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
}
impl ::core::default::Default for Adads0 {
    #[inline(always)]
    fn default() -> Adads0 {
        <crate::RegValueT<Adads0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adads0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads0_SPEC;
    pub type Ads0 = crate::EnumBitfieldStruct<u8, Ads0_SPEC>;
    impl Ads0 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads1_SPEC;
    pub type Ads1 = crate::EnumBitfieldStruct<u8, Ads1_SPEC>;
    impl Ads1 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads2_SPEC;
    pub type Ads2 = crate::EnumBitfieldStruct<u8, Ads2_SPEC>;
    impl Ads2 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads3_SPEC;
    pub type Ads3 = crate::EnumBitfieldStruct<u8, Ads3_SPEC>;
    impl Ads3 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads4_SPEC;
    pub type Ads4 = crate::EnumBitfieldStruct<u8, Ads4_SPEC>;
    impl Ads4 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads5_SPEC;
    pub type Ads5 = crate::EnumBitfieldStruct<u8, Ads5_SPEC>;
    impl Ads5 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads6_SPEC;
    pub type Ads6 = crate::EnumBitfieldStruct<u8, Ads6_SPEC>;
    impl Ads6 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads7_SPEC;
    pub type Ads7 = crate::EnumBitfieldStruct<u8, Ads7_SPEC>;
    impl Ads7 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads8_SPEC;
    pub type Ads8 = crate::EnumBitfieldStruct<u8, Ads8_SPEC>;
    impl Ads8 {
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
    pub struct Ocsad_SPEC;
    pub type Ocsad = crate::EnumBitfieldStruct<u8, Ocsad_SPEC>;
    impl Ocsad {
        #[doc = "Do not select addition/average mode for internal reference voltage."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select addition/average mode for internal reference voltage."]
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
    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adansb0::Ansb0,
        adansb0::Ansb0,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adansb0::Ansb0,
            adansb0::Ansb0,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adansb0::Ansb1,
        adansb0::Ansb1,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adansb0::Ansb1,
            adansb0::Ansb1,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adansb0::Ansb2,
        adansb0::Ansb2,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adansb0::Ansb2,
            adansb0::Ansb2,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adansb0::Ansb3,
        adansb0::Ansb3,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adansb0::Ansb3,
            adansb0::Ansb3,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adansb0::Ansb4,
        adansb0::Ansb4,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adansb0::Ansb4,
            adansb0::Ansb4,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adansb0::Ansb5,
        adansb0::Ansb5,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adansb0::Ansb5,
            adansb0::Ansb5,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adansb0::Ansb6,
        adansb0::Ansb6,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adansb0::Ansb6,
            adansb0::Ansb6,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adansb0::Ansb7,
        adansb0::Ansb7,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adansb0::Ansb7,
            adansb0::Ansb7,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
    #[inline(always)]
    pub fn ansb8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adansb0::Ansb8,
        adansb0::Ansb8,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adansb0::Ansb8,
            adansb0::Ansb8,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "A/D Conversion Channels Select n"]
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

    #[doc = "A/D Conversion Channels Select n"]
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
}
impl ::core::default::Default for Adansb0 {
    #[inline(always)]
    fn default() -> Adansb0 {
        <crate::RegValueT<Adansb0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adansb0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb0_SPEC;
    pub type Ansb0 = crate::EnumBitfieldStruct<u8, Ansb0_SPEC>;
    impl Ansb0 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb1_SPEC;
    pub type Ansb1 = crate::EnumBitfieldStruct<u8, Ansb1_SPEC>;
    impl Ansb1 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb2_SPEC;
    pub type Ansb2 = crate::EnumBitfieldStruct<u8, Ansb2_SPEC>;
    impl Ansb2 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb3_SPEC;
    pub type Ansb3 = crate::EnumBitfieldStruct<u8, Ansb3_SPEC>;
    impl Ansb3 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb4_SPEC;
    pub type Ansb4 = crate::EnumBitfieldStruct<u8, Ansb4_SPEC>;
    impl Ansb4 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb5_SPEC;
    pub type Ansb5 = crate::EnumBitfieldStruct<u8, Ansb5_SPEC>;
    impl Ansb5 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb6_SPEC;
    pub type Ansb6 = crate::EnumBitfieldStruct<u8, Ansb6_SPEC>;
    impl Ansb6 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb7_SPEC;
    pub type Ansb7 = crate::EnumBitfieldStruct<u8, Ansb7_SPEC>;
    impl Ansb7 {
        #[doc = "Do not select associated input channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "Select associated input channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb8_SPEC;
    pub type Ansb8 = crate::EnumBitfieldStruct<u8, Ansb8_SPEC>;
    impl Ansb8 {
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
    #[doc = "Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha0,
        adcmpansr0::Cmpcha0,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha0,
            adcmpansr0::Cmpcha0,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha1,
        adcmpansr0::Cmpcha1,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha1,
            adcmpansr0::Cmpcha1,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha2,
        adcmpansr0::Cmpcha2,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha2,
            adcmpansr0::Cmpcha2,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha3,
        adcmpansr0::Cmpcha3,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha3,
            adcmpansr0::Cmpcha3,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha4,
        adcmpansr0::Cmpcha4,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha4,
            adcmpansr0::Cmpcha4,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha5,
        adcmpansr0::Cmpcha5,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha5,
            adcmpansr0::Cmpcha5,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha6,
        adcmpansr0::Cmpcha6,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha6,
            adcmpansr0::Cmpcha6,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha7,
        adcmpansr0::Cmpcha7,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha7,
            adcmpansr0::Cmpcha7,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select n"]
    #[inline(always)]
    pub fn cmpcha8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha8,
        adcmpansr0::Cmpcha8,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha8,
            adcmpansr0::Cmpcha8,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Channel Select n"]
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

    #[doc = "Compare Window A Channel Select n"]
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
}
impl ::core::default::Default for Adcmpansr0 {
    #[inline(always)]
    fn default() -> Adcmpansr0 {
        <crate::RegValueT<Adcmpansr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpansr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha0_SPEC;
    pub type Cmpcha0 = crate::EnumBitfieldStruct<u8, Cmpcha0_SPEC>;
    impl Cmpcha0 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha1_SPEC;
    pub type Cmpcha1 = crate::EnumBitfieldStruct<u8, Cmpcha1_SPEC>;
    impl Cmpcha1 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha2_SPEC;
    pub type Cmpcha2 = crate::EnumBitfieldStruct<u8, Cmpcha2_SPEC>;
    impl Cmpcha2 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha3_SPEC;
    pub type Cmpcha3 = crate::EnumBitfieldStruct<u8, Cmpcha3_SPEC>;
    impl Cmpcha3 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha4_SPEC;
    pub type Cmpcha4 = crate::EnumBitfieldStruct<u8, Cmpcha4_SPEC>;
    impl Cmpcha4 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha5_SPEC;
    pub type Cmpcha5 = crate::EnumBitfieldStruct<u8, Cmpcha5_SPEC>;
    impl Cmpcha5 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha6_SPEC;
    pub type Cmpcha6 = crate::EnumBitfieldStruct<u8, Cmpcha6_SPEC>;
    impl Cmpcha6 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha7_SPEC;
    pub type Cmpcha7 = crate::EnumBitfieldStruct<u8, Cmpcha7_SPEC>;
    impl Cmpcha7 {
        #[doc = "Disable compare function for associated input channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable compare function for associated input channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha8_SPEC;
    pub type Cmpcha8 = crate::EnumBitfieldStruct<u8, Cmpcha8_SPEC>;
    impl Cmpcha8 {
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
    #[doc = "Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha0,
        adcmplr0::Cmplcha0,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha0,
            adcmplr0::Cmplcha0,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha1,
        adcmplr0::Cmplcha1,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha1,
            adcmplr0::Cmplcha1,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha2,
        adcmplr0::Cmplcha2,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha2,
            adcmplr0::Cmplcha2,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha3,
        adcmplr0::Cmplcha3,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha3,
            adcmplr0::Cmplcha3,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha4,
        adcmplr0::Cmplcha4,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha4,
            adcmplr0::Cmplcha4,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha5,
        adcmplr0::Cmplcha5,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha5,
            adcmplr0::Cmplcha5,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha6,
        adcmplr0::Cmplcha6,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha6,
            adcmplr0::Cmplcha6,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha7,
        adcmplr0::Cmplcha7,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha7,
            adcmplr0::Cmplcha7,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select n"]
    #[inline(always)]
    pub fn cmplcha8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha8,
        adcmplr0::Cmplcha8,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha8,
            adcmplr0::Cmplcha8,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Comparison Condition Select n"]
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

    #[doc = "Compare Window A Comparison Condition Select n"]
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
}
impl ::core::default::Default for Adcmplr0 {
    #[inline(always)]
    fn default() -> Adcmplr0 {
        <crate::RegValueT<Adcmplr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmplr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha0_SPEC;
    pub type Cmplcha0 = crate::EnumBitfieldStruct<u8, Cmplcha0_SPEC>;
    impl Cmplcha0 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha1_SPEC;
    pub type Cmplcha1 = crate::EnumBitfieldStruct<u8, Cmplcha1_SPEC>;
    impl Cmplcha1 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha2_SPEC;
    pub type Cmplcha2 = crate::EnumBitfieldStruct<u8, Cmplcha2_SPEC>;
    impl Cmplcha2 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha3_SPEC;
    pub type Cmplcha3 = crate::EnumBitfieldStruct<u8, Cmplcha3_SPEC>;
    impl Cmplcha3 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha4_SPEC;
    pub type Cmplcha4 = crate::EnumBitfieldStruct<u8, Cmplcha4_SPEC>;
    impl Cmplcha4 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha5_SPEC;
    pub type Cmplcha5 = crate::EnumBitfieldStruct<u8, Cmplcha5_SPEC>;
    impl Cmplcha5 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha6_SPEC;
    pub type Cmplcha6 = crate::EnumBitfieldStruct<u8, Cmplcha6_SPEC>;
    impl Cmplcha6 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha7_SPEC;
    pub type Cmplcha7 = crate::EnumBitfieldStruct<u8, Cmplcha7_SPEC>;
    impl Cmplcha7 {
        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value > A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): A/D-converted value < ADCMPDR0 value, or ADCMPDR1 value < A/D-converted value"]
        pub const _0: Self = Self::new(0);

        #[doc = "When window function is disabled (ADCMPCR.WCMPE = 0): ADCMPDR0 value < A/D-converted value When window function is enabled (ADCMPCR.WCMPE = 1): ADCMPDR0 value < A/D-converted value < ADCMPDR1 value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha8_SPEC;
    pub type Cmplcha8 = crate::EnumBitfieldStruct<u8, Cmplcha8_SPEC>;
    impl Cmplcha8 {
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
    #[doc = "Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha0,
        adcmpsr0::Cmpstcha0,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha0,
            adcmpsr0::Cmpstcha0,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha1,
        adcmpsr0::Cmpstcha1,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha1,
            adcmpsr0::Cmpstcha1,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha2,
        adcmpsr0::Cmpstcha2,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha2,
            adcmpsr0::Cmpstcha2,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha3,
        adcmpsr0::Cmpstcha3,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha3,
            adcmpsr0::Cmpstcha3,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha4,
        adcmpsr0::Cmpstcha4,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha4,
            adcmpsr0::Cmpstcha4,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha5,
        adcmpsr0::Cmpstcha5,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha5,
            adcmpsr0::Cmpstcha5,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha6,
        adcmpsr0::Cmpstcha6,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha6,
            adcmpsr0::Cmpstcha6,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha7,
        adcmpsr0::Cmpstcha7,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha7,
            adcmpsr0::Cmpstcha7,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag n"]
    #[inline(always)]
    pub fn cmpstcha8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha8,
        adcmpsr0::Cmpstcha8,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha8,
            adcmpsr0::Cmpstcha8,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compare Window A Flag n"]
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

    #[doc = "Compare Window A Flag n"]
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
}
impl ::core::default::Default for Adcmpsr0 {
    #[inline(always)]
    fn default() -> Adcmpsr0 {
        <crate::RegValueT<Adcmpsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha0_SPEC;
    pub type Cmpstcha0 = crate::EnumBitfieldStruct<u8, Cmpstcha0_SPEC>;
    impl Cmpstcha0 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha1_SPEC;
    pub type Cmpstcha1 = crate::EnumBitfieldStruct<u8, Cmpstcha1_SPEC>;
    impl Cmpstcha1 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha2_SPEC;
    pub type Cmpstcha2 = crate::EnumBitfieldStruct<u8, Cmpstcha2_SPEC>;
    impl Cmpstcha2 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha3_SPEC;
    pub type Cmpstcha3 = crate::EnumBitfieldStruct<u8, Cmpstcha3_SPEC>;
    impl Cmpstcha3 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha4_SPEC;
    pub type Cmpstcha4 = crate::EnumBitfieldStruct<u8, Cmpstcha4_SPEC>;
    impl Cmpstcha4 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha5_SPEC;
    pub type Cmpstcha5 = crate::EnumBitfieldStruct<u8, Cmpstcha5_SPEC>;
    impl Cmpstcha5 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha6_SPEC;
    pub type Cmpstcha6 = crate::EnumBitfieldStruct<u8, Cmpstcha6_SPEC>;
    impl Cmpstcha6 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha7_SPEC;
    pub type Cmpstcha7 = crate::EnumBitfieldStruct<u8, Cmpstcha7_SPEC>;
    impl Cmpstcha7 {
        #[doc = "Comparison conditions are not met."]
        pub const _0: Self = Self::new(0);

        #[doc = "Comparison conditions are met."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha8_SPEC;
    pub type Cmpstcha8 = crate::EnumBitfieldStruct<u8, Cmpstcha8_SPEC>;
    impl Cmpstcha8 {
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

impl NoBitfieldReg<Adbuf_SPEC> for Adbuf {}
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
