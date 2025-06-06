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
// Generated from SVD 1.40.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:21:24 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"12-bit A/D Converter"]
unsafe impl ::core::marker::Send for super::AdcB {}
unsafe impl ::core::marker::Sync for super::AdcB {}
impl super::AdcB {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn adclkenr(
        &self,
    ) -> &'static crate::common::Reg<self::Adclkenr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adclkenr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adclksr(
        &self,
    ) -> &'static crate::common::Reg<self::Adclksr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adclksr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adclkcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adclkcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adclkcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsycr(
        &self,
    ) -> &'static crate::common::Reg<self::Adsycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn aderintcr(
        &self,
    ) -> &'static crate::common::Reg<self::Aderintcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Aderintcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adovfintcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adovfintcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adovfintcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcalintcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcalintcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcalintcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn admdr(&self) -> &'static crate::common::Reg<self::Admdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Admdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adgspcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adgspcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adgspcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsger(
        &self,
    ) -> &'static crate::common::Reg<self::Adsger_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsger_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsgcr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adsgcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsgcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsgcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adsgcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsgcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsgcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Adsgcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsgcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adintcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adintcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adintcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adtrgext(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adtrgext_SPEC, crate::common::RW>,
        9,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc0usize))
        }
    }

    #[inline(always)]
    pub const fn adtrgelc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adtrgelc_SPEC, crate::common::RW>,
        9,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc4usize))
        }
    }

    #[inline(always)]
    pub const fn adtrggpt(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adtrggpt_SPEC, crate::common::RW>,
        9,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc8usize))
        }
    }

    #[inline(always)]
    pub const fn adtrgdlr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adtrgdlr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adtrgdlr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(448usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adtrgdlr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adtrgdlr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adtrgdlr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(452usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adtrgdlr2(
        &self,
    ) -> &'static crate::common::Reg<self::Adtrgdlr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adtrgdlr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(456usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adtrgdlr3(
        &self,
    ) -> &'static crate::common::Reg<self::Adtrgdlr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adtrgdlr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(460usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adtrgdlr4(
        &self,
    ) -> &'static crate::common::Reg<self::Adtrgdlr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adtrgdlr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(464usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsgdcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adsgdcr_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }

    #[inline(always)]
    pub const fn adsstr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(576usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsstr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(580usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsstr2(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(584usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsstr3(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(588usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsstr4(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(592usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsstr5(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(596usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsstr6(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(600usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsstr7(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstr7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstr7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(604usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcnvstr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcnvstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcnvstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(608usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcalstcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcalstcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcalstcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(612usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adshcr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adshcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adshcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(640usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adshstr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adshstr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adshstr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(648usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adshcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adshcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adshcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(652usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adshstr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adshstr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adshstr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(660usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcalshcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcalshcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcalshcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(688usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpgacr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adpgacr_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2c0usize))
        }
    }

    #[inline(always)]
    pub const fn adpgamoncr(
        &self,
    ) -> &'static crate::common::Reg<self::Adpgamoncr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adpgamoncr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adrefcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adrefcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adrefcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(800usize),
            )
        }
    }

    #[inline(always)]
    pub const fn addfsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Addfsr_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x340usize))
        }
    }

    #[inline(always)]
    pub const fn aduoftr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Aduoftr_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x360usize))
        }
    }

    #[inline(always)]
    pub const fn adugtr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adugtr_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x380usize))
        }
    }

    #[inline(always)]
    pub const fn adlimintcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adlimintcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adlimintcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(928usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adlimtr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adlimtr_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x3a4usize))
        }
    }

    #[inline(always)]
    pub const fn adcmpenr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpenr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpenr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpintcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpintcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpintcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1028usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adccmpcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adccmpcr_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x408usize))
        }
    }

    #[inline(always)]
    pub const fn adcmpmdr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpmdr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpmdr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1096usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpmdr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpmdr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpmdr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmptbr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adcmptbr_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x458usize))
        }
    }

    #[inline(always)]
    pub const fn adfifocr(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adfifocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1216usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adfifointcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifointcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adfifointcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1220usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adfifointlr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifointlr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adfifointlr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1224usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adfifointlr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifointlr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adfifointlr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1228usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adfifointlr2(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifointlr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adfifointlr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1232usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adfifointlr3(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifointlr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adfifointlr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1236usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adfifointlr4(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifointlr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adfifointlr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1240usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adchcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adchcr_SPEC, crate::common::RW>,
        37,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x600usize))
        }
    }

    #[inline(always)]
    pub const fn addopcra(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Addopcra_SPEC, crate::common::RW>,
        37,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x604usize))
        }
    }

    #[inline(always)]
    pub const fn addopcrb(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Addopcrb_SPEC, crate::common::RW>,
        37,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x608usize))
        }
    }

    #[inline(always)]
    pub const fn addopcrc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Addopcrc_SPEC, crate::common::RW>,
        37,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x60cusize))
        }
    }

    #[inline(always)]
    pub const fn adcalstr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcalstr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adcalstr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3072usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adtrgenr(
        &self,
    ) -> &'static crate::common::Reg<self::Adtrgenr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adtrgenr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3080usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsystr(
        &self,
    ) -> &'static crate::common::Reg<self::Adsystr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adsystr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3088usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adstr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adstr_SPEC, crate::common::W>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc20usize))
        }
    }

    #[inline(always)]
    pub const fn adstopr(
        &self,
    ) -> &'static crate::common::Reg<self::Adstopr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adstopr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3168usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsr(&self) -> &'static crate::common::Reg<self::Adsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3200usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adgrsr(&self) -> &'static crate::common::Reg<self::Adgrsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adgrsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3204usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adersr(&self) -> &'static crate::common::Reg<self::Adersr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adersr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3208usize),
            )
        }
    }

    #[inline(always)]
    pub const fn aderscr(
        &self,
    ) -> &'static crate::common::Reg<self::Aderscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Aderscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3212usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcalendsr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcalendsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adcalendsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3224usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcalendscr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcalendscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adcalendscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3228usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adovfersr(
        &self,
    ) -> &'static crate::common::Reg<self::Adovfersr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adovfersr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3232usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adovfchsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adovfchsr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adovfchsr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3236usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adovfexsr(
        &self,
    ) -> &'static crate::common::Reg<self::Adovfexsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adovfexsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3248usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adovferscr(
        &self,
    ) -> &'static crate::common::Reg<self::Adovferscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adovferscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3252usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adovfchscr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adovfchscr0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adovfchscr0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adovfexscr(
        &self,
    ) -> &'static crate::common::Reg<self::Adovfexscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adovfexscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3268usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adfifosr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifosr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adfifosr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3280usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adfifosr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifosr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adfifosr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3284usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adfifosr2(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifosr2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adfifosr2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3288usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adfifosr3(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifosr3_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adfifosr3_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3292usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adfifosr4(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifosr4_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adfifosr4_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3296usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adfifodcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifodcr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adfifodcr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3312usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adfifoersr(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifoersr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adfifoersr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3316usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adfifoerscr(
        &self,
    ) -> &'static crate::common::Reg<self::Adfifoerscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adfifoerscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3320usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmptbsr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmptbsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adcmptbsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3328usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmptbscr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmptbscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adcmptbscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3332usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpchsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpchsr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adcmpchsr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3336usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpexsr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpexsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adcmpexsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3348usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpchscr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpchscr0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adcmpchscr0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3352usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpexscr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpexscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adcmpexscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3364usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adlimgrsr(
        &self,
    ) -> &'static crate::common::Reg<self::Adlimgrsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adlimgrsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3368usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adlimchsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adlimchsr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adlimchsr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3372usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adlimexsr(
        &self,
    ) -> &'static crate::common::Reg<self::Adlimexsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adlimexsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3384usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adlimgrscr(
        &self,
    ) -> &'static crate::common::Reg<self::Adlimgrscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adlimgrscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3388usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adlimchscr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adlimchscr0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adlimchscr0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3392usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adlimexscr(
        &self,
    ) -> &'static crate::common::Reg<self::Adlimexscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adlimexscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3404usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adscanendsr(
        &self,
    ) -> &'static crate::common::Reg<self::Adscanendsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adscanendsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3408usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adscanendscr(
        &self,
    ) -> &'static crate::common::Reg<self::Adscanendscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Adscanendscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3412usize),
            )
        }
    }

    #[inline(always)]
    pub const fn addr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Addr_SPEC, crate::common::R>,
        29,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1000usize))
        }
    }

    #[inline(always)]
    pub const fn adexdr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adexdr_SPEC, crate::common::R>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1194usize))
        }
    }

    #[inline(always)]
    pub const fn adfifodr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adfifodr_SPEC, crate::common::R>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1200usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adclkenr_SPEC;
impl crate::sealed::RegSpec for Adclkenr_SPEC {
    type DataType = u32;
}

pub type Adclkenr = crate::RegValueT<Adclkenr_SPEC>;

