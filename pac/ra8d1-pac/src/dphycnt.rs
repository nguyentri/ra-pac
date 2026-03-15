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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:38:08 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"D-PHY Controller Top"]
unsafe impl ::core::marker::Send for super::Dphycnt {}
unsafe impl ::core::marker::Sync for super::Dphycnt {}
impl super::Dphycnt {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "D-PHY Reference Clock Setting Register"]
    #[inline(always)]
    pub const fn dphyrefcr(
        &self,
    ) -> &'static crate::common::Reg<self::Dphyrefcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphyrefcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "D-PHY Reference Clock Setting Register"]
    #[inline(always)]
    pub const fn dphyrefcr_l(
        &self,
    ) -> &'static crate::common::Reg<self::DphyrefcrL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyrefcrL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "D-PHY Reference Clock Setting Register"]
    #[inline(always)]
    pub const fn dphyrefcr_ll(
        &self,
    ) -> &'static crate::common::Reg<self::DphyrefcrLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyrefcrLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "D-PHY PLL Frequency Control Register"]
    #[inline(always)]
    pub const fn dphyplfcr(
        &self,
    ) -> &'static crate::common::Reg<self::Dphyplfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphyplfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "D-PHY PLL Frequency Control Register"]
    #[inline(always)]
    pub const fn dphyplfcr_l(
        &self,
    ) -> &'static crate::common::Reg<self::DphyplfcrL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyplfcrL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "D-PHY PLL Frequency Control Register"]
    #[inline(always)]
    pub const fn dphyplfcr_ll(
        &self,
    ) -> &'static crate::common::Reg<self::DphyplfcrLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyplfcrLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "D-PHY PLL Frequency Control Register"]
    #[inline(always)]
    pub const fn dphyplfcr_lh(
        &self,
    ) -> &'static crate::common::Reg<self::DphyplfcrLh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyplfcrLh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[doc = "D-PHY PLL Frequency Control Register"]
    #[inline(always)]
    pub const fn dphyplfcr_h(
        &self,
    ) -> &'static crate::common::Reg<self::DphyplfcrH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyplfcrH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "D-PHY PLL Frequency Control Register"]
    #[inline(always)]
    pub const fn dphyplfcr_hl(
        &self,
    ) -> &'static crate::common::Reg<self::DphyplfcrHl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyplfcrHl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "D-PHY PLL Operation Control Register"]
    #[inline(always)]
    pub const fn dphyplocr(
        &self,
    ) -> &'static crate::common::Reg<self::Dphyplocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphyplocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "D-PHY PLL Operation Control Register"]
    #[inline(always)]
    pub const fn dphyplocr_l(
        &self,
    ) -> &'static crate::common::Reg<self::DphyplocrL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyplocrL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "D-PHY PLL Operation Control Register"]
    #[inline(always)]
    pub const fn dphyplocr_ll(
        &self,
    ) -> &'static crate::common::Reg<self::DphyplocrLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyplocrLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "D-PHY Escape Mode Clock Control Register"]
    #[inline(always)]
    pub const fn dphyesccr(
        &self,
    ) -> &'static crate::common::Reg<self::Dphyesccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphyesccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "D-PHY Escape Mode Clock Control Register"]
    #[inline(always)]
    pub const fn dphyesccr_l(
        &self,
    ) -> &'static crate::common::Reg<self::DphyesccrL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyesccrL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "D-PHY Escape Mode Clock Control Register"]
    #[inline(always)]
    pub const fn dphyesccr_ll(
        &self,
    ) -> &'static crate::common::Reg<self::DphyesccrLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyesccrLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "D-PHY Power Supplying Control Register"]
    #[inline(always)]
    pub const fn dphypwrcr(
        &self,
    ) -> &'static crate::common::Reg<self::Dphypwrcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphypwrcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "D-PHY Power Supplying Control Register"]
    #[inline(always)]
    pub const fn dphypwrcr_l(
        &self,
    ) -> &'static crate::common::Reg<self::DphypwrcrL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphypwrcrL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "D-PHY Power Supplying Control Register"]
    #[inline(always)]
    pub const fn dphypwrcr_ll(
        &self,
    ) -> &'static crate::common::Reg<self::DphypwrcrLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphypwrcrLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "D-PHY Status Flag Register"]
    #[inline(always)]
    pub const fn dphysfr(
        &self,
    ) -> &'static crate::common::Reg<self::Dphysfr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dphysfr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "D-PHY Status Flag Register"]
    #[inline(always)]
    pub const fn dphysfr_l(
        &self,
    ) -> &'static crate::common::Reg<self::DphysfrL_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DphysfrL_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "D-PHY Status Flag Register"]
    #[inline(always)]
    pub const fn dphysfr_ll(
        &self,
    ) -> &'static crate::common::Reg<self::DphysfrLl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DphysfrLl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "D-PHY Status Flag Register"]
    #[inline(always)]
    pub const fn dphysfr_lh(
        &self,
    ) -> &'static crate::common::Reg<self::DphysfrLh_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DphysfrLh_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(29usize),
            )
        }
    }

    #[doc = "D-PHY Operation Control Register"]
    #[inline(always)]
    pub const fn dphyocr(
        &self,
    ) -> &'static crate::common::Reg<self::Dphyocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphyocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "D-PHY Operation Control Register"]
    #[inline(always)]
    pub const fn dphyocr_l(
        &self,
    ) -> &'static crate::common::Reg<self::DphyocrL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyocrL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "D-PHY Operation Control Register"]
    #[inline(always)]
    pub const fn dphyocr_ll(
        &self,
    ) -> &'static crate::common::Reg<self::DphyocrLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyocrLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 1"]
    #[inline(always)]
    pub const fn dphytim1(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 1"]
    #[inline(always)]
    pub const fn dphytim1_l(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim1L_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim1L_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 1"]
    #[inline(always)]
    pub const fn dphytim1_ll(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim1Ll_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim1Ll_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 1"]
    #[inline(always)]
    pub const fn dphytim1_lh(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim1Lh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim1Lh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(37usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 1"]
    #[inline(always)]
    pub const fn dphytim1_h(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim1H_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim1H_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 1"]
    #[inline(always)]
    pub const fn dphytim1_hl(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim1Hl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim1Hl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 2"]
    #[inline(always)]
    pub const fn dphytim2_ll(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim2Ll_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim2Ll_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 2"]
    #[inline(always)]
    pub const fn dphytim2_lh(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim2Lh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim2Lh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(41usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 2"]
    #[inline(always)]
    pub const fn dphytim2_hl(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim2Hl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim2Hl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 3"]
    #[inline(always)]
    pub const fn dphytim3_ll(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim3Ll_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim3Ll_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 3"]
    #[inline(always)]
    pub const fn dphytim3_lh(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim3Lh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim3Lh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(45usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 4"]
    #[inline(always)]
    pub const fn dphytim4(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 4"]
    #[inline(always)]
    pub const fn dphytim4_l(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim4L_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim4L_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 4"]
    #[inline(always)]
    pub const fn dphytim4_ll(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim4Ll_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim4Ll_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 4"]
    #[inline(always)]
    pub const fn dphytim4_lh(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim4Lh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim4Lh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(49usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 4"]
    #[inline(always)]
    pub const fn dphytim4_h(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim4H_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim4H_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 4"]
    #[inline(always)]
    pub const fn dphytim4_hl(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim4Hl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim4Hl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 4"]
    #[inline(always)]
    pub const fn dphytim4_hh(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim4Hh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim4Hh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(51usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 5"]
    #[inline(always)]
    pub const fn dphytim5(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 5"]
    #[inline(always)]
    pub const fn dphytim5_l(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim5L_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim5L_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 5"]
    #[inline(always)]
    pub const fn dphytim5_ll(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim5Ll_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim5Ll_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 5"]
    #[inline(always)]
    pub const fn dphytim5_lh(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim5Lh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim5Lh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(53usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 5"]
    #[inline(always)]
    pub const fn dphytim5_h(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim5H_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim5H_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 5"]
    #[inline(always)]
    pub const fn dphytim5_hl(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim5Hl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim5Hl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 6"]
    #[inline(always)]
    pub const fn dphytim6(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 6"]
    #[inline(always)]
    pub const fn dphytim6_l(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim6L_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim6L_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "D-PHY Timing Control Register 6"]
    #[inline(always)]
    pub const fn dphytim6_ll(
        &self,
    ) -> &'static crate::common::Reg<self::Dphytim6Ll_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dphytim6Ll_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphyrefcr_SPEC;
impl crate::sealed::RegSpec for Dphyrefcr_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Reference Clock Setting Register"]
pub type Dphyrefcr = crate::RegValueT<Dphyrefcr_SPEC>;

impl Dphyrefcr {
    #[doc = "Reference Clock Frequency Setting"]
    #[inline(always)]
    pub fn rfreq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        dphyrefcr::Rfreq,
        dphyrefcr::Rfreq,
        Dphyrefcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            dphyrefcr::Rfreq,
            dphyrefcr::Rfreq,
            Dphyrefcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000000000000. The write value should be 000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, u32, Dphyrefcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xffffff,1,0,u32,u32,Dphyrefcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphyrefcr {
    #[inline(always)]
    fn default() -> Dphyrefcr {
        <crate::RegValueT<Dphyrefcr_SPEC> as RegisterValue<_>>::new(47)
    }
}
pub mod dphyrefcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfreq_SPEC;
    pub type Rfreq = crate::EnumBitfieldStruct<u8, Rfreq_SPEC>;
    impl Rfreq {
        #[doc = "40MHz"]
        pub const _00100111: Self = Self::new(39);

        #[doc = "41MHz"]
        pub const _00101000: Self = Self::new(40);

        #[doc = "42MHz"]
        pub const _00101001: Self = Self::new(41);

        #[doc = "118MHz"]
        pub const _01110101: Self = Self::new(117);

        #[doc = "119MHz"]
        pub const _01110110: Self = Self::new(118);

        #[doc = "120MHz"]
        pub const _01110111: Self = Self::new(119);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyrefcrL_SPEC;
impl crate::sealed::RegSpec for DphyrefcrL_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY Reference Clock Setting Register"]
pub type DphyrefcrL = crate::RegValueT<DphyrefcrL_SPEC>;

impl DphyrefcrL {
    #[doc = "Reference Clock Frequency Setting"]
    #[inline(always)]
    pub fn rfreq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        dphyrefcr_l::Rfreq,
        dphyrefcr_l::Rfreq,
        DphyrefcrL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            dphyrefcr_l::Rfreq,
            dphyrefcr_l::Rfreq,
            DphyrefcrL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, DphyrefcrL_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,DphyrefcrL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyrefcrL {
    #[inline(always)]
    fn default() -> DphyrefcrL {
        <crate::RegValueT<DphyrefcrL_SPEC> as RegisterValue<_>>::new(47)
    }
}
pub mod dphyrefcr_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfreq_SPEC;
    pub type Rfreq = crate::EnumBitfieldStruct<u8, Rfreq_SPEC>;
    impl Rfreq {
        #[doc = "40MHz"]
        pub const _00100111: Self = Self::new(39);

        #[doc = "41MHz"]
        pub const _00101000: Self = Self::new(40);

        #[doc = "42MHz"]
        pub const _00101001: Self = Self::new(41);

        #[doc = "118MHz"]
        pub const _01110101: Self = Self::new(117);

        #[doc = "119MHz"]
        pub const _01110110: Self = Self::new(118);

        #[doc = "120MHz"]
        pub const _01110111: Self = Self::new(119);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyrefcrLl_SPEC;
impl crate::sealed::RegSpec for DphyrefcrLl_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Reference Clock Setting Register"]
pub type DphyrefcrLl = crate::RegValueT<DphyrefcrLl_SPEC>;

impl DphyrefcrLl {
    #[doc = "Reference Clock Frequency Setting"]
    #[inline(always)]
    pub fn rfreq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        dphyrefcr_ll::Rfreq,
        dphyrefcr_ll::Rfreq,
        DphyrefcrLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            dphyrefcr_ll::Rfreq,
            dphyrefcr_ll::Rfreq,
            DphyrefcrLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DphyrefcrLl {
    #[inline(always)]
    fn default() -> DphyrefcrLl {
        <crate::RegValueT<DphyrefcrLl_SPEC> as RegisterValue<_>>::new(47)
    }
}
pub mod dphyrefcr_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfreq_SPEC;
    pub type Rfreq = crate::EnumBitfieldStruct<u8, Rfreq_SPEC>;
    impl Rfreq {
        #[doc = "40MHz"]
        pub const _00100111: Self = Self::new(39);

        #[doc = "41MHz"]
        pub const _00101000: Self = Self::new(40);

        #[doc = "42MHz"]
        pub const _00101001: Self = Self::new(41);

        #[doc = "118MHz"]
        pub const _01110101: Self = Self::new(117);

        #[doc = "119MHz"]
        pub const _01110110: Self = Self::new(118);

        #[doc = "120MHz"]
        pub const _01110111: Self = Self::new(119);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphyplfcr_SPEC;
impl crate::sealed::RegSpec for Dphyplfcr_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY PLL Frequency Control Register"]
pub type Dphyplfcr = crate::RegValueT<Dphyplfcr_SPEC>;

impl Dphyplfcr {
    #[doc = "D-PHY PLL Input Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn idiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        dphyplfcr::Idiv,
        dphyplfcr::Idiv,
        Dphyplfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            dphyplfcr::Idiv,
            dphyplfcr::Idiv,
            Dphyplfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "D-PHY PLL Frequency Multiplication Factor Select (Fractional Part)"]
    #[inline(always)]
    pub fn nfmul(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        dphyplfcr::Nfmul,
        dphyplfcr::Nfmul,
        Dphyplfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            dphyplfcr::Nfmul,
            dphyplfcr::Nfmul,
            Dphyplfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "D-PHY PLL Frequency Multiplication Factor Select (Integer Part)"]
    #[inline(always)]
    pub fn nmul(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        dphyplfcr::Nmul,
        dphyplfcr::Nmul,
        Dphyplfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            dphyplfcr::Nmul,
            dphyplfcr::Nmul,
            Dphyplfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Dphyplfcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Dphyplfcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphyplfcr {
    #[inline(always)]
    fn default() -> Dphyplfcr {
        <crate::RegValueT<Dphyplfcr_SPEC> as RegisterValue<_>>::new(1245184)
    }
}
pub mod dphyplfcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idiv_SPEC;
    pub type Idiv = crate::EnumBitfieldStruct<u8, Idiv_SPEC>;
    impl Idiv {
        #[doc = "1"]
        pub const _00: Self = Self::new(0);

        #[doc = "1/2"]
        pub const _01: Self = Self::new(1);

        #[doc = "1/3"]
        pub const _10: Self = Self::new(2);

        #[doc = "1/4"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfmul_SPEC;
    pub type Nfmul = crate::EnumBitfieldStruct<u8, Nfmul_SPEC>;
    impl Nfmul {
        #[doc = "0.00"]
        pub const _00: Self = Self::new(0);

        #[doc = "0.33"]
        pub const _01: Self = Self::new(1);

        #[doc = "0.66"]
        pub const _10: Self = Self::new(2);

        #[doc = "0.50"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmul_SPEC;
    pub type Nmul = crate::EnumBitfieldStruct<u8, Nmul_SPEC>;
    impl Nmul {
        #[doc = "20"]
        pub const _00010011: Self = Self::new(19);

        #[doc = "21"]
        pub const _00010100: Self = Self::new(20);

        #[doc = "22"]
        pub const _00010101: Self = Self::new(21);

        #[doc = "99"]
        pub const _01100010: Self = Self::new(98);

        #[doc = "100"]
        pub const _01100011: Self = Self::new(99);

        #[doc = "101"]
        pub const _01100100: Self = Self::new(100);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyplfcrL_SPEC;
impl crate::sealed::RegSpec for DphyplfcrL_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY PLL Frequency Control Register"]
pub type DphyplfcrL = crate::RegValueT<DphyplfcrL_SPEC>;

impl DphyplfcrL {
    #[doc = "D-PHY PLL Input Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn idiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        dphyplfcr_l::Idiv,
        dphyplfcr_l::Idiv,
        DphyplfcrL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            dphyplfcr_l::Idiv,
            dphyplfcr_l::Idiv,
            DphyplfcrL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "D-PHY PLL Frequency Multiplication Factor Select (Fractional Part)"]
    #[inline(always)]
    pub fn nfmul(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        dphyplfcr_l::Nfmul,
        dphyplfcr_l::Nfmul,
        DphyplfcrL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            dphyplfcr_l::Nfmul,
            dphyplfcr_l::Nfmul,
            DphyplfcrL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, DphyplfcrL_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,DphyplfcrL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyplfcrL {
    #[inline(always)]
    fn default() -> DphyplfcrL {
        <crate::RegValueT<DphyplfcrL_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphyplfcr_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idiv_SPEC;
    pub type Idiv = crate::EnumBitfieldStruct<u8, Idiv_SPEC>;
    impl Idiv {
        #[doc = "1"]
        pub const _00: Self = Self::new(0);

        #[doc = "1/2"]
        pub const _01: Self = Self::new(1);

        #[doc = "1/3"]
        pub const _10: Self = Self::new(2);

        #[doc = "1/4"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfmul_SPEC;
    pub type Nfmul = crate::EnumBitfieldStruct<u8, Nfmul_SPEC>;
    impl Nfmul {
        #[doc = "0.00"]
        pub const _00: Self = Self::new(0);

        #[doc = "0.33"]
        pub const _01: Self = Self::new(1);

        #[doc = "0.66"]
        pub const _10: Self = Self::new(2);

        #[doc = "0.50"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyplfcrLl_SPEC;
impl crate::sealed::RegSpec for DphyplfcrLl_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY PLL Frequency Control Register"]
pub type DphyplfcrLl = crate::RegValueT<DphyplfcrLl_SPEC>;

impl DphyplfcrLl {
    #[doc = "D-PHY PLL Input Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn idiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        dphyplfcr_ll::Idiv,
        dphyplfcr_ll::Idiv,
        DphyplfcrLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            dphyplfcr_ll::Idiv,
            dphyplfcr_ll::Idiv,
            DphyplfcrLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, DphyplfcrLl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,DphyplfcrLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyplfcrLl {
    #[inline(always)]
    fn default() -> DphyplfcrLl {
        <crate::RegValueT<DphyplfcrLl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphyplfcr_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idiv_SPEC;
    pub type Idiv = crate::EnumBitfieldStruct<u8, Idiv_SPEC>;
    impl Idiv {
        #[doc = "1"]
        pub const _00: Self = Self::new(0);

        #[doc = "1/2"]
        pub const _01: Self = Self::new(1);

        #[doc = "1/3"]
        pub const _10: Self = Self::new(2);

        #[doc = "1/4"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyplfcrLh_SPEC;
impl crate::sealed::RegSpec for DphyplfcrLh_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY PLL Frequency Control Register"]
pub type DphyplfcrLh = crate::RegValueT<DphyplfcrLh_SPEC>;

impl DphyplfcrLh {
    #[doc = "D-PHY PLL Frequency Multiplication Factor Select (Fractional Part)"]
    #[inline(always)]
    pub fn nfmul(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        dphyplfcr_lh::Nfmul,
        dphyplfcr_lh::Nfmul,
        DphyplfcrLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            dphyplfcr_lh::Nfmul,
            dphyplfcr_lh::Nfmul,
            DphyplfcrLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, DphyplfcrLh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,DphyplfcrLh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyplfcrLh {
    #[inline(always)]
    fn default() -> DphyplfcrLh {
        <crate::RegValueT<DphyplfcrLh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphyplfcr_lh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfmul_SPEC;
    pub type Nfmul = crate::EnumBitfieldStruct<u8, Nfmul_SPEC>;
    impl Nfmul {
        #[doc = "0.00"]
        pub const _00: Self = Self::new(0);

        #[doc = "0.33"]
        pub const _01: Self = Self::new(1);

        #[doc = "0.66"]
        pub const _10: Self = Self::new(2);

        #[doc = "0.50"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyplfcrH_SPEC;
impl crate::sealed::RegSpec for DphyplfcrH_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY PLL Frequency Control Register"]
pub type DphyplfcrH = crate::RegValueT<DphyplfcrH_SPEC>;

impl DphyplfcrH {
    #[doc = "D-PHY PLL Frequency Multiplication Factor Select (Integer Part)"]
    #[inline(always)]
    pub fn nmul(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        dphyplfcr_h::Nmul,
        dphyplfcr_h::Nmul,
        DphyplfcrH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            dphyplfcr_h::Nmul,
            dphyplfcr_h::Nmul,
            DphyplfcrH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, DphyplfcrH_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,DphyplfcrH_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyplfcrH {
    #[inline(always)]
    fn default() -> DphyplfcrH {
        <crate::RegValueT<DphyplfcrH_SPEC> as RegisterValue<_>>::new(19)
    }
}
pub mod dphyplfcr_h {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmul_SPEC;
    pub type Nmul = crate::EnumBitfieldStruct<u8, Nmul_SPEC>;
    impl Nmul {
        #[doc = "20"]
        pub const _00010011: Self = Self::new(19);

        #[doc = "21"]
        pub const _00010100: Self = Self::new(20);

        #[doc = "22"]
        pub const _00010101: Self = Self::new(21);

        #[doc = "99"]
        pub const _01100010: Self = Self::new(98);

        #[doc = "100"]
        pub const _01100011: Self = Self::new(99);

        #[doc = "101"]
        pub const _01100100: Self = Self::new(100);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyplfcrHl_SPEC;
impl crate::sealed::RegSpec for DphyplfcrHl_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY PLL Frequency Control Register"]
pub type DphyplfcrHl = crate::RegValueT<DphyplfcrHl_SPEC>;

impl DphyplfcrHl {
    #[doc = "D-PHY PLL Frequency Multiplication Factor Select (Integer Part)"]
    #[inline(always)]
    pub fn nmul(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        dphyplfcr_hl::Nmul,
        dphyplfcr_hl::Nmul,
        DphyplfcrHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            dphyplfcr_hl::Nmul,
            dphyplfcr_hl::Nmul,
            DphyplfcrHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DphyplfcrHl {
    #[inline(always)]
    fn default() -> DphyplfcrHl {
        <crate::RegValueT<DphyplfcrHl_SPEC> as RegisterValue<_>>::new(19)
    }
}
pub mod dphyplfcr_hl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmul_SPEC;
    pub type Nmul = crate::EnumBitfieldStruct<u8, Nmul_SPEC>;
    impl Nmul {
        #[doc = "20"]
        pub const _00010011: Self = Self::new(19);

        #[doc = "21"]
        pub const _00010100: Self = Self::new(20);

        #[doc = "22"]
        pub const _00010101: Self = Self::new(21);

        #[doc = "99"]
        pub const _01100010: Self = Self::new(98);

        #[doc = "100"]
        pub const _01100011: Self = Self::new(99);

        #[doc = "101"]
        pub const _01100100: Self = Self::new(100);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphyplocr_SPEC;
impl crate::sealed::RegSpec for Dphyplocr_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY PLL Operation Control Register"]
pub type Dphyplocr = crate::RegValueT<Dphyplocr_SPEC>;

impl Dphyplocr {
    #[doc = "D-PHY PLL Operation Control"]
    #[inline(always)]
    pub fn pllstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphyplocr::Pllstp,
        dphyplocr::Pllstp,
        Dphyplocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphyplocr::Pllstp,
            dphyplocr::Pllstp,
            Dphyplocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7fffffff,
        1,
        0,
        u32,
        u32,
        Dphyplocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7fffffff,
            1,
            0,
            u32,
            u32,
            Dphyplocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dphyplocr {
    #[inline(always)]
    fn default() -> Dphyplocr {
        <crate::RegValueT<Dphyplocr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod dphyplocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllstp_SPEC;
    pub type Pllstp = crate::EnumBitfieldStruct<u8, Pllstp_SPEC>;
    impl Pllstp {
        #[doc = "Operate the PLL"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the PLL"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyplocrL_SPEC;
impl crate::sealed::RegSpec for DphyplocrL_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY PLL Operation Control Register"]
pub type DphyplocrL = crate::RegValueT<DphyplocrL_SPEC>;

impl DphyplocrL {
    #[doc = "D-PHY PLL Operation Control"]
    #[inline(always)]
    pub fn pllstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphyplocr_l::Pllstp,
        dphyplocr_l::Pllstp,
        DphyplocrL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphyplocr_l::Pllstp,
            dphyplocr_l::Pllstp,
            DphyplocrL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fff, 1, 0, u16, u16, DphyplocrL_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fff,1,0,u16,u16,DphyplocrL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyplocrL {
    #[inline(always)]
    fn default() -> DphyplocrL {
        <crate::RegValueT<DphyplocrL_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod dphyplocr_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllstp_SPEC;
    pub type Pllstp = crate::EnumBitfieldStruct<u8, Pllstp_SPEC>;
    impl Pllstp {
        #[doc = "Operate the PLL"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the PLL"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyplocrLl_SPEC;
impl crate::sealed::RegSpec for DphyplocrLl_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY PLL Operation Control Register"]
pub type DphyplocrLl = crate::RegValueT<DphyplocrLl_SPEC>;

impl DphyplocrLl {
    #[doc = "D-PHY PLL Operation Control"]
    #[inline(always)]
    pub fn pllstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphyplocr_ll::Pllstp,
        dphyplocr_ll::Pllstp,
        DphyplocrLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphyplocr_ll::Pllstp,
            dphyplocr_ll::Pllstp,
            DphyplocrLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, DphyplocrLl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,DphyplocrLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyplocrLl {
    #[inline(always)]
    fn default() -> DphyplocrLl {
        <crate::RegValueT<DphyplocrLl_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod dphyplocr_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllstp_SPEC;
    pub type Pllstp = crate::EnumBitfieldStruct<u8, Pllstp_SPEC>;
    impl Pllstp {
        #[doc = "Operate the PLL"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the PLL"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphyesccr_SPEC;
impl crate::sealed::RegSpec for Dphyesccr_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Escape Mode Clock Control Register"]
pub type Dphyesccr = crate::RegValueT<Dphyesccr_SPEC>;

impl Dphyesccr {
    #[doc = "Escape Mode Transfer Clock Division Ratio"]
    #[inline(always)]
    pub fn escdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        dphyesccr::Escdiv,
        dphyesccr::Escdiv,
        Dphyesccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            dphyesccr::Escdiv,
            dphyesccr::Escdiv,
            Dphyesccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000000000000000. The write value should be 000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, u32, Dphyesccr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            u32,
            Dphyesccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dphyesccr {
    #[inline(always)]
    fn default() -> Dphyesccr {
        <crate::RegValueT<Dphyesccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphyesccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Escdiv_SPEC;
    pub type Escdiv = crate::EnumBitfieldStruct<u8, Escdiv_SPEC>;
    impl Escdiv {
        #[doc = "x1/1"]
        pub const _00000: Self = Self::new(0);

        #[doc = "x1/2"]
        pub const _00001: Self = Self::new(1);

        #[doc = "x1/3"]
        pub const _00010: Self = Self::new(2);

        #[doc = "x1/4"]
        pub const _00011: Self = Self::new(3);

        #[doc = "x1/30"]
        pub const _11101: Self = Self::new(29);

        #[doc = "x1/31"]
        pub const _11110: Self = Self::new(30);

        #[doc = "x1/32"]
        pub const _11111: Self = Self::new(31);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyesccrL_SPEC;
impl crate::sealed::RegSpec for DphyesccrL_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY Escape Mode Clock Control Register"]
pub type DphyesccrL = crate::RegValueT<DphyesccrL_SPEC>;

impl DphyesccrL {
    #[doc = "Escape Mode Transfer Clock Division Ratio"]
    #[inline(always)]
    pub fn escdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        dphyesccr_l::Escdiv,
        dphyesccr_l::Escdiv,
        DphyesccrL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            dphyesccr_l::Escdiv,
            dphyesccr_l::Escdiv,
            DphyesccrL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000000. The write value should be 00000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7ff, 1, 0, u16, u16, DphyesccrL_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7ff,1,0,u16,u16,DphyesccrL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyesccrL {
    #[inline(always)]
    fn default() -> DphyesccrL {
        <crate::RegValueT<DphyesccrL_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphyesccr_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Escdiv_SPEC;
    pub type Escdiv = crate::EnumBitfieldStruct<u8, Escdiv_SPEC>;
    impl Escdiv {
        #[doc = "x1/1"]
        pub const _00000: Self = Self::new(0);

        #[doc = "x1/2"]
        pub const _00001: Self = Self::new(1);

        #[doc = "x1/3"]
        pub const _00010: Self = Self::new(2);

        #[doc = "x1/4"]
        pub const _00011: Self = Self::new(3);

        #[doc = "x1/30"]
        pub const _11101: Self = Self::new(29);

        #[doc = "x1/31"]
        pub const _11110: Self = Self::new(30);

        #[doc = "x1/32"]
        pub const _11111: Self = Self::new(31);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyesccrLl_SPEC;
impl crate::sealed::RegSpec for DphyesccrLl_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Escape Mode Clock Control Register"]
pub type DphyesccrLl = crate::RegValueT<DphyesccrLl_SPEC>;

impl DphyesccrLl {
    #[doc = "Escape Mode Transfer Clock Division Ratio"]
    #[inline(always)]
    pub fn escdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        dphyesccr_ll::Escdiv,
        dphyesccr_ll::Escdiv,
        DphyesccrLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            dphyesccr_ll::Escdiv,
            dphyesccr_ll::Escdiv,
            DphyesccrLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, DphyesccrLl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,DphyesccrLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyesccrLl {
    #[inline(always)]
    fn default() -> DphyesccrLl {
        <crate::RegValueT<DphyesccrLl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphyesccr_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Escdiv_SPEC;
    pub type Escdiv = crate::EnumBitfieldStruct<u8, Escdiv_SPEC>;
    impl Escdiv {
        #[doc = "x1/1"]
        pub const _00000: Self = Self::new(0);

        #[doc = "x1/2"]
        pub const _00001: Self = Self::new(1);

        #[doc = "x1/3"]
        pub const _00010: Self = Self::new(2);

        #[doc = "x1/4"]
        pub const _00011: Self = Self::new(3);

        #[doc = "x1/30"]
        pub const _11101: Self = Self::new(29);

        #[doc = "x1/31"]
        pub const _11110: Self = Self::new(30);

        #[doc = "x1/32"]
        pub const _11111: Self = Self::new(31);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphypwrcr_SPEC;
impl crate::sealed::RegSpec for Dphypwrcr_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Power Supplying Control Register"]
pub type Dphypwrcr = crate::RegValueT<Dphypwrcr_SPEC>;

impl Dphypwrcr {
    #[doc = "D-PHY Power Supplying Control"]
    #[inline(always)]
    pub fn pwrsen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphypwrcr::Pwrsen,
        dphypwrcr::Pwrsen,
        Dphypwrcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphypwrcr::Pwrsen,
            dphypwrcr::Pwrsen,
            Dphypwrcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7fffffff,
        1,
        0,
        u32,
        u32,
        Dphypwrcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7fffffff,
            1,
            0,
            u32,
            u32,
            Dphypwrcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dphypwrcr {
    #[inline(always)]
    fn default() -> Dphypwrcr {
        <crate::RegValueT<Dphypwrcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphypwrcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pwrsen_SPEC;
    pub type Pwrsen = crate::EnumBitfieldStruct<u8, Pwrsen_SPEC>;
    impl Pwrsen {
        #[doc = "Disable D-PHY LDO operation (Stop supplying VDD_DPHY)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable D-PHY LDO operation (Supply VDD_DPHY)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphypwrcrL_SPEC;
impl crate::sealed::RegSpec for DphypwrcrL_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY Power Supplying Control Register"]
pub type DphypwrcrL = crate::RegValueT<DphypwrcrL_SPEC>;

impl DphypwrcrL {
    #[doc = "D-PHY Power Supplying Control"]
    #[inline(always)]
    pub fn pwrsen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphypwrcr_l::Pwrsen,
        dphypwrcr_l::Pwrsen,
        DphypwrcrL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphypwrcr_l::Pwrsen,
            dphypwrcr_l::Pwrsen,
            DphypwrcrL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fff, 1, 0, u16, u16, DphypwrcrL_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fff,1,0,u16,u16,DphypwrcrL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphypwrcrL {
    #[inline(always)]
    fn default() -> DphypwrcrL {
        <crate::RegValueT<DphypwrcrL_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphypwrcr_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pwrsen_SPEC;
    pub type Pwrsen = crate::EnumBitfieldStruct<u8, Pwrsen_SPEC>;
    impl Pwrsen {
        #[doc = "Disable D-PHY LDO operation (Stop supplying VDD_DPHY)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable D-PHY LDO operation (Supply VDD_DPHY)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphypwrcrLl_SPEC;
impl crate::sealed::RegSpec for DphypwrcrLl_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Power Supplying Control Register"]
pub type DphypwrcrLl = crate::RegValueT<DphypwrcrLl_SPEC>;

impl DphypwrcrLl {
    #[doc = "D-PHY Power Supplying Control"]
    #[inline(always)]
    pub fn pwrsen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphypwrcr_ll::Pwrsen,
        dphypwrcr_ll::Pwrsen,
        DphypwrcrLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphypwrcr_ll::Pwrsen,
            dphypwrcr_ll::Pwrsen,
            DphypwrcrLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, DphypwrcrLl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,DphypwrcrLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphypwrcrLl {
    #[inline(always)]
    fn default() -> DphypwrcrLl {
        <crate::RegValueT<DphypwrcrLl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphypwrcr_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pwrsen_SPEC;
    pub type Pwrsen = crate::EnumBitfieldStruct<u8, Pwrsen_SPEC>;
    impl Pwrsen {
        #[doc = "Disable D-PHY LDO operation (Stop supplying VDD_DPHY)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable D-PHY LDO operation (Supply VDD_DPHY)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphysfr_SPEC;
impl crate::sealed::RegSpec for Dphysfr_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Status Flag Register"]
pub type Dphysfr = crate::RegValueT<Dphysfr_SPEC>;

impl Dphysfr {
    #[doc = "D-PHY LDO Power-on Status Flag"]
    #[inline(always)]
    pub fn pwrsf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphysfr::Pwrsf,
        dphysfr::Pwrsf,
        Dphysfr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphysfr::Pwrsf,
            dphysfr::Pwrsf,
            Dphysfr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "D-PHY PLL Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn pllsf(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dphysfr::Pllsf,
        dphysfr::Pllsf,
        Dphysfr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dphysfr::Pllsf,
            dphysfr::Pllsf,
            Dphysfr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7fffff, 1, 0, u32, u32, Dphysfr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x7fffff,1,0,u32,u32,Dphysfr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphysfr {
    #[inline(always)]
    fn default() -> Dphysfr {
        <crate::RegValueT<Dphysfr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphysfr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pwrsf_SPEC;
    pub type Pwrsf = crate::EnumBitfieldStruct<u8, Pwrsf_SPEC>;
    impl Pwrsf {
        #[doc = "D-PHY LDO is stopped or starting up"]
        pub const _0: Self = Self::new(0);

        #[doc = "D-PHY LDO startup is completed (VDD_DPHY is stable)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllsf_SPEC;
    pub type Pllsf = crate::EnumBitfieldStruct<u8, Pllsf_SPEC>;
    impl Pllsf {
        #[doc = "D-PHY PLL clock is stopped or is not yet stable"]
        pub const _0: Self = Self::new(0);

        #[doc = "D-PHY PLL clock is stable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphysfrL_SPEC;
impl crate::sealed::RegSpec for DphysfrL_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY Status Flag Register"]
pub type DphysfrL = crate::RegValueT<DphysfrL_SPEC>;

impl DphysfrL {
    #[doc = "D-PHY LDO Power-on Status Flag"]
    #[inline(always)]
    pub fn pwrsf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphysfr_l::Pwrsf,
        dphysfr_l::Pwrsf,
        DphysfrL_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphysfr_l::Pwrsf,
            dphysfr_l::Pwrsf,
            DphysfrL_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "D-PHY PLL Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn pllsf(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dphysfr_l::Pllsf,
        dphysfr_l::Pllsf,
        DphysfrL_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dphysfr_l::Pllsf,
            dphysfr_l::Pllsf,
            DphysfrL_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, DphysfrL_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,DphysfrL_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DphysfrL {
    #[inline(always)]
    fn default() -> DphysfrL {
        <crate::RegValueT<DphysfrL_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphysfr_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pwrsf_SPEC;
    pub type Pwrsf = crate::EnumBitfieldStruct<u8, Pwrsf_SPEC>;
    impl Pwrsf {
        #[doc = "D-PHY LDO is stopped or starting up"]
        pub const _0: Self = Self::new(0);

        #[doc = "D-PHY LDO startup is completed (VDD_DPHY is stable)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllsf_SPEC;
    pub type Pllsf = crate::EnumBitfieldStruct<u8, Pllsf_SPEC>;
    impl Pllsf {
        #[doc = "D-PHY PLL clock is stopped or is not yet stable"]
        pub const _0: Self = Self::new(0);

        #[doc = "D-PHY PLL clock is stable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphysfrLl_SPEC;
impl crate::sealed::RegSpec for DphysfrLl_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Status Flag Register"]
pub type DphysfrLl = crate::RegValueT<DphysfrLl_SPEC>;

impl DphysfrLl {
    #[doc = "D-PHY LDO Power-on Status Flag"]
    #[inline(always)]
    pub fn pwrsf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphysfr_ll::Pwrsf,
        dphysfr_ll::Pwrsf,
        DphysfrLl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphysfr_ll::Pwrsf,
            dphysfr_ll::Pwrsf,
            DphysfrLl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, DphysfrLl_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,DphysfrLl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DphysfrLl {
    #[inline(always)]
    fn default() -> DphysfrLl {
        <crate::RegValueT<DphysfrLl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphysfr_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pwrsf_SPEC;
    pub type Pwrsf = crate::EnumBitfieldStruct<u8, Pwrsf_SPEC>;
    impl Pwrsf {
        #[doc = "D-PHY LDO is stopped or starting up"]
        pub const _0: Self = Self::new(0);

        #[doc = "D-PHY LDO startup is completed (VDD_DPHY is stable)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphysfrLh_SPEC;
impl crate::sealed::RegSpec for DphysfrLh_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Status Flag Register"]
pub type DphysfrLh = crate::RegValueT<DphysfrLh_SPEC>;

impl DphysfrLh {
    #[doc = "D-PHY PLL Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn pllsf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphysfr_lh::Pllsf,
        dphysfr_lh::Pllsf,
        DphysfrLh_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphysfr_lh::Pllsf,
            dphysfr_lh::Pllsf,
            DphysfrLh_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, DphysfrLh_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,DphysfrLh_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DphysfrLh {
    #[inline(always)]
    fn default() -> DphysfrLh {
        <crate::RegValueT<DphysfrLh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphysfr_lh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllsf_SPEC;
    pub type Pllsf = crate::EnumBitfieldStruct<u8, Pllsf_SPEC>;
    impl Pllsf {
        #[doc = "D-PHY PLL clock is stopped or is not yet stable"]
        pub const _0: Self = Self::new(0);

        #[doc = "D-PHY PLL clock is stable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphyocr_SPEC;
impl crate::sealed::RegSpec for Dphyocr_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Operation Control Register"]
pub type Dphyocr = crate::RegValueT<Dphyocr_SPEC>;

impl Dphyocr {
    #[doc = "D-PHY Operation Control"]
    #[inline(always)]
    pub fn dphyen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphyocr::Dphyen,
        dphyocr::Dphyen,
        Dphyocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphyocr::Dphyen,
            dphyocr::Dphyen,
            Dphyocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, u32, Dphyocr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32,u32,Dphyocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphyocr {
    #[inline(always)]
    fn default() -> Dphyocr {
        <crate::RegValueT<Dphyocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphyocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dphyen_SPEC;
    pub type Dphyen = crate::EnumBitfieldStruct<u8, Dphyen_SPEC>;
    impl Dphyen {
        #[doc = "Disable D-PHY operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable D-PHY operation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyocrL_SPEC;
impl crate::sealed::RegSpec for DphyocrL_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY Operation Control Register"]
pub type DphyocrL = crate::RegValueT<DphyocrL_SPEC>;

impl DphyocrL {
    #[doc = "D-PHY Operation Control"]
    #[inline(always)]
    pub fn dphyen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphyocr_l::Dphyen,
        dphyocr_l::Dphyen,
        DphyocrL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphyocr_l::Dphyen,
            dphyocr_l::Dphyen,
            DphyocrL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fff, 1, 0, u16, u16, DphyocrL_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fff,1,0,u16,u16,DphyocrL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyocrL {
    #[inline(always)]
    fn default() -> DphyocrL {
        <crate::RegValueT<DphyocrL_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphyocr_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dphyen_SPEC;
    pub type Dphyen = crate::EnumBitfieldStruct<u8, Dphyen_SPEC>;
    impl Dphyen {
        #[doc = "Disable D-PHY operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable D-PHY operation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyocrLl_SPEC;
impl crate::sealed::RegSpec for DphyocrLl_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Operation Control Register"]
pub type DphyocrLl = crate::RegValueT<DphyocrLl_SPEC>;

impl DphyocrLl {
    #[doc = "D-PHY Operation Control"]
    #[inline(always)]
    pub fn dphyen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dphyocr_ll::Dphyen,
        dphyocr_ll::Dphyen,
        DphyocrLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dphyocr_ll::Dphyen,
            dphyocr_ll::Dphyen,
            DphyocrLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, DphyocrLl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,DphyocrLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyocrLl {
    #[inline(always)]
    fn default() -> DphyocrLl {
        <crate::RegValueT<DphyocrLl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dphyocr_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dphyen_SPEC;
    pub type Dphyen = crate::EnumBitfieldStruct<u8, Dphyen_SPEC>;
    impl Dphyen {
        #[doc = "Disable D-PHY operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable D-PHY operation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim1_SPEC;
impl crate::sealed::RegSpec for Dphytim1_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Timing Control Register 1"]
pub type Dphytim1 = crate::RegValueT<Dphytim1_SPEC>;

impl Dphytim1 {
    #[doc = "D-PHY T_INIT Parameter Setting"]
    #[inline(always)]
    pub fn tinit(
        self,
    ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, u32, Dphytim1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ffff,1,0,u32,u32,Dphytim1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000000000000. The write value should be 0000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<19, 0x1fff, 1, 0, u16, u16, Dphytim1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1fff,1,0,u16,u16,Dphytim1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim1 {
    #[inline(always)]
    fn default() -> Dphytim1 {
        <crate::RegValueT<Dphytim1_SPEC> as RegisterValue<_>>::new(72001)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim1L_SPEC;
impl crate::sealed::RegSpec for Dphytim1L_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY Timing Control Register 1"]
pub type Dphytim1L = crate::RegValueT<Dphytim1L_SPEC>;

impl Dphytim1L {
    #[doc = "D-PHY T_INIT Parameter Setting"]
    #[inline(always)]
    pub fn tinit(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dphytim1L_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dphytim1L_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim1L {
    #[inline(always)]
    fn default() -> Dphytim1L {
        <crate::RegValueT<Dphytim1L_SPEC> as RegisterValue<_>>::new(6465)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim1Ll_SPEC;
impl crate::sealed::RegSpec for Dphytim1Ll_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 1"]
pub type Dphytim1Ll = crate::RegValueT<Dphytim1Ll_SPEC>;

impl Dphytim1Ll {
    #[doc = "D-PHY T_INIT Parameter Setting"]
    #[inline(always)]
    pub fn tinit(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim1Ll_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim1Ll_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim1Ll {
    #[inline(always)]
    fn default() -> Dphytim1Ll {
        <crate::RegValueT<Dphytim1Ll_SPEC> as RegisterValue<_>>::new(65)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim1Lh_SPEC;
impl crate::sealed::RegSpec for Dphytim1Lh_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 1"]
pub type Dphytim1Lh = crate::RegValueT<Dphytim1Lh_SPEC>;

impl Dphytim1Lh {
    #[doc = "D-PHY T_INIT Parameter Setting"]
    #[inline(always)]
    pub fn tinit(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim1Lh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim1Lh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim1Lh {
    #[inline(always)]
    fn default() -> Dphytim1Lh {
        <crate::RegValueT<Dphytim1Lh_SPEC> as RegisterValue<_>>::new(25)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim1H_SPEC;
impl crate::sealed::RegSpec for Dphytim1H_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY Timing Control Register 1"]
pub type Dphytim1H = crate::RegValueT<Dphytim1H_SPEC>;

impl Dphytim1H {
    #[doc = "D-PHY T_INIT Parameter Setting"]
    #[inline(always)]
    pub fn tinit(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Dphytim1H_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Dphytim1H_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000000000000. The write value should be 0000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1fff, 1, 0, u16, u16, Dphytim1H_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1fff,1,0,u16,u16,Dphytim1H_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim1H {
    #[inline(always)]
    fn default() -> Dphytim1H {
        <crate::RegValueT<Dphytim1H_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim1Hl_SPEC;
impl crate::sealed::RegSpec for Dphytim1Hl_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 1"]
pub type Dphytim1Hl = crate::RegValueT<Dphytim1Hl_SPEC>;

impl Dphytim1Hl {
    #[doc = "D-PHY T_INIT Parameter Setting"]
    #[inline(always)]
    pub fn tinit(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Dphytim1Hl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Dphytim1Hl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Dphytim1Hl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Dphytim1Hl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim1Hl {
    #[inline(always)]
    fn default() -> Dphytim1Hl {
        <crate::RegValueT<Dphytim1Hl_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim2Ll_SPEC;
impl crate::sealed::RegSpec for Dphytim2Ll_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 2"]
pub type Dphytim2Ll = crate::RegValueT<Dphytim2Ll_SPEC>;

impl Dphytim2Ll {
    #[doc = "D-PHY T_CLK_PREPARE Parameter Setting"]
    #[inline(always)]
    pub fn tclkprep(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim2Ll_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim2Ll_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim2Ll {
    #[inline(always)]
    fn default() -> Dphytim2Ll {
        <crate::RegValueT<Dphytim2Ll_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim2Lh_SPEC;
impl crate::sealed::RegSpec for Dphytim2Lh_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 2"]
pub type Dphytim2Lh = crate::RegValueT<Dphytim2Lh_SPEC>;

impl Dphytim2Lh {
    #[doc = "These bits are read as 00010000. The write value should be 00010000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim2Lh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim2Lh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim2Lh {
    #[inline(always)]
    fn default() -> Dphytim2Lh {
        <crate::RegValueT<Dphytim2Lh_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim2Hl_SPEC;
impl crate::sealed::RegSpec for Dphytim2Hl_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 2"]
pub type Dphytim2Hl = crate::RegValueT<Dphytim2Hl_SPEC>;

impl Dphytim2Hl {
    #[doc = "These bits are read as 00000010. The write value should be 00000010."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim2Hl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim2Hl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim2Hl {
    #[inline(always)]
    fn default() -> Dphytim2Hl {
        <crate::RegValueT<Dphytim2Hl_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim3Ll_SPEC;
impl crate::sealed::RegSpec for Dphytim3Ll_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 3"]
pub type Dphytim3Ll = crate::RegValueT<Dphytim3Ll_SPEC>;

impl Dphytim3Ll {
    #[doc = "D-PHY T_THS_PREPARE Parameter Setting"]
    #[inline(always)]
    pub fn thsprep(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim3Ll_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim3Ll_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim3Ll {
    #[inline(always)]
    fn default() -> Dphytim3Ll {
        <crate::RegValueT<Dphytim3Ll_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim3Lh_SPEC;
impl crate::sealed::RegSpec for Dphytim3Lh_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 3"]
pub type Dphytim3Lh = crate::RegValueT<Dphytim3Lh_SPEC>;

impl Dphytim3Lh {
    #[doc = "These bits are read as 00010000. The write value should be 00010000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim3Lh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim3Lh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim3Lh {
    #[inline(always)]
    fn default() -> Dphytim3Lh {
        <crate::RegValueT<Dphytim3Lh_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim4_SPEC;
impl crate::sealed::RegSpec for Dphytim4_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Timing Control Register 4"]
pub type Dphytim4 = crate::RegValueT<Dphytim4_SPEC>;

impl Dphytim4 {
    #[doc = "D-PHY T_CLK_ZERO Parameter Setting"]
    #[inline(always)]
    pub fn tclkzero(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_TCLK_PRE Parameter Setting"]
    #[inline(always)]
    pub fn tclkpre(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dphytim4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Dphytim4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_TCLK_POST Parameter Setting"]
    #[inline(always)]
    pub fn tclkpost(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Dphytim4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Dphytim4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_TCLK_TRAIL Parameter Setting"]
    #[inline(always)]
    pub fn tclktrl(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Dphytim4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Dphytim4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim4 {
    #[inline(always)]
    fn default() -> Dphytim4 {
        <crate::RegValueT<Dphytim4_SPEC> as RegisterValue<_>>::new(102761247)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim4L_SPEC;
impl crate::sealed::RegSpec for Dphytim4L_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY Timing Control Register 4"]
pub type Dphytim4L = crate::RegValueT<Dphytim4L_SPEC>;

impl Dphytim4L {
    #[doc = "D-PHY T_CLK_ZERO Parameter Setting"]
    #[inline(always)]
    pub fn tclkzero(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim4L_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim4L_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_TCLK_PRE Parameter Setting"]
    #[inline(always)]
    pub fn tclkpre(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dphytim4L_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Dphytim4L_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim4L {
    #[inline(always)]
    fn default() -> Dphytim4L {
        <crate::RegValueT<Dphytim4L_SPEC> as RegisterValue<_>>::new(799)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim4Ll_SPEC;
impl crate::sealed::RegSpec for Dphytim4Ll_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 4"]
pub type Dphytim4Ll = crate::RegValueT<Dphytim4Ll_SPEC>;

impl Dphytim4Ll {
    #[doc = "D-PHY T_CLK_ZERO Parameter Setting"]
    #[inline(always)]
    pub fn tclkzero(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim4Ll_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim4Ll_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim4Ll {
    #[inline(always)]
    fn default() -> Dphytim4Ll {
        <crate::RegValueT<Dphytim4Ll_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim4Lh_SPEC;
impl crate::sealed::RegSpec for Dphytim4Lh_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 4"]
pub type Dphytim4Lh = crate::RegValueT<Dphytim4Lh_SPEC>;

impl Dphytim4Lh {
    #[doc = "D-PHY T_TCLK_PRE Parameter Setting"]
    #[inline(always)]
    pub fn tclkpre(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim4Lh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim4Lh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim4Lh {
    #[inline(always)]
    fn default() -> Dphytim4Lh {
        <crate::RegValueT<Dphytim4Lh_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim4H_SPEC;
impl crate::sealed::RegSpec for Dphytim4H_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY Timing Control Register 4"]
pub type Dphytim4H = crate::RegValueT<Dphytim4H_SPEC>;

impl Dphytim4H {
    #[doc = "D-PHY T_TCLK_POST Parameter Setting"]
    #[inline(always)]
    pub fn tclkpost(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim4H_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim4H_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_TCLK_TRAIL Parameter Setting"]
    #[inline(always)]
    pub fn tclktrl(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dphytim4H_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Dphytim4H_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim4H {
    #[inline(always)]
    fn default() -> Dphytim4H {
        <crate::RegValueT<Dphytim4H_SPEC> as RegisterValue<_>>::new(1568)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim4Hl_SPEC;
impl crate::sealed::RegSpec for Dphytim4Hl_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 4"]
pub type Dphytim4Hl = crate::RegValueT<Dphytim4Hl_SPEC>;

impl Dphytim4Hl {
    #[doc = "D-PHY T_TCLK_POST Parameter Setting"]
    #[inline(always)]
    pub fn tclkpost(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim4Hl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim4Hl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim4Hl {
    #[inline(always)]
    fn default() -> Dphytim4Hl {
        <crate::RegValueT<Dphytim4Hl_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim4Hh_SPEC;
impl crate::sealed::RegSpec for Dphytim4Hh_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 4"]
pub type Dphytim4Hh = crate::RegValueT<Dphytim4Hh_SPEC>;

impl Dphytim4Hh {
    #[doc = "D-PHY T_TCLK_TRAIL Parameter Setting"]
    #[inline(always)]
    pub fn tclktrl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim4Hh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim4Hh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim4Hh {
    #[inline(always)]
    fn default() -> Dphytim4Hh {
        <crate::RegValueT<Dphytim4Hh_SPEC> as RegisterValue<_>>::new(6)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim5_SPEC;
impl crate::sealed::RegSpec for Dphytim5_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Timing Control Register 5"]
pub type Dphytim5 = crate::RegValueT<Dphytim5_SPEC>;

impl Dphytim5 {
    #[doc = "D-PHY T_THS_ZERO Parameter Setting"]
    #[inline(always)]
    pub fn thszero(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_THS_TRAIL Parameter Setting"]
    #[inline(always)]
    pub fn thstrl(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dphytim5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Dphytim5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_THS_EXIT Parameter Setting"]
    #[inline(always)]
    pub fn thsexit(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Dphytim5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Dphytim5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Dphytim5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Dphytim5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim5 {
    #[inline(always)]
    fn default() -> Dphytim5 {
        <crate::RegValueT<Dphytim5_SPEC> as RegisterValue<_>>::new(788495)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim5L_SPEC;
impl crate::sealed::RegSpec for Dphytim5L_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY Timing Control Register 5"]
pub type Dphytim5L = crate::RegValueT<Dphytim5L_SPEC>;

impl Dphytim5L {
    #[doc = "D-PHY T_THS_ZERO Parameter Setting"]
    #[inline(always)]
    pub fn thszero(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim5L_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim5L_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D-PHY T_THS_TRAIL Parameter Setting"]
    #[inline(always)]
    pub fn thstrl(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dphytim5L_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Dphytim5L_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim5L {
    #[inline(always)]
    fn default() -> Dphytim5L {
        <crate::RegValueT<Dphytim5L_SPEC> as RegisterValue<_>>::new(2063)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim5Ll_SPEC;
impl crate::sealed::RegSpec for Dphytim5Ll_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 5"]
pub type Dphytim5Ll = crate::RegValueT<Dphytim5Ll_SPEC>;

impl Dphytim5Ll {
    #[doc = "D-PHY T_THS_ZERO Parameter Setting"]
    #[inline(always)]
    pub fn thszero(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim5Ll_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim5Ll_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim5Ll {
    #[inline(always)]
    fn default() -> Dphytim5Ll {
        <crate::RegValueT<Dphytim5Ll_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim5Lh_SPEC;
impl crate::sealed::RegSpec for Dphytim5Lh_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 5"]
pub type Dphytim5Lh = crate::RegValueT<Dphytim5Lh_SPEC>;

impl Dphytim5Lh {
    #[doc = "D-PHY T_THS_TRAIL Parameter Setting"]
    #[inline(always)]
    pub fn thstrl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim5Lh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim5Lh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim5Lh {
    #[inline(always)]
    fn default() -> Dphytim5Lh {
        <crate::RegValueT<Dphytim5Lh_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim5H_SPEC;
impl crate::sealed::RegSpec for Dphytim5H_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY Timing Control Register 5"]
pub type Dphytim5H = crate::RegValueT<Dphytim5H_SPEC>;

impl Dphytim5H {
    #[doc = "D-PHY T_THS_EXIT Parameter Setting"]
    #[inline(always)]
    pub fn thsexit(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim5H_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim5H_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dphytim5H_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Dphytim5H_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim5H {
    #[inline(always)]
    fn default() -> Dphytim5H {
        <crate::RegValueT<Dphytim5H_SPEC> as RegisterValue<_>>::new(12)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim5Hl_SPEC;
impl crate::sealed::RegSpec for Dphytim5Hl_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 5"]
pub type Dphytim5Hl = crate::RegValueT<Dphytim5Hl_SPEC>;

impl Dphytim5Hl {
    #[doc = "D-PHY T_THS_EXIT Parameter Setting"]
    #[inline(always)]
    pub fn thsexit(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim5Hl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim5Hl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim5Hl {
    #[inline(always)]
    fn default() -> Dphytim5Hl {
        <crate::RegValueT<Dphytim5Hl_SPEC> as RegisterValue<_>>::new(12)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim6_SPEC;
impl crate::sealed::RegSpec for Dphytim6_SPEC {
    type DataType = u32;
}

#[doc = "D-PHY Timing Control Register 6"]
pub type Dphytim6 = crate::RegValueT<Dphytim6_SPEC>;

impl Dphytim6 {
    #[doc = "D-PHY T_TLPX Parameter Setting"]
    #[inline(always)]
    pub fn tlpx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000000000000000000000000. The write value should be 000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, u32, Dphytim6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xffffff,1,0,u32,u32,Dphytim6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim6 {
    #[inline(always)]
    fn default() -> Dphytim6 {
        <crate::RegValueT<Dphytim6_SPEC> as RegisterValue<_>>::new(6)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim6L_SPEC;
impl crate::sealed::RegSpec for Dphytim6L_SPEC {
    type DataType = u16;
}

#[doc = "D-PHY Timing Control Register 6"]
pub type Dphytim6L = crate::RegValueT<Dphytim6L_SPEC>;

impl Dphytim6L {
    #[doc = "D-PHY T_TLPX Parameter Setting"]
    #[inline(always)]
    pub fn tlpx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim6L_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim6L_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dphytim6L_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Dphytim6L_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim6L {
    #[inline(always)]
    fn default() -> Dphytim6L {
        <crate::RegValueT<Dphytim6L_SPEC> as RegisterValue<_>>::new(6)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dphytim6Ll_SPEC;
impl crate::sealed::RegSpec for Dphytim6Ll_SPEC {
    type DataType = u8;
}

#[doc = "D-PHY Timing Control Register 6"]
pub type Dphytim6Ll = crate::RegValueT<Dphytim6Ll_SPEC>;

impl Dphytim6Ll {
    #[doc = "D-PHY T_TLPX Parameter Setting"]
    #[inline(always)]
    pub fn tlpx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dphytim6Ll_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dphytim6Ll_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dphytim6Ll {
    #[inline(always)]
    fn default() -> Dphytim6Ll {
        <crate::RegValueT<Dphytim6Ll_SPEC> as RegisterValue<_>>::new(6)
    }
}
