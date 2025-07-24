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
// Generated from SVD 1.00.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:55:39 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"PDMIF_NS Register area"]
unsafe impl ::core::marker::Send for super::PdmifNs {}
unsafe impl ::core::marker::Sync for super::PdmifNs {}
impl super::PdmifNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Channel Software Start Trigger Register"]
    #[inline(always)]
    pub const fn pdcstrtr(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcstrtr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdcstrtr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Channel Software Stop Trigger Register"]
    #[inline(always)]
    pub const fn pdcstptr(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcstptr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdcstptr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Channel Software Change Trigger Register"]
    #[inline(always)]
    pub const fn pdcchgtr(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcchgtr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdcchgtr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Channel Interrupt Control Register"]
    #[inline(always)]
    pub const fn pdcicr(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Channel Status Register"]
    #[inline(always)]
    pub const fn pdcsr(&self) -> &'static crate::common::Reg<self::Pdcsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pdcsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Channel Status Clear Register"]
    #[inline(always)]
    pub const fn pdcscr(&self) -> &'static crate::common::Reg<self::Pdcscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdcscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Channel Sound Detection Control Register"]
    #[inline(always)]
    pub const fn pdcsdcr(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcsdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcsdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Channel Data Read Control Register"]
    #[inline(always)]
    pub const fn pdcdrcr(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcdrcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcdrcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Channel Data Clear Register"]
    #[inline(always)]
    pub const fn pdcdcr(&self) -> &'static crate::common::Reg<self::Pdcdcr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdcdcr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Version Register"]
    #[inline(always)]
    pub const fn pdvr(&self) -> &'static crate::common::Reg<self::Pdvr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pdvr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Software Start Trigger Register Channel %s"]
    #[inline(always)]
    pub const fn pdstrtrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdstrtrch_SPEC, crate::common::W>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
    #[inline(always)]
    pub const fn pdstrtrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdstrtrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdstrtrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdstrtrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdstrtrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdstrtrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdstrtrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdstrtrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdstrtrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x300usize),
            )
        }
    }

    #[doc = "Software Stop Trigger Register Channel %s"]
    #[inline(always)]
    pub const fn pdstptrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdstptrch_SPEC, crate::common::W>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x104usize))
        }
    }
    #[inline(always)]
    pub const fn pdstptrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdstptrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdstptrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdstptrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdstptrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdstptrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x204usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdstptrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdstptrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdstptrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x304usize),
            )
        }
    }

    #[doc = "Software Change Trigger Register Channel %s"]
    #[inline(always)]
    pub const fn pdchgtrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdchgtrch_SPEC, crate::common::W>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x108usize))
        }
    }
    #[inline(always)]
    pub const fn pdchgtrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdchgtrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdchgtrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdchgtrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdchgtrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdchgtrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x208usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdchgtrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdchgtrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdchgtrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x308usize),
            )
        }
    }

    #[doc = "Interrupt Control Register Channel %s"]
    #[inline(always)]
    pub const fn pdicrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdicrch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10cusize))
        }
    }
    #[inline(always)]
    pub const fn pdicrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdicrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdicrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdicrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdicrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdicrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdicrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdicrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdicrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30cusize),
            )
        }
    }

    #[doc = "Status Detection Control Register Channel %s"]
    #[inline(always)]
    pub const fn pdsdcrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdsdcrch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x110usize))
        }
    }
    #[inline(always)]
    pub const fn pdsdcrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsdcrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsdcrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdsdcrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsdcrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsdcrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x210usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdsdcrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsdcrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsdcrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x310usize),
            )
        }
    }

    #[doc = "Status Register Channel %s"]
    #[inline(always)]
    pub const fn pdsrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdsrch_SPEC, crate::common::R>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x114usize))
        }
    }
    #[inline(always)]
    pub const fn pdsrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsrch_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pdsrch_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x114usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdsrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsrch_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pdsrch_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x214usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdsrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsrch_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pdsrch_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x314usize),
            )
        }
    }

    #[doc = "Status Clear Register Channel %s"]
    #[inline(always)]
    pub const fn pdscrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdscrch_SPEC, crate::common::W>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x118usize))
        }
    }
    #[inline(always)]
    pub const fn pdscrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdscrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdscrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdscrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdscrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdscrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x218usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdscrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdscrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pdscrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x318usize),
            )
        }
    }

    #[doc = "Mode Setting Register Channel %s"]
    #[inline(always)]
    pub const fn pdmdsrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdmdsrch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x120usize))
        }
    }
    #[inline(always)]
    pub const fn pdmdsrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdmdsrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdmdsrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x120usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdmdsrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdmdsrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdmdsrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x220usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdmdsrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdmdsrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdmdsrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x320usize),
            )
        }
    }

    #[doc = "Sinc Filter Control Register Channel %s"]
    #[inline(always)]
    pub const fn pdsfcrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdsfcrch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x124usize))
        }
    }
    #[inline(always)]
    pub const fn pdsfcrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsfcrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsfcrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x124usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdsfcrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsfcrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsfcrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x224usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdsfcrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsfcrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsfcrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x324usize),
            )
        }
    }

    #[doc = "High-pass Filter Coefficient s(0) Register Channel %s"]
    #[inline(always)]
    pub const fn pdhfcs0rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdhfcs0Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x128usize))
        }
    }
    #[inline(always)]
    pub const fn pdhfcs0rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdhfcs0Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdhfcs0Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x128usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdhfcs0rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdhfcs0Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdhfcs0Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x228usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdhfcs0rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdhfcs0Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdhfcs0Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x328usize),
            )
        }
    }

    #[doc = "High-pass Filter Coefficient k(1) Register Channel %s"]
    #[inline(always)]
    pub const fn pdhfck1rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdhfck1Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x12cusize))
        }
    }
    #[inline(always)]
    pub const fn pdhfck1rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdhfck1Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdhfck1Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdhfck1rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdhfck1Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdhfck1Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdhfck1rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdhfck1Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdhfck1Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x32cusize),
            )
        }
    }

    #[doc = "High-pass Filter Coefficient h(0) Register Channel %s"]
    #[inline(always)]
    pub const fn pdhfch0rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdhfch0Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x130usize))
        }
    }
    #[inline(always)]
    pub const fn pdhfch0rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdhfch0Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdhfch0Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdhfch0rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdhfch0Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdhfch0Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x230usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdhfch0rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdhfch0Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdhfch0Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x330usize),
            )
        }
    }

    #[doc = "High-pass Filter Coefficient h(1) Register Channel %s"]
    #[inline(always)]
    pub const fn pdhfch1rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdhfch1Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x134usize))
        }
    }
    #[inline(always)]
    pub const fn pdhfch1rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdhfch1Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdhfch1Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdhfch1rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdhfch1Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdhfch1Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x234usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdhfch1rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdhfch1Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdhfch1Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x334usize),
            )
        }
    }

    #[doc = "Compensation Filter Coefficient h(0) Register Channel %s"]
    #[inline(always)]
    pub const fn pdcfch00rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdcfch00Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x138usize))
        }
    }
    #[inline(always)]
    pub const fn pdcfch00rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch00Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch00Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x138usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch00rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch00Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch00Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x238usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch00rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch00Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch00Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x338usize),
            )
        }
    }

    #[doc = "Compensation Filter Coefficient h(1) Register Channel %s"]
    #[inline(always)]
    pub const fn pdcfch01rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdcfch01Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13cusize))
        }
    }
    #[inline(always)]
    pub const fn pdcfch01rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch01Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch01Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch01rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch01Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch01Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch01rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch01Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch01Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x33cusize),
            )
        }
    }

    #[doc = "Compensation Filter Coefficient h(2) Register Channel %s"]
    #[inline(always)]
    pub const fn pdcfch02rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdcfch02Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x140usize))
        }
    }
    #[inline(always)]
    pub const fn pdcfch02rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch02Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch02Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x140usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch02rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch02Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch02Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x240usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch02rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch02Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch02Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x340usize),
            )
        }
    }

    #[doc = "Compensation Filter Coefficient h(3) Register Channel %s"]
    #[inline(always)]
    pub const fn pdcfch03rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdcfch03Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x144usize))
        }
    }
    #[inline(always)]
    pub const fn pdcfch03rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch03Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch03Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x144usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch03rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch03Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch03Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x244usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch03rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch03Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch03Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x344usize),
            )
        }
    }

    #[doc = "Compensation Filter Coefficient h(4) Register Channel %s"]
    #[inline(always)]
    pub const fn pdcfch04rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdcfch04Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x148usize))
        }
    }
    #[inline(always)]
    pub const fn pdcfch04rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch04Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch04Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x148usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch04rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch04Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch04Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x248usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch04rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch04Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch04Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x348usize),
            )
        }
    }

    #[doc = "Compensation Filter Coefficient h(5) Register Channel %s"]
    #[inline(always)]
    pub const fn pdcfch05rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdcfch05Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x14cusize))
        }
    }
    #[inline(always)]
    pub const fn pdcfch05rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch05Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch05Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch05rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch05Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch05Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch05rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch05Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch05Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34cusize),
            )
        }
    }

    #[doc = "Compensation Filter Coefficient h(6) Register Channel n"]
    #[inline(always)]
    pub const fn pdcfch06rchn(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch06RcHn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch06RcHn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(336usize),
            )
        }
    }

    #[doc = "Compensation Filter Coefficient h(7) Register Channel %s"]
    #[inline(always)]
    pub const fn pdcfch07rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdcfch07Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x154usize))
        }
    }
    #[inline(always)]
    pub const fn pdcfch07rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch07Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch07Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x154usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch07rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch07Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch07Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x254usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch07rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch07Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch07Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x354usize),
            )
        }
    }

    #[doc = "Compensation Filter Coefficient h(8) Register Channel %s"]
    #[inline(always)]
    pub const fn pdcfch08rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdcfch08Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x158usize))
        }
    }
    #[inline(always)]
    pub const fn pdcfch08rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch08Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch08Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x158usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch08rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch08Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch08Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x258usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch08rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch08Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch08Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x358usize),
            )
        }
    }

    #[doc = "Compensation Filter Coefficient h(9) Register Channel %s"]
    #[inline(always)]
    pub const fn pdcfch09rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdcfch09Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x15cusize))
        }
    }
    #[inline(always)]
    pub const fn pdcfch09rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch09Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch09Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x15cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch09rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch09Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch09Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x25cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch09rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch09Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch09Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x35cusize),
            )
        }
    }

    #[doc = "Compensation Filter Coefficient h(10) Register Channel %s"]
    #[inline(always)]
    pub const fn pdcfch10rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdcfch10Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x160usize))
        }
    }
    #[inline(always)]
    pub const fn pdcfch10rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch10Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch10Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x160usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch10rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch10Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch10Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x260usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdcfch10rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdcfch10Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdcfch10Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x360usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h0(10) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch010rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch010Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x164usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch010rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch010Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch010Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x164usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch010rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch010Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch010Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x264usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch010rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch010Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch010Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x364usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(0) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch100rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch100Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x168usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch100rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch100Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch100Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x168usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch100rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch100Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch100Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x268usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch100rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch100Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch100Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x368usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(1) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch101rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch101Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x16cusize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch101rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch101Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch101Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x16cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch101rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch101Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch101Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x26cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch101rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch101Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch101Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x36cusize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(2) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch102rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch102Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x170usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch102rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch102Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch102Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x170usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch102rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch102Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch102Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x270usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch102rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch102Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch102Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x370usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(3) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch103rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch103Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x174usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch103rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch103Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch103Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x174usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch103rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch103Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch103Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x274usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch103rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch103Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch103Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x374usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(4) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch104rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch104Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x178usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch104rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch104Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch104Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x178usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch104rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch104Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch104Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x278usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch104rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch104Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch104Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x378usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(5) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch105rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch105Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x17cusize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch105rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch105Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch105Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x17cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch105rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch105Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch105Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x27cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch105rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch105Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch105Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x37cusize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(6) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch106rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch106Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x180usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch106rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch106Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch106Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x180usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch106rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch106Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch106Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x280usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch106rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch106Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch106Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x380usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(7) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch107rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch107Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x184usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch107rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch107Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch107Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x184usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch107rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch107Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch107Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x284usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch107rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch107Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch107Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x384usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(8) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch108rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch108Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x188usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch108rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch108Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch108Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x188usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch108rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch108Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch108Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x288usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch108rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch108Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch108Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x388usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(9) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch109rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch109Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x18cusize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch109rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch109Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch109Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch109rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch109Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch109Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch109rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch109Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch109Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38cusize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(10) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch110rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch110Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x190usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch110rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch110Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch110Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x190usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch110rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch110Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch110Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x290usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch110rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch110Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch110Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x390usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(11) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch111rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch111Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x194usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch111rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch111Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch111Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x194usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch111rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch111Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch111Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x294usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch111rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch111Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch111Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x394usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(12) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch112rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch112Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x198usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch112rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch112Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch112Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x198usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch112rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch112Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch112Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x298usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch112rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch112Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch112Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x398usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(13) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch113rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch113Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x19cusize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch113rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch113Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch113Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x19cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch113rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch113Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch113Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x29cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch113rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch113Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch113Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x39cusize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(14) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch114rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch114Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a0usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch114rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch114Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch114Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch114rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch114Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch114Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch114rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch114Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch114Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a0usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(15) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch115rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch115Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a4usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch115rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch115Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch115Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch115rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch115Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch115Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch115rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch115Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch115Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a4usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(16) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch116rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch116Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a8usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch116rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch116Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch116Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch116rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch116Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch116Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch116rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch116Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch116Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a8usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(17) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch117rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch117Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1acusize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch117rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch117Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch117Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch117rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch117Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch117Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch117rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch117Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch117Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3acusize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(18) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch118rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch118Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1b0usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch118rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch118Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch118Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch118rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch118Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch118Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch118rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch118Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch118Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b0usize),
            )
        }
    }

    #[doc = "Low-pass Filter Coefficient h1(19) Register Channel %s"]
    #[inline(always)]
    pub const fn pdlfch119rch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdlfch119Rch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1b4usize))
        }
    }
    #[inline(always)]
    pub const fn pdlfch119rch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch119Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch119Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch119rch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch119Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch119Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdlfch119rch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdlfch119Rch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdlfch119Rch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b4usize),
            )
        }
    }

    #[doc = "Sound Detection Lower Threshold Register Channel %s"]
    #[inline(always)]
    pub const fn pdsdltrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdsdltrch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1b8usize))
        }
    }
    #[inline(always)]
    pub const fn pdsdltrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsdltrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsdltrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdsdltrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsdltrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsdltrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdsdltrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsdltrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsdltrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b8usize),
            )
        }
    }

    #[doc = "Sound Detection Upper Threshold Register Channel %s"]
    #[inline(always)]
    pub const fn pdsdutrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdsdutrch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1bcusize))
        }
    }
    #[inline(always)]
    pub const fn pdsdutrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsdutrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsdutrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdsdutrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsdutrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsdutrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdsdutrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsdutrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsdutrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3bcusize),
            )
        }
    }

    #[doc = "Data Buffer Control Register Channel %s"]
    #[inline(always)]
    pub const fn pddbcrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pddbcrch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1c0usize))
        }
    }
    #[inline(always)]
    pub const fn pddbcrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pddbcrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pddbcrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pddbcrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pddbcrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pddbcrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pddbcrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pddbcrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pddbcrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c0usize),
            )
        }
    }

    #[doc = "Short Circuit Threshold Setting Register Channel %s"]
    #[inline(always)]
    pub const fn pdsctsrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdsctsrch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1c4usize))
        }
    }
    #[inline(always)]
    pub const fn pdsctsrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsctsrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsctsrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdsctsrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsctsrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsctsrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdsctsrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdsctsrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsctsrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c4usize),
            )
        }
    }

    #[doc = "Overvoltage Lower Threshold Register Channel %s"]
    #[inline(always)]
    pub const fn pdovltrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdovltrch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1c8usize))
        }
    }
    #[inline(always)]
    pub const fn pdovltrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdovltrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdovltrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdovltrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdovltrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdovltrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdovltrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdovltrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdovltrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c8usize),
            )
        }
    }

    #[doc = "Overvoltage Upper Threshold Register Channel %s"]
    #[inline(always)]
    pub const fn pdovutrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pdovutrch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1ccusize))
        }
    }
    #[inline(always)]
    pub const fn pdovutrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pdovutrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdovutrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdovutrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pdovutrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdovutrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pdovutrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pdovutrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdovutrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3ccusize),
            )
        }
    }

    #[doc = "Data Read Control Register Channel %s"]
    #[inline(always)]
    pub const fn pddrcrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pddrcrch_SPEC, crate::common::RW>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1e0usize))
        }
    }
    #[inline(always)]
    pub const fn pddrcrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pddrcrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pddrcrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pddrcrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pddrcrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pddrcrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pddrcrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pddrcrch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pddrcrch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e0usize),
            )
        }
    }

    #[doc = "Data Clear Register Channel %s"]
    #[inline(always)]
    pub const fn pddcrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pddcrch_SPEC, crate::common::W>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1e4usize))
        }
    }
    #[inline(always)]
    pub const fn pddcrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pddcrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pddcrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pddcrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pddcrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pddcrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pddcrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pddcrch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pddcrch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e4usize),
            )
        }
    }

    #[doc = "Data Read Register Channel %s"]
    #[inline(always)]
    pub const fn pddrrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pddrrch_SPEC, crate::common::R>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1e8usize))
        }
    }
    #[inline(always)]
    pub const fn pddrrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pddrrch_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pddrrch_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pddrrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pddrrch_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pddrrch_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pddrrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pddrrch_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pddrrch_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e8usize),
            )
        }
    }

    #[doc = "Data Status Register Channel %s"]
    #[inline(always)]
    pub const fn pddsrch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pddsrch_SPEC, crate::common::R>,
        3,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1ecusize))
        }
    }
    #[inline(always)]
    pub const fn pddsrch0(
        &self,
    ) -> &'static crate::common::Reg<self::Pddsrch_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pddsrch_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pddsrch1(
        &self,
    ) -> &'static crate::common::Reg<self::Pddsrch_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pddsrch_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pddsrch2(
        &self,
    ) -> &'static crate::common::Reg<self::Pddsrch_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pddsrch_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x3ecusize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcstrtr_SPEC;
