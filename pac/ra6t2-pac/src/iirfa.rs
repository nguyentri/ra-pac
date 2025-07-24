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
// Generated from SVD 1.40.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:52:39 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"IIR filter accelerator"]
unsafe impl ::core::marker::Send for super::Iirfa {}
unsafe impl ::core::marker::Sync for super::Iirfa {}
impl super::Iirfa {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Channel Processing Status Register"]
    #[inline(always)]
    pub const fn iircprcs(
        &self,
    ) -> &'static crate::common::Reg<self::Iircprcs_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iircprcs_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Channel Processing Completion Flag Register"]
    #[inline(always)]
    pub const fn iircprcff(
        &self,
    ) -> &'static crate::common::Reg<self::Iircprcff_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iircprcff_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Output Data Preparation Completion Flag Register"]
    #[inline(always)]
    pub const fn iirordyf(
        &self,
    ) -> &'static crate::common::Reg<self::Iirordyf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirordyf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Operation Error Flag Register"]
    #[inline(always)]
    pub const fn iircerrf(
        &self,
    ) -> &'static crate::common::Reg<self::Iircerrf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iircerrf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Operation Control Register"]
    #[inline(always)]
    pub const fn iiropcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iiropcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iiropcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "ECC Control Register"]
    #[inline(always)]
    pub const fn iirecccnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirecccnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirecccnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "ECC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn iireccint(
        &self,
    ) -> &'static crate::common::Reg<self::Iireccint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iireccint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "ECC Error Flag Register"]
    #[inline(always)]
    pub const fn iireccef(
        &self,
    ) -> &'static crate::common::Reg<self::Iireccef_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iireccef_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "ECC Error Flag Clear Register"]
    #[inline(always)]
    pub const fn iireccefclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iireccefclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iireccefclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "ECC 1-bit Error Address Register"]
    #[inline(always)]
    pub const fn iireseadr(
        &self,
    ) -> &'static crate::common::Reg<self::Iireseadr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iireseadr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "ECC 2-bit Error Address Register"]
    #[inline(always)]
    pub const fn iiredeadr(
        &self,
    ) -> &'static crate::common::Reg<self::Iiredeadr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iiredeadr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Channel %s Input Register"]
    #[inline(always)]
    pub const fn iirchinp(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirchinp_SPEC, crate::common::W>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
    #[inline(always)]
    pub const fn iirch0inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch1inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch2inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x120usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch3inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch4inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x140usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch5inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x150usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch6inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x160usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch7inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x170usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch8inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x180usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch9inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x190usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch10inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch11inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch12inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch13inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch14inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch15inp(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchinp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchinp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f0usize),
            )
        }
    }

    #[doc = "Channel %s Output Register"]
    #[inline(always)]
    pub const fn iirchout(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirchout_SPEC, crate::common::R>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x104usize))
        }
    }
    #[inline(always)]
    pub const fn iirch0out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch1out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x114usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch2out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x124usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch3out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch4out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x144usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch5out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x154usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch6out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x164usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch7out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x174usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch8out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x184usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch9out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x194usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch10out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch11out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch12out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch13out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch14out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch15out(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchout_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchout_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f4usize),
            )
        }
    }

    #[doc = "Channel %s Control Register"]
    #[inline(always)]
    pub const fn iirchcnt(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x108usize))
        }
    }
    #[inline(always)]
    pub const fn iirch0cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch1cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch2cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x128usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch3cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x138usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch4cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x148usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch5cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x158usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch6cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x168usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch7cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x178usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch8cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x188usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch9cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x198usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch10cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch11cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch12cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch13cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch14cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch15cnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f8usize),
            )
        }
    }

    #[doc = "Channel %s Interrupt Enable Register"]
    #[inline(always)]
    pub const fn iirchint(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirchint_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10cusize))
        }
    }
    #[inline(always)]
    pub const fn iirch0int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch1int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch2int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch3int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch4int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch5int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x15cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch6int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x16cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch7int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x17cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch8int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch9int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x19cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch10int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch11int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch12int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch13int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch14int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch15int(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirchint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1fcusize),
            )
        }
    }

    #[doc = "Channel %s Status Register"]
    #[inline(always)]
    pub const fn iirchsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirchsts_SPEC, crate::common::R>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10dusize))
        }
    }
    #[inline(always)]
    pub const fn iirch0sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x10dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch1sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x11dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch2sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x12dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch3sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x13dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch4sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x14dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch5sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x15dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch6sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x16dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch7sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x17dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch8sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x18dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch9sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x19dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch10sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1adusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch11sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1bdusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch12sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cdusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch13sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ddusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch14sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1edusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch15sts(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirchsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1fdusize),
            )
        }
    }

    #[doc = "Channel %s Flag Clear Register"]
    #[inline(always)]
    pub const fn iirchfclr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10eusize))
        }
    }
    #[inline(always)]
    pub const fn iirch0fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x10eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch1fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x11eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch2fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x12eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch3fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x13eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch4fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x14eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch5fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x15eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch6fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x16eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch7fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x17eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch8fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x18eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch9fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x19eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch10fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1aeusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch11fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1beusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch12fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ceusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch13fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1deusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch14fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1eeusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirch15fclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iirchfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1feusize),
            )
        }
    }

    #[doc = "Stage %s Coefficient b0 Register"]
    #[inline(always)]
    pub const fn iirstgb0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x400usize))
        }
    }
    #[inline(always)]
    pub const fn iirstg0b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x400usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg1b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x420usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg2b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x440usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg3b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x460usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg4b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x480usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg5b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg6b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg7b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg8b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x500usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg9b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x520usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg10b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x540usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg11b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x560usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg12b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x580usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg13b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg14b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg15b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg16b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x600usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg17b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x620usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg18b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x640usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg19b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x660usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg20b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x680usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg21b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg22b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg23b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg24b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x700usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg25b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x720usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg26b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x740usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg27b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x760usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg28b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x780usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg29b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg30b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg31b0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7e0usize),
            )
        }
    }

    #[doc = "Stage %s Coefficient b1 Register"]
    #[inline(always)]
    pub const fn iirstgb1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x404usize))
        }
    }
    #[inline(always)]
    pub const fn iirstg0b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x404usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg1b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x424usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg2b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x444usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg3b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x464usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg4b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x484usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg5b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg6b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg7b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg8b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x504usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg9b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x524usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg10b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x544usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg11b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x564usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg12b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x584usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg13b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg14b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg15b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg16b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x604usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg17b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x624usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg18b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x644usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg19b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x664usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg20b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x684usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg21b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg22b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg23b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg24b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x704usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg25b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x724usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg26b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x744usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg27b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x764usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg28b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x784usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg29b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg30b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg31b1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7e4usize),
            )
        }
    }

    #[doc = "Stage %s Coefficient b2 Register"]
    #[inline(always)]
    pub const fn iirstgb2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x408usize))
        }
    }
    #[inline(always)]
    pub const fn iirstg0b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x408usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg1b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x428usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg2b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x448usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg3b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x468usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg4b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x488usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg5b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg6b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg7b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg8b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x508usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg9b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x528usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg10b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x548usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg11b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x568usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg12b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x588usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg13b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg14b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg15b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg16b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x608usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg17b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x628usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg18b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x648usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg19b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x668usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg20b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x688usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg21b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg22b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg23b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg24b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x708usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg25b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x728usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg26b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x748usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg27b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x768usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg28b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x788usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg29b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg30b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg31b2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7e8usize),
            )
        }
    }

    #[doc = "Stage %s Coefficient a1 Register"]
    #[inline(always)]
    pub const fn iirstga1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40cusize))
        }
    }
    #[inline(always)]
    pub const fn iirstg0a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg1a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x42cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg2a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg3a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x46cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg4a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg5a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg6a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg7a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg8a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg9a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x52cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg10a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg11a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg12a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg13a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg14a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg15a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg16a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg17a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x62cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg18a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg19a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x66cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg20a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg21a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg22a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg23a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg24a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x70cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg25a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x72cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg26a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg27a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x76cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg28a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg29a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg30a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg31a1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7ecusize),
            )
        }
    }

    #[doc = "Stage %s Coefficient a2 Register"]
    #[inline(always)]
    pub const fn iirstga2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x410usize))
        }
    }
    #[inline(always)]
    pub const fn iirstg0a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x410usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg1a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x430usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg2a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x450usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg3a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x470usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg4a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x490usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg5a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg6a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg7a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg8a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x510usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg9a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x530usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg10a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x550usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg11a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x570usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg12a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x590usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg13a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg14a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg15a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg16a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x610usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg17a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x630usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg18a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x650usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg19a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x670usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg20a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x690usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg21a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg22a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg23a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg24a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x710usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg25a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x730usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg26a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x750usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg27a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x770usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg28a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x790usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg29a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg30a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg31a2(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstga2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7f0usize),
            )
        }
    }

    #[doc = "Stage %s Delay Data D0 Register"]
    #[inline(always)]
    pub const fn iirstgd0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x414usize))
        }
    }
    #[inline(always)]
    pub const fn iirstg0d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x414usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg1d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x434usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg2d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x454usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg3d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x474usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg4d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x494usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg5d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg6d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg7d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg8d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x514usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg9d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x534usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg10d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x554usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg11d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x574usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg12d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x594usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg13d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg14d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg15d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg16d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x614usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg17d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x634usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg18d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x654usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg19d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x674usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg20d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x694usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg21d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg22d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg23d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg24d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x714usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg25d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x734usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg26d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x754usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg27d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x774usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg28d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x794usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg29d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg30d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg31d0(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7f4usize),
            )
        }
    }

    #[doc = "Stage %s Delay Data D1 Register"]
    #[inline(always)]
    pub const fn iirstgd1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x418usize))
        }
    }
    #[inline(always)]
    pub const fn iirstg0d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x418usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg1d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x438usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg2d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x458usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg3d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x478usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg4d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x498usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg5d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg6d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg7d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg8d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x518usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg9d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x538usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg10d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x558usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg11d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x578usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg12d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x598usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg13d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg14d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg15d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg16d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x618usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg17d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x638usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg18d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x658usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg19d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x678usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg20d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x698usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg21d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg22d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg23d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg24d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x718usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg25d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x738usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg26d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x758usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg27d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x778usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg28d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x798usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg29d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg30d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn iirstg31d1(
        &self,
    ) -> &'static crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirstgd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7f8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iircprcs_SPEC;