impl Adclkenr {
    #[inline(always)]
    pub fn clken(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adclkenr::Clken,
        adclkenr::Clken,
        Adclkenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adclkenr::Clken,
            adclkenr::Clken,
            Adclkenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adclkenr {
    #[inline(always)]
    fn default() -> Adclkenr {
        <crate::RegValueT<Adclkenr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adclkenr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clken_SPEC;
    pub type Clken = crate::EnumBitfieldStruct<u8, Clken_SPEC>;
    impl Clken {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adclksr_SPEC;
impl crate::sealed::RegSpec for Adclksr_SPEC {
    type DataType = u32;
}

pub type Adclksr = crate::RegValueT<Adclksr_SPEC>;

impl Adclksr {
    #[inline(always)]
    pub fn clksr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adclksr::Clksr,
        adclksr::Clksr,
        Adclksr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adclksr::Clksr,
            adclksr::Clksr,
            Adclksr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adclksr {
    #[inline(always)]
    fn default() -> Adclksr {
        <crate::RegValueT<Adclksr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adclksr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clksr_SPEC;
    pub type Clksr = crate::EnumBitfieldStruct<u8, Clksr_SPEC>;
    impl Clksr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adclkcr_SPEC;
impl crate::sealed::RegSpec for Adclkcr_SPEC {
    type DataType = u32;
}

pub type Adclkcr = crate::RegValueT<Adclkcr_SPEC>;

impl Adclkcr {
    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adclkcr::Clksel,
        adclkcr::Clksel,
        Adclkcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adclkcr::Clksel,
            adclkcr::Clksel,
            Adclkcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn divr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        adclkcr::Divr,
        adclkcr::Divr,
        Adclkcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            adclkcr::Divr,
            adclkcr::Divr,
            Adclkcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adclkcr {
    #[inline(always)]
    fn default() -> Adclkcr {
        <crate::RegValueT<Adclkcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adclkcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clksel_SPEC;
    pub type Clksel = crate::EnumBitfieldStruct<u8, Clksel_SPEC>;
    impl Clksel {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Divr_SPEC;
    pub type Divr = crate::EnumBitfieldStruct<u8, Divr_SPEC>;
    impl Divr {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsycr_SPEC;
impl crate::sealed::RegSpec for Adsycr_SPEC {
    type DataType = u32;
}

pub type Adsycr = crate::RegValueT<Adsycr_SPEC>;

impl Adsycr {
    #[inline(always)]
    pub fn adsycyc(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Adsycr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Adsycr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adsydis0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adsycr::Adsydis0,
        adsycr::Adsydis0,
        Adsycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adsycr::Adsydis0,
            adsycr::Adsydis0,
            Adsycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adsydis1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adsycr::Adsydis1,
        adsycr::Adsydis1,
        Adsycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adsycr::Adsydis1,
            adsycr::Adsydis1,
            Adsycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adsycr {
    #[inline(always)]
    fn default() -> Adsycr {
        <crate::RegValueT<Adsycr_SPEC> as RegisterValue<_>>::new(5)
    }
}
pub mod adsycr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsydis0_SPEC;
    pub type Adsydis0 = crate::EnumBitfieldStruct<u8, Adsydis0_SPEC>;
    impl Adsydis0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsydis1_SPEC;
    pub type Adsydis1 = crate::EnumBitfieldStruct<u8, Adsydis1_SPEC>;
    impl Adsydis1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aderintcr_SPEC;
impl crate::sealed::RegSpec for Aderintcr_SPEC {
    type DataType = u32;
}

pub type Aderintcr = crate::RegValueT<Aderintcr_SPEC>;

impl Aderintcr {
    #[inline(always)]
    pub fn adeie0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        aderintcr::Adeie0,
        aderintcr::Adeie0,
        Aderintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            aderintcr::Adeie0,
            aderintcr::Adeie0,
            Aderintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adeie1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        aderintcr::Adeie1,
        aderintcr::Adeie1,
        Aderintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            aderintcr::Adeie1,
            aderintcr::Adeie1,
            Aderintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Aderintcr {
    #[inline(always)]
    fn default() -> Aderintcr {
        <crate::RegValueT<Aderintcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod aderintcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adeie0_SPEC;
    pub type Adeie0 = crate::EnumBitfieldStruct<u8, Adeie0_SPEC>;
    impl Adeie0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adeie1_SPEC;
    pub type Adeie1 = crate::EnumBitfieldStruct<u8, Adeie1_SPEC>;
    impl Adeie1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adovfintcr_SPEC;
impl crate::sealed::RegSpec for Adovfintcr_SPEC {
    type DataType = u32;
}

pub type Adovfintcr = crate::RegValueT<Adovfintcr_SPEC>;

impl Adovfintcr {
    #[inline(always)]
    pub fn adovfie0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adovfintcr::Adovfie0,
        adovfintcr::Adovfie0,
        Adovfintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adovfintcr::Adovfie0,
            adovfintcr::Adovfie0,
            Adovfintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adovfie1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adovfintcr::Adovfie1,
        adovfintcr::Adovfie1,
        Adovfintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adovfintcr::Adovfie1,
            adovfintcr::Adovfie1,
            Adovfintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adovfintcr {
    #[inline(always)]
    fn default() -> Adovfintcr {
        <crate::RegValueT<Adovfintcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adovfintcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adovfie0_SPEC;
    pub type Adovfie0 = crate::EnumBitfieldStruct<u8, Adovfie0_SPEC>;
    impl Adovfie0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adovfie1_SPEC;
    pub type Adovfie1 = crate::EnumBitfieldStruct<u8, Adovfie1_SPEC>;
    impl Adovfie1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcalintcr_SPEC;
impl crate::sealed::RegSpec for Adcalintcr_SPEC {
    type DataType = u32;
}

pub type Adcalintcr = crate::RegValueT<Adcalintcr_SPEC>;

impl Adcalintcr {
    #[inline(always)]
    pub fn calendie0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adcalintcr::Calendie0,
        adcalintcr::Calendie0,
        Adcalintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adcalintcr::Calendie0,
            adcalintcr::Calendie0,
            Adcalintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn calendie1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adcalintcr::Calendie1,
        adcalintcr::Calendie1,
        Adcalintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adcalintcr::Calendie1,
            adcalintcr::Calendie1,
            Adcalintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcalintcr {
    #[inline(always)]
    fn default() -> Adcalintcr {
        <crate::RegValueT<Adcalintcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcalintcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Calendie0_SPEC;
    pub type Calendie0 = crate::EnumBitfieldStruct<u8, Calendie0_SPEC>;
    impl Calendie0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Calendie1_SPEC;
    pub type Calendie1 = crate::EnumBitfieldStruct<u8, Calendie1_SPEC>;
    impl Calendie1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Admdr_SPEC;
impl crate::sealed::RegSpec for Admdr_SPEC {
    type DataType = u32;
}

pub type Admdr = crate::RegValueT<Admdr_SPEC>;

impl Admdr {
    #[inline(always)]
    pub fn admd0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        admdr::Admd0,
        admdr::Admd0,
        Admdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            admdr::Admd0,
            admdr::Admd0,
            Admdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn admd1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        admdr::Admd1,
        admdr::Admd1,
        Admdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            admdr::Admd1,
            admdr::Admd1,
            Admdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Admdr {
    #[inline(always)]
    fn default() -> Admdr {
        <crate::RegValueT<Admdr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod admdr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Admd0_SPEC;
    pub type Admd0 = crate::EnumBitfieldStruct<u8, Admd0_SPEC>;
    impl Admd0 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const _0_X_8: Self = Self::new(8);

        pub const _0_X_9: Self = Self::new(9);

        pub const _0_X_A: Self = Self::new(10);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Admd1_SPEC;
    pub type Admd1 = crate::EnumBitfieldStruct<u8, Admd1_SPEC>;
    impl Admd1 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const _0_X_8: Self = Self::new(8);

        pub const _0_X_9: Self = Self::new(9);

        pub const _0_X_A: Self = Self::new(10);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adgspcr_SPEC;
impl crate::sealed::RegSpec for Adgspcr_SPEC {
    type DataType = u32;
}

pub type Adgspcr = crate::RegValueT<Adgspcr_SPEC>;

impl Adgspcr {
    #[inline(always)]
    pub fn pgs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adgspcr::Pgs0,
        adgspcr::Pgs0,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adgspcr::Pgs0,
            adgspcr::Pgs0,
            Adgspcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rscn0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adgspcr::Rscn0,
        adgspcr::Rscn0,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adgspcr::Rscn0,
            adgspcr::Rscn0,
            Adgspcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lgrrs0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adgspcr::Lgrrs0,
        adgspcr::Lgrrs0,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adgspcr::Lgrrs0,
            adgspcr::Lgrrs0,
            Adgspcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn grp0(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adgspcr::Grp0,
        adgspcr::Grp0,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adgspcr::Grp0,
            adgspcr::Grp0,
            Adgspcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pgs1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adgspcr::Pgs1,
        adgspcr::Pgs1,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adgspcr::Pgs1,
            adgspcr::Pgs1,
            Adgspcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rscn1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adgspcr::Rscn1,
        adgspcr::Rscn1,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adgspcr::Rscn1,
            adgspcr::Rscn1,
            Adgspcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lgrrs1(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adgspcr::Lgrrs1,
        adgspcr::Lgrrs1,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adgspcr::Lgrrs1,
            adgspcr::Lgrrs1,
            Adgspcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn grp1(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adgspcr::Grp1,
        adgspcr::Grp1,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adgspcr::Grp1,
            adgspcr::Grp1,
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
    pub struct Pgs0_SPEC;
    pub type Pgs0 = crate::EnumBitfieldStruct<u8, Pgs0_SPEC>;
    impl Pgs0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscn0_SPEC;
    pub type Rscn0 = crate::EnumBitfieldStruct<u8, Rscn0_SPEC>;
    impl Rscn0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lgrrs0_SPEC;
    pub type Lgrrs0 = crate::EnumBitfieldStruct<u8, Lgrrs0_SPEC>;
    impl Lgrrs0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grp0_SPEC;
    pub type Grp0 = crate::EnumBitfieldStruct<u8, Grp0_SPEC>;
    impl Grp0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgs1_SPEC;
    pub type Pgs1 = crate::EnumBitfieldStruct<u8, Pgs1_SPEC>;
    impl Pgs1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscn1_SPEC;
    pub type Rscn1 = crate::EnumBitfieldStruct<u8, Rscn1_SPEC>;
    impl Rscn1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lgrrs1_SPEC;
    pub type Lgrrs1 = crate::EnumBitfieldStruct<u8, Lgrrs1_SPEC>;
    impl Lgrrs1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grp1_SPEC;
    pub type Grp1 = crate::EnumBitfieldStruct<u8, Grp1_SPEC>;
    impl Grp1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsger_SPEC;
impl crate::sealed::RegSpec for Adsger_SPEC {
    type DataType = u32;
}

pub type Adsger = crate::RegValueT<Adsger_SPEC>;

impl Adsger {
    #[inline(always)]
    pub fn sgre0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adsger::Sgre0,
        adsger::Sgre0,
        Adsger_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adsger::Sgre0,
            adsger::Sgre0,
            Adsger_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgre1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adsger::Sgre1,
        adsger::Sgre1,
        Adsger_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adsger::Sgre1,
            adsger::Sgre1,
            Adsger_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgre2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adsger::Sgre2,
        adsger::Sgre2,
        Adsger_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adsger::Sgre2,
            adsger::Sgre2,
            Adsger_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgre3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adsger::Sgre3,
        adsger::Sgre3,
        Adsger_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adsger::Sgre3,
            adsger::Sgre3,
            Adsger_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgre4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adsger::Sgre4,
        adsger::Sgre4,
        Adsger_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adsger::Sgre4,
            adsger::Sgre4,
            Adsger_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgre5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adsger::Sgre5,
        adsger::Sgre5,
        Adsger_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adsger::Sgre5,
            adsger::Sgre5,
            Adsger_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgre6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adsger::Sgre6,
        adsger::Sgre6,
        Adsger_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adsger::Sgre6,
            adsger::Sgre6,
            Adsger_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgre7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adsger::Sgre7,
        adsger::Sgre7,
        Adsger_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adsger::Sgre7,
            adsger::Sgre7,
            Adsger_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgre8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adsger::Sgre8,
        adsger::Sgre8,
        Adsger_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adsger::Sgre8,
            adsger::Sgre8,
            Adsger_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adsger {
    #[inline(always)]
    fn default() -> Adsger {
        <crate::RegValueT<Adsger_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adsger {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgre0_SPEC;
    pub type Sgre0 = crate::EnumBitfieldStruct<u8, Sgre0_SPEC>;
    impl Sgre0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgre1_SPEC;
    pub type Sgre1 = crate::EnumBitfieldStruct<u8, Sgre1_SPEC>;
    impl Sgre1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgre2_SPEC;
    pub type Sgre2 = crate::EnumBitfieldStruct<u8, Sgre2_SPEC>;
    impl Sgre2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgre3_SPEC;
    pub type Sgre3 = crate::EnumBitfieldStruct<u8, Sgre3_SPEC>;
    impl Sgre3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgre4_SPEC;
    pub type Sgre4 = crate::EnumBitfieldStruct<u8, Sgre4_SPEC>;
    impl Sgre4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgre5_SPEC;
    pub type Sgre5 = crate::EnumBitfieldStruct<u8, Sgre5_SPEC>;
    impl Sgre5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgre6_SPEC;
    pub type Sgre6 = crate::EnumBitfieldStruct<u8, Sgre6_SPEC>;
    impl Sgre6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgre7_SPEC;
    pub type Sgre7 = crate::EnumBitfieldStruct<u8, Sgre7_SPEC>;
    impl Sgre7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgre8_SPEC;
    pub type Sgre8 = crate::EnumBitfieldStruct<u8, Sgre8_SPEC>;
    impl Sgre8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsgcr0_SPEC;
impl crate::sealed::RegSpec for Adsgcr0_SPEC {
    type DataType = u32;
}

pub type Adsgcr0 = crate::RegValueT<Adsgcr0_SPEC>;

impl Adsgcr0 {
    #[inline(always)]
    pub fn sgads0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adsgcr0::Sgads0,
        adsgcr0::Sgads0,
        Adsgcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adsgcr0::Sgads0,
            adsgcr0::Sgads0,
            Adsgcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgads1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        adsgcr0::Sgads1,
        adsgcr0::Sgads1,
        Adsgcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            adsgcr0::Sgads1,
            adsgcr0::Sgads1,
            Adsgcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgads2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        adsgcr0::Sgads2,
        adsgcr0::Sgads2,
        Adsgcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            adsgcr0::Sgads2,
            adsgcr0::Sgads2,
            Adsgcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgads3(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        adsgcr0::Sgads3,
        adsgcr0::Sgads3,
        Adsgcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            adsgcr0::Sgads3,
            adsgcr0::Sgads3,
            Adsgcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adsgcr0 {
    #[inline(always)]
    fn default() -> Adsgcr0 {
        <crate::RegValueT<Adsgcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adsgcr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgads0_SPEC;
    pub type Sgads0 = crate::EnumBitfieldStruct<u8, Sgads0_SPEC>;
    impl Sgads0 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgads1_SPEC;
    pub type Sgads1 = crate::EnumBitfieldStruct<u8, Sgads1_SPEC>;
    impl Sgads1 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgads2_SPEC;
    pub type Sgads2 = crate::EnumBitfieldStruct<u8, Sgads2_SPEC>;
    impl Sgads2 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgads3_SPEC;
    pub type Sgads3 = crate::EnumBitfieldStruct<u8, Sgads3_SPEC>;
    impl Sgads3 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsgcr1_SPEC;
impl crate::sealed::RegSpec for Adsgcr1_SPEC {
    type DataType = u32;
}

pub type Adsgcr1 = crate::RegValueT<Adsgcr1_SPEC>;

impl Adsgcr1 {
    #[inline(always)]
    pub fn sgads4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adsgcr1::Sgads4,
        adsgcr1::Sgads4,
        Adsgcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adsgcr1::Sgads4,
            adsgcr1::Sgads4,
            Adsgcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgads5(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        adsgcr1::Sgads5,
        adsgcr1::Sgads5,
        Adsgcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            adsgcr1::Sgads5,
            adsgcr1::Sgads5,
            Adsgcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgads6(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        adsgcr1::Sgads6,
        adsgcr1::Sgads6,
        Adsgcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            adsgcr1::Sgads6,
            adsgcr1::Sgads6,
            Adsgcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgads7(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        adsgcr1::Sgads7,
        adsgcr1::Sgads7,
        Adsgcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            adsgcr1::Sgads7,
            adsgcr1::Sgads7,
            Adsgcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adsgcr1 {
    #[inline(always)]
    fn default() -> Adsgcr1 {
        <crate::RegValueT<Adsgcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adsgcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgads4_SPEC;
    pub type Sgads4 = crate::EnumBitfieldStruct<u8, Sgads4_SPEC>;
    impl Sgads4 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgads5_SPEC;
    pub type Sgads5 = crate::EnumBitfieldStruct<u8, Sgads5_SPEC>;
    impl Sgads5 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgads6_SPEC;
    pub type Sgads6 = crate::EnumBitfieldStruct<u8, Sgads6_SPEC>;
    impl Sgads6 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgads7_SPEC;
    pub type Sgads7 = crate::EnumBitfieldStruct<u8, Sgads7_SPEC>;
    impl Sgads7 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsgcr2_SPEC;
impl crate::sealed::RegSpec for Adsgcr2_SPEC {
    type DataType = u32;
}

pub type Adsgcr2 = crate::RegValueT<Adsgcr2_SPEC>;

impl Adsgcr2 {
    #[inline(always)]
    pub fn sgads8(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adsgcr2::Sgads8,
        adsgcr2::Sgads8,
        Adsgcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adsgcr2::Sgads8,
            adsgcr2::Sgads8,
            Adsgcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adsgcr2 {
    #[inline(always)]
    fn default() -> Adsgcr2 {
        <crate::RegValueT<Adsgcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adsgcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sgads8_SPEC;
    pub type Sgads8 = crate::EnumBitfieldStruct<u8, Sgads8_SPEC>;
    impl Sgads8 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adintcr_SPEC;
impl crate::sealed::RegSpec for Adintcr_SPEC {
    type DataType = u32;
}

pub type Adintcr = crate::RegValueT<Adintcr_SPEC>;

impl Adintcr {
    #[inline(always)]
    pub fn adie0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adintcr::Adie0,
        adintcr::Adie0,
        Adintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adintcr::Adie0,
            adintcr::Adie0,
            Adintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adie1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adintcr::Adie1,
        adintcr::Adie1,
        Adintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adintcr::Adie1,
            adintcr::Adie1,
            Adintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adie2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adintcr::Adie2,
        adintcr::Adie2,
        Adintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adintcr::Adie2,
            adintcr::Adie2,
            Adintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adie3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adintcr::Adie3,
        adintcr::Adie3,
        Adintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adintcr::Adie3,
            adintcr::Adie3,
            Adintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adie4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adintcr::Adie4,
        adintcr::Adie4,
        Adintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adintcr::Adie4,
            adintcr::Adie4,
            Adintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adie5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adintcr::Adie5,
        adintcr::Adie5,
        Adintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adintcr::Adie5,
            adintcr::Adie5,
            Adintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adie6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adintcr::Adie6,
        adintcr::Adie6,
        Adintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adintcr::Adie6,
            adintcr::Adie6,
            Adintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adie7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adintcr::Adie7,
        adintcr::Adie7,
        Adintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adintcr::Adie7,
            adintcr::Adie7,
            Adintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adie8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adintcr::Adie8,
        adintcr::Adie8,
        Adintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adintcr::Adie8,
            adintcr::Adie8,
            Adintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adintcr {
    #[inline(always)]
    fn default() -> Adintcr {
        <crate::RegValueT<Adintcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adintcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adie0_SPEC;
    pub type Adie0 = crate::EnumBitfieldStruct<u8, Adie0_SPEC>;
    impl Adie0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adie1_SPEC;
    pub type Adie1 = crate::EnumBitfieldStruct<u8, Adie1_SPEC>;
    impl Adie1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adie2_SPEC;
    pub type Adie2 = crate::EnumBitfieldStruct<u8, Adie2_SPEC>;
    impl Adie2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adie3_SPEC;
    pub type Adie3 = crate::EnumBitfieldStruct<u8, Adie3_SPEC>;
    impl Adie3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adie4_SPEC;
    pub type Adie4 = crate::EnumBitfieldStruct<u8, Adie4_SPEC>;
    impl Adie4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adie5_SPEC;
    pub type Adie5 = crate::EnumBitfieldStruct<u8, Adie5_SPEC>;
    impl Adie5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adie6_SPEC;
    pub type Adie6 = crate::EnumBitfieldStruct<u8, Adie6_SPEC>;
    impl Adie6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adie7_SPEC;
    pub type Adie7 = crate::EnumBitfieldStruct<u8, Adie7_SPEC>;
    impl Adie7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adie8_SPEC;
    pub type Adie8 = crate::EnumBitfieldStruct<u8, Adie8_SPEC>;
    impl Adie8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adtrgext_SPEC;
impl crate::sealed::RegSpec for Adtrgext_SPEC {
    type DataType = u32;
}

pub type Adtrgext = crate::RegValueT<Adtrgext_SPEC>;

impl Adtrgext {
    #[inline(always)]
    pub fn trgext0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adtrgext::Trgext0,
        adtrgext::Trgext0,
        Adtrgext_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adtrgext::Trgext0,
            adtrgext::Trgext0,
            Adtrgext_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trgext1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adtrgext::Trgext1,
        adtrgext::Trgext1,
        Adtrgext_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adtrgext::Trgext1,
            adtrgext::Trgext1,
            Adtrgext_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adtrgext {
    #[inline(always)]
    fn default() -> Adtrgext {
        <crate::RegValueT<Adtrgext_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adtrgext {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trgext0_SPEC;
    pub type Trgext0 = crate::EnumBitfieldStruct<u8, Trgext0_SPEC>;
    impl Trgext0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trgext1_SPEC;
    pub type Trgext1 = crate::EnumBitfieldStruct<u8, Trgext1_SPEC>;
    impl Trgext1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adtrgelc_SPEC;
impl crate::sealed::RegSpec for Adtrgelc_SPEC {
    type DataType = u32;
}

pub type Adtrgelc = crate::RegValueT<Adtrgelc_SPEC>;

impl Adtrgelc {
    #[inline(always)]
    pub fn trgelc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adtrgelc::Trgelc0,
        adtrgelc::Trgelc0,
        Adtrgelc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adtrgelc::Trgelc0,
            adtrgelc::Trgelc0,
            Adtrgelc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trgelc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adtrgelc::Trgelc1,
        adtrgelc::Trgelc1,
        Adtrgelc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adtrgelc::Trgelc1,
            adtrgelc::Trgelc1,
            Adtrgelc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trgelc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adtrgelc::Trgelc2,
        adtrgelc::Trgelc2,
        Adtrgelc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adtrgelc::Trgelc2,
            adtrgelc::Trgelc2,
            Adtrgelc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trgelc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adtrgelc::Trgelc3,
        adtrgelc::Trgelc3,
        Adtrgelc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adtrgelc::Trgelc3,
            adtrgelc::Trgelc3,
            Adtrgelc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trgelc4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adtrgelc::Trgelc4,
        adtrgelc::Trgelc4,
        Adtrgelc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adtrgelc::Trgelc4,
            adtrgelc::Trgelc4,
            Adtrgelc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trgelc5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adtrgelc::Trgelc5,
        adtrgelc::Trgelc5,
        Adtrgelc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adtrgelc::Trgelc5,
            adtrgelc::Trgelc5,
            Adtrgelc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adtrgelc {
    #[inline(always)]
    fn default() -> Adtrgelc {
        <crate::RegValueT<Adtrgelc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adtrgelc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trgelc0_SPEC;
    pub type Trgelc0 = crate::EnumBitfieldStruct<u8, Trgelc0_SPEC>;
    impl Trgelc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trgelc1_SPEC;
    pub type Trgelc1 = crate::EnumBitfieldStruct<u8, Trgelc1_SPEC>;
    impl Trgelc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trgelc2_SPEC;
    pub type Trgelc2 = crate::EnumBitfieldStruct<u8, Trgelc2_SPEC>;
    impl Trgelc2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trgelc3_SPEC;
    pub type Trgelc3 = crate::EnumBitfieldStruct<u8, Trgelc3_SPEC>;
    impl Trgelc3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trgelc4_SPEC;
    pub type Trgelc4 = crate::EnumBitfieldStruct<u8, Trgelc4_SPEC>;
    impl Trgelc4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trgelc5_SPEC;
    pub type Trgelc5 = crate::EnumBitfieldStruct<u8, Trgelc5_SPEC>;
    impl Trgelc5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adtrggpt_SPEC;
impl crate::sealed::RegSpec for Adtrggpt_SPEC {
    type DataType = u32;
}

pub type Adtrggpt = crate::RegValueT<Adtrggpt_SPEC>;

impl Adtrggpt {
    #[inline(always)]
    pub fn trggpta0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adtrggpt::Trggpta0,
        adtrggpt::Trggpta0,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adtrggpt::Trggpta0,
            adtrggpt::Trggpta0,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggpta1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adtrggpt::Trggpta1,
        adtrggpt::Trggpta1,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adtrggpt::Trggpta1,
            adtrggpt::Trggpta1,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggpta2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adtrggpt::Trggpta2,
        adtrggpt::Trggpta2,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adtrggpt::Trggpta2,
            adtrggpt::Trggpta2,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggpta3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adtrggpt::Trggpta3,
        adtrggpt::Trggpta3,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adtrggpt::Trggpta3,
            adtrggpt::Trggpta3,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggpta4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adtrggpt::Trggpta4,
        adtrggpt::Trggpta4,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adtrggpt::Trggpta4,
            adtrggpt::Trggpta4,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggpta5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adtrggpt::Trggpta5,
        adtrggpt::Trggpta5,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adtrggpt::Trggpta5,
            adtrggpt::Trggpta5,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggpta6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adtrggpt::Trggpta6,
        adtrggpt::Trggpta6,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adtrggpt::Trggpta6,
            adtrggpt::Trggpta6,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggpta7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adtrggpt::Trggpta7,
        adtrggpt::Trggpta7,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adtrggpt::Trggpta7,
            adtrggpt::Trggpta7,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggpta8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adtrggpt::Trggpta8,
        adtrggpt::Trggpta8,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adtrggpt::Trggpta8,
            adtrggpt::Trggpta8,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggpta9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adtrggpt::Trggpta9,
        adtrggpt::Trggpta9,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adtrggpt::Trggpta9,
            adtrggpt::Trggpta9,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggptb0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adtrggpt::Trggptb0,
        adtrggpt::Trggptb0,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adtrggpt::Trggptb0,
            adtrggpt::Trggptb0,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggptb1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adtrggpt::Trggptb1,
        adtrggpt::Trggptb1,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adtrggpt::Trggptb1,
            adtrggpt::Trggptb1,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggptb2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        adtrggpt::Trggptb2,
        adtrggpt::Trggptb2,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            adtrggpt::Trggptb2,
            adtrggpt::Trggptb2,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggptb3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        adtrggpt::Trggptb3,
        adtrggpt::Trggptb3,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            adtrggpt::Trggptb3,
            adtrggpt::Trggptb3,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggptb4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        adtrggpt::Trggptb4,
        adtrggpt::Trggptb4,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            adtrggpt::Trggptb4,
            adtrggpt::Trggptb4,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggptb5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        adtrggpt::Trggptb5,
        adtrggpt::Trggptb5,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            adtrggpt::Trggptb5,
            adtrggpt::Trggptb5,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggptb6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        adtrggpt::Trggptb6,
        adtrggpt::Trggptb6,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            adtrggpt::Trggptb6,
            adtrggpt::Trggptb6,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggptb7(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        adtrggpt::Trggptb7,
        adtrggpt::Trggptb7,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            adtrggpt::Trggptb7,
            adtrggpt::Trggptb7,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggptb8(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        adtrggpt::Trggptb8,
        adtrggpt::Trggptb8,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            adtrggpt::Trggptb8,
            adtrggpt::Trggptb8,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trggptb9(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        adtrggpt::Trggptb9,
        adtrggpt::Trggptb9,
        Adtrggpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            adtrggpt::Trggptb9,
            adtrggpt::Trggptb9,
            Adtrggpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adtrggpt {
    #[inline(always)]
    fn default() -> Adtrggpt {
        <crate::RegValueT<Adtrggpt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adtrggpt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggpta0_SPEC;
    pub type Trggpta0 = crate::EnumBitfieldStruct<u8, Trggpta0_SPEC>;
    impl Trggpta0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggpta1_SPEC;
    pub type Trggpta1 = crate::EnumBitfieldStruct<u8, Trggpta1_SPEC>;
    impl Trggpta1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggpta2_SPEC;
    pub type Trggpta2 = crate::EnumBitfieldStruct<u8, Trggpta2_SPEC>;
    impl Trggpta2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggpta3_SPEC;
    pub type Trggpta3 = crate::EnumBitfieldStruct<u8, Trggpta3_SPEC>;
    impl Trggpta3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggpta4_SPEC;
    pub type Trggpta4 = crate::EnumBitfieldStruct<u8, Trggpta4_SPEC>;
    impl Trggpta4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggpta5_SPEC;
    pub type Trggpta5 = crate::EnumBitfieldStruct<u8, Trggpta5_SPEC>;
    impl Trggpta5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggpta6_SPEC;
    pub type Trggpta6 = crate::EnumBitfieldStruct<u8, Trggpta6_SPEC>;
    impl Trggpta6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggpta7_SPEC;
    pub type Trggpta7 = crate::EnumBitfieldStruct<u8, Trggpta7_SPEC>;
    impl Trggpta7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggpta8_SPEC;
    pub type Trggpta8 = crate::EnumBitfieldStruct<u8, Trggpta8_SPEC>;
    impl Trggpta8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggpta9_SPEC;
    pub type Trggpta9 = crate::EnumBitfieldStruct<u8, Trggpta9_SPEC>;
    impl Trggpta9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggptb0_SPEC;
    pub type Trggptb0 = crate::EnumBitfieldStruct<u8, Trggptb0_SPEC>;
    impl Trggptb0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggptb1_SPEC;
    pub type Trggptb1 = crate::EnumBitfieldStruct<u8, Trggptb1_SPEC>;
    impl Trggptb1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggptb2_SPEC;
    pub type Trggptb2 = crate::EnumBitfieldStruct<u8, Trggptb2_SPEC>;
    impl Trggptb2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggptb3_SPEC;
    pub type Trggptb3 = crate::EnumBitfieldStruct<u8, Trggptb3_SPEC>;
    impl Trggptb3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggptb4_SPEC;
    pub type Trggptb4 = crate::EnumBitfieldStruct<u8, Trggptb4_SPEC>;
    impl Trggptb4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggptb5_SPEC;
    pub type Trggptb5 = crate::EnumBitfieldStruct<u8, Trggptb5_SPEC>;
    impl Trggptb5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggptb6_SPEC;
    pub type Trggptb6 = crate::EnumBitfieldStruct<u8, Trggptb6_SPEC>;
    impl Trggptb6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggptb7_SPEC;
    pub type Trggptb7 = crate::EnumBitfieldStruct<u8, Trggptb7_SPEC>;
    impl Trggptb7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggptb8_SPEC;
    pub type Trggptb8 = crate::EnumBitfieldStruct<u8, Trggptb8_SPEC>;
    impl Trggptb8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trggptb9_SPEC;
    pub type Trggptb9 = crate::EnumBitfieldStruct<u8, Trggptb9_SPEC>;
    impl Trggptb9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adtrgdlr0_SPEC;
impl crate::sealed::RegSpec for Adtrgdlr0_SPEC {
    type DataType = u32;
}

pub type Adtrgdlr0 = crate::RegValueT<Adtrgdlr0_SPEC>;

impl Adtrgdlr0 {
    #[inline(always)]
    pub fn trgdly0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adtrgdlr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adtrgdlr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn trgdly1(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Adtrgdlr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Adtrgdlr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adtrgdlr0 {
    #[inline(always)]
    fn default() -> Adtrgdlr0 {
        <crate::RegValueT<Adtrgdlr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adtrgdlr1_SPEC;
impl crate::sealed::RegSpec for Adtrgdlr1_SPEC {
    type DataType = u32;
}

pub type Adtrgdlr1 = crate::RegValueT<Adtrgdlr1_SPEC>;

impl Adtrgdlr1 {
    #[inline(always)]
    pub fn trgdly2(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adtrgdlr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adtrgdlr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn trgdly3(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Adtrgdlr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Adtrgdlr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adtrgdlr1 {
    #[inline(always)]
    fn default() -> Adtrgdlr1 {
        <crate::RegValueT<Adtrgdlr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adtrgdlr2_SPEC;
impl crate::sealed::RegSpec for Adtrgdlr2_SPEC {
    type DataType = u32;
}

pub type Adtrgdlr2 = crate::RegValueT<Adtrgdlr2_SPEC>;

impl Adtrgdlr2 {
    #[inline(always)]
    pub fn trgdly4(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adtrgdlr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adtrgdlr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn trgdly5(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Adtrgdlr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Adtrgdlr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adtrgdlr2 {
    #[inline(always)]
    fn default() -> Adtrgdlr2 {
        <crate::RegValueT<Adtrgdlr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adtrgdlr3_SPEC;
impl crate::sealed::RegSpec for Adtrgdlr3_SPEC {
    type DataType = u32;
}

pub type Adtrgdlr3 = crate::RegValueT<Adtrgdlr3_SPEC>;

impl Adtrgdlr3 {
    #[inline(always)]
    pub fn trgdly6(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adtrgdlr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adtrgdlr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn trgdly7(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Adtrgdlr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Adtrgdlr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adtrgdlr3 {
    #[inline(always)]
    fn default() -> Adtrgdlr3 {
        <crate::RegValueT<Adtrgdlr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adtrgdlr4_SPEC;
impl crate::sealed::RegSpec for Adtrgdlr4_SPEC {
    type DataType = u32;
}

pub type Adtrgdlr4 = crate::RegValueT<Adtrgdlr4_SPEC>;

impl Adtrgdlr4 {
    #[inline(always)]
    pub fn trgdly8(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adtrgdlr4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adtrgdlr4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adtrgdlr4 {
    #[inline(always)]
    fn default() -> Adtrgdlr4 {
        <crate::RegValueT<Adtrgdlr4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsgdcr_SPEC;
impl crate::sealed::RegSpec for Adsgdcr_SPEC {
    type DataType = u32;
}

pub type Adsgdcr = crate::RegValueT<Adsgdcr_SPEC>;

impl Adsgdcr {
    #[inline(always)]
    pub fn diagval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        adsgdcr::Diagval,
        adsgdcr::Diagval,
        Adsgdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            adsgdcr::Diagval,
            adsgdcr::Diagval,
            Adsgdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn addisen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adsgdcr::Addisen,
        adsgdcr::Addisen,
        Adsgdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adsgdcr::Addisen,
            adsgdcr::Addisen,
            Adsgdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn addisp(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        adsgdcr::Addisp,
        adsgdcr::Addisp,
        Adsgdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            adsgdcr::Addisp,
            adsgdcr::Addisp,
            Adsgdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn addisn(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        adsgdcr::Addisn,
        adsgdcr::Addisn,
        Adsgdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            adsgdcr::Addisn,
            adsgdcr::Addisn,
            Adsgdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adndis(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        adsgdcr::Adndis,
        adsgdcr::Adndis,
        Adsgdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            adsgdcr::Adndis,
            adsgdcr::Adndis,
            Adsgdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adsgdcr {
    #[inline(always)]
    fn default() -> Adsgdcr {
        <crate::RegValueT<Adsgdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adsgdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diagval_SPEC;
    pub type Diagval = crate::EnumBitfieldStruct<u8, Diagval_SPEC>;
    impl Diagval {
        pub const _000: Self = Self::new(0);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Addisen_SPEC;
    pub type Addisen = crate::EnumBitfieldStruct<u8, Addisen_SPEC>;
    impl Addisen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Addisp_SPEC;
    pub type Addisp = crate::EnumBitfieldStruct<u8, Addisp_SPEC>;
    impl Addisp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Addisn_SPEC;
    pub type Addisn = crate::EnumBitfieldStruct<u8, Addisn_SPEC>;
    impl Addisn {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adndis_SPEC;
    pub type Adndis = crate::EnumBitfieldStruct<u8, Adndis_SPEC>;
    impl Adndis {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstr0_SPEC;
impl crate::sealed::RegSpec for Adsstr0_SPEC {
    type DataType = u32;
}

pub type Adsstr0 = crate::RegValueT<Adsstr0_SPEC>;

impl Adsstr0 {
    #[inline(always)]
    pub fn sst0(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Adsstr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Adsstr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sst1(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Adsstr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Adsstr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstr0 {
    #[inline(always)]
    fn default() -> Adsstr0 {
        <crate::RegValueT<Adsstr0_SPEC> as RegisterValue<_>>::new(131074)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstr1_SPEC;
impl crate::sealed::RegSpec for Adsstr1_SPEC {
    type DataType = u32;
}

pub type Adsstr1 = crate::RegValueT<Adsstr1_SPEC>;

impl Adsstr1 {
    #[inline(always)]
    pub fn sst2(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Adsstr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Adsstr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sst3(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Adsstr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Adsstr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstr1 {
    #[inline(always)]
    fn default() -> Adsstr1 {
        <crate::RegValueT<Adsstr1_SPEC> as RegisterValue<_>>::new(131074)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstr2_SPEC;
impl crate::sealed::RegSpec for Adsstr2_SPEC {
    type DataType = u32;
}

pub type Adsstr2 = crate::RegValueT<Adsstr2_SPEC>;

impl Adsstr2 {
    #[inline(always)]
    pub fn sst4(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Adsstr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Adsstr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sst5(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Adsstr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Adsstr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstr2 {
    #[inline(always)]
    fn default() -> Adsstr2 {
        <crate::RegValueT<Adsstr2_SPEC> as RegisterValue<_>>::new(131074)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstr3_SPEC;
impl crate::sealed::RegSpec for Adsstr3_SPEC {
    type DataType = u32;
}

pub type Adsstr3 = crate::RegValueT<Adsstr3_SPEC>;

impl Adsstr3 {
    #[inline(always)]
    pub fn sst6(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Adsstr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Adsstr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sst7(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Adsstr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Adsstr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstr3 {
    #[inline(always)]
    fn default() -> Adsstr3 {
        <crate::RegValueT<Adsstr3_SPEC> as RegisterValue<_>>::new(131074)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstr4_SPEC;
impl crate::sealed::RegSpec for Adsstr4_SPEC {
    type DataType = u32;
}

pub type Adsstr4 = crate::RegValueT<Adsstr4_SPEC>;

impl Adsstr4 {
    #[inline(always)]
    pub fn sst8(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Adsstr4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Adsstr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sst9(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Adsstr4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Adsstr4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstr4 {
    #[inline(always)]
    fn default() -> Adsstr4 {
        <crate::RegValueT<Adsstr4_SPEC> as RegisterValue<_>>::new(131074)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstr5_SPEC;
impl crate::sealed::RegSpec for Adsstr5_SPEC {
    type DataType = u32;
}

pub type Adsstr5 = crate::RegValueT<Adsstr5_SPEC>;

impl Adsstr5 {
    #[inline(always)]
    pub fn sst10(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Adsstr5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Adsstr5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sst11(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Adsstr5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Adsstr5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstr5 {
    #[inline(always)]
    fn default() -> Adsstr5 {
        <crate::RegValueT<Adsstr5_SPEC> as RegisterValue<_>>::new(131074)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstr6_SPEC;
impl crate::sealed::RegSpec for Adsstr6_SPEC {
    type DataType = u32;
}

pub type Adsstr6 = crate::RegValueT<Adsstr6_SPEC>;

impl Adsstr6 {
    #[inline(always)]
    pub fn sst12(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Adsstr6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Adsstr6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sst13(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Adsstr6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Adsstr6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstr6 {
    #[inline(always)]
    fn default() -> Adsstr6 {
        <crate::RegValueT<Adsstr6_SPEC> as RegisterValue<_>>::new(131074)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstr7_SPEC;
impl crate::sealed::RegSpec for Adsstr7_SPEC {
    type DataType = u32;
}

pub type Adsstr7 = crate::RegValueT<Adsstr7_SPEC>;

impl Adsstr7 {
    #[inline(always)]
    pub fn sst14(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Adsstr7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Adsstr7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sst15(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Adsstr7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Adsstr7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstr7 {
    #[inline(always)]
    fn default() -> Adsstr7 {
        <crate::RegValueT<Adsstr7_SPEC> as RegisterValue<_>>::new(131074)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcnvstr_SPEC;
impl crate::sealed::RegSpec for Adcnvstr_SPEC {
    type DataType = u32;
}

pub type Adcnvstr = crate::RegValueT<Adcnvstr_SPEC>;

impl Adcnvstr {
    #[inline(always)]
    pub fn cst0(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Adcnvstr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Adcnvstr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cst1(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Adcnvstr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Adcnvstr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adcnvstr {
    #[inline(always)]
    fn default() -> Adcnvstr {
        <crate::RegValueT<Adcnvstr_SPEC> as RegisterValue<_>>::new(771)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcalstcr_SPEC;
impl crate::sealed::RegSpec for Adcalstcr_SPEC {
    type DataType = u32;
}

pub type Adcalstcr = crate::RegValueT<Adcalstcr_SPEC>;

impl Adcalstcr {
    #[inline(always)]
    pub fn caladsst(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Adcalstcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Adcalstcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn caladcst(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Adcalstcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,Adcalstcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adcalstcr {
    #[inline(always)]
    fn default() -> Adcalstcr {
        <crate::RegValueT<Adcalstcr_SPEC> as RegisterValue<_>>::new(196610)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adshcr0_SPEC;
impl crate::sealed::RegSpec for Adshcr0_SPEC {
    type DataType = u32;
}

pub type Adshcr0 = crate::RegValueT<Adshcr0_SPEC>;

impl Adshcr0 {
    #[inline(always)]
    pub fn shen0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adshcr0::Shen0,
        adshcr0::Shen0,
        Adshcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adshcr0::Shen0,
            adshcr0::Shen0,
            Adshcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn shen1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adshcr0::Shen1,
        adshcr0::Shen1,
        Adshcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adshcr0::Shen1,
            adshcr0::Shen1,
            Adshcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn shen2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adshcr0::Shen2,
        adshcr0::Shen2,
        Adshcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adshcr0::Shen2,
            adshcr0::Shen2,
            Adshcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn shmd0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adshcr0::Shmd0,
        adshcr0::Shmd0,
        Adshcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adshcr0::Shmd0,
            adshcr0::Shmd0,
            Adshcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn shmd1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adshcr0::Shmd1,
        adshcr0::Shmd1,
        Adshcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adshcr0::Shmd1,
            adshcr0::Shmd1,
            Adshcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn shmd2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        adshcr0::Shmd2,
        adshcr0::Shmd2,
        Adshcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            adshcr0::Shmd2,
            adshcr0::Shmd2,
            Adshcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adshcr0 {
    #[inline(always)]
    fn default() -> Adshcr0 {
        <crate::RegValueT<Adshcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adshcr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shen0_SPEC;
    pub type Shen0 = crate::EnumBitfieldStruct<u8, Shen0_SPEC>;
    impl Shen0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shen1_SPEC;
    pub type Shen1 = crate::EnumBitfieldStruct<u8, Shen1_SPEC>;
    impl Shen1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shen2_SPEC;
    pub type Shen2 = crate::EnumBitfieldStruct<u8, Shen2_SPEC>;
    impl Shen2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shmd0_SPEC;
    pub type Shmd0 = crate::EnumBitfieldStruct<u8, Shmd0_SPEC>;
    impl Shmd0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shmd1_SPEC;
    pub type Shmd1 = crate::EnumBitfieldStruct<u8, Shmd1_SPEC>;
    impl Shmd1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shmd2_SPEC;
    pub type Shmd2 = crate::EnumBitfieldStruct<u8, Shmd2_SPEC>;
    impl Shmd2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adshstr0_SPEC;
impl crate::sealed::RegSpec for Adshstr0_SPEC {
    type DataType = u32;
}

pub type Adshstr0 = crate::RegValueT<Adshstr0_SPEC>;

impl Adshstr0 {
    #[inline(always)]
    pub fn shsst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adshstr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adshstr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn shhst(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Adshstr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Adshstr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adshstr0 {
    #[inline(always)]
    fn default() -> Adshstr0 {
        <crate::RegValueT<Adshstr0_SPEC> as RegisterValue<_>>::new(131076)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adshcr1_SPEC;
impl crate::sealed::RegSpec for Adshcr1_SPEC {
    type DataType = u32;
}

pub type Adshcr1 = crate::RegValueT<Adshcr1_SPEC>;

impl Adshcr1 {
    #[inline(always)]
    pub fn shen4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adshcr1::Shen4,
        adshcr1::Shen4,
        Adshcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adshcr1::Shen4,
            adshcr1::Shen4,
            Adshcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn shen5(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adshcr1::Shen5,
        adshcr1::Shen5,
        Adshcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adshcr1::Shen5,
            adshcr1::Shen5,
            Adshcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn shen6(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adshcr1::Shen6,
        adshcr1::Shen6,
        Adshcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adshcr1::Shen6,
            adshcr1::Shen6,
            Adshcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn shmd4(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adshcr1::Shmd4,
        adshcr1::Shmd4,
        Adshcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adshcr1::Shmd4,
            adshcr1::Shmd4,
            Adshcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn shmd5(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adshcr1::Shmd5,
        adshcr1::Shmd5,
        Adshcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adshcr1::Shmd5,
            adshcr1::Shmd5,
            Adshcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn shmd6(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        adshcr1::Shmd6,
        adshcr1::Shmd6,
        Adshcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            adshcr1::Shmd6,
            adshcr1::Shmd6,
            Adshcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adshcr1 {
    #[inline(always)]
    fn default() -> Adshcr1 {
        <crate::RegValueT<Adshcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adshcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shen4_SPEC;
    pub type Shen4 = crate::EnumBitfieldStruct<u8, Shen4_SPEC>;
    impl Shen4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shen5_SPEC;
    pub type Shen5 = crate::EnumBitfieldStruct<u8, Shen5_SPEC>;
    impl Shen5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shen6_SPEC;
    pub type Shen6 = crate::EnumBitfieldStruct<u8, Shen6_SPEC>;
    impl Shen6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shmd4_SPEC;
    pub type Shmd4 = crate::EnumBitfieldStruct<u8, Shmd4_SPEC>;
    impl Shmd4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shmd5_SPEC;
    pub type Shmd5 = crate::EnumBitfieldStruct<u8, Shmd5_SPEC>;
    impl Shmd5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shmd6_SPEC;
    pub type Shmd6 = crate::EnumBitfieldStruct<u8, Shmd6_SPEC>;
    impl Shmd6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adshstr1_SPEC;
impl crate::sealed::RegSpec for Adshstr1_SPEC {
    type DataType = u32;
}

pub type Adshstr1 = crate::RegValueT<Adshstr1_SPEC>;

impl Adshstr1 {
    #[inline(always)]
    pub fn shsst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adshstr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adshstr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn shhst(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Adshstr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Adshstr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adshstr1 {
    #[inline(always)]
    fn default() -> Adshstr1 {
        <crate::RegValueT<Adshstr1_SPEC> as RegisterValue<_>>::new(131076)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcalshcr_SPEC;
impl crate::sealed::RegSpec for Adcalshcr_SPEC {
    type DataType = u32;
}

pub type Adcalshcr = crate::RegValueT<Adcalshcr_SPEC>;

impl Adcalshcr {
    #[inline(always)]
    pub fn calshsst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adcalshcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adcalshcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn calshhst(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Adcalshcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Adcalshcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adcalshcr {
    #[inline(always)]
    fn default() -> Adcalshcr {
        <crate::RegValueT<Adcalshcr_SPEC> as RegisterValue<_>>::new(131076)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adpgacr_SPEC;
impl crate::sealed::RegSpec for Adpgacr_SPEC {
    type DataType = u32;
}

pub type Adpgacr = crate::RegValueT<Adpgacr_SPEC>;

impl Adpgacr {
    #[inline(always)]
    pub fn pgaden(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adpgacr::Pgaden,
        adpgacr::Pgaden,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adpgacr::Pgaden,
            adpgacr::Pgaden,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pgasel1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adpgacr::Pgasel1,
        adpgacr::Pgasel1,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adpgacr::Pgasel1,
            adpgacr::Pgasel1,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pgaenamp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adpgacr::Pgaenamp,
        adpgacr::Pgaenamp,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adpgacr::Pgaenamp,
            adpgacr::Pgaenamp,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pgagen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adpgacr::Pgagen,
        adpgacr::Pgagen,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adpgacr::Pgagen,
            adpgacr::Pgagen,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pgadg(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        adpgacr::Pgadg,
        adpgacr::Pgadg,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            adpgacr::Pgadg,
            adpgacr::Pgadg,
            Adpgacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pgagain(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        adpgacr::Pgagain,
        adpgacr::Pgagain,
        Adpgacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            adpgacr::Pgagain,
            adpgacr::Pgagain,
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
    pub struct Pgaden_SPEC;
    pub type Pgaden = crate::EnumBitfieldStruct<u8, Pgaden_SPEC>;
    impl Pgaden {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgasel1_SPEC;
    pub type Pgasel1 = crate::EnumBitfieldStruct<u8, Pgasel1_SPEC>;
    impl Pgasel1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgaenamp_SPEC;
    pub type Pgaenamp = crate::EnumBitfieldStruct<u8, Pgaenamp_SPEC>;
    impl Pgaenamp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgagen_SPEC;
    pub type Pgagen = crate::EnumBitfieldStruct<u8, Pgagen_SPEC>;
    impl Pgagen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgadg_SPEC;
    pub type Pgadg = crate::EnumBitfieldStruct<u8, Pgadg_SPEC>;
    impl Pgadg {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgagain_SPEC;
    pub type Pgagain = crate::EnumBitfieldStruct<u8, Pgagain_SPEC>;
    impl Pgagain {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const _0_X_6: Self = Self::new(6);

        pub const _0_X_7: Self = Self::new(7);

        pub const _0_X_8: Self = Self::new(8);

        pub const _0_X_9: Self = Self::new(9);

        pub const _0_X_A: Self = Self::new(10);

        pub const _0_X_B: Self = Self::new(11);

        pub const _0_X_C: Self = Self::new(12);

        pub const _0_X_D: Self = Self::new(13);

        pub const _0_X_E: Self = Self::new(14);

        pub const _0_X_F: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adpgamoncr_SPEC;
impl crate::sealed::RegSpec for Adpgamoncr_SPEC {
    type DataType = u32;
}

pub type Adpgamoncr = crate::RegValueT<Adpgamoncr_SPEC>;

impl Adpgamoncr {
    #[inline(always)]
    pub fn pgamon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        adpgamoncr::Pgamon,
        adpgamoncr::Pgamon,
        Adpgamoncr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            adpgamoncr::Pgamon,
            adpgamoncr::Pgamon,
            Adpgamoncr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn monsel0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adpgamoncr::Monsel0,
        adpgamoncr::Monsel0,
        Adpgamoncr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adpgamoncr::Monsel0,
            adpgamoncr::Monsel0,
            Adpgamoncr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn monsel1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adpgamoncr::Monsel1,
        adpgamoncr::Monsel1,
        Adpgamoncr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adpgamoncr::Monsel1,
            adpgamoncr::Monsel1,
            Adpgamoncr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn monsel2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        adpgamoncr::Monsel2,
        adpgamoncr::Monsel2,
        Adpgamoncr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            adpgamoncr::Monsel2,
            adpgamoncr::Monsel2,
            Adpgamoncr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn monsel3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        adpgamoncr::Monsel3,
        adpgamoncr::Monsel3,
        Adpgamoncr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            adpgamoncr::Monsel3,
            adpgamoncr::Monsel3,
            Adpgamoncr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adpgamoncr {
    #[inline(always)]
    fn default() -> Adpgamoncr {
        <crate::RegValueT<Adpgamoncr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adpgamoncr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgamon_SPEC;
    pub type Pgamon = crate::EnumBitfieldStruct<u8, Pgamon_SPEC>;
    impl Pgamon {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Monsel0_SPEC;
    pub type Monsel0 = crate::EnumBitfieldStruct<u8, Monsel0_SPEC>;
    impl Monsel0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Monsel1_SPEC;
    pub type Monsel1 = crate::EnumBitfieldStruct<u8, Monsel1_SPEC>;
    impl Monsel1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Monsel2_SPEC;
    pub type Monsel2 = crate::EnumBitfieldStruct<u8, Monsel2_SPEC>;
    impl Monsel2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Monsel3_SPEC;
    pub type Monsel3 = crate::EnumBitfieldStruct<u8, Monsel3_SPEC>;
    impl Monsel3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adrefcr_SPEC;
impl crate::sealed::RegSpec for Adrefcr_SPEC {
    type DataType = u32;
}

pub type Adrefcr = crate::RegValueT<Adrefcr_SPEC>;

impl Adrefcr {
    #[inline(always)]
    pub fn vde(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adrefcr::Vde,
        adrefcr::Vde,
        Adrefcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adrefcr::Vde,
            adrefcr::Vde,
            Adrefcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adrefcr {
    #[inline(always)]
    fn default() -> Adrefcr {
        <crate::RegValueT<Adrefcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adrefcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vde_SPEC;
    pub type Vde = crate::EnumBitfieldStruct<u8, Vde_SPEC>;
    impl Vde {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addfsr_SPEC;
impl crate::sealed::RegSpec for Addfsr_SPEC {
    type DataType = u32;
}

pub type Addfsr = crate::RegValueT<Addfsr_SPEC>;

impl Addfsr {
    #[inline(always)]
    pub fn dfsel0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        addfsr::Dfsel0,
        addfsr::Dfsel0,
        Addfsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            addfsr::Dfsel0,
            addfsr::Dfsel0,
            Addfsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dfsel1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        addfsr::Dfsel1,
        addfsr::Dfsel1,
        Addfsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            addfsr::Dfsel1,
            addfsr::Dfsel1,
            Addfsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dfsel2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        addfsr::Dfsel2,
        addfsr::Dfsel2,
        Addfsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            addfsr::Dfsel2,
            addfsr::Dfsel2,
            Addfsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dfsel3(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        addfsr::Dfsel3,
        addfsr::Dfsel3,
        Addfsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            addfsr::Dfsel3,
            addfsr::Dfsel3,
            Addfsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Addfsr {
    #[inline(always)]
    fn default() -> Addfsr {
        <crate::RegValueT<Addfsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod addfsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfsel0_SPEC;
    pub type Dfsel0 = crate::EnumBitfieldStruct<u8, Dfsel0_SPEC>;
    impl Dfsel0 {
        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfsel1_SPEC;
    pub type Dfsel1 = crate::EnumBitfieldStruct<u8, Dfsel1_SPEC>;
    impl Dfsel1 {
        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfsel2_SPEC;
    pub type Dfsel2 = crate::EnumBitfieldStruct<u8, Dfsel2_SPEC>;
    impl Dfsel2 {
        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfsel3_SPEC;
    pub type Dfsel3 = crate::EnumBitfieldStruct<u8, Dfsel3_SPEC>;
    impl Dfsel3 {
        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aduoftr_SPEC;
impl crate::sealed::RegSpec for Aduoftr_SPEC {
    type DataType = u32;
}

pub type Aduoftr = crate::RegValueT<Aduoftr_SPEC>;

impl Aduoftr {
    #[inline(always)]
    pub fn uofset(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Aduoftr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Aduoftr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Aduoftr {
    #[inline(always)]
    fn default() -> Aduoftr {
        <crate::RegValueT<Aduoftr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adugtr_SPEC;
impl crate::sealed::RegSpec for Adugtr_SPEC {
    type DataType = u32;
}

pub type Adugtr = crate::RegValueT<Adugtr_SPEC>;

impl Adugtr {
    #[inline(always)]
    pub fn ugain(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Adugtr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Adugtr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adugtr {
    #[inline(always)]
    fn default() -> Adugtr {
        <crate::RegValueT<Adugtr_SPEC> as RegisterValue<_>>::new(4194304)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adlimintcr_SPEC;
impl crate::sealed::RegSpec for Adlimintcr_SPEC {
    type DataType = u32;
}

pub type Adlimintcr = crate::RegValueT<Adlimintcr_SPEC>;

impl Adlimintcr {
    #[inline(always)]
    pub fn limie0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adlimintcr::Limie0,
        adlimintcr::Limie0,
        Adlimintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adlimintcr::Limie0,
            adlimintcr::Limie0,
            Adlimintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limie1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adlimintcr::Limie1,
        adlimintcr::Limie1,
        Adlimintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adlimintcr::Limie1,
            adlimintcr::Limie1,
            Adlimintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limie2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adlimintcr::Limie2,
        adlimintcr::Limie2,
        Adlimintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adlimintcr::Limie2,
            adlimintcr::Limie2,
            Adlimintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limie3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adlimintcr::Limie3,
        adlimintcr::Limie3,
        Adlimintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adlimintcr::Limie3,
            adlimintcr::Limie3,
            Adlimintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limie4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adlimintcr::Limie4,
        adlimintcr::Limie4,
        Adlimintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adlimintcr::Limie4,
            adlimintcr::Limie4,
            Adlimintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limie5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adlimintcr::Limie5,
        adlimintcr::Limie5,
        Adlimintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adlimintcr::Limie5,
            adlimintcr::Limie5,
            Adlimintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limie6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adlimintcr::Limie6,
        adlimintcr::Limie6,
        Adlimintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adlimintcr::Limie6,
            adlimintcr::Limie6,
            Adlimintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limie7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adlimintcr::Limie7,
        adlimintcr::Limie7,
        Adlimintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adlimintcr::Limie7,
            adlimintcr::Limie7,
            Adlimintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limie8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adlimintcr::Limie8,
        adlimintcr::Limie8,
        Adlimintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adlimintcr::Limie8,
            adlimintcr::Limie8,
            Adlimintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adlimintcr {
    #[inline(always)]
    fn default() -> Adlimintcr {
        <crate::RegValueT<Adlimintcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adlimintcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limie0_SPEC;
    pub type Limie0 = crate::EnumBitfieldStruct<u8, Limie0_SPEC>;
    impl Limie0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limie1_SPEC;
    pub type Limie1 = crate::EnumBitfieldStruct<u8, Limie1_SPEC>;
    impl Limie1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limie2_SPEC;
    pub type Limie2 = crate::EnumBitfieldStruct<u8, Limie2_SPEC>;
    impl Limie2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limie3_SPEC;
    pub type Limie3 = crate::EnumBitfieldStruct<u8, Limie3_SPEC>;
    impl Limie3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limie4_SPEC;
    pub type Limie4 = crate::EnumBitfieldStruct<u8, Limie4_SPEC>;
    impl Limie4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limie5_SPEC;
    pub type Limie5 = crate::EnumBitfieldStruct<u8, Limie5_SPEC>;
    impl Limie5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limie6_SPEC;
    pub type Limie6 = crate::EnumBitfieldStruct<u8, Limie6_SPEC>;
    impl Limie6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limie7_SPEC;
    pub type Limie7 = crate::EnumBitfieldStruct<u8, Limie7_SPEC>;
    impl Limie7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limie8_SPEC;
    pub type Limie8 = crate::EnumBitfieldStruct<u8, Limie8_SPEC>;
    impl Limie8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adlimtr_SPEC;
impl crate::sealed::RegSpec for Adlimtr_SPEC {
    type DataType = u32;
}

pub type Adlimtr = crate::RegValueT<Adlimtr_SPEC>;

impl Adlimtr {
    #[inline(always)]
    pub fn liml(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adlimtr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adlimtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn limu(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Adlimtr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Adlimtr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adlimtr {
    #[inline(always)]
    fn default() -> Adlimtr {
        <crate::RegValueT<Adlimtr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpenr_SPEC;
impl crate::sealed::RegSpec for Adcmpenr_SPEC {
    type DataType = u32;
}

pub type Adcmpenr = crate::RegValueT<Adcmpenr_SPEC>;

impl Adcmpenr {
    #[inline(always)]
    pub fn cmpen0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpenr::Cmpen0,
        adcmpenr::Cmpen0,
        Adcmpenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpenr::Cmpen0,
            adcmpenr::Cmpen0,
            Adcmpenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpen1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpenr::Cmpen1,
        adcmpenr::Cmpen1,
        Adcmpenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpenr::Cmpen1,
            adcmpenr::Cmpen1,
            Adcmpenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpen2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpenr::Cmpen2,
        adcmpenr::Cmpen2,
        Adcmpenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpenr::Cmpen2,
            adcmpenr::Cmpen2,
            Adcmpenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpen3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmpenr::Cmpen3,
        adcmpenr::Cmpen3,
        Adcmpenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmpenr::Cmpen3,
            adcmpenr::Cmpen3,
            Adcmpenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpen4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmpenr::Cmpen4,
        adcmpenr::Cmpen4,
        Adcmpenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmpenr::Cmpen4,
            adcmpenr::Cmpen4,
            Adcmpenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpen5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpenr::Cmpen5,
        adcmpenr::Cmpen5,
        Adcmpenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpenr::Cmpen5,
            adcmpenr::Cmpen5,
            Adcmpenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpen6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpenr::Cmpen6,
        adcmpenr::Cmpen6,
        Adcmpenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpenr::Cmpen6,
            adcmpenr::Cmpen6,
            Adcmpenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpen7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpenr::Cmpen7,
        adcmpenr::Cmpen7,
        Adcmpenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpenr::Cmpen7,
            adcmpenr::Cmpen7,
            Adcmpenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpenr {
    #[inline(always)]
    fn default() -> Adcmpenr {
        <crate::RegValueT<Adcmpenr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpenr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpen0_SPEC;
    pub type Cmpen0 = crate::EnumBitfieldStruct<u8, Cmpen0_SPEC>;
    impl Cmpen0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpen1_SPEC;
    pub type Cmpen1 = crate::EnumBitfieldStruct<u8, Cmpen1_SPEC>;
    impl Cmpen1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpen2_SPEC;
    pub type Cmpen2 = crate::EnumBitfieldStruct<u8, Cmpen2_SPEC>;
    impl Cmpen2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpen3_SPEC;
    pub type Cmpen3 = crate::EnumBitfieldStruct<u8, Cmpen3_SPEC>;
    impl Cmpen3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpen4_SPEC;
    pub type Cmpen4 = crate::EnumBitfieldStruct<u8, Cmpen4_SPEC>;
    impl Cmpen4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpen5_SPEC;
    pub type Cmpen5 = crate::EnumBitfieldStruct<u8, Cmpen5_SPEC>;
    impl Cmpen5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpen6_SPEC;
    pub type Cmpen6 = crate::EnumBitfieldStruct<u8, Cmpen6_SPEC>;
    impl Cmpen6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpen7_SPEC;
    pub type Cmpen7 = crate::EnumBitfieldStruct<u8, Cmpen7_SPEC>;
    impl Cmpen7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpintcr_SPEC;
impl crate::sealed::RegSpec for Adcmpintcr_SPEC {
    type DataType = u32;
}

pub type Adcmpintcr = crate::RegValueT<Adcmpintcr_SPEC>;

impl Adcmpintcr {
    #[inline(always)]
    pub fn cmpie0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpintcr::Cmpie0,
        adcmpintcr::Cmpie0,
        Adcmpintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpintcr::Cmpie0,
            adcmpintcr::Cmpie0,
            Adcmpintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpie1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpintcr::Cmpie1,
        adcmpintcr::Cmpie1,
        Adcmpintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpintcr::Cmpie1,
            adcmpintcr::Cmpie1,
            Adcmpintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpie2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpintcr::Cmpie2,
        adcmpintcr::Cmpie2,
        Adcmpintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpintcr::Cmpie2,
            adcmpintcr::Cmpie2,
            Adcmpintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpie3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmpintcr::Cmpie3,
        adcmpintcr::Cmpie3,
        Adcmpintcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmpintcr::Cmpie3,
            adcmpintcr::Cmpie3,
            Adcmpintcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpintcr {
    #[inline(always)]
    fn default() -> Adcmpintcr {
        <crate::RegValueT<Adcmpintcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpintcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpie0_SPEC;
    pub type Cmpie0 = crate::EnumBitfieldStruct<u8, Cmpie0_SPEC>;
    impl Cmpie0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpie1_SPEC;
    pub type Cmpie1 = crate::EnumBitfieldStruct<u8, Cmpie1_SPEC>;
    impl Cmpie1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpie2_SPEC;
    pub type Cmpie2 = crate::EnumBitfieldStruct<u8, Cmpie2_SPEC>;
    impl Cmpie2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpie3_SPEC;
    pub type Cmpie3 = crate::EnumBitfieldStruct<u8, Cmpie3_SPEC>;
    impl Cmpie3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adccmpcr_SPEC;
impl crate::sealed::RegSpec for Adccmpcr_SPEC {
    type DataType = u32;
}

pub type Adccmpcr = crate::RegValueT<Adccmpcr_SPEC>;

impl Adccmpcr {
    #[inline(always)]
    pub fn ccmpcnd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adccmpcr::Ccmpcnd,
        adccmpcr::Ccmpcnd,
        Adccmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adccmpcr::Ccmpcnd,
            adccmpcr::Ccmpcnd,
            Adccmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccmptbl0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adccmpcr::Ccmptbl0,
        adccmpcr::Ccmptbl0,
        Adccmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adccmpcr::Ccmptbl0,
            adccmpcr::Ccmptbl0,
            Adccmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccmptbl1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adccmpcr::Ccmptbl1,
        adccmpcr::Ccmptbl1,
        Adccmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adccmpcr::Ccmptbl1,
            adccmpcr::Ccmptbl1,
            Adccmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccmptbl2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        adccmpcr::Ccmptbl2,
        adccmpcr::Ccmptbl2,
        Adccmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            adccmpcr::Ccmptbl2,
            adccmpcr::Ccmptbl2,
            Adccmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccmptbl3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        adccmpcr::Ccmptbl3,
        adccmpcr::Ccmptbl3,
        Adccmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            adccmpcr::Ccmptbl3,
            adccmpcr::Ccmptbl3,
            Adccmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccmptbl4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        adccmpcr::Ccmptbl4,
        adccmpcr::Ccmptbl4,
        Adccmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            adccmpcr::Ccmptbl4,
            adccmpcr::Ccmptbl4,
            Adccmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccmptbl5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        adccmpcr::Ccmptbl5,
        adccmpcr::Ccmptbl5,
        Adccmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            adccmpcr::Ccmptbl5,
            adccmpcr::Ccmptbl5,
            Adccmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccmptbl6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        adccmpcr::Ccmptbl6,
        adccmpcr::Ccmptbl6,
        Adccmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            adccmpcr::Ccmptbl6,
            adccmpcr::Ccmptbl6,
            Adccmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccmptbl7(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        adccmpcr::Ccmptbl7,
        adccmpcr::Ccmptbl7,
        Adccmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            adccmpcr::Ccmptbl7,
            adccmpcr::Ccmptbl7,
            Adccmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adccmpcr {
    #[inline(always)]
    fn default() -> Adccmpcr {
        <crate::RegValueT<Adccmpcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adccmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccmpcnd_SPEC;
    pub type Ccmpcnd = crate::EnumBitfieldStruct<u8, Ccmpcnd_SPEC>;
    impl Ccmpcnd {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccmptbl0_SPEC;
    pub type Ccmptbl0 = crate::EnumBitfieldStruct<u8, Ccmptbl0_SPEC>;
    impl Ccmptbl0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccmptbl1_SPEC;
    pub type Ccmptbl1 = crate::EnumBitfieldStruct<u8, Ccmptbl1_SPEC>;
    impl Ccmptbl1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccmptbl2_SPEC;
    pub type Ccmptbl2 = crate::EnumBitfieldStruct<u8, Ccmptbl2_SPEC>;
    impl Ccmptbl2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccmptbl3_SPEC;
    pub type Ccmptbl3 = crate::EnumBitfieldStruct<u8, Ccmptbl3_SPEC>;
    impl Ccmptbl3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccmptbl4_SPEC;
    pub type Ccmptbl4 = crate::EnumBitfieldStruct<u8, Ccmptbl4_SPEC>;
    impl Ccmptbl4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccmptbl5_SPEC;
    pub type Ccmptbl5 = crate::EnumBitfieldStruct<u8, Ccmptbl5_SPEC>;
    impl Ccmptbl5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccmptbl6_SPEC;
    pub type Ccmptbl6 = crate::EnumBitfieldStruct<u8, Ccmptbl6_SPEC>;
    impl Ccmptbl6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccmptbl7_SPEC;
    pub type Ccmptbl7 = crate::EnumBitfieldStruct<u8, Ccmptbl7_SPEC>;
    impl Ccmptbl7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpmdr0_SPEC;
impl crate::sealed::RegSpec for Adcmpmdr0_SPEC {
    type DataType = u32;
}

pub type Adcmpmdr0 = crate::RegValueT<Adcmpmdr0_SPEC>;

impl Adcmpmdr0 {
    #[inline(always)]
    pub fn cmpmd0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adcmpmdr0::Cmpmd0,
        adcmpmdr0::Cmpmd0,
        Adcmpmdr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adcmpmdr0::Cmpmd0,
            adcmpmdr0::Cmpmd0,
            Adcmpmdr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpmd1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        adcmpmdr0::Cmpmd1,
        adcmpmdr0::Cmpmd1,
        Adcmpmdr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            adcmpmdr0::Cmpmd1,
            adcmpmdr0::Cmpmd1,
            Adcmpmdr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpmd2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        adcmpmdr0::Cmpmd2,
        adcmpmdr0::Cmpmd2,
        Adcmpmdr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            adcmpmdr0::Cmpmd2,
            adcmpmdr0::Cmpmd2,
            Adcmpmdr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpmd3(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        adcmpmdr0::Cmpmd3,
        adcmpmdr0::Cmpmd3,
        Adcmpmdr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            adcmpmdr0::Cmpmd3,
            adcmpmdr0::Cmpmd3,
            Adcmpmdr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpmdr0 {
    #[inline(always)]
    fn default() -> Adcmpmdr0 {
        <crate::RegValueT<Adcmpmdr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpmdr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpmd0_SPEC;
    pub type Cmpmd0 = crate::EnumBitfieldStruct<u8, Cmpmd0_SPEC>;
    impl Cmpmd0 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpmd1_SPEC;
    pub type Cmpmd1 = crate::EnumBitfieldStruct<u8, Cmpmd1_SPEC>;
    impl Cmpmd1 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpmd2_SPEC;
    pub type Cmpmd2 = crate::EnumBitfieldStruct<u8, Cmpmd2_SPEC>;
    impl Cmpmd2 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpmd3_SPEC;
    pub type Cmpmd3 = crate::EnumBitfieldStruct<u8, Cmpmd3_SPEC>;
    impl Cmpmd3 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpmdr1_SPEC;
impl crate::sealed::RegSpec for Adcmpmdr1_SPEC {
    type DataType = u32;
}

pub type Adcmpmdr1 = crate::RegValueT<Adcmpmdr1_SPEC>;

impl Adcmpmdr1 {
    #[inline(always)]
    pub fn cmpmd4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adcmpmdr1::Cmpmd4,
        adcmpmdr1::Cmpmd4,
        Adcmpmdr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adcmpmdr1::Cmpmd4,
            adcmpmdr1::Cmpmd4,
            Adcmpmdr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpmd5(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        adcmpmdr1::Cmpmd5,
        adcmpmdr1::Cmpmd5,
        Adcmpmdr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            adcmpmdr1::Cmpmd5,
            adcmpmdr1::Cmpmd5,
            Adcmpmdr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpmd6(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        adcmpmdr1::Cmpmd6,
        adcmpmdr1::Cmpmd6,
        Adcmpmdr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            adcmpmdr1::Cmpmd6,
            adcmpmdr1::Cmpmd6,
            Adcmpmdr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpmd7(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        adcmpmdr1::Cmpmd7,
        adcmpmdr1::Cmpmd7,
        Adcmpmdr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            adcmpmdr1::Cmpmd7,
            adcmpmdr1::Cmpmd7,
            Adcmpmdr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpmdr1 {
    #[inline(always)]
    fn default() -> Adcmpmdr1 {
        <crate::RegValueT<Adcmpmdr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpmdr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpmd4_SPEC;
    pub type Cmpmd4 = crate::EnumBitfieldStruct<u8, Cmpmd4_SPEC>;
    impl Cmpmd4 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpmd5_SPEC;
    pub type Cmpmd5 = crate::EnumBitfieldStruct<u8, Cmpmd5_SPEC>;
    impl Cmpmd5 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpmd6_SPEC;
    pub type Cmpmd6 = crate::EnumBitfieldStruct<u8, Cmpmd6_SPEC>;
    impl Cmpmd6 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpmd7_SPEC;
    pub type Cmpmd7 = crate::EnumBitfieldStruct<u8, Cmpmd7_SPEC>;
    impl Cmpmd7 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmptbr_SPEC;
impl crate::sealed::RegSpec for Adcmptbr_SPEC {
    type DataType = u32;
}

pub type Adcmptbr = crate::RegValueT<Adcmptbr_SPEC>;

impl Adcmptbr {
    #[inline(always)]
    pub fn cmptbl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adcmptbr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adcmptbr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmptbh(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Adcmptbr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Adcmptbr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adcmptbr {
    #[inline(always)]
    fn default() -> Adcmptbr {
        <crate::RegValueT<Adcmptbr_SPEC> as RegisterValue<_>>::new(4294901760)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifocr_SPEC;
impl crate::sealed::RegSpec for Adfifocr_SPEC {
    type DataType = u32;
}

pub type Adfifocr = crate::RegValueT<Adfifocr_SPEC>;

impl Adfifocr {
    #[inline(always)]
    pub fn fifoen0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adfifocr::Fifoen0,
        adfifocr::Fifoen0,
        Adfifocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adfifocr::Fifoen0,
            adfifocr::Fifoen0,
            Adfifocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoen1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adfifocr::Fifoen1,
        adfifocr::Fifoen1,
        Adfifocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adfifocr::Fifoen1,
            adfifocr::Fifoen1,
            Adfifocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoen2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adfifocr::Fifoen2,
        adfifocr::Fifoen2,
        Adfifocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adfifocr::Fifoen2,
            adfifocr::Fifoen2,
            Adfifocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoen3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adfifocr::Fifoen3,
        adfifocr::Fifoen3,
        Adfifocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adfifocr::Fifoen3,
            adfifocr::Fifoen3,
            Adfifocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoen4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adfifocr::Fifoen4,
        adfifocr::Fifoen4,
        Adfifocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adfifocr::Fifoen4,
            adfifocr::Fifoen4,
            Adfifocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoen5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adfifocr::Fifoen5,
        adfifocr::Fifoen5,
        Adfifocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adfifocr::Fifoen5,
            adfifocr::Fifoen5,
            Adfifocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoen6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adfifocr::Fifoen6,
        adfifocr::Fifoen6,
        Adfifocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adfifocr::Fifoen6,
            adfifocr::Fifoen6,
            Adfifocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoen7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adfifocr::Fifoen7,
        adfifocr::Fifoen7,
        Adfifocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adfifocr::Fifoen7,
            adfifocr::Fifoen7,
            Adfifocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoen8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adfifocr::Fifoen8,
        adfifocr::Fifoen8,
        Adfifocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adfifocr::Fifoen8,
            adfifocr::Fifoen8,
            Adfifocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adfifocr {
    #[inline(always)]
    fn default() -> Adfifocr {
        <crate::RegValueT<Adfifocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adfifocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoen0_SPEC;
    pub type Fifoen0 = crate::EnumBitfieldStruct<u8, Fifoen0_SPEC>;
    impl Fifoen0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoen1_SPEC;
    pub type Fifoen1 = crate::EnumBitfieldStruct<u8, Fifoen1_SPEC>;
    impl Fifoen1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoen2_SPEC;
    pub type Fifoen2 = crate::EnumBitfieldStruct<u8, Fifoen2_SPEC>;
    impl Fifoen2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoen3_SPEC;
    pub type Fifoen3 = crate::EnumBitfieldStruct<u8, Fifoen3_SPEC>;
    impl Fifoen3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoen4_SPEC;
    pub type Fifoen4 = crate::EnumBitfieldStruct<u8, Fifoen4_SPEC>;
    impl Fifoen4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoen5_SPEC;
    pub type Fifoen5 = crate::EnumBitfieldStruct<u8, Fifoen5_SPEC>;
    impl Fifoen5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoen6_SPEC;
    pub type Fifoen6 = crate::EnumBitfieldStruct<u8, Fifoen6_SPEC>;
    impl Fifoen6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoen7_SPEC;
    pub type Fifoen7 = crate::EnumBitfieldStruct<u8, Fifoen7_SPEC>;
    impl Fifoen7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoen8_SPEC;
    pub type Fifoen8 = crate::EnumBitfieldStruct<u8, Fifoen8_SPEC>;
    impl Fifoen8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifointcr_SPEC;
impl crate::sealed::RegSpec for Adfifointcr_SPEC {
    type DataType = u32;
}

pub type Adfifointcr = crate::RegValueT<Adfifointcr_SPEC>;

impl Adfifointcr {
    #[inline(always)]
    pub fn fifoie0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adfifointcr::Fifoie0,
        adfifointcr::Fifoie0,
        Adfifointcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adfifointcr::Fifoie0,
            adfifointcr::Fifoie0,
            Adfifointcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoie1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adfifointcr::Fifoie1,
        adfifointcr::Fifoie1,
        Adfifointcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adfifointcr::Fifoie1,
            adfifointcr::Fifoie1,
            Adfifointcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoie2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adfifointcr::Fifoie2,
        adfifointcr::Fifoie2,
        Adfifointcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adfifointcr::Fifoie2,
            adfifointcr::Fifoie2,
            Adfifointcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoie3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adfifointcr::Fifoie3,
        adfifointcr::Fifoie3,
        Adfifointcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adfifointcr::Fifoie3,
            adfifointcr::Fifoie3,
            Adfifointcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoie4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adfifointcr::Fifoie4,
        adfifointcr::Fifoie4,
        Adfifointcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adfifointcr::Fifoie4,
            adfifointcr::Fifoie4,
            Adfifointcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoie5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adfifointcr::Fifoie5,
        adfifointcr::Fifoie5,
        Adfifointcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adfifointcr::Fifoie5,
            adfifointcr::Fifoie5,
            Adfifointcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoie6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adfifointcr::Fifoie6,
        adfifointcr::Fifoie6,
        Adfifointcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adfifointcr::Fifoie6,
            adfifointcr::Fifoie6,
            Adfifointcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoie7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adfifointcr::Fifoie7,
        adfifointcr::Fifoie7,
        Adfifointcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adfifointcr::Fifoie7,
            adfifointcr::Fifoie7,
            Adfifointcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoie8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adfifointcr::Fifoie8,
        adfifointcr::Fifoie8,
        Adfifointcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adfifointcr::Fifoie8,
            adfifointcr::Fifoie8,
            Adfifointcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adfifointcr {
    #[inline(always)]
    fn default() -> Adfifointcr {
        <crate::RegValueT<Adfifointcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adfifointcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoie0_SPEC;
    pub type Fifoie0 = crate::EnumBitfieldStruct<u8, Fifoie0_SPEC>;
    impl Fifoie0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoie1_SPEC;
    pub type Fifoie1 = crate::EnumBitfieldStruct<u8, Fifoie1_SPEC>;
    impl Fifoie1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoie2_SPEC;
    pub type Fifoie2 = crate::EnumBitfieldStruct<u8, Fifoie2_SPEC>;
    impl Fifoie2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoie3_SPEC;
    pub type Fifoie3 = crate::EnumBitfieldStruct<u8, Fifoie3_SPEC>;
    impl Fifoie3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoie4_SPEC;
    pub type Fifoie4 = crate::EnumBitfieldStruct<u8, Fifoie4_SPEC>;
    impl Fifoie4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoie5_SPEC;
    pub type Fifoie5 = crate::EnumBitfieldStruct<u8, Fifoie5_SPEC>;
    impl Fifoie5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoie6_SPEC;
    pub type Fifoie6 = crate::EnumBitfieldStruct<u8, Fifoie6_SPEC>;
    impl Fifoie6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoie7_SPEC;
    pub type Fifoie7 = crate::EnumBitfieldStruct<u8, Fifoie7_SPEC>;
    impl Fifoie7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoie8_SPEC;
    pub type Fifoie8 = crate::EnumBitfieldStruct<u8, Fifoie8_SPEC>;
    impl Fifoie8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifointlr0_SPEC;
impl crate::sealed::RegSpec for Adfifointlr0_SPEC {
    type DataType = u32;
}

pub type Adfifointlr0 = crate::RegValueT<Adfifointlr0_SPEC>;

impl Adfifointlr0 {
    #[inline(always)]
    pub fn fifoilv0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Adfifointlr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Adfifointlr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fifoilv1(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Adfifointlr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Adfifointlr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adfifointlr0 {
    #[inline(always)]
    fn default() -> Adfifointlr0 {
        <crate::RegValueT<Adfifointlr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifointlr1_SPEC;
impl crate::sealed::RegSpec for Adfifointlr1_SPEC {
    type DataType = u32;
}

pub type Adfifointlr1 = crate::RegValueT<Adfifointlr1_SPEC>;

impl Adfifointlr1 {
    #[inline(always)]
    pub fn fifoilv2(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Adfifointlr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Adfifointlr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fifoilv3(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Adfifointlr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Adfifointlr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adfifointlr1 {
    #[inline(always)]
    fn default() -> Adfifointlr1 {
        <crate::RegValueT<Adfifointlr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifointlr2_SPEC;
impl crate::sealed::RegSpec for Adfifointlr2_SPEC {
    type DataType = u32;
}

pub type Adfifointlr2 = crate::RegValueT<Adfifointlr2_SPEC>;

impl Adfifointlr2 {
    #[inline(always)]
    pub fn fifoilv4(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Adfifointlr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Adfifointlr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fifoilv5(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Adfifointlr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Adfifointlr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adfifointlr2 {
    #[inline(always)]
    fn default() -> Adfifointlr2 {
        <crate::RegValueT<Adfifointlr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifointlr3_SPEC;
impl crate::sealed::RegSpec for Adfifointlr3_SPEC {
    type DataType = u32;
}

pub type Adfifointlr3 = crate::RegValueT<Adfifointlr3_SPEC>;

impl Adfifointlr3 {
    #[inline(always)]
    pub fn fifoilv6(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Adfifointlr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Adfifointlr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fifoilv7(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Adfifointlr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Adfifointlr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adfifointlr3 {
    #[inline(always)]
    fn default() -> Adfifointlr3 {
        <crate::RegValueT<Adfifointlr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifointlr4_SPEC;
impl crate::sealed::RegSpec for Adfifointlr4_SPEC {
    type DataType = u32;
}

pub type Adfifointlr4 = crate::RegValueT<Adfifointlr4_SPEC>;

impl Adfifointlr4 {
    #[inline(always)]
    pub fn fifoilv8(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Adfifointlr4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Adfifointlr4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adfifointlr4 {
    #[inline(always)]
    fn default() -> Adfifointlr4 {
        <crate::RegValueT<Adfifointlr4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adchcr_SPEC;
impl crate::sealed::RegSpec for Adchcr_SPEC {
    type DataType = u32;
}

pub type Adchcr = crate::RegValueT<Adchcr_SPEC>;

impl Adchcr {
    #[inline(always)]
    pub fn sgsel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Adchcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Adchcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cnvcs(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, Adchcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,Adchcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ainmd(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adchcr::Ainmd,
        adchcr::Ainmd,
        Adchcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adchcr::Ainmd,
            adchcr::Ainmd,
            Adchcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sstsel(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Adchcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Adchcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adchcr {
    #[inline(always)]
    fn default() -> Adchcr {
        <crate::RegValueT<Adchcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adchcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ainmd_SPEC;
    pub type Ainmd = crate::EnumBitfieldStruct<u8, Ainmd_SPEC>;
    impl Ainmd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addopcra_SPEC;
impl crate::sealed::RegSpec for Addopcra_SPEC {
    type DataType = u32;
}

pub type Addopcra = crate::RegValueT<Addopcra_SPEC>;

impl Addopcra {
    #[inline(always)]
    pub fn dfsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        addopcra::Dfsel,
        addopcra::Dfsel,
        Addopcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            addopcra::Dfsel,
            addopcra::Dfsel,
            Addopcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gainsel(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Addopcra_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Addopcra_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ofsetsel(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Addopcra_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Addopcra_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Addopcra {
    #[inline(always)]
    fn default() -> Addopcra {
        <crate::RegValueT<Addopcra_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod addopcra {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfsel_SPEC;
    pub type Dfsel = crate::EnumBitfieldStruct<u8, Dfsel_SPEC>;
    impl Dfsel {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addopcrb_SPEC;
impl crate::sealed::RegSpec for Addopcrb_SPEC {
    type DataType = u32;
}

pub type Addopcrb = crate::RegValueT<Addopcrb_SPEC>;

impl Addopcrb {
    #[inline(always)]
    pub fn avemd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        addopcrb::Avemd,
        addopcrb::Avemd,
        Addopcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            addopcrb::Avemd,
            addopcrb::Avemd,
            Addopcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        addopcrb::Adc,
        addopcrb::Adc,
        Addopcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            addopcrb::Adc,
            addopcrb::Adc,
            Addopcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptble0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        addopcrb::Cmptble0,
        addopcrb::Cmptble0,
        Addopcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            addopcrb::Cmptble0,
            addopcrb::Cmptble0,
            Addopcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptble1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        addopcrb::Cmptble1,
        addopcrb::Cmptble1,
        Addopcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            addopcrb::Cmptble1,
            addopcrb::Cmptble1,
            Addopcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptble2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        addopcrb::Cmptble2,
        addopcrb::Cmptble2,
        Addopcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            addopcrb::Cmptble2,
            addopcrb::Cmptble2,
            Addopcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptble3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        addopcrb::Cmptble3,
        addopcrb::Cmptble3,
        Addopcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            addopcrb::Cmptble3,
            addopcrb::Cmptble3,
            Addopcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptble4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        addopcrb::Cmptble4,
        addopcrb::Cmptble4,
        Addopcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            addopcrb::Cmptble4,
            addopcrb::Cmptble4,
            Addopcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptble5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        addopcrb::Cmptble5,
        addopcrb::Cmptble5,
        Addopcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            addopcrb::Cmptble5,
            addopcrb::Cmptble5,
            Addopcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptble6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        addopcrb::Cmptble6,
        addopcrb::Cmptble6,
        Addopcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            addopcrb::Cmptble6,
            addopcrb::Cmptble6,
            Addopcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptble7(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        addopcrb::Cmptble7,
        addopcrb::Cmptble7,
        Addopcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            addopcrb::Cmptble7,
            addopcrb::Cmptble7,
            Addopcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Addopcrb {
    #[inline(always)]
    fn default() -> Addopcrb {
        <crate::RegValueT<Addopcrb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod addopcrb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Avemd_SPEC;
    pub type Avemd = crate::EnumBitfieldStruct<u8, Avemd_SPEC>;
    impl Avemd {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adc_SPEC;
    pub type Adc = crate::EnumBitfieldStruct<u8, Adc_SPEC>;
    impl Adc {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const _0_X_6: Self = Self::new(6);

        pub const _0_X_7: Self = Self::new(7);

        pub const _0_X_8: Self = Self::new(8);

        pub const _0_X_9: Self = Self::new(9);

        pub const _0_X_A: Self = Self::new(10);

        pub const _0_X_B: Self = Self::new(11);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptble0_SPEC;
    pub type Cmptble0 = crate::EnumBitfieldStruct<u8, Cmptble0_SPEC>;
    impl Cmptble0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptble1_SPEC;
    pub type Cmptble1 = crate::EnumBitfieldStruct<u8, Cmptble1_SPEC>;
    impl Cmptble1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptble2_SPEC;
    pub type Cmptble2 = crate::EnumBitfieldStruct<u8, Cmptble2_SPEC>;
    impl Cmptble2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptble3_SPEC;
    pub type Cmptble3 = crate::EnumBitfieldStruct<u8, Cmptble3_SPEC>;
    impl Cmptble3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptble4_SPEC;
    pub type Cmptble4 = crate::EnumBitfieldStruct<u8, Cmptble4_SPEC>;
    impl Cmptble4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptble5_SPEC;
    pub type Cmptble5 = crate::EnumBitfieldStruct<u8, Cmptble5_SPEC>;
    impl Cmptble5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptble6_SPEC;
    pub type Cmptble6 = crate::EnumBitfieldStruct<u8, Cmptble6_SPEC>;
    impl Cmptble6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptble7_SPEC;
    pub type Cmptble7 = crate::EnumBitfieldStruct<u8, Cmptble7_SPEC>;
    impl Cmptble7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addopcrc_SPEC;
impl crate::sealed::RegSpec for Addopcrc_SPEC {
    type DataType = u32;
}

pub type Addopcrc = crate::RegValueT<Addopcrc_SPEC>;

impl Addopcrc {
    #[inline(always)]
    pub fn limtbls(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Addopcrc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Addopcrc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adprc(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        addopcrc::Adprc,
        addopcrc::Adprc,
        Addopcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            addopcrc::Adprc,
            addopcrc::Adprc,
            Addopcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn signsel(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        addopcrc::Signsel,
        addopcrc::Signsel,
        Addopcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            addopcrc::Signsel,
            addopcrc::Signsel,
            Addopcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Addopcrc {
    #[inline(always)]
    fn default() -> Addopcrc {
        <crate::RegValueT<Addopcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod addopcrc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adprc_SPEC;
    pub type Adprc = crate::EnumBitfieldStruct<u8, Adprc_SPEC>;
    impl Adprc {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Signsel_SPEC;
    pub type Signsel = crate::EnumBitfieldStruct<u8, Signsel_SPEC>;
    impl Signsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcalstr_SPEC;
impl crate::sealed::RegSpec for Adcalstr_SPEC {
    type DataType = u32;
}

pub type Adcalstr = crate::RegValueT<Adcalstr_SPEC>;

impl Adcalstr {
    #[inline(always)]
    pub fn adcalst0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Adcalstr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Adcalstr_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adcalst1(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Adcalstr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Adcalstr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Adcalstr {
    #[inline(always)]
    fn default() -> Adcalstr {
        <crate::RegValueT<Adcalstr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adtrgenr_SPEC;
impl crate::sealed::RegSpec for Adtrgenr_SPEC {
    type DataType = u32;
}

pub type Adtrgenr = crate::RegValueT<Adtrgenr_SPEC>;

impl Adtrgenr {
    #[inline(always)]
    pub fn sttrgen0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adtrgenr::Sttrgen0,
        adtrgenr::Sttrgen0,
        Adtrgenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adtrgenr::Sttrgen0,
            adtrgenr::Sttrgen0,
            Adtrgenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sttrgen1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adtrgenr::Sttrgen1,
        adtrgenr::Sttrgen1,
        Adtrgenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adtrgenr::Sttrgen1,
            adtrgenr::Sttrgen1,
            Adtrgenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sttrgen2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adtrgenr::Sttrgen2,
        adtrgenr::Sttrgen2,
        Adtrgenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adtrgenr::Sttrgen2,
            adtrgenr::Sttrgen2,
            Adtrgenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sttrgen3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adtrgenr::Sttrgen3,
        adtrgenr::Sttrgen3,
        Adtrgenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adtrgenr::Sttrgen3,
            adtrgenr::Sttrgen3,
            Adtrgenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sttrgen4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adtrgenr::Sttrgen4,
        adtrgenr::Sttrgen4,
        Adtrgenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adtrgenr::Sttrgen4,
            adtrgenr::Sttrgen4,
            Adtrgenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sttrgen5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adtrgenr::Sttrgen5,
        adtrgenr::Sttrgen5,
        Adtrgenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adtrgenr::Sttrgen5,
            adtrgenr::Sttrgen5,
            Adtrgenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sttrgen6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adtrgenr::Sttrgen6,
        adtrgenr::Sttrgen6,
        Adtrgenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adtrgenr::Sttrgen6,
            adtrgenr::Sttrgen6,
            Adtrgenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sttrgen7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adtrgenr::Sttrgen7,
        adtrgenr::Sttrgen7,
        Adtrgenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adtrgenr::Sttrgen7,
            adtrgenr::Sttrgen7,
            Adtrgenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sttrgen8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adtrgenr::Sttrgen8,
        adtrgenr::Sttrgen8,
        Adtrgenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adtrgenr::Sttrgen8,
            adtrgenr::Sttrgen8,
            Adtrgenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adtrgenr {
    #[inline(always)]
    fn default() -> Adtrgenr {
        <crate::RegValueT<Adtrgenr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adtrgenr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sttrgen0_SPEC;
    pub type Sttrgen0 = crate::EnumBitfieldStruct<u8, Sttrgen0_SPEC>;
    impl Sttrgen0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sttrgen1_SPEC;
    pub type Sttrgen1 = crate::EnumBitfieldStruct<u8, Sttrgen1_SPEC>;
    impl Sttrgen1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sttrgen2_SPEC;
    pub type Sttrgen2 = crate::EnumBitfieldStruct<u8, Sttrgen2_SPEC>;
    impl Sttrgen2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sttrgen3_SPEC;
    pub type Sttrgen3 = crate::EnumBitfieldStruct<u8, Sttrgen3_SPEC>;
    impl Sttrgen3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sttrgen4_SPEC;
    pub type Sttrgen4 = crate::EnumBitfieldStruct<u8, Sttrgen4_SPEC>;
    impl Sttrgen4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sttrgen5_SPEC;
    pub type Sttrgen5 = crate::EnumBitfieldStruct<u8, Sttrgen5_SPEC>;
    impl Sttrgen5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sttrgen6_SPEC;
    pub type Sttrgen6 = crate::EnumBitfieldStruct<u8, Sttrgen6_SPEC>;
    impl Sttrgen6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sttrgen7_SPEC;
    pub type Sttrgen7 = crate::EnumBitfieldStruct<u8, Sttrgen7_SPEC>;
    impl Sttrgen7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sttrgen8_SPEC;
    pub type Sttrgen8 = crate::EnumBitfieldStruct<u8, Sttrgen8_SPEC>;
    impl Sttrgen8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsystr_SPEC;
impl crate::sealed::RegSpec for Adsystr_SPEC {
    type DataType = u32;
}

pub type Adsystr = crate::RegValueT<Adsystr_SPEC>;

impl Adsystr {
    #[inline(always)]
    pub fn adsyst0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adsystr::Adsyst0,
        adsystr::Adsyst0,
        Adsystr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adsystr::Adsyst0,
            adsystr::Adsyst0,
            Adsystr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adsyst1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adsystr::Adsyst1,
        adsystr::Adsyst1,
        Adsystr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adsystr::Adsyst1,
            adsystr::Adsyst1,
            Adsystr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adsyst2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adsystr::Adsyst2,
        adsystr::Adsyst2,
        Adsystr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adsystr::Adsyst2,
            adsystr::Adsyst2,
            Adsystr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adsyst3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adsystr::Adsyst3,
        adsystr::Adsyst3,
        Adsystr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adsystr::Adsyst3,
            adsystr::Adsyst3,
            Adsystr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adsyst4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adsystr::Adsyst4,
        adsystr::Adsyst4,
        Adsystr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adsystr::Adsyst4,
            adsystr::Adsyst4,
            Adsystr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adsyst5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adsystr::Adsyst5,
        adsystr::Adsyst5,
        Adsystr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adsystr::Adsyst5,
            adsystr::Adsyst5,
            Adsystr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adsyst6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adsystr::Adsyst6,
        adsystr::Adsyst6,
        Adsystr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adsystr::Adsyst6,
            adsystr::Adsyst6,
            Adsystr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adsyst7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adsystr::Adsyst7,
        adsystr::Adsyst7,
        Adsystr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adsystr::Adsyst7,
            adsystr::Adsyst7,
            Adsystr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adsyst8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adsystr::Adsyst8,
        adsystr::Adsyst8,
        Adsystr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adsystr::Adsyst8,
            adsystr::Adsyst8,
            Adsystr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adsystr {
    #[inline(always)]
    fn default() -> Adsystr {
        <crate::RegValueT<Adsystr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adsystr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsyst0_SPEC;
    pub type Adsyst0 = crate::EnumBitfieldStruct<u8, Adsyst0_SPEC>;
    impl Adsyst0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsyst1_SPEC;
    pub type Adsyst1 = crate::EnumBitfieldStruct<u8, Adsyst1_SPEC>;
    impl Adsyst1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsyst2_SPEC;
    pub type Adsyst2 = crate::EnumBitfieldStruct<u8, Adsyst2_SPEC>;
    impl Adsyst2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsyst3_SPEC;
    pub type Adsyst3 = crate::EnumBitfieldStruct<u8, Adsyst3_SPEC>;
    impl Adsyst3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsyst4_SPEC;
    pub type Adsyst4 = crate::EnumBitfieldStruct<u8, Adsyst4_SPEC>;
    impl Adsyst4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsyst5_SPEC;
    pub type Adsyst5 = crate::EnumBitfieldStruct<u8, Adsyst5_SPEC>;
    impl Adsyst5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsyst6_SPEC;
    pub type Adsyst6 = crate::EnumBitfieldStruct<u8, Adsyst6_SPEC>;
    impl Adsyst6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsyst7_SPEC;
    pub type Adsyst7 = crate::EnumBitfieldStruct<u8, Adsyst7_SPEC>;
    impl Adsyst7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsyst8_SPEC;
    pub type Adsyst8 = crate::EnumBitfieldStruct<u8, Adsyst8_SPEC>;
    impl Adsyst8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adstr_SPEC;
impl crate::sealed::RegSpec for Adstr_SPEC {
    type DataType = u32;
}

pub type Adstr = crate::RegValueT<Adstr_SPEC>;

impl Adstr {
    #[inline(always)]
    pub fn adst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adstr::Adst,
        adstr::Adst,
        Adstr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adstr::Adst,
            adstr::Adst,
            Adstr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adstr {
    #[inline(always)]
    fn default() -> Adstr {
        <crate::RegValueT<Adstr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adstr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adst_SPEC;
    pub type Adst = crate::EnumBitfieldStruct<u8, Adst_SPEC>;
    impl Adst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adstopr_SPEC;
impl crate::sealed::RegSpec for Adstopr_SPEC {
    type DataType = u32;
}

pub type Adstopr = crate::RegValueT<Adstopr_SPEC>;

impl Adstopr {
    #[inline(always)]
    pub fn adstop0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adstopr::Adstop0,
        adstopr::Adstop0,
        Adstopr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adstopr::Adstop0,
            adstopr::Adstop0,
            Adstopr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adstop1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adstopr::Adstop1,
        adstopr::Adstop1,
        Adstopr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adstopr::Adstop1,
            adstopr::Adstop1,
            Adstopr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adstopr {
    #[inline(always)]
    fn default() -> Adstopr {
        <crate::RegValueT<Adstopr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adstopr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adstop0_SPEC;
    pub type Adstop0 = crate::EnumBitfieldStruct<u8, Adstop0_SPEC>;
    impl Adstop0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adstop1_SPEC;
    pub type Adstop1 = crate::EnumBitfieldStruct<u8, Adstop1_SPEC>;
    impl Adstop1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsr_SPEC;
impl crate::sealed::RegSpec for Adsr_SPEC {
    type DataType = u32;
}

pub type Adsr = crate::RegValueT<Adsr_SPEC>;

impl Adsr {
    #[inline(always)]
    pub fn adact0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adsr::Adact0,
        adsr::Adact0,
        Adsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adsr::Adact0,
            adsr::Adact0,
            Adsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adact1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adsr::Adact1,
        adsr::Adact1,
        Adsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adsr::Adact1,
            adsr::Adact1,
            Adsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn calact0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adsr::Calact0,
        adsr::Calact0,
        Adsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adsr::Calact0,
            adsr::Calact0,
            Adsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn calact1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adsr::Calact1,
        adsr::Calact1,
        Adsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adsr::Calact1,
            adsr::Calact1,
            Adsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adsr {
    #[inline(always)]
    fn default() -> Adsr {
        <crate::RegValueT<Adsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adact0_SPEC;
    pub type Adact0 = crate::EnumBitfieldStruct<u8, Adact0_SPEC>;
    impl Adact0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adact1_SPEC;
    pub type Adact1 = crate::EnumBitfieldStruct<u8, Adact1_SPEC>;
    impl Adact1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Calact0_SPEC;
    pub type Calact0 = crate::EnumBitfieldStruct<u8, Calact0_SPEC>;
    impl Calact0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Calact1_SPEC;
    pub type Calact1 = crate::EnumBitfieldStruct<u8, Calact1_SPEC>;
    impl Calact1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adgrsr_SPEC;
impl crate::sealed::RegSpec for Adgrsr_SPEC {
    type DataType = u32;
}

pub type Adgrsr = crate::RegValueT<Adgrsr_SPEC>;

impl Adgrsr {
    #[inline(always)]
    pub fn actgr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adgrsr::Actgr0,
        adgrsr::Actgr0,
        Adgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adgrsr::Actgr0,
            adgrsr::Actgr0,
            Adgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn actgr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adgrsr::Actgr1,
        adgrsr::Actgr1,
        Adgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adgrsr::Actgr1,
            adgrsr::Actgr1,
            Adgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn actgr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adgrsr::Actgr2,
        adgrsr::Actgr2,
        Adgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adgrsr::Actgr2,
            adgrsr::Actgr2,
            Adgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn actgr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adgrsr::Actgr3,
        adgrsr::Actgr3,
        Adgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adgrsr::Actgr3,
            adgrsr::Actgr3,
            Adgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn actgr4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adgrsr::Actgr4,
        adgrsr::Actgr4,
        Adgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adgrsr::Actgr4,
            adgrsr::Actgr4,
            Adgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn actgr5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adgrsr::Actgr5,
        adgrsr::Actgr5,
        Adgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adgrsr::Actgr5,
            adgrsr::Actgr5,
            Adgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn actgr6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adgrsr::Actgr6,
        adgrsr::Actgr6,
        Adgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adgrsr::Actgr6,
            adgrsr::Actgr6,
            Adgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn actgr7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adgrsr::Actgr7,
        adgrsr::Actgr7,
        Adgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adgrsr::Actgr7,
            adgrsr::Actgr7,
            Adgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn actgr8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adgrsr::Actgr8,
        adgrsr::Actgr8,
        Adgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adgrsr::Actgr8,
            adgrsr::Actgr8,
            Adgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adgrsr {
    #[inline(always)]
    fn default() -> Adgrsr {
        <crate::RegValueT<Adgrsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adgrsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actgr0_SPEC;
    pub type Actgr0 = crate::EnumBitfieldStruct<u8, Actgr0_SPEC>;
    impl Actgr0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actgr1_SPEC;
    pub type Actgr1 = crate::EnumBitfieldStruct<u8, Actgr1_SPEC>;
    impl Actgr1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actgr2_SPEC;
    pub type Actgr2 = crate::EnumBitfieldStruct<u8, Actgr2_SPEC>;
    impl Actgr2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actgr3_SPEC;
    pub type Actgr3 = crate::EnumBitfieldStruct<u8, Actgr3_SPEC>;
    impl Actgr3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actgr4_SPEC;
    pub type Actgr4 = crate::EnumBitfieldStruct<u8, Actgr4_SPEC>;
    impl Actgr4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actgr5_SPEC;
    pub type Actgr5 = crate::EnumBitfieldStruct<u8, Actgr5_SPEC>;
    impl Actgr5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actgr6_SPEC;
    pub type Actgr6 = crate::EnumBitfieldStruct<u8, Actgr6_SPEC>;
    impl Actgr6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actgr7_SPEC;
    pub type Actgr7 = crate::EnumBitfieldStruct<u8, Actgr7_SPEC>;
    impl Actgr7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actgr8_SPEC;
    pub type Actgr8 = crate::EnumBitfieldStruct<u8, Actgr8_SPEC>;
    impl Actgr8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adersr_SPEC;
impl crate::sealed::RegSpec for Adersr_SPEC {
    type DataType = u32;
}

pub type Adersr = crate::RegValueT<Adersr_SPEC>;

impl Adersr {
    #[inline(always)]
    pub fn aderf0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adersr::Aderf0,
        adersr::Aderf0,
        Adersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adersr::Aderf0,
            adersr::Aderf0,
            Adersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aderf1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adersr::Aderf1,
        adersr::Aderf1,
        Adersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adersr::Aderf1,
            adersr::Aderf1,
            Adersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adersr {
    #[inline(always)]
    fn default() -> Adersr {
        <crate::RegValueT<Adersr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adersr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aderf0_SPEC;
    pub type Aderf0 = crate::EnumBitfieldStruct<u8, Aderf0_SPEC>;
    impl Aderf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aderf1_SPEC;
    pub type Aderf1 = crate::EnumBitfieldStruct<u8, Aderf1_SPEC>;
    impl Aderf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aderscr_SPEC;
impl crate::sealed::RegSpec for Aderscr_SPEC {
    type DataType = u32;
}

pub type Aderscr = crate::RegValueT<Aderscr_SPEC>;

impl Aderscr {
    #[inline(always)]
    pub fn aderclr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        aderscr::Aderclr0,
        aderscr::Aderclr0,
        Aderscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            aderscr::Aderclr0,
            aderscr::Aderclr0,
            Aderscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aderclr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        aderscr::Aderclr1,
        aderscr::Aderclr1,
        Aderscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            aderscr::Aderclr1,
            aderscr::Aderclr1,
            Aderscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Aderscr {
    #[inline(always)]
    fn default() -> Aderscr {
        <crate::RegValueT<Aderscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod aderscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aderclr0_SPEC;
    pub type Aderclr0 = crate::EnumBitfieldStruct<u8, Aderclr0_SPEC>;
    impl Aderclr0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aderclr1_SPEC;
    pub type Aderclr1 = crate::EnumBitfieldStruct<u8, Aderclr1_SPEC>;
    impl Aderclr1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcalendsr_SPEC;
impl crate::sealed::RegSpec for Adcalendsr_SPEC {
    type DataType = u32;
}

pub type Adcalendsr = crate::RegValueT<Adcalendsr_SPEC>;

impl Adcalendsr {
    #[inline(always)]
    pub fn calendf0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcalendsr::Calendf0,
        adcalendsr::Calendf0,
        Adcalendsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcalendsr::Calendf0,
            adcalendsr::Calendf0,
            Adcalendsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn calendf1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcalendsr::Calendf1,
        adcalendsr::Calendf1,
        Adcalendsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcalendsr::Calendf1,
            adcalendsr::Calendf1,
            Adcalendsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcalendsr {
    #[inline(always)]
    fn default() -> Adcalendsr {
        <crate::RegValueT<Adcalendsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcalendsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Calendf0_SPEC;
    pub type Calendf0 = crate::EnumBitfieldStruct<u8, Calendf0_SPEC>;
    impl Calendf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Calendf1_SPEC;
    pub type Calendf1 = crate::EnumBitfieldStruct<u8, Calendf1_SPEC>;
    impl Calendf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcalendscr_SPEC;
impl crate::sealed::RegSpec for Adcalendscr_SPEC {
    type DataType = u32;
}

pub type Adcalendscr = crate::RegValueT<Adcalendscr_SPEC>;

impl Adcalendscr {
    #[inline(always)]
    pub fn calendc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcalendscr::Calendc0,
        adcalendscr::Calendc0,
        Adcalendscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcalendscr::Calendc0,
            adcalendscr::Calendc0,
            Adcalendscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn calendc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcalendscr::Calendc1,
        adcalendscr::Calendc1,
        Adcalendscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcalendscr::Calendc1,
            adcalendscr::Calendc1,
            Adcalendscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcalendscr {
    #[inline(always)]
    fn default() -> Adcalendscr {
        <crate::RegValueT<Adcalendscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcalendscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Calendc0_SPEC;
    pub type Calendc0 = crate::EnumBitfieldStruct<u8, Calendc0_SPEC>;
    impl Calendc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Calendc1_SPEC;
    pub type Calendc1 = crate::EnumBitfieldStruct<u8, Calendc1_SPEC>;
    impl Calendc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adovfersr_SPEC;
impl crate::sealed::RegSpec for Adovfersr_SPEC {
    type DataType = u32;
}

pub type Adovfersr = crate::RegValueT<Adovfersr_SPEC>;

impl Adovfersr {
    #[inline(always)]
    pub fn adovfef0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adovfersr::Adovfef0,
        adovfersr::Adovfef0,
        Adovfersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adovfersr::Adovfef0,
            adovfersr::Adovfef0,
            Adovfersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adovfef1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adovfersr::Adovfef1,
        adovfersr::Adovfef1,
        Adovfersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adovfersr::Adovfef1,
            adovfersr::Adovfef1,
            Adovfersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adovfersr {
    #[inline(always)]
    fn default() -> Adovfersr {
        <crate::RegValueT<Adovfersr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adovfersr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adovfef0_SPEC;
    pub type Adovfef0 = crate::EnumBitfieldStruct<u8, Adovfef0_SPEC>;
    impl Adovfef0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adovfef1_SPEC;
    pub type Adovfef1 = crate::EnumBitfieldStruct<u8, Adovfef1_SPEC>;
    impl Adovfef1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adovfchsr0_SPEC;
impl crate::sealed::RegSpec for Adovfchsr0_SPEC {
    type DataType = u32;
}

pub type Adovfchsr0 = crate::RegValueT<Adovfchsr0_SPEC>;

impl Adovfchsr0 {
    #[inline(always)]
    pub fn ovfchf0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf0,
        adovfchsr0::Ovfchf0,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf0,
            adovfchsr0::Ovfchf0,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf1,
        adovfchsr0::Ovfchf1,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf1,
            adovfchsr0::Ovfchf1,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf2,
        adovfchsr0::Ovfchf2,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf2,
            adovfchsr0::Ovfchf2,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf3,
        adovfchsr0::Ovfchf3,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf3,
            adovfchsr0::Ovfchf3,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf4,
        adovfchsr0::Ovfchf4,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf4,
            adovfchsr0::Ovfchf4,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf5,
        adovfchsr0::Ovfchf5,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf5,
            adovfchsr0::Ovfchf5,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf6,
        adovfchsr0::Ovfchf6,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf6,
            adovfchsr0::Ovfchf6,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf7,
        adovfchsr0::Ovfchf7,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf7,
            adovfchsr0::Ovfchf7,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf8,
        adovfchsr0::Ovfchf8,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf8,
            adovfchsr0::Ovfchf8,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf9,
        adovfchsr0::Ovfchf9,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf9,
            adovfchsr0::Ovfchf9,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf10,
        adovfchsr0::Ovfchf10,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf10,
            adovfchsr0::Ovfchf10,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf11,
        adovfchsr0::Ovfchf11,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf11,
            adovfchsr0::Ovfchf11,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf12,
        adovfchsr0::Ovfchf12,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf12,
            adovfchsr0::Ovfchf12,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf13,
        adovfchsr0::Ovfchf13,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf13,
            adovfchsr0::Ovfchf13,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf14,
        adovfchsr0::Ovfchf14,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf14,
            adovfchsr0::Ovfchf14,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf15,
        adovfchsr0::Ovfchf15,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf15,
            adovfchsr0::Ovfchf15,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf16,
        adovfchsr0::Ovfchf16,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf16,
            adovfchsr0::Ovfchf16,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf17,
        adovfchsr0::Ovfchf17,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf17,
            adovfchsr0::Ovfchf17,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf18,
        adovfchsr0::Ovfchf18,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf18,
            adovfchsr0::Ovfchf18,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf19,
        adovfchsr0::Ovfchf19,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf19,
            adovfchsr0::Ovfchf19,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf20,
        adovfchsr0::Ovfchf20,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf20,
            adovfchsr0::Ovfchf20,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf21,
        adovfchsr0::Ovfchf21,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf21,
            adovfchsr0::Ovfchf21,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf22,
        adovfchsr0::Ovfchf22,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf22,
            adovfchsr0::Ovfchf22,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf23,
        adovfchsr0::Ovfchf23,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf23,
            adovfchsr0::Ovfchf23,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf24,
        adovfchsr0::Ovfchf24,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf24,
            adovfchsr0::Ovfchf24,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf25,
        adovfchsr0::Ovfchf25,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf25,
            adovfchsr0::Ovfchf25,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf26,
        adovfchsr0::Ovfchf26,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf26,
            adovfchsr0::Ovfchf26,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf27,
        adovfchsr0::Ovfchf27,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf27,
            adovfchsr0::Ovfchf27,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchf28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        adovfchsr0::Ovfchf28,
        adovfchsr0::Ovfchf28,
        Adovfchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            adovfchsr0::Ovfchf28,
            adovfchsr0::Ovfchf28,
            Adovfchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adovfchsr0 {
    #[inline(always)]
    fn default() -> Adovfchsr0 {
        <crate::RegValueT<Adovfchsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adovfchsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf0_SPEC;
    pub type Ovfchf0 = crate::EnumBitfieldStruct<u8, Ovfchf0_SPEC>;
    impl Ovfchf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf1_SPEC;
    pub type Ovfchf1 = crate::EnumBitfieldStruct<u8, Ovfchf1_SPEC>;
    impl Ovfchf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf2_SPEC;
    pub type Ovfchf2 = crate::EnumBitfieldStruct<u8, Ovfchf2_SPEC>;
    impl Ovfchf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf3_SPEC;
    pub type Ovfchf3 = crate::EnumBitfieldStruct<u8, Ovfchf3_SPEC>;
    impl Ovfchf3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf4_SPEC;
    pub type Ovfchf4 = crate::EnumBitfieldStruct<u8, Ovfchf4_SPEC>;
    impl Ovfchf4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf5_SPEC;
    pub type Ovfchf5 = crate::EnumBitfieldStruct<u8, Ovfchf5_SPEC>;
    impl Ovfchf5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf6_SPEC;
    pub type Ovfchf6 = crate::EnumBitfieldStruct<u8, Ovfchf6_SPEC>;
    impl Ovfchf6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf7_SPEC;
    pub type Ovfchf7 = crate::EnumBitfieldStruct<u8, Ovfchf7_SPEC>;
    impl Ovfchf7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf8_SPEC;
    pub type Ovfchf8 = crate::EnumBitfieldStruct<u8, Ovfchf8_SPEC>;
    impl Ovfchf8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf9_SPEC;
    pub type Ovfchf9 = crate::EnumBitfieldStruct<u8, Ovfchf9_SPEC>;
    impl Ovfchf9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf10_SPEC;
    pub type Ovfchf10 = crate::EnumBitfieldStruct<u8, Ovfchf10_SPEC>;
    impl Ovfchf10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf11_SPEC;
    pub type Ovfchf11 = crate::EnumBitfieldStruct<u8, Ovfchf11_SPEC>;
    impl Ovfchf11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf12_SPEC;
    pub type Ovfchf12 = crate::EnumBitfieldStruct<u8, Ovfchf12_SPEC>;
    impl Ovfchf12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf13_SPEC;
    pub type Ovfchf13 = crate::EnumBitfieldStruct<u8, Ovfchf13_SPEC>;
    impl Ovfchf13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf14_SPEC;
    pub type Ovfchf14 = crate::EnumBitfieldStruct<u8, Ovfchf14_SPEC>;
    impl Ovfchf14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf15_SPEC;
    pub type Ovfchf15 = crate::EnumBitfieldStruct<u8, Ovfchf15_SPEC>;
    impl Ovfchf15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf16_SPEC;
    pub type Ovfchf16 = crate::EnumBitfieldStruct<u8, Ovfchf16_SPEC>;
    impl Ovfchf16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf17_SPEC;
    pub type Ovfchf17 = crate::EnumBitfieldStruct<u8, Ovfchf17_SPEC>;
    impl Ovfchf17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf18_SPEC;
    pub type Ovfchf18 = crate::EnumBitfieldStruct<u8, Ovfchf18_SPEC>;
    impl Ovfchf18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf19_SPEC;
    pub type Ovfchf19 = crate::EnumBitfieldStruct<u8, Ovfchf19_SPEC>;
    impl Ovfchf19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf20_SPEC;
    pub type Ovfchf20 = crate::EnumBitfieldStruct<u8, Ovfchf20_SPEC>;
    impl Ovfchf20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf21_SPEC;
    pub type Ovfchf21 = crate::EnumBitfieldStruct<u8, Ovfchf21_SPEC>;
    impl Ovfchf21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf22_SPEC;
    pub type Ovfchf22 = crate::EnumBitfieldStruct<u8, Ovfchf22_SPEC>;
    impl Ovfchf22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf23_SPEC;
    pub type Ovfchf23 = crate::EnumBitfieldStruct<u8, Ovfchf23_SPEC>;
    impl Ovfchf23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf24_SPEC;
    pub type Ovfchf24 = crate::EnumBitfieldStruct<u8, Ovfchf24_SPEC>;
    impl Ovfchf24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf25_SPEC;
    pub type Ovfchf25 = crate::EnumBitfieldStruct<u8, Ovfchf25_SPEC>;
    impl Ovfchf25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf26_SPEC;
    pub type Ovfchf26 = crate::EnumBitfieldStruct<u8, Ovfchf26_SPEC>;
    impl Ovfchf26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf27_SPEC;
    pub type Ovfchf27 = crate::EnumBitfieldStruct<u8, Ovfchf27_SPEC>;
    impl Ovfchf27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchf28_SPEC;
    pub type Ovfchf28 = crate::EnumBitfieldStruct<u8, Ovfchf28_SPEC>;
    impl Ovfchf28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adovfexsr_SPEC;
impl crate::sealed::RegSpec for Adovfexsr_SPEC {
    type DataType = u32;
}

pub type Adovfexsr = crate::RegValueT<Adovfexsr_SPEC>;

impl Adovfexsr {
    #[inline(always)]
    pub fn ovfexf0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adovfexsr::Ovfexf0,
        adovfexsr::Ovfexf0,
        Adovfexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adovfexsr::Ovfexf0,
            adovfexsr::Ovfexf0,
            Adovfexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfexf1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adovfexsr::Ovfexf1,
        adovfexsr::Ovfexf1,
        Adovfexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adovfexsr::Ovfexf1,
            adovfexsr::Ovfexf1,
            Adovfexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfexf2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adovfexsr::Ovfexf2,
        adovfexsr::Ovfexf2,
        Adovfexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adovfexsr::Ovfexf2,
            adovfexsr::Ovfexf2,
            Adovfexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfexf5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adovfexsr::Ovfexf5,
        adovfexsr::Ovfexf5,
        Adovfexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adovfexsr::Ovfexf5,
            adovfexsr::Ovfexf5,
            Adovfexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfexf6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adovfexsr::Ovfexf6,
        adovfexsr::Ovfexf6,
        Adovfexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adovfexsr::Ovfexf6,
            adovfexsr::Ovfexf6,
            Adovfexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfexf7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adovfexsr::Ovfexf7,
        adovfexsr::Ovfexf7,
        Adovfexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adovfexsr::Ovfexf7,
            adovfexsr::Ovfexf7,
            Adovfexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfexf8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adovfexsr::Ovfexf8,
        adovfexsr::Ovfexf8,
        Adovfexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adovfexsr::Ovfexf8,
            adovfexsr::Ovfexf8,
            Adovfexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adovfexsr {
    #[inline(always)]
    fn default() -> Adovfexsr {
        <crate::RegValueT<Adovfexsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adovfexsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfexf0_SPEC;
    pub type Ovfexf0 = crate::EnumBitfieldStruct<u8, Ovfexf0_SPEC>;
    impl Ovfexf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfexf1_SPEC;
    pub type Ovfexf1 = crate::EnumBitfieldStruct<u8, Ovfexf1_SPEC>;
    impl Ovfexf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfexf2_SPEC;
    pub type Ovfexf2 = crate::EnumBitfieldStruct<u8, Ovfexf2_SPEC>;
    impl Ovfexf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfexf5_SPEC;
    pub type Ovfexf5 = crate::EnumBitfieldStruct<u8, Ovfexf5_SPEC>;
    impl Ovfexf5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfexf6_SPEC;
    pub type Ovfexf6 = crate::EnumBitfieldStruct<u8, Ovfexf6_SPEC>;
    impl Ovfexf6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfexf7_SPEC;
    pub type Ovfexf7 = crate::EnumBitfieldStruct<u8, Ovfexf7_SPEC>;
    impl Ovfexf7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfexf8_SPEC;
    pub type Ovfexf8 = crate::EnumBitfieldStruct<u8, Ovfexf8_SPEC>;
    impl Ovfexf8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adovferscr_SPEC;
impl crate::sealed::RegSpec for Adovferscr_SPEC {
    type DataType = u32;
}

pub type Adovferscr = crate::RegValueT<Adovferscr_SPEC>;

impl Adovferscr {
    #[inline(always)]
    pub fn adovfec0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adovferscr::Adovfec0,
        adovferscr::Adovfec0,
        Adovferscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adovferscr::Adovfec0,
            adovferscr::Adovfec0,
            Adovferscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adovfec1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adovferscr::Adovfec1,
        adovferscr::Adovfec1,
        Adovferscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adovferscr::Adovfec1,
            adovferscr::Adovfec1,
            Adovferscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adovferscr {
    #[inline(always)]
    fn default() -> Adovferscr {
        <crate::RegValueT<Adovferscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adovferscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adovfec0_SPEC;
    pub type Adovfec0 = crate::EnumBitfieldStruct<u8, Adovfec0_SPEC>;
    impl Adovfec0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adovfec1_SPEC;
    pub type Adovfec1 = crate::EnumBitfieldStruct<u8, Adovfec1_SPEC>;
    impl Adovfec1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adovfchscr0_SPEC;
impl crate::sealed::RegSpec for Adovfchscr0_SPEC {
    type DataType = u32;
}

pub type Adovfchscr0 = crate::RegValueT<Adovfchscr0_SPEC>;

impl Adovfchscr0 {
    #[inline(always)]
    pub fn ovfchc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc0,
        adovfchscr0::Ovfchc0,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc0,
            adovfchscr0::Ovfchc0,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc1,
        adovfchscr0::Ovfchc1,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc1,
            adovfchscr0::Ovfchc1,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc2,
        adovfchscr0::Ovfchc2,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc2,
            adovfchscr0::Ovfchc2,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc3,
        adovfchscr0::Ovfchc3,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc3,
            adovfchscr0::Ovfchc3,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc4,
        adovfchscr0::Ovfchc4,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc4,
            adovfchscr0::Ovfchc4,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc5,
        adovfchscr0::Ovfchc5,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc5,
            adovfchscr0::Ovfchc5,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc6,
        adovfchscr0::Ovfchc6,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc6,
            adovfchscr0::Ovfchc6,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc7,
        adovfchscr0::Ovfchc7,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc7,
            adovfchscr0::Ovfchc7,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc8,
        adovfchscr0::Ovfchc8,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc8,
            adovfchscr0::Ovfchc8,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc9,
        adovfchscr0::Ovfchc9,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc9,
            adovfchscr0::Ovfchc9,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc10,
        adovfchscr0::Ovfchc10,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc10,
            adovfchscr0::Ovfchc10,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc11,
        adovfchscr0::Ovfchc11,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc11,
            adovfchscr0::Ovfchc11,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc12,
        adovfchscr0::Ovfchc12,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc12,
            adovfchscr0::Ovfchc12,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc13,
        adovfchscr0::Ovfchc13,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc13,
            adovfchscr0::Ovfchc13,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc14,
        adovfchscr0::Ovfchc14,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc14,
            adovfchscr0::Ovfchc14,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc15,
        adovfchscr0::Ovfchc15,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc15,
            adovfchscr0::Ovfchc15,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc16,
        adovfchscr0::Ovfchc16,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc16,
            adovfchscr0::Ovfchc16,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc17,
        adovfchscr0::Ovfchc17,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc17,
            adovfchscr0::Ovfchc17,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc18,
        adovfchscr0::Ovfchc18,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc18,
            adovfchscr0::Ovfchc18,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc19,
        adovfchscr0::Ovfchc19,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc19,
            adovfchscr0::Ovfchc19,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc20,
        adovfchscr0::Ovfchc20,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc20,
            adovfchscr0::Ovfchc20,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc21,
        adovfchscr0::Ovfchc21,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc21,
            adovfchscr0::Ovfchc21,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc22,
        adovfchscr0::Ovfchc22,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc22,
            adovfchscr0::Ovfchc22,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc23,
        adovfchscr0::Ovfchc23,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc23,
            adovfchscr0::Ovfchc23,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc24,
        adovfchscr0::Ovfchc24,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc24,
            adovfchscr0::Ovfchc24,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc25,
        adovfchscr0::Ovfchc25,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc25,
            adovfchscr0::Ovfchc25,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc26,
        adovfchscr0::Ovfchc26,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc26,
            adovfchscr0::Ovfchc26,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc27,
        adovfchscr0::Ovfchc27,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc27,
            adovfchscr0::Ovfchc27,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfchc28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        adovfchscr0::Ovfchc28,
        adovfchscr0::Ovfchc28,
        Adovfchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            adovfchscr0::Ovfchc28,
            adovfchscr0::Ovfchc28,
            Adovfchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adovfchscr0 {
    #[inline(always)]
    fn default() -> Adovfchscr0 {
        <crate::RegValueT<Adovfchscr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adovfchscr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc0_SPEC;
    pub type Ovfchc0 = crate::EnumBitfieldStruct<u8, Ovfchc0_SPEC>;
    impl Ovfchc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc1_SPEC;
    pub type Ovfchc1 = crate::EnumBitfieldStruct<u8, Ovfchc1_SPEC>;
    impl Ovfchc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc2_SPEC;
    pub type Ovfchc2 = crate::EnumBitfieldStruct<u8, Ovfchc2_SPEC>;
    impl Ovfchc2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc3_SPEC;
    pub type Ovfchc3 = crate::EnumBitfieldStruct<u8, Ovfchc3_SPEC>;
    impl Ovfchc3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc4_SPEC;
    pub type Ovfchc4 = crate::EnumBitfieldStruct<u8, Ovfchc4_SPEC>;
    impl Ovfchc4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc5_SPEC;
    pub type Ovfchc5 = crate::EnumBitfieldStruct<u8, Ovfchc5_SPEC>;
    impl Ovfchc5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc6_SPEC;
    pub type Ovfchc6 = crate::EnumBitfieldStruct<u8, Ovfchc6_SPEC>;
    impl Ovfchc6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc7_SPEC;
    pub type Ovfchc7 = crate::EnumBitfieldStruct<u8, Ovfchc7_SPEC>;
    impl Ovfchc7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc8_SPEC;
    pub type Ovfchc8 = crate::EnumBitfieldStruct<u8, Ovfchc8_SPEC>;
    impl Ovfchc8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc9_SPEC;
    pub type Ovfchc9 = crate::EnumBitfieldStruct<u8, Ovfchc9_SPEC>;
    impl Ovfchc9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc10_SPEC;
    pub type Ovfchc10 = crate::EnumBitfieldStruct<u8, Ovfchc10_SPEC>;
    impl Ovfchc10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc11_SPEC;
    pub type Ovfchc11 = crate::EnumBitfieldStruct<u8, Ovfchc11_SPEC>;
    impl Ovfchc11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc12_SPEC;
    pub type Ovfchc12 = crate::EnumBitfieldStruct<u8, Ovfchc12_SPEC>;
    impl Ovfchc12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc13_SPEC;
    pub type Ovfchc13 = crate::EnumBitfieldStruct<u8, Ovfchc13_SPEC>;
    impl Ovfchc13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc14_SPEC;
    pub type Ovfchc14 = crate::EnumBitfieldStruct<u8, Ovfchc14_SPEC>;
    impl Ovfchc14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc15_SPEC;
    pub type Ovfchc15 = crate::EnumBitfieldStruct<u8, Ovfchc15_SPEC>;
    impl Ovfchc15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc16_SPEC;
    pub type Ovfchc16 = crate::EnumBitfieldStruct<u8, Ovfchc16_SPEC>;
    impl Ovfchc16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc17_SPEC;
    pub type Ovfchc17 = crate::EnumBitfieldStruct<u8, Ovfchc17_SPEC>;
    impl Ovfchc17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc18_SPEC;
    pub type Ovfchc18 = crate::EnumBitfieldStruct<u8, Ovfchc18_SPEC>;
    impl Ovfchc18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc19_SPEC;
    pub type Ovfchc19 = crate::EnumBitfieldStruct<u8, Ovfchc19_SPEC>;
    impl Ovfchc19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc20_SPEC;
    pub type Ovfchc20 = crate::EnumBitfieldStruct<u8, Ovfchc20_SPEC>;
    impl Ovfchc20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc21_SPEC;
    pub type Ovfchc21 = crate::EnumBitfieldStruct<u8, Ovfchc21_SPEC>;
    impl Ovfchc21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc22_SPEC;
    pub type Ovfchc22 = crate::EnumBitfieldStruct<u8, Ovfchc22_SPEC>;
    impl Ovfchc22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc23_SPEC;
    pub type Ovfchc23 = crate::EnumBitfieldStruct<u8, Ovfchc23_SPEC>;
    impl Ovfchc23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc24_SPEC;
    pub type Ovfchc24 = crate::EnumBitfieldStruct<u8, Ovfchc24_SPEC>;
    impl Ovfchc24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc25_SPEC;
    pub type Ovfchc25 = crate::EnumBitfieldStruct<u8, Ovfchc25_SPEC>;
    impl Ovfchc25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc26_SPEC;
    pub type Ovfchc26 = crate::EnumBitfieldStruct<u8, Ovfchc26_SPEC>;
    impl Ovfchc26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc27_SPEC;
    pub type Ovfchc27 = crate::EnumBitfieldStruct<u8, Ovfchc27_SPEC>;
    impl Ovfchc27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfchc28_SPEC;
    pub type Ovfchc28 = crate::EnumBitfieldStruct<u8, Ovfchc28_SPEC>;
    impl Ovfchc28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adovfexscr_SPEC;
impl crate::sealed::RegSpec for Adovfexscr_SPEC {
    type DataType = u32;
}

pub type Adovfexscr = crate::RegValueT<Adovfexscr_SPEC>;

impl Adovfexscr {
    #[inline(always)]
    pub fn ovfexc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adovfexscr::Ovfexc0,
        adovfexscr::Ovfexc0,
        Adovfexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adovfexscr::Ovfexc0,
            adovfexscr::Ovfexc0,
            Adovfexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfexc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adovfexscr::Ovfexc1,
        adovfexscr::Ovfexc1,
        Adovfexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adovfexscr::Ovfexc1,
            adovfexscr::Ovfexc1,
            Adovfexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfexc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adovfexscr::Ovfexc2,
        adovfexscr::Ovfexc2,
        Adovfexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adovfexscr::Ovfexc2,
            adovfexscr::Ovfexc2,
            Adovfexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfexc5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adovfexscr::Ovfexc5,
        adovfexscr::Ovfexc5,
        Adovfexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adovfexscr::Ovfexc5,
            adovfexscr::Ovfexc5,
            Adovfexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfexc6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adovfexscr::Ovfexc6,
        adovfexscr::Ovfexc6,
        Adovfexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adovfexscr::Ovfexc6,
            adovfexscr::Ovfexc6,
            Adovfexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfexc7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adovfexscr::Ovfexc7,
        adovfexscr::Ovfexc7,
        Adovfexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adovfexscr::Ovfexc7,
            adovfexscr::Ovfexc7,
            Adovfexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovfexc8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adovfexscr::Ovfexc8,
        adovfexscr::Ovfexc8,
        Adovfexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adovfexscr::Ovfexc8,
            adovfexscr::Ovfexc8,
            Adovfexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adovfexscr {
    #[inline(always)]
    fn default() -> Adovfexscr {
        <crate::RegValueT<Adovfexscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adovfexscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfexc0_SPEC;
    pub type Ovfexc0 = crate::EnumBitfieldStruct<u8, Ovfexc0_SPEC>;
    impl Ovfexc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfexc1_SPEC;
    pub type Ovfexc1 = crate::EnumBitfieldStruct<u8, Ovfexc1_SPEC>;
    impl Ovfexc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfexc2_SPEC;
    pub type Ovfexc2 = crate::EnumBitfieldStruct<u8, Ovfexc2_SPEC>;
    impl Ovfexc2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfexc5_SPEC;
    pub type Ovfexc5 = crate::EnumBitfieldStruct<u8, Ovfexc5_SPEC>;
    impl Ovfexc5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfexc6_SPEC;
    pub type Ovfexc6 = crate::EnumBitfieldStruct<u8, Ovfexc6_SPEC>;
    impl Ovfexc6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfexc7_SPEC;
    pub type Ovfexc7 = crate::EnumBitfieldStruct<u8, Ovfexc7_SPEC>;
    impl Ovfexc7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfexc8_SPEC;
    pub type Ovfexc8 = crate::EnumBitfieldStruct<u8, Ovfexc8_SPEC>;
    impl Ovfexc8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifosr0_SPEC;
impl crate::sealed::RegSpec for Adfifosr0_SPEC {
    type DataType = u32;
}

pub type Adfifosr0 = crate::RegValueT<Adfifosr0_SPEC>;

impl Adfifosr0 {
    #[inline(always)]
    pub fn fifost0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Adfifosr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Adfifosr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fifost1(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Adfifosr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Adfifosr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Adfifosr0 {
    #[inline(always)]
    fn default() -> Adfifosr0 {
        <crate::RegValueT<Adfifosr0_SPEC> as RegisterValue<_>>::new(524296)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifosr1_SPEC;
impl crate::sealed::RegSpec for Adfifosr1_SPEC {
    type DataType = u32;
}

pub type Adfifosr1 = crate::RegValueT<Adfifosr1_SPEC>;

impl Adfifosr1 {
    #[inline(always)]
    pub fn fifost2(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Adfifosr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Adfifosr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fifost3(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Adfifosr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Adfifosr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Adfifosr1 {
    #[inline(always)]
    fn default() -> Adfifosr1 {
        <crate::RegValueT<Adfifosr1_SPEC> as RegisterValue<_>>::new(524296)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifosr2_SPEC;
impl crate::sealed::RegSpec for Adfifosr2_SPEC {
    type DataType = u32;
}

pub type Adfifosr2 = crate::RegValueT<Adfifosr2_SPEC>;

impl Adfifosr2 {
    #[inline(always)]
    pub fn fifost4(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Adfifosr2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Adfifosr2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fifost5(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Adfifosr2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Adfifosr2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Adfifosr2 {
    #[inline(always)]
    fn default() -> Adfifosr2 {
        <crate::RegValueT<Adfifosr2_SPEC> as RegisterValue<_>>::new(524296)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifosr3_SPEC;
impl crate::sealed::RegSpec for Adfifosr3_SPEC {
    type DataType = u32;
}

pub type Adfifosr3 = crate::RegValueT<Adfifosr3_SPEC>;

impl Adfifosr3 {
    #[inline(always)]
    pub fn fifost6(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Adfifosr3_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Adfifosr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fifost7(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Adfifosr3_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Adfifosr3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Adfifosr3 {
    #[inline(always)]
    fn default() -> Adfifosr3 {
        <crate::RegValueT<Adfifosr3_SPEC> as RegisterValue<_>>::new(524296)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifosr4_SPEC;
impl crate::sealed::RegSpec for Adfifosr4_SPEC {
    type DataType = u32;
}

pub type Adfifosr4 = crate::RegValueT<Adfifosr4_SPEC>;

impl Adfifosr4 {
    #[inline(always)]
    pub fn fifost8(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Adfifosr4_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Adfifosr4_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Adfifosr4 {
    #[inline(always)]
    fn default() -> Adfifosr4 {
        <crate::RegValueT<Adfifosr4_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifodcr_SPEC;
impl crate::sealed::RegSpec for Adfifodcr_SPEC {
    type DataType = u32;
}

pub type Adfifodcr = crate::RegValueT<Adfifodcr_SPEC>;

impl Adfifodcr {
    #[inline(always)]
    pub fn fifodc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adfifodcr::Fifodc0,
        adfifodcr::Fifodc0,
        Adfifodcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adfifodcr::Fifodc0,
            adfifodcr::Fifodc0,
            Adfifodcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifodc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adfifodcr::Fifodc1,
        adfifodcr::Fifodc1,
        Adfifodcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adfifodcr::Fifodc1,
            adfifodcr::Fifodc1,
            Adfifodcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifodc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adfifodcr::Fifodc2,
        adfifodcr::Fifodc2,
        Adfifodcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adfifodcr::Fifodc2,
            adfifodcr::Fifodc2,
            Adfifodcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifodc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adfifodcr::Fifodc3,
        adfifodcr::Fifodc3,
        Adfifodcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adfifodcr::Fifodc3,
            adfifodcr::Fifodc3,
            Adfifodcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifodc4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adfifodcr::Fifodc4,
        adfifodcr::Fifodc4,
        Adfifodcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adfifodcr::Fifodc4,
            adfifodcr::Fifodc4,
            Adfifodcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifodc5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adfifodcr::Fifodc5,
        adfifodcr::Fifodc5,
        Adfifodcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adfifodcr::Fifodc5,
            adfifodcr::Fifodc5,
            Adfifodcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifodc6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adfifodcr::Fifodc6,
        adfifodcr::Fifodc6,
        Adfifodcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adfifodcr::Fifodc6,
            adfifodcr::Fifodc6,
            Adfifodcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifodc7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adfifodcr::Fifodc7,
        adfifodcr::Fifodc7,
        Adfifodcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adfifodcr::Fifodc7,
            adfifodcr::Fifodc7,
            Adfifodcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifodc8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adfifodcr::Fifodc8,
        adfifodcr::Fifodc8,
        Adfifodcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adfifodcr::Fifodc8,
            adfifodcr::Fifodc8,
            Adfifodcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adfifodcr {
    #[inline(always)]
    fn default() -> Adfifodcr {
        <crate::RegValueT<Adfifodcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adfifodcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifodc0_SPEC;
    pub type Fifodc0 = crate::EnumBitfieldStruct<u8, Fifodc0_SPEC>;
    impl Fifodc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifodc1_SPEC;
    pub type Fifodc1 = crate::EnumBitfieldStruct<u8, Fifodc1_SPEC>;
    impl Fifodc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifodc2_SPEC;
    pub type Fifodc2 = crate::EnumBitfieldStruct<u8, Fifodc2_SPEC>;
    impl Fifodc2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifodc3_SPEC;
    pub type Fifodc3 = crate::EnumBitfieldStruct<u8, Fifodc3_SPEC>;
    impl Fifodc3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifodc4_SPEC;
    pub type Fifodc4 = crate::EnumBitfieldStruct<u8, Fifodc4_SPEC>;
    impl Fifodc4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifodc5_SPEC;
    pub type Fifodc5 = crate::EnumBitfieldStruct<u8, Fifodc5_SPEC>;
    impl Fifodc5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifodc6_SPEC;
    pub type Fifodc6 = crate::EnumBitfieldStruct<u8, Fifodc6_SPEC>;
    impl Fifodc6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifodc7_SPEC;
    pub type Fifodc7 = crate::EnumBitfieldStruct<u8, Fifodc7_SPEC>;
    impl Fifodc7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifodc8_SPEC;
    pub type Fifodc8 = crate::EnumBitfieldStruct<u8, Fifodc8_SPEC>;
    impl Fifodc8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifoersr_SPEC;
impl crate::sealed::RegSpec for Adfifoersr_SPEC {
    type DataType = u32;
}

pub type Adfifoersr = crate::RegValueT<Adfifoersr_SPEC>;

impl Adfifoersr {
    #[inline(always)]
    pub fn fifoovf0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adfifoersr::Fifoovf0,
        adfifoersr::Fifoovf0,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adfifoersr::Fifoovf0,
            adfifoersr::Fifoovf0,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovf1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adfifoersr::Fifoovf1,
        adfifoersr::Fifoovf1,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adfifoersr::Fifoovf1,
            adfifoersr::Fifoovf1,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovf2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adfifoersr::Fifoovf2,
        adfifoersr::Fifoovf2,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adfifoersr::Fifoovf2,
            adfifoersr::Fifoovf2,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovf3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adfifoersr::Fifoovf3,
        adfifoersr::Fifoovf3,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adfifoersr::Fifoovf3,
            adfifoersr::Fifoovf3,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovf4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adfifoersr::Fifoovf4,
        adfifoersr::Fifoovf4,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adfifoersr::Fifoovf4,
            adfifoersr::Fifoovf4,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovf5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adfifoersr::Fifoovf5,
        adfifoersr::Fifoovf5,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adfifoersr::Fifoovf5,
            adfifoersr::Fifoovf5,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovf6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adfifoersr::Fifoovf6,
        adfifoersr::Fifoovf6,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adfifoersr::Fifoovf6,
            adfifoersr::Fifoovf6,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovf7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adfifoersr::Fifoovf7,
        adfifoersr::Fifoovf7,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adfifoersr::Fifoovf7,
            adfifoersr::Fifoovf7,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovf8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adfifoersr::Fifoovf8,
        adfifoersr::Fifoovf8,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adfifoersr::Fifoovf8,
            adfifoersr::Fifoovf8,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflf0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adfifoersr::Fifoflf0,
        adfifoersr::Fifoflf0,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adfifoersr::Fifoflf0,
            adfifoersr::Fifoflf0,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflf1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adfifoersr::Fifoflf1,
        adfifoersr::Fifoflf1,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adfifoersr::Fifoflf1,
            adfifoersr::Fifoflf1,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflf2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        adfifoersr::Fifoflf2,
        adfifoersr::Fifoflf2,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            adfifoersr::Fifoflf2,
            adfifoersr::Fifoflf2,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflf3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        adfifoersr::Fifoflf3,
        adfifoersr::Fifoflf3,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            adfifoersr::Fifoflf3,
            adfifoersr::Fifoflf3,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflf4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        adfifoersr::Fifoflf4,
        adfifoersr::Fifoflf4,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            adfifoersr::Fifoflf4,
            adfifoersr::Fifoflf4,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflf5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        adfifoersr::Fifoflf5,
        adfifoersr::Fifoflf5,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            adfifoersr::Fifoflf5,
            adfifoersr::Fifoflf5,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflf6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        adfifoersr::Fifoflf6,
        adfifoersr::Fifoflf6,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            adfifoersr::Fifoflf6,
            adfifoersr::Fifoflf6,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflf7(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        adfifoersr::Fifoflf7,
        adfifoersr::Fifoflf7,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            adfifoersr::Fifoflf7,
            adfifoersr::Fifoflf7,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflf8(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        adfifoersr::Fifoflf8,
        adfifoersr::Fifoflf8,
        Adfifoersr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            adfifoersr::Fifoflf8,
            adfifoersr::Fifoflf8,
            Adfifoersr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adfifoersr {
    #[inline(always)]
    fn default() -> Adfifoersr {
        <crate::RegValueT<Adfifoersr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adfifoersr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovf0_SPEC;
    pub type Fifoovf0 = crate::EnumBitfieldStruct<u8, Fifoovf0_SPEC>;
    impl Fifoovf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovf1_SPEC;
    pub type Fifoovf1 = crate::EnumBitfieldStruct<u8, Fifoovf1_SPEC>;
    impl Fifoovf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovf2_SPEC;
    pub type Fifoovf2 = crate::EnumBitfieldStruct<u8, Fifoovf2_SPEC>;
    impl Fifoovf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovf3_SPEC;
    pub type Fifoovf3 = crate::EnumBitfieldStruct<u8, Fifoovf3_SPEC>;
    impl Fifoovf3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovf4_SPEC;
    pub type Fifoovf4 = crate::EnumBitfieldStruct<u8, Fifoovf4_SPEC>;
    impl Fifoovf4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovf5_SPEC;
    pub type Fifoovf5 = crate::EnumBitfieldStruct<u8, Fifoovf5_SPEC>;
    impl Fifoovf5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovf6_SPEC;
    pub type Fifoovf6 = crate::EnumBitfieldStruct<u8, Fifoovf6_SPEC>;
    impl Fifoovf6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovf7_SPEC;
    pub type Fifoovf7 = crate::EnumBitfieldStruct<u8, Fifoovf7_SPEC>;
    impl Fifoovf7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovf8_SPEC;
    pub type Fifoovf8 = crate::EnumBitfieldStruct<u8, Fifoovf8_SPEC>;
    impl Fifoovf8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflf0_SPEC;
    pub type Fifoflf0 = crate::EnumBitfieldStruct<u8, Fifoflf0_SPEC>;
    impl Fifoflf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflf1_SPEC;
    pub type Fifoflf1 = crate::EnumBitfieldStruct<u8, Fifoflf1_SPEC>;
    impl Fifoflf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflf2_SPEC;
    pub type Fifoflf2 = crate::EnumBitfieldStruct<u8, Fifoflf2_SPEC>;
    impl Fifoflf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflf3_SPEC;
    pub type Fifoflf3 = crate::EnumBitfieldStruct<u8, Fifoflf3_SPEC>;
    impl Fifoflf3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflf4_SPEC;
    pub type Fifoflf4 = crate::EnumBitfieldStruct<u8, Fifoflf4_SPEC>;
    impl Fifoflf4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflf5_SPEC;
    pub type Fifoflf5 = crate::EnumBitfieldStruct<u8, Fifoflf5_SPEC>;
    impl Fifoflf5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflf6_SPEC;
    pub type Fifoflf6 = crate::EnumBitfieldStruct<u8, Fifoflf6_SPEC>;
    impl Fifoflf6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflf7_SPEC;
    pub type Fifoflf7 = crate::EnumBitfieldStruct<u8, Fifoflf7_SPEC>;
    impl Fifoflf7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflf8_SPEC;
    pub type Fifoflf8 = crate::EnumBitfieldStruct<u8, Fifoflf8_SPEC>;
    impl Fifoflf8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifoerscr_SPEC;
impl crate::sealed::RegSpec for Adfifoerscr_SPEC {
    type DataType = u32;
}

pub type Adfifoerscr = crate::RegValueT<Adfifoerscr_SPEC>;

impl Adfifoerscr {
    #[inline(always)]
    pub fn fifoovfc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adfifoerscr::Fifoovfc0,
        adfifoerscr::Fifoovfc0,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adfifoerscr::Fifoovfc0,
            adfifoerscr::Fifoovfc0,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovfc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adfifoerscr::Fifoovfc1,
        adfifoerscr::Fifoovfc1,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adfifoerscr::Fifoovfc1,
            adfifoerscr::Fifoovfc1,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovfc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adfifoerscr::Fifoovfc2,
        adfifoerscr::Fifoovfc2,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adfifoerscr::Fifoovfc2,
            adfifoerscr::Fifoovfc2,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovfc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adfifoerscr::Fifoovfc3,
        adfifoerscr::Fifoovfc3,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adfifoerscr::Fifoovfc3,
            adfifoerscr::Fifoovfc3,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovfc4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adfifoerscr::Fifoovfc4,
        adfifoerscr::Fifoovfc4,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adfifoerscr::Fifoovfc4,
            adfifoerscr::Fifoovfc4,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovfc5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adfifoerscr::Fifoovfc5,
        adfifoerscr::Fifoovfc5,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adfifoerscr::Fifoovfc5,
            adfifoerscr::Fifoovfc5,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovfc6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adfifoerscr::Fifoovfc6,
        adfifoerscr::Fifoovfc6,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adfifoerscr::Fifoovfc6,
            adfifoerscr::Fifoovfc6,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovfc7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adfifoerscr::Fifoovfc7,
        adfifoerscr::Fifoovfc7,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adfifoerscr::Fifoovfc7,
            adfifoerscr::Fifoovfc7,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoovfc8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adfifoerscr::Fifoovfc8,
        adfifoerscr::Fifoovfc8,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adfifoerscr::Fifoovfc8,
            adfifoerscr::Fifoovfc8,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflc0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adfifoerscr::Fifoflc0,
        adfifoerscr::Fifoflc0,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adfifoerscr::Fifoflc0,
            adfifoerscr::Fifoflc0,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflc1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adfifoerscr::Fifoflc1,
        adfifoerscr::Fifoflc1,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adfifoerscr::Fifoflc1,
            adfifoerscr::Fifoflc1,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflc2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        adfifoerscr::Fifoflc2,
        adfifoerscr::Fifoflc2,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            adfifoerscr::Fifoflc2,
            adfifoerscr::Fifoflc2,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflc3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        adfifoerscr::Fifoflc3,
        adfifoerscr::Fifoflc3,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            adfifoerscr::Fifoflc3,
            adfifoerscr::Fifoflc3,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflc4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        adfifoerscr::Fifoflc4,
        adfifoerscr::Fifoflc4,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            adfifoerscr::Fifoflc4,
            adfifoerscr::Fifoflc4,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflc5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        adfifoerscr::Fifoflc5,
        adfifoerscr::Fifoflc5,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            adfifoerscr::Fifoflc5,
            adfifoerscr::Fifoflc5,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflc6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        adfifoerscr::Fifoflc6,
        adfifoerscr::Fifoflc6,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            adfifoerscr::Fifoflc6,
            adfifoerscr::Fifoflc6,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflc7(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        adfifoerscr::Fifoflc7,
        adfifoerscr::Fifoflc7,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            adfifoerscr::Fifoflc7,
            adfifoerscr::Fifoflc7,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifoflc8(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        adfifoerscr::Fifoflc8,
        adfifoerscr::Fifoflc8,
        Adfifoerscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            adfifoerscr::Fifoflc8,
            adfifoerscr::Fifoflc8,
            Adfifoerscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adfifoerscr {
    #[inline(always)]
    fn default() -> Adfifoerscr {
        <crate::RegValueT<Adfifoerscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adfifoerscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovfc0_SPEC;
    pub type Fifoovfc0 = crate::EnumBitfieldStruct<u8, Fifoovfc0_SPEC>;
    impl Fifoovfc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovfc1_SPEC;
    pub type Fifoovfc1 = crate::EnumBitfieldStruct<u8, Fifoovfc1_SPEC>;
    impl Fifoovfc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovfc2_SPEC;
    pub type Fifoovfc2 = crate::EnumBitfieldStruct<u8, Fifoovfc2_SPEC>;
    impl Fifoovfc2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovfc3_SPEC;
    pub type Fifoovfc3 = crate::EnumBitfieldStruct<u8, Fifoovfc3_SPEC>;
    impl Fifoovfc3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovfc4_SPEC;
    pub type Fifoovfc4 = crate::EnumBitfieldStruct<u8, Fifoovfc4_SPEC>;
    impl Fifoovfc4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovfc5_SPEC;
    pub type Fifoovfc5 = crate::EnumBitfieldStruct<u8, Fifoovfc5_SPEC>;
    impl Fifoovfc5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovfc6_SPEC;
    pub type Fifoovfc6 = crate::EnumBitfieldStruct<u8, Fifoovfc6_SPEC>;
    impl Fifoovfc6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovfc7_SPEC;
    pub type Fifoovfc7 = crate::EnumBitfieldStruct<u8, Fifoovfc7_SPEC>;
    impl Fifoovfc7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoovfc8_SPEC;
    pub type Fifoovfc8 = crate::EnumBitfieldStruct<u8, Fifoovfc8_SPEC>;
    impl Fifoovfc8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflc0_SPEC;
    pub type Fifoflc0 = crate::EnumBitfieldStruct<u8, Fifoflc0_SPEC>;
    impl Fifoflc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflc1_SPEC;
    pub type Fifoflc1 = crate::EnumBitfieldStruct<u8, Fifoflc1_SPEC>;
    impl Fifoflc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflc2_SPEC;
    pub type Fifoflc2 = crate::EnumBitfieldStruct<u8, Fifoflc2_SPEC>;
    impl Fifoflc2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflc3_SPEC;
    pub type Fifoflc3 = crate::EnumBitfieldStruct<u8, Fifoflc3_SPEC>;
    impl Fifoflc3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflc4_SPEC;
    pub type Fifoflc4 = crate::EnumBitfieldStruct<u8, Fifoflc4_SPEC>;
    impl Fifoflc4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflc5_SPEC;
    pub type Fifoflc5 = crate::EnumBitfieldStruct<u8, Fifoflc5_SPEC>;
    impl Fifoflc5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflc6_SPEC;
    pub type Fifoflc6 = crate::EnumBitfieldStruct<u8, Fifoflc6_SPEC>;
    impl Fifoflc6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflc7_SPEC;
    pub type Fifoflc7 = crate::EnumBitfieldStruct<u8, Fifoflc7_SPEC>;
    impl Fifoflc7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fifoflc8_SPEC;
    pub type Fifoflc8 = crate::EnumBitfieldStruct<u8, Fifoflc8_SPEC>;
    impl Fifoflc8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmptbsr_SPEC;
impl crate::sealed::RegSpec for Adcmptbsr_SPEC {
    type DataType = u32;
}

pub type Adcmptbsr = crate::RegValueT<Adcmptbsr_SPEC>;

impl Adcmptbsr {
    #[inline(always)]
    pub fn cmptbf0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmptbsr::Cmptbf0,
        adcmptbsr::Cmptbf0,
        Adcmptbsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmptbsr::Cmptbf0,
            adcmptbsr::Cmptbf0,
            Adcmptbsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptbf1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmptbsr::Cmptbf1,
        adcmptbsr::Cmptbf1,
        Adcmptbsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmptbsr::Cmptbf1,
            adcmptbsr::Cmptbf1,
            Adcmptbsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptbf2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmptbsr::Cmptbf2,
        adcmptbsr::Cmptbf2,
        Adcmptbsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmptbsr::Cmptbf2,
            adcmptbsr::Cmptbf2,
            Adcmptbsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptbf3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmptbsr::Cmptbf3,
        adcmptbsr::Cmptbf3,
        Adcmptbsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmptbsr::Cmptbf3,
            adcmptbsr::Cmptbf3,
            Adcmptbsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptbf4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmptbsr::Cmptbf4,
        adcmptbsr::Cmptbf4,
        Adcmptbsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmptbsr::Cmptbf4,
            adcmptbsr::Cmptbf4,
            Adcmptbsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptbf5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmptbsr::Cmptbf5,
        adcmptbsr::Cmptbf5,
        Adcmptbsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmptbsr::Cmptbf5,
            adcmptbsr::Cmptbf5,
            Adcmptbsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptbf6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmptbsr::Cmptbf6,
        adcmptbsr::Cmptbf6,
        Adcmptbsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmptbsr::Cmptbf6,
            adcmptbsr::Cmptbf6,
            Adcmptbsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptbf7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmptbsr::Cmptbf7,
        adcmptbsr::Cmptbf7,
        Adcmptbsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmptbsr::Cmptbf7,
            adcmptbsr::Cmptbf7,
            Adcmptbsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmptbsr {
    #[inline(always)]
    fn default() -> Adcmptbsr {
        <crate::RegValueT<Adcmptbsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmptbsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbf0_SPEC;
    pub type Cmptbf0 = crate::EnumBitfieldStruct<u8, Cmptbf0_SPEC>;
    impl Cmptbf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbf1_SPEC;
    pub type Cmptbf1 = crate::EnumBitfieldStruct<u8, Cmptbf1_SPEC>;
    impl Cmptbf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbf2_SPEC;
    pub type Cmptbf2 = crate::EnumBitfieldStruct<u8, Cmptbf2_SPEC>;
    impl Cmptbf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbf3_SPEC;
    pub type Cmptbf3 = crate::EnumBitfieldStruct<u8, Cmptbf3_SPEC>;
    impl Cmptbf3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbf4_SPEC;
    pub type Cmptbf4 = crate::EnumBitfieldStruct<u8, Cmptbf4_SPEC>;
    impl Cmptbf4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbf5_SPEC;
    pub type Cmptbf5 = crate::EnumBitfieldStruct<u8, Cmptbf5_SPEC>;
    impl Cmptbf5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbf6_SPEC;
    pub type Cmptbf6 = crate::EnumBitfieldStruct<u8, Cmptbf6_SPEC>;
    impl Cmptbf6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbf7_SPEC;
    pub type Cmptbf7 = crate::EnumBitfieldStruct<u8, Cmptbf7_SPEC>;
    impl Cmptbf7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmptbscr_SPEC;
impl crate::sealed::RegSpec for Adcmptbscr_SPEC {
    type DataType = u32;
}

pub type Adcmptbscr = crate::RegValueT<Adcmptbscr_SPEC>;

impl Adcmptbscr {
    #[inline(always)]
    pub fn cmptbc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmptbscr::Cmptbc0,
        adcmptbscr::Cmptbc0,
        Adcmptbscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmptbscr::Cmptbc0,
            adcmptbscr::Cmptbc0,
            Adcmptbscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptbc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmptbscr::Cmptbc1,
        adcmptbscr::Cmptbc1,
        Adcmptbscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmptbscr::Cmptbc1,
            adcmptbscr::Cmptbc1,
            Adcmptbscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptbc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmptbscr::Cmptbc2,
        adcmptbscr::Cmptbc2,
        Adcmptbscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmptbscr::Cmptbc2,
            adcmptbscr::Cmptbc2,
            Adcmptbscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptbc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmptbscr::Cmptbc3,
        adcmptbscr::Cmptbc3,
        Adcmptbscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmptbscr::Cmptbc3,
            adcmptbscr::Cmptbc3,
            Adcmptbscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptbc4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmptbscr::Cmptbc4,
        adcmptbscr::Cmptbc4,
        Adcmptbscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmptbscr::Cmptbc4,
            adcmptbscr::Cmptbc4,
            Adcmptbscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptbc5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmptbscr::Cmptbc5,
        adcmptbscr::Cmptbc5,
        Adcmptbscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmptbscr::Cmptbc5,
            adcmptbscr::Cmptbc5,
            Adcmptbscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptbc6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmptbscr::Cmptbc6,
        adcmptbscr::Cmptbc6,
        Adcmptbscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmptbscr::Cmptbc6,
            adcmptbscr::Cmptbc6,
            Adcmptbscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmptbc7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmptbscr::Cmptbc7,
        adcmptbscr::Cmptbc7,
        Adcmptbscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmptbscr::Cmptbc7,
            adcmptbscr::Cmptbc7,
            Adcmptbscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmptbscr {
    #[inline(always)]
    fn default() -> Adcmptbscr {
        <crate::RegValueT<Adcmptbscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmptbscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbc0_SPEC;
    pub type Cmptbc0 = crate::EnumBitfieldStruct<u8, Cmptbc0_SPEC>;
    impl Cmptbc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbc1_SPEC;
    pub type Cmptbc1 = crate::EnumBitfieldStruct<u8, Cmptbc1_SPEC>;
    impl Cmptbc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbc2_SPEC;
    pub type Cmptbc2 = crate::EnumBitfieldStruct<u8, Cmptbc2_SPEC>;
    impl Cmptbc2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbc3_SPEC;
    pub type Cmptbc3 = crate::EnumBitfieldStruct<u8, Cmptbc3_SPEC>;
    impl Cmptbc3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbc4_SPEC;
    pub type Cmptbc4 = crate::EnumBitfieldStruct<u8, Cmptbc4_SPEC>;
    impl Cmptbc4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbc5_SPEC;
    pub type Cmptbc5 = crate::EnumBitfieldStruct<u8, Cmptbc5_SPEC>;
    impl Cmptbc5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbc6_SPEC;
    pub type Cmptbc6 = crate::EnumBitfieldStruct<u8, Cmptbc6_SPEC>;
    impl Cmptbc6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptbc7_SPEC;
    pub type Cmptbc7 = crate::EnumBitfieldStruct<u8, Cmptbc7_SPEC>;
    impl Cmptbc7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpchsr0_SPEC;
impl crate::sealed::RegSpec for Adcmpchsr0_SPEC {
    type DataType = u32;
}

pub type Adcmpchsr0 = crate::RegValueT<Adcmpchsr0_SPEC>;

impl Adcmpchsr0 {
    #[inline(always)]
    pub fn cmpchf0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf0,
        adcmpchsr0::Cmpchf0,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf0,
            adcmpchsr0::Cmpchf0,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf1,
        adcmpchsr0::Cmpchf1,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf1,
            adcmpchsr0::Cmpchf1,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf2,
        adcmpchsr0::Cmpchf2,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf2,
            adcmpchsr0::Cmpchf2,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf3,
        adcmpchsr0::Cmpchf3,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf3,
            adcmpchsr0::Cmpchf3,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf4,
        adcmpchsr0::Cmpchf4,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf4,
            adcmpchsr0::Cmpchf4,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf5,
        adcmpchsr0::Cmpchf5,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf5,
            adcmpchsr0::Cmpchf5,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf6,
        adcmpchsr0::Cmpchf6,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf6,
            adcmpchsr0::Cmpchf6,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf7,
        adcmpchsr0::Cmpchf7,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf7,
            adcmpchsr0::Cmpchf7,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf8,
        adcmpchsr0::Cmpchf8,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf8,
            adcmpchsr0::Cmpchf8,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf9,
        adcmpchsr0::Cmpchf9,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf9,
            adcmpchsr0::Cmpchf9,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf10,
        adcmpchsr0::Cmpchf10,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf10,
            adcmpchsr0::Cmpchf10,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf11,
        adcmpchsr0::Cmpchf11,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf11,
            adcmpchsr0::Cmpchf11,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf12,
        adcmpchsr0::Cmpchf12,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf12,
            adcmpchsr0::Cmpchf12,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf13,
        adcmpchsr0::Cmpchf13,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf13,
            adcmpchsr0::Cmpchf13,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf14,
        adcmpchsr0::Cmpchf14,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf14,
            adcmpchsr0::Cmpchf14,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf15,
        adcmpchsr0::Cmpchf15,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf15,
            adcmpchsr0::Cmpchf15,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf16,
        adcmpchsr0::Cmpchf16,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf16,
            adcmpchsr0::Cmpchf16,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf17,
        adcmpchsr0::Cmpchf17,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf17,
            adcmpchsr0::Cmpchf17,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf18,
        adcmpchsr0::Cmpchf18,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf18,
            adcmpchsr0::Cmpchf18,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf19,
        adcmpchsr0::Cmpchf19,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf19,
            adcmpchsr0::Cmpchf19,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf20,
        adcmpchsr0::Cmpchf20,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf20,
            adcmpchsr0::Cmpchf20,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf21,
        adcmpchsr0::Cmpchf21,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf21,
            adcmpchsr0::Cmpchf21,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf22,
        adcmpchsr0::Cmpchf22,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf22,
            adcmpchsr0::Cmpchf22,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf23,
        adcmpchsr0::Cmpchf23,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf23,
            adcmpchsr0::Cmpchf23,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf24,
        adcmpchsr0::Cmpchf24,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf24,
            adcmpchsr0::Cmpchf24,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf25,
        adcmpchsr0::Cmpchf25,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf25,
            adcmpchsr0::Cmpchf25,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf26,
        adcmpchsr0::Cmpchf26,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf26,
            adcmpchsr0::Cmpchf26,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf27,
        adcmpchsr0::Cmpchf27,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf27,
            adcmpchsr0::Cmpchf27,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchf28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        adcmpchsr0::Cmpchf28,
        adcmpchsr0::Cmpchf28,
        Adcmpchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            adcmpchsr0::Cmpchf28,
            adcmpchsr0::Cmpchf28,
            Adcmpchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpchsr0 {
    #[inline(always)]
    fn default() -> Adcmpchsr0 {
        <crate::RegValueT<Adcmpchsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpchsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf0_SPEC;
    pub type Cmpchf0 = crate::EnumBitfieldStruct<u8, Cmpchf0_SPEC>;
    impl Cmpchf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf1_SPEC;
    pub type Cmpchf1 = crate::EnumBitfieldStruct<u8, Cmpchf1_SPEC>;
    impl Cmpchf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf2_SPEC;
    pub type Cmpchf2 = crate::EnumBitfieldStruct<u8, Cmpchf2_SPEC>;
    impl Cmpchf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf3_SPEC;
    pub type Cmpchf3 = crate::EnumBitfieldStruct<u8, Cmpchf3_SPEC>;
    impl Cmpchf3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf4_SPEC;
    pub type Cmpchf4 = crate::EnumBitfieldStruct<u8, Cmpchf4_SPEC>;
    impl Cmpchf4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf5_SPEC;
    pub type Cmpchf5 = crate::EnumBitfieldStruct<u8, Cmpchf5_SPEC>;
    impl Cmpchf5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf6_SPEC;
    pub type Cmpchf6 = crate::EnumBitfieldStruct<u8, Cmpchf6_SPEC>;
    impl Cmpchf6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf7_SPEC;
    pub type Cmpchf7 = crate::EnumBitfieldStruct<u8, Cmpchf7_SPEC>;
    impl Cmpchf7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf8_SPEC;
    pub type Cmpchf8 = crate::EnumBitfieldStruct<u8, Cmpchf8_SPEC>;
    impl Cmpchf8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf9_SPEC;
    pub type Cmpchf9 = crate::EnumBitfieldStruct<u8, Cmpchf9_SPEC>;
    impl Cmpchf9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf10_SPEC;
    pub type Cmpchf10 = crate::EnumBitfieldStruct<u8, Cmpchf10_SPEC>;
    impl Cmpchf10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf11_SPEC;
    pub type Cmpchf11 = crate::EnumBitfieldStruct<u8, Cmpchf11_SPEC>;
    impl Cmpchf11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf12_SPEC;
    pub type Cmpchf12 = crate::EnumBitfieldStruct<u8, Cmpchf12_SPEC>;
    impl Cmpchf12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf13_SPEC;
    pub type Cmpchf13 = crate::EnumBitfieldStruct<u8, Cmpchf13_SPEC>;
    impl Cmpchf13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf14_SPEC;
    pub type Cmpchf14 = crate::EnumBitfieldStruct<u8, Cmpchf14_SPEC>;
    impl Cmpchf14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf15_SPEC;
    pub type Cmpchf15 = crate::EnumBitfieldStruct<u8, Cmpchf15_SPEC>;
    impl Cmpchf15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf16_SPEC;
    pub type Cmpchf16 = crate::EnumBitfieldStruct<u8, Cmpchf16_SPEC>;
    impl Cmpchf16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf17_SPEC;
    pub type Cmpchf17 = crate::EnumBitfieldStruct<u8, Cmpchf17_SPEC>;
    impl Cmpchf17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf18_SPEC;
    pub type Cmpchf18 = crate::EnumBitfieldStruct<u8, Cmpchf18_SPEC>;
    impl Cmpchf18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf19_SPEC;
    pub type Cmpchf19 = crate::EnumBitfieldStruct<u8, Cmpchf19_SPEC>;
    impl Cmpchf19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf20_SPEC;
    pub type Cmpchf20 = crate::EnumBitfieldStruct<u8, Cmpchf20_SPEC>;
    impl Cmpchf20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf21_SPEC;
    pub type Cmpchf21 = crate::EnumBitfieldStruct<u8, Cmpchf21_SPEC>;
    impl Cmpchf21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf22_SPEC;
    pub type Cmpchf22 = crate::EnumBitfieldStruct<u8, Cmpchf22_SPEC>;
    impl Cmpchf22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf23_SPEC;
    pub type Cmpchf23 = crate::EnumBitfieldStruct<u8, Cmpchf23_SPEC>;
    impl Cmpchf23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf24_SPEC;
    pub type Cmpchf24 = crate::EnumBitfieldStruct<u8, Cmpchf24_SPEC>;
    impl Cmpchf24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf25_SPEC;
    pub type Cmpchf25 = crate::EnumBitfieldStruct<u8, Cmpchf25_SPEC>;
    impl Cmpchf25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf26_SPEC;
    pub type Cmpchf26 = crate::EnumBitfieldStruct<u8, Cmpchf26_SPEC>;
    impl Cmpchf26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf27_SPEC;
    pub type Cmpchf27 = crate::EnumBitfieldStruct<u8, Cmpchf27_SPEC>;
    impl Cmpchf27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchf28_SPEC;
    pub type Cmpchf28 = crate::EnumBitfieldStruct<u8, Cmpchf28_SPEC>;
    impl Cmpchf28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpexsr_SPEC;
impl crate::sealed::RegSpec for Adcmpexsr_SPEC {
    type DataType = u32;
}

pub type Adcmpexsr = crate::RegValueT<Adcmpexsr_SPEC>;

impl Adcmpexsr {
    #[inline(always)]
    pub fn cmpexf0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpexsr::Cmpexf0,
        adcmpexsr::Cmpexf0,
        Adcmpexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpexsr::Cmpexf0,
            adcmpexsr::Cmpexf0,
            Adcmpexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpexf1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpexsr::Cmpexf1,
        adcmpexsr::Cmpexf1,
        Adcmpexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpexsr::Cmpexf1,
            adcmpexsr::Cmpexf1,
            Adcmpexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpexf2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpexsr::Cmpexf2,
        adcmpexsr::Cmpexf2,
        Adcmpexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpexsr::Cmpexf2,
            adcmpexsr::Cmpexf2,
            Adcmpexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpexf5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpexsr::Cmpexf5,
        adcmpexsr::Cmpexf5,
        Adcmpexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpexsr::Cmpexf5,
            adcmpexsr::Cmpexf5,
            Adcmpexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpexf6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpexsr::Cmpexf6,
        adcmpexsr::Cmpexf6,
        Adcmpexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpexsr::Cmpexf6,
            adcmpexsr::Cmpexf6,
            Adcmpexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpexf7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpexsr::Cmpexf7,
        adcmpexsr::Cmpexf7,
        Adcmpexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpexsr::Cmpexf7,
            adcmpexsr::Cmpexf7,
            Adcmpexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpexf8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmpexsr::Cmpexf8,
        adcmpexsr::Cmpexf8,
        Adcmpexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmpexsr::Cmpexf8,
            adcmpexsr::Cmpexf8,
            Adcmpexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpexsr {
    #[inline(always)]
    fn default() -> Adcmpexsr {
        <crate::RegValueT<Adcmpexsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpexsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpexf0_SPEC;
    pub type Cmpexf0 = crate::EnumBitfieldStruct<u8, Cmpexf0_SPEC>;
    impl Cmpexf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpexf1_SPEC;
    pub type Cmpexf1 = crate::EnumBitfieldStruct<u8, Cmpexf1_SPEC>;
    impl Cmpexf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpexf2_SPEC;
    pub type Cmpexf2 = crate::EnumBitfieldStruct<u8, Cmpexf2_SPEC>;
    impl Cmpexf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpexf5_SPEC;
    pub type Cmpexf5 = crate::EnumBitfieldStruct<u8, Cmpexf5_SPEC>;
    impl Cmpexf5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpexf6_SPEC;
    pub type Cmpexf6 = crate::EnumBitfieldStruct<u8, Cmpexf6_SPEC>;
    impl Cmpexf6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpexf7_SPEC;
    pub type Cmpexf7 = crate::EnumBitfieldStruct<u8, Cmpexf7_SPEC>;
    impl Cmpexf7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpexf8_SPEC;
    pub type Cmpexf8 = crate::EnumBitfieldStruct<u8, Cmpexf8_SPEC>;
    impl Cmpexf8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpchscr0_SPEC;
impl crate::sealed::RegSpec for Adcmpchscr0_SPEC {
    type DataType = u32;
}

pub type Adcmpchscr0 = crate::RegValueT<Adcmpchscr0_SPEC>;

impl Adcmpchscr0 {
    #[inline(always)]
    pub fn cmpchc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc0,
        adcmpchscr0::Cmpchc0,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc0,
            adcmpchscr0::Cmpchc0,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc1,
        adcmpchscr0::Cmpchc1,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc1,
            adcmpchscr0::Cmpchc1,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc2,
        adcmpchscr0::Cmpchc2,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc2,
            adcmpchscr0::Cmpchc2,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc3,
        adcmpchscr0::Cmpchc3,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc3,
            adcmpchscr0::Cmpchc3,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc4,
        adcmpchscr0::Cmpchc4,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc4,
            adcmpchscr0::Cmpchc4,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc5,
        adcmpchscr0::Cmpchc5,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc5,
            adcmpchscr0::Cmpchc5,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc6,
        adcmpchscr0::Cmpchc6,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc6,
            adcmpchscr0::Cmpchc6,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc7,
        adcmpchscr0::Cmpchc7,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc7,
            adcmpchscr0::Cmpchc7,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc8,
        adcmpchscr0::Cmpchc8,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc8,
            adcmpchscr0::Cmpchc8,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc9,
        adcmpchscr0::Cmpchc9,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc9,
            adcmpchscr0::Cmpchc9,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc10,
        adcmpchscr0::Cmpchc10,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc10,
            adcmpchscr0::Cmpchc10,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc11,
        adcmpchscr0::Cmpchc11,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc11,
            adcmpchscr0::Cmpchc11,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc12,
        adcmpchscr0::Cmpchc12,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc12,
            adcmpchscr0::Cmpchc12,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc13,
        adcmpchscr0::Cmpchc13,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc13,
            adcmpchscr0::Cmpchc13,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc14,
        adcmpchscr0::Cmpchc14,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc14,
            adcmpchscr0::Cmpchc14,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc15,
        adcmpchscr0::Cmpchc15,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc15,
            adcmpchscr0::Cmpchc15,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc16,
        adcmpchscr0::Cmpchc16,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc16,
            adcmpchscr0::Cmpchc16,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc17,
        adcmpchscr0::Cmpchc17,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc17,
            adcmpchscr0::Cmpchc17,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc18,
        adcmpchscr0::Cmpchc18,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc18,
            adcmpchscr0::Cmpchc18,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc19,
        adcmpchscr0::Cmpchc19,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc19,
            adcmpchscr0::Cmpchc19,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc20,
        adcmpchscr0::Cmpchc20,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc20,
            adcmpchscr0::Cmpchc20,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc21,
        adcmpchscr0::Cmpchc21,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc21,
            adcmpchscr0::Cmpchc21,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc22,
        adcmpchscr0::Cmpchc22,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc22,
            adcmpchscr0::Cmpchc22,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc23,
        adcmpchscr0::Cmpchc23,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc23,
            adcmpchscr0::Cmpchc23,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc24,
        adcmpchscr0::Cmpchc24,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc24,
            adcmpchscr0::Cmpchc24,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc25,
        adcmpchscr0::Cmpchc25,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc25,
            adcmpchscr0::Cmpchc25,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc26,
        adcmpchscr0::Cmpchc26,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc26,
            adcmpchscr0::Cmpchc26,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc27,
        adcmpchscr0::Cmpchc27,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc27,
            adcmpchscr0::Cmpchc27,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpchc28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        adcmpchscr0::Cmpchc28,
        adcmpchscr0::Cmpchc28,
        Adcmpchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            adcmpchscr0::Cmpchc28,
            adcmpchscr0::Cmpchc28,
            Adcmpchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpchscr0 {
    #[inline(always)]
    fn default() -> Adcmpchscr0 {
        <crate::RegValueT<Adcmpchscr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpchscr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc0_SPEC;
    pub type Cmpchc0 = crate::EnumBitfieldStruct<u8, Cmpchc0_SPEC>;
    impl Cmpchc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc1_SPEC;
    pub type Cmpchc1 = crate::EnumBitfieldStruct<u8, Cmpchc1_SPEC>;
    impl Cmpchc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc2_SPEC;
    pub type Cmpchc2 = crate::EnumBitfieldStruct<u8, Cmpchc2_SPEC>;
    impl Cmpchc2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc3_SPEC;
    pub type Cmpchc3 = crate::EnumBitfieldStruct<u8, Cmpchc3_SPEC>;
    impl Cmpchc3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc4_SPEC;
    pub type Cmpchc4 = crate::EnumBitfieldStruct<u8, Cmpchc4_SPEC>;
    impl Cmpchc4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc5_SPEC;
    pub type Cmpchc5 = crate::EnumBitfieldStruct<u8, Cmpchc5_SPEC>;
    impl Cmpchc5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc6_SPEC;
    pub type Cmpchc6 = crate::EnumBitfieldStruct<u8, Cmpchc6_SPEC>;
    impl Cmpchc6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc7_SPEC;
    pub type Cmpchc7 = crate::EnumBitfieldStruct<u8, Cmpchc7_SPEC>;
    impl Cmpchc7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc8_SPEC;
    pub type Cmpchc8 = crate::EnumBitfieldStruct<u8, Cmpchc8_SPEC>;
    impl Cmpchc8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc9_SPEC;
    pub type Cmpchc9 = crate::EnumBitfieldStruct<u8, Cmpchc9_SPEC>;
    impl Cmpchc9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc10_SPEC;
    pub type Cmpchc10 = crate::EnumBitfieldStruct<u8, Cmpchc10_SPEC>;
    impl Cmpchc10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc11_SPEC;
    pub type Cmpchc11 = crate::EnumBitfieldStruct<u8, Cmpchc11_SPEC>;
    impl Cmpchc11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc12_SPEC;
    pub type Cmpchc12 = crate::EnumBitfieldStruct<u8, Cmpchc12_SPEC>;
    impl Cmpchc12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc13_SPEC;
    pub type Cmpchc13 = crate::EnumBitfieldStruct<u8, Cmpchc13_SPEC>;
    impl Cmpchc13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc14_SPEC;
    pub type Cmpchc14 = crate::EnumBitfieldStruct<u8, Cmpchc14_SPEC>;
    impl Cmpchc14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc15_SPEC;
    pub type Cmpchc15 = crate::EnumBitfieldStruct<u8, Cmpchc15_SPEC>;
    impl Cmpchc15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc16_SPEC;
    pub type Cmpchc16 = crate::EnumBitfieldStruct<u8, Cmpchc16_SPEC>;
    impl Cmpchc16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc17_SPEC;
    pub type Cmpchc17 = crate::EnumBitfieldStruct<u8, Cmpchc17_SPEC>;
    impl Cmpchc17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc18_SPEC;
    pub type Cmpchc18 = crate::EnumBitfieldStruct<u8, Cmpchc18_SPEC>;
    impl Cmpchc18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc19_SPEC;
    pub type Cmpchc19 = crate::EnumBitfieldStruct<u8, Cmpchc19_SPEC>;
    impl Cmpchc19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc20_SPEC;
    pub type Cmpchc20 = crate::EnumBitfieldStruct<u8, Cmpchc20_SPEC>;
    impl Cmpchc20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc21_SPEC;
    pub type Cmpchc21 = crate::EnumBitfieldStruct<u8, Cmpchc21_SPEC>;
    impl Cmpchc21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc22_SPEC;
    pub type Cmpchc22 = crate::EnumBitfieldStruct<u8, Cmpchc22_SPEC>;
    impl Cmpchc22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc23_SPEC;
    pub type Cmpchc23 = crate::EnumBitfieldStruct<u8, Cmpchc23_SPEC>;
    impl Cmpchc23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc24_SPEC;
    pub type Cmpchc24 = crate::EnumBitfieldStruct<u8, Cmpchc24_SPEC>;
    impl Cmpchc24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc25_SPEC;
    pub type Cmpchc25 = crate::EnumBitfieldStruct<u8, Cmpchc25_SPEC>;
    impl Cmpchc25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc26_SPEC;
    pub type Cmpchc26 = crate::EnumBitfieldStruct<u8, Cmpchc26_SPEC>;
    impl Cmpchc26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc27_SPEC;
    pub type Cmpchc27 = crate::EnumBitfieldStruct<u8, Cmpchc27_SPEC>;
    impl Cmpchc27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpchc28_SPEC;
    pub type Cmpchc28 = crate::EnumBitfieldStruct<u8, Cmpchc28_SPEC>;
    impl Cmpchc28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpexscr_SPEC;
impl crate::sealed::RegSpec for Adcmpexscr_SPEC {
    type DataType = u32;
}

pub type Adcmpexscr = crate::RegValueT<Adcmpexscr_SPEC>;

impl Adcmpexscr {
    #[inline(always)]
    pub fn cmpexc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpexscr::Cmpexc0,
        adcmpexscr::Cmpexc0,
        Adcmpexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpexscr::Cmpexc0,
            adcmpexscr::Cmpexc0,
            Adcmpexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpexc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpexscr::Cmpexc1,
        adcmpexscr::Cmpexc1,
        Adcmpexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpexscr::Cmpexc1,
            adcmpexscr::Cmpexc1,
            Adcmpexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpexc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpexscr::Cmpexc2,
        adcmpexscr::Cmpexc2,
        Adcmpexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpexscr::Cmpexc2,
            adcmpexscr::Cmpexc2,
            Adcmpexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpexc5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpexscr::Cmpexc5,
        adcmpexscr::Cmpexc5,
        Adcmpexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpexscr::Cmpexc5,
            adcmpexscr::Cmpexc5,
            Adcmpexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpexc6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpexscr::Cmpexc6,
        adcmpexscr::Cmpexc6,
        Adcmpexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpexscr::Cmpexc6,
            adcmpexscr::Cmpexc6,
            Adcmpexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpexc7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpexscr::Cmpexc7,
        adcmpexscr::Cmpexc7,
        Adcmpexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpexscr::Cmpexc7,
            adcmpexscr::Cmpexc7,
            Adcmpexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpexc8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmpexscr::Cmpexc8,
        adcmpexscr::Cmpexc8,
        Adcmpexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmpexscr::Cmpexc8,
            adcmpexscr::Cmpexc8,
            Adcmpexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpexscr {
    #[inline(always)]
    fn default() -> Adcmpexscr {
        <crate::RegValueT<Adcmpexscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpexscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpexc0_SPEC;
    pub type Cmpexc0 = crate::EnumBitfieldStruct<u8, Cmpexc0_SPEC>;
    impl Cmpexc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpexc1_SPEC;
    pub type Cmpexc1 = crate::EnumBitfieldStruct<u8, Cmpexc1_SPEC>;
    impl Cmpexc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpexc2_SPEC;
    pub type Cmpexc2 = crate::EnumBitfieldStruct<u8, Cmpexc2_SPEC>;
    impl Cmpexc2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpexc5_SPEC;
    pub type Cmpexc5 = crate::EnumBitfieldStruct<u8, Cmpexc5_SPEC>;
    impl Cmpexc5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpexc6_SPEC;
    pub type Cmpexc6 = crate::EnumBitfieldStruct<u8, Cmpexc6_SPEC>;
    impl Cmpexc6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpexc7_SPEC;
    pub type Cmpexc7 = crate::EnumBitfieldStruct<u8, Cmpexc7_SPEC>;
    impl Cmpexc7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpexc8_SPEC;
    pub type Cmpexc8 = crate::EnumBitfieldStruct<u8, Cmpexc8_SPEC>;
    impl Cmpexc8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adlimgrsr_SPEC;
impl crate::sealed::RegSpec for Adlimgrsr_SPEC {
    type DataType = u32;
}

pub type Adlimgrsr = crate::RegValueT<Adlimgrsr_SPEC>;

impl Adlimgrsr {
    #[inline(always)]
    pub fn limgrf0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adlimgrsr::Limgrf0,
        adlimgrsr::Limgrf0,
        Adlimgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adlimgrsr::Limgrf0,
            adlimgrsr::Limgrf0,
            Adlimgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrf1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adlimgrsr::Limgrf1,
        adlimgrsr::Limgrf1,
        Adlimgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adlimgrsr::Limgrf1,
            adlimgrsr::Limgrf1,
            Adlimgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrf2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adlimgrsr::Limgrf2,
        adlimgrsr::Limgrf2,
        Adlimgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adlimgrsr::Limgrf2,
            adlimgrsr::Limgrf2,
            Adlimgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrf3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adlimgrsr::Limgrf3,
        adlimgrsr::Limgrf3,
        Adlimgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adlimgrsr::Limgrf3,
            adlimgrsr::Limgrf3,
            Adlimgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrf4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adlimgrsr::Limgrf4,
        adlimgrsr::Limgrf4,
        Adlimgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adlimgrsr::Limgrf4,
            adlimgrsr::Limgrf4,
            Adlimgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrf5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adlimgrsr::Limgrf5,
        adlimgrsr::Limgrf5,
        Adlimgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adlimgrsr::Limgrf5,
            adlimgrsr::Limgrf5,
            Adlimgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrf6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adlimgrsr::Limgrf6,
        adlimgrsr::Limgrf6,
        Adlimgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adlimgrsr::Limgrf6,
            adlimgrsr::Limgrf6,
            Adlimgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrf7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adlimgrsr::Limgrf7,
        adlimgrsr::Limgrf7,
        Adlimgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adlimgrsr::Limgrf7,
            adlimgrsr::Limgrf7,
            Adlimgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrf8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adlimgrsr::Limgrf8,
        adlimgrsr::Limgrf8,
        Adlimgrsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adlimgrsr::Limgrf8,
            adlimgrsr::Limgrf8,
            Adlimgrsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adlimgrsr {
    #[inline(always)]
    fn default() -> Adlimgrsr {
        <crate::RegValueT<Adlimgrsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adlimgrsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrf0_SPEC;
    pub type Limgrf0 = crate::EnumBitfieldStruct<u8, Limgrf0_SPEC>;
    impl Limgrf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrf1_SPEC;
    pub type Limgrf1 = crate::EnumBitfieldStruct<u8, Limgrf1_SPEC>;
    impl Limgrf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrf2_SPEC;
    pub type Limgrf2 = crate::EnumBitfieldStruct<u8, Limgrf2_SPEC>;
    impl Limgrf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrf3_SPEC;
    pub type Limgrf3 = crate::EnumBitfieldStruct<u8, Limgrf3_SPEC>;
    impl Limgrf3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrf4_SPEC;
    pub type Limgrf4 = crate::EnumBitfieldStruct<u8, Limgrf4_SPEC>;
    impl Limgrf4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrf5_SPEC;
    pub type Limgrf5 = crate::EnumBitfieldStruct<u8, Limgrf5_SPEC>;
    impl Limgrf5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrf6_SPEC;
    pub type Limgrf6 = crate::EnumBitfieldStruct<u8, Limgrf6_SPEC>;
    impl Limgrf6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrf7_SPEC;
    pub type Limgrf7 = crate::EnumBitfieldStruct<u8, Limgrf7_SPEC>;
    impl Limgrf7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrf8_SPEC;
    pub type Limgrf8 = crate::EnumBitfieldStruct<u8, Limgrf8_SPEC>;
    impl Limgrf8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adlimchsr0_SPEC;
impl crate::sealed::RegSpec for Adlimchsr0_SPEC {
    type DataType = u32;
}

pub type Adlimchsr0 = crate::RegValueT<Adlimchsr0_SPEC>;

impl Adlimchsr0 {
    #[inline(always)]
    pub fn limchf0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adlimchsr0::Limchf0,
        adlimchsr0::Limchf0,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adlimchsr0::Limchf0,
            adlimchsr0::Limchf0,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adlimchsr0::Limchf1,
        adlimchsr0::Limchf1,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adlimchsr0::Limchf1,
            adlimchsr0::Limchf1,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adlimchsr0::Limchf2,
        adlimchsr0::Limchf2,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adlimchsr0::Limchf2,
            adlimchsr0::Limchf2,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adlimchsr0::Limchf3,
        adlimchsr0::Limchf3,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adlimchsr0::Limchf3,
            adlimchsr0::Limchf3,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adlimchsr0::Limchf4,
        adlimchsr0::Limchf4,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adlimchsr0::Limchf4,
            adlimchsr0::Limchf4,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adlimchsr0::Limchf5,
        adlimchsr0::Limchf5,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adlimchsr0::Limchf5,
            adlimchsr0::Limchf5,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adlimchsr0::Limchf6,
        adlimchsr0::Limchf6,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adlimchsr0::Limchf6,
            adlimchsr0::Limchf6,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adlimchsr0::Limchf7,
        adlimchsr0::Limchf7,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adlimchsr0::Limchf7,
            adlimchsr0::Limchf7,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adlimchsr0::Limchf8,
        adlimchsr0::Limchf8,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adlimchsr0::Limchf8,
            adlimchsr0::Limchf8,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adlimchsr0::Limchf9,
        adlimchsr0::Limchf9,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adlimchsr0::Limchf9,
            adlimchsr0::Limchf9,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adlimchsr0::Limchf10,
        adlimchsr0::Limchf10,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adlimchsr0::Limchf10,
            adlimchsr0::Limchf10,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adlimchsr0::Limchf11,
        adlimchsr0::Limchf11,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adlimchsr0::Limchf11,
            adlimchsr0::Limchf11,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adlimchsr0::Limchf12,
        adlimchsr0::Limchf12,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adlimchsr0::Limchf12,
            adlimchsr0::Limchf12,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adlimchsr0::Limchf13,
        adlimchsr0::Limchf13,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adlimchsr0::Limchf13,
            adlimchsr0::Limchf13,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adlimchsr0::Limchf14,
        adlimchsr0::Limchf14,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adlimchsr0::Limchf14,
            adlimchsr0::Limchf14,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adlimchsr0::Limchf15,
        adlimchsr0::Limchf15,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adlimchsr0::Limchf15,
            adlimchsr0::Limchf15,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adlimchsr0::Limchf16,
        adlimchsr0::Limchf16,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adlimchsr0::Limchf16,
            adlimchsr0::Limchf16,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adlimchsr0::Limchf17,
        adlimchsr0::Limchf17,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adlimchsr0::Limchf17,
            adlimchsr0::Limchf17,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        adlimchsr0::Limchf18,
        adlimchsr0::Limchf18,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            adlimchsr0::Limchf18,
            adlimchsr0::Limchf18,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        adlimchsr0::Limchf19,
        adlimchsr0::Limchf19,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            adlimchsr0::Limchf19,
            adlimchsr0::Limchf19,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        adlimchsr0::Limchf20,
        adlimchsr0::Limchf20,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            adlimchsr0::Limchf20,
            adlimchsr0::Limchf20,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        adlimchsr0::Limchf21,
        adlimchsr0::Limchf21,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            adlimchsr0::Limchf21,
            adlimchsr0::Limchf21,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        adlimchsr0::Limchf22,
        adlimchsr0::Limchf22,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            adlimchsr0::Limchf22,
            adlimchsr0::Limchf22,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        adlimchsr0::Limchf23,
        adlimchsr0::Limchf23,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            adlimchsr0::Limchf23,
            adlimchsr0::Limchf23,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        adlimchsr0::Limchf24,
        adlimchsr0::Limchf24,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            adlimchsr0::Limchf24,
            adlimchsr0::Limchf24,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        adlimchsr0::Limchf25,
        adlimchsr0::Limchf25,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            adlimchsr0::Limchf25,
            adlimchsr0::Limchf25,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        adlimchsr0::Limchf26,
        adlimchsr0::Limchf26,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            adlimchsr0::Limchf26,
            adlimchsr0::Limchf26,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        adlimchsr0::Limchf27,
        adlimchsr0::Limchf27,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            adlimchsr0::Limchf27,
            adlimchsr0::Limchf27,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchf28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        adlimchsr0::Limchf28,
        adlimchsr0::Limchf28,
        Adlimchsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            adlimchsr0::Limchf28,
            adlimchsr0::Limchf28,
            Adlimchsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adlimchsr0 {
    #[inline(always)]
    fn default() -> Adlimchsr0 {
        <crate::RegValueT<Adlimchsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adlimchsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf0_SPEC;
    pub type Limchf0 = crate::EnumBitfieldStruct<u8, Limchf0_SPEC>;
    impl Limchf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf1_SPEC;
    pub type Limchf1 = crate::EnumBitfieldStruct<u8, Limchf1_SPEC>;
    impl Limchf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf2_SPEC;
    pub type Limchf2 = crate::EnumBitfieldStruct<u8, Limchf2_SPEC>;
    impl Limchf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf3_SPEC;
    pub type Limchf3 = crate::EnumBitfieldStruct<u8, Limchf3_SPEC>;
    impl Limchf3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf4_SPEC;
    pub type Limchf4 = crate::EnumBitfieldStruct<u8, Limchf4_SPEC>;
    impl Limchf4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf5_SPEC;
    pub type Limchf5 = crate::EnumBitfieldStruct<u8, Limchf5_SPEC>;
    impl Limchf5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf6_SPEC;
    pub type Limchf6 = crate::EnumBitfieldStruct<u8, Limchf6_SPEC>;
    impl Limchf6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf7_SPEC;
    pub type Limchf7 = crate::EnumBitfieldStruct<u8, Limchf7_SPEC>;
    impl Limchf7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf8_SPEC;
    pub type Limchf8 = crate::EnumBitfieldStruct<u8, Limchf8_SPEC>;
    impl Limchf8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf9_SPEC;
    pub type Limchf9 = crate::EnumBitfieldStruct<u8, Limchf9_SPEC>;
    impl Limchf9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf10_SPEC;
    pub type Limchf10 = crate::EnumBitfieldStruct<u8, Limchf10_SPEC>;
    impl Limchf10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf11_SPEC;
    pub type Limchf11 = crate::EnumBitfieldStruct<u8, Limchf11_SPEC>;
    impl Limchf11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf12_SPEC;
    pub type Limchf12 = crate::EnumBitfieldStruct<u8, Limchf12_SPEC>;
    impl Limchf12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf13_SPEC;
    pub type Limchf13 = crate::EnumBitfieldStruct<u8, Limchf13_SPEC>;
    impl Limchf13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf14_SPEC;
    pub type Limchf14 = crate::EnumBitfieldStruct<u8, Limchf14_SPEC>;
    impl Limchf14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf15_SPEC;
    pub type Limchf15 = crate::EnumBitfieldStruct<u8, Limchf15_SPEC>;
    impl Limchf15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf16_SPEC;
    pub type Limchf16 = crate::EnumBitfieldStruct<u8, Limchf16_SPEC>;
    impl Limchf16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf17_SPEC;
    pub type Limchf17 = crate::EnumBitfieldStruct<u8, Limchf17_SPEC>;
    impl Limchf17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf18_SPEC;
    pub type Limchf18 = crate::EnumBitfieldStruct<u8, Limchf18_SPEC>;
    impl Limchf18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf19_SPEC;
    pub type Limchf19 = crate::EnumBitfieldStruct<u8, Limchf19_SPEC>;
    impl Limchf19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf20_SPEC;
    pub type Limchf20 = crate::EnumBitfieldStruct<u8, Limchf20_SPEC>;
    impl Limchf20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf21_SPEC;
    pub type Limchf21 = crate::EnumBitfieldStruct<u8, Limchf21_SPEC>;
    impl Limchf21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf22_SPEC;
    pub type Limchf22 = crate::EnumBitfieldStruct<u8, Limchf22_SPEC>;
    impl Limchf22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf23_SPEC;
    pub type Limchf23 = crate::EnumBitfieldStruct<u8, Limchf23_SPEC>;
    impl Limchf23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf24_SPEC;
    pub type Limchf24 = crate::EnumBitfieldStruct<u8, Limchf24_SPEC>;
    impl Limchf24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf25_SPEC;
    pub type Limchf25 = crate::EnumBitfieldStruct<u8, Limchf25_SPEC>;
    impl Limchf25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf26_SPEC;
    pub type Limchf26 = crate::EnumBitfieldStruct<u8, Limchf26_SPEC>;
    impl Limchf26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf27_SPEC;
    pub type Limchf27 = crate::EnumBitfieldStruct<u8, Limchf27_SPEC>;
    impl Limchf27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchf28_SPEC;
    pub type Limchf28 = crate::EnumBitfieldStruct<u8, Limchf28_SPEC>;
    impl Limchf28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adlimexsr_SPEC;
impl crate::sealed::RegSpec for Adlimexsr_SPEC {
    type DataType = u32;
}

pub type Adlimexsr = crate::RegValueT<Adlimexsr_SPEC>;

impl Adlimexsr {
    #[inline(always)]
    pub fn limexf0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adlimexsr::Limexf0,
        adlimexsr::Limexf0,
        Adlimexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adlimexsr::Limexf0,
            adlimexsr::Limexf0,
            Adlimexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limexf1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adlimexsr::Limexf1,
        adlimexsr::Limexf1,
        Adlimexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adlimexsr::Limexf1,
            adlimexsr::Limexf1,
            Adlimexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limexf2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adlimexsr::Limexf2,
        adlimexsr::Limexf2,
        Adlimexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adlimexsr::Limexf2,
            adlimexsr::Limexf2,
            Adlimexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limexf5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adlimexsr::Limexf5,
        adlimexsr::Limexf5,
        Adlimexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adlimexsr::Limexf5,
            adlimexsr::Limexf5,
            Adlimexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limexf6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adlimexsr::Limexf6,
        adlimexsr::Limexf6,
        Adlimexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adlimexsr::Limexf6,
            adlimexsr::Limexf6,
            Adlimexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limexf7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adlimexsr::Limexf7,
        adlimexsr::Limexf7,
        Adlimexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adlimexsr::Limexf7,
            adlimexsr::Limexf7,
            Adlimexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limexf8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adlimexsr::Limexf8,
        adlimexsr::Limexf8,
        Adlimexsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adlimexsr::Limexf8,
            adlimexsr::Limexf8,
            Adlimexsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adlimexsr {
    #[inline(always)]
    fn default() -> Adlimexsr {
        <crate::RegValueT<Adlimexsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adlimexsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limexf0_SPEC;
    pub type Limexf0 = crate::EnumBitfieldStruct<u8, Limexf0_SPEC>;
    impl Limexf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limexf1_SPEC;
    pub type Limexf1 = crate::EnumBitfieldStruct<u8, Limexf1_SPEC>;
    impl Limexf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limexf2_SPEC;
    pub type Limexf2 = crate::EnumBitfieldStruct<u8, Limexf2_SPEC>;
    impl Limexf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limexf5_SPEC;
    pub type Limexf5 = crate::EnumBitfieldStruct<u8, Limexf5_SPEC>;
    impl Limexf5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limexf6_SPEC;
    pub type Limexf6 = crate::EnumBitfieldStruct<u8, Limexf6_SPEC>;
    impl Limexf6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limexf7_SPEC;
    pub type Limexf7 = crate::EnumBitfieldStruct<u8, Limexf7_SPEC>;
    impl Limexf7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limexf8_SPEC;
    pub type Limexf8 = crate::EnumBitfieldStruct<u8, Limexf8_SPEC>;
    impl Limexf8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adlimgrscr_SPEC;
impl crate::sealed::RegSpec for Adlimgrscr_SPEC {
    type DataType = u32;
}

pub type Adlimgrscr = crate::RegValueT<Adlimgrscr_SPEC>;

impl Adlimgrscr {
    #[inline(always)]
    pub fn limgrc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adlimgrscr::Limgrc0,
        adlimgrscr::Limgrc0,
        Adlimgrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adlimgrscr::Limgrc0,
            adlimgrscr::Limgrc0,
            Adlimgrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adlimgrscr::Limgrc1,
        adlimgrscr::Limgrc1,
        Adlimgrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adlimgrscr::Limgrc1,
            adlimgrscr::Limgrc1,
            Adlimgrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adlimgrscr::Limgrc2,
        adlimgrscr::Limgrc2,
        Adlimgrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adlimgrscr::Limgrc2,
            adlimgrscr::Limgrc2,
            Adlimgrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adlimgrscr::Limgrc3,
        adlimgrscr::Limgrc3,
        Adlimgrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adlimgrscr::Limgrc3,
            adlimgrscr::Limgrc3,
            Adlimgrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrc4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adlimgrscr::Limgrc4,
        adlimgrscr::Limgrc4,
        Adlimgrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adlimgrscr::Limgrc4,
            adlimgrscr::Limgrc4,
            Adlimgrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrc5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adlimgrscr::Limgrc5,
        adlimgrscr::Limgrc5,
        Adlimgrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adlimgrscr::Limgrc5,
            adlimgrscr::Limgrc5,
            Adlimgrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrc6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adlimgrscr::Limgrc6,
        adlimgrscr::Limgrc6,
        Adlimgrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adlimgrscr::Limgrc6,
            adlimgrscr::Limgrc6,
            Adlimgrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrc7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adlimgrscr::Limgrc7,
        adlimgrscr::Limgrc7,
        Adlimgrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adlimgrscr::Limgrc7,
            adlimgrscr::Limgrc7,
            Adlimgrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limgrc8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adlimgrscr::Limgrc8,
        adlimgrscr::Limgrc8,
        Adlimgrscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adlimgrscr::Limgrc8,
            adlimgrscr::Limgrc8,
            Adlimgrscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adlimgrscr {
    #[inline(always)]
    fn default() -> Adlimgrscr {
        <crate::RegValueT<Adlimgrscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adlimgrscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrc0_SPEC;
    pub type Limgrc0 = crate::EnumBitfieldStruct<u8, Limgrc0_SPEC>;
    impl Limgrc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrc1_SPEC;
    pub type Limgrc1 = crate::EnumBitfieldStruct<u8, Limgrc1_SPEC>;
    impl Limgrc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrc2_SPEC;
    pub type Limgrc2 = crate::EnumBitfieldStruct<u8, Limgrc2_SPEC>;
    impl Limgrc2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrc3_SPEC;
    pub type Limgrc3 = crate::EnumBitfieldStruct<u8, Limgrc3_SPEC>;
    impl Limgrc3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrc4_SPEC;
    pub type Limgrc4 = crate::EnumBitfieldStruct<u8, Limgrc4_SPEC>;
    impl Limgrc4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrc5_SPEC;
    pub type Limgrc5 = crate::EnumBitfieldStruct<u8, Limgrc5_SPEC>;
    impl Limgrc5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrc6_SPEC;
    pub type Limgrc6 = crate::EnumBitfieldStruct<u8, Limgrc6_SPEC>;
    impl Limgrc6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrc7_SPEC;
    pub type Limgrc7 = crate::EnumBitfieldStruct<u8, Limgrc7_SPEC>;
    impl Limgrc7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limgrc8_SPEC;
    pub type Limgrc8 = crate::EnumBitfieldStruct<u8, Limgrc8_SPEC>;
    impl Limgrc8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adlimchscr0_SPEC;
impl crate::sealed::RegSpec for Adlimchscr0_SPEC {
    type DataType = u32;
}

pub type Adlimchscr0 = crate::RegValueT<Adlimchscr0_SPEC>;

impl Adlimchscr0 {
    #[inline(always)]
    pub fn limchc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adlimchscr0::Limchc0,
        adlimchscr0::Limchc0,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adlimchscr0::Limchc0,
            adlimchscr0::Limchc0,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adlimchscr0::Limchc1,
        adlimchscr0::Limchc1,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adlimchscr0::Limchc1,
            adlimchscr0::Limchc1,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adlimchscr0::Limchc2,
        adlimchscr0::Limchc2,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adlimchscr0::Limchc2,
            adlimchscr0::Limchc2,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adlimchscr0::Limchc3,
        adlimchscr0::Limchc3,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adlimchscr0::Limchc3,
            adlimchscr0::Limchc3,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adlimchscr0::Limchc4,
        adlimchscr0::Limchc4,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adlimchscr0::Limchc4,
            adlimchscr0::Limchc4,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adlimchscr0::Limchc5,
        adlimchscr0::Limchc5,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adlimchscr0::Limchc5,
            adlimchscr0::Limchc5,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adlimchscr0::Limchc6,
        adlimchscr0::Limchc6,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adlimchscr0::Limchc6,
            adlimchscr0::Limchc6,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adlimchscr0::Limchc7,
        adlimchscr0::Limchc7,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adlimchscr0::Limchc7,
            adlimchscr0::Limchc7,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adlimchscr0::Limchc8,
        adlimchscr0::Limchc8,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adlimchscr0::Limchc8,
            adlimchscr0::Limchc8,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adlimchscr0::Limchc9,
        adlimchscr0::Limchc9,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adlimchscr0::Limchc9,
            adlimchscr0::Limchc9,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adlimchscr0::Limchc10,
        adlimchscr0::Limchc10,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adlimchscr0::Limchc10,
            adlimchscr0::Limchc10,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adlimchscr0::Limchc11,
        adlimchscr0::Limchc11,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adlimchscr0::Limchc11,
            adlimchscr0::Limchc11,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        adlimchscr0::Limchc12,
        adlimchscr0::Limchc12,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            adlimchscr0::Limchc12,
            adlimchscr0::Limchc12,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adlimchscr0::Limchc13,
        adlimchscr0::Limchc13,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adlimchscr0::Limchc13,
            adlimchscr0::Limchc13,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adlimchscr0::Limchc14,
        adlimchscr0::Limchc14,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adlimchscr0::Limchc14,
            adlimchscr0::Limchc14,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adlimchscr0::Limchc15,
        adlimchscr0::Limchc15,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adlimchscr0::Limchc15,
            adlimchscr0::Limchc15,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        adlimchscr0::Limchc16,
        adlimchscr0::Limchc16,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            adlimchscr0::Limchc16,
            adlimchscr0::Limchc16,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        adlimchscr0::Limchc17,
        adlimchscr0::Limchc17,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            adlimchscr0::Limchc17,
            adlimchscr0::Limchc17,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        adlimchscr0::Limchc18,
        adlimchscr0::Limchc18,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            adlimchscr0::Limchc18,
            adlimchscr0::Limchc18,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        adlimchscr0::Limchc19,
        adlimchscr0::Limchc19,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            adlimchscr0::Limchc19,
            adlimchscr0::Limchc19,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        adlimchscr0::Limchc20,
        adlimchscr0::Limchc20,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            adlimchscr0::Limchc20,
            adlimchscr0::Limchc20,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        adlimchscr0::Limchc21,
        adlimchscr0::Limchc21,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            adlimchscr0::Limchc21,
            adlimchscr0::Limchc21,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        adlimchscr0::Limchc22,
        adlimchscr0::Limchc22,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            adlimchscr0::Limchc22,
            adlimchscr0::Limchc22,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        adlimchscr0::Limchc23,
        adlimchscr0::Limchc23,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            adlimchscr0::Limchc23,
            adlimchscr0::Limchc23,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        adlimchscr0::Limchc24,
        adlimchscr0::Limchc24,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            adlimchscr0::Limchc24,
            adlimchscr0::Limchc24,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        adlimchscr0::Limchc25,
        adlimchscr0::Limchc25,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            adlimchscr0::Limchc25,
            adlimchscr0::Limchc25,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        adlimchscr0::Limchc26,
        adlimchscr0::Limchc26,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            adlimchscr0::Limchc26,
            adlimchscr0::Limchc26,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        adlimchscr0::Limchc27,
        adlimchscr0::Limchc27,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            adlimchscr0::Limchc27,
            adlimchscr0::Limchc27,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limchc28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        adlimchscr0::Limchc28,
        adlimchscr0::Limchc28,
        Adlimchscr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            adlimchscr0::Limchc28,
            adlimchscr0::Limchc28,
            Adlimchscr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adlimchscr0 {
    #[inline(always)]
    fn default() -> Adlimchscr0 {
        <crate::RegValueT<Adlimchscr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adlimchscr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc0_SPEC;
    pub type Limchc0 = crate::EnumBitfieldStruct<u8, Limchc0_SPEC>;
    impl Limchc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc1_SPEC;
    pub type Limchc1 = crate::EnumBitfieldStruct<u8, Limchc1_SPEC>;
    impl Limchc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc2_SPEC;
    pub type Limchc2 = crate::EnumBitfieldStruct<u8, Limchc2_SPEC>;
    impl Limchc2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc3_SPEC;
    pub type Limchc3 = crate::EnumBitfieldStruct<u8, Limchc3_SPEC>;
    impl Limchc3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc4_SPEC;
    pub type Limchc4 = crate::EnumBitfieldStruct<u8, Limchc4_SPEC>;
    impl Limchc4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc5_SPEC;
    pub type Limchc5 = crate::EnumBitfieldStruct<u8, Limchc5_SPEC>;
    impl Limchc5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc6_SPEC;
    pub type Limchc6 = crate::EnumBitfieldStruct<u8, Limchc6_SPEC>;
    impl Limchc6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc7_SPEC;
    pub type Limchc7 = crate::EnumBitfieldStruct<u8, Limchc7_SPEC>;
    impl Limchc7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc8_SPEC;
    pub type Limchc8 = crate::EnumBitfieldStruct<u8, Limchc8_SPEC>;
    impl Limchc8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc9_SPEC;
    pub type Limchc9 = crate::EnumBitfieldStruct<u8, Limchc9_SPEC>;
    impl Limchc9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc10_SPEC;
    pub type Limchc10 = crate::EnumBitfieldStruct<u8, Limchc10_SPEC>;
    impl Limchc10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc11_SPEC;
    pub type Limchc11 = crate::EnumBitfieldStruct<u8, Limchc11_SPEC>;
    impl Limchc11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc12_SPEC;
    pub type Limchc12 = crate::EnumBitfieldStruct<u8, Limchc12_SPEC>;
    impl Limchc12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc13_SPEC;
    pub type Limchc13 = crate::EnumBitfieldStruct<u8, Limchc13_SPEC>;
    impl Limchc13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc14_SPEC;
    pub type Limchc14 = crate::EnumBitfieldStruct<u8, Limchc14_SPEC>;
    impl Limchc14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc15_SPEC;
    pub type Limchc15 = crate::EnumBitfieldStruct<u8, Limchc15_SPEC>;
    impl Limchc15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc16_SPEC;
    pub type Limchc16 = crate::EnumBitfieldStruct<u8, Limchc16_SPEC>;
    impl Limchc16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc17_SPEC;
    pub type Limchc17 = crate::EnumBitfieldStruct<u8, Limchc17_SPEC>;
    impl Limchc17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc18_SPEC;
    pub type Limchc18 = crate::EnumBitfieldStruct<u8, Limchc18_SPEC>;
    impl Limchc18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc19_SPEC;
    pub type Limchc19 = crate::EnumBitfieldStruct<u8, Limchc19_SPEC>;
    impl Limchc19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc20_SPEC;
    pub type Limchc20 = crate::EnumBitfieldStruct<u8, Limchc20_SPEC>;
    impl Limchc20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc21_SPEC;
    pub type Limchc21 = crate::EnumBitfieldStruct<u8, Limchc21_SPEC>;
    impl Limchc21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc22_SPEC;
    pub type Limchc22 = crate::EnumBitfieldStruct<u8, Limchc22_SPEC>;
    impl Limchc22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc23_SPEC;
    pub type Limchc23 = crate::EnumBitfieldStruct<u8, Limchc23_SPEC>;
    impl Limchc23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc24_SPEC;
    pub type Limchc24 = crate::EnumBitfieldStruct<u8, Limchc24_SPEC>;
    impl Limchc24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc25_SPEC;
    pub type Limchc25 = crate::EnumBitfieldStruct<u8, Limchc25_SPEC>;
    impl Limchc25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc26_SPEC;
    pub type Limchc26 = crate::EnumBitfieldStruct<u8, Limchc26_SPEC>;
    impl Limchc26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc27_SPEC;
    pub type Limchc27 = crate::EnumBitfieldStruct<u8, Limchc27_SPEC>;
    impl Limchc27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limchc28_SPEC;
    pub type Limchc28 = crate::EnumBitfieldStruct<u8, Limchc28_SPEC>;
    impl Limchc28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adlimexscr_SPEC;
impl crate::sealed::RegSpec for Adlimexscr_SPEC {
    type DataType = u32;
}

pub type Adlimexscr = crate::RegValueT<Adlimexscr_SPEC>;

impl Adlimexscr {
    #[inline(always)]
    pub fn limexf0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adlimexscr::Limexf0,
        adlimexscr::Limexf0,
        Adlimexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adlimexscr::Limexf0,
            adlimexscr::Limexf0,
            Adlimexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limexf1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adlimexscr::Limexf1,
        adlimexscr::Limexf1,
        Adlimexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adlimexscr::Limexf1,
            adlimexscr::Limexf1,
            Adlimexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limexf2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adlimexscr::Limexf2,
        adlimexscr::Limexf2,
        Adlimexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adlimexscr::Limexf2,
            adlimexscr::Limexf2,
            Adlimexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limexf5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adlimexscr::Limexf5,
        adlimexscr::Limexf5,
        Adlimexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adlimexscr::Limexf5,
            adlimexscr::Limexf5,
            Adlimexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limexf6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adlimexscr::Limexf6,
        adlimexscr::Limexf6,
        Adlimexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adlimexscr::Limexf6,
            adlimexscr::Limexf6,
            Adlimexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limexf7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adlimexscr::Limexf7,
        adlimexscr::Limexf7,
        Adlimexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adlimexscr::Limexf7,
            adlimexscr::Limexf7,
            Adlimexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn limexf8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adlimexscr::Limexf8,
        adlimexscr::Limexf8,
        Adlimexscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adlimexscr::Limexf8,
            adlimexscr::Limexf8,
            Adlimexscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adlimexscr {
    #[inline(always)]
    fn default() -> Adlimexscr {
        <crate::RegValueT<Adlimexscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adlimexscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limexf0_SPEC;
    pub type Limexf0 = crate::EnumBitfieldStruct<u8, Limexf0_SPEC>;
    impl Limexf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limexf1_SPEC;
    pub type Limexf1 = crate::EnumBitfieldStruct<u8, Limexf1_SPEC>;
    impl Limexf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limexf2_SPEC;
    pub type Limexf2 = crate::EnumBitfieldStruct<u8, Limexf2_SPEC>;
    impl Limexf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limexf5_SPEC;
    pub type Limexf5 = crate::EnumBitfieldStruct<u8, Limexf5_SPEC>;
    impl Limexf5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limexf6_SPEC;
    pub type Limexf6 = crate::EnumBitfieldStruct<u8, Limexf6_SPEC>;
    impl Limexf6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limexf7_SPEC;
    pub type Limexf7 = crate::EnumBitfieldStruct<u8, Limexf7_SPEC>;
    impl Limexf7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Limexf8_SPEC;
    pub type Limexf8 = crate::EnumBitfieldStruct<u8, Limexf8_SPEC>;
    impl Limexf8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adscanendsr_SPEC;
impl crate::sealed::RegSpec for Adscanendsr_SPEC {
    type DataType = u32;
}

pub type Adscanendsr = crate::RegValueT<Adscanendsr_SPEC>;

impl Adscanendsr {
    #[inline(always)]
    pub fn scendf0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adscanendsr::Scendf0,
        adscanendsr::Scendf0,
        Adscanendsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adscanendsr::Scendf0,
            adscanendsr::Scendf0,
            Adscanendsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendf1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adscanendsr::Scendf1,
        adscanendsr::Scendf1,
        Adscanendsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adscanendsr::Scendf1,
            adscanendsr::Scendf1,
            Adscanendsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendf2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adscanendsr::Scendf2,
        adscanendsr::Scendf2,
        Adscanendsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adscanendsr::Scendf2,
            adscanendsr::Scendf2,
            Adscanendsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendf3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adscanendsr::Scendf3,
        adscanendsr::Scendf3,
        Adscanendsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adscanendsr::Scendf3,
            adscanendsr::Scendf3,
            Adscanendsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendf4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adscanendsr::Scendf4,
        adscanendsr::Scendf4,
        Adscanendsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adscanendsr::Scendf4,
            adscanendsr::Scendf4,
            Adscanendsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendf5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adscanendsr::Scendf5,
        adscanendsr::Scendf5,
        Adscanendsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adscanendsr::Scendf5,
            adscanendsr::Scendf5,
            Adscanendsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendf6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adscanendsr::Scendf6,
        adscanendsr::Scendf6,
        Adscanendsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adscanendsr::Scendf6,
            adscanendsr::Scendf6,
            Adscanendsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendf7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adscanendsr::Scendf7,
        adscanendsr::Scendf7,
        Adscanendsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adscanendsr::Scendf7,
            adscanendsr::Scendf7,
            Adscanendsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendf8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adscanendsr::Scendf8,
        adscanendsr::Scendf8,
        Adscanendsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adscanendsr::Scendf8,
            adscanendsr::Scendf8,
            Adscanendsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adscanendsr {
    #[inline(always)]
    fn default() -> Adscanendsr {
        <crate::RegValueT<Adscanendsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adscanendsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendf0_SPEC;
    pub type Scendf0 = crate::EnumBitfieldStruct<u8, Scendf0_SPEC>;
    impl Scendf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendf1_SPEC;
    pub type Scendf1 = crate::EnumBitfieldStruct<u8, Scendf1_SPEC>;
    impl Scendf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendf2_SPEC;
    pub type Scendf2 = crate::EnumBitfieldStruct<u8, Scendf2_SPEC>;
    impl Scendf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendf3_SPEC;
    pub type Scendf3 = crate::EnumBitfieldStruct<u8, Scendf3_SPEC>;
    impl Scendf3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendf4_SPEC;
    pub type Scendf4 = crate::EnumBitfieldStruct<u8, Scendf4_SPEC>;
    impl Scendf4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendf5_SPEC;
    pub type Scendf5 = crate::EnumBitfieldStruct<u8, Scendf5_SPEC>;
    impl Scendf5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendf6_SPEC;
    pub type Scendf6 = crate::EnumBitfieldStruct<u8, Scendf6_SPEC>;
    impl Scendf6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendf7_SPEC;
    pub type Scendf7 = crate::EnumBitfieldStruct<u8, Scendf7_SPEC>;
    impl Scendf7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendf8_SPEC;
    pub type Scendf8 = crate::EnumBitfieldStruct<u8, Scendf8_SPEC>;
    impl Scendf8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adscanendscr_SPEC;
impl crate::sealed::RegSpec for Adscanendscr_SPEC {
    type DataType = u32;
}

pub type Adscanendscr = crate::RegValueT<Adscanendscr_SPEC>;

impl Adscanendscr {
    #[inline(always)]
    pub fn scendc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adscanendscr::Scendc0,
        adscanendscr::Scendc0,
        Adscanendscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adscanendscr::Scendc0,
            adscanendscr::Scendc0,
            Adscanendscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adscanendscr::Scendc1,
        adscanendscr::Scendc1,
        Adscanendscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adscanendscr::Scendc1,
            adscanendscr::Scendc1,
            Adscanendscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adscanendscr::Scendc2,
        adscanendscr::Scendc2,
        Adscanendscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adscanendscr::Scendc2,
            adscanendscr::Scendc2,
            Adscanendscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adscanendscr::Scendc3,
        adscanendscr::Scendc3,
        Adscanendscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adscanendscr::Scendc3,
            adscanendscr::Scendc3,
            Adscanendscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendc4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adscanendscr::Scendc4,
        adscanendscr::Scendc4,
        Adscanendscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adscanendscr::Scendc4,
            adscanendscr::Scendc4,
            Adscanendscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendc5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adscanendscr::Scendc5,
        adscanendscr::Scendc5,
        Adscanendscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adscanendscr::Scendc5,
            adscanendscr::Scendc5,
            Adscanendscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendc6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adscanendscr::Scendc6,
        adscanendscr::Scendc6,
        Adscanendscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adscanendscr::Scendc6,
            adscanendscr::Scendc6,
            Adscanendscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendc7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adscanendscr::Scendc7,
        adscanendscr::Scendc7,
        Adscanendscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adscanendscr::Scendc7,
            adscanendscr::Scendc7,
            Adscanendscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scendc8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adscanendscr::Scendc8,
        adscanendscr::Scendc8,
        Adscanendscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adscanendscr::Scendc8,
            adscanendscr::Scendc8,
            Adscanendscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adscanendscr {
    #[inline(always)]
    fn default() -> Adscanendscr {
        <crate::RegValueT<Adscanendscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adscanendscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendc0_SPEC;
    pub type Scendc0 = crate::EnumBitfieldStruct<u8, Scendc0_SPEC>;
    impl Scendc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendc1_SPEC;
    pub type Scendc1 = crate::EnumBitfieldStruct<u8, Scendc1_SPEC>;
    impl Scendc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendc2_SPEC;
    pub type Scendc2 = crate::EnumBitfieldStruct<u8, Scendc2_SPEC>;
    impl Scendc2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendc3_SPEC;
    pub type Scendc3 = crate::EnumBitfieldStruct<u8, Scendc3_SPEC>;
    impl Scendc3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendc4_SPEC;
    pub type Scendc4 = crate::EnumBitfieldStruct<u8, Scendc4_SPEC>;
    impl Scendc4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendc5_SPEC;
    pub type Scendc5 = crate::EnumBitfieldStruct<u8, Scendc5_SPEC>;
    impl Scendc5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendc6_SPEC;
    pub type Scendc6 = crate::EnumBitfieldStruct<u8, Scendc6_SPEC>;
    impl Scendc6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendc7_SPEC;
    pub type Scendc7 = crate::EnumBitfieldStruct<u8, Scendc7_SPEC>;
    impl Scendc7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scendc8_SPEC;
    pub type Scendc8 = crate::EnumBitfieldStruct<u8, Scendc8_SPEC>;
    impl Scendc8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addr_SPEC;
impl crate::sealed::RegSpec for Addr_SPEC {
    type DataType = u32;
}

pub type Addr = crate::RegValueT<Addr_SPEC>;

impl Addr {
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Addr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Addr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn err(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        addr::Err,
        addr::Err,
        Addr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            addr::Err,
            addr::Err,
            Addr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Addr {
    #[inline(always)]
    fn default() -> Addr {
        <crate::RegValueT<Addr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod addr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err_SPEC;
    pub type Err = crate::EnumBitfieldStruct<u8, Err_SPEC>;
    impl Err {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adexdr_SPEC;
impl crate::sealed::RegSpec for Adexdr_SPEC {
    type DataType = u32;
}

pub type Adexdr = crate::RegValueT<Adexdr_SPEC>;

impl Adexdr {
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adexdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adexdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diagsr(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, u8, Adexdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x7,1,0,u8,u8,Adexdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn err(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        adexdr::Err,
        adexdr::Err,
        Adexdr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            adexdr::Err,
            adexdr::Err,
            Adexdr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adexdr {
    #[inline(always)]
    fn default() -> Adexdr {
        <crate::RegValueT<Adexdr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adexdr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err_SPEC;
    pub type Err = crate::EnumBitfieldStruct<u8, Err_SPEC>;
    impl Err {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adfifodr_SPEC;
impl crate::sealed::RegSpec for Adfifodr_SPEC {
    type DataType = u32;
}

pub type Adfifodr = crate::RegValueT<Adfifodr_SPEC>;

impl Adfifodr {
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adfifodr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adfifodr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ch(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, u8, Adfifodr_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x7f,1,0,u8,u8,Adfifodr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn err(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        adfifodr::Err,
        adfifodr::Err,
        Adfifodr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            adfifodr::Err,
            adfifodr::Err,
            Adfifodr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adfifodr {
    #[inline(always)]
    fn default() -> Adfifodr {
        <crate::RegValueT<Adfifodr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adfifodr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err_SPEC;
    pub type Err = crate::EnumBitfieldStruct<u8, Err_SPEC>;
    impl Err {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
