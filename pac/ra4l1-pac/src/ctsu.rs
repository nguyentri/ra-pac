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
// Generated from SVD 1.20.01, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:48:57 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Capacitive Touch Sensing Unit"]
unsafe impl ::core::marker::Send for super::Ctsu {}
unsafe impl ::core::marker::Sync for super::Ctsu {}
impl super::Ctsu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "CTSU Control Register A"]
    #[inline(always)]
    pub const fn ctsucra(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "CTSU Control Register A"]
    #[inline(always)]
    pub const fn ctsucral(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucral_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucral_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "CTSU Control Register A"]
    #[inline(always)]
    pub const fn ctsucr0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "CTSU Control Register A"]
    #[inline(always)]
    pub const fn ctsucr1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "CTSU Control Register A"]
    #[inline(always)]
    pub const fn ctsucr2(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "CTSU Control Register A"]
    #[inline(always)]
    pub const fn ctsucr3(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "CTSU Control Register B"]
    #[inline(always)]
    pub const fn ctsucrb(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "CTSU Control Register B"]
    #[inline(always)]
    pub const fn ctsucrbl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucrbl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucrbl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "CTSU Control Register B"]
    #[inline(always)]
    pub const fn ctsusdprs(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusdprs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusdprs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "CTSU Control Register B"]
    #[inline(always)]
    pub const fn ctsusst(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[doc = "CTSU Control Register B"]
    #[inline(always)]
    pub const fn ctsucrbh(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucrbh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucrbh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "CTSU Control Register B"]
    #[inline(always)]
    pub const fn ctsudclkc(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsudclkc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsudclkc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[doc = "CTSU Measurement Channel Register"]
    #[inline(always)]
    pub const fn ctsumch(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "CTSU Measurement Channel Register"]
    #[inline(always)]
    pub const fn ctsumchl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumchl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumchl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "CTSU Measurement Channel Register"]
    #[inline(always)]
    pub const fn ctsumch0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumch0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumch0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "CTSU Measurement Channel Register"]
    #[inline(always)]
    pub const fn ctsumch1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumch1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumch1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }

    #[doc = "CTSU Measurement Channel Register"]
    #[inline(always)]
    pub const fn ctsumchh(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumchh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumchh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "CTSU Measurement Channel Register"]
    #[inline(always)]
    pub const fn ctsumfaf(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumfaf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumfaf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "CTSU Channel Enable Control Register A"]
    #[inline(always)]
    pub const fn ctsuchaca(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "CTSU Channel Enable Control Register A"]
    #[inline(always)]
    pub const fn ctsuchacal(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchacal_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchacal_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "CTSU Channel Enable Control Register A"]
    #[inline(always)]
    pub const fn ctsuchac0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "CTSU Channel Enable Control Register A"]
    #[inline(always)]
    pub const fn ctsuchac1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[doc = "CTSU Channel Transmit/Receive Control Register A"]
    #[inline(always)]
    pub const fn ctsuchtrca(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "CTSU Channel Transmit/Receive Control Register A"]
    #[inline(always)]
    pub const fn ctsuchtrcal(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrcal_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrcal_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "CTSU Channel Transmit/Receive Control Register A"]
    #[inline(always)]
    pub const fn ctsuchtrc0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "CTSU Channel Transmit/Receive Control Register A"]
    #[inline(always)]
    pub const fn ctsuchtrc1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(21usize),
            )
        }
    }

    #[doc = "CTSU Status Register"]
    #[inline(always)]
    pub const fn ctsusr(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "CTSU Status Register"]
    #[inline(always)]
    pub const fn ctsusrl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "CTSU Status Register"]
    #[inline(always)]
    pub const fn ctsusr0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "CTSU Status Register"]
    #[inline(always)]
    pub const fn ctsust(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsust_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsust_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(29usize),
            )
        }
    }

    #[doc = "CTSU Status Register"]
    #[inline(always)]
    pub const fn ctsusrh(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusrh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusrh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[doc = "CTSU Status Register"]
    #[inline(always)]
    pub const fn ctsusr2(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[doc = "CTSU Sensor Offset Register"]
    #[inline(always)]
    pub const fn ctsuso(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuso_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuso_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "CTSU Sensor Offset Register"]
    #[inline(always)]
    pub const fn ctsuso0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuso0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuso0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "CTSU Sensor Offset Register"]
    #[inline(always)]
    pub const fn ctsuso1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuso1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuso1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[doc = "CTSU Sensor Counter Register"]
    #[inline(always)]
    pub const fn ctsuscnt(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuscnt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ctsuscnt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "CTSU Sensor Counter Register"]
    #[inline(always)]
    pub const fn ctsusc(&self) -> &'static crate::common::Reg<self::Ctsusc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ctsusc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "CTSU Calibration Register"]
    #[inline(always)]
    pub const fn ctsucalib(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucalib_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucalib_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "CTSU Calibration Register"]
    #[inline(always)]
    pub const fn ctsudbgr0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsudbgr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsudbgr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "CTSU Calibration Register"]
    #[inline(always)]
    pub const fn ctsudbgr1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsudbgr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsudbgr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[doc = "CTSU Sensor Unit Clock Control Register A"]
    #[inline(always)]
    pub const fn ctsusuclka(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusuclka_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusuclka_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "CTSU Sensor Unit Clock Control Register A"]
    #[inline(always)]
    pub const fn ctsusuclk0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusuclk0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusuclk0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "CTSU Sensor Unit Clock Control Register A"]
    #[inline(always)]
    pub const fn ctsusuclk1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusuclk1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusuclk1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[doc = "CTSU Sensor Unit Clock Control Register B"]
    #[inline(always)]
    pub const fn ctsusuclkb(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusuclkb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusuclkb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "CTSU Sensor Unit Clock Control Register B"]
    #[inline(always)]
    pub const fn ctsusuclk2(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusuclk2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusuclk2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "CTSU Sensor Unit Clock Control Register B"]
    #[inline(always)]
    pub const fn ctsusuclk3(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusuclk3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusuclk3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "CTSU Option Setting Register"]
    #[inline(always)]
    pub const fn ctsuopt(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuopt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuopt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "CTSU Option Setting Register"]
    #[inline(always)]
    pub const fn ctsuoptl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuoptl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuoptl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "CTSU Option Setting Register"]
    #[inline(always)]
    pub const fn ac(&self) -> &'static crate::common::Reg<self::Ac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "CTSU Option Setting Register"]
    #[inline(always)]
    pub const fn aj(&self) -> &'static crate::common::Reg<self::Aj_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Aj_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65usize),
            )
        }
    }

    #[doc = "CTSU Option Setting Register"]
    #[inline(always)]
    pub const fn ctsuopth(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuopth_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuopth_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[doc = "CTSU Option Setting Register"]
    #[inline(always)]
    pub const fn acsb(&self) -> &'static crate::common::Reg<self::Acsb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Acsb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[doc = "CTSU Sensor Counter Automatic Correction Table Access Register"]
    #[inline(always)]
    pub const fn ctsuscntact(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuscntact_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuscntact_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "CTSU Sensor Counter Automatic Correction Table Access Register"]
    #[inline(always)]
    pub const fn ctsuscntactl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuscntactl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuscntactl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "CTSU Sensor Counter Automatic Correction Table Access Register"]
    #[inline(always)]
    pub const fn ctsuscntacth(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuscntacth_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuscntacth_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(70usize),
            )
        }
    }

    #[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
    #[inline(always)]
    pub const fn ctsumact1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
    #[inline(always)]
    pub const fn ctsumact1l(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact1L_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact1L_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
    #[inline(always)]
    pub const fn ctsumact1h(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact1H_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact1H_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(78usize),
            )
        }
    }

    #[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
    #[inline(always)]
    pub const fn ctsumact2(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
    #[inline(always)]
    pub const fn ctsumact2l(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact2L_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact2L_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
    #[inline(always)]
    pub const fn ctsumact2h(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact2H_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact2H_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(82usize),
            )
        }
    }

    #[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
    #[inline(always)]
    pub const fn ctsumact3(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
    #[inline(always)]
    pub const fn ctsumact3l(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact3L_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact3L_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
    #[inline(always)]
    pub const fn ctsumact3h(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact3H_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact3H_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(86usize),
            )
        }
    }

    #[doc = "CTSU Automatic Judgment Control Register"]
    #[inline(always)]
    pub const fn ctsuajcr(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "CTSU Automatic Judgment Control Register"]
    #[inline(always)]
    pub const fn ctsuajcrl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajcrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajcrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "CTSU Automatic Judgment Control Register"]
    #[inline(always)]
    pub const fn ajcr0(&self) -> &'static crate::common::Reg<self::Ajcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ajcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "CTSU Automatic Judgment Control Register"]
    #[inline(always)]
    pub const fn ajcr1(&self) -> &'static crate::common::Reg<self::Ajcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ajcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(89usize),
            )
        }
    }

    #[doc = "CTSU Automatic Judgment Control Register"]
    #[inline(always)]
    pub const fn ctsuajcrh(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajcrh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajcrh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(90usize),
            )
        }
    }

    #[doc = "CTSU Automatic Judgment Control Register"]
    #[inline(always)]
    pub const fn ajcr2(&self) -> &'static crate::common::Reg<self::Ajcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ajcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(90usize),
            )
        }
    }

    #[doc = "CTSU Automatic Judgment Control Register"]
    #[inline(always)]
    pub const fn ajcr3(&self) -> &'static crate::common::Reg<self::Ajcr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ajcr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(91usize),
            )
        }
    }

    #[doc = "CTSU Automatic Judgment Control Register"]
    #[inline(always)]
    pub const fn ctsuajthr(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajthr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajthr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "CTSU Automatic Judgment Control Register"]
    #[inline(always)]
    pub const fn ctsuajthrl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajthrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajthrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "CTSU Automatic Judgment Control Register"]
    #[inline(always)]
    pub const fn ctsuajthrh(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajthrh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajthrh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(94usize),
            )
        }
    }

    #[doc = "CTSU Threshold Register"]
    #[inline(always)]
    pub const fn ctsuajmmar(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajmmar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajmmar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "CTSU Threshold Register"]
    #[inline(always)]
    pub const fn ctsuajmmarl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajmmarl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajmmarl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "CTSU Threshold Register"]
    #[inline(always)]
    pub const fn ctsuajmmarh(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajmmarh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajmmarh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(98usize),
            )
        }
    }

    #[doc = "CTSU Baseline Average Intermediate Result Register"]
    #[inline(always)]
    pub const fn ctsuajblact(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajblact_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajblact_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "CTSU Baseline Average Intermediate Result Register"]
    #[inline(always)]
    pub const fn ctsuajblactl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajblactl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajblactl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "CTSU Baseline Average Intermediate Result Register"]
    #[inline(always)]
    pub const fn ctsuajblacth(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajblacth_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajblacth_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(102usize),
            )
        }
    }

    #[doc = "CTSU Baseline Average Result Register"]
    #[inline(always)]
    pub const fn ctsuajblar(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajblar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajblar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "CTSU Baseline Average Result Register"]
    #[inline(always)]
    pub const fn ctsuajblarl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajblarl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajblarl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "CTSU Baseline Average Result Register"]
    #[inline(always)]
    pub const fn ctsuajblarh(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajblarh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajblarh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(106usize),
            )
        }
    }

    #[doc = "CTSU Automatic Judgment Result Register"]
    #[inline(always)]
    pub const fn ctsuajrr(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "CTSU Automatic Judgment Result Register"]
    #[inline(always)]
    pub const fn ctsuajrrl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajrrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajrrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "CTSU Automatic Judgment Result Register"]
    #[inline(always)]
    pub const fn ctsuajrr0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajrr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajrr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "CTSU Automatic Judgment Result Register"]
    #[inline(always)]
    pub const fn ctsuajrr1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajrr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajrr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(109usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucra_SPEC;
impl crate::sealed::RegSpec for Ctsucra_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Control Register A"]
pub type Ctsucra = crate::RegValueT<Ctsucra_SPEC>;

impl Ctsucra {
    #[doc = "CTSU Measurement Operation Start"]
    #[inline(always)]
    pub fn strt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsucra::Strt,
        ctsucra::Strt,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsucra::Strt,
            ctsucra::Strt,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Measurement Operation Start Trigger Select"]
    #[inline(always)]
    pub fn cap(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsucra::Cap,
        ctsucra::Cap,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsucra::Cap,
            ctsucra::Cap,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Wait State Power-Saving Enable"]
    #[inline(always)]
    pub fn snz(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsucra::Snz,
        ctsucra::Snz,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsucra::Snz,
            ctsucra::Snz,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Control Block Initialization"]
    #[inline(always)]
    pub fn init(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ctsucra_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ctsucra_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "CTSU Boost Circuit Control"]
    #[inline(always)]
    pub fn pumpon(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ctsucra::Pumpon,
        ctsucra::Pumpon,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ctsucra::Pumpon,
            ctsucra::Pumpon,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Transmission Power Supply Selection"]
    #[inline(always)]
    pub fn txvsel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        ctsucra::Txvsel,
        ctsucra::Txvsel,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            ctsucra::Txvsel,
            ctsucra::Txvsel,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Power On Control"]
    #[inline(always)]
    pub fn pon(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ctsucra::Pon,
        ctsucra::Pon,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ctsucra::Pon,
            ctsucra::Pon,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TSCAP Pin Enable"]
    #[inline(always)]
    pub fn csw(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ctsucra::Csw,
        ctsucra::Csw,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ctsucra::Csw,
            ctsucra::Csw,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Power Supply Operating Mode Setting"]
    #[inline(always)]
    pub fn atune0(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ctsucra::Atune0,
        ctsucra::Atune0,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ctsucra::Atune0,
            ctsucra::Atune0,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Current Range Adjustment"]
    #[inline(always)]
    pub fn atune1(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ctsucra::Atune1,
        ctsucra::Atune1,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ctsucra::Atune1,
            ctsucra::Atune1,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Operating Clock Select"]
    #[inline(always)]
    pub fn clk(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        ctsucra::Clk,
        ctsucra::Clk,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            ctsucra::Clk,
            ctsucra::Clk,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Measurement Mode Select 0"]
    #[inline(always)]
    pub fn md0(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ctsucra::Md0,
        ctsucra::Md0,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ctsucra::Md0,
            ctsucra::Md0,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Measurement Mode Select 1"]
    #[inline(always)]
    pub fn md1(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ctsucra::Md1,
        ctsucra::Md1,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ctsucra::Md1,
            ctsucra::Md1,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Current Range Adjustment"]
    #[inline(always)]
    pub fn atune2(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ctsucra::Atune2,
        ctsucra::Atune2,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ctsucra::Atune2,
            ctsucra::Atune2,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Load Control During Measurement"]
    #[inline(always)]
    pub fn load(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3,
        1,
        0,
        ctsucra::Load,
        ctsucra::Load,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3,
            1,
            0,
            ctsucra::Load,
            ctsucra::Load,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Non-Measured Channel Output Select"]
    #[inline(always)]
    pub fn posel(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        ctsucra::Posel,
        ctsucra::Posel,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            ctsucra::Posel,
            ctsucra::Posel,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Sensor Drive Pulse Select"]
    #[inline(always)]
    pub fn sdpsel(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        ctsucra::Sdpsel,
        ctsucra::Sdpsel,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            ctsucra::Sdpsel,
            ctsucra::Sdpsel,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Boost Circuit Clock Select"]
    #[inline(always)]
    pub fn pcsel(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        ctsucra::Pcsel,
        ctsucra::Pcsel,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            ctsucra::Pcsel,
            ctsucra::Pcsel,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU STCLK Select"]
    #[inline(always)]
    pub fn stclk(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, Ctsucra_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3f,1,0,u8,u8,Ctsucra_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CTSU Current Measurement Mode Select"]
    #[inline(always)]
    pub fn dcmode(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        ctsucra::Dcmode,
        ctsucra::Dcmode,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            ctsucra::Dcmode,
            ctsucra::Dcmode,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Current Measurement Feedback Select"]
    #[inline(always)]
    pub fn dcback(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        ctsucra::Dcback,
        ctsucra::Dcback,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            ctsucra::Dcback,
            ctsucra::Dcback,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsucra {
    #[inline(always)]
    fn default() -> Ctsucra {
        <crate::RegValueT<Ctsucra_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsucra {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Strt_SPEC;
    pub type Strt = crate::EnumBitfieldStruct<u8, Strt_SPEC>;
    impl Strt {
        #[doc = "Stop measurement operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start measurement operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cap_SPEC;
    pub type Cap = crate::EnumBitfieldStruct<u8, Cap_SPEC>;
    impl Cap {
        #[doc = "Software trigger"]
        pub const _0: Self = Self::new(0);

        #[doc = "External trigger"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snz_SPEC;
    pub type Snz = crate::EnumBitfieldStruct<u8, Snz_SPEC>;
    impl Snz {
        #[doc = "Disable power-saving function during wait state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable power-saving function during wait state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pumpon_SPEC;
    pub type Pumpon = crate::EnumBitfieldStruct<u8, Pumpon_SPEC>;
    impl Pumpon {
        #[doc = "Boost circuit off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Boost circuit on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txvsel_SPEC;
    pub type Txvsel = crate::EnumBitfieldStruct<u8, Txvsel_SPEC>;
    impl Txvsel {
        #[doc = "Selecting VCC as the power supply for the transmit pins of mutual capacitance method."]
        pub const _00: Self = Self::new(0);

        #[doc = "Selecting VCC as the power supply for the transmit pins of the mutual capacitance method. In addition, noise is reduced during GPIO operation. (Recommended)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Select VCC as the power source for the transmitter pins used as the active shield."]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon_SPEC;
    pub type Pon = crate::EnumBitfieldStruct<u8, Pon_SPEC>;
    impl Pon {
        #[doc = "Power off the CTSU"]
        pub const _0: Self = Self::new(0);

        #[doc = "Power on the CTSU"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csw_SPEC;
    pub type Csw = crate::EnumBitfieldStruct<u8, Csw_SPEC>;
    impl Csw {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Atune0_SPEC;
    pub type Atune0 = crate::EnumBitfieldStruct<u8, Atune0_SPEC>;
    impl Atune0 {
        #[doc = "VCC ≥ 2.4 V: Normal voltage operating mode VCC < 2.4 V: Setting prohibited"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low-voltage operating mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Atune1_SPEC;
    pub type Atune1 = crate::EnumBitfieldStruct<u8, Atune1_SPEC>;
    impl Atune1 {
        #[doc = "80 µA when ATUNE2 = 0 20 µA when ATUNE2 = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "40 µA when ATUNE2 = 0 160 µA when ATUNE2 = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clk_SPEC;
    pub type Clk = crate::EnumBitfieldStruct<u8, Clk_SPEC>;
    impl Clk {
        #[doc = "PCLKB"]
        pub const _00: Self = Self::new(0);

        #[doc = "PCLKB/2 (PCLKB divided by 2)"]
        pub const _01: Self = Self::new(1);

        #[doc = "PCLKB/4 (PCLKB divided by 4)"]
        pub const _10: Self = Self::new(2);

        #[doc = "PCLKB/8 (PCLKB divided by 8)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md0_SPEC;
    pub type Md0 = crate::EnumBitfieldStruct<u8, Md0_SPEC>;
    impl Md0 {
        #[doc = "Single scan mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multi-scan mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md1_SPEC;
    pub type Md1 = crate::EnumBitfieldStruct<u8, Md1_SPEC>;
    impl Md1 {
        #[doc = "One-time measurement (self-capacitance method)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Two times measurement (mutual capacitance method)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Atune2_SPEC;
    pub type Atune2 = crate::EnumBitfieldStruct<u8, Atune2_SPEC>;
    impl Atune2 {
        #[doc = "80 µA when ATUNE1 = 0 40 µA when ATUNE1 = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "20 µA when ATUNE1 = 0 160 µA when ATUNE1 = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Load_SPEC;
    pub type Load = crate::EnumBitfieldStruct<u8, Load_SPEC>;
    impl Load {
        #[doc = "2.5 µA constant current load"]
        pub const _00: Self = Self::new(0);

        #[doc = "No load"]
        pub const _01: Self = Self::new(1);

        #[doc = "20 µA constant current load and overcurrent detector disabled"]
        pub const _10: Self = Self::new(2);

        #[doc = "Resistance load for calibration. To set LOAD\\[1:0\\] bits to resistance load for calibration, set these bits to 01b before they are set to 11b."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posel_SPEC;
    pub type Posel = crate::EnumBitfieldStruct<u8, Posel_SPEC>;
    impl Posel {
        #[doc = "Output low"]
        pub const _00: Self = Self::new(0);

        #[doc = "Hi-Z"]
        pub const _01: Self = Self::new(1);

        #[doc = "Output low (uses power supply selected by TXVSEL\\[1:0\\] bits)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Output a pulse in phase with the transmit channel"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdpsel_SPEC;
    pub type Sdpsel = crate::EnumBitfieldStruct<u8, Sdpsel_SPEC>;
    impl Sdpsel {
        #[doc = "Random pulse"]
        pub const _0: Self = Self::new(0);

        #[doc = "Normal pulse using the sensor unit clock"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcsel_SPEC;
    pub type Pcsel = crate::EnumBitfieldStruct<u8, Pcsel_SPEC>;
    impl Pcsel {
        #[doc = "Sensor drive pulse divided by 2"]
        pub const _0: Self = Self::new(0);

        #[doc = "STCLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcmode_SPEC;
    pub type Dcmode = crate::EnumBitfieldStruct<u8, Dcmode_SPEC>;
    impl Dcmode {
        #[doc = "Electrostatic capacitance measurement mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Current measurement mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcback_SPEC;
    pub type Dcback = crate::EnumBitfieldStruct<u8, Dcback_SPEC>;
    impl Dcback {
        #[doc = "TSCAP pin is selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Measurement pin is selected. It is recommended in the current measurement mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucral_SPEC;
impl crate::sealed::RegSpec for Ctsucral_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Control Register A"]
pub type Ctsucral = crate::RegValueT<Ctsucral_SPEC>;

impl NoBitfieldReg<Ctsucral_SPEC> for Ctsucral {}
impl ::core::default::Default for Ctsucral {
    #[inline(always)]
    fn default() -> Ctsucral {
        <crate::RegValueT<Ctsucral_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucr0_SPEC;
impl crate::sealed::RegSpec for Ctsucr0_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Control Register A"]
pub type Ctsucr0 = crate::RegValueT<Ctsucr0_SPEC>;

impl NoBitfieldReg<Ctsucr0_SPEC> for Ctsucr0 {}
impl ::core::default::Default for Ctsucr0 {
    #[inline(always)]
    fn default() -> Ctsucr0 {
        <crate::RegValueT<Ctsucr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucr1_SPEC;
impl crate::sealed::RegSpec for Ctsucr1_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Control Register A"]
pub type Ctsucr1 = crate::RegValueT<Ctsucr1_SPEC>;

impl NoBitfieldReg<Ctsucr1_SPEC> for Ctsucr1 {}
impl ::core::default::Default for Ctsucr1 {
    #[inline(always)]
    fn default() -> Ctsucr1 {
        <crate::RegValueT<Ctsucr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucr2_SPEC;
impl crate::sealed::RegSpec for Ctsucr2_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Control Register A"]
pub type Ctsucr2 = crate::RegValueT<Ctsucr2_SPEC>;

impl NoBitfieldReg<Ctsucr2_SPEC> for Ctsucr2 {}
impl ::core::default::Default for Ctsucr2 {
    #[inline(always)]
    fn default() -> Ctsucr2 {
        <crate::RegValueT<Ctsucr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucr3_SPEC;
impl crate::sealed::RegSpec for Ctsucr3_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Control Register A"]
pub type Ctsucr3 = crate::RegValueT<Ctsucr3_SPEC>;

impl NoBitfieldReg<Ctsucr3_SPEC> for Ctsucr3 {}
impl ::core::default::Default for Ctsucr3 {
    #[inline(always)]
    fn default() -> Ctsucr3 {
        <crate::RegValueT<Ctsucr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucrb_SPEC;
impl crate::sealed::RegSpec for Ctsucrb_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Control Register B"]
pub type Ctsucrb = crate::RegValueT<Ctsucrb_SPEC>;

impl Ctsucrb {
    #[doc = "Phase Shift Frequency Setting"]
    #[inline(always)]
    pub fn prratio(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ctsucrb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ctsucrb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pseudo-Random Number Generation Cycle Setting"]
    #[inline(always)]
    pub fn prmode(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ctsucrb::Prmode,
        ctsucrb::Prmode,
        Ctsucrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ctsucrb::Prmode,
            ctsucrb::Prmode,
            Ctsucrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Jitter Injection Off Setting"]
    #[inline(always)]
    pub fn soff(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ctsucrb::Soff,
        ctsucrb::Soff,
        Ctsucrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ctsucrb::Soff,
            ctsucrb::Soff,
            Ctsucrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pseudo-Random Number Off"]
    #[inline(always)]
    pub fn proff(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ctsucrb::Proff,
        ctsucrb::Proff,
        Ctsucrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ctsucrb::Proff,
            ctsucrb::Proff,
            Ctsucrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sensor Stabilization Wait Time Setting"]
    #[inline(always)]
    pub fn sst(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsucrb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsucrb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SUCLK Spread-Spectrum Mode Select"]
    #[inline(always)]
    pub fn ssmod(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        ctsucrb::Ssmod,
        ctsucrb::Ssmod,
        Ctsucrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            ctsucrb::Ssmod,
            ctsucrb::Ssmod,
            Ctsucrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SUCLK Spread-Spectrum Control"]
    #[inline(always)]
    pub fn sscnt(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        ctsucrb::Sscnt,
        ctsucrb::Sscnt,
        Ctsucrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            ctsucrb::Sscnt,
            ctsucrb::Sscnt,
            Ctsucrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsucrb {
    #[inline(always)]
    fn default() -> Ctsucrb {
        <crate::RegValueT<Ctsucrb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsucrb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prmode_SPEC;
    pub type Prmode = crate::EnumBitfieldStruct<u8, Prmode_SPEC>;
    impl Prmode {
        #[doc = "255 cycles"]
        pub const _00: Self = Self::new(0);

        #[doc = "63 cycles"]
        pub const _01: Self = Self::new(1);

        #[doc = "31 cycles"]
        pub const _10: Self = Self::new(2);

        #[doc = "3 cycles"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Soff_SPEC;
    pub type Soff = crate::EnumBitfieldStruct<u8, Soff_SPEC>;
    impl Soff {
        #[doc = "Inject jitter"]
        pub const _0: Self = Self::new(0);

        #[doc = "Do not inject jitter"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Proff_SPEC;
    pub type Proff = crate::EnumBitfieldStruct<u8, Proff_SPEC>;
    impl Proff {
        #[doc = "Perform pseudo-random number control"]
        pub const _0: Self = Self::new(0);

        #[doc = "Do not perform pseudo-random number control"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssmod_SPEC;
    pub type Ssmod = crate::EnumBitfieldStruct<u8, Ssmod_SPEC>;
    impl Ssmod {
        #[doc = "256 cycles"]
        pub const _000: Self = Self::new(0);

        #[doc = "384 cycles"]
        pub const _001: Self = Self::new(1);

        #[doc = "512 cycles"]
        pub const _010: Self = Self::new(2);

        #[doc = "1024 cycles"]
        pub const _011: Self = Self::new(3);

        #[doc = "No spread-spectrum"]
        pub const _111: Self = Self::new(7);

        #[doc = "Settings are prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscnt_SPEC;
    pub type Sscnt = crate::EnumBitfieldStruct<u8, Sscnt_SPEC>;
    impl Sscnt {
        #[doc = "CTSUTRIMA.SUADJD\\[7:0\\] + 0x00"]
        pub const _00: Self = Self::new(0);

        #[doc = "CTSUTRIMA.SUADJD\\[7:0\\] + 0x20"]
        pub const _01: Self = Self::new(1);

        #[doc = "CTSUTRIMA.SUADJD\\[7:0\\] + 0x40"]
        pub const _10: Self = Self::new(2);

        #[doc = "CTSUTRIMA.SUADJD\\[7:0\\] + 0x60"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucrbl_SPEC;
impl crate::sealed::RegSpec for Ctsucrbl_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Control Register B"]
pub type Ctsucrbl = crate::RegValueT<Ctsucrbl_SPEC>;

impl NoBitfieldReg<Ctsucrbl_SPEC> for Ctsucrbl {}
impl ::core::default::Default for Ctsucrbl {
    #[inline(always)]
    fn default() -> Ctsucrbl {
        <crate::RegValueT<Ctsucrbl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusdprs_SPEC;
impl crate::sealed::RegSpec for Ctsusdprs_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Control Register B"]
pub type Ctsusdprs = crate::RegValueT<Ctsusdprs_SPEC>;

impl NoBitfieldReg<Ctsusdprs_SPEC> for Ctsusdprs {}
impl ::core::default::Default for Ctsusdprs {
    #[inline(always)]
    fn default() -> Ctsusdprs {
        <crate::RegValueT<Ctsusdprs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusst_SPEC;
impl crate::sealed::RegSpec for Ctsusst_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Control Register B"]
pub type Ctsusst = crate::RegValueT<Ctsusst_SPEC>;

impl NoBitfieldReg<Ctsusst_SPEC> for Ctsusst {}
impl ::core::default::Default for Ctsusst {
    #[inline(always)]
    fn default() -> Ctsusst {
        <crate::RegValueT<Ctsusst_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucrbh_SPEC;
impl crate::sealed::RegSpec for Ctsucrbh_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Control Register B"]
pub type Ctsucrbh = crate::RegValueT<Ctsucrbh_SPEC>;

impl NoBitfieldReg<Ctsucrbh_SPEC> for Ctsucrbh {}
impl ::core::default::Default for Ctsucrbh {
    #[inline(always)]
    fn default() -> Ctsucrbh {
        <crate::RegValueT<Ctsucrbh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsudclkc_SPEC;
impl crate::sealed::RegSpec for Ctsudclkc_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Control Register B"]
pub type Ctsudclkc = crate::RegValueT<Ctsudclkc_SPEC>;

impl NoBitfieldReg<Ctsudclkc_SPEC> for Ctsudclkc {}
impl ::core::default::Default for Ctsudclkc {
    #[inline(always)]
    fn default() -> Ctsudclkc {
        <crate::RegValueT<Ctsudclkc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumch_SPEC;
impl crate::sealed::RegSpec for Ctsumch_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Measurement Channel Register"]
pub type Ctsumch = crate::RegValueT<Ctsumch_SPEC>;

impl Ctsumch {
    #[doc = "CTSU Measurement Channel 0"]
    #[inline(always)]
    pub fn mch0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        ctsumch::Mch0,
        ctsumch::Mch0,
        Ctsumch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            ctsumch::Mch0,
            ctsumch::Mch0,
            Ctsumch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Measurement Channel 1"]
    #[inline(always)]
    pub fn mch1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3f,
        1,
        0,
        ctsumch::Mch1,
        ctsumch::Mch1,
        Ctsumch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            ctsumch::Mch1,
            ctsumch::Mch1,
            Ctsumch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiple Clocks Control"]
    #[inline(always)]
    pub fn mca0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ctsumch::Mca0,
        ctsumch::Mca0,
        Ctsumch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ctsumch::Mca0,
            ctsumch::Mca0,
            Ctsumch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiple Clocks Control"]
    #[inline(always)]
    pub fn mca1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ctsumch::Mca1,
        ctsumch::Mca1,
        Ctsumch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ctsumch::Mca1,
            ctsumch::Mca1,
            Ctsumch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiple Clocks Control"]
    #[inline(always)]
    pub fn mca2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        ctsumch::Mca2,
        ctsumch::Mca2,
        Ctsumch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            ctsumch::Mca2,
            ctsumch::Mca2,
            Ctsumch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiple Clocks Control"]
    #[inline(always)]
    pub fn mca3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        ctsumch::Mca3,
        ctsumch::Mca3,
        Ctsumch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            ctsumch::Mca3,
            ctsumch::Mca3,
            Ctsumch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsumch {
    #[inline(always)]
    fn default() -> Ctsumch {
        <crate::RegValueT<Ctsumch_SPEC> as RegisterValue<_>>::new(16191)
    }
}
pub mod ctsumch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mch0_SPEC;
    pub type Mch0 = crate::EnumBitfieldStruct<u8, Mch0_SPEC>;
    impl Mch0 {
        #[doc = "TS00"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "TS01"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "TS02"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "TS03"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "TS04"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "TS05"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "TS06"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "TS07"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "TS08"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "TS09"]
        pub const _0_X_09: Self = Self::new(9);

        #[doc = "TS10"]
        pub const _0_X_0_A: Self = Self::new(10);

        #[doc = "TS11"]
        pub const _0_X_0_B: Self = Self::new(11);

        #[doc = "Measurement is being stopped."]
        pub const _0_X_3_F: Self = Self::new(63);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mch1_SPEC;
    pub type Mch1 = crate::EnumBitfieldStruct<u8, Mch1_SPEC>;
    impl Mch1 {
        #[doc = "TS00"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "TS01"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "TS02"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "TS03"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "TS04"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "TS05"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "TS06"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "TS07"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "TS08"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "TS09"]
        pub const _0_X_09: Self = Self::new(9);

        #[doc = "TS10"]
        pub const _0_X_0_A: Self = Self::new(10);

        #[doc = "TS11"]
        pub const _0_X_0_B: Self = Self::new(11);

        #[doc = "Measurement is being stopped."]
        pub const _0_X_3_F: Self = Self::new(63);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mca0_SPEC;
    pub type Mca0 = crate::EnumBitfieldStruct<u8, Mca0_SPEC>;
    impl Mca0 {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mca1_SPEC;
    pub type Mca1 = crate::EnumBitfieldStruct<u8, Mca1_SPEC>;
    impl Mca1 {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mca2_SPEC;
    pub type Mca2 = crate::EnumBitfieldStruct<u8, Mca2_SPEC>;
    impl Mca2 {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mca3_SPEC;
    pub type Mca3 = crate::EnumBitfieldStruct<u8, Mca3_SPEC>;
    impl Mca3 {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumchl_SPEC;
impl crate::sealed::RegSpec for Ctsumchl_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Measurement Channel Register"]
pub type Ctsumchl = crate::RegValueT<Ctsumchl_SPEC>;

impl NoBitfieldReg<Ctsumchl_SPEC> for Ctsumchl {}
impl ::core::default::Default for Ctsumchl {
    #[inline(always)]
    fn default() -> Ctsumchl {
        <crate::RegValueT<Ctsumchl_SPEC> as RegisterValue<_>>::new(16191)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumch0_SPEC;
impl crate::sealed::RegSpec for Ctsumch0_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Measurement Channel Register"]
pub type Ctsumch0 = crate::RegValueT<Ctsumch0_SPEC>;

impl NoBitfieldReg<Ctsumch0_SPEC> for Ctsumch0 {}
impl ::core::default::Default for Ctsumch0 {
    #[inline(always)]
    fn default() -> Ctsumch0 {
        <crate::RegValueT<Ctsumch0_SPEC> as RegisterValue<_>>::new(63)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumch1_SPEC;
impl crate::sealed::RegSpec for Ctsumch1_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Measurement Channel Register"]
pub type Ctsumch1 = crate::RegValueT<Ctsumch1_SPEC>;

impl NoBitfieldReg<Ctsumch1_SPEC> for Ctsumch1 {}
impl ::core::default::Default for Ctsumch1 {
    #[inline(always)]
    fn default() -> Ctsumch1 {
        <crate::RegValueT<Ctsumch1_SPEC> as RegisterValue<_>>::new(63)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumchh_SPEC;
impl crate::sealed::RegSpec for Ctsumchh_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Measurement Channel Register"]
pub type Ctsumchh = crate::RegValueT<Ctsumchh_SPEC>;

impl NoBitfieldReg<Ctsumchh_SPEC> for Ctsumchh {}
impl ::core::default::Default for Ctsumchh {
    #[inline(always)]
    fn default() -> Ctsumchh {
        <crate::RegValueT<Ctsumchh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumfaf_SPEC;
impl crate::sealed::RegSpec for Ctsumfaf_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Measurement Channel Register"]
pub type Ctsumfaf = crate::RegValueT<Ctsumfaf_SPEC>;

impl NoBitfieldReg<Ctsumfaf_SPEC> for Ctsumfaf {}
impl ::core::default::Default for Ctsumfaf {
    #[inline(always)]
    fn default() -> Ctsumfaf {
        <crate::RegValueT<Ctsumfaf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchaca_SPEC;
impl crate::sealed::RegSpec for Ctsuchaca_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Channel Enable Control Register A"]
pub type Ctsuchaca = crate::RegValueT<Ctsuchaca_SPEC>;

impl Ctsuchaca {
    #[doc = "CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuchaca::Chac00,
        ctsuchaca::Chac00,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuchaca::Chac00,
            ctsuchaca::Chac00,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuchaca::Chac01,
        ctsuchaca::Chac01,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuchaca::Chac01,
            ctsuchaca::Chac01,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuchaca::Chac02,
        ctsuchaca::Chac02,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuchaca::Chac02,
            ctsuchaca::Chac02,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsuchaca::Chac03,
        ctsuchaca::Chac03,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsuchaca::Chac03,
            ctsuchaca::Chac03,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsuchaca::Chac04,
        ctsuchaca::Chac04,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsuchaca::Chac04,
            ctsuchaca::Chac04,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ctsuchaca::Chac05,
        ctsuchaca::Chac05,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ctsuchaca::Chac05,
            ctsuchaca::Chac05,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ctsuchaca::Chac06,
        ctsuchaca::Chac06,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ctsuchaca::Chac06,
            ctsuchaca::Chac06,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ctsuchaca::Chac07,
        ctsuchaca::Chac07,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ctsuchaca::Chac07,
            ctsuchaca::Chac07,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ctsuchaca::Chac08,
        ctsuchaca::Chac08,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ctsuchaca::Chac08,
            ctsuchaca::Chac08,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ctsuchaca::Chac09,
        ctsuchaca::Chac09,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ctsuchaca::Chac09,
            ctsuchaca::Chac09,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ctsuchaca::Chac10,
        ctsuchaca::Chac10,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ctsuchaca::Chac10,
            ctsuchaca::Chac10,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Enable Control A"]
    #[inline(always)]
    pub fn chac11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ctsuchaca::Chac11,
        ctsuchaca::Chac11,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ctsuchaca::Chac11,
            ctsuchaca::Chac11,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuchaca {
    #[inline(always)]
    fn default() -> Ctsuchaca {
        <crate::RegValueT<Ctsuchaca_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuchaca {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac00_SPEC;
    pub type Chac00 = crate::EnumBitfieldStruct<u8, Chac00_SPEC>;
    impl Chac00 {
        #[doc = "Do not measure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Measure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac01_SPEC;
    pub type Chac01 = crate::EnumBitfieldStruct<u8, Chac01_SPEC>;
    impl Chac01 {
        #[doc = "Do not measure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Measure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac02_SPEC;
    pub type Chac02 = crate::EnumBitfieldStruct<u8, Chac02_SPEC>;
    impl Chac02 {
        #[doc = "Do not measure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Measure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac03_SPEC;
    pub type Chac03 = crate::EnumBitfieldStruct<u8, Chac03_SPEC>;
    impl Chac03 {
        #[doc = "Do not measure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Measure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac04_SPEC;
    pub type Chac04 = crate::EnumBitfieldStruct<u8, Chac04_SPEC>;
    impl Chac04 {
        #[doc = "Do not measure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Measure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac05_SPEC;
    pub type Chac05 = crate::EnumBitfieldStruct<u8, Chac05_SPEC>;
    impl Chac05 {
        #[doc = "Do not measure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Measure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac06_SPEC;
    pub type Chac06 = crate::EnumBitfieldStruct<u8, Chac06_SPEC>;
    impl Chac06 {
        #[doc = "Do not measure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Measure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac07_SPEC;
    pub type Chac07 = crate::EnumBitfieldStruct<u8, Chac07_SPEC>;
    impl Chac07 {
        #[doc = "Do not measure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Measure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac08_SPEC;
    pub type Chac08 = crate::EnumBitfieldStruct<u8, Chac08_SPEC>;
    impl Chac08 {
        #[doc = "Do not measure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Measure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac09_SPEC;
    pub type Chac09 = crate::EnumBitfieldStruct<u8, Chac09_SPEC>;
    impl Chac09 {
        #[doc = "Do not measure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Measure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac10_SPEC;
    pub type Chac10 = crate::EnumBitfieldStruct<u8, Chac10_SPEC>;
    impl Chac10 {
        #[doc = "Do not measure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Measure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac11_SPEC;
    pub type Chac11 = crate::EnumBitfieldStruct<u8, Chac11_SPEC>;
    impl Chac11 {
        #[doc = "Do not measure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Measure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchacal_SPEC;
impl crate::sealed::RegSpec for Ctsuchacal_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Channel Enable Control Register A"]
pub type Ctsuchacal = crate::RegValueT<Ctsuchacal_SPEC>;

impl NoBitfieldReg<Ctsuchacal_SPEC> for Ctsuchacal {}
impl ::core::default::Default for Ctsuchacal {
    #[inline(always)]
    fn default() -> Ctsuchacal {
        <crate::RegValueT<Ctsuchacal_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac0_SPEC;
impl crate::sealed::RegSpec for Ctsuchac0_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Channel Enable Control Register A"]
pub type Ctsuchac0 = crate::RegValueT<Ctsuchac0_SPEC>;

impl NoBitfieldReg<Ctsuchac0_SPEC> for Ctsuchac0 {}
impl ::core::default::Default for Ctsuchac0 {
    #[inline(always)]
    fn default() -> Ctsuchac0 {
        <crate::RegValueT<Ctsuchac0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac1_SPEC;
impl crate::sealed::RegSpec for Ctsuchac1_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Channel Enable Control Register A"]
pub type Ctsuchac1 = crate::RegValueT<Ctsuchac1_SPEC>;

impl NoBitfieldReg<Ctsuchac1_SPEC> for Ctsuchac1 {}
impl ::core::default::Default for Ctsuchac1 {
    #[inline(always)]
    fn default() -> Ctsuchac1 {
        <crate::RegValueT<Ctsuchac1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrca_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrca_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Channel Transmit/Receive Control Register A"]
pub type Ctsuchtrca = crate::RegValueT<Ctsuchtrca_SPEC>;

impl Ctsuchtrca {
    #[doc = "CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc00,
        ctsuchtrca::Chtrc00,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc00,
            ctsuchtrca::Chtrc00,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc01,
        ctsuchtrca::Chtrc01,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc01,
            ctsuchtrca::Chtrc01,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc02,
        ctsuchtrca::Chtrc02,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc02,
            ctsuchtrca::Chtrc02,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc03,
        ctsuchtrca::Chtrc03,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc03,
            ctsuchtrca::Chtrc03,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc04,
        ctsuchtrca::Chtrc04,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc04,
            ctsuchtrca::Chtrc04,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc05,
        ctsuchtrca::Chtrc05,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc05,
            ctsuchtrca::Chtrc05,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc06,
        ctsuchtrca::Chtrc06,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc06,
            ctsuchtrca::Chtrc06,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc07,
        ctsuchtrca::Chtrc07,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc07,
            ctsuchtrca::Chtrc07,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc08,
        ctsuchtrca::Chtrc08,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc08,
            ctsuchtrca::Chtrc08,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc09,
        ctsuchtrca::Chtrc09,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc09,
            ctsuchtrca::Chtrc09,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc10,
        ctsuchtrca::Chtrc10,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc10,
            ctsuchtrca::Chtrc10,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Channel Transmit/Receive Control A"]
    #[inline(always)]
    pub fn chtrc11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc11,
        ctsuchtrca::Chtrc11,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc11,
            ctsuchtrca::Chtrc11,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuchtrca {
    #[inline(always)]
    fn default() -> Ctsuchtrca {
        <crate::RegValueT<Ctsuchtrca_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuchtrca {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc00_SPEC;
    pub type Chtrc00 = crate::EnumBitfieldStruct<u8, Chtrc00_SPEC>;
    impl Chtrc00 {
        #[doc = "Reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc01_SPEC;
    pub type Chtrc01 = crate::EnumBitfieldStruct<u8, Chtrc01_SPEC>;
    impl Chtrc01 {
        #[doc = "Reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc02_SPEC;
    pub type Chtrc02 = crate::EnumBitfieldStruct<u8, Chtrc02_SPEC>;
    impl Chtrc02 {
        #[doc = "Reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc03_SPEC;
    pub type Chtrc03 = crate::EnumBitfieldStruct<u8, Chtrc03_SPEC>;
    impl Chtrc03 {
        #[doc = "Reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc04_SPEC;
    pub type Chtrc04 = crate::EnumBitfieldStruct<u8, Chtrc04_SPEC>;
    impl Chtrc04 {
        #[doc = "Reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc05_SPEC;
    pub type Chtrc05 = crate::EnumBitfieldStruct<u8, Chtrc05_SPEC>;
    impl Chtrc05 {
        #[doc = "Reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc06_SPEC;
    pub type Chtrc06 = crate::EnumBitfieldStruct<u8, Chtrc06_SPEC>;
    impl Chtrc06 {
        #[doc = "Reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc07_SPEC;
    pub type Chtrc07 = crate::EnumBitfieldStruct<u8, Chtrc07_SPEC>;
    impl Chtrc07 {
        #[doc = "Reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc08_SPEC;
    pub type Chtrc08 = crate::EnumBitfieldStruct<u8, Chtrc08_SPEC>;
    impl Chtrc08 {
        #[doc = "Reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc09_SPEC;
    pub type Chtrc09 = crate::EnumBitfieldStruct<u8, Chtrc09_SPEC>;
    impl Chtrc09 {
        #[doc = "Reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc10_SPEC;
    pub type Chtrc10 = crate::EnumBitfieldStruct<u8, Chtrc10_SPEC>;
    impl Chtrc10 {
        #[doc = "Reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc11_SPEC;
    pub type Chtrc11 = crate::EnumBitfieldStruct<u8, Chtrc11_SPEC>;
    impl Chtrc11 {
        #[doc = "Reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrcal_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrcal_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Channel Transmit/Receive Control Register A"]
pub type Ctsuchtrcal = crate::RegValueT<Ctsuchtrcal_SPEC>;

impl NoBitfieldReg<Ctsuchtrcal_SPEC> for Ctsuchtrcal {}
impl ::core::default::Default for Ctsuchtrcal {
    #[inline(always)]
    fn default() -> Ctsuchtrcal {
        <crate::RegValueT<Ctsuchtrcal_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc0_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrc0_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Channel Transmit/Receive Control Register A"]
pub type Ctsuchtrc0 = crate::RegValueT<Ctsuchtrc0_SPEC>;

impl NoBitfieldReg<Ctsuchtrc0_SPEC> for Ctsuchtrc0 {}
impl ::core::default::Default for Ctsuchtrc0 {
    #[inline(always)]
    fn default() -> Ctsuchtrc0 {
        <crate::RegValueT<Ctsuchtrc0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc1_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrc1_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Channel Transmit/Receive Control Register A"]
pub type Ctsuchtrc1 = crate::RegValueT<Ctsuchtrc1_SPEC>;

impl NoBitfieldReg<Ctsuchtrc1_SPEC> for Ctsuchtrc1 {}
impl ::core::default::Default for Ctsuchtrc1 {
    #[inline(always)]
    fn default() -> Ctsuchtrc1 {
        <crate::RegValueT<Ctsuchtrc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusr_SPEC;
impl crate::sealed::RegSpec for Ctsusr_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Status Register"]
pub type Ctsusr = crate::RegValueT<Ctsusr_SPEC>;

impl Ctsusr {
    #[doc = "CTSU Multi-Clock Counter"]
    #[inline(always)]
    pub fn mfc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        ctsusr::Mfc,
        ctsusr::Mfc,
        Ctsusr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            ctsusr::Mfc,
            ctsusr::Mfc,
            Ctsusr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU ICOMP0 and ICOMP1 Flags Reset"]
    #[inline(always)]
    pub fn icomprst(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ctsusr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ctsusr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "CTSU Sense Current Error Monitor"]
    #[inline(always)]
    pub fn icomp1(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ctsusr::Icomp1,
        ctsusr::Icomp1,
        Ctsusr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ctsusr::Icomp1,
            ctsusr::Icomp1,
            Ctsusr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Over-Voltage Detection Flag"]
    #[inline(always)]
    pub fn icomp0(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ctsusr::Icomp0,
        ctsusr::Icomp0,
        Ctsusr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ctsusr::Icomp0,
            ctsusr::Icomp0,
            Ctsusr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Measurement Status Counter"]
    #[inline(always)]
    pub fn stc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        ctsusr::Stc,
        ctsusr::Stc,
        Ctsusr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            ctsusr::Stc,
            ctsusr::Stc,
            Ctsusr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Data Transfer Status Flag"]
    #[inline(always)]
    pub fn dtsr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ctsusr::Dtsr,
        ctsusr::Dtsr,
        Ctsusr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ctsusr::Dtsr,
            ctsusr::Dtsr,
            Ctsusr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Sensor Counter Overflow Flag"]
    #[inline(always)]
    pub fn sensovf(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ctsusr::Sensovf,
        ctsusr::Sensovf,
        Ctsusr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ctsusr::Sensovf,
            ctsusr::Sensovf,
            Ctsusr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU SUCLK Counter Overflow Flag"]
    #[inline(always)]
    pub fn suckovf(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ctsusr::Suckovf,
        ctsusr::Suckovf,
        Ctsusr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ctsusr::Suckovf,
            ctsusr::Suckovf,
            Ctsusr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTSU Mutual Capacitance Status Flag"]
    #[inline(always)]
    pub fn ps(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ctsusr::Ps,
        ctsusr::Ps,
        Ctsusr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ctsusr::Ps,
            ctsusr::Ps,
            Ctsusr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsusr {
    #[inline(always)]
    fn default() -> Ctsusr {
        <crate::RegValueT<Ctsusr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsusr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mfc_SPEC;
    pub type Mfc = crate::EnumBitfieldStruct<u8, Mfc_SPEC>;
    impl Mfc {
        #[doc = "Multi-clock 0"]
        pub const _00: Self = Self::new(0);

        #[doc = "Multi-clock 1"]
        pub const _01: Self = Self::new(1);

        #[doc = "Multi-clock 2"]
        pub const _10: Self = Self::new(2);

        #[doc = "Multi-clock 3"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icomp1_SPEC;
    pub type Icomp1 = crate::EnumBitfieldStruct<u8, Icomp1_SPEC>;
    impl Icomp1 {
        #[doc = "Normal sensor current"]
        pub const _0: Self = Self::new(0);

        #[doc = "Abnormal sensor current"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icomp0_SPEC;
    pub type Icomp0 = crate::EnumBitfieldStruct<u8, Icomp0_SPEC>;
    impl Icomp0 {
        #[doc = "Normal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Over-voltage is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stc_SPEC;
    pub type Stc = crate::EnumBitfieldStruct<u8, Stc_SPEC>;
    impl Stc {
        #[doc = "Status 0"]
        pub const _000: Self = Self::new(0);

        #[doc = "Status 1"]
        pub const _001: Self = Self::new(1);

        #[doc = "Status 2"]
        pub const _010: Self = Self::new(2);

        #[doc = "Status 3"]
        pub const _011: Self = Self::new(3);

        #[doc = "Status 4"]
        pub const _100: Self = Self::new(4);

        #[doc = "Status 5"]
        pub const _101: Self = Self::new(5);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtsr_SPEC;
    pub type Dtsr = crate::EnumBitfieldStruct<u8, Dtsr_SPEC>;
    impl Dtsr {
        #[doc = "Read"]
        pub const _0: Self = Self::new(0);

        #[doc = "Not read"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sensovf_SPEC;
    pub type Sensovf = crate::EnumBitfieldStruct<u8, Sensovf_SPEC>;
    impl Sensovf {
        #[doc = "No overflow occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Overflow occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Suckovf_SPEC;
    pub type Suckovf = crate::EnumBitfieldStruct<u8, Suckovf_SPEC>;
    impl Suckovf {
        #[doc = "No overflow occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Overflow occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ps_SPEC;
    pub type Ps = crate::EnumBitfieldStruct<u8, Ps_SPEC>;
    impl Ps {
        #[doc = "First measurement"]
        pub const _0: Self = Self::new(0);

        #[doc = "Second measurement"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusrl_SPEC;
impl crate::sealed::RegSpec for Ctsusrl_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Status Register"]
pub type Ctsusrl = crate::RegValueT<Ctsusrl_SPEC>;

impl NoBitfieldReg<Ctsusrl_SPEC> for Ctsusrl {}
impl ::core::default::Default for Ctsusrl {
    #[inline(always)]
    fn default() -> Ctsusrl {
        <crate::RegValueT<Ctsusrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusr0_SPEC;
impl crate::sealed::RegSpec for Ctsusr0_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Status Register"]
pub type Ctsusr0 = crate::RegValueT<Ctsusr0_SPEC>;

impl NoBitfieldReg<Ctsusr0_SPEC> for Ctsusr0 {}
impl ::core::default::Default for Ctsusr0 {
    #[inline(always)]
    fn default() -> Ctsusr0 {
        <crate::RegValueT<Ctsusr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsust_SPEC;
impl crate::sealed::RegSpec for Ctsust_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Status Register"]
pub type Ctsust = crate::RegValueT<Ctsust_SPEC>;

impl NoBitfieldReg<Ctsust_SPEC> for Ctsust {}
impl ::core::default::Default for Ctsust {
    #[inline(always)]
    fn default() -> Ctsust {
        <crate::RegValueT<Ctsust_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusrh_SPEC;
impl crate::sealed::RegSpec for Ctsusrh_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Status Register"]
pub type Ctsusrh = crate::RegValueT<Ctsusrh_SPEC>;

impl NoBitfieldReg<Ctsusrh_SPEC> for Ctsusrh {}
impl ::core::default::Default for Ctsusrh {
    #[inline(always)]
    fn default() -> Ctsusrh {
        <crate::RegValueT<Ctsusrh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusr2_SPEC;
impl crate::sealed::RegSpec for Ctsusr2_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Status Register"]
pub type Ctsusr2 = crate::RegValueT<Ctsusr2_SPEC>;

impl NoBitfieldReg<Ctsusr2_SPEC> for Ctsusr2 {}
impl ::core::default::Default for Ctsusr2 {
    #[inline(always)]
    fn default() -> Ctsusr2 {
        <crate::RegValueT<Ctsusr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuso_SPEC;
impl crate::sealed::RegSpec for Ctsuso_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Sensor Offset Register"]
pub type Ctsuso = crate::RegValueT<Ctsuso_SPEC>;

impl Ctsuso {
    #[doc = "CTSU Sensor Offset Adjustment"]
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsuso_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsuso_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CTSU Measurement Count Setting"]
    #[inline(always)]
    pub fn snum(
        self,
    ) -> crate::common::RegisterField<10, 0xff, 1, 0, u8, u8, Ctsuso_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0xff,1,0,u8,u8,Ctsuso_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Spread Spectrum Frequency"]
    #[inline(always)]
    pub fn ssdiv(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Ctsuso_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Ctsuso_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CTSU Base Clock Setting"]
    #[inline(always)]
    pub fn sdpa(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ctsuso_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ctsuso_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuso {
    #[inline(always)]
    fn default() -> Ctsuso {
        <crate::RegValueT<Ctsuso_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuso0_SPEC;
impl crate::sealed::RegSpec for Ctsuso0_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Sensor Offset Register"]
pub type Ctsuso0 = crate::RegValueT<Ctsuso0_SPEC>;

impl NoBitfieldReg<Ctsuso0_SPEC> for Ctsuso0 {}
impl ::core::default::Default for Ctsuso0 {
    #[inline(always)]
    fn default() -> Ctsuso0 {
        <crate::RegValueT<Ctsuso0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuso1_SPEC;
impl crate::sealed::RegSpec for Ctsuso1_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Sensor Offset Register"]
pub type Ctsuso1 = crate::RegValueT<Ctsuso1_SPEC>;

impl NoBitfieldReg<Ctsuso1_SPEC> for Ctsuso1 {}
impl ::core::default::Default for Ctsuso1 {
    #[inline(always)]
    fn default() -> Ctsuso1 {
        <crate::RegValueT<Ctsuso1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuscnt_SPEC;
impl crate::sealed::RegSpec for Ctsuscnt_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Sensor Counter Register"]
pub type Ctsuscnt = crate::RegValueT<Ctsuscnt_SPEC>;

impl Ctsuscnt {
    #[doc = "CTSU Sensor Counter"]
    #[inline(always)]
    pub fn senscnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuscnt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuscnt_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "CTSU SUCLK Counter"]
    #[inline(always)]
    pub fn suckcnt(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ctsuscnt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ctsuscnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuscnt {
    #[inline(always)]
    fn default() -> Ctsuscnt {
        <crate::RegValueT<Ctsuscnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusc_SPEC;
impl crate::sealed::RegSpec for Ctsusc_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Sensor Counter Register"]
pub type Ctsusc = crate::RegValueT<Ctsusc_SPEC>;

impl NoBitfieldReg<Ctsusc_SPEC> for Ctsusc {}
impl ::core::default::Default for Ctsusc {
    #[inline(always)]
    fn default() -> Ctsusc {
        <crate::RegValueT<Ctsusc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucalib_SPEC;
impl crate::sealed::RegSpec for Ctsucalib_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Calibration Register"]
pub type Ctsucalib = crate::RegValueT<Ctsucalib_SPEC>;

impl Ctsucalib {
    #[doc = "All TS Pin Output Control"]
    #[inline(always)]
    pub fn tsod(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsucalib::Tsod,
        ctsucalib::Tsod,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsucalib::Tsod,
            ctsucalib::Tsod,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Power Supply Calibration Select"]
    #[inline(always)]
    pub fn drv(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsucalib::Drv,
        ctsucalib::Drv,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsucalib::Drv,
            ctsucalib::Drv,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Observation Clock Select"]
    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ctsucalib::Clksel,
        ctsucalib::Clksel,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ctsucalib::Clksel,
            ctsucalib::Clksel,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SUCLK Forced Oscillation Control"]
    #[inline(always)]
    pub fn suclken(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ctsucalib::Suclken,
        ctsucalib::Suclken,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ctsucalib::Suclken,
            ctsucalib::Suclken,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Switched Capacitor Operation Calibration Select Bit"]
    #[inline(always)]
    pub fn tsoc(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ctsucalib::Tsoc,
        ctsucalib::Tsoc,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ctsucalib::Tsoc,
            ctsucalib::Tsoc,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read Count Select of Sensor Counter"]
    #[inline(always)]
    pub fn cntrdsel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ctsucalib::Cntrdsel,
        ctsucalib::Cntrdsel,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ctsucalib::Cntrdsel,
            ctsucalib::Cntrdsel,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Pin Control"]
    #[inline(always)]
    pub fn ioc(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ctsucalib_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Ctsucalib_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Down Converter Control"]
    #[inline(always)]
    pub fn dcoff(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ctsucalib::Dcoff,
        ctsucalib::Dcoff,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ctsucalib::Dcoff,
            ctsucalib::Dcoff,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TS Pins Fixed Output Select"]
    #[inline(always)]
    pub fn iocsel(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ctsucalib::Iocsel,
        ctsucalib::Iocsel,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ctsucalib::Iocsel,
            ctsucalib::Iocsel,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Current Offset DAC Current Matrix Calibration Select"]
    #[inline(always)]
    pub fn dacmsel(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        ctsucalib::Dacmsel,
        ctsucalib::Dacmsel,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            ctsucalib::Dacmsel,
            ctsucalib::Dacmsel,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset Current Adjustment for Calibration"]
    #[inline(always)]
    pub fn daccarry(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ctsucalib::Daccarry,
        ctsucalib::Daccarry,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ctsucalib::Daccarry,
            ctsucalib::Daccarry,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Current Control Oscillator Input Current Matrix Calibration Select"]
    #[inline(always)]
    pub fn sumsel(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        ctsucalib::Sumsel,
        ctsucalib::Sumsel,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            ctsucalib::Sumsel,
            ctsucalib::Sumsel,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Current Control Oscillator Input Current Adjustment for SUCLK"]
    #[inline(always)]
    pub fn sucarry(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        ctsucalib::Sucarry,
        ctsucalib::Sucarry,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            ctsucalib::Sucarry,
            ctsucalib::Sucarry,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Modulation Clock Select for Offset Current Circuits"]
    #[inline(always)]
    pub fn dacclk(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ctsucalib::Dacclk,
        ctsucalib::Dacclk,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ctsucalib::Dacclk,
            ctsucalib::Dacclk,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Modulation Clock Select for Current Controlled Oscillator Input Current of SUCLK"]
    #[inline(always)]
    pub fn ccoclk(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ctsucalib::Ccoclk,
        ctsucalib::Ccoclk,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ctsucalib::Ccoclk,
            ctsucalib::Ccoclk,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Calibration Selection of Current Controlled Oscillator for Measurement"]
    #[inline(always)]
    pub fn ccocalib(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        ctsucalib::Ccocalib,
        ctsucalib::Ccocalib,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            ctsucalib::Ccocalib,
            ctsucalib::Ccocalib,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Pin Inverted Output"]
    #[inline(always)]
    pub fn txrev(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        ctsucalib::Txrev,
        ctsucalib::Txrev,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            ctsucalib::Txrev,
            ctsucalib::Txrev,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsucalib {
    #[inline(always)]
    fn default() -> Ctsucalib {
        <crate::RegValueT<Ctsucalib_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsucalib {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsod_SPEC;
    pub type Tsod = crate::EnumBitfieldStruct<u8, Tsod_SPEC>;
    impl Tsod {
        #[doc = "Does not output from TS terminals"]
        pub const _0: Self = Self::new(0);

        #[doc = "All TS terminals output (controlled by the IOCSEL bit)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drv_SPEC;
    pub type Drv = crate::EnumBitfieldStruct<u8, Drv_SPEC>;
    impl Drv {
        #[doc = "Capacitance measurement mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Power supply calibration mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clksel_SPEC;
    pub type Clksel = crate::EnumBitfieldStruct<u8, Clksel_SPEC>;
    impl Clksel {
        #[doc = "Not selected (L fixed output)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Measurement clock (divided by 8)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "SUCLK (divided by 8)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Suclken_SPEC;
    pub type Suclken = crate::EnumBitfieldStruct<u8, Suclken_SPEC>;
    impl Suclken {
        #[doc = "SUCLK oscillation only during measurement"]
        pub const _0: Self = Self::new(0);

        #[doc = "SUCLK always oscillates"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsoc_SPEC;
    pub type Tsoc = crate::EnumBitfieldStruct<u8, Tsoc_SPEC>;
    impl Tsoc {
        #[doc = "Capacitance measurement mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Switched capacitor operation calibration mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cntrdsel_SPEC;
    pub type Cntrdsel = crate::EnumBitfieldStruct<u8, Cntrdsel_SPEC>;
    impl Cntrdsel {
        #[doc = "Read once"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read twice"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcoff_SPEC;
    pub type Dcoff = crate::EnumBitfieldStruct<u8, Dcoff_SPEC>;
    impl Dcoff {
        #[doc = "Voltage down converter operation (TSCAP voltage generation)"]
        pub const _0: Self = Self::new(0);

        #[doc = "The voltage down converter is off"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iocsel_SPEC;
    pub type Iocsel = crate::EnumBitfieldStruct<u8, Iocsel_SPEC>;
    impl Iocsel {
        #[doc = "Sensor drive pulse"]
        pub const _0: Self = Self::new(0);

        #[doc = "The level selected by the IOC bit is output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dacmsel_SPEC;
    pub type Dacmsel = crate::EnumBitfieldStruct<u8, Dacmsel_SPEC>;
    impl Dacmsel {
        #[doc = "Capacitance measurement mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Current offset DAC current Calibration mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Daccarry_SPEC;
    pub type Daccarry = crate::EnumBitfieldStruct<u8, Daccarry_SPEC>;
    impl Daccarry {
        #[doc = "Normal operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "All current sources can be turned on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sumsel_SPEC;
    pub type Sumsel = crate::EnumBitfieldStruct<u8, Sumsel_SPEC>;
    impl Sumsel {
        #[doc = "Capacitance measurement mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Current control oscillator input current matrix calibration mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sucarry_SPEC;
    pub type Sucarry = crate::EnumBitfieldStruct<u8, Sucarry_SPEC>;
    impl Sucarry {
        #[doc = "Normal operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "All current sources can be turned on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dacclk_SPEC;
    pub type Dacclk = crate::EnumBitfieldStruct<u8, Dacclk_SPEC>;
    impl Dacclk {
        #[doc = "Operating clock selected by CTSUCRA.CLK \\[1:0\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "SUCLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccoclk_SPEC;
    pub type Ccoclk = crate::EnumBitfieldStruct<u8, Ccoclk_SPEC>;
    impl Ccoclk {
        #[doc = "Operating clock selected by CTSUCRA.CLK \\[1:0\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "SUCLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccocalib_SPEC;
    pub type Ccocalib = crate::EnumBitfieldStruct<u8, Ccocalib_SPEC>;
    impl Ccocalib {
        #[doc = "Capacitance measurement mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Oscillator calibration mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txrev_SPEC;
    pub type Txrev = crate::EnumBitfieldStruct<u8, Txrev_SPEC>;
    impl Txrev {
        #[doc = "Normal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invert"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsudbgr0_SPEC;
impl crate::sealed::RegSpec for Ctsudbgr0_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Calibration Register"]
pub type Ctsudbgr0 = crate::RegValueT<Ctsudbgr0_SPEC>;

impl NoBitfieldReg<Ctsudbgr0_SPEC> for Ctsudbgr0 {}
impl ::core::default::Default for Ctsudbgr0 {
    #[inline(always)]
    fn default() -> Ctsudbgr0 {
        <crate::RegValueT<Ctsudbgr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsudbgr1_SPEC;
impl crate::sealed::RegSpec for Ctsudbgr1_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Calibration Register"]
pub type Ctsudbgr1 = crate::RegValueT<Ctsudbgr1_SPEC>;

impl NoBitfieldReg<Ctsudbgr1_SPEC> for Ctsudbgr1 {}
impl ::core::default::Default for Ctsudbgr1 {
    #[inline(always)]
    fn default() -> Ctsudbgr1 {
        <crate::RegValueT<Ctsudbgr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusuclka_SPEC;
impl crate::sealed::RegSpec for Ctsusuclka_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Sensor Unit Clock Control Register A"]
pub type Ctsusuclka = crate::RegValueT<Ctsusuclka_SPEC>;

impl Ctsusuclka {
    #[doc = "CTSU SUCLK Frequency Adjustment"]
    #[inline(always)]
    pub fn suadj0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsusuclka_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsusuclka_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CTSU SUCLK Multiplier Rate Setting"]
    #[inline(always)]
    pub fn sumulti0(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsusuclka_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsusuclka_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CTSU SUCLK Frequency Adjustment"]
    #[inline(always)]
    pub fn suadj1(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ctsusuclka_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ctsusuclka_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CTSU SUCLK Multiplier Rate Setting"]
    #[inline(always)]
    pub fn sumulti1(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ctsusuclka_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ctsusuclka_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsusuclka {
    #[inline(always)]
    fn default() -> Ctsusuclka {
        <crate::RegValueT<Ctsusuclka_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusuclk0_SPEC;
impl crate::sealed::RegSpec for Ctsusuclk0_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Sensor Unit Clock Control Register A"]
pub type Ctsusuclk0 = crate::RegValueT<Ctsusuclk0_SPEC>;

impl NoBitfieldReg<Ctsusuclk0_SPEC> for Ctsusuclk0 {}
impl ::core::default::Default for Ctsusuclk0 {
    #[inline(always)]
    fn default() -> Ctsusuclk0 {
        <crate::RegValueT<Ctsusuclk0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusuclk1_SPEC;
impl crate::sealed::RegSpec for Ctsusuclk1_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Sensor Unit Clock Control Register A"]
pub type Ctsusuclk1 = crate::RegValueT<Ctsusuclk1_SPEC>;

impl NoBitfieldReg<Ctsusuclk1_SPEC> for Ctsusuclk1 {}
impl ::core::default::Default for Ctsusuclk1 {
    #[inline(always)]
    fn default() -> Ctsusuclk1 {
        <crate::RegValueT<Ctsusuclk1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusuclkb_SPEC;
impl crate::sealed::RegSpec for Ctsusuclkb_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Sensor Unit Clock Control Register B"]
pub type Ctsusuclkb = crate::RegValueT<Ctsusuclkb_SPEC>;

impl Ctsusuclkb {
    #[doc = "CTSU SUCLK Frequency Adjustment"]
    #[inline(always)]
    pub fn suadj2(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsusuclkb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsusuclkb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CTSU SUCLK Multiplier Rate Setting"]
    #[inline(always)]
    pub fn sumulti2(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsusuclkb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsusuclkb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CTSU SUCLK Frequency Adjustment"]
    #[inline(always)]
    pub fn suadj3(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ctsusuclkb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ctsusuclkb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CTSU SUCLK Multiplier Rate Setting"]
    #[inline(always)]
    pub fn sumulti3(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ctsusuclkb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ctsusuclkb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsusuclkb {
    #[inline(always)]
    fn default() -> Ctsusuclkb {
        <crate::RegValueT<Ctsusuclkb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusuclk2_SPEC;
impl crate::sealed::RegSpec for Ctsusuclk2_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Sensor Unit Clock Control Register B"]
pub type Ctsusuclk2 = crate::RegValueT<Ctsusuclk2_SPEC>;

impl NoBitfieldReg<Ctsusuclk2_SPEC> for Ctsusuclk2 {}
impl ::core::default::Default for Ctsusuclk2 {
    #[inline(always)]
    fn default() -> Ctsusuclk2 {
        <crate::RegValueT<Ctsusuclk2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusuclk3_SPEC;
impl crate::sealed::RegSpec for Ctsusuclk3_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Sensor Unit Clock Control Register B"]
pub type Ctsusuclk3 = crate::RegValueT<Ctsusuclk3_SPEC>;

impl NoBitfieldReg<Ctsusuclk3_SPEC> for Ctsusuclk3 {}
impl ::core::default::Default for Ctsusuclk3 {
    #[inline(always)]
    fn default() -> Ctsusuclk3 {
        <crate::RegValueT<Ctsusuclk3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuopt_SPEC;
impl crate::sealed::RegSpec for Ctsuopt_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Option Setting Register"]
pub type Ctsuopt = crate::RegValueT<Ctsuopt_SPEC>;

impl Ctsuopt {
    #[doc = "CCO Characteristics Correction Function Enable"]
    #[inline(always)]
    pub fn ccocfen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuopt::Ccocfen,
        ctsuopt::Ccocfen,
        Ctsuopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuopt::Ccocfen,
            ctsuopt::Ccocfen,
            Ctsuopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multi-clock Correction Function Enable"]
    #[inline(always)]
    pub fn mcacefn(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuopt::Mcacefn,
        ctsuopt::Mcacefn,
        Ctsuopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuopt::Mcacefn,
            ctsuopt::Mcacefn,
            Ctsuopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Majority Mode"]
    #[inline(always)]
    pub fn majirimd(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuopt::Majirimd,
        ctsuopt::Majirimd,
        Ctsuopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuopt::Majirimd,
            ctsuopt::Majirimd,
            Ctsuopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Transfer Request Disable"]
    #[inline(always)]
    pub fn dtcless(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsuopt::Dtcless,
        ctsuopt::Dtcless,
        Ctsuopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsuopt::Dtcless,
            ctsuopt::Dtcless,
            Ctsuopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mutual Capacitance Calculation Enable"]
    #[inline(always)]
    pub fn mtucfen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ctsuopt::Mtucfen,
        ctsuopt::Mtucfen,
        Ctsuopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ctsuopt::Mtucfen,
            ctsuopt::Mtucfen,
            Ctsuopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Automatic Judgment Function Enable"]
    #[inline(always)]
    pub fn ajfen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ctsuopt::Ajfen,
        ctsuopt::Ajfen,
        Ctsuopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ctsuopt::Ajfen,
            ctsuopt::Ajfen,
            Ctsuopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Automatic Judgment Interrupt Control"]
    #[inline(always)]
    pub fn ajintc(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ctsuopt::Ajintc,
        ctsuopt::Ajintc,
        Ctsuopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ctsuopt::Ajintc,
            ctsuopt::Ajintc,
            Ctsuopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sensor Counter Automatic Correction Table Number Setting"]
    #[inline(always)]
    pub fn scactb(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Ctsuopt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Ctsuopt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuopt {
    #[inline(always)]
    fn default() -> Ctsuopt {
        <crate::RegValueT<Ctsuopt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuopt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccocfen_SPEC;
    pub type Ccocfen = crate::EnumBitfieldStruct<u8, Ccocfen_SPEC>;
    impl Ccocfen {
        #[doc = "CCO characteristics correction function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CCO characteristics correction function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcacefn_SPEC;
    pub type Mcacefn = crate::EnumBitfieldStruct<u8, Mcacefn_SPEC>;
    impl Mcacefn {
        #[doc = "Multi-clock correction function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multi-clock correction function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Majirimd_SPEC;
    pub type Majirimd = crate::EnumBitfieldStruct<u8, Majirimd_SPEC>;
    impl Majirimd {
        #[doc = "Majority mode is not used"]
        pub const _0: Self = Self::new(0);

        #[doc = "Majority mode is used"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtcless_SPEC;
    pub type Dtcless = crate::EnumBitfieldStruct<u8, Dtcless_SPEC>;
    impl Dtcless {
        #[doc = "Data transfer request is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data transfer request is disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtucfen_SPEC;
    pub type Mtucfen = crate::EnumBitfieldStruct<u8, Mtucfen_SPEC>;
    impl Mtucfen {
        #[doc = "Do not subtract the first measurement data from the second measurement data"]
        pub const _0: Self = Self::new(0);

        #[doc = "Subtract the first measurement data from the second measurement data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajfen_SPEC;
    pub type Ajfen = crate::EnumBitfieldStruct<u8, Ajfen_SPEC>;
    impl Ajfen {
        #[doc = "Automatic judgment function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Automatic judgment function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajintc_SPEC;
    pub type Ajintc = crate::EnumBitfieldStruct<u8, Ajintc_SPEC>;
    impl Ajintc {
        #[doc = "INTFN_N for touch detection, INTAJN_N for non-touch detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "INTFN_N only regardless of touch/non-touch detection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuoptl_SPEC;
impl crate::sealed::RegSpec for Ctsuoptl_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Option Setting Register"]
pub type Ctsuoptl = crate::RegValueT<Ctsuoptl_SPEC>;

impl Ctsuoptl {
    #[doc = "CCO Characteristics Correction Function Enable"]
    #[inline(always)]
    pub fn ccocfen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuoptl::Ccocfen,
        ctsuoptl::Ccocfen,
        Ctsuoptl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuoptl::Ccocfen,
            ctsuoptl::Ccocfen,
            Ctsuoptl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multi-clock Correction Function Enable"]
    #[inline(always)]
    pub fn mcacefn(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuoptl::Mcacefn,
        ctsuoptl::Mcacefn,
        Ctsuoptl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuoptl::Mcacefn,
            ctsuoptl::Mcacefn,
            Ctsuoptl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Majority Mode"]
    #[inline(always)]
    pub fn majirimd(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuoptl::Majirimd,
        ctsuoptl::Majirimd,
        Ctsuoptl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuoptl::Majirimd,
            ctsuoptl::Majirimd,
            Ctsuoptl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Transfer Request Disable"]
    #[inline(always)]
    pub fn dtcless(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsuoptl::Dtcless,
        ctsuoptl::Dtcless,
        Ctsuoptl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsuoptl::Dtcless,
            ctsuoptl::Dtcless,
            Ctsuoptl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mutual Capacitance Calculation Enable"]
    #[inline(always)]
    pub fn mtucfen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ctsuoptl::Mtucfen,
        ctsuoptl::Mtucfen,
        Ctsuoptl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ctsuoptl::Mtucfen,
            ctsuoptl::Mtucfen,
            Ctsuoptl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Automatic Judgment Function Enable"]
    #[inline(always)]
    pub fn ajfen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ctsuoptl::Ajfen,
        ctsuoptl::Ajfen,
        Ctsuoptl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ctsuoptl::Ajfen,
            ctsuoptl::Ajfen,
            Ctsuoptl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Automatic Judgment Interrupt Control"]
    #[inline(always)]
    pub fn ajintc(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ctsuoptl::Ajintc,
        ctsuoptl::Ajintc,
        Ctsuoptl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ctsuoptl::Ajintc,
            ctsuoptl::Ajintc,
            Ctsuoptl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuoptl {
    #[inline(always)]
    fn default() -> Ctsuoptl {
        <crate::RegValueT<Ctsuoptl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuoptl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccocfen_SPEC;
    pub type Ccocfen = crate::EnumBitfieldStruct<u8, Ccocfen_SPEC>;
    impl Ccocfen {
        #[doc = "CCO characteristics correction function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CCO characteristics correction function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcacefn_SPEC;
    pub type Mcacefn = crate::EnumBitfieldStruct<u8, Mcacefn_SPEC>;
    impl Mcacefn {
        #[doc = "Multi-clock correction function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multi-clock correction function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Majirimd_SPEC;
    pub type Majirimd = crate::EnumBitfieldStruct<u8, Majirimd_SPEC>;
    impl Majirimd {
        #[doc = "Majority mode is not used"]
        pub const _0: Self = Self::new(0);

        #[doc = "Majority mode is used"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtcless_SPEC;
    pub type Dtcless = crate::EnumBitfieldStruct<u8, Dtcless_SPEC>;
    impl Dtcless {
        #[doc = "Data transfer request is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data transfer request is disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtucfen_SPEC;
    pub type Mtucfen = crate::EnumBitfieldStruct<u8, Mtucfen_SPEC>;
    impl Mtucfen {
        #[doc = "Do not subtract the first measurement data from the second measurement data"]
        pub const _0: Self = Self::new(0);

        #[doc = "Subtract the first measurement data from the second measurement data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajfen_SPEC;
    pub type Ajfen = crate::EnumBitfieldStruct<u8, Ajfen_SPEC>;
    impl Ajfen {
        #[doc = "Automatic judgment function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Automatic judgment function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajintc_SPEC;
    pub type Ajintc = crate::EnumBitfieldStruct<u8, Ajintc_SPEC>;
    impl Ajintc {
        #[doc = "INTFN_N for touch detection, INTAJN_N for non-touch detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "INTFN_N only regardless of touch/non-touch detection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ac_SPEC;
impl crate::sealed::RegSpec for Ac_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Option Setting Register"]
pub type Ac = crate::RegValueT<Ac_SPEC>;

impl Ac {
    #[doc = "CCO Characteristics Correction Function Enable"]
    #[inline(always)]
    pub fn ccocfen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ac::Ccocfen,
        ac::Ccocfen,
        Ac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ac::Ccocfen,
            ac::Ccocfen,
            Ac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multi-clock Correction Function Enable"]
    #[inline(always)]
    pub fn mcacefn(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ac::Mcacefn,
        ac::Mcacefn,
        Ac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ac::Mcacefn,
            ac::Mcacefn,
            Ac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Majority Mode"]
    #[inline(always)]
    pub fn majirimd(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ac::Majirimd,
        ac::Majirimd,
        Ac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ac::Majirimd,
            ac::Majirimd,
            Ac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Transfer Request Disable"]
    #[inline(always)]
    pub fn dtcless(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ac::Dtcless,
        ac::Dtcless,
        Ac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ac::Dtcless,
            ac::Dtcless,
            Ac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mutual Capacitance Calculation Enable"]
    #[inline(always)]
    pub fn mtucfen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ac::Mtucfen,
        ac::Mtucfen,
        Ac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ac::Mtucfen,
            ac::Mtucfen,
            Ac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ac {
    #[inline(always)]
    fn default() -> Ac {
        <crate::RegValueT<Ac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccocfen_SPEC;
    pub type Ccocfen = crate::EnumBitfieldStruct<u8, Ccocfen_SPEC>;
    impl Ccocfen {
        #[doc = "CCO characteristics correction function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CCO characteristics correction function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcacefn_SPEC;
    pub type Mcacefn = crate::EnumBitfieldStruct<u8, Mcacefn_SPEC>;
    impl Mcacefn {
        #[doc = "Multi-clock correction function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multi-clock correction function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Majirimd_SPEC;
    pub type Majirimd = crate::EnumBitfieldStruct<u8, Majirimd_SPEC>;
    impl Majirimd {
        #[doc = "Majority mode is not used"]
        pub const _0: Self = Self::new(0);

        #[doc = "Majority mode is used"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtcless_SPEC;
    pub type Dtcless = crate::EnumBitfieldStruct<u8, Dtcless_SPEC>;
    impl Dtcless {
        #[doc = "Data transfer request is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data transfer request is disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtucfen_SPEC;
    pub type Mtucfen = crate::EnumBitfieldStruct<u8, Mtucfen_SPEC>;
    impl Mtucfen {
        #[doc = "Do not subtract the first measurement data from the second measurement data"]
        pub const _0: Self = Self::new(0);

        #[doc = "Subtract the first measurement data from the second measurement data"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aj_SPEC;
impl crate::sealed::RegSpec for Aj_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Option Setting Register"]
pub type Aj = crate::RegValueT<Aj_SPEC>;

impl Aj {
    #[doc = "Automatic Judgment Function Enable"]
    #[inline(always)]
    pub fn ajfen(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, aj::Ajfen, aj::Ajfen, Aj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,aj::Ajfen,aj::Ajfen,Aj_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Automatic Judgment Interrupt Control"]
    #[inline(always)]
    pub fn ajintc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        aj::Ajintc,
        aj::Ajintc,
        Aj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            aj::Ajintc,
            aj::Ajintc,
            Aj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Aj {
    #[inline(always)]
    fn default() -> Aj {
        <crate::RegValueT<Aj_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod aj {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajfen_SPEC;
    pub type Ajfen = crate::EnumBitfieldStruct<u8, Ajfen_SPEC>;
    impl Ajfen {
        #[doc = "Automatic judgment function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Automatic judgment function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajintc_SPEC;
    pub type Ajintc = crate::EnumBitfieldStruct<u8, Ajintc_SPEC>;
    impl Ajintc {
        #[doc = "INTFN_N for touch detection, INTAJN_N for non-touch detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "INTFN_N only regardless of touch/non-touch detection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuopth_SPEC;
impl crate::sealed::RegSpec for Ctsuopth_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Option Setting Register"]
pub type Ctsuopth = crate::RegValueT<Ctsuopth_SPEC>;

impl Ctsuopth {
    #[doc = "Sensor Counter Automatic Correction Table Number Setting"]
    #[inline(always)]
    pub fn scactb(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ctsuopth_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ctsuopth_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuopth {
    #[inline(always)]
    fn default() -> Ctsuopth {
        <crate::RegValueT<Ctsuopth_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acsb_SPEC;
impl crate::sealed::RegSpec for Acsb_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Option Setting Register"]
pub type Acsb = crate::RegValueT<Acsb_SPEC>;

impl Acsb {
    #[doc = "Sensor Counter Automatic Correction Table Number Setting"]
    #[inline(always)]
    pub fn scactb(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Acsb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Acsb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Acsb {
    #[inline(always)]
    fn default() -> Acsb {
        <crate::RegValueT<Acsb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuscntact_SPEC;
impl crate::sealed::RegSpec for Ctsuscntact_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Sensor Counter Automatic Correction Table Access Register"]
pub type Ctsuscntact = crate::RegValueT<Ctsuscntact_SPEC>;

impl Ctsuscntact {
    #[doc = "Sensor Counter Correction Coefficient Setting"]
    #[inline(always)]
    pub fn scntaccoeff(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuscntact_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuscntact_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Set the measurement value to be compared."]
    #[inline(always)]
    pub fn scntaccount(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ctsuscntact_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Ctsuscntact_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuscntact {
    #[inline(always)]
    fn default() -> Ctsuscntact {
        <crate::RegValueT<Ctsuscntact_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuscntactl_SPEC;
impl crate::sealed::RegSpec for Ctsuscntactl_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Sensor Counter Automatic Correction Table Access Register"]
pub type Ctsuscntactl = crate::RegValueT<Ctsuscntactl_SPEC>;

impl Ctsuscntactl {
    #[doc = "Sensor Counter Correction Coefficient Setting"]
    #[inline(always)]
    pub fn scntaccoeff(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuscntactl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Ctsuscntactl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuscntactl {
    #[inline(always)]
    fn default() -> Ctsuscntactl {
        <crate::RegValueT<Ctsuscntactl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuscntacth_SPEC;
impl crate::sealed::RegSpec for Ctsuscntacth_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Sensor Counter Automatic Correction Table Access Register"]
pub type Ctsuscntacth = crate::RegValueT<Ctsuscntacth_SPEC>;

impl Ctsuscntacth {
    #[doc = "Set the measurement value to be compared."]
    #[inline(always)]
    pub fn scntaccount(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuscntacth_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Ctsuscntacth_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuscntacth {
    #[inline(always)]
    fn default() -> Ctsuscntacth {
        <crate::RegValueT<Ctsuscntacth_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact1_SPEC;
impl crate::sealed::RegSpec for Ctsumact1_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
pub type Ctsumact1 = crate::RegValueT<Ctsumact1_SPEC>;

impl Ctsumact1 {
    #[doc = "Sensor offset adjustment bits for multi-clock"]
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsumact1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsumact1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Offset coefficient bits for multi-clock"]
    #[inline(always)]
    pub fn offsetcoeff(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ctsumact1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ctsumact1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact1 {
    #[inline(always)]
    fn default() -> Ctsumact1 {
        <crate::RegValueT<Ctsumact1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact1L_SPEC;
impl crate::sealed::RegSpec for Ctsumact1L_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
pub type Ctsumact1L = crate::RegValueT<Ctsumact1L_SPEC>;

impl Ctsumact1L {
    #[doc = "Sensor offset adjustment bits for multi-clock"]
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsumact1L_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsumact1L_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact1L {
    #[inline(always)]
    fn default() -> Ctsumact1L {
        <crate::RegValueT<Ctsumact1L_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact1H_SPEC;
impl crate::sealed::RegSpec for Ctsumact1H_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
pub type Ctsumact1H = crate::RegValueT<Ctsumact1H_SPEC>;

impl Ctsumact1H {
    #[doc = "Offset coefficient bits for multi-clock"]
    #[inline(always)]
    pub fn offsetcoeff(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsumact1H_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsumact1H_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact1H {
    #[inline(always)]
    fn default() -> Ctsumact1H {
        <crate::RegValueT<Ctsumact1H_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact2_SPEC;
impl crate::sealed::RegSpec for Ctsumact2_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
pub type Ctsumact2 = crate::RegValueT<Ctsumact2_SPEC>;

impl Ctsumact2 {
    #[doc = "Sensor offset adjustment bits for multi-clock"]
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsumact2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsumact2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Offset coefficient bits for multi-clock"]
    #[inline(always)]
    pub fn offsetcoeff(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ctsumact2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ctsumact2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact2 {
    #[inline(always)]
    fn default() -> Ctsumact2 {
        <crate::RegValueT<Ctsumact2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact2L_SPEC;
impl crate::sealed::RegSpec for Ctsumact2L_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
pub type Ctsumact2L = crate::RegValueT<Ctsumact2L_SPEC>;

impl Ctsumact2L {
    #[doc = "Sensor offset adjustment bits for multi-clock"]
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsumact2L_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsumact2L_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact2L {
    #[inline(always)]
    fn default() -> Ctsumact2L {
        <crate::RegValueT<Ctsumact2L_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact2H_SPEC;
impl crate::sealed::RegSpec for Ctsumact2H_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
pub type Ctsumact2H = crate::RegValueT<Ctsumact2H_SPEC>;

impl Ctsumact2H {
    #[doc = "Offset coefficient bits for multi-clock"]
    #[inline(always)]
    pub fn offsetcoeff(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsumact2H_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsumact2H_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact2H {
    #[inline(always)]
    fn default() -> Ctsumact2H {
        <crate::RegValueT<Ctsumact2H_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact3_SPEC;
impl crate::sealed::RegSpec for Ctsumact3_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
pub type Ctsumact3 = crate::RegValueT<Ctsumact3_SPEC>;

impl Ctsumact3 {
    #[doc = "Sensor offset adjustment bits for multi-clock"]
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsumact3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsumact3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Offset coefficient bits for multi-clock"]
    #[inline(always)]
    pub fn offsetcoeff(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ctsumact3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ctsumact3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact3 {
    #[inline(always)]
    fn default() -> Ctsumact3 {
        <crate::RegValueT<Ctsumact3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact3L_SPEC;
impl crate::sealed::RegSpec for Ctsumact3L_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
pub type Ctsumact3L = crate::RegValueT<Ctsumact3L_SPEC>;

impl Ctsumact3L {
    #[doc = "Sensor offset adjustment bits for multi-clock"]
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsumact3L_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsumact3L_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact3L {
    #[inline(always)]
    fn default() -> Ctsumact3L {
        <crate::RegValueT<Ctsumact3L_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact3H_SPEC;
impl crate::sealed::RegSpec for Ctsumact3H_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Multi-Clock Automatic Correction Table Access Register"]
pub type Ctsumact3H = crate::RegValueT<Ctsumact3H_SPEC>;

impl Ctsumact3H {
    #[doc = "Offset coefficient bits for multi-clock"]
    #[inline(always)]
    pub fn offsetcoeff(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsumact3H_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsumact3H_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact3H {
    #[inline(always)]
    fn default() -> Ctsumact3H {
        <crate::RegValueT<Ctsumact3H_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajcr_SPEC;
impl crate::sealed::RegSpec for Ctsuajcr_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Automatic Judgment Control Register"]
pub type Ctsuajcr = crate::RegValueT<Ctsuajcr_SPEC>;

impl Ctsuajcr {
    #[doc = "Non-Touch Judgment Criterion Setting"]
    #[inline(always)]
    pub fn tlot(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsuajcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsuajcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Touch Judgment Criterion Setting"]
    #[inline(always)]
    pub fn thot(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsuajcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsuajcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Baseline Initialization"]
    #[inline(always)]
    pub fn blini(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ctsuajcr::Blini,
        ctsuajcr::Blini,
        Ctsuajcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ctsuajcr::Blini,
            ctsuajcr::Blini,
            Ctsuajcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Judgment Condition Setting"]
    #[inline(always)]
    pub fn jc(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        ctsuajcr::Jc,
        ctsuajcr::Jc,
        Ctsuajcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            ctsuajcr::Jc,
            ctsuajcr::Jc,
            Ctsuajcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Measured Value Moving Average Number Setting"]
    #[inline(always)]
    pub fn ajmmat(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Ctsuajcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Ctsuajcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Baseline Average Number Setting"]
    #[inline(always)]
    pub fn ajbmat(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Ctsuajcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Ctsuajcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajcr {
    #[inline(always)]
    fn default() -> Ctsuajcr {
        <crate::RegValueT<Ctsuajcr_SPEC> as RegisterValue<_>>::new(65536)
    }
}
pub mod ctsuajcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blini_SPEC;
    pub type Blini = crate::EnumBitfieldStruct<u8, Blini_SPEC>;
    impl Blini {
        #[doc = "Perform baseline operations"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initialize the results of baseline operations"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jc_SPEC;
    pub type Jc = crate::EnumBitfieldStruct<u8, Jc_SPEC>;
    impl Jc {
        #[doc = "Judges as touch when there is one or more touch detections"]
        pub const _00: Self = Self::new(0);

        #[doc = "Judges as touch when there are two or more touch detections"]
        pub const _01: Self = Self::new(1);

        #[doc = "Judges as touch when there are three or more touch detections"]
        pub const _10: Self = Self::new(2);

        #[doc = "Judges as touch only when there are four touch detections"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajcrl_SPEC;
impl crate::sealed::RegSpec for Ctsuajcrl_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Automatic Judgment Control Register"]
pub type Ctsuajcrl = crate::RegValueT<Ctsuajcrl_SPEC>;

impl Ctsuajcrl {
    #[doc = "Non-Touch Judgment Criterion Setting"]
    #[inline(always)]
    pub fn tlot(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsuajcrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsuajcrl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Touch Judgment Criterion Setting"]
    #[inline(always)]
    pub fn thot(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsuajcrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsuajcrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajcrl {
    #[inline(always)]
    fn default() -> Ctsuajcrl {
        <crate::RegValueT<Ctsuajcrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ajcr0_SPEC;
impl crate::sealed::RegSpec for Ajcr0_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Automatic Judgment Control Register"]
pub type Ajcr0 = crate::RegValueT<Ajcr0_SPEC>;

impl Ajcr0 {
    #[doc = "Non-Touch Judgment Criterion Setting"]
    #[inline(always)]
    pub fn tlot(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ajcr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ajcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ajcr0 {
    #[inline(always)]
    fn default() -> Ajcr0 {
        <crate::RegValueT<Ajcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ajcr1_SPEC;
impl crate::sealed::RegSpec for Ajcr1_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Automatic Judgment Control Register"]
pub type Ajcr1 = crate::RegValueT<Ajcr1_SPEC>;

impl Ajcr1 {
    #[doc = "Touch Judgment Criterion Setting"]
    #[inline(always)]
    pub fn thot(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ajcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ajcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ajcr1 {
    #[inline(always)]
    fn default() -> Ajcr1 {
        <crate::RegValueT<Ajcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajcrh_SPEC;
impl crate::sealed::RegSpec for Ctsuajcrh_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Automatic Judgment Control Register"]
pub type Ctsuajcrh = crate::RegValueT<Ctsuajcrh_SPEC>;

impl Ctsuajcrh {
    #[doc = "Baseline Initialization"]
    #[inline(always)]
    pub fn blini(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuajcrh::Blini,
        ctsuajcrh::Blini,
        Ctsuajcrh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuajcrh::Blini,
            ctsuajcrh::Blini,
            Ctsuajcrh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Judgment Condition Setting"]
    #[inline(always)]
    pub fn jc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ctsuajcrh::Jc,
        ctsuajcrh::Jc,
        Ctsuajcrh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ctsuajcrh::Jc,
            ctsuajcrh::Jc,
            Ctsuajcrh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Measured Value Moving Average Number Setting"]
    #[inline(always)]
    pub fn ajmmat(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Ctsuajcrh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Ctsuajcrh_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Baseline Average Number Setting"]
    #[inline(always)]
    pub fn ajbmat(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ctsuajcrh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Ctsuajcrh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajcrh {
    #[inline(always)]
    fn default() -> Ctsuajcrh {
        <crate::RegValueT<Ctsuajcrh_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod ctsuajcrh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blini_SPEC;
    pub type Blini = crate::EnumBitfieldStruct<u8, Blini_SPEC>;
    impl Blini {
        #[doc = "Perform baseline operations"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initialize the results of baseline operations"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jc_SPEC;
    pub type Jc = crate::EnumBitfieldStruct<u8, Jc_SPEC>;
    impl Jc {
        #[doc = "Judges as touch when there is one or more touch detections"]
        pub const _00: Self = Self::new(0);

        #[doc = "Judges as touch when there are two or more touch detections"]
        pub const _01: Self = Self::new(1);

        #[doc = "Judges as touch when there are three or more touch detections"]
        pub const _10: Self = Self::new(2);

        #[doc = "Judges as touch only when there are four touch detections"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ajcr2_SPEC;
impl crate::sealed::RegSpec for Ajcr2_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Automatic Judgment Control Register"]
pub type Ajcr2 = crate::RegValueT<Ajcr2_SPEC>;

impl Ajcr2 {
    #[doc = "Baseline Initialization"]
    #[inline(always)]
    pub fn blini(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ajcr2::Blini,
        ajcr2::Blini,
        Ajcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ajcr2::Blini,
            ajcr2::Blini,
            Ajcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Judgment Condition Setting"]
    #[inline(always)]
    pub fn jc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ajcr2::Jc,
        ajcr2::Jc,
        Ajcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ajcr2::Jc,
            ajcr2::Jc,
            Ajcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ajcr2 {
    #[inline(always)]
    fn default() -> Ajcr2 {
        <crate::RegValueT<Ajcr2_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod ajcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blini_SPEC;
    pub type Blini = crate::EnumBitfieldStruct<u8, Blini_SPEC>;
    impl Blini {
        #[doc = "Perform baseline operations"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initialize the results of baseline operations"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jc_SPEC;
    pub type Jc = crate::EnumBitfieldStruct<u8, Jc_SPEC>;
    impl Jc {
        #[doc = "Judges as touch when there is one or more touch detections"]
        pub const _00: Self = Self::new(0);

        #[doc = "Judges as touch when there are two or more touch detections"]
        pub const _01: Self = Self::new(1);

        #[doc = "Judges as touch when there are three or more touch detections"]
        pub const _10: Self = Self::new(2);

        #[doc = "Judges as touch only when there are four touch detections"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ajcr3_SPEC;
impl crate::sealed::RegSpec for Ajcr3_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Automatic Judgment Control Register"]
pub type Ajcr3 = crate::RegValueT<Ajcr3_SPEC>;

impl Ajcr3 {
    #[doc = "Measured Value Moving Average Number Setting"]
    #[inline(always)]
    pub fn ajmmat(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ajcr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ajcr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Baseline Average Number Setting"]
    #[inline(always)]
    pub fn ajbmat(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Ajcr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Ajcr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ajcr3 {
    #[inline(always)]
    fn default() -> Ajcr3 {
        <crate::RegValueT<Ajcr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajthr_SPEC;
impl crate::sealed::RegSpec for Ctsuajthr_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Automatic Judgment Control Register"]
pub type Ctsuajthr = crate::RegValueT<Ctsuajthr_SPEC>;

impl Ctsuajthr {
    #[doc = "Lower Threshold Setting"]
    #[inline(always)]
    pub fn ajthl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajthr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuajthr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Upper Threshold Setting"]
    #[inline(always)]
    pub fn ajthh(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ctsuajthr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ctsuajthr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajthr {
    #[inline(always)]
    fn default() -> Ctsuajthr {
        <crate::RegValueT<Ctsuajthr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajthrl_SPEC;
impl crate::sealed::RegSpec for Ctsuajthrl_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Automatic Judgment Control Register"]
pub type Ctsuajthrl = crate::RegValueT<Ctsuajthrl_SPEC>;

impl Ctsuajthrl {
    #[doc = "Lower Threshold Setting"]
    #[inline(always)]
    pub fn ajthl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajthrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuajthrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajthrl {
    #[inline(always)]
    fn default() -> Ctsuajthrl {
        <crate::RegValueT<Ctsuajthrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajthrh_SPEC;
impl crate::sealed::RegSpec for Ctsuajthrh_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Automatic Judgment Control Register"]
pub type Ctsuajthrh = crate::RegValueT<Ctsuajthrh_SPEC>;

impl Ctsuajthrh {
    #[doc = "Upper Threshold Setting"]
    #[inline(always)]
    pub fn ajthh(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajthrh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuajthrh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajthrh {
    #[inline(always)]
    fn default() -> Ctsuajthrh {
        <crate::RegValueT<Ctsuajthrh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajmmar_SPEC;
impl crate::sealed::RegSpec for Ctsuajmmar_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Threshold Register"]
pub type Ctsuajmmar = crate::RegValueT<Ctsuajmmar_SPEC>;

impl Ctsuajmmar {
    #[doc = "Moving Average Count"]
    #[inline(always)]
    pub fn ajmmati(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ctsuajmmar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ctsuajmmar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Moving Average Result"]
    #[inline(always)]
    pub fn ajmmr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        u32,
        Ctsuajmmar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            u32,
            Ctsuajmmar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuajmmar {
    #[inline(always)]
    fn default() -> Ctsuajmmar {
        <crate::RegValueT<Ctsuajmmar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajmmarl_SPEC;
impl crate::sealed::RegSpec for Ctsuajmmarl_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Threshold Register"]
pub type Ctsuajmmarl = crate::RegValueT<Ctsuajmmarl_SPEC>;

impl Ctsuajmmarl {
    #[doc = "Moving Average Count"]
    #[inline(always)]
    pub fn ajmmati(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ctsuajmmarl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ctsuajmmarl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Moving Average Result"]
    #[inline(always)]
    pub fn ajmmr(
        self,
    ) -> crate::common::RegisterField<5, 0x7ff, 1, 0, u16, u16, Ctsuajmmarl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7ff,1,0,u16,u16,Ctsuajmmarl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajmmarl {
    #[inline(always)]
    fn default() -> Ctsuajmmarl {
        <crate::RegValueT<Ctsuajmmarl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajmmarh_SPEC;
impl crate::sealed::RegSpec for Ctsuajmmarh_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Threshold Register"]
pub type Ctsuajmmarh = crate::RegValueT<Ctsuajmmarh_SPEC>;

impl Ctsuajmmarh {
    #[doc = "Moving Average Result"]
    #[inline(always)]
    pub fn ajmmr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajmmarh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuajmmarh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajmmarh {
    #[inline(always)]
    fn default() -> Ctsuajmmarh {
        <crate::RegValueT<Ctsuajmmarh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajblact_SPEC;
impl crate::sealed::RegSpec for Ctsuajblact_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Baseline Average Intermediate Result Register"]
pub type Ctsuajblact = crate::RegValueT<Ctsuajblact_SPEC>;

impl Ctsuajblact {
    #[doc = "Automatic determination baseline average calculation bits"]
    #[inline(always)]
    pub fn ajblact(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Ctsuajblact_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Ctsuajblact_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuajblact {
    #[inline(always)]
    fn default() -> Ctsuajblact {
        <crate::RegValueT<Ctsuajblact_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajblactl_SPEC;
impl crate::sealed::RegSpec for Ctsuajblactl_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Baseline Average Intermediate Result Register"]
pub type Ctsuajblactl = crate::RegValueT<Ctsuajblactl_SPEC>;

impl Ctsuajblactl {
    #[doc = "Automatic determination baseline average calculation bits"]
    #[inline(always)]
    pub fn ajblact(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajblactl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Ctsuajblactl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuajblactl {
    #[inline(always)]
    fn default() -> Ctsuajblactl {
        <crate::RegValueT<Ctsuajblactl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajblacth_SPEC;
impl crate::sealed::RegSpec for Ctsuajblacth_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Baseline Average Intermediate Result Register"]
pub type Ctsuajblacth = crate::RegValueT<Ctsuajblacth_SPEC>;

impl Ctsuajblacth {
    #[doc = "Automatic determination baseline average calculation bits"]
    #[inline(always)]
    pub fn ajblact(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajblacth_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Ctsuajblacth_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuajblacth {
    #[inline(always)]
    fn default() -> Ctsuajblacth {
        <crate::RegValueT<Ctsuajblacth_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajblar_SPEC;
impl crate::sealed::RegSpec for Ctsuajblar_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Baseline Average Result Register"]
pub type Ctsuajblar = crate::RegValueT<Ctsuajblar_SPEC>;

impl Ctsuajblar {
    #[doc = "Baseline Average Count"]
    #[inline(always)]
    pub fn ajblac(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajblar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuajblar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Baseline Average Result"]
    #[inline(always)]
    pub fn ajblar(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ctsuajblar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ctsuajblar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajblar {
    #[inline(always)]
    fn default() -> Ctsuajblar {
        <crate::RegValueT<Ctsuajblar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajblarl_SPEC;
impl crate::sealed::RegSpec for Ctsuajblarl_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Baseline Average Result Register"]
pub type Ctsuajblarl = crate::RegValueT<Ctsuajblarl_SPEC>;

impl Ctsuajblarl {
    #[doc = "Baseline Average Count"]
    #[inline(always)]
    pub fn ajblac(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajblarl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuajblarl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajblarl {
    #[inline(always)]
    fn default() -> Ctsuajblarl {
        <crate::RegValueT<Ctsuajblarl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajblarh_SPEC;
impl crate::sealed::RegSpec for Ctsuajblarh_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Baseline Average Result Register"]
pub type Ctsuajblarh = crate::RegValueT<Ctsuajblarh_SPEC>;

impl Ctsuajblarh {
    #[doc = "Baseline Average Result"]
    #[inline(always)]
    pub fn ajblar(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajblarh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuajblarh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajblarh {
    #[inline(always)]
    fn default() -> Ctsuajblarh {
        <crate::RegValueT<Ctsuajblarh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajrr_SPEC;
impl crate::sealed::RegSpec for Ctsuajrr_SPEC {
    type DataType = u32;
}

#[doc = "CTSU Automatic Judgment Result Register"]
pub type Ctsuajrr = crate::RegValueT<Ctsuajrr_SPEC>;

impl Ctsuajrr {
    #[doc = "The judgment result when using random pulse or SUCLK0 is stored."]
    #[inline(always)]
    pub fn tjr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuajrr::Tjr0,
        ctsuajrr::Tjr0,
        Ctsuajrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuajrr::Tjr0,
            ctsuajrr::Tjr0,
            Ctsuajrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The judgment result when using SUCLK1 is stored."]
    #[inline(always)]
    pub fn tjr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuajrr::Tjr1,
        ctsuajrr::Tjr1,
        Ctsuajrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuajrr::Tjr1,
            ctsuajrr::Tjr1,
            Ctsuajrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The judgment result when using SUCLK2 is stored."]
    #[inline(always)]
    pub fn tjr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuajrr::Tjr2,
        ctsuajrr::Tjr2,
        Ctsuajrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuajrr::Tjr2,
            ctsuajrr::Tjr2,
            Ctsuajrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The judgment result when using SUCLK3 is stored."]
    #[inline(always)]
    pub fn tjr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsuajrr::Tjr3,
        ctsuajrr::Tjr3,
        Ctsuajrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsuajrr::Tjr3,
            ctsuajrr::Tjr3,
            Ctsuajrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The final judgment result on multi-clock measurement is stored. If multi-clock measurement is not specified, the same value as the TJR0 flag is stored."]
    #[inline(always)]
    pub fn fjr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsuajrr::Fjr,
        ctsuajrr::Fjr,
        Ctsuajrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsuajrr::Fjr,
            ctsuajrr::Fjr,
            Ctsuajrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Remaining Number of Consecutive Detections"]
    #[inline(always)]
    pub fn sjccr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsuajrr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsuajrr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajrr {
    #[inline(always)]
    fn default() -> Ctsuajrr {
        <crate::RegValueT<Ctsuajrr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuajrr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr0_SPEC;
    pub type Tjr0 = crate::EnumBitfieldStruct<u8, Tjr0_SPEC>;
    impl Tjr0 {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr1_SPEC;
    pub type Tjr1 = crate::EnumBitfieldStruct<u8, Tjr1_SPEC>;
    impl Tjr1 {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr2_SPEC;
    pub type Tjr2 = crate::EnumBitfieldStruct<u8, Tjr2_SPEC>;
    impl Tjr2 {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr3_SPEC;
    pub type Tjr3 = crate::EnumBitfieldStruct<u8, Tjr3_SPEC>;
    impl Tjr3 {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fjr_SPEC;
    pub type Fjr = crate::EnumBitfieldStruct<u8, Fjr_SPEC>;
    impl Fjr {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajrrl_SPEC;
impl crate::sealed::RegSpec for Ctsuajrrl_SPEC {
    type DataType = u16;
}

#[doc = "CTSU Automatic Judgment Result Register"]
pub type Ctsuajrrl = crate::RegValueT<Ctsuajrrl_SPEC>;

impl Ctsuajrrl {
    #[doc = "The judgment result when using random pulse or SUCLK0 is stored."]
    #[inline(always)]
    pub fn tjr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuajrrl::Tjr0,
        ctsuajrrl::Tjr0,
        Ctsuajrrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuajrrl::Tjr0,
            ctsuajrrl::Tjr0,
            Ctsuajrrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The judgment result when using SUCLK1 is stored."]
    #[inline(always)]
    pub fn tjr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuajrrl::Tjr1,
        ctsuajrrl::Tjr1,
        Ctsuajrrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuajrrl::Tjr1,
            ctsuajrrl::Tjr1,
            Ctsuajrrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The judgment result when using SUCLK2 is stored."]
    #[inline(always)]
    pub fn tjr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuajrrl::Tjr2,
        ctsuajrrl::Tjr2,
        Ctsuajrrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuajrrl::Tjr2,
            ctsuajrrl::Tjr2,
            Ctsuajrrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The judgment result when using SUCLK3 is stored."]
    #[inline(always)]
    pub fn tjr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsuajrrl::Tjr3,
        ctsuajrrl::Tjr3,
        Ctsuajrrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsuajrrl::Tjr3,
            ctsuajrrl::Tjr3,
            Ctsuajrrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The final judgment result on multi-clock measurement is stored. If multi-clock measurement is not specified, the same value as the TJR0 flag is stored."]
    #[inline(always)]
    pub fn fjr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsuajrrl::Fjr,
        ctsuajrrl::Fjr,
        Ctsuajrrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsuajrrl::Fjr,
            ctsuajrrl::Fjr,
            Ctsuajrrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Remaining Number of Consecutive Detections"]
    #[inline(always)]
    pub fn sjccr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsuajrrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsuajrrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajrrl {
    #[inline(always)]
    fn default() -> Ctsuajrrl {
        <crate::RegValueT<Ctsuajrrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuajrrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr0_SPEC;
    pub type Tjr0 = crate::EnumBitfieldStruct<u8, Tjr0_SPEC>;
    impl Tjr0 {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr1_SPEC;
    pub type Tjr1 = crate::EnumBitfieldStruct<u8, Tjr1_SPEC>;
    impl Tjr1 {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr2_SPEC;
    pub type Tjr2 = crate::EnumBitfieldStruct<u8, Tjr2_SPEC>;
    impl Tjr2 {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr3_SPEC;
    pub type Tjr3 = crate::EnumBitfieldStruct<u8, Tjr3_SPEC>;
    impl Tjr3 {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fjr_SPEC;
    pub type Fjr = crate::EnumBitfieldStruct<u8, Fjr_SPEC>;
    impl Fjr {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajrr0_SPEC;
impl crate::sealed::RegSpec for Ctsuajrr0_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Automatic Judgment Result Register"]
pub type Ctsuajrr0 = crate::RegValueT<Ctsuajrr0_SPEC>;

impl Ctsuajrr0 {
    #[doc = "The judgment result when using random pulse or SUCLK0 is stored."]
    #[inline(always)]
    pub fn tjr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuajrr0::Tjr0,
        ctsuajrr0::Tjr0,
        Ctsuajrr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuajrr0::Tjr0,
            ctsuajrr0::Tjr0,
            Ctsuajrr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The judgment result when using SUCLK1 is stored."]
    #[inline(always)]
    pub fn tjr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuajrr0::Tjr1,
        ctsuajrr0::Tjr1,
        Ctsuajrr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuajrr0::Tjr1,
            ctsuajrr0::Tjr1,
            Ctsuajrr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The judgment result when using SUCLK2 is stored."]
    #[inline(always)]
    pub fn tjr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuajrr0::Tjr2,
        ctsuajrr0::Tjr2,
        Ctsuajrr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuajrr0::Tjr2,
            ctsuajrr0::Tjr2,
            Ctsuajrr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The judgment result when using SUCLK3 is stored."]
    #[inline(always)]
    pub fn tjr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsuajrr0::Tjr3,
        ctsuajrr0::Tjr3,
        Ctsuajrr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsuajrr0::Tjr3,
            ctsuajrr0::Tjr3,
            Ctsuajrr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "The final judgment result on multi-clock measurement is stored. If multi-clock measurement is not specified, the same value as the TJR0 flag is stored."]
    #[inline(always)]
    pub fn fjr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsuajrr0::Fjr,
        ctsuajrr0::Fjr,
        Ctsuajrr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsuajrr0::Fjr,
            ctsuajrr0::Fjr,
            Ctsuajrr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuajrr0 {
    #[inline(always)]
    fn default() -> Ctsuajrr0 {
        <crate::RegValueT<Ctsuajrr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuajrr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr0_SPEC;
    pub type Tjr0 = crate::EnumBitfieldStruct<u8, Tjr0_SPEC>;
    impl Tjr0 {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr1_SPEC;
    pub type Tjr1 = crate::EnumBitfieldStruct<u8, Tjr1_SPEC>;
    impl Tjr1 {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr2_SPEC;
    pub type Tjr2 = crate::EnumBitfieldStruct<u8, Tjr2_SPEC>;
    impl Tjr2 {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr3_SPEC;
    pub type Tjr3 = crate::EnumBitfieldStruct<u8, Tjr3_SPEC>;
    impl Tjr3 {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fjr_SPEC;
    pub type Fjr = crate::EnumBitfieldStruct<u8, Fjr_SPEC>;
    impl Fjr {
        #[doc = "Non-touch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Touch"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajrr1_SPEC;
impl crate::sealed::RegSpec for Ctsuajrr1_SPEC {
    type DataType = u8;
}

#[doc = "CTSU Automatic Judgment Result Register"]
pub type Ctsuajrr1 = crate::RegValueT<Ctsuajrr1_SPEC>;

impl Ctsuajrr1 {
    #[doc = "Remaining Number of Consecutive Detections"]
    #[inline(always)]
    pub fn sjccr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsuajrr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsuajrr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajrr1 {
    #[inline(always)]
    fn default() -> Ctsuajrr1 {
        <crate::RegValueT<Ctsuajrr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