impl crate::sealed::RegSpec for Pdcstrtr_SPEC {
    type DataType = u32;
}

#[doc = "Channel Software Start Trigger Register"]
pub type Pdcstrtr = crate::RegValueT<Pdcstrtr_SPEC>;

impl Pdcstrtr {
    #[doc = "Channel 0 Start Trigger"]
    #[inline(always)]
    pub fn strtrg0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdcstrtr::Strtrg0,
        pdcstrtr::Strtrg0,
        Pdcstrtr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdcstrtr::Strtrg0,
            pdcstrtr::Strtrg0,
            Pdcstrtr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 Start Trigger"]
    #[inline(always)]
    pub fn strtrg1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdcstrtr::Strtrg1,
        pdcstrtr::Strtrg1,
        Pdcstrtr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdcstrtr::Strtrg1,
            pdcstrtr::Strtrg1,
            Pdcstrtr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 Start Trigger"]
    #[inline(always)]
    pub fn strtrg2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pdcstrtr::Strtrg2,
        pdcstrtr::Strtrg2,
        Pdcstrtr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pdcstrtr::Strtrg2,
            pdcstrtr::Strtrg2,
            Pdcstrtr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdcstrtr {
    #[inline(always)]
    fn default() -> Pdcstrtr {
        <crate::RegValueT<Pdcstrtr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdcstrtr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Strtrg0_SPEC;
    pub type Strtrg0 = crate::EnumBitfieldStruct<u8, Strtrg0_SPEC>;
    impl Strtrg0 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start channel 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Strtrg1_SPEC;
    pub type Strtrg1 = crate::EnumBitfieldStruct<u8, Strtrg1_SPEC>;
    impl Strtrg1 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start channel 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Strtrg2_SPEC;
    pub type Strtrg2 = crate::EnumBitfieldStruct<u8, Strtrg2_SPEC>;
    impl Strtrg2 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start channel 2"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcstptr_SPEC;
impl crate::sealed::RegSpec for Pdcstptr_SPEC {
    type DataType = u32;
}

#[doc = "Channel Software Stop Trigger Register"]
pub type Pdcstptr = crate::RegValueT<Pdcstptr_SPEC>;

impl Pdcstptr {
    #[doc = "Channel 0 Stop Trigger"]
    #[inline(always)]
    pub fn stptrg0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdcstptr::Stptrg0,
        pdcstptr::Stptrg0,
        Pdcstptr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdcstptr::Stptrg0,
            pdcstptr::Stptrg0,
            Pdcstptr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 Stop Trigger"]
    #[inline(always)]
    pub fn stptrg1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdcstptr::Stptrg1,
        pdcstptr::Stptrg1,
        Pdcstptr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdcstptr::Stptrg1,
            pdcstptr::Stptrg1,
            Pdcstptr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 Stop Trigger"]
    #[inline(always)]
    pub fn stptrg2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pdcstptr::Stptrg2,
        pdcstptr::Stptrg2,
        Pdcstptr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pdcstptr::Stptrg2,
            pdcstptr::Stptrg2,
            Pdcstptr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdcstptr {
    #[inline(always)]
    fn default() -> Pdcstptr {
        <crate::RegValueT<Pdcstptr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdcstptr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stptrg0_SPEC;
    pub type Stptrg0 = crate::EnumBitfieldStruct<u8, Stptrg0_SPEC>;
    impl Stptrg0 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop channel 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stptrg1_SPEC;
    pub type Stptrg1 = crate::EnumBitfieldStruct<u8, Stptrg1_SPEC>;
    impl Stptrg1 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop channel 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stptrg2_SPEC;
    pub type Stptrg2 = crate::EnumBitfieldStruct<u8, Stptrg2_SPEC>;
    impl Stptrg2 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop channel 2"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcchgtr_SPEC;
impl crate::sealed::RegSpec for Pdcchgtr_SPEC {
    type DataType = u32;
}

#[doc = "Channel Software Change Trigger Register"]
pub type Pdcchgtr = crate::RegValueT<Pdcchgtr_SPEC>;

impl Pdcchgtr {
    #[doc = "Channel 0 Change Trigger"]
    #[inline(always)]
    pub fn chgtrg0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdcchgtr::Chgtrg0,
        pdcchgtr::Chgtrg0,
        Pdcchgtr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdcchgtr::Chgtrg0,
            pdcchgtr::Chgtrg0,
            Pdcchgtr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 Change Trigger"]
    #[inline(always)]
    pub fn chgtrg1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdcchgtr::Chgtrg1,
        pdcchgtr::Chgtrg1,
        Pdcchgtr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdcchgtr::Chgtrg1,
            pdcchgtr::Chgtrg1,
            Pdcchgtr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 Change Trigger"]
    #[inline(always)]
    pub fn chgtrg2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pdcchgtr::Chgtrg2,
        pdcchgtr::Chgtrg2,
        Pdcchgtr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pdcchgtr::Chgtrg2,
            pdcchgtr::Chgtrg2,
            Pdcchgtr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdcchgtr {
    #[inline(always)]
    fn default() -> Pdcchgtr {
        <crate::RegValueT<Pdcchgtr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdcchgtr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chgtrg0_SPEC;
    pub type Chgtrg0 = crate::EnumBitfieldStruct<u8, Chgtrg0_SPEC>;
    impl Chgtrg0 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Change clock (PDM_CLK0) setting"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chgtrg1_SPEC;
    pub type Chgtrg1 = crate::EnumBitfieldStruct<u8, Chgtrg1_SPEC>;
    impl Chgtrg1 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Change clock (PDM_CLK1) setting"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chgtrg2_SPEC;
    pub type Chgtrg2 = crate::EnumBitfieldStruct<u8, Chgtrg2_SPEC>;
    impl Chgtrg2 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Change clock (PDM_CLK2) setting"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcicr_SPEC;
impl crate::sealed::RegSpec for Pdcicr_SPEC {
    type DataType = u32;
}

#[doc = "Channel Interrupt Control Register"]
pub type Pdcicr = crate::RegValueT<Pdcicr_SPEC>;

impl Pdcicr {
    #[doc = "Channel 0 Sound Detection Interrupt Enable Bit"]
    #[inline(always)]
    pub fn isde0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pdcicr::Isde0,
        pdcicr::Isde0,
        Pdcicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pdcicr::Isde0,
            pdcicr::Isde0,
            Pdcicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 Sound Detection Interrupt Enable Bit"]
    #[inline(always)]
    pub fn isde1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pdcicr::Isde1,
        pdcicr::Isde1,
        Pdcicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pdcicr::Isde1,
            pdcicr::Isde1,
            Pdcicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 Sound Detection Interrupt Enable Bit"]
    #[inline(always)]
    pub fn isde2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pdcicr::Isde2,
        pdcicr::Isde2,
        Pdcicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pdcicr::Isde2,
            pdcicr::Isde2,
            Pdcicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 0 Data Reception Interrupt Enable Bit"]
    #[inline(always)]
    pub fn idre0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        pdcicr::Idre0,
        pdcicr::Idre0,
        Pdcicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pdcicr::Idre0,
            pdcicr::Idre0,
            Pdcicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 Data Reception Interrupt Enable Bit"]
    #[inline(always)]
    pub fn idre1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        pdcicr::Idre1,
        pdcicr::Idre1,
        Pdcicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            pdcicr::Idre1,
            pdcicr::Idre1,
            Pdcicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 Data Reception Interrupt Enable Bit"]
    #[inline(always)]
    pub fn idre2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        pdcicr::Idre2,
        pdcicr::Idre2,
        Pdcicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            pdcicr::Idre2,
            pdcicr::Idre2,
            Pdcicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 0 Error Detection Interrupt Enable Bit"]
    #[inline(always)]
    pub fn iede0(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        pdcicr::Iede0,
        pdcicr::Iede0,
        Pdcicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            pdcicr::Iede0,
            pdcicr::Iede0,
            Pdcicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 Error Detection Interrupt Enable Bit"]
    #[inline(always)]
    pub fn iede1(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        pdcicr::Iede1,
        pdcicr::Iede1,
        Pdcicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            pdcicr::Iede1,
            pdcicr::Iede1,
            Pdcicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 Error Detection Interrupt Enable Bit"]
    #[inline(always)]
    pub fn iede2(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        pdcicr::Iede2,
        pdcicr::Iede2,
        Pdcicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            pdcicr::Iede2,
            pdcicr::Iede2,
            Pdcicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdcicr {
    #[inline(always)]
    fn default() -> Pdcicr {
        <crate::RegValueT<Pdcicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdcicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isde0_SPEC;
    pub type Isde0 = crate::EnumBitfieldStruct<u8, Isde0_SPEC>;
    impl Isde0 {
        #[doc = "Do not allow to issue PDM_SDET interrupt when the sound for channel 0 is detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow to issue PDM_SDET interrupt when the sound for channel 0 is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isde1_SPEC;
    pub type Isde1 = crate::EnumBitfieldStruct<u8, Isde1_SPEC>;
    impl Isde1 {
        #[doc = "Do not allow to issue PDM_SDET interrupt when the sound for channel 1 is detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow to issue PDM_SDET interrupt when the sound for channel 1 is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isde2_SPEC;
    pub type Isde2 = crate::EnumBitfieldStruct<u8, Isde2_SPEC>;
    impl Isde2 {
        #[doc = "Do not allow to issue PDM_SDET interrupt when the sound for channel 2 is detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow to issue PDM_SDET interrupt when the sound for channel 2 is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idre0_SPEC;
    pub type Idre0 = crate::EnumBitfieldStruct<u8, Idre0_SPEC>;
    impl Idre0 {
        #[doc = "Do not allow to issue PDM_DAT0 interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow to issue PDM_DAT0 interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idre1_SPEC;
    pub type Idre1 = crate::EnumBitfieldStruct<u8, Idre1_SPEC>;
    impl Idre1 {
        #[doc = "Do not allow to issue PDM_DAT1 interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow to issue PDM_DAT1 interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idre2_SPEC;
    pub type Idre2 = crate::EnumBitfieldStruct<u8, Idre2_SPEC>;
    impl Idre2 {
        #[doc = "Do not allow to issue PDM_DAT2 interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow to issue PDM_DAT2 interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iede0_SPEC;
    pub type Iede0 = crate::EnumBitfieldStruct<u8, Iede0_SPEC>;
    impl Iede0 {
        #[doc = "Do not allow to issue PDM_ERR0 interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow to issue PDM_ERR0 interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iede1_SPEC;
    pub type Iede1 = crate::EnumBitfieldStruct<u8, Iede1_SPEC>;
    impl Iede1 {
        #[doc = "Do not allow to issue PDM_ERR1 interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow to issue PDM_ERR1 interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iede2_SPEC;
    pub type Iede2 = crate::EnumBitfieldStruct<u8, Iede2_SPEC>;
    impl Iede2 {
        #[doc = "Do not allow to issue PDM_ERR2 interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow to issue PDM_ERR2 interrupt"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcsr_SPEC;
impl crate::sealed::RegSpec for Pdcsr_SPEC {
    type DataType = u32;
}

#[doc = "Channel Status Register"]
pub type Pdcsr = crate::RegValueT<Pdcsr_SPEC>;

impl Pdcsr {
    #[doc = "Channel 0 State"]
    #[inline(always)]
    pub fn state0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdcsr::State0,
        pdcsr::State0,
        Pdcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdcsr::State0,
            pdcsr::State0,
            Pdcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 State"]
    #[inline(always)]
    pub fn state1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdcsr::State1,
        pdcsr::State1,
        Pdcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdcsr::State1,
            pdcsr::State1,
            Pdcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 State"]
    #[inline(always)]
    pub fn state2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pdcsr::State2,
        pdcsr::State2,
        Pdcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pdcsr::State2,
            pdcsr::State2,
            Pdcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Channel 0 Sound Detection Flag"]
    #[inline(always)]
    pub fn sdf0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pdcsr::Sdf0,
        pdcsr::Sdf0,
        Pdcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pdcsr::Sdf0,
            pdcsr::Sdf0,
            Pdcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 Sound Detection Flag"]
    #[inline(always)]
    pub fn sdf1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pdcsr::Sdf1,
        pdcsr::Sdf1,
        Pdcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pdcsr::Sdf1,
            pdcsr::Sdf1,
            Pdcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 Sound Detection Flag"]
    #[inline(always)]
    pub fn sdf2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pdcsr::Sdf2,
        pdcsr::Sdf2,
        Pdcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pdcsr::Sdf2,
            pdcsr::Sdf2,
            Pdcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Channel 0 Data Reception Flag"]
    #[inline(always)]
    pub fn drf0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        pdcsr::Drf0,
        pdcsr::Drf0,
        Pdcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pdcsr::Drf0,
            pdcsr::Drf0,
            Pdcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 Data Reception Flag"]
    #[inline(always)]
    pub fn drf1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        pdcsr::Drf1,
        pdcsr::Drf1,
        Pdcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            pdcsr::Drf1,
            pdcsr::Drf1,
            Pdcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 Data Reception Flag"]
    #[inline(always)]
    pub fn drf2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        pdcsr::Drf2,
        pdcsr::Drf2,
        Pdcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            pdcsr::Drf2,
            pdcsr::Drf2,
            Pdcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Channel 0 Error Detection Flag"]
    #[inline(always)]
    pub fn edf0(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        pdcsr::Edf0,
        pdcsr::Edf0,
        Pdcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            pdcsr::Edf0,
            pdcsr::Edf0,
            Pdcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 Error Detection Flag"]
    #[inline(always)]
    pub fn edf1(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        pdcsr::Edf1,
        pdcsr::Edf1,
        Pdcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            pdcsr::Edf1,
            pdcsr::Edf1,
            Pdcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 Error Detection Flag"]
    #[inline(always)]
    pub fn edf2(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        pdcsr::Edf2,
        pdcsr::Edf2,
        Pdcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            pdcsr::Edf2,
            pdcsr::Edf2,
            Pdcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdcsr {
    #[inline(always)]
    fn default() -> Pdcsr {
        <crate::RegValueT<Pdcsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdcsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct State0_SPEC;
    pub type State0 = crate::EnumBitfieldStruct<u8, State0_SPEC>;
    impl State0 {
        #[doc = "Channel 0 stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "Channel 0 in operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct State1_SPEC;
    pub type State1 = crate::EnumBitfieldStruct<u8, State1_SPEC>;
    impl State1 {
        #[doc = "Channel 1 stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "Channel 1 in operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct State2_SPEC;
    pub type State2 = crate::EnumBitfieldStruct<u8, State2_SPEC>;
    impl State2 {
        #[doc = "Channel 2 stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "Channel 2 in operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdf0_SPEC;
    pub type Sdf0 = crate::EnumBitfieldStruct<u8, Sdf0_SPEC>;
    impl Sdf0 {
        #[doc = "Indicates that the sound which exceeded the threshold is not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that the sound which exceeded the threshold is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdf1_SPEC;
    pub type Sdf1 = crate::EnumBitfieldStruct<u8, Sdf1_SPEC>;
    impl Sdf1 {
        #[doc = "Indicates that the sound which exceeded the threshold is not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that the sound which exceeded the threshold is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdf2_SPEC;
    pub type Sdf2 = crate::EnumBitfieldStruct<u8, Sdf2_SPEC>;
    impl Sdf2 {
        #[doc = "Indicates that the sound which exceeded the threshold is not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that the sound which exceeded the threshold is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drf0_SPEC;
    pub type Drf0 = crate::EnumBitfieldStruct<u8, Drf0_SPEC>;
    impl Drf0 {
        #[doc = "Indicates that the number of data stored in buffer does not exceed the threshold"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that the number of data stored in buffer exceeded the threshold"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drf1_SPEC;
    pub type Drf1 = crate::EnumBitfieldStruct<u8, Drf1_SPEC>;
    impl Drf1 {
        #[doc = "Indicates that the number of data stored in buffer does not exceed the threshold"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that the number of data stored in buffer exceeded the threshold"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drf2_SPEC;
    pub type Drf2 = crate::EnumBitfieldStruct<u8, Drf2_SPEC>;
    impl Drf2 {
        #[doc = "Indicates that the number of data stored in buffer does not exceed the threshold"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that the number of data stored in buffer exceeded the threshold"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edf0_SPEC;
    pub type Edf0 = crate::EnumBitfieldStruct<u8, Edf0_SPEC>;
    impl Edf0 {
        #[doc = "Indicates that errors are not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that errors are detected. See 1.2.17Status Register (PDSR) for details."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edf1_SPEC;
    pub type Edf1 = crate::EnumBitfieldStruct<u8, Edf1_SPEC>;
    impl Edf1 {
        #[doc = "Indicates that errors are not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that errors are detected. See 1.2.17Status Register (PDSR) for details."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edf2_SPEC;
    pub type Edf2 = crate::EnumBitfieldStruct<u8, Edf2_SPEC>;
    impl Edf2 {
        #[doc = "Indicates that errors are not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that errors are detected. See 1.2.17Status Register (PDSR) for details."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcscr_SPEC;
impl crate::sealed::RegSpec for Pdcscr_SPEC {
    type DataType = u32;
}

#[doc = "Channel Status Clear Register"]
pub type Pdcscr = crate::RegValueT<Pdcscr_SPEC>;

impl Pdcscr {
    #[doc = "Channel 0 Sound Detection Flag Clear"]
    #[inline(always)]
    pub fn sdfc0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pdcscr::Sdfc0,
        pdcscr::Sdfc0,
        Pdcscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pdcscr::Sdfc0,
            pdcscr::Sdfc0,
            Pdcscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 Sound Detection Flag Clear"]
    #[inline(always)]
    pub fn sdfc1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pdcscr::Sdfc1,
        pdcscr::Sdfc1,
        Pdcscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pdcscr::Sdfc1,
            pdcscr::Sdfc1,
            Pdcscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 Sound Detection Flag Clear"]
    #[inline(always)]
    pub fn sdfc2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pdcscr::Sdfc2,
        pdcscr::Sdfc2,
        Pdcscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pdcscr::Sdfc2,
            pdcscr::Sdfc2,
            Pdcscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdcscr {
    #[inline(always)]
    fn default() -> Pdcscr {
        <crate::RegValueT<Pdcscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdcscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdfc0_SPEC;
    pub type Sdfc0 = crate::EnumBitfieldStruct<u8, Sdfc0_SPEC>;
    impl Sdfc0 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear PDCSR.SDF0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdfc1_SPEC;
    pub type Sdfc1 = crate::EnumBitfieldStruct<u8, Sdfc1_SPEC>;
    impl Sdfc1 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear PDCSR.SDF1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdfc2_SPEC;
    pub type Sdfc2 = crate::EnumBitfieldStruct<u8, Sdfc2_SPEC>;
    impl Sdfc2 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear PDCSR.SDF2"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcsdcr_SPEC;
impl crate::sealed::RegSpec for Pdcsdcr_SPEC {
    type DataType = u32;
}

#[doc = "Channel Sound Detection Control Register"]
pub type Pdcsdcr = crate::RegValueT<Pdcsdcr_SPEC>;

impl Pdcsdcr {
    #[doc = "Channel 0 Sound Detection Enable Bit"]
    #[inline(always)]
    pub fn sde0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdcsdcr::Sde0,
        pdcsdcr::Sde0,
        Pdcsdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdcsdcr::Sde0,
            pdcsdcr::Sde0,
            Pdcsdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 Sound Detection Enable Bit"]
    #[inline(always)]
    pub fn sde1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdcsdcr::Sde1,
        pdcsdcr::Sde1,
        Pdcsdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdcsdcr::Sde1,
            pdcsdcr::Sde1,
            Pdcsdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 Sound Detection Enable Bit"]
    #[inline(always)]
    pub fn sde2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pdcsdcr::Sde2,
        pdcsdcr::Sde2,
        Pdcsdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pdcsdcr::Sde2,
            pdcsdcr::Sde2,
            Pdcsdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdcsdcr {
    #[inline(always)]
    fn default() -> Pdcsdcr {
        <crate::RegValueT<Pdcsdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdcsdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sde0_SPEC;
    pub type Sde0 = crate::EnumBitfieldStruct<u8, Sde0_SPEC>;
    impl Sde0 {
        #[doc = "Do not allow sound detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow sound detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sde1_SPEC;
    pub type Sde1 = crate::EnumBitfieldStruct<u8, Sde1_SPEC>;
    impl Sde1 {
        #[doc = "Do not allow sound detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow sound detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sde2_SPEC;
    pub type Sde2 = crate::EnumBitfieldStruct<u8, Sde2_SPEC>;
    impl Sde2 {
        #[doc = "Do not allow sound detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow sound detection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcdrcr_SPEC;
impl crate::sealed::RegSpec for Pdcdrcr_SPEC {
    type DataType = u32;
}

#[doc = "Channel Data Read Control Register"]
pub type Pdcdrcr = crate::RegValueT<Pdcdrcr_SPEC>;

impl Pdcdrcr {
    #[doc = "Channel 0 Data Read Enable Bit"]
    #[inline(always)]
    pub fn datre0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdcdrcr::Datre0,
        pdcdrcr::Datre0,
        Pdcdrcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdcdrcr::Datre0,
            pdcdrcr::Datre0,
            Pdcdrcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 Data Read Enable Bit"]
    #[inline(always)]
    pub fn datre1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdcdrcr::Datre1,
        pdcdrcr::Datre1,
        Pdcdrcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdcdrcr::Datre1,
            pdcdrcr::Datre1,
            Pdcdrcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 Data Read Enable Bit"]
    #[inline(always)]
    pub fn datre2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pdcdrcr::Datre2,
        pdcdrcr::Datre2,
        Pdcdrcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pdcdrcr::Datre2,
            pdcdrcr::Datre2,
            Pdcdrcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdcdrcr {
    #[inline(always)]
    fn default() -> Pdcdrcr {
        <crate::RegValueT<Pdcdrcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdcdrcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Datre0_SPEC;
    pub type Datre0 = crate::EnumBitfieldStruct<u8, Datre0_SPEC>;
    impl Datre0 {
        #[doc = "Do not allow reading data from data buffer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow reading data from data buffer"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Datre1_SPEC;
    pub type Datre1 = crate::EnumBitfieldStruct<u8, Datre1_SPEC>;
    impl Datre1 {
        #[doc = "Do not allow reading data from data buffer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow reading data from data buffer"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Datre2_SPEC;
    pub type Datre2 = crate::EnumBitfieldStruct<u8, Datre2_SPEC>;
    impl Datre2 {
        #[doc = "Do not allow reading data from data buffer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow reading data from data buffer"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcdcr_SPEC;
impl crate::sealed::RegSpec for Pdcdcr_SPEC {
    type DataType = u32;
}

#[doc = "Channel Data Clear Register"]
pub type Pdcdcr = crate::RegValueT<Pdcdcr_SPEC>;

impl Pdcdcr {
    #[doc = "Channel 0 Data Clear"]
    #[inline(always)]
    pub fn datc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdcdcr::Datc0,
        pdcdcr::Datc0,
        Pdcdcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdcdcr::Datc0,
            pdcdcr::Datc0,
            Pdcdcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 Data Clear"]
    #[inline(always)]
    pub fn datc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdcdcr::Datc1,
        pdcdcr::Datc1,
        Pdcdcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdcdcr::Datc1,
            pdcdcr::Datc1,
            Pdcdcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 Data Clear"]
    #[inline(always)]
    pub fn datc2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pdcdcr::Datc2,
        pdcdcr::Datc2,
        Pdcdcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pdcdcr::Datc2,
            pdcdcr::Datc2,
            Pdcdcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdcdcr {
    #[inline(always)]
    fn default() -> Pdcdcr {
        <crate::RegValueT<Pdcdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdcdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Datc0_SPEC;
    pub type Datc0 = crate::EnumBitfieldStruct<u8, Datc0_SPEC>;
    impl Datc0 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Datc1_SPEC;
    pub type Datc1 = crate::EnumBitfieldStruct<u8, Datc1_SPEC>;
    impl Datc1 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Datc2_SPEC;
    pub type Datc2 = crate::EnumBitfieldStruct<u8, Datc2_SPEC>;
    impl Datc2 {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear data"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdvr_SPEC;
impl crate::sealed::RegSpec for Pdvr_SPEC {
    type DataType = u32;
}

#[doc = "Version Register"]
pub type Pdvr = crate::RegValueT<Pdvr_SPEC>;

impl Pdvr {
    #[doc = "PDM-IF version is shown."]
    #[inline(always)]
    pub fn ver(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Pdvr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Pdvr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdvr {
    #[inline(always)]
    fn default() -> Pdvr {
        <crate::RegValueT<Pdvr_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdstrtrch_SPEC;
impl crate::sealed::RegSpec for Pdstrtrch_SPEC {
    type DataType = u32;
}

#[doc = "Software Start Trigger Register Channel %s"]
pub type Pdstrtrch = crate::RegValueT<Pdstrtrch_SPEC>;

impl Pdstrtrch {
    #[doc = "Start Trigger"]
    #[inline(always)]
    pub fn strtrg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdstrtrch::Strtrg,
        pdstrtrch::Strtrg,
        Pdstrtrch_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdstrtrch::Strtrg,
            pdstrtrch::Strtrg,
            Pdstrtrch_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdstrtrch {
    #[inline(always)]
    fn default() -> Pdstrtrch {
        <crate::RegValueT<Pdstrtrch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdstrtrch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Strtrg_SPEC;
    pub type Strtrg = crate::EnumBitfieldStruct<u8, Strtrg_SPEC>;
    impl Strtrg {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start the channel"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdstptrch_SPEC;
impl crate::sealed::RegSpec for Pdstptrch_SPEC {
    type DataType = u32;
}

#[doc = "Software Stop Trigger Register Channel %s"]
pub type Pdstptrch = crate::RegValueT<Pdstptrch_SPEC>;

impl Pdstptrch {
    #[doc = "Stop Trigger"]
    #[inline(always)]
    pub fn stptrg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdstptrch::Stptrg,
        pdstptrch::Stptrg,
        Pdstptrch_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdstptrch::Stptrg,
            pdstptrch::Stptrg,
            Pdstptrch_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdstptrch {
    #[inline(always)]
    fn default() -> Pdstptrch {
        <crate::RegValueT<Pdstptrch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdstptrch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stptrg_SPEC;
    pub type Stptrg = crate::EnumBitfieldStruct<u8, Stptrg_SPEC>;
    impl Stptrg {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the channel"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdchgtrch_SPEC;
impl crate::sealed::RegSpec for Pdchgtrch_SPEC {
    type DataType = u32;
}

#[doc = "Software Change Trigger Register Channel %s"]
pub type Pdchgtrch = crate::RegValueT<Pdchgtrch_SPEC>;

impl Pdchgtrch {
    #[doc = "Change Trigger"]
    #[inline(always)]
    pub fn chgtrg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdchgtrch::Chgtrg,
        pdchgtrch::Chgtrg,
        Pdchgtrch_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdchgtrch::Chgtrg,
            pdchgtrch::Chgtrg,
            Pdchgtrch_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdchgtrch {
    #[inline(always)]
    fn default() -> Pdchgtrch {
        <crate::RegValueT<Pdchgtrch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdchgtrch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chgtrg_SPEC;
    pub type Chgtrg = crate::EnumBitfieldStruct<u8, Chgtrg_SPEC>;
    impl Chgtrg {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Change settings of PDM_CLKn and sinc filter"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdicrch_SPEC;
impl crate::sealed::RegSpec for Pdicrch_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Control Register Channel %s"]
pub type Pdicrch = crate::RegValueT<Pdicrch_SPEC>;

impl Pdicrch {
    #[doc = "Sound Detection Interrupt Enable Bit"]
    #[inline(always)]
    pub fn isde(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdicrch::Isde,
        pdicrch::Isde,
        Pdicrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdicrch::Isde,
            pdicrch::Isde,
            Pdicrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Reception Interrupt Enable Bit"]
    #[inline(always)]
    pub fn idre(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pdicrch::Idre,
        pdicrch::Idre,
        Pdicrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pdicrch::Idre,
            pdicrch::Idre,
            Pdicrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Error Detection Interrupt Enable Bit"]
    #[inline(always)]
    pub fn iede(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        pdicrch::Iede,
        pdicrch::Iede,
        Pdicrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pdicrch::Iede,
            pdicrch::Iede,
            Pdicrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdicrch {
    #[inline(always)]
    fn default() -> Pdicrch {
        <crate::RegValueT<Pdicrch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdicrch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isde_SPEC;
    pub type Isde = crate::EnumBitfieldStruct<u8, Isde_SPEC>;
    impl Isde {
        #[doc = "Do not allow to issue PDM_SDET interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow to issue PDM_SDET interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idre_SPEC;
    pub type Idre = crate::EnumBitfieldStruct<u8, Idre_SPEC>;
    impl Idre {
        #[doc = "Do not allow to issue PDM_DATn interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow to issue PDM_DATn interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iede_SPEC;
    pub type Iede = crate::EnumBitfieldStruct<u8, Iede_SPEC>;
    impl Iede {
        #[doc = "Do not allow to issue PDM_ERRn interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow to issue PDM_ERRn interrupt"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdsdcrch_SPEC;
impl crate::sealed::RegSpec for Pdsdcrch_SPEC {
    type DataType = u32;
}

#[doc = "Status Detection Control Register Channel %s"]
pub type Pdsdcrch = crate::RegValueT<Pdsdcrch_SPEC>;

impl Pdsdcrch {
    #[doc = "Sound Detection Enable Bit"]
    #[inline(always)]
    pub fn sde(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdsdcrch::Sde,
        pdsdcrch::Sde,
        Pdsdcrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdsdcrch::Sde,
            pdsdcrch::Sde,
            Pdsdcrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Short Circuit Detection Enable Bit"]
    #[inline(always)]
    pub fn scde(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        pdsdcrch::Scde,
        pdsdcrch::Scde,
        Pdsdcrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pdsdcrch::Scde,
            pdsdcrch::Scde,
            Pdsdcrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Overvoltage Lower Limit Exceeded Detection Enable Bit"]
    #[inline(always)]
    pub fn ovlde(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        pdsdcrch::Ovlde,
        pdsdcrch::Ovlde,
        Pdsdcrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            pdsdcrch::Ovlde,
            pdsdcrch::Ovlde,
            Pdsdcrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Overvoltage Upper Limit Exceeded Detection Enable Bit"]
    #[inline(always)]
    pub fn ovude(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        pdsdcrch::Ovude,
        pdsdcrch::Ovude,
        Pdsdcrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            pdsdcrch::Ovude,
            pdsdcrch::Ovude,
            Pdsdcrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Buffer Overwriting Detection Enable Bit"]
    #[inline(always)]
    pub fn bfowde(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        pdsdcrch::Bfowde,
        pdsdcrch::Bfowde,
        Pdsdcrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            pdsdcrch::Bfowde,
            pdsdcrch::Bfowde,
            Pdsdcrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdsdcrch {
    #[inline(always)]
    fn default() -> Pdsdcrch {
        <crate::RegValueT<Pdsdcrch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdsdcrch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sde_SPEC;
    pub type Sde = crate::EnumBitfieldStruct<u8, Sde_SPEC>;
    impl Sde {
        #[doc = "Do not allow sound detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow sound detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scde_SPEC;
    pub type Scde = crate::EnumBitfieldStruct<u8, Scde_SPEC>;
    impl Scde {
        #[doc = "Do not allow detection of the PDM_DATAn pin short circuit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow detection of the PDM_DATAn pin short circuit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovlde_SPEC;
    pub type Ovlde = crate::EnumBitfieldStruct<u8, Ovlde_SPEC>;
    impl Ovlde {
        #[doc = "Do not allow detection of data fallen lower limit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow detection of data fallen lower limit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovude_SPEC;
    pub type Ovude = crate::EnumBitfieldStruct<u8, Ovude_SPEC>;
    impl Ovude {
        #[doc = "Do not allow detection of data exceeded upper limit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow detection of data exceeded upper limit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfowde_SPEC;
    pub type Bfowde = crate::EnumBitfieldStruct<u8, Bfowde_SPEC>;
    impl Bfowde {
        #[doc = "Do not allow detection of buffer overwriting"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow detection of buffer overwriting"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdsrch_SPEC;
impl crate::sealed::RegSpec for Pdsrch_SPEC {
    type DataType = u32;
}

#[doc = "Status Register Channel %s"]
pub type Pdsrch = crate::RegValueT<Pdsrch_SPEC>;

impl Pdsrch {
    #[doc = "State"]
    #[inline(always)]
    pub fn state(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdsrch::State,
        pdsrch::State,
        Pdsrch_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdsrch::State,
            pdsrch::State,
            Pdsrch_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Sound Detection Flag"]
    #[inline(always)]
    pub fn sdf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdsrch::Sdf,
        pdsrch::Sdf,
        Pdsrch_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdsrch::Sdf,
            pdsrch::Sdf,
            Pdsrch_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Reception Flag"]
    #[inline(always)]
    pub fn drf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pdsrch::Drf,
        pdsrch::Drf,
        Pdsrch_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pdsrch::Drf,
            pdsrch::Drf,
            Pdsrch_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Short circuit detection flag."]
    #[inline(always)]
    pub fn scdf(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        pdsrch::Scdf,
        pdsrch::Scdf,
        Pdsrch_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pdsrch::Scdf,
            pdsrch::Scdf,
            Pdsrch_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Overvoltage Lower Limit Exceeded Detection Flag"]
    #[inline(always)]
    pub fn ovldf(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        pdsrch::Ovldf,
        pdsrch::Ovldf,
        Pdsrch_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            pdsrch::Ovldf,
            pdsrch::Ovldf,
            Pdsrch_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Overvoltage Upper Limit Exceeded Detection Flag"]
    #[inline(always)]
    pub fn ovudf(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        pdsrch::Ovudf,
        pdsrch::Ovudf,
        Pdsrch_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            pdsrch::Ovudf,
            pdsrch::Ovudf,
            Pdsrch_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Buffer Overwriting Detection Flag"]
    #[inline(always)]
    pub fn bfowdf(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        pdsrch::Bfowdf,
        pdsrch::Bfowdf,
        Pdsrch_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            pdsrch::Bfowdf,
            pdsrch::Bfowdf,
            Pdsrch_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdsrch {
    #[inline(always)]
    fn default() -> Pdsrch {
        <crate::RegValueT<Pdsrch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdsrch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct State_SPEC;
    pub type State = crate::EnumBitfieldStruct<u8, State_SPEC>;
    impl State {
        #[doc = "Channel stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "Channel in operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdf_SPEC;
    pub type Sdf = crate::EnumBitfieldStruct<u8, Sdf_SPEC>;
    impl Sdf {
        #[doc = "Indicates that the sound is not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that the sound is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drf_SPEC;
    pub type Drf = crate::EnumBitfieldStruct<u8, Drf_SPEC>;
    impl Drf {
        #[doc = "Indicates that the number of data stored in buffer is lower than a threshold"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that the number of data stored in buffer is higher than or equal to a threshold"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scdf_SPEC;
    pub type Scdf = crate::EnumBitfieldStruct<u8, Scdf_SPEC>;
    impl Scdf {
        #[doc = "Indicates that the PDM_DATAn pin short circuit is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that a short circuit on the PDM_DATAn pin has been detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovldf_SPEC;
    pub type Ovldf = crate::EnumBitfieldStruct<u8, Ovldf_SPEC>;
    impl Ovldf {
        #[doc = "Indicates that the data is not lower than the lower limit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that the data has fallen the lower limit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovudf_SPEC;
    pub type Ovudf = crate::EnumBitfieldStruct<u8, Ovudf_SPEC>;
    impl Ovudf {
        #[doc = "Indicates that the data has not exceeded the upper limit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that the data has exceeded the upper limit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfowdf_SPEC;
    pub type Bfowdf = crate::EnumBitfieldStruct<u8, Bfowdf_SPEC>;
    impl Bfowdf {
        #[doc = "Indicates that buffer overwriting does not occur"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that buffer overwriting occurs"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdscrch_SPEC;
impl crate::sealed::RegSpec for Pdscrch_SPEC {
    type DataType = u32;
}

#[doc = "Status Clear Register Channel %s"]
pub type Pdscrch = crate::RegValueT<Pdscrch_SPEC>;

impl Pdscrch {
    #[doc = "Sound Detection Flag Clear"]
    #[inline(always)]
    pub fn sdfc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdscrch::Sdfc,
        pdscrch::Sdfc,
        Pdscrch_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdscrch::Sdfc,
            pdscrch::Sdfc,
            Pdscrch_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Short Circuit Detection Flag Clear"]
    #[inline(always)]
    pub fn scdfc(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        pdscrch::Scdfc,
        pdscrch::Scdfc,
        Pdscrch_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pdscrch::Scdfc,
            pdscrch::Scdfc,
            Pdscrch_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Overvoltage Lower Limit Exceeded Detection Flag Clear"]
    #[inline(always)]
    pub fn ovldfc(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        pdscrch::Ovldfc,
        pdscrch::Ovldfc,
        Pdscrch_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            pdscrch::Ovldfc,
            pdscrch::Ovldfc,
            Pdscrch_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Overvoltage Upper Limit Exceeded Detection Flag Clear"]
    #[inline(always)]
    pub fn ovudfc(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        pdscrch::Ovudfc,
        pdscrch::Ovudfc,
        Pdscrch_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            pdscrch::Ovudfc,
            pdscrch::Ovudfc,
            Pdscrch_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Buffer Overwriting Detection Flag Clear"]
    #[inline(always)]
    pub fn bfowdfc(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        pdscrch::Bfowdfc,
        pdscrch::Bfowdfc,
        Pdscrch_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            pdscrch::Bfowdfc,
            pdscrch::Bfowdfc,
            Pdscrch_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdscrch {
    #[inline(always)]
    fn default() -> Pdscrch {
        <crate::RegValueT<Pdscrch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdscrch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdfc_SPEC;
    pub type Sdfc = crate::EnumBitfieldStruct<u8, Sdfc_SPEC>;
    impl Sdfc {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear PDSRCHn.SDF"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scdfc_SPEC;
    pub type Scdfc = crate::EnumBitfieldStruct<u8, Scdfc_SPEC>;
    impl Scdfc {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear PDSRCHn.SCDF"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovldfc_SPEC;
    pub type Ovldfc = crate::EnumBitfieldStruct<u8, Ovldfc_SPEC>;
    impl Ovldfc {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear PDSRCHn.OVLDF"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovudfc_SPEC;
    pub type Ovudfc = crate::EnumBitfieldStruct<u8, Ovudfc_SPEC>;
    impl Ovudfc {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear PDSRCHn.OVUDF"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfowdfc_SPEC;
    pub type Bfowdfc = crate::EnumBitfieldStruct<u8, Bfowdfc_SPEC>;
    impl Bfowdfc {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear PDSRCHn.BFOWDF"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdmdsrch_SPEC;
impl crate::sealed::RegSpec for Pdmdsrch_SPEC {
    type DataType = u32;
}

#[doc = "Mode Setting Register Channel %s"]
pub type Pdmdsrch = crate::RegValueT<Pdmdsrch_SPEC>;

impl Pdmdsrch {
    #[doc = "Input Data Select"]
    #[inline(always)]
    pub fn inpsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdmdsrch::Inpsel,
        pdmdsrch::Inpsel,
        Pdmdsrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdmdsrch::Inpsel,
            pdmdsrch::Inpsel,
            Pdmdsrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sinc Filter Mode Setting"]
    #[inline(always)]
    pub fn sfmd(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        pdmdsrch::Sfmd,
        pdmdsrch::Sfmd,
        Pdmdsrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            pdmdsrch::Sfmd,
            pdmdsrch::Sfmd,
            Pdmdsrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "High-pass Filter Input Shift Setting"]
    #[inline(always)]
    pub fn hfis(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        pdmdsrch::Hfis,
        pdmdsrch::Hfis,
        Pdmdsrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            pdmdsrch::Hfis,
            pdmdsrch::Hfis,
            Pdmdsrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Compensation Filter Input Shift Setting"]
    #[inline(always)]
    pub fn cfis(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        pdmdsrch::Cfis,
        pdmdsrch::Cfis,
        Pdmdsrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pdmdsrch::Cfis,
            pdmdsrch::Cfis,
            Pdmdsrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Low-pass (half-band decimation) Filter Input Shift Setting"]
    #[inline(always)]
    pub fn lfis(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        pdmdsrch::Lfis,
        pdmdsrch::Lfis,
        Pdmdsrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            pdmdsrch::Lfis,
            pdmdsrch::Lfis,
            Pdmdsrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Moving Average Mode for Sound Detection Data"]
    #[inline(always)]
    pub fn sdmamd(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        pdmdsrch::Sdmamd,
        pdmdsrch::Sdmamd,
        Pdmdsrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            pdmdsrch::Sdmamd,
            pdmdsrch::Sdmamd,
            Pdmdsrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Buffer Input Shift Setting"]
    #[inline(always)]
    pub fn dbis(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        pdmdsrch::Dbis,
        pdmdsrch::Dbis,
        Pdmdsrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            pdmdsrch::Dbis,
            pdmdsrch::Dbis,
            Pdmdsrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdmdsrch {
    #[inline(always)]
    fn default() -> Pdmdsrch {
        <crate::RegValueT<Pdmdsrch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdmdsrch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inpsel_SPEC;
    pub type Inpsel = crate::EnumBitfieldStruct<u8, Inpsel_SPEC>;
    impl Inpsel {
        #[doc = "Rise-edge data of channel n."]
        pub const _0: Self = Self::new(0);

        #[doc = "Fall-edge data of channel n-1. When n = 0, fall-edge data of channel 2 is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfmd_SPEC;
    pub type Sfmd = crate::EnumBitfieldStruct<u8, Sfmd_SPEC>;
    impl Sfmd {
        #[doc = "1-order"]
        pub const _001: Self = Self::new(1);

        #[doc = "2-order"]
        pub const _010: Self = Self::new(2);

        #[doc = "3-order"]
        pub const _011: Self = Self::new(3);

        #[doc = "4-order (default)"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hfis_SPEC;
    pub type Hfis = crate::EnumBitfieldStruct<u8, Hfis_SPEC>;
    impl Hfis {
        #[doc = "No shift"]
        pub const _00: Self = Self::new(0);

        #[doc = "1-bit right shift"]
        pub const _01: Self = Self::new(1);

        #[doc = "2-bit right shift"]
        pub const _10: Self = Self::new(2);

        #[doc = "3-bit right shift"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfis_SPEC;
    pub type Cfis = crate::EnumBitfieldStruct<u8, Cfis_SPEC>;
    impl Cfis {
        #[doc = "No shift"]
        pub const _00: Self = Self::new(0);

        #[doc = "1-bit right shift"]
        pub const _01: Self = Self::new(1);

        #[doc = "2-bit right shift"]
        pub const _10: Self = Self::new(2);

        #[doc = "3-bit right shift"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lfis_SPEC;
    pub type Lfis = crate::EnumBitfieldStruct<u8, Lfis_SPEC>;
    impl Lfis {
        #[doc = "No shift"]
        pub const _00: Self = Self::new(0);

        #[doc = "1-bit right shift"]
        pub const _01: Self = Self::new(1);

        #[doc = "2-bit right shift"]
        pub const _10: Self = Self::new(2);

        #[doc = "3-bit right shift"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdmamd_SPEC;
    pub type Sdmamd = crate::EnumBitfieldStruct<u8, Sdmamd_SPEC>;
    impl Sdmamd {
        #[doc = "1-order (filter is skipped) (default)"]
        pub const _00: Self = Self::new(0);

        #[doc = "2-order"]
        pub const _01: Self = Self::new(1);

        #[doc = "4-order"]
        pub const _10: Self = Self::new(2);

        #[doc = "User prohibition"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbis_SPEC;
    pub type Dbis = crate::EnumBitfieldStruct<u8, Dbis_SPEC>;
    impl Dbis {
        #[doc = "20-bit mode, {1{S}, \\[18:0\\]}"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "20-bit mode, {2{S}, \\[18:1\\]}"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "20-bit mode, {3{S}, \\[18:2\\]}"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "20-bit mode, {4{S}, \\[18:3\\]}"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "16-bit mode, {S,D\\[18:4\\]}"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "16-bit mode, {S,D\\[17:3\\]}"]
        pub const _0_X_9: Self = Self::new(9);

        #[doc = "16-bit mode, {S,D\\[16:2\\]}"]
        pub const _0_X_A: Self = Self::new(10);

        #[doc = "16-bit mode, {S,D\\[15:1\\]}"]
        pub const _0_X_B: Self = Self::new(11);

        #[doc = "16-bit mode, {S,D\\[14:0\\]}"]
        pub const _0_X_C: Self = Self::new(12);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdsfcrch_SPEC;
impl crate::sealed::RegSpec for Pdsfcrch_SPEC {
    type DataType = u32;
}

#[doc = "Sinc Filter Control Register Channel %s"]
pub type Pdsfcrch = crate::RegValueT<Pdsfcrch_SPEC>;

impl Pdsfcrch {
    #[doc = "PDM_CLKn Dividend Ratio to Core Clock"]
    #[inline(always)]
    pub fn ckdiv(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Pdsfcrch_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Pdsfcrch_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sinc Filter Decimation Ratio"]
    #[inline(always)]
    pub fn sincdec(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Pdsfcrch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Pdsfcrch_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sinc Filter Output Valid Range"]
    #[inline(always)]
    pub fn sincrng(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1f,
        1,
        0,
        pdsfcrch::Sincrng,
        pdsfcrch::Sincrng,
        Pdsfcrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1f,
            1,
            0,
            pdsfcrch::Sincrng,
            pdsfcrch::Sincrng,
            Pdsfcrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdsfcrch {
    #[inline(always)]
    fn default() -> Pdsfcrch {
        <crate::RegValueT<Pdsfcrch_SPEC> as RegisterValue<_>>::new(92012544)
    }
}
pub mod pdsfcrch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sincrng_SPEC;
    pub type Sincrng = crate::EnumBitfieldStruct<u8, Sincrng_SPEC>;
    impl Sincrng {
        #[doc = "{S, \\[32:14\\]}"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "{S, \\[31:13\\]}"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "{S, \\[30:12\\]}"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "{S, \\[29:11\\]}"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "{S, \\[28:10\\]}"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "{S, \\[27:9\\]} (default)"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "{S, \\[26:8\\]}"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "{S, \\[25:7\\]}"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "{S, \\[24:6\\]}"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "{S, \\[23:5\\]}"]
        pub const _0_X_09: Self = Self::new(9);

        #[doc = "{S, \\[22:4\\]}"]
        pub const _0_X_0_A: Self = Self::new(10);

        #[doc = "{S, \\[21:3\\]}"]
        pub const _0_X_0_B: Self = Self::new(11);

        #[doc = "{S, \\[20:2\\]}"]
        pub const _0_X_0_C: Self = Self::new(12);

        #[doc = "{S, \\[19:1\\]}"]
        pub const _0_X_0_D: Self = Self::new(13);

        #[doc = "{S, \\[18:0\\]}"]
        pub const _0_X_0_E: Self = Self::new(14);

        #[doc = "{S, \\[17:0\\], 0}"]
        pub const _0_X_0_F: Self = Self::new(15);

        #[doc = "{S, \\[16:0\\], 00}"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "{S, \\[15:0\\], 000}"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "{S, \\[14:0\\], 0000}"]
        pub const _0_X_12: Self = Self::new(18);

        #[doc = "{S, \\[13:0\\], 00000}"]
        pub const _0_X_13: Self = Self::new(19);

        #[doc = "{S, \\[12:0\\], 000000}"]
        pub const _0_X_14: Self = Self::new(20);

        #[doc = "{S, \\[11:0\\], 0000000}"]
        pub const _0_X_15: Self = Self::new(21);

        #[doc = "{S, \\[10:0\\], 00000000}"]
        pub const _0_X_16: Self = Self::new(22);

        #[doc = "{S, \\[9:0\\], 000000000}"]
        pub const _0_X_17: Self = Self::new(23);

        #[doc = "{S, \\[8:0\\], 0000000000}"]
        pub const _0_X_18: Self = Self::new(24);

        #[doc = "{S, \\[7:0\\], 00000000000}"]
        pub const _0_X_19: Self = Self::new(25);

        #[doc = "{S, \\[6:0\\], 000000000000}"]
        pub const _0_X_1_A: Self = Self::new(26);

        #[doc = "{S, \\[5:0\\], 0000000000000}"]
        pub const _0_X_1_B: Self = Self::new(27);

        #[doc = "{S, \\[4:0\\], 00000000000000}"]
        pub const _0_X_1_C: Self = Self::new(28);

        #[doc = "{S, \\[3:0\\], 000000000000000}"]
        pub const _0_X_1_D: Self = Self::new(29);

        #[doc = "{S, \\[2:0\\], 0000000000000000}"]
        pub const _0_X_1_E: Self = Self::new(30);

        #[doc = "{S, \\[1:0\\], 00000000000000000} (prohibited)"]
        pub const _0_X_1_F: Self = Self::new(31);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdhfcs0Rch_SPEC;
impl crate::sealed::RegSpec for Pdhfcs0Rch_SPEC {
    type DataType = u32;
}

#[doc = "High-pass Filter Coefficient s(0) Register Channel %s"]
pub type Pdhfcs0Rch = crate::RegValueT<Pdhfcs0Rch_SPEC>;

impl Pdhfcs0Rch {
    #[doc = "High-pass Filter Coefficient s(0)"]
    #[inline(always)]
    pub fn hfs(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Pdhfcs0Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Pdhfcs0Rch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdhfcs0Rch {
    #[inline(always)]
    fn default() -> Pdhfcs0Rch {
        <crate::RegValueT<Pdhfcs0Rch_SPEC> as RegisterValue<_>>::new(16225)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdhfck1Rch_SPEC;
impl crate::sealed::RegSpec for Pdhfck1Rch_SPEC {
    type DataType = u32;
}

#[doc = "High-pass Filter Coefficient k(1) Register Channel %s"]
pub type Pdhfck1Rch = crate::RegValueT<Pdhfck1Rch_SPEC>;

impl Pdhfck1Rch {
    #[doc = "High-pass Filter Coefficient k(1)"]
    #[inline(always)]
    pub fn hfk(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Pdhfck1Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Pdhfck1Rch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdhfck1Rch {
    #[inline(always)]
    fn default() -> Pdhfck1Rch {
        <crate::RegValueT<Pdhfck1Rch_SPEC> as RegisterValue<_>>::new(16065)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdhfch0Rch_SPEC;
impl crate::sealed::RegSpec for Pdhfch0Rch_SPEC {
    type DataType = u32;
}

#[doc = "High-pass Filter Coefficient h(0) Register Channel %s"]
pub type Pdhfch0Rch = crate::RegValueT<Pdhfch0Rch_SPEC>;

impl Pdhfch0Rch {
    #[doc = "High-pass Filter Coefficient h(0)"]
    #[inline(always)]
    pub fn hfh(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Pdhfch0Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Pdhfch0Rch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdhfch0Rch {
    #[inline(always)]
    fn default() -> Pdhfch0Rch {
        <crate::RegValueT<Pdhfch0Rch_SPEC> as RegisterValue<_>>::new(16384)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdhfch1Rch_SPEC;
impl crate::sealed::RegSpec for Pdhfch1Rch_SPEC {
    type DataType = u32;
}

#[doc = "High-pass Filter Coefficient h(1) Register Channel %s"]
pub type Pdhfch1Rch = crate::RegValueT<Pdhfch1Rch_SPEC>;

impl Pdhfch1Rch {
    #[doc = "High-pass Filter Coefficient h(1)"]
    #[inline(always)]
    pub fn hfh(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Pdhfch1Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Pdhfch1Rch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdhfch1Rch {
    #[inline(always)]
    fn default() -> Pdhfch1Rch {
        <crate::RegValueT<Pdhfch1Rch_SPEC> as RegisterValue<_>>::new(49152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcfch00Rch_SPEC;
impl crate::sealed::RegSpec for Pdcfch00Rch_SPEC {
    type DataType = u32;
}

#[doc = "Compensation Filter Coefficient h(0) Register Channel %s"]
pub type Pdcfch00Rch = crate::RegValueT<Pdcfch00Rch_SPEC>;

impl Pdcfch00Rch {
    #[doc = "Compensation Filter Coefficients h(0)"]
    #[inline(always)]
    pub fn cfh(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdcfch00Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Pdcfch00Rch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdcfch00Rch {
    #[inline(always)]
    fn default() -> Pdcfch00Rch {
        <crate::RegValueT<Pdcfch00Rch_SPEC> as RegisterValue<_>>::new(8168)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcfch01Rch_SPEC;
impl crate::sealed::RegSpec for Pdcfch01Rch_SPEC {
    type DataType = u32;
}

#[doc = "Compensation Filter Coefficient h(1) Register Channel %s"]
pub type Pdcfch01Rch = crate::RegValueT<Pdcfch01Rch_SPEC>;

impl Pdcfch01Rch {
    #[doc = "Compensation Filter Coefficients h(1)"]
    #[inline(always)]
    pub fn cfh(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdcfch01Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Pdcfch01Rch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdcfch01Rch {
    #[inline(always)]
    fn default() -> Pdcfch01Rch {
        <crate::RegValueT<Pdcfch01Rch_SPEC> as RegisterValue<_>>::new(57)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcfch02Rch_SPEC;
impl crate::sealed::RegSpec for Pdcfch02Rch_SPEC {
    type DataType = u32;
}

#[doc = "Compensation Filter Coefficient h(2) Register Channel %s"]
pub type Pdcfch02Rch = crate::RegValueT<Pdcfch02Rch_SPEC>;

impl Pdcfch02Rch {
    #[doc = "Compensation Filter Coefficients h(2)"]
    #[inline(always)]
    pub fn cfh(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdcfch02Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Pdcfch02Rch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdcfch02Rch {
    #[inline(always)]
    fn default() -> Pdcfch02Rch {
        <crate::RegValueT<Pdcfch02Rch_SPEC> as RegisterValue<_>>::new(60)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcfch03Rch_SPEC;
impl crate::sealed::RegSpec for Pdcfch03Rch_SPEC {
    type DataType = u32;
}

#[doc = "Compensation Filter Coefficient h(3) Register Channel %s"]
pub type Pdcfch03Rch = crate::RegValueT<Pdcfch03Rch_SPEC>;

impl Pdcfch03Rch {
    #[doc = "Compensation Filter Coefficients h(3)"]
    #[inline(always)]
    pub fn cfh(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdcfch03Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Pdcfch03Rch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdcfch03Rch {
    #[inline(always)]
    fn default() -> Pdcfch03Rch {
        <crate::RegValueT<Pdcfch03Rch_SPEC> as RegisterValue<_>>::new(7766)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcfch04Rch_SPEC;
impl crate::sealed::RegSpec for Pdcfch04Rch_SPEC {
    type DataType = u32;
}

#[doc = "Compensation Filter Coefficient h(4) Register Channel %s"]
pub type Pdcfch04Rch = crate::RegValueT<Pdcfch04Rch_SPEC>;

impl Pdcfch04Rch {
    #[doc = "Compensation Filter Coefficients h(4)"]
    #[inline(always)]
    pub fn cfh(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdcfch04Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Pdcfch04Rch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdcfch04Rch {
    #[inline(always)]
    fn default() -> Pdcfch04Rch {
        <crate::RegValueT<Pdcfch04Rch_SPEC> as RegisterValue<_>>::new(476)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcfch05Rch_SPEC;
impl crate::sealed::RegSpec for Pdcfch05Rch_SPEC {
    type DataType = u32;
}

#[doc = "Compensation Filter Coefficient h(5) Register Channel %s"]
pub type Pdcfch05Rch = crate::RegValueT<Pdcfch05Rch_SPEC>;

impl Pdcfch05Rch {
    #[doc = "Compensation Filter Coefficients h(5)"]
    #[inline(always)]
    pub fn cfh(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdcfch05Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Pdcfch05Rch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdcfch05Rch {
    #[inline(always)]
    fn default() -> Pdcfch05Rch {
        <crate::RegValueT<Pdcfch05Rch_SPEC> as RegisterValue<_>>::new(1761)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcfch06RcHn_SPEC;
impl crate::sealed::RegSpec for Pdcfch06RcHn_SPEC {
    type DataType = u32;
}

#[doc = "Compensation Filter Coefficient h(6) Register Channel n"]
pub type Pdcfch06RcHn = crate::RegValueT<Pdcfch06RcHn_SPEC>;

impl Pdcfch06RcHn {
    #[doc = "Compensation Filter Coefficients h(6)"]
    #[inline(always)]
    pub fn cfh(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdcfch06RcHn_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdcfch06RcHn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdcfch06RcHn {
    #[inline(always)]
    fn default() -> Pdcfch06RcHn {
        <crate::RegValueT<Pdcfch06RcHn_SPEC> as RegisterValue<_>>::new(476)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcfch07Rch_SPEC;
impl crate::sealed::RegSpec for Pdcfch07Rch_SPEC {
    type DataType = u32;
}

#[doc = "Compensation Filter Coefficient h(7) Register Channel %s"]
pub type Pdcfch07Rch = crate::RegValueT<Pdcfch07Rch_SPEC>;

impl Pdcfch07Rch {
    #[doc = "Compensation Filter Coefficients h(7)"]
    #[inline(always)]
    pub fn cfh(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdcfch07Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Pdcfch07Rch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdcfch07Rch {
    #[inline(always)]
    fn default() -> Pdcfch07Rch {
        <crate::RegValueT<Pdcfch07Rch_SPEC> as RegisterValue<_>>::new(7766)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcfch08Rch_SPEC;
impl crate::sealed::RegSpec for Pdcfch08Rch_SPEC {
    type DataType = u32;
}

#[doc = "Compensation Filter Coefficient h(8) Register Channel %s"]
pub type Pdcfch08Rch = crate::RegValueT<Pdcfch08Rch_SPEC>;

impl Pdcfch08Rch {
    #[doc = "Compensation Filter Coefficients h(8)"]
    #[inline(always)]
    pub fn cfh(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdcfch08Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Pdcfch08Rch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdcfch08Rch {
    #[inline(always)]
    fn default() -> Pdcfch08Rch {
        <crate::RegValueT<Pdcfch08Rch_SPEC> as RegisterValue<_>>::new(60)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcfch09Rch_SPEC;
impl crate::sealed::RegSpec for Pdcfch09Rch_SPEC {
    type DataType = u32;
}

#[doc = "Compensation Filter Coefficient h(9) Register Channel %s"]
pub type Pdcfch09Rch = crate::RegValueT<Pdcfch09Rch_SPEC>;

impl Pdcfch09Rch {
    #[doc = "Compensation Filter Coefficients h(9)"]
    #[inline(always)]
    pub fn cfh(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdcfch09Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Pdcfch09Rch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdcfch09Rch {
    #[inline(always)]
    fn default() -> Pdcfch09Rch {
        <crate::RegValueT<Pdcfch09Rch_SPEC> as RegisterValue<_>>::new(57)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcfch10Rch_SPEC;
impl crate::sealed::RegSpec for Pdcfch10Rch_SPEC {
    type DataType = u32;
}

#[doc = "Compensation Filter Coefficient h(10) Register Channel %s"]
pub type Pdcfch10Rch = crate::RegValueT<Pdcfch10Rch_SPEC>;

impl Pdcfch10Rch {
    #[doc = "Compensation Filter Coefficients h(10)"]
    #[inline(always)]
    pub fn cfh(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdcfch10Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Pdcfch10Rch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdcfch10Rch {
    #[inline(always)]
    fn default() -> Pdcfch10Rch {
        <crate::RegValueT<Pdcfch10Rch_SPEC> as RegisterValue<_>>::new(8168)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch010Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch010Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h0(10) Register Channel %s"]
pub type Pdlfch010Rch = crate::RegValueT<Pdlfch010Rch_SPEC>;

impl Pdlfch010Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h0(10)"]
    #[inline(always)]
    pub fn lfh0(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch010Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch010Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch010Rch {
    #[inline(always)]
    fn default() -> Pdlfch010Rch {
        <crate::RegValueT<Pdlfch010Rch_SPEC> as RegisterValue<_>>::new(1024)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch100Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch100Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(0) Register Channel %s"]
pub type Pdlfch100Rch = crate::RegValueT<Pdlfch100Rch_SPEC>;

impl Pdlfch100Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(0)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch100Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch100Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch100Rch {
    #[inline(always)]
    fn default() -> Pdlfch100Rch {
        <crate::RegValueT<Pdlfch100Rch_SPEC> as RegisterValue<_>>::new(8184)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch101Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch101Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(1) Register Channel %s"]
pub type Pdlfch101Rch = crate::RegValueT<Pdlfch101Rch_SPEC>;

impl Pdlfch101Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(1)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch101Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch101Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch101Rch {
    #[inline(always)]
    fn default() -> Pdlfch101Rch {
        <crate::RegValueT<Pdlfch101Rch_SPEC> as RegisterValue<_>>::new(10)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch102Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch102Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(2) Register Channel %s"]
pub type Pdlfch102Rch = crate::RegValueT<Pdlfch102Rch_SPEC>;

impl Pdlfch102Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(2)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch102Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch102Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch102Rch {
    #[inline(always)]
    fn default() -> Pdlfch102Rch {
        <crate::RegValueT<Pdlfch102Rch_SPEC> as RegisterValue<_>>::new(8176)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch103Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch103Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(3) Register Channel %s"]
pub type Pdlfch103Rch = crate::RegValueT<Pdlfch103Rch_SPEC>;

impl Pdlfch103Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(3)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch103Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch103Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch103Rch {
    #[inline(always)]
    fn default() -> Pdlfch103Rch {
        <crate::RegValueT<Pdlfch103Rch_SPEC> as RegisterValue<_>>::new(24)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch104Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch104Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(4) Register Channel %s"]
pub type Pdlfch104Rch = crate::RegValueT<Pdlfch104Rch_SPEC>;

impl Pdlfch104Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(4)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch104Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch104Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch104Rch {
    #[inline(always)]
    fn default() -> Pdlfch104Rch {
        <crate::RegValueT<Pdlfch104Rch_SPEC> as RegisterValue<_>>::new(8156)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch105Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch105Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(5) Register Channel %s"]
pub type Pdlfch105Rch = crate::RegValueT<Pdlfch105Rch_SPEC>;

impl Pdlfch105Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(5)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch105Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch105Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch105Rch {
    #[inline(always)]
    fn default() -> Pdlfch105Rch {
        <crate::RegValueT<Pdlfch105Rch_SPEC> as RegisterValue<_>>::new(52)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch106Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch106Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(6) Register Channel %s"]
pub type Pdlfch106Rch = crate::RegValueT<Pdlfch106Rch_SPEC>;

impl Pdlfch106Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(6)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch106Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch106Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch106Rch {
    #[inline(always)]
    fn default() -> Pdlfch106Rch {
        <crate::RegValueT<Pdlfch106Rch_SPEC> as RegisterValue<_>>::new(8115)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch107Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch107Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(7) Register Channel %s"]
pub type Pdlfch107Rch = crate::RegValueT<Pdlfch107Rch_SPEC>;

impl Pdlfch107Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(7)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch107Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch107Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch107Rch {
    #[inline(always)]
    fn default() -> Pdlfch107Rch {
        <crate::RegValueT<Pdlfch107Rch_SPEC> as RegisterValue<_>>::new(118)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch108Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch108Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(8) Register Channel %s"]
pub type Pdlfch108Rch = crate::RegValueT<Pdlfch108Rch_SPEC>;

impl Pdlfch108Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(8)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch108Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch108Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch108Rch {
    #[inline(always)]
    fn default() -> Pdlfch108Rch {
        <crate::RegValueT<Pdlfch108Rch_SPEC> as RegisterValue<_>>::new(7982)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch109Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch109Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(9) Register Channel %s"]
pub type Pdlfch109Rch = crate::RegValueT<Pdlfch109Rch_SPEC>;

impl Pdlfch109Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(9)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch109Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch109Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch109Rch {
    #[inline(always)]
    fn default() -> Pdlfch109Rch {
        <crate::RegValueT<Pdlfch109Rch_SPEC> as RegisterValue<_>>::new(649)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch110Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch110Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(10) Register Channel %s"]
pub type Pdlfch110Rch = crate::RegValueT<Pdlfch110Rch_SPEC>;

impl Pdlfch110Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(10)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch110Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch110Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch110Rch {
    #[inline(always)]
    fn default() -> Pdlfch110Rch {
        <crate::RegValueT<Pdlfch110Rch_SPEC> as RegisterValue<_>>::new(649)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch111Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch111Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(11) Register Channel %s"]
pub type Pdlfch111Rch = crate::RegValueT<Pdlfch111Rch_SPEC>;

impl Pdlfch111Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(11)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch111Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch111Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch111Rch {
    #[inline(always)]
    fn default() -> Pdlfch111Rch {
        <crate::RegValueT<Pdlfch111Rch_SPEC> as RegisterValue<_>>::new(7982)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch112Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch112Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(12) Register Channel %s"]
pub type Pdlfch112Rch = crate::RegValueT<Pdlfch112Rch_SPEC>;

impl Pdlfch112Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(12)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch112Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch112Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch112Rch {
    #[inline(always)]
    fn default() -> Pdlfch112Rch {
        <crate::RegValueT<Pdlfch112Rch_SPEC> as RegisterValue<_>>::new(118)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch113Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch113Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(13) Register Channel %s"]
pub type Pdlfch113Rch = crate::RegValueT<Pdlfch113Rch_SPEC>;

impl Pdlfch113Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(13)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch113Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch113Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch113Rch {
    #[inline(always)]
    fn default() -> Pdlfch113Rch {
        <crate::RegValueT<Pdlfch113Rch_SPEC> as RegisterValue<_>>::new(8115)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch114Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch114Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(14) Register Channel %s"]
pub type Pdlfch114Rch = crate::RegValueT<Pdlfch114Rch_SPEC>;

impl Pdlfch114Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(14)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch114Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch114Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch114Rch {
    #[inline(always)]
    fn default() -> Pdlfch114Rch {
        <crate::RegValueT<Pdlfch114Rch_SPEC> as RegisterValue<_>>::new(52)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch115Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch115Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(15) Register Channel %s"]
pub type Pdlfch115Rch = crate::RegValueT<Pdlfch115Rch_SPEC>;

impl Pdlfch115Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(15)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch115Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch115Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch115Rch {
    #[inline(always)]
    fn default() -> Pdlfch115Rch {
        <crate::RegValueT<Pdlfch115Rch_SPEC> as RegisterValue<_>>::new(8156)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch116Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch116Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(16) Register Channel %s"]
pub type Pdlfch116Rch = crate::RegValueT<Pdlfch116Rch_SPEC>;

impl Pdlfch116Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(15)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch116Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch116Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch116Rch {
    #[inline(always)]
    fn default() -> Pdlfch116Rch {
        <crate::RegValueT<Pdlfch116Rch_SPEC> as RegisterValue<_>>::new(24)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch117Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch117Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(17) Register Channel %s"]
pub type Pdlfch117Rch = crate::RegValueT<Pdlfch117Rch_SPEC>;

impl Pdlfch117Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(17)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch117Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch117Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch117Rch {
    #[inline(always)]
    fn default() -> Pdlfch117Rch {
        <crate::RegValueT<Pdlfch117Rch_SPEC> as RegisterValue<_>>::new(8176)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch118Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch118Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(18) Register Channel %s"]
pub type Pdlfch118Rch = crate::RegValueT<Pdlfch118Rch_SPEC>;

impl Pdlfch118Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(18)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch118Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch118Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch118Rch {
    #[inline(always)]
    fn default() -> Pdlfch118Rch {
        <crate::RegValueT<Pdlfch118Rch_SPEC> as RegisterValue<_>>::new(10)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdlfch119Rch_SPEC;
impl crate::sealed::RegSpec for Pdlfch119Rch_SPEC {
    type DataType = u32;
}

#[doc = "Low-pass Filter Coefficient h1(19) Register Channel %s"]
pub type Pdlfch119Rch = crate::RegValueT<Pdlfch119Rch_SPEC>;

impl Pdlfch119Rch {
    #[doc = "Low-pass (half-band decimation) Filter Coefficient h1(19)"]
    #[inline(always)]
    pub fn lfh1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdlfch119Rch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            Pdlfch119Rch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdlfch119Rch {
    #[inline(always)]
    fn default() -> Pdlfch119Rch {
        <crate::RegValueT<Pdlfch119Rch_SPEC> as RegisterValue<_>>::new(8184)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdsdltrch_SPEC;
impl crate::sealed::RegSpec for Pdsdltrch_SPEC {
    type DataType = u32;
}

#[doc = "Sound Detection Lower Threshold Register Channel %s"]
pub type Pdsdltrch = crate::RegValueT<Pdsdltrch_SPEC>;

impl Pdsdltrch {
    #[doc = "Sound Detection Lower Limit"]
    #[inline(always)]
    pub fn sdetl(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, u32, Pdsdltrch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfffff,1,0,u32,u32,Pdsdltrch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdsdltrch {
    #[inline(always)]
    fn default() -> Pdsdltrch {
        <crate::RegValueT<Pdsdltrch_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdsdutrch_SPEC;
impl crate::sealed::RegSpec for Pdsdutrch_SPEC {
    type DataType = u32;
}

#[doc = "Sound Detection Upper Threshold Register Channel %s"]
pub type Pdsdutrch = crate::RegValueT<Pdsdutrch_SPEC>;

impl Pdsdutrch {
    #[doc = "Sound Detection Upper Limit"]
    #[inline(always)]
    pub fn sdetu(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, u32, Pdsdutrch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfffff,1,0,u32,u32,Pdsdutrch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdsdutrch {
    #[inline(always)]
    fn default() -> Pdsdutrch {
        <crate::RegValueT<Pdsdutrch_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pddbcrch_SPEC;
impl crate::sealed::RegSpec for Pddbcrch_SPEC {
    type DataType = u32;
}

#[doc = "Data Buffer Control Register Channel %s"]
pub type Pddbcrch = crate::RegValueT<Pddbcrch_SPEC>;

impl Pddbcrch {
    #[doc = "Data Reception Interrupt Threshold"]
    #[inline(always)]
    pub fn datrithr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        pddbcrch::Datrithr,
        pddbcrch::Datrithr,
        Pddbcrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            pddbcrch::Datrithr,
            pddbcrch::Datrithr,
            Pddbcrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pddbcrch {
    #[inline(always)]
    fn default() -> Pddbcrch {
        <crate::RegValueT<Pddbcrch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pddbcrch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Datrithr_SPEC;
    pub type Datrithr = crate::EnumBitfieldStruct<u8, Datrithr_SPEC>;
    impl Datrithr {
        #[doc = "Output interrupt when receiving 1 or more data"]
        pub const _000: Self = Self::new(0);

        #[doc = "Output interrupt when receiving 2 or more data"]
        pub const _001: Self = Self::new(1);

        #[doc = "Output interrupt when receiving 4 or more data"]
        pub const _010: Self = Self::new(2);

        #[doc = "Output interrupt when receiving 8 or more data"]
        pub const _011: Self = Self::new(3);

        #[doc = "Output interrupt when receiving 16 or more data"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdsctsrch_SPEC;
impl crate::sealed::RegSpec for Pdsctsrch_SPEC {
    type DataType = u32;
}

#[doc = "Short Circuit Threshold Setting Register Channel %s"]
pub type Pdsctsrch = crate::RegValueT<Pdsctsrch_SPEC>;

impl Pdsctsrch {
    #[doc = "Short Circuit Detection Low Continuous Detection Count"]
    #[inline(always)]
    pub fn scdl(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Pdsctsrch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Pdsctsrch_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Short Circuit Detection High Continuous Detection Count"]
    #[inline(always)]
    pub fn scdh(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, u16, Pdsctsrch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1fff,1,0,u16,u16,Pdsctsrch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdsctsrch {
    #[inline(always)]
    fn default() -> Pdsctsrch {
        <crate::RegValueT<Pdsctsrch_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdovltrch_SPEC;
impl crate::sealed::RegSpec for Pdovltrch_SPEC {
    type DataType = u32;
}

#[doc = "Overvoltage Lower Threshold Register Channel %s"]
pub type Pdovltrch = crate::RegValueT<Pdovltrch_SPEC>;

impl Pdovltrch {
    #[doc = "Overvoltage Detection Lower Limit"]
    #[inline(always)]
    pub fn ovdl(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, u32, Pdovltrch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfffff,1,0,u32,u32,Pdovltrch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdovltrch {
    #[inline(always)]
    fn default() -> Pdovltrch {
        <crate::RegValueT<Pdovltrch_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdovutrch_SPEC;
impl crate::sealed::RegSpec for Pdovutrch_SPEC {
    type DataType = u32;
}

#[doc = "Overvoltage Upper Threshold Register Channel %s"]
pub type Pdovutrch = crate::RegValueT<Pdovutrch_SPEC>;

impl Pdovutrch {
    #[doc = "Overvoltage Detection Upper Limit"]
    #[inline(always)]
    pub fn ovdu(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, u32, Pdovutrch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfffff,1,0,u32,u32,Pdovutrch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdovutrch {
    #[inline(always)]
    fn default() -> Pdovutrch {
        <crate::RegValueT<Pdovutrch_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pddrcrch_SPEC;
impl crate::sealed::RegSpec for Pddrcrch_SPEC {
    type DataType = u32;
}

#[doc = "Data Read Control Register Channel %s"]
pub type Pddrcrch = crate::RegValueT<Pddrcrch_SPEC>;

impl Pddrcrch {
    #[doc = "Data Read Enable Bit"]
    #[inline(always)]
    pub fn datre(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pddrcrch::Datre,
        pddrcrch::Datre,
        Pddrcrch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pddrcrch::Datre,
            pddrcrch::Datre,
            Pddrcrch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pddrcrch {
    #[inline(always)]
    fn default() -> Pddrcrch {
        <crate::RegValueT<Pddrcrch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pddrcrch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Datre_SPEC;
    pub type Datre = crate::EnumBitfieldStruct<u8, Datre_SPEC>;
    impl Datre {
        #[doc = "Do not allow reading of data from buffer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Allow reading of data from buffer"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pddcrch_SPEC;
impl crate::sealed::RegSpec for Pddcrch_SPEC {
    type DataType = u32;
}

#[doc = "Data Clear Register Channel %s"]
pub type Pddcrch = crate::RegValueT<Pddcrch_SPEC>;

impl Pddcrch {
    #[doc = "Data Clear"]
    #[inline(always)]
    pub fn datc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pddcrch::Datc,
        pddcrch::Datc,
        Pddcrch_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pddcrch::Datc,
            pddcrch::Datc,
            Pddcrch_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pddcrch {
    #[inline(always)]
    fn default() -> Pddcrch {
        <crate::RegValueT<Pddcrch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pddcrch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Datc_SPEC;
    pub type Datc = crate::EnumBitfieldStruct<u8, Datc_SPEC>;
    impl Datc {
        #[doc = "Do nothing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear data"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pddrrch_SPEC;
impl crate::sealed::RegSpec for Pddrrch_SPEC {
    type DataType = u32;
}

#[doc = "Data Read Register Channel %s"]
pub type Pddrrch = crate::RegValueT<Pddrrch_SPEC>;

impl Pddrrch {
    #[doc = "Data"]
    #[inline(always)]
    pub fn dat(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, u32, Pddrrch_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xfffff,1,0,u32,u32,Pddrrch_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Pddrrch {
    #[inline(always)]
    fn default() -> Pddrrch {
        <crate::RegValueT<Pddrrch_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pddsrch_SPEC;
impl crate::sealed::RegSpec for Pddsrch_SPEC {
    type DataType = u32;
}

#[doc = "Data Status Register Channel %s"]
pub type Pddsrch = crate::RegValueT<Pddsrch_SPEC>;

impl Pddsrch {
    #[doc = "Number of Data Stored in Buffer"]
    #[inline(always)]
    pub fn datnum(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Pddsrch_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Pddsrch_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Pddsrch {
    #[inline(always)]
    fn default() -> Pddsrch {
        <crate::RegValueT<Pddsrch_SPEC> as RegisterValue<_>>::new(0)
    }
}