impl crate::sealed::RegSpec for Iircprcs_SPEC {
    type DataType = u32;
}

#[doc = "Channel Processing Status Register"]
pub type Iircprcs = crate::RegValueT<Iircprcs_SPEC>;

impl Iircprcs {
    #[doc = "Channel processing status bit"]
    #[inline(always)]
    pub fn cprcs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        iircprcs::Cprcs,
        iircprcs::Cprcs,
        Iircprcs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            iircprcs::Cprcs,
            iircprcs::Cprcs,
            Iircprcs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iircprcs {
    #[inline(always)]
    fn default() -> Iircprcs {
        <crate::RegValueT<Iircprcs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iircprcs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cprcs_SPEC;
    pub type Cprcs = crate::EnumBitfieldStruct<u8, Cprcs_SPEC>;
    impl Cprcs {
        #[doc = "The channel processing of the corresponding channel is not being performed."]
        pub const _0: Self = Self::new(0);

        #[doc = "The channel processing of the corresponding channel is being performed."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iircprcff_SPEC;
impl crate::sealed::RegSpec for Iircprcff_SPEC {
    type DataType = u32;
}

#[doc = "Channel Processing Completion Flag Register"]
pub type Iircprcff = crate::RegValueT<Iircprcff_SPEC>;

impl Iircprcff {
    #[doc = "Channel processing completion flag"]
    #[inline(always)]
    pub fn cprcff(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        iircprcff::Cprcff,
        iircprcff::Cprcff,
        Iircprcff_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            iircprcff::Cprcff,
            iircprcff::Cprcff,
            Iircprcff_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iircprcff {
    #[inline(always)]
    fn default() -> Iircprcff {
        <crate::RegValueT<Iircprcff_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iircprcff {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cprcff_SPEC;
    pub type Cprcff = crate::EnumBitfieldStruct<u8, Cprcff_SPEC>;
    impl Cprcff {
        #[doc = "The channel processing of the corresponding channel is not completed."]
        pub const _0: Self = Self::new(0);

        #[doc = "The channel processing of the corresponding channel is completed."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirordyf_SPEC;
impl crate::sealed::RegSpec for Iirordyf_SPEC {
    type DataType = u32;
}

#[doc = "Output Data Preparation Completion Flag Register"]
pub type Iirordyf = crate::RegValueT<Iirordyf_SPEC>;

impl Iirordyf {
    #[doc = "Output data preparation completion flag"]
    #[inline(always)]
    pub fn ordyf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        iirordyf::Ordyf,
        iirordyf::Ordyf,
        Iirordyf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            iirordyf::Ordyf,
            iirordyf::Ordyf,
            Iirordyf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iirordyf {
    #[inline(always)]
    fn default() -> Iirordyf {
        <crate::RegValueT<Iirordyf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iirordyf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ordyf_SPEC;
    pub type Ordyf = crate::EnumBitfieldStruct<u8, Ordyf_SPEC>;
    impl Ordyf {
        #[doc = "The output data preparation of the corresponding channel is not completed."]
        pub const _0: Self = Self::new(0);

        #[doc = "The output data preparation of the corresponding channel is completed."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iircerrf_SPEC;
impl crate::sealed::RegSpec for Iircerrf_SPEC {
    type DataType = u32;
}

#[doc = "Operation Error Flag Register"]
pub type Iircerrf = crate::RegValueT<Iircerrf_SPEC>;

impl Iircerrf {
    #[doc = "Operation error flag"]
    #[inline(always)]
    pub fn cerrf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        iircerrf::Cerrf,
        iircerrf::Cerrf,
        Iircerrf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            iircerrf::Cerrf,
            iircerrf::Cerrf,
            Iircerrf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iircerrf {
    #[inline(always)]
    fn default() -> Iircerrf {
        <crate::RegValueT<Iircerrf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iircerrf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerrf_SPEC;
    pub type Cerrf = crate::EnumBitfieldStruct<u8, Cerrf_SPEC>;
    impl Cerrf {
        #[doc = "No operation error has occurred in the corresponding channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "An operation error has occurred in the corresponding channel."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iiropcnt_SPEC;
impl crate::sealed::RegSpec for Iiropcnt_SPEC {
    type DataType = u32;
}

#[doc = "Operation Control Register"]
pub type Iiropcnt = crate::RegValueT<Iiropcnt_SPEC>;

impl Iiropcnt {
    #[doc = "Setting for the rounding mode for addition and multiplication"]
    #[inline(always)]
    pub fn rnd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        iiropcnt::Rnd,
        iiropcnt::Rnd,
        Iiropcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            iiropcnt::Rnd,
            iiropcnt::Rnd,
            Iiropcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iiropcnt {
    #[inline(always)]
    fn default() -> Iiropcnt {
        <crate::RegValueT<Iiropcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iiropcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rnd_SPEC;
    pub type Rnd = crate::EnumBitfieldStruct<u8, Rnd_SPEC>;
    impl Rnd {
        #[doc = "Round to nearest"]
        pub const _000: Self = Self::new(0);

        #[doc = "Round toward zero"]
        pub const _001: Self = Self::new(1);

        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirecccnt_SPEC;
impl crate::sealed::RegSpec for Iirecccnt_SPEC {
    type DataType = u32;
}

#[doc = "ECC Control Register"]
pub type Iirecccnt = crate::RegValueT<Iirecccnt_SPEC>;

impl Iirecccnt {
    #[doc = "ECC setting bit"]
    #[inline(always)]
    pub fn eccmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iirecccnt::Eccmd,
        iirecccnt::Eccmd,
        Iirecccnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iirecccnt::Eccmd,
            iirecccnt::Eccmd,
            Iirecccnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC-corrected data write-back disable bit"]
    #[inline(always)]
    pub fn eccwbdis(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iirecccnt::Eccwbdis,
        iirecccnt::Eccwbdis,
        Iirecccnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iirecccnt::Eccwbdis,
            iirecccnt::Eccwbdis,
            Iirecccnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iirecccnt {
    #[inline(always)]
    fn default() -> Iirecccnt {
        <crate::RegValueT<Iirecccnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iirecccnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccmd_SPEC;
    pub type Eccmd = crate::EnumBitfieldStruct<u8, Eccmd_SPEC>;
    impl Eccmd {
        #[doc = "The ECC error detection/correction function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "The ECC error detection/correction function is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccwbdis_SPEC;
    pub type Eccwbdis = crate::EnumBitfieldStruct<u8, Eccwbdis_SPEC>;
    impl Eccwbdis {
        #[doc = "The error-corrected data write-back is enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "The error-corrected data write-back is disabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iireccint_SPEC;
impl crate::sealed::RegSpec for Iireccint_SPEC {
    type DataType = u32;
}

#[doc = "ECC Interrupt Enable Register"]
pub type Iireccint = crate::RegValueT<Iireccint_SPEC>;

impl Iireccint {
    #[doc = "ECC 1-bit error interrupt enable bit"]
    #[inline(always)]
    pub fn eseie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iireccint::Eseie,
        iireccint::Eseie,
        Iireccint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iireccint::Eseie,
            iireccint::Eseie,
            Iireccint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC 2-bit error interrupt enable bit"]
    #[inline(always)]
    pub fn edeie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iireccint::Edeie,
        iireccint::Edeie,
        Iireccint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iireccint::Edeie,
            iireccint::Edeie,
            Iireccint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iireccint {
    #[inline(always)]
    fn default() -> Iireccint {
        <crate::RegValueT<Iireccint_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iireccint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eseie_SPEC;
    pub type Eseie = crate::EnumBitfieldStruct<u8, Eseie_SPEC>;
    impl Eseie {
        #[doc = "The generation of ECC 1-bit error interrupt requests is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "The generation of ECC 1-bit error interrupt requests is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edeie_SPEC;
    pub type Edeie = crate::EnumBitfieldStruct<u8, Edeie_SPEC>;
    impl Edeie {
        #[doc = "The generation of ECC 2-bit error interrupt requests is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "The generation of ECC 2-bit error interrupt requests is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iireccef_SPEC;
impl crate::sealed::RegSpec for Iireccef_SPEC {
    type DataType = u32;
}

#[doc = "ECC Error Flag Register"]
pub type Iireccef = crate::RegValueT<Iireccef_SPEC>;

impl Iireccef {
    #[doc = "ECC 1-bit error flag"]
    #[inline(always)]
    pub fn esef(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iireccef::Esef,
        iireccef::Esef,
        Iireccef_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iireccef::Esef,
            iireccef::Esef,
            Iireccef_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "ECC 2-bit error flag"]
    #[inline(always)]
    pub fn edef(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iireccef::Edef,
        iireccef::Edef,
        Iireccef_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iireccef::Edef,
            iireccef::Edef,
            Iireccef_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iireccef {
    #[inline(always)]
    fn default() -> Iireccef {
        <crate::RegValueT<Iireccef_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iireccef {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esef_SPEC;
    pub type Esef = crate::EnumBitfieldStruct<u8, Esef_SPEC>;
    impl Esef {
        #[doc = "No 1-bit ECC error is detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "1-bit ECC error is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edef_SPEC;
    pub type Edef = crate::EnumBitfieldStruct<u8, Edef_SPEC>;
    impl Edef {
        #[doc = "No 2-bit ECC error is detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "2-bit ECC error is detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iireccefclr_SPEC;
impl crate::sealed::RegSpec for Iireccefclr_SPEC {
    type DataType = u32;
}

#[doc = "ECC Error Flag Clear Register"]
pub type Iireccefclr = crate::RegValueT<Iireccefclr_SPEC>;

impl Iireccefclr {
    #[doc = "ECC 1-bit error flag clear bit"]
    #[inline(always)]
    pub fn esefclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iireccefclr::Esefclr,
        iireccefclr::Esefclr,
        Iireccefclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iireccefclr::Esefclr,
            iireccefclr::Esefclr,
            Iireccefclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "ECC 2-bit error status flag clear bit"]
    #[inline(always)]
    pub fn edefclr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iireccefclr::Edefclr,
        iireccefclr::Edefclr,
        Iireccefclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iireccefclr::Edefclr,
            iireccefclr::Edefclr,
            Iireccefclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iireccefclr {
    #[inline(always)]
    fn default() -> Iireccefclr {
        <crate::RegValueT<Iireccefclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iireccefclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esefclr_SPEC;
    pub type Esefclr = crate::EnumBitfieldStruct<u8, Esefclr_SPEC>;
    impl Esefclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the ESEF flag of the IIRECCEF register."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edefclr_SPEC;
    pub type Edefclr = crate::EnumBitfieldStruct<u8, Edefclr_SPEC>;
    impl Edefclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the EDEF flag of the IIRECCEF register."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iireseadr_SPEC;
impl crate::sealed::RegSpec for Iireseadr_SPEC {
    type DataType = u32;
}

#[doc = "ECC 1-bit Error Address Register"]
pub type Iireseadr = crate::RegValueT<Iireseadr_SPEC>;

impl Iireseadr {
    #[doc = "Error address"]
    #[inline(always)]
    pub fn seadr(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Iireseadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Iireseadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Iireseadr {
    #[inline(always)]
    fn default() -> Iireseadr {
        <crate::RegValueT<Iireseadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iiredeadr_SPEC;
impl crate::sealed::RegSpec for Iiredeadr_SPEC {
    type DataType = u32;
}

#[doc = "ECC 2-bit Error Address Register"]
pub type Iiredeadr = crate::RegValueT<Iiredeadr_SPEC>;

impl Iiredeadr {
    #[doc = "Error address"]
    #[inline(always)]
    pub fn deadr(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Iiredeadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Iiredeadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Iiredeadr {
    #[inline(always)]
    fn default() -> Iiredeadr {
        <crate::RegValueT<Iiredeadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchinp_SPEC;
impl crate::sealed::RegSpec for Iirchinp_SPEC {
    type DataType = u32;
}

#[doc = "Channel %s Input Register"]
pub type Iirchinp = crate::RegValueT<Iirchinp_SPEC>;

impl NoBitfieldReg<Iirchinp_SPEC> for Iirchinp {}
impl ::core::default::Default for Iirchinp {
    #[inline(always)]
    fn default() -> Iirchinp {
        <crate::RegValueT<Iirchinp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchout_SPEC;
impl crate::sealed::RegSpec for Iirchout_SPEC {
    type DataType = u32;
}

#[doc = "Channel %s Output Register"]
pub type Iirchout = crate::RegValueT<Iirchout_SPEC>;

impl NoBitfieldReg<Iirchout_SPEC> for Iirchout {}
impl ::core::default::Default for Iirchout {
    #[inline(always)]
    fn default() -> Iirchout {
        <crate::RegValueT<Iirchout_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchcnt_SPEC;
impl crate::sealed::RegSpec for Iirchcnt_SPEC {
    type DataType = u32;
}

#[doc = "Channel %s Control Register"]
pub type Iirchcnt = crate::RegValueT<Iirchcnt_SPEC>;

impl Iirchcnt {
    #[doc = "Stage selection bit"]
    #[inline(always)]
    pub fn stgsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        iirchcnt::Stgsel,
        iirchcnt::Stgsel,
        Iirchcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            iirchcnt::Stgsel,
            iirchcnt::Stgsel,
            Iirchcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iirchcnt {
    #[inline(always)]
    fn default() -> Iirchcnt {
        <crate::RegValueT<Iirchcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iirchcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stgsel_SPEC;
    pub type Stgsel = crate::EnumBitfieldStruct<u8, Stgsel_SPEC>;
    impl Stgsel {
        #[doc = "The corresponding stage is not used for channel n."]
        pub const _0: Self = Self::new(0);

        #[doc = "The corresponding stage is used for channel n."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchint_SPEC;
impl crate::sealed::RegSpec for Iirchint_SPEC {
    type DataType = u8;
}

#[doc = "Channel %s Interrupt Enable Register"]
pub type Iirchint = crate::RegValueT<Iirchint_SPEC>;

impl Iirchint {
    #[doc = "Channel processing completion interrupt enable bit"]
    #[inline(always)]
    pub fn cprcfie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iirchint::Cprcfie,
        iirchint::Cprcfie,
        Iirchint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iirchint::Cprcfie,
            iirchint::Cprcfie,
            Iirchint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Output data preparation completion interrupt enable bit"]
    #[inline(always)]
    pub fn ordyie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        iirchint::Ordyie,
        iirchint::Ordyie,
        Iirchint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            iirchint::Ordyie,
            iirchint::Ordyie,
            Iirchint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operation error interrupt enable bit"]
    #[inline(always)]
    pub fn cerrie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iirchint::Cerrie,
        iirchint::Cerrie,
        Iirchint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iirchint::Cerrie,
            iirchint::Cerrie,
            Iirchint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iirchint {
    #[inline(always)]
    fn default() -> Iirchint {
        <crate::RegValueT<Iirchint_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iirchint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cprcfie_SPEC;
    pub type Cprcfie = crate::EnumBitfieldStruct<u8, Cprcfie_SPEC>;
    impl Cprcfie {
        #[doc = "The generation of channel processing completion interrupt requests is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "The generation of channel processing completion interrupt requests is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ordyie_SPEC;
    pub type Ordyie = crate::EnumBitfieldStruct<u8, Ordyie_SPEC>;
    impl Ordyie {
        #[doc = "The generation of output data preparation completion interrupt requests is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "The generation of output data preparation completion interrupt requests is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerrie_SPEC;
    pub type Cerrie = crate::EnumBitfieldStruct<u8, Cerrie_SPEC>;
    impl Cerrie {
        #[doc = "The generation of operation error interrupt requests is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "The generation of operation error interrupt requests is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchsts_SPEC;
impl crate::sealed::RegSpec for Iirchsts_SPEC {
    type DataType = u8;
}

#[doc = "Channel %s Status Register"]
pub type Iirchsts = crate::RegValueT<Iirchsts_SPEC>;

impl Iirchsts {
    #[doc = "Channel processing status flag"]
    #[inline(always)]
    pub fn cprcs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iirchsts::Cprcs,
        iirchsts::Cprcs,
        Iirchsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iirchsts::Cprcs,
            iirchsts::Cprcs,
            Iirchsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Channel processing completion flag"]
    #[inline(always)]
    pub fn cprcff(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iirchsts::Cprcff,
        iirchsts::Cprcff,
        Iirchsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iirchsts::Cprcff,
            iirchsts::Cprcff,
            Iirchsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Output data preparation completion flag"]
    #[inline(always)]
    pub fn ordyf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        iirchsts::Ordyf,
        iirchsts::Ordyf,
        Iirchsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            iirchsts::Ordyf,
            iirchsts::Ordyf,
            Iirchsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Operation error flag"]
    #[inline(always)]
    pub fn cerrf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iirchsts::Cerrf,
        iirchsts::Cerrf,
        Iirchsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iirchsts::Cerrf,
            iirchsts::Cerrf,
            Iirchsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iirchsts {
    #[inline(always)]
    fn default() -> Iirchsts {
        <crate::RegValueT<Iirchsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iirchsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cprcs_SPEC;
    pub type Cprcs = crate::EnumBitfieldStruct<u8, Cprcs_SPEC>;
    impl Cprcs {
        #[doc = "The channel processing is not being performed."]
        pub const _0: Self = Self::new(0);

        #[doc = "The channel processing is being performed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cprcff_SPEC;
    pub type Cprcff = crate::EnumBitfieldStruct<u8, Cprcff_SPEC>;
    impl Cprcff {
        #[doc = "The channel processing is not completed."]
        pub const _0: Self = Self::new(0);

        #[doc = "The channel processing is completed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ordyf_SPEC;
    pub type Ordyf = crate::EnumBitfieldStruct<u8, Ordyf_SPEC>;
    impl Ordyf {
        #[doc = "The output data preparation is not completed."]
        pub const _0: Self = Self::new(0);

        #[doc = "The output data preparation is completed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerrf_SPEC;
    pub type Cerrf = crate::EnumBitfieldStruct<u8, Cerrf_SPEC>;
    impl Cerrf {
        #[doc = "No operation error has occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "An operation error has occurred."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchfclr_SPEC;
impl crate::sealed::RegSpec for Iirchfclr_SPEC {
    type DataType = u8;
}

#[doc = "Channel %s Flag Clear Register"]
pub type Iirchfclr = crate::RegValueT<Iirchfclr_SPEC>;

impl Iirchfclr {
    #[doc = "Channel processing completion flag clear bit"]
    #[inline(always)]
    pub fn cprcffclr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iirchfclr::Cprcffclr,
        iirchfclr::Cprcffclr,
        Iirchfclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iirchfclr::Cprcffclr,
            iirchfclr::Cprcffclr,
            Iirchfclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Operation error flag clear bit"]
    #[inline(always)]
    pub fn cerrfclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iirchfclr::Cerrfclr,
        iirchfclr::Cerrfclr,
        Iirchfclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iirchfclr::Cerrfclr,
            iirchfclr::Cerrfclr,
            Iirchfclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iirchfclr {
    #[inline(always)]
    fn default() -> Iirchfclr {
        <crate::RegValueT<Iirchfclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iirchfclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cprcffclr_SPEC;
    pub type Cprcffclr = crate::EnumBitfieldStruct<u8, Cprcffclr_SPEC>;
    impl Cprcffclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the CPRCFF flag of the IIRCHnSTS register."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerrfclr_SPEC;
    pub type Cerrfclr = crate::EnumBitfieldStruct<u8, Cerrfclr_SPEC>;
    impl Cerrfclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the CERRF flag of the IIRCHnSTS register."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstgb0_SPEC;
impl crate::sealed::RegSpec for Iirstgb0_SPEC {
    type DataType = u32;
}

#[doc = "Stage %s Coefficient b0 Register"]
pub type Iirstgb0 = crate::RegValueT<Iirstgb0_SPEC>;

impl NoBitfieldReg<Iirstgb0_SPEC> for Iirstgb0 {}
impl ::core::default::Default for Iirstgb0 {
    #[inline(always)]
    fn default() -> Iirstgb0 {
        <crate::RegValueT<Iirstgb0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstgb1_SPEC;
impl crate::sealed::RegSpec for Iirstgb1_SPEC {
    type DataType = u32;
}

#[doc = "Stage %s Coefficient b1 Register"]
pub type Iirstgb1 = crate::RegValueT<Iirstgb1_SPEC>;

impl NoBitfieldReg<Iirstgb1_SPEC> for Iirstgb1 {}
impl ::core::default::Default for Iirstgb1 {
    #[inline(always)]
    fn default() -> Iirstgb1 {
        <crate::RegValueT<Iirstgb1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstgb2_SPEC;
impl crate::sealed::RegSpec for Iirstgb2_SPEC {
    type DataType = u32;
}

#[doc = "Stage %s Coefficient b2 Register"]
pub type Iirstgb2 = crate::RegValueT<Iirstgb2_SPEC>;

impl NoBitfieldReg<Iirstgb2_SPEC> for Iirstgb2 {}
impl ::core::default::Default for Iirstgb2 {
    #[inline(always)]
    fn default() -> Iirstgb2 {
        <crate::RegValueT<Iirstgb2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstga1_SPEC;
impl crate::sealed::RegSpec for Iirstga1_SPEC {
    type DataType = u32;
}

#[doc = "Stage %s Coefficient a1 Register"]
pub type Iirstga1 = crate::RegValueT<Iirstga1_SPEC>;

impl NoBitfieldReg<Iirstga1_SPEC> for Iirstga1 {}
impl ::core::default::Default for Iirstga1 {
    #[inline(always)]
    fn default() -> Iirstga1 {
        <crate::RegValueT<Iirstga1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstga2_SPEC;
impl crate::sealed::RegSpec for Iirstga2_SPEC {
    type DataType = u32;
}

#[doc = "Stage %s Coefficient a2 Register"]
pub type Iirstga2 = crate::RegValueT<Iirstga2_SPEC>;

impl NoBitfieldReg<Iirstga2_SPEC> for Iirstga2 {}
impl ::core::default::Default for Iirstga2 {
    #[inline(always)]
    fn default() -> Iirstga2 {
        <crate::RegValueT<Iirstga2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstgd0_SPEC;
impl crate::sealed::RegSpec for Iirstgd0_SPEC {
    type DataType = u32;
}

#[doc = "Stage %s Delay Data D0 Register"]
pub type Iirstgd0 = crate::RegValueT<Iirstgd0_SPEC>;

impl NoBitfieldReg<Iirstgd0_SPEC> for Iirstgd0 {}
impl ::core::default::Default for Iirstgd0 {
    #[inline(always)]
    fn default() -> Iirstgd0 {
        <crate::RegValueT<Iirstgd0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstgd1_SPEC;
impl crate::sealed::RegSpec for Iirstgd1_SPEC {
    type DataType = u32;
}

#[doc = "Stage %s Delay Data D1 Register"]
pub type Iirstgd1 = crate::RegValueT<Iirstgd1_SPEC>;

impl NoBitfieldReg<Iirstgd1_SPEC> for Iirstgd1 {}
impl ::core::default::Default for Iirstgd1 {
    #[inline(always)]
    fn default() -> Iirstgd1 {
        <crate::RegValueT<Iirstgd1_SPEC> as RegisterValue<_>>::new(0)
    }
}
